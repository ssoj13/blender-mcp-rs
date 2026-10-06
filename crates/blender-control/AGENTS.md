# blender-control — crate

Pure Blender process control from Rust. **`std` + `sysinfo`, zero `pyo3`, zero
MCP, zero `bpy`.** The crate owns exactly one thing: the OS process. "Is the add-on connected",
"send a tool call" — not here; that needs the pipe, not a process handle.

**Internal workspace member** (`crates/blender-control` in the `blender-mcp-rs` repository):
built and versioned together with it, shared `target`/`Cargo.lock`. Split out into a
separate crate so that pure process control does not drag in `pyo3`/MCP and stays
reusable. The root `../../src/control.rs` is the MCP wrapper over it.

## API (`src/blender_control.rs`)

| Item | What |
|---|---|
| `find(explicit) -> Option<PathBuf>` | Find `blender`: explicit path → `$BLENDER_PATH`/`$BLENDER` (file OR directory) → standard locations → `None` |
| `Blender::launch(exe, args)` | Launch; **stdio → null** (see pitfall); detached — drop does NOT kill |
| `Blender::{pid,exe,is_running,exit_code,kill}` | Monitoring/killing the launched process |
| `pid_alive(pid)` / `running_pids()` | By process table — for a process we did NOT launch |

`ENV_VARS = ["BLENDER_PATH", "BLENDER"]`. `default_locations()` — for Windows/macOS/Linux.

## Pitfalls

- **`launch` silences stdio to null on purpose.** The launcher's inherited stdout may be
  a protocol channel (the MCP server writes to stdout) — Blender's output would break it. And
  an inherited stderr that nobody reads overflows the ~64 KB pipe buffer and
  **hangs Blender at startup**. The GUI shows its own window; headless has nothing to show.
- **`is_running()` = `try_wait` + `pid_alive`.** `try_wait` reaps zombies and knows only about
  our child; `pid_alive` confirms via the table (a PID can be reused).
- **Drop does not kill Blender** — a launched Blender survives a launcher restart
  (the MCP server may restart, Blender lives on). Kill only via an explicit `kill()`.
- **`sysinfo` without default-features**, only the `system` feature — we don't pull in network/disks.

## Verification

`cargo test` — 3 unit + 1 doctest. `cargo clippy` — clean. No FFI.
