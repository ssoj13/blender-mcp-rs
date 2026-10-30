//! # main.rs — Blender MCP server (rmcp stdio + local named pipe)
//!
//! An MCP client (Claude Code, etc.) talks to this process over stdio via `rmcp`.
//! Tool calls are forwarded to Blender over a local named pipe; the tool list and
//! its schemas come from `blender_mcp::tools`, the same registry that executes them
//! on the Blender side.
#![recursion_limit = "512"]

use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, Mutex};

use clap::Parser;
use tracing::{info, warn};

use rmcp_bridge::{BlenderMcpServer, PendingMap, run_stdio, spawn_listener};

mod control;
mod kdb;
/// Vendored copy of the private `mcp-setup-rs` crate — see `src/mcp_setup/VENDOR.md`.
#[allow(dead_code, unused_imports)]
mod mcp_setup;
mod rmcp_bridge;

/// Blender MCP server: MCP over stdio, Blender over a local named pipe.
///
/// With no subcommand it *is* the server, speaking MCP on stdio — which is how an
/// agent launches it. The subcommands are for a human setting it up.
#[derive(Parser, Debug)]
#[command(name = "blender-mcp-rs", version, about)]
struct Args {
    /// install / uninstall / status — register this binary with the AI agents you have.
    #[command(subcommand)]
    setup: Option<mcp_setup::cli::SetupCommand>,

    /// Enable logging to file next to the binary as <binary-name>.log. Optional custom path accepted.
    #[arg(short, long, value_name = "FILE")]
    log: Option<Option<PathBuf>>,

    /// Pipe name suffix. Must match the tag the Blender addon connects with; give
    /// each Blender instance its own tag to drive several at once.
    #[arg(short, long, default_value = "blender")]
    tag: String,

    /// Path to the Blender executable or its install directory, for the control
    /// tools (`blender_launch` etc.). Falls back to `$BLENDER_PATH` / `$BLENDER`,
    /// then the usual install locations.
    #[arg(short, long, value_name = "PATH")]
    blender: Option<PathBuf>,
}

/// How this server describes itself when it registers with an agent.
///
/// The hint block is what the model reads *before* it has called anything, so it
/// covers the two things that are not discoverable from `tools/list`: that Blender
/// must be running with the addon started, and that the addon is where the pipe is
/// opened from.
fn host_spec() -> anyhow::Result<mcp_setup::host::HostSpec> {
    let docs = mcp_setup::docs::HintDocs::new([format!(
        "## blender-mcp\n\n\
         Drives Blender ({} tools) over a local named pipe.\n\n\
         **The scene tools need Blender running with the MCP client connected.** You can\n\
         start it yourself: call `blender_launch` (opens a Blender window; pass\n\
         `background: true` for headless). `blender_status` tells you whether Blender is\n\
         up and connected; `blender_quit` / `blender_kill` / `blender_restart` manage it.\n\
         If a scene tool reports \"Blender is not connected\", call `blender_launch` first.\n\n\
         `scene_screenshot` returns the viewport as an image, so prefer looking over\n\
         guessing. `util_execute_python` runs arbitrary `bpy` code when no tool fits.",
        blender_mcp::tools::all().count()
    )]);

    let spec = mcp_setup::host::HostSpec::from_current_exe(
        "blender-mcp",
        concat!("blender-mcp:", env!("CARGO_PKG_VERSION")),
    )?
    .with_docs(docs);

    // Bake the discovered Blender path into the agent config as $BLENDER_PATH, so the
    // control tools find it without the user passing --blender every time. Only when we
    // actually found one — otherwise leave it unset and let runtime discovery try again.
    Ok(match blender_control::find(None) {
        Some(exe) => spec.with_env([("BLENDER_PATH", exe.display().to_string())]),
        None => spec,
    })
}

/// Get log file path (next to binary)
fn get_log_path(custom: Option<PathBuf>) -> PathBuf {
    if let Some(path) = custom {
        return path;
    }
    std::env::current_exe()
        .ok()
        .and_then(|p| {
            let name = p
                .file_stem()
                .map(|stem| format!("{}.log", stem.to_string_lossy()));
            name.and_then(|n| p.parent().map(|d| d.join(n)))
        })
        .unwrap_or_else(|| PathBuf::from("blender-mcp.log"))
}

/// Setup logging - to file if -l specified, otherwise stderr only
fn setup_logging(log_arg: Option<Option<PathBuf>>) -> Option<File> {
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    if let Some(custom_path) = log_arg {
        let log_path = get_log_path(custom_path);
        match File::create(&log_path) {
            Ok(file) => {
                let file_clone = file.try_clone().ok();
                tracing_subscriber::registry()
                    .with(filter.clone())
                    .with(fmt::layer().with_writer(file).with_ansi(false))
                    .init();
                info!("Logging to {}", log_path.display());
                return file_clone;
            }
            Err(e) => {
                tracing_subscriber::registry()
                    .with(filter.clone())
                    .with(fmt::layer().with_writer(std::io::sink))
                    .init();
                warn!("Failed to create log file at {}: {}", log_path.display(), e);
            }
        }
    }

    // Default: no file logging (discard logs)
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(std::io::sink))
        .init();
    None
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _crash_guard = crashdump::init!(|cfg| { cfg.spawn_dialog_on_crash = false; });
    let args = Args::parse();

    // Setup runs and exits — it must not touch stdio, which belongs to the MCP
    // protocol, nor open the pipe.
    if let Some(cmd) = &args.setup {
        // `mcp_setup`'s error is not Send+Sync, so it cannot cross into anyhow as-is.
        // This is the CLI boundary — the message is all that matters here.
        let outcome =
            mcp_setup::cli::run(cmd, &host_spec()?).map_err(|e| anyhow::anyhow!("{e}"))?;
        // A broken or unreachable client is already spelled out in the table; exit non-zero
        // rather than printing an error chain (and a backtrace) over it.
        std::process::exit(outcome.exit_code());
    }

    let _log_file = setup_logging(args.log);

    info!("Blender MCP Server starting (rmcp stdio)...");

    let writer = Arc::new(Mutex::new(None));
    let pending: Arc<Mutex<PendingMap>> = Arc::new(Mutex::new(HashMap::new()));
    let next_id = Arc::new(AtomicU64::new(1));

    // We listen; Blender connects and reconnects on its own. Failing to listen at
    // all is fatal — without a pipe no tool can ever run, and pretending otherwise
    // would surface as a confusing timeout on the first call.
    // The listener may end up on a fallback tag if the preferred one is owned by
    // another server; everything downstream must use the tag we actually claimed,
    // because that is what a launched Blender will be told to connect to.
    let tag = spawn_listener(args.tag.clone(), writer.clone(), pending.clone())?;

    let control = control::ControlCtx {
        writer,
        pending,
        next_id,
        tag: tag.clone(),
        blender_path: args.blender.clone(),
        launched: Arc::new(Mutex::new(None)),
        launched_bg: Arc::new(std::sync::atomic::AtomicBool::new(false)),
    };
    let server = BlenderMcpServer { control };

    info!("Ready for MCP on stdio (tag={tag})");
    run_stdio(server).await?;

    info!("Server stopped");
    Ok(())
}
