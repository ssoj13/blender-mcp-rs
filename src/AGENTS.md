# src — the crate

## The key oddity: one crate, two artifacts

`Cargo.toml`: `crate-type = ["cdylib", "rlib"]` + `[[bin]]`.

The same code builds into:

- **`blender_mcp.pyd`** (cdylib) — the Python extension Blender loads.
  Entry point — `lib.rs` (`#[pymodule]`).
- **`blender-mcp-rs.exe`** (bin) — the MCP server. Entry point — `main.rs`, linked against
  the same crate as a library (`use blender_mcp::...`).

**This is exactly why the tool registry works.** `src/tools/` is compiled into both
artifacts: the server takes schemas from it, the `.pyd` takes handlers. One source, two
consumers, drift is impossible by construction.

Forget this and it's easy to "optimize" the wrong thing: e.g. move `tools` into the binary
(then the `.pyd` won't see them) or into a separate crate (then `pyo3` has to be dragged
there and back).

## Files

| File | Role | Used by |
|---|---|---|
| `tools/` | **Tool registry.** See `tools/AGENTS.md` | both |
| `ipc.rs` | Named pipe: `connect`, `send_line`, `read_commands`, pipe name | both |
| `protocol.rs` | `Command` / `Response` | both |
| `lib.rs` | `#[pymodule] blender_mcp`: `BlenderMcp` (pipe client) + `execute_tool` | `.pyd` |
| `main.rs` | `rmcp` on stdio, starts the pipe listener; builds `ControlCtx` | binary |
| `rmcp_bridge.rs` | `ServerHandler`: `tools/list` (registry + control), `tools/call` → control OR pipe | binary |
| `control.rs` | **Server-side Blender process-control tools** (see below) | binary |
| `mcp_catalog.rs` | **Resources and prompts only.** No tools here | binary |

Plus the internal workspace crate `crates/blender-control` — pure process control
(find/launch/monitor/kill, `std` + `sysinfo`, zero `bpy`/MCP). `control.rs` is the
MCP wrapper over it.

## Who calls whom

```
MCP client --stdio--> main.rs/rmcp_bridge.rs
                          |  Command (JSON string)
                     named pipe (server listens, Blender connects)
                          v
                      lib.rs (.pyd)
                          |  queue -> bpy.app.timers -> main thread
                          v
                      tools::execute_tool -> bpy
```

The server **listens**, Blender **connects** and retries forever. So startup order
doesn't matter, and Blender survives a server restart. `--tag` sets the pipe name
(`blender-mcp-<tag>.sock`) — this lets you run several Blender instances.

## Two classes of tools (`control.rs`)

A regular tool executes **inside** Blender: client → server → pipe → `bpy`.
The five control tools can't work that way — you can't launch Blender through a pipe
that only exists once Blender is already running, and you can't politely ask a
hung Blender to die over that same pipe. So they execute **in the server
process** and act on the OS process:

- `blender_status` — whether Blender is running and whether the client is connected; PID and path.
- `blender_launch` — launch Blender (`background:true` — headless) and wait for it to
  connect. Writes `../addon/autostart.py` (embedded via `include_str!`) to a temp file
  and hands it to Blender via `--python`, so connecting doesn't require the addon to be
  enabled in Preferences.
- `blender_quit` — GUI: graceful via `wm.quit_blender` on a timer; headless and
  fallback: kill.
- `blender_kill` / `blender_restart`.

`rmcp_bridge::call_tool` calls `control::dispatch` first; `None` = it's a scene
tool, goes to the pipe. `tools_cached()` merges both lists into one `tools/list`.

**Isolation trap:** if we launched Blender ourselves, our `launched` handle is authoritative,
NOT the process table — otherwise the user's own open Blender gets mixed up with ours, and
`quit`/`status` start lying. `running_pids()` is only for the hand-started case.

**stdio trap:** the launched Blender gets `Stdio::null()` for everything. The server's
inherited stdout is the MCP channel (Blender's output would break the protocol), and an
undrained inherited stderr fills the 64 KB pipe buffer and **hangs Blender at startup**.

## Traps

- **`mcp_catalog.rs` is not about tools.** Only resources remain there
  (`blender://guide/*`, `examples/*`, `ref/*`) and prompts. It's a huge `json!`,
  which is why `lib.rs` has `#![recursion_limit = "512"]`. It goes away together with it.
- **`pending` in `rmcp_bridge` uses `std::sync::Mutex`, not tokio.** The pipe listener
  lives in a plain thread; there's no `await` inside the lock, so this is correct and simpler.
  Don't "fix" it back to `tokio::sync::Mutex`.
- **If Blender isn't connected**, a tool call fails immediately with a clear
  message instead of hanging until the 60-second timeout.
- **`build.rs` is deleted.** It linked `advapi32` for ZeroMQ. Verified: it builds
  without it. Don't bring it back "just in case".
