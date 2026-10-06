# src/tools — MCP tool registry

## What it is

The single source of truth for all Blender MCP tools. A tool is
declared **once** with the `tool!` macro and registers itself via
`inventory`. There is no central `match` and no registration list.

```rust
crate::tool!(
    "obj_list",                                  // MCP tool name
    "List objects. Filter by type or collection", // description for the model
    ObjList,                                     // parameter struct
    |py: Python<'_>, p: ObjList| { ... }         // body, executes inside Blender
);
```

The JSON schema is generated from the parameter type via `schemars`. **Field doc comments
(`///`) become the schema properties' `description`** — this is what the
model reads, so write them meaningfully.

## Why it's done this way (and why it can't go back to "how it was")

Previously a tool was declared in three places the compiler couldn't cross-check:
a handwritten JSON schema in `mcp_catalog.rs`, a `match` arm in `bpy_dispatch/mod.rs`, and
the implementation itself. Adding a tool = three synchronized edits. Keeping them in sync
relied on discipline.

## Two consumers, one registry

The catalog is needed by the **server process** (`main.rs`), which has no `bpy`. Execution
happens **inside Blender** (`.pyd`). Both are Rust, both are built from the same
crate (`crate-type = ["cdylib", "rlib"]`), so one registry serves both:

```
                 tools::all()   (inventory)
                /                          \
   server (main.rs)                     Blender (lib.rs)
   name + description + schema          name -> handler(py, params)
   -> rmcp ToolRouter                   -> bpy via PyO3
```

The server **needs no per-tool code**: a single blanket route drives
`params -> IPC -> Blender -> result` for all of them.

## Error contract (important)

A call that couldn't do what was asked returns **`Err`**. It reaches
the model as an MCP error with `isError`, and cannot be mistaken for success.

**Do not** report failure via `Ok(json!({"error": ...}))`. That is a *successful*
result whose text merely mentions an error: the model reads past it and keeps
going down the broken path. Plus the compiler can't see that link.

The only exception is a **batch item**. If a call works on many targets and
some succeeded, individual item failures are data, and they belong in the `results` array:

```rust
results.push(json!({ "name": name, "error": "not found" }));   // ok: partial success
return Err(PyValueError::new_err(format!("object not found: {name}")));  // single target
```

Separately: "empty" is not an error. A mesh has no shape keys, an object has no animation —
the honest answer is an empty list/dict, not `Err` and not `{"error"}`.

## Blender traps, paid for in blood

- **`bpy.ops` doesn't raise an exception when it refuses to work.** It returns
  a set: `{'FINISHED'}` — it worked, `{'CANCELLED'}` — poll passed, but **nothing
  changed** (`transform_apply` with no selection, `join` with a single object,
  `remove_doubles` with no selection). Ignoring the result = reporting success for an
  operation that never happened. All calls must go through `bpy::op_ok()`.
- **Dynamic enums (`itemf`) lie under static introspection.** For `render.engine`,
  `bl_rna.enum_items` returns **only** `BLENDER_EEVEE`, even though `CYCLES` and
  `BLENDER_WORKBENCH` set just fine. Code that trusted `enum_items` **rejected
  Cycles on every render** — this was a real bug. Same with `view_transform` and `look`.
  Blender only gives the real list in the error text for an invalid value; that's what
  `bpy::enum_values()` does. Don't read `enum_items` directly.
- **Pointers are not strings.** `node.image = "brick.png"` raises: `image`, `object`,
  `collection`, `material`, `node_tree` are pointers. The datablock is referenced by name and
  resolved to an object (`nodes::set_node_prop` / `set_socket_value`).
- **Geometry-node zones are PAIRS of nodes.** Simulation/Repeat/ForEach Input has no
  geometry sockets until `pair_with_output()` is called.
- **`is` doesn't work on bpy wrappers** — compare via `as_pointer()`.
- **Blender silently writes nothing when baking** if the required Image Texture node isn't
  selected: it returns `FINISHED` and doesn't change a single pixel. Only caught by comparing
  pixels before/after.

## Traps

- **`inventory` inside the `.pyd`.** The registry relies on linker sections. Verified on
  live Blender 5.2 / CPython 3.13 on Windows — it works. If it ever stops working,
  fallbacks: `linkme`, then an explicit `register_all()`.
- **Blender 5.2 moved Geometry Nodes modifier inputs.** `modifier["Socket_2"]` now
  raises `TypeError: this type doesn't support IDProperties`; the value lives at
  `modifier.properties.inputs.Socket_2.value`, with `.type` (VALUE/ATTRIBUTE) and
  `.attribute_name` beside it. Go through `bpy::write_modifier_input` /
  `read_modifier_input`, which fall back to the old address on 5.0/5.1. This hits
  `geo_modifier_set` and `obj_shade` (its "Smooth by Angle" modifier).
- **Blender 5.2 made `GreasePencilLayer.channel_color` read-only.** The writable
  per-layer colour there is `tint_color` (+ `tint_factor`); `gp_add_layer` reports
  which one it used as `color_applied_as`.
- **Blender 5.2 refuses a misplaced CYCLES F-Curve modifier *silently*.** A CYCLES
  modifier is only ever valid as the FIRST one on a curve. 5.0/5.1 declined by
  returning `None` from `modifiers.new()`; 5.2 instead **creates** it and flags
  `is_valid == False` — it sits in the stack, every `setattr` on it succeeds, and it
  does nothing. That is a failure dressed as success, so `new_fmodifier()` checks
  both shapes, removes the dead modifier again and errors. Verified live on 5.2.2:
  CYCLES after NOISE gives `[('NOISE', True), ('CYCLES', False)]`.
- **Blender 5.0 removed `Action.fcurves`.** Slotted actions since 4.4: curves live
  in `action.layers[].strips[].channelbag(slot).fcurves`. Any curve read goes only
  through `bpy::action_fcurves()`, otherwise `AttributeError` at runtime.
- **`bpy` is not thread-safe.** Handlers execute on Blender's main thread
  (the addon's timer), with the GIL held. Don't spawn threads that touch `bpy`.
- **No panics.** `unwrap()`/`expect()`/`let _ =` on fallible operations are forbidden —
  a crash inside Blender takes down more than just the tool. Errors go through `?`.

## Guard tests (`mod.rs`, `#[cfg(test)]`)

- `names_are_unique` — `inventory` happily accepts a duplicate name, and `find()` then
  silently takes whichever one the linker put first.
- `every_tool_is_well_formed` — non-empty name and description, schema is a JSON object. This is
  what the model sees in `tools/list`.
- `no_failure_is_reported_as_success` — scans the sources and fails the build if the
  `Ok(json!({"error": ...}))` anti-pattern comes back. The compiler doesn't catch this.

(The parity test `parity.rs` was a safety net during the migration: it compared the schema from the Rust type
with the one the catalog promised. Removed together with `mcp_catalog::get_tools()` — there's nothing
left to compare against, the registry became the single source.)

## Migration status — COMPLETE

- [x] Registry core; `bpy_dispatch` removed
- [x] All tools in the registry (one file per domain); the exact count is in `tools/list`,
      generated from the registry. At the time of writing — 233 scene tools (+ 5 process-control
      tools in `src/control.rs` and 3 knowledge tools in `src/kdb.rs`, both server-side)
- [x] Python monolith demolished: addon 5976 → 379 lines, only the bridge + UI remain
- [x] `mcp_catalog::get_tools()` removed (1272 lines of schemas); the server builds `tools/list`
      directly from the registry. Only resources and prompts remain in the catalog.
- [x] No legacy fallback: an unknown tool name is an error, not a silent fallback
- [x] Transport: ZMQ → named pipe (`ipc.rs`), pure Rust, no FFI and no TCP ports
- [x] Error contract unified; the guard test keeps the anti-pattern from coming back
- [x] Clippy clean across all targets

## What was broken on Blender 5 (and fixed)

The old Python was written for Blender 4.x. All of these tools **crashed** on Blender 5:

- **Grease Pencil** rewritten in 4.3: `GPENCIL` → `GREASEPENCIL`, `layer.info` → `.name`,
  `frame.strokes` → `frame.drawing.strokes`, `obj.grease_pencil_modifiers` merged into
  `obj.modifiers`, enum `GP_*` → `GREASE_PENCIL_*`. All names checked against the live
  Blender 5 enum (an agent made up four of them — verify).
- **VSE** renamed in 4.4: `sequence_editor.sequences` → `.strips`. All `vse_*`.
- **`Action.fcurves` removed** in 5.0 → `bpy::action_fcurves()`.
- `drv_add` crashed on any array property: `driver_add(path, -1)` returns
  a **list** of F-Curves, not a single one.
- `phys_update_particles` crashed with `IndexError` even when the system name was passed
  (eager evaluation of the default value in `p.get(k, obj.particle_systems[0].name)`).
- The `musgrave` node was removed in 4.1 → `ShaderNodeTexNoise`.

## Check when working on this code

- **Don't guess the Blender API.** There's a live Blender: ask it for the enum/attribute
  (`blender.exe --background --python probe.py`) instead of inferring by analogy.
- **`scene_screenshot` viewport/window modes can only be tested in the GUI.** They
  need an OpenGL context; `mode: "render"` works in `--background` through the camera.
  The `window` mode once passed `screen.screenshot` a `full` argument that does not
  exist in Blender 5. Viewport and window modes were verified in the GUI.
