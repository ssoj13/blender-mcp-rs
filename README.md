# Blender MCP Server

Control Blender from an AI agent (Claude Code, or any MCP client). **241 tools** —
233 that act on the scene, 5 that manage the Blender process itself, and 3 that search
the built-in Blender knowledge base — all written in Rust, the scene ones executed
against `bpy` inside Blender. Includes a unified node layer that covers
every one of Blender's 570 node types.

📖 **Full documentation:** [`docs/`](docs/) (mdbook — Introduction, Whitepaper, User &
Developer guides). This README is the short version.

## Architecture

```
MCP client  <--stdio-->  blender-mcp-rs.exe  <--named pipe-->  Blender
  (Claude)                 (MCP server)                   (addon + blender_mcp.pyd)
```

A tool is **declared once**, in `src/tools/`, with its name, description, typed
params and handler:

```rust
crate::tool!(
    "obj_list",
    "List objects. Filter by type or collection",
    ObjList,
    |py: Python<'_>, p: ObjList| { /* bpy */ }
);
```

The JSON Schema the model sees is generated from the params struct by `schemars`;
the doc comment on each field becomes that property's description. Registration is
automatic (`inventory`) — there is no central list to update.

### One registry, two processes

The tool catalog is needed in the **server** process, which has no `bpy`. Execution
happens **inside Blender**. Both are Rust, and both are built from the same crate
(`crate-type = ["cdylib", "rlib"]`), so a single registry serves both:

```
                 tools::all()
                /            \
   server binary             blender_mcp.pyd
   name + description        name -> handler
   + schema -> tools/list    -> bpy via PyO3
```

The server needs no per-tool code: one route forwards `params -> pipe -> Blender ->
result` for every tool.

### Why Python is still in the addon

`bpy` is reachable only from Blender's Python process — that is a Blender constraint,
not a design choice. But the addon is *thin*: it starts the pipe client and, from a
`bpy.app.timers` callback, hands each command to `blender_mcp.execute_tool`. All 233
scene tools are Rust. (`bpy` is not thread-safe, hence the timer: the reader thread only
queues commands, and they execute on Blender's main thread.)

### Transport

One duplex local socket — a named pipe on Windows, a Unix socket elsewhere, via
`interprocess`. Pure Rust: no C library to build, no TCP port, no firewall prompt.
Framing is newline-delimited JSON.

The **server listens**; **Blender connects** and retries forever. So the two may start
in either order, and Blender survives a server restart. `--tag` names the pipe
(`blender-mcp-<tag>.sock`), so several Blender instances can be driven independently.

## Quick Start

```bash
python bootstrap.py b      # build: the server binary AND the Python extension
python bootstrap.py d      # deploy the addon + extension into Blender
python bootstrap.py i      # register the server with the AI agents you have
```

Then connect Blender to the agent, either way:

- **Let the agent launch it** — ask it to start Blender and it calls `blender_launch`
  (`blender_status`, `blender_quit`, `blender_kill`, `blender_restart` manage it too). No
  need to open Blender yourself. Blender is found via `--blender`, then
  `$BLENDER_PATH` / `$BLENDER`, then standard locations; `install` bakes the discovered
  path into the agent config as `BLENDER_PATH`.
- **Or start it by hand** — in Blender: `Edit > Preferences > Add-ons > MCP Server`,
  enable it, and click `MCP > Start` in the topbar.

Either way the agent then discovers the tools.

`bootstrap.py` also has `t` (test, including against a real Blender), `c` (fmt +
clippy) and `cl` (clean). Run it with no arguments for the full list.

### Registering by hand

`blender-mcp-rs install` knows 18 agents and writes each one's dialect for you
(`status` shows where it is registered, `uninstall` removes only what it added). If
you would rather do it yourself:

```json
{
  "mcpServers": {
    "blender": { "command": "C:\\path\\to\\blender-mcp-rs.exe" }
  }
}
```

## Project Structure

```
src/
  tools/            # THE tool registry — one file per domain, see tools/AGENTS.md
    mod.rs          #   ToolDef, the `tool!` macro, execute_tool
    bpy.rs          #   shared bpy helpers
    nodes/          #   the unified node layer — see nodes/AGENTS.md
    object.rs …     #   the tools themselves
  ipc.rs            # named-pipe transport, shared by both processes
  protocol.rs       # Command / Response
  main.rs           # MCP server: rmcp on stdio, pipe listener
  rmcp_bridge.rs    # rmcp ServerHandler; builds tools/list from the registry
  control.rs        # the 5 process-control tools (run in the server, not Blender)
  mcp_catalog.rs    # resources and prompts only (tools are NOT here)
crates/
  blender-control/  # internal crate: find/launch/monitor/kill, no bpy/MCP
addon/mcp_server/
  __init__.py       # thin addon: pipe client + timer + UI
addon/autostart.py  # auto-connect script used by blender_launch
```

## Tools (241)

The authoritative list is `tools/list` — it is generated from the registry, so it
cannot go stale. 233 scene tools by domain, plus 5 process-control tools
(`blender_status` / `launch` / `quit` / `kill` / `restart`) and 3 knowledge tools
(`kdb_search` / `kdb_read` / `kdb_guide`), all of which run in the server process
rather than inside Blender:

| Domain | Count | What |
|---|---|---|
| `nodes` | 12 | **Every node tree Blender has** — shader, geometry, compositor, world, texture. See below |
| `misc` | 19 | render, screenshot, file I/O, modifiers, collections, undo/redo, `util_execute_python` |
| `mesh` | 18 | mesh data, UVs unwrap, curves, text, shape keys, instancing |
| `rig` | 18 | armatures, bones, IK, weights, drivers, and constraints **on pose bones** |
| `util` | 17 | align/distribute/snap, booleans, assets, video sequencer |
| `scene_extra` | 17 | scene analysis and auto-fix, stats, purge, templates, cameras, lights, presets |
| `anim_extra` | 17 | F-curve modifiers, motion paths, NLA, Grease Pencil |
| `phys` | 15 | rigid body, cloth, particles, force fields, simulation zones, render queue |
| `object` | 14 | create, transform, parent, **shade smooth, apply transform, join, separate, origin** |
| `attribute` | 14 | **vertex groups, colour attributes, named mesh attributes** |
| `material` | 13 | create/assign/query, **material slots, per-face assignment, blend settings** |
| `animation` | 6 | keyframes, frame, animation data |
| `volume` | 6 | volume objects, hair curves |
| `scene` | 5 | scene info, **frame range/fps, render settings, colour management, render passes** |
| `mesh_edit` | 5 | **build topology from raw data, select by predicate, extrude/bevel/inset/…, seams** |
| `uv` | 4 | UV maps: add, remove, choose the active and the render layer |
| `image` | 4 | load, create, list, save image datablocks |
| `world` | 3 | background, HDRI |
| `custom_prop` | 3 | custom properties on any datablock **or pose bone** — this is what a rig control is |
| `bake` | 1 | texture baking (normal / AO / diffuse / emit / …) |

### Building node graphs

One family covers all 570 of Blender's node types — nothing is hand-listed, the types
and their sockets are read from Blender itself:

```
node_types(kind, search)     search the 500 creatable node types
node_schema(type, props)     a type's REAL sockets, properties and enums
node_graph_build(tree, ...)  build a whole graph in ONE call, auto-laid-out
node_graph_get / _edit       read it back, patch it
node_group_make / _add       reusable subgraphs
mat_preview(material)        render it and look at it
```

**Use `node_schema` instead of guessing socket names.** Blender 5 renamed most of
them: Principled's `Specular` is now `Specular IOR Level`, `Emission` split into
`Emission Color` and `Emission Strength`, and Noise's `Fac` output is `Factor`. A
wrong socket name is an error that hands back the node's actual sockets.

Trees are addressed by one string: `material:Rust`, `world`, `compositor`,
`group:MyGroup`, `object:Cube/GeoNodes`.

`scene_screenshot` returns the viewport as an image; `mat_preview` renders a single
material on a preview sphere in a throwaway scene. `util_execute_python` is the escape
hatch for anything not covered.

## Resources (14) and Prompts (18)

Guides, examples and references the model can read via `resources/read`
(`blender://guide/*`, `blender://examples/*`, `blender://ref/*`), plus 18 prompt
templates. Both live in `src/mcp_catalog.rs`.

## Requirements

- Blender 5.2 LTS (the tools target the Blender 5.2 API; see `src/tools/AGENTS.md` for the
  4.x APIs that were removed and how they are handled)
- Rust 1.85+ (edition 2024)
- Python 3 for `bootstrap.py` (standard library only)

The `.pyd` comes straight out of `cargo build` (the crate is a `cdylib`); `bootstrap.py`
drives it. maturin is only needed if you build the extension via `pip`/PEP 517.

## License

Dual-licensed: **BSD 3-Clause** (open source) **OR** a **commercial** license — use
whichever fits. See [`LICENSE`](LICENSE), [`LICENSE-BSD3`](LICENSE-BSD3), and
[`LICENSE-COMMERCIAL`](LICENSE-COMMERCIAL).

`SPDX-License-Identifier: BSD-3-Clause OR LicenseRef-Commercial`
