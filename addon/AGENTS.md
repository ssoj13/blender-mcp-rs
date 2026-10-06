# addon — Blender add-on

## What it is and what it does NOT do

`mcp_server/__init__.py`, 375 lines. **Zero `bpy.ops` / `bpy.data` calls.**
There is no tool logic here and there must not be — all of it is in Rust (`src/tools/`).

Python remains precisely because Blender can load add-ons **only** as
Python modules. This is a Blender limitation, not an architect's decision.

This used to hold a 6751-line monolith with 168 handlers. It has been deleted.
If you are tempted to "just add a handler here" — don't: a tool is
declared in `src/tools/`, see `src/tools/AGENTS.md`.

## What is here

| Class / function | Role |
|---|---|
| `BlenderMcpBridge` | Bridge: brings up the pipe client, runs the timer, calls Rust |
| `MCPPreferences` | Add-on settings (auto-start) |
| `MCP_OT_Start / Stop / Status` | Menu operators |
| `MCP_OT_TestHandlers` | Runs a few tools directly in Blender |
| `MCP_OT_CopyCommand / CopyConfig` | Copy the binary path / MCP config |
| `MCP_MT_Menu` | The `MCP` item in the top bar |
| `get_mcp()` | Lazy relative import of the adjacent `blender_mcp.pyd` |

## Threading model — mandatory reading

**`bpy` is not thread-safe.** Therefore:

- the pipe reader thread (inside the `.pyd`, Rust) **only enqueues commands** and
  never touches `bpy`;
- `bpy.app.timers` ticks on Blender's **main thread**, picks up a command via the
  non-blocking `poll()` and executes it via `blender_mcp.execute_tool`;
- the response goes back via `respond()`.

```python
def _poll(self, interval):
    while True:
        cmd = self._mcp.poll()      # non-blocking; None when empty
        if cmd is None:
            break
        self._process(cmd)          # -> blender_mcp.execute_tool -> bpy
    return interval                 # re-arm the timer
```

Any attempt to execute a tool from a background thread is a Blender crash, not a
tool bug.

## `autostart.py` — auto-connect for `blender_launch`

A separate script (NOT part of the add-on module). The server launches Blender with
`--python autostart.py -- <tag>`; the script finds the installed `mcp_server` via
`addon_utils.modules()`, puts its directory on `sys.path`, does `import blender_mcp` and
brings up the pipe client — **without enabling the add-on in Preferences**. It is embedded
in the server binary via `include_str!` and written to a temp file on every launch.

Two modes, by `bpy.app.background`:

- **GUI** — registers a `bpy.app.timers` callback and returns control (like the add-on).
- **`--background`** — there is NO event loop after the script, timers do not tick. So
  the script **blocks on the main thread** in a drain loop, executing tools inline,
  and returns (→ Blender exits) only when the server has disconnected. The script is its own
  event loop. That is exactly why a headless instance can connect at all.

## Installation

The `.pyd` must sit **next to `__init__.py`**: the add-on imports it as a package
submodule (`from . import blender_mcp`). A plain top-level `import blender_mcp` fails
when Blender only adds the parent add-ons directory to `sys.path`.
`bootstrap.py d` deploys the add-on. If the target directory turns out to be a stale junction (the repository was moved) — `mkdir` fails, and the junction
must be removed: `cmd //c rmdir "<addons>\mcp_server"` (removes only the link, not the target).

## Pitfalls

- **`tag` must match** the server's `--tag`, otherwise they will talk on different
  pipes and Blender will simply never connect (silently, retrying forever). The default on
  both sides is `blender`.
- `_invoke_tool()` calls Rust directly, bypassing the pipe — `Test Handlers` uses it.
  This is not a bypass of the architecture, but a local run without a server.
- In `--background`, `bpy.app.timers` do not tick. So the add-on's automated tests run
  in a manual loop on the main thread (equivalent in terms of the threading model), and
  `scene_screenshot` is verified only in the GUI at all.
