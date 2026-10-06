"""Launched by `blender_launch`: connect this Blender to the MCP server on its own.

The server starts Blender with `--python <this> -- <tag>`. It finds the installed
mcp_server addon (for the blender_mcp extension next to it), starts the pipe client
with the given tag, and drives tool calls on the main thread — without needing the
addon enabled in Preferences first.

Two run modes, because Blender's event loop differs:

* **GUI** — register a `bpy.app.timers` callback and return. The window's event loop
  fires the timer; the addon lives alongside a normal editing session.
* **`--background`** — there is no event loop after this script returns, so timers
  never fire and Blender would exit immediately. Instead we *block here* on the main
  thread, draining commands inline, and only return (letting Blender exit) once the
  server disconnects. The script itself is the event loop.
"""
import os
import sys
import time

import addon_utils
import bpy

# How often the drain loop wakes when idle, and how long headless startup waits for
# the first connection before giving up.
IDLE_SLEEP = 0.02
CONNECT_WAIT = 30.0


def _tag() -> str:
    # Args after "--" are ours; the first is the pipe tag.
    if "--" in sys.argv:
        rest = sys.argv[sys.argv.index("--") + 1:]
        if rest:
            return rest[0]
    return "blender"


def _addon_dir() -> str | None:
    for mod in addon_utils.modules():
        if mod.__name__ == "mcp_server":
            return os.path.dirname(mod.__file__)
    return None


def _drain_once(mcp, blender_mcp) -> None:
    """Execute every queued command on the main thread (bpy is not thread-safe; the
    reader thread only queues)."""
    while True:
        cmd = mcp.poll()
        if cmd is None:
            return
        try:
            out = blender_mcp.execute_tool(cmd["method"], cmd["params"])
            mcp.respond(cmd["id"], out, None)
        except Exception as e:  # noqa: BLE001 - report every tool error back
            mcp.respond(cmd["id"], None, str(e))


def _run_headless(mcp, blender_mcp) -> None:
    """Block on the main thread, driving tools until the server disconnects — the only
    way to keep a `--background` Blender alive and responsive."""
    deadline = time.monotonic() + CONNECT_WAIT
    while not mcp.is_connected() and time.monotonic() < deadline:
        time.sleep(0.1)
    if not mcp.is_connected():
        print("[autostart] headless: server never connected — exiting")
        return
    print(f"[autostart] headless MCP loop running (tag={_tag()})")
    # Once connected, run until the server goes away, then return so Blender exits.
    while mcp.is_connected():
        _drain_once(mcp, blender_mcp)
        time.sleep(IDLE_SLEEP)
    print("[autostart] server disconnected — exiting")


def main() -> None:
    # The blender_mcp extension lives next to the addon; put it on the path so a plain
    # `import blender_mcp` finds it without the addon having to be enabled.
    d = _addon_dir()
    if d and d not in sys.path:
        sys.path.insert(0, d)

    try:
        import blender_mcp
    except ImportError:
        print("[autostart] blender_mcp extension not found — run `bootstrap.py d`")
        return

    mcp = blender_mcp.BlenderMcp(_tag())
    mcp.start()

    if bpy.app.background:
        _run_headless(mcp, blender_mcp)
        return

    # GUI: the window's event loop will fire this timer.
    def poll():
        _drain_once(mcp, blender_mcp)
        return 0.05

    bpy.app.timers.register(poll, first_interval=0.1)
    print(f"[autostart] MCP client started (tag={_tag()})")


main()
