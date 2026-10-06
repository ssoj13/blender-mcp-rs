//! Server-side Blender process control: `blender_status` / `launch` / `quit` / `kill`
//! / `restart`.
//!
//! ## A second class of tool
//! Every other tool runs *inside* Blender: MCP client -> server -> pipe -> `bpy`. These
//! cannot. You cannot launch Blender through a pipe that only exists once Blender is
//! running, and you cannot kill a hung Blender by asking it politely over that pipe. So
//! these run **in the server process** and act on the OS process, using the
//! `blender-control` crate (which knows nothing about MCP or `bpy` — just processes).
//!
//! `rmcp_bridge` dispatches by name: a control tool runs here; everything else goes to
//! Blender. Both appear together in `tools/list`.
//!
//! `blender_quit` is the one that still needs the pipe — a graceful quit is a command
//! Blender runs on itself — so it reuses `call_blender`, then falls back to a kill if
//! Blender does not go.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use blender_control::Blender;
use blender_mcp::ipc;
use serde_json::{Value, json};
use tracing::debug;

use crate::rmcp_bridge::{PendingMap, call_blender};

/// The autostart script, baked into the binary. Written to a temp file at launch and
/// handed to Blender with `--python`, so a launched Blender connects on its own without
/// the addon having to be enabled in Preferences first.
const AUTOSTART_PY: &str = include_str!("../addon/autostart.py");

/// How long `blender_launch` waits for the addon to actually connect before giving up.
/// A Blender with many linked asset/material libraries can take ~10 s just to start.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(45);
/// How long `blender_quit` waits for a graceful exit before falling back to a kill. The
/// loop returns the instant the process is gone, so this only bounds a hung shutdown;
/// a heavy library set makes shutdown slower too, hence the generous margin.
const QUIT_TIMEOUT: Duration = Duration::from_secs(20);

/// Everything a control tool needs from the running server.
#[derive(Clone)]
pub struct ControlCtx {
    /// Write half of the pipe; `Some` exactly when Blender is connected.
    pub writer: Arc<Mutex<Option<ipc::Writer>>>,
    pub pending: Arc<Mutex<PendingMap>>,
    pub next_id: Arc<AtomicU64>,
    /// Pipe tag; a launched Blender is told to use this so it lands on our pipe.
    pub tag: String,
    /// Explicit Blender path from `--blender` / `$BLENDER_PATH`, if any.
    pub blender_path: Option<PathBuf>,
    /// The Blender we launched, so we can monitor and kill it. `None` if Blender was
    /// started by the user by hand (we can still see it via the process table).
    pub launched: Arc<Mutex<Option<Blender>>>,
    /// Whether the Blender we launched is headless (`--background`). A headless
    /// Blender has no event loop to run a graceful-quit timer, and nothing to save,
    /// so `blender_quit` kills it directly instead.
    pub launched_bg: Arc<AtomicBool>,
}

impl ControlCtx {
    fn connected(&self) -> bool {
        self.writer.lock().map(|g| g.is_some()).unwrap_or(false)
    }

    /// The PID of the Blender we control.
    ///
    /// If we launched one, its handle is **authoritative** — we report its PID while it
    /// runs and `None` once it exits, and never consult the global process table. That
    /// matters: the user may have their own Blender open, and conflating it with ours
    /// would make `quit`/`status` lie (e.g. "still running" after ours has gone, then a
    /// kill-after-timeout). Only when we did *not* launch Blender do we discover a
    /// hand-started one from the process table (and only if there is exactly one).
    fn pid(&self) -> Option<u32> {
        if let Ok(mut g) = self.launched.lock()
            && let Some(b) = g.as_mut()
        {
            return b.is_running().then(|| b.pid());
        }
        match blender_control::running_pids().as_slice() {
            [only] => Some(*only),
            _ => None,
        }
    }
}

/// Name + description + JSON schema of every control tool, for `tools/list`.
pub fn control_tools() -> Vec<(&'static str, &'static str, Value)> {
    vec![
        (
            "blender_status",
            "Is Blender running and connected to this server? Reports the process PID, \
             whether the MCP addon is connected (tools that act on the scene need this), \
             and the Blender path in use.",
            json!({"type": "object", "properties": {}}),
        ),
        (
            "blender_launch",
            "Start Blender and wait for it to connect to this server, so the scene tools \
             can be used. No-op if Blender is already connected.",
            json!({"type": "object", "properties": {
                "background": {"type": "boolean",
                    "description": "Run headless with no window (default false — a normal Blender window)."}
            }}),
        ),
        (
            "blender_quit",
            "Ask Blender to quit gracefully; kills it only if it does not exit in time.",
            json!({"type": "object", "properties": {}}),
        ),
        (
            "blender_kill",
            "Force-terminate Blender immediately. Use when it is hung and blender_quit \
             will not work.",
            json!({"type": "object", "properties": {}}),
        ),
        (
            "blender_restart",
            "Quit Blender (or kill it if unresponsive) and start a fresh one, waiting for \
             it to reconnect.",
            json!({"type": "object", "properties": {
                "background": {"type": "boolean", "description": "Run the new Blender headless."}
            }}),
        ),
    ]
}

/// Run a control tool, or return `None` if `name` is not one — so the caller falls
/// through to the Blender pipe.
pub async fn dispatch(ctx: &ControlCtx, name: &str, args: Value) -> Option<Result<String, String>> {
    let result = match name {
        "blender_status" => status(ctx),
        "blender_launch" => launch(ctx, &args).await,
        "blender_quit" => quit(ctx).await,
        "blender_kill" => kill(ctx),
        "blender_restart" => restart(ctx, &args).await,
        _ => return None,
    };
    Some(result.map(|v| v.to_string()))
}

fn status(ctx: &ControlCtx) -> Result<Value, String> {
    let pid = ctx.pid();
    let path = ctx
        .blender_path
        .clone()
        .or_else(|| blender_control::find(None))
        .map(|p| p.display().to_string());
    Ok(json!({
        "process_running": pid.is_some(),
        "pid": pid,
        "connected": ctx.connected(),
        "tag": ctx.tag,
        "blender_path": path,
    }))
}

async fn launch(ctx: &ControlCtx, args: &Value) -> Result<Value, String> {
    if ctx.connected() {
        return Ok(json!({ "launched": false, "reason": "already connected", "pid": ctx.pid() }));
    }

    let exe = blender_control::find(ctx.blender_path.as_deref()).ok_or_else(|| {
        "Blender not found — set $BLENDER_PATH or pass --blender, or install Blender in a \
         standard location"
            .to_string()
    })?;

    // The autostart script must exist on disk for `--python`. Write the baked-in copy to
    // a temp file; Blender reads it once at startup.
    let script = std::env::temp_dir().join("blender_mcp_autostart.py");
    std::fs::write(&script, AUTOSTART_PY).map_err(|e| format!("write autostart script: {e}"))?;

    let background = args
        .get("background")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let mut cmd_args: Vec<String> = Vec::new();
    if background {
        cmd_args.push("--background".into());
    }
    cmd_args.push("--python".into());
    cmd_args.push(script.to_string_lossy().into_owned());
    // Everything after `--` is handed to the script; the tag tells it which pipe to use.
    cmd_args.push("--".into());
    cmd_args.push(ctx.tag.clone());

    let blender = Blender::launch(&exe, &cmd_args).map_err(|e| format!("launch Blender: {e}"))?;
    let pid = blender.pid();
    if let Ok(mut g) = ctx.launched.lock() {
        *g = Some(blender);
    }
    ctx.launched_bg.store(background, Ordering::SeqCst);

    // Wait for the addon to actually connect — a launched Blender that never connects is
    // useless, and the model needs to know which happened.
    let start = std::time::Instant::now();
    while start.elapsed() < CONNECT_TIMEOUT {
        if ctx.connected() {
            return Ok(json!({ "launched": true, "connected": true, "pid": pid }));
        }
        // Blender died during startup — say so instead of waiting out the timeout.
        if let Ok(mut g) = ctx.launched.lock()
            && let Some(b) = g.as_mut()
            && !b.is_running()
        {
            return Err(format!(
                "Blender (pid {pid}) exited during startup without connecting — is the \
                 addon deployed? (bootstrap.py d)"
            ));
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    Err(format!(
        "Blender (pid {pid}) started but did not connect within {}s — the MCP addon may \
         not be deployed (bootstrap.py d)",
        CONNECT_TIMEOUT.as_secs()
    ))
}

async fn quit(ctx: &ControlCtx) -> Result<Value, String> {
    let pid = ctx.pid();
    if pid.is_none() {
        return Ok(json!({ "quit": false, "reason": "Blender is not running" }));
    }

    // A headless Blender runs no event loop, so a quit timer would never fire — and it
    // has no unsaved UI state to preserve. Kill it directly.
    if ctx.launched_bg.load(Ordering::SeqCst) {
        return kill(ctx).map(|_| json!({ "quit": true, "graceful": false, "headless": true }));
    }

    // A graceful quit is a command Blender runs on itself. Schedule it on a timer so the
    // operator runs in a valid context, off the tool-call callback.
    if ctx.connected() {
        let code = "import bpy; bpy.app.timers.register(lambda: bpy.ops.wm.quit_blender(), \
                    first_interval=0.05)";
        // Best effort: a successful quit tears down the pipe before the response is
        // sent, so this call usually returns Err (channel closed) even when it worked.
        // The real signal is whether the process disappears, polled below.
        if let Err(e) = call_blender(
            "util_execute_python",
            json!({ "code": code }),
            &ctx.writer,
            &ctx.pending,
            &ctx.next_id,
        )
        .await
        {
            debug!("quit command returned {e} (expected if Blender exited first)");
        }
    }

    // Wait for it to actually go; kill if it does not.
    let start = std::time::Instant::now();
    while start.elapsed() < QUIT_TIMEOUT {
        if ctx.pid().is_none() {
            return Ok(json!({ "quit": true, "graceful": true }));
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    kill(ctx).map(|_| json!({ "quit": true, "graceful": false, "note": "killed after timeout" }))
}

fn kill(ctx: &ControlCtx) -> Result<Value, String> {
    // Prefer the child we launched — a clean handle, no PID-reuse risk.
    if let Ok(mut g) = ctx.launched.lock()
        && let Some(b) = g.as_mut()
    {
        if b.is_running() {
            b.kill().map_err(|e| format!("kill Blender: {e}"))?;
        }
        *g = None;
        return Ok(json!({ "killed": true }));
    }

    // Otherwise a Blender the user started by hand.
    match blender_control::running_pids().as_slice() {
        [] => Ok(json!({ "killed": false, "reason": "no Blender process running" })),
        pids => {
            for &pid in pids {
                kill_pid(pid)?;
            }
            Ok(json!({ "killed": true, "pids": pids }))
        }
    }
}

async fn restart(ctx: &ControlCtx, args: &Value) -> Result<Value, String> {
    // Best effort: bring the old Blender down first. Even if that errors we still try to
    // launch — `launch` reports the real outcome (and no-ops if one is still connected).
    if let Err(e) = quit(ctx).await {
        debug!("restart: quitting the old Blender failed: {e}");
    }
    launch(ctx, args).await
}

/// Terminate a PID we did not spawn (a hand-started Blender). std has no portable kill
/// for an arbitrary PID, so shell out to the platform tool.
fn kill_pid(pid: u32) -> Result<(), String> {
    let status = if cfg!(windows) {
        std::process::Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .status()
    } else {
        std::process::Command::new("kill")
            .args(["-9", &pid.to_string()])
            .status()
    };
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(format!("kill pid {pid} exited with {s}")),
        Err(e) => Err(format!("kill pid {pid}: {e}")),
    }
}
