# MCP Server addon — installation

The easy way:

```bash
python bootstrap.py b     # build the server binary and the Python extension
python bootstrap.py d     # copy the addon + the extension into Blender
```

Then in Blender: `Edit > Preferences > Add-ons`, search "MCP Server", enable it, and
click `MCP > Start` in the topbar. (Optionally enable "Auto-start on Blender launch"
in the addon preferences.)

## What has to end up where

The addon is Python, but it does nothing on its own — every tool lives in the Rust
extension. Both files must sit in the **same** directory, because `__init__.py` does a
plain `import blender_mcp`, and Blender only puts the addon's own directory on
`sys.path`:

```
<blender>/scripts/addons/mcp_server/
    __init__.py        the addon (bridge + UI, no bpy logic)
    blender_mcp.pyd    the Rust extension (.so on Linux/macOS)
```

A missing extension shows up as `ModuleNotFoundError: blender_mcp` when the addon is
enabled — that means `bootstrap.py d` was not run, or was run before `b`.

## By hand

Blender 5.2 addon directories:

| OS | Path |
|---|---|
| Windows | `%APPDATA%\Blender Foundation\Blender\5.2\scripts\addons\` |
| Linux | `~/.config/blender/5.2/scripts/addons/` |
| macOS | `~/Library/Application Support/Blender/5.2/scripts/addons/` |

Copy `addon/mcp_server` there, then copy the built extension next to `__init__.py`,
renaming it: `target/release/blender_mcp.dll` → `blender_mcp.pyd` (Windows), or
`libblender_mcp.so` → `blender_mcp.so` (Linux).

For development a symlink to `addon/mcp_server` avoids re-copying the Python on every
change — but the extension still has to be copied in after each `cargo build`:

```powershell
# Windows, as Administrator
New-Item -ItemType Junction `
    -Path "$env:APPDATA\Blender Foundation\Blender\5.2\scripts\addons\mcp_server" `
    -Target "C:\path\to\blender-mcp-rs\addon\mcp_server"
```

```bash
ln -s /path/to/blender-mcp-rs/addon/mcp_server ~/.config/blender/5.2/scripts/addons/mcp_server
```

## Connecting to the server

The addon connects to a named pipe; the **server listens**. Start order does not
matter — the addon retries until the server appears, and it survives a server restart.

The `tag` must match on both sides (default `blender` on each). Run the server with
`--tag foo` and the addon must use the same tag, or the two sit on different pipes and
never meet — silently, because the addon just keeps retrying.
