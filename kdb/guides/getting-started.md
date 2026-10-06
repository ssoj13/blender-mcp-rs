---
area: guide
title: Getting started — how a session with this server works
version: 5.2
---

# Getting started

This is the first guide to read. It covers the shape of a whole session: bringing
Blender up, looking at the scene instead of assuming what is in it, doing work in
batched calls instead of twenty round-trips, what a failure looks like, when to drop
down to raw Python, and how to use the knowledge base itself. Everything here is
verified against Blender 5.2.2 LTS.

## The tools

| Tool | What it is for |
|---|---|
| `blender_launch` | Start Blender and wait until the add-on has connected. No-op if one is already connected |
| `blender_status` | Is a Blender running, is the add-on connected, which executable. The first call to make when anything is unclear |
| `blender_quit` | Ask Blender to exit gracefully; falls back to a kill if it will not go |
| `blender_kill` | Force-terminate now. For a hung Blender that `blender_quit` cannot reach |
| `blender_restart` | Quit (or kill) and launch a fresh one. Use it to get back to a known state |
| `scene_screenshot` | Look at the scene as an image. GUI only, except `mode: "render"` |
| `scene_get_info` | Scene name, fps, frame range, resolution, engine, view transform, object/material counts |
| `scene_get_stats` | Heavier: per-type object counts, vert/edge/face/tri totals, data-block counts, memory estimate |
| `obj_list` | The real object names. Optional `type` / `collection` filter |
| `obj_get` | One object in detail: transform, parent, children, mesh counts, material slots |
| `obj_get_selection` | Which objects are selected and which is active — the state `bpy.ops` reads |
| `util_execute_python` | The escape hatch, for what no tool covers. Not a substitute for the tools |
| `kdb_guide` | These guides. No argument lists them; `area` reads one |
| `kdb_search` | Rank-search the whole knowledge base (guides, API dump, release notes, manual) |
| `kdb_read` | Read one knowledge-base document by the `path` a `kdb_search` hit reports |

## How to do the usual things

### Start a session

1. `blender_status`. If `connected` is true there is already a Blender to work in;
   do not launch a second one.
2. Otherwise `blender_launch`. `background: false` (the default) opens a normal
   Blender window; `background: true` runs headless.
3. The call returns only once the add-on has connected — up to 45 s, because a
   Blender with many linked libraries is slow to start. A launch that succeeds
   reports `{"launched": true, "connected": true, "pid": N}`. If Blender started but
   never connected, the call fails and says so; the add-on is probably not deployed.
4. When finished, `blender_quit`. A headless Blender has no event loop to run a
   graceful-quit timer on, so it is killed directly — that is expected, not a fault.

Choose background vs GUI by whether you need to *see* anything:

- **GUI** — you will call `scene_screenshot` to check your work, or a human is
  watching. Slower to start, but the viewport exists.
- **background** — pure data work: build geometry, set materials, render to a file,
  export. Faster, no window. `scene_screenshot` in its default `viewport` mode fails
  here with "Cannot use OpenGL render in background mode", and `mode: "window"` fails
  its operator poll. Only `mode: "render"` works headless, because that is a real
  render rather than a viewport grab.

### Look at the state; do not assume it

The scene is not necessarily what you last told it to be — the user may have moved
things, and a `.blend` may have been loaded since. Before acting on named objects,
read:

- `obj_list` — the authoritative names. Guessing "Cube" when the object is
  "Cube.001" costs a failed call. `{"type": "MESH"}` narrows it.
- `obj_get` — everything about one object, including `parent`, `children` and
  `mesh.materials` (the slot list, with `null` for an empty slot).
- `scene_get_info` — fps and frame range before keying or baking anything;
  `engine` and `view_transform` before judging a render.
- `scene_get_stats` — before a heavy operation, so you know whether you are about to
  subdivide 500 or 5 000 000 triangles.
- `scene_screenshot` — the only tool that shows what the result actually looks like.
  Use it after building something, not instead of reading the data.

`scene_screenshot` takes `mode` (`viewport`, `window`, `render`), `shading`
(`wireframe`, `solid`, `material`, `rendered`), `engine`, `width`, `height`
(512 x 512 by default) and returns the image to you directly:

```json
{"mode": "viewport", "shading": "material", "width": 800, "height": 600}
```

### Batch: one call instead of twenty

Most write tools have a single-item form and a batch form, and the batch parameter is
the one to reach for. MCP round-trips dominate the cost of a session; twenty
`obj_update` calls take twenty times as long as one with twenty `updates`.

| Batch parameter | Tools |
|---|---|
| `primitives` | `obj_create_primitive` |
| `updates` | `obj_update` |
| `names` | `obj_delete`, `util_snap_to_ground`, `util_randomize_transform`, `util_apply_booleans`, `mat_delete`, `anim_clear` |
| `objects` | `obj_shade`, `obj_apply_transform`, `obj_separate`, `obj_origin`, `col_move_to`, `mat_assign` |
| `duplicates` | `obj_duplicate` |
| `operations` | `obj_parent`, `obj_join` |
| `materials` | `mat_create` |
| `keyframes` | `anim_set_keyframe` |
| `modifiers` / `applications` / `deletions` | `mod_add` / `mod_apply` / `mod_delete` |

```json
{"primitives": [
  {"type": "cube",     "name": "Base", "location": [0, 0, 0]},
  {"type": "cylinder", "name": "Post", "location": [0, 0, 1], "scale": [0.2, 0.2, 2]},
  {"type": "sphere",   "name": "Knob", "location": [0, 0, 3]}
]}
```

The batch form also changes the error contract, deliberately: a per-item failure is
**data**, reported in the `results` array as `{"name": ..., "error": ...}` alongside a
count of the ones that worked. The same failure in single-item form is a hard error.
So a batch of twenty where one name was misspelled still does the other nineteen —
read the `results` array rather than assuming the whole call landed.

### When a call fails

The contract is strict and worth relying on:

- **A failed call is an error, never a success whose text mentions a problem.** No
  tool returns `{"error": ...}` as its whole result; there is a guard test for it.
- **An operator that quietly does nothing is a failure too.** Blender's `bpy.ops`
  return `{'CANCELLED'}` instead of raising when their poll passed but nothing
  happened — verified on 5.2.2: `object.join` with a single object and
  `object.transform_apply` with nothing selected both do exactly that. The tools
  check for `FINISHED` and turn everything else into an error.
- **A rejected enum is answered with the real values.** Enum identifiers are read
  from this Blender's own RNA, not from a hardcoded table, so the message names what
  is actually accepted — `invalid type 'PARTS'. This Blender accepts: SELECTED,
  MATERIAL, LOOSE`. Take the list from the error; do not guess again.
- **A missing object is answered with the name it could not find**, e.g.
  `object not found: Cube.003`. Call `obj_list` for the real names rather than trying
  variants.

### When to use `util_execute_python`

Use it when no tool covers what you need — reading an RNA property nothing exposes,
probing whether an add-on is registered, a one-off data-level fix. It runs on
Blender's main thread like every other tool, with `bpy` already in scope.

```json
{"code": "__result__ = [m.name for m in bpy.data.objects['Cube'].modifiers]"}
```

Rules that make it behave:

- Return data by assigning to `__result__`. It is serialised with `json.dumps`, so it
  must be JSON-serialisable — assign names and numbers, not `bpy` structs.
- Only `bpy` is pre-imported; `import` anything else inside the code.
- Prefer the real tool where one exists. The tools validate enums, check operator
  results, restore selection and mode, and know the 5.x API changes; raw Python does
  none of that.

### Use the knowledge base before guessing

`kdb_search`, `kdb_read` and `kdb_guide` work whether or not Blender is running, and
answer faster than a failed tool call.

- `kdb_guide` with no argument lists the guides; with `area` (e.g.
  `{"area": "objects-and-scene"}`) it returns one whole. Start here in an unfamiliar
  area rather than probing tools one at a time.
- `kdb_search` takes `query`, optional `area` (matched as a substring: `guide`,
  `api-ops`, `api-types`, `api-nodes`, `release-notes`, `manual-modeling`, …) and
  `limit` (default 8, max 25). Use literal keywords — an operator name, an enum
  identifier, a node label — not a sentence. Each hit carries a `path`.
- `kdb_read` takes that `path` and returns the document, 400 lines at a time; pass
  `offset` to continue.

Search it **before** guessing an enum value or a socket name, and **after** a failure
that smells like a version difference. The `api-*` documents were dumped from this
exact Blender, so an operator signature or enum identifier found there is true here by
construction.

## Traps

- A second `blender_launch` while one is connected is a no-op that reports
  `{"launched": false, "reason": "already connected"}` — not an error, and not a
  fresh scene. Use `blender_restart` for that.
- Headless has no OpenGL context. `scene_screenshot` fails there in `viewport` and
  `window` modes; this is exactly the breakage that survived 26 headless test runs.
  If you need to see the scene, launch with `background: false`.
- `blender_kill` loses unsaved work. Prefer `blender_quit`, and `file_save` before
  either if the scene matters.
- Batch calls swallow per-item failures into `results`. Read the count.
- `obj_list` reports `visible` from the object's real visibility flags, which is not
  the same as "you can see it" — a visible object can still be behind the camera or
  inside another.

## See also

- `kdb_guide` `objects-and-scene` — creating, transforming, parenting, grouping and
  cleaning up the things `obj_list` shows you.
- `kdb_guide` with no argument — the full list of area briefings.
- `kdb_search` queries worth running: `"operator did not finish"`, `"bpy.ops.object"`
  with `area: "api-ops"`, `"shade auto smooth"`.
- `kdb_read` `api/ops/object.md` — every `bpy.ops.object.*` signature in this Blender,
  for when you do reach for `util_execute_python`.
