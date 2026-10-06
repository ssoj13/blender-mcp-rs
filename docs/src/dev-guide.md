# Developer guide

How to build on the code: add a tool, run the tests, and avoid the Blender-5 traps that
cost real debugging time.

## Layout

It is a Cargo **workspace**. The root package `blender-mcp-rs` builds both the server
binary and the `.pyd`; `crates/blender-control` is an internal member for pure process
control.

```
src/
  tools/            # THE tool registry — one file per domain
    mod.rs          #   ToolDef, the `tool!` macro, execute_tool, guard tests
    bpy.rs          #   shared bpy helpers (op_ok, enum_values, action_fcurves…)
    nodes/          #   the unified node layer
    object.rs …     #   the tools themselves
  ipc.rs            # named-pipe transport, shared by both processes
  protocol.rs       # Command / Response
  lib.rs            # the .pyd: pipe client + execute_tool
  main.rs           # MCP server: rmcp on stdio, pipe listener, ControlCtx
  rmcp_bridge.rs    # rmcp ServerHandler; tools/list from the registry
  control.rs        # the 5 process-control tools
  kdb.rs            # the knowledge base: BM25 search + the 3 kdb_* tools
  mcp_catalog.rs    # resources and prompts only
kdb/                      # the corpus itself (generated + kdb/guides/, hand-written)
crates/blender-control/   # internal crate: find/launch/monitor/kill, no bpy/MCP
addon/mcp_server/         # thin add-on
addon/autostart.py        # auto-connect script for blender_launch
```

Each significant directory carries a local `AGENTS.md` with its own notes — read it
before working there.

## Adding a tool

A tool is declared **once** with the `tool!` macro and registers itself via `inventory`.
There is no central `match`, no registration list.

```rust
crate::tool!(
    "obj_list",                                    // MCP tool name (must be unique)
    "List objects. Filter by type or collection",  // description the model reads
    ObjList,                                        // params struct (derives Deserialize + JsonSchema)
    |py: Python<'_>, p: ObjList| { /* runs inside Blender against bpy */ }
);
```

The JSON Schema comes from the params struct via `schemars`; **each field's `///` doc
comment becomes that property's description** — the model reads those, so write them
well. Add the tool to the right domain file (or a new one wired into `src/tools/mod.rs`);
the server picks it up in `tools/list` automatically, and Blender gets the handler. There
is nothing else to keep in sync — that is the whole design (see the
[whitepaper](./whitepaper.md)).

## The error contract

- A call that could not do what was asked returns **`Err`** → reaches the model as an MCP
  error with `isError`.
- **Never** report failure as `Ok(json!({"error": ...}))` — that is a *success* the model
  reads past. A guard test (`no_failure_is_reported_as_success`) fails the build if this
  pattern reappears.
- A per-item failure inside a **batch** is data: keep it in the `results` array and let
  the call succeed.
- "Empty" is not an error — a mesh with no shape keys returns an empty list, not `Err`.
- **No panics in tool code**: `unwrap` / `expect` / `let _ =` on fallible operations are
  banned. A panic inside Blender takes down more than the tool. Propagate with `?`.

## Blender-5 traps (paid for in blood)

- **`bpy.ops` does not raise when it refuses to act.** It returns a set: `{'FINISHED'}`
  worked, `{'CANCELLED'}` means the poll passed but *nothing changed* (`transform_apply`
  with nothing selected, `join` of one object). Ignoring the result reports success for an
  operation that never happened. Route every op through `bpy::op_ok()`.
- **Dynamic enums (`itemf`) lie under static introspection.** `render.engine`'s
  `bl_rna.enum_items` returns *only* `BLENDER_EEVEE`, though `CYCLES` sets fine — trusting
  it rejected Cycles on every render. Blender only reveals the real list in the *error
  text* for a wrong value; that is what `bpy::enum_values()` does. Never read `enum_items`
  directly.
- **Pointers are not strings.** `node.image = "brick.png"` throws — `image`, `object`,
  `collection`, `material`, `node_tree` are pointers, resolved from a datablock name.
- **Geometry-node zones are node *pairs*.** Simulation/Repeat/ForEach Input nodes have no
  geometry sockets until `pair_with_output()` is called.
- **`is` on bpy wrappers is unreliable** — compare via `as_pointer()`.
- **Baking silently writes nothing** if the target Image Texture node is not selected —
  it returns `FINISHED` and changes no pixels. Caught only by comparing pixels.
- **Removed/renamed APIs**: `Action.fcurves` was removed in 5.0 (use
  `bpy::action_fcurves()`); Grease Pencil was rewritten in 4.3 (`GPENCIL` →
  `GREASEPENCIL`, and much more); the VSE `sequences` became `strips` in 4.4; the
  `musgrave` node was removed in 4.1 (use `ShaderNodeTexNoise`).

> **Do not guess Blender's API.** There is a live Blender — ask it:
> `blender.exe --background --python probe.py`. Agents confidently invent enum names;
> four were caught exactly this way.

## Building and testing

```bash
python bootstrap.py b      # build both artifacts (release)
python bootstrap.py t      # cargo tests + stdio smoke + tools against a live Blender
python bootstrap.py c      # cargo fmt --check + clippy -D warnings
```

`bootstrap.py t` builds first (never tests a stale artifact), then runs:

- **`cargo test --lib`** — registry guard tests: names are unique, every tool is
  well-formed, and the "no failure reported as success" scanner.
- **stdio smoke** (`python/test_mcp_server.py`, no Blender) — `initialize`, `tools/list`,
  and the server-side control tools (`blender_status` answers with no Blender).
- **live Blender** (`python/blender_smoke.py` and the per-domain suite in
  `python/tests/`) — every domain exercised against a fresh scene, asserting the *effect*
  in `bpy`, not merely that the call returned.

`scene_screenshot` is **verified only in a GUI Blender** — it needs an OpenGL context and
will not run in `--background` at all. That is how a regression in it survived dozens of
headless runs.

## Building this book

The diagrams are Mermaid, rendered by the `mdbook-mermaid` preprocessor:

```bash
cargo install mdbook mdbook-mermaid
mdbook-mermaid install docs     # once: drops in the mermaid assets
mdbook serve docs               # live preview
mdbook build docs               # output to docs/book/ (gitignored)
```
