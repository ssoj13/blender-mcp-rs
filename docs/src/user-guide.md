# User guide

This chapter takes you from a clean checkout to an agent building scenes in Blender.

## Requirements

- **Blender 5.2 LTS** — the tools target the Blender 5.2 API.
- **Rust 1.85+** (edition 2024).
- **Python 3** for `bootstrap.py` (standard library only, cross-platform).

The build produces two artifacts from one crate: the server binary
(`blender-mcp-rs.exe`) and the Python extension (`blender_mcp.pyd`).

## Build, deploy, register

Everything goes through `bootstrap.py`:

```bash
python bootstrap.py b      # build: the server binary AND the Python extension
python bootstrap.py d      # deploy the add-on + extension into Blender
python bootstrap.py i      # register the server with the AI agents you have
```

Other subcommands: `t` (test, including against a live Blender), `c` (fmt + clippy),
`cl` (clean). Run `bootstrap.py` with no arguments for the full list.

Blender is found at `C:\Programs\Blender5` by default; override with `$BLENDER` or
`--blender PATH`.

## Connecting Blender to the agent

There are two ways. Pick whichever fits.

### The agent launches Blender itself (recommended)

Once the server is registered with your agent, just ask it to start Blender — it calls
`blender_launch`. Blender opens, connects on its own, and the scene tools become usable.
No need to open Blender or enable the add-on by hand.

The control tools:

| Tool | What it does |
|---|---|
| `blender_status` | Is Blender running and connected? PID, connection state, Blender path. |
| `blender_launch` | Start Blender and wait for it to connect. `background: true` runs it headless. |
| `blender_quit` | Ask Blender to quit gracefully; kill it only if it will not exit. |
| `blender_kill` | Force-terminate immediately (for a hung Blender). |
| `blender_restart` | Quit (or kill) and start a fresh one, waiting for it to reconnect. |

Blender is located by, in order: the `--blender` flag, then `$BLENDER_PATH` / `$BLENDER`,
then the usual install locations. On `bootstrap.py i` / `blender-mcp-rs install`, the
discovered path is baked into the agent config as `BLENDER_PATH`, so the control tools
find Blender without any per-call argument.

> A Blender with many linked asset/material libraries can take ~10 s to start and to
> quit; the launch and quit tools wait accordingly.

### You start Blender by hand

Open Blender, then `Edit > Preferences > Add-ons > MCP Server`, enable it, and click
`MCP > Start` in the topbar. The agent discovers the tools once the client connects. This
is the classic flow and still works; `blender_status` will report `connected: true`.

## Registering the server by hand

`blender-mcp-rs install` knows 18 agents (Claude Code, Codex, Cursor, VS Code, Zed…) and
writes each one's dialect for you. `status` shows where it is registered; `uninstall`
removes only what it added. If you would rather edit the config yourself:

```json
{
  "mcpServers": {
    "blender": { "command": "C:\\path\\to\\blender-mcp-rs.exe" }
  }
}
```

## Using the tools

The authoritative tool list is `tools/list` — generated from the registry, so it never
goes stale. Of the 241, 5 manage the Blender process and 3 search the knowledge base;
the 233 scene tools, by domain:

| Domain | Count | What |
|---|---|---|
| `nodes` | 12 | **Every node tree Blender has** — shader, geometry, compositor, world, texture |
| `misc` | 19 | render, screenshot, file I/O, modifiers, collections, undo/redo, `util_execute_python` |
| `mesh` | 18 | mesh data, UV unwrap, curves, text, shape keys, instancing |
| `rig` | 18 | armatures, bones, IK, weights, drivers, constraints on pose bones |
| `util` | 17 | align/distribute/snap, booleans, assets, video sequencer |
| `scene_extra` | 17 | scene analysis and auto-fix, stats, purge, templates, cameras, lights, presets |
| `anim_extra` | 17 | F-curve modifiers, motion paths, NLA, Grease Pencil |
| `phys` | 15 | rigid body, cloth, particles, force fields, simulation zones, render queue |
| `object` | 14 | create, transform, parent, shade, apply transform, join, separate, origin |
| `attribute` | 14 | vertex groups, colour attributes, named mesh attributes |
| `material` | 13 | create/assign/query, slots, per-face assignment, blend settings |
| `animation` | 6 | keyframes, frame, animation data |
| `volume` | 6 | volume objects, hair curves |
| `scene` | 5 | scene info, frame range/fps, render settings, colour management, passes |
| `mesh_edit` | 5 | build topology from raw data, select by predicate, extrude/bevel/inset, seams |
| `uv` | 4 | UV maps: add, remove, choose active and render layer |
| `image` | 4 | load, create, list, save image datablocks |
| `world` | 3 | background, HDRI |
| `custom_prop` | 3 | custom properties on any datablock or pose bone |
| `bake` | 1 | texture baking (normal / AO / diffuse / emit / …) |

Plus the 5 process-control tools above.

### Building node graphs

One family covers all 570 node types; nothing is hand-listed:

```
node_types(kind, search)     search the ~500 creatable node types
node_schema(type, props)     a type's REAL sockets, properties and enums
node_graph_build(tree, ...)  build a whole graph in ONE call, auto-laid-out
node_graph_get / _edit       read it back, patch it
node_group_make / _add       reusable subgraphs
mat_preview(material)        render it and look at it
```

**Use `node_schema` instead of guessing socket names** — Blender 5 renamed most of them.
Trees are addressed by one string: `material:Rust`, `world`, `compositor`,
`group:MyGroup`, `object:Cube/GeoNodes`.

### Looking instead of guessing

- `scene_screenshot` returns the viewport as an image.
- `mat_preview` renders a single material on a preview sphere in a throwaway scene.
- `util_execute_python` runs arbitrary `bpy` code when no tool fits — the escape hatch.

## The knowledge base (`kdb`)

Nothing in `tools/list` says how Blender works, so the server ships a corpus and three
tools that search it. They answer **with Blender closed** — an agent can read how
something is done before it launches anything.

| Tool | What it does |
|---|---|
| `kdb_search` | BM25 search over the whole corpus. Keywords, not sentences; API names work verbatim |
| `kdb_read` | Open one hit whole, by the `path` a search result reports. Long documents page with `offset` |
| `kdb_guide` | The per-area briefings. No argument lists them; `area` reads one |

What is in there:

- **`kdb/guides/`** — eleven hand-written area briefings (getting started, modelling,
  UV and attributes, shading, geometry nodes, animation, rigging, physics, rendering,
  objects and scene, pipeline I/O). Each names the tools for the job, the order to call
  them in, and the traps. These rank above everything else in search, because "how do I"
  is usually the real question.
- **`kdb/api/`** — the API reference, dumped from a **live Blender** by introspection:
  RNA types with their enum values, operator signatures, and the real input/output socket
  names of every node type. This is the "ask Blender, don't guess" rule as data.
- **`kdb/release-notes/`** — 5.0 through 5.3, which is what explains a call that used to
  work and now does not.
- **`kdb/manual/`** — the Blender 5.2 user manual.

### Adding your own notes

Point `$BLENDER_MCP_KDB` at a directory of `.md` files (nested however you like) and they
join the same index at startup, marked `local`. A studio can put its naming conventions,
rig standards or scene setup rules there without rebuilding anything.

A document may carry a small frontmatter block, which is what `area` filtering reads:

```markdown
---
area: guide
title: Our rig naming rules
version: 5.2
---
```

### Rebuilding the corpus

Everything except `kdb/guides/` is generated, and regenerating it needs a Blender to
introspect:

```bash
python python/kdb_build.py --blender "C:\Programs\Blender5\blender.exe"
```

That refreshes `kdb/api/` from the live build, re-fetches the release notes and the
manual for that version, and leaves the hand-written guides alone. The corpus is baked
into the binary at compile time, so a rebuild of the server follows.

## Resources and prompts

Beyond tools, the server exposes **14 resources** (guides, examples and references the
model can read via `resources/read`: `blender://guide/*`, `blender://examples/*`,
`blender://ref/*`) and **18 prompt templates**. Both live in `src/mcp_catalog.rs`.

## Next

Copy-ready node setups are in the [recipes](./recipes.md); a full scene built end to end
is in the [worked example](./example-city.md).
