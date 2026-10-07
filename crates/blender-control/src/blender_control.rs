//! Find, launch, monitor and terminate a Blender process from Rust.
//!
//! Pure `std` + `sysinfo` — no `pyo3`, no MCP, no knowledge of what the process is
//! *for*. This crate owns exactly one thing: the OS process. "Is the addon connected",
//! "send a tool call" and the like live in the caller, because they need the pipe, not
//! the process handle.
//!
//! ```no_run
//! use blender_control::{find, Blender};
//! let exe = find(None).expect("no Blender found");
//! let mut b = Blender::launch(&exe, ["--background"]).unwrap();
//! assert!(b.is_running());
//! b.kill().unwrap();
//! ```

use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

/// The environment variables consulted, in order, before falling back to well-known
/// install locations. `BLENDER_PATH` is the documented one; `BLENDER` is accepted too
/// because the project's `bootstrap.py` already uses it.
pub const ENV_VARS: [&str; 2] = ["BLENDER_PATH", "BLENDER"];

/// Locate the `blender` executable.
///
/// Order: an explicit path, then `$BLENDER_PATH` / `$BLENDER` (each may point at the
/// executable itself OR at the install directory), then the platform's usual
/// locations, then `blender` on `PATH`. Returns `None` if nothing exists — the caller
/// decides whether that is an error.
pub fn find(explicit: Option<&Path>) -> Option<PathBuf> {
    if let Some(p) = explicit {
        return resolve(p);
    }
    for var in ENV_VARS {
        if let Some(v) = std::env::var_os(var)
            && let Some(found) = resolve(Path::new(&v))
        {
            return Some(found);
        }
    }
    default_locations().into_iter().find(|p| p.is_file())
}

/// Accept either the executable itself or the directory that contains it.
fn resolve(p: &Path) -> Option<PathBuf> {
    if p.is_file() {
        return Some(p.to_path_buf());
    }
    if p.is_dir() {
        let exe = p.join(exe_name());
        if exe.is_file() {
            return Some(exe);
        }
    }
    None
}

fn exe_name() -> &'static str {
    if cfg!(windows) {
        "blender.exe"
    } else {
        "blender"
    }
}

fn default_locations() -> Vec<PathBuf> {
    if cfg!(windows) {
        let mut v = vec![PathBuf::from(r"C:\Programs\Blender5\blender.exe")];
        for series in ["5.2", "5.1", "5.0", "4.5", "4.4", "4.3"] {
            v.push(PathBuf::from(format!(
                r"C:\Program Files\Blender Foundation\Blender {series}\blender.exe"
            )));
        }
        v
    } else if cfg!(target_os = "macos") {
        vec![PathBuf::from(
            "/Applications/Blender.app/Contents/MacOS/Blender",
        )]
    } else {
        vec![
            PathBuf::from("/usr/bin/blender"),
            PathBuf::from("/usr/local/bin/blender"),
        ]
    }
}

/// A launched Blender process. Dropping this does **not** kill Blender — a launched
/// Blender is meant to outlive the launcher (the MCP server may restart while Blender
/// keeps running). Call [`Blender::kill`] to terminate it explicitly.
#[derive(Debug)]
pub struct Blender {
    child: Child,
    exe: PathBuf,
}

impl Blender {
    /// Launch `exe` with `args`, detached enough to survive the caller.
    ///
    /// The child's stdio is set to null, deliberately: the launcher's stdout may be a
    /// protocol channel (e.g. an MCP server speaking on stdout), and an inherited
    /// stdout would inject Blender's console spam straight into it. Worse, an inherited
    /// stderr the caller never drains fills its ~64 KB pipe buffer and *blocks Blender*
    /// mid-startup. A GUI Blender's own window is its UI regardless; a headless one has
    /// nothing to show. Callers wanting the log should redirect it to a file themselves.
    pub fn launch<I, S>(exe: &Path, args: I) -> io::Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let child = Command::new(exe)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(Self {
            child,
            exe: exe.to_path_buf(),
        })
    }

    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    pub fn exe(&self) -> &Path {
        &self.exe
    }

    /// Whether the process is still alive.
    ///
    /// Uses `try_wait` first (cheap, and reaps the child if it exited so it does not
    /// linger as a zombie), then confirms against the process table — a PID can be
    /// recycled, and `try_wait` only knows about a child we spawned.
    pub fn is_running(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(Some(_)) => false, // exited, now reaped
            Ok(None) => pid_alive(self.child.id()),
            Err(_) => false,
        }
    }

    /// The process's exit code if it has already exited, else `None`.
    pub fn exit_code(&mut self) -> Option<i32> {
        match self.child.try_wait() {
            Ok(Some(status)) => Some(status.code().unwrap_or(-1)),
            _ => None,
        }
    }

    /// Terminate Blender immediately (`kill`/`TerminateProcess`). For a graceful exit,
    /// the caller should first send Blender its own quit command over the pipe and only
    /// fall back to this if it does not go.
    pub fn kill(&mut self) -> io::Result<()> {
        // Already gone is success, not an error.
        if let Ok(Some(_)) = self.child.try_wait() {
            return Ok(());
        }
        self.child.kill()?;
        let _ = self.child.wait();
        Ok(())
    }
}

/// Is a process with this PID alive right now, regardless of who started it.
///
/// This is the half of "status" that works even when the caller did not launch Blender
/// itself (the user started it by hand) — given a PID from elsewhere, it still answers.
pub fn pid_alive(pid: u32) -> bool {
    let mut sys = sysinfo::System::new();
    let spid = sysinfo::Pid::from_u32(pid);
    sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[spid]), true);
    sys.process(spid).is_some()
}

/// Every running Blender process's PID, found by executable name. Lets the caller
/// discover a Blender the user launched by hand, or notice strays.
pub fn running_pids() -> Vec<u32> {
    let mut sys = sysinfo::System::new();
    // Only names and PIDs are needed. Fetching every process's environment and
    // command line can block on an unrelated process; Linux tasks aren't Blender
    // instances either. Names/PIDs are collected even with all refreshes disabled.
    sys.refresh_processes_specifics(
        sysinfo::ProcessesToUpdate::All,
        true,
        sysinfo::ProcessRefreshKind::nothing().without_tasks(),
    );
    let target = exe_name();
    sys.processes()
        .values()
        .filter(|p| {
            p.name()
                .to_str()
                .is_some_and(|n| n.eq_ignore_ascii_case(target))
        })
        .map(|p| p.pid().as_u32())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_accepts_file_and_dir() {
        // A path that does not exist resolves to nothing, never panics.
        assert!(find(Some(Path::new(r"C:\nope\blender.exe"))).is_none());
    }

    #[test]
    fn pid_alive_for_self_is_true() {
        assert!(pid_alive(std::process::id()));
    }

    #[test]
    fn pid_alive_for_bogus_is_false() {
        // PID 0 / a very high unlikely PID is not a real process.
        assert!(!pid_alive(u32::MAX - 1));
    }
}
