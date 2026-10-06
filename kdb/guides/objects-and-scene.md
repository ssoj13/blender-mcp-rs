---
area: guide
title: Objects and scene — creating, transforming, organising
version: 5.2
---

# Objects and scene

Everything that treats an object as a whole: making it, moving it, baking that move
into the mesh, parenting, duplicating, joining, splitting, shading, collections,
instancing, plus the scene settings and cleanup around them. Editing *inside* a mesh
(vertices, faces, UVs) is a different area. Verified against Blender 5.2.2 LTS.

## The tools

| Tool | What it is for |
|---|---|
| `obj_create_primitive`, `obj_delete` | Add a cube/sphere/cylinder/plane/cone/torus/monkey/circle/grid/icosphere; remove objects |
| `obj_update` | Move, rotate, scale, show/hide. Changes the object transform only |
| `obj_apply_transform` | Bake loc/rot/scale into the mesh and reset the transform to identity |
| `obj_parent` | Set parent-child relationships |
| `obj_duplicate` | Copy an object, sharing its mesh data (`linked`) or not |
| `obj_join` | Merge several meshes into one; `target` survives |
| `obj_separate` | Split one mesh by loose parts, material slot, or selection |
| `obj_origin` | Move the origin to the geometry or to the 3D cursor |
| `obj_shade` | Smooth / flat / auto-smooth-by-angle |
| `obj_set_selection`, `obj_get_selection` | Set and read selection + active object |
| `col_create`, `col_delete`, `col_rename`, `col_move_to`, `col_list` | Collections |
| `obj_instance_to_points` | Instance one object over another's verts or faces |
| `obj_make_instances_real` | Turn those instances into ordinary objects |
| `util_align`, `util_distribute`, `util_snap_to_ground` | Layout |
| `scene_set`, `scene_get_info` | Frame range, current frame, fps — write and read |
| `scene_list_unused`, `scene_purge_unused` | Find and drop orphan data-blocks |
| `scene_detect_issues`, `scene_auto_fix` | Find geometry problems, then fix them |

## How to do the usual things

### Create primitives

`obj_create_primitive` takes `type`, optional `name`, `location`, `scale` — or a
`primitives` array (`[{type, name?, location?, scale?}, ...]`) to make several in one
call. `type` is one of `cube`, `sphere`, `cylinder`, `plane`, `cone`, `torus`,
`monkey`, `circle`, `grid`, `icosphere`; any other value is rejected by name. `scale`
sets the *object's* scale, not the mesh size: the mesh is still the default
2 m cube and the object is stretched. See `obj_apply_transform`.

### Transform, then bake the transform

`obj_update` sets `location`, `rotation` (Euler, **radians**), `scale` and `visible` —
each a 3-element array except `visible`, which hides in viewport and render together.
Batch through `updates`: `{"updates": [{"name": "Ball", "location": [0, 0, 1.5]},
{"name": "Floor", "visible": false}]}`.

`obj_update` moves an object; to bake the transform into the mesh use
`obj_apply_transform`. It takes `name` (or an `objects` batch) plus `location`,
`rotation`, `scale` booleans, all defaulting to true like Blender's own operator:
`{"objects": [{"name": "Ball", "location": false, "rotation": false, "scale": true}]}`.

**Un-applied scale is the usual cause of "the modifier looks wrong".** Bevel width,
Solidify thickness and Array offset are local-space measurements, so an object scaled
2x in X bevels twice as wide in X as in Y. Verified on 5.2.2: a cube with scale
`(2, 1, 1)` has dimensions `(4, 2, 2)`; after `transform_apply(scale=True)` the scale
reads `(1, 1, 1)` and the dimensions are unchanged — the stretch now lives in the mesh
and modifier widths become uniform. Apply scale before width-based modifiers and
before exporting. Applying `location` also drags the origin to the world origin: pass
`location: false` and place origins deliberately with `obj_origin`.

### Parent, duplicate, join, separate

`obj_parent` takes `parent` plus `children` (or an `operations` batch of those pairs,
e.g. `{"operations": [{"parent": "Rig_Root", "children": ["Body", "Head"]}]}`).
`keep_transform` defaults to true, setting the parent-inverse matrix so nothing jumps;
`false` snaps children into the parent's space.

`obj_duplicate` copies into the active collection. `linked: false` (default) copies
the mesh data too — an independent object. `linked: true` points both at one mesh
data-block: editing one edits all, and the file stays small — what you want for
repeated props. `offset` is relative to the source location, `new_name` renames the
copy, `duplicates` batches it.

`obj_join` merges `sources` into `target`: the target keeps its name and origin, the
sources are merged in and removed. Every source name is validated up front, so a typo
fails the call instead of silently producing different geometry. Joining fewer than
two compatible objects makes Blender return `{'CANCELLED'}` (verified on 5.2.2) and
the tool reports that as an error. `obj_separate` is the inverse, with `mode` — the
three identifiers this Blender's `mesh.separate` RNA accepts:

- `LOOSE` — one object per disconnected island. The usual choice.
- `MATERIAL` — one object per material slot.
- `SELECTED` — splits off the mesh's current edit-mode selection; set it with
  `mesh_select` first or there is nothing to split.

It reports the objects created, and restores the previous selection, active object and
mode afterwards.

### Origins

`obj_origin` moves the origin without moving the geometry.

- `{"name": "Ball", "mode": "GEOMETRY"}` — `center` is `MEDIAN` (default, vertex
  average) or `BOUNDS` (bounding-box centre); both read from this Blender's RNA.
- `{"name": "Door", "mode": "CURSOR", "cursor_position": [0, 0, 0]}` — with no
  viewport to click in, `cursor_position` is what makes cursor mode usable: it places
  the 3D cursor at that world point first and leaves it there.

A hinge or a turntable needs its origin on the axis of rotation; set it here rather
than compensating in the animation.

### Shading

`obj_shade` takes `name` (or an `objects` batch) and `mode`: `SMOOTH` / `FLAT` (with
`keep_sharp_edges`, default true, leaving Sharp-marked edges alone), or `AUTO_SMOOTH`
with `angle` in degrees (default 30) — smooth below the angle, faceted above, e.g.
`{"objects": [{"name": "Ball", "mode": "AUTO_SMOOTH", "angle": 45}]}`. Every primitive
Blender creates is faceted; this is what fixes that.

Know what AUTO_SMOOTH is on 5.2: `mesh.use_auto_smooth` was removed in 4.1 and
auto-smooth is now a geometry-nodes modifier. Verified on 5.2.2 — the call leaves a
modifier named "Smooth by Angle", type `NODES`, with the angle on socket `Input_1`.
So it shows up in `mod_list`, can be reordered or removed with `mod_delete`, and a
second AUTO_SMOOTH call updates that modifier instead of stacking another.

### Selection

`bpy.ops` reads selection and the active object. `obj_set_selection` takes `names` and
optional `active`, and deselects everything first — it sets the selection, it does not
extend it. `obj_get_selection` returns `selected`, `active`, `count`. Most tools here
do not need either: `obj_shade`, `obj_apply_transform`, `obj_join`, `obj_separate` and
`obj_origin` stage their own selection and restore yours. Set it explicitly for the
tools that read it — `obj_separate` with `mode: "SELECTED"`, and `scene_auto_fix`
without `objects`, which falls back to the selected meshes.

### Collections

`col_list` returns the hierarchy from the scene's root collection with the objects in
each. `col_create` takes `name` and optional `parent` (without it the collection is
linked to the scene root) and fails if the name is taken. `col_rename` takes `name`
and `new_name`; `col_delete` removes a collection, with `delete_objects` (default
false) deciding whether its objects go too.

`col_move_to` takes `object` or an `objects` array plus `collection`, creating the
collection if needed. It **unlinks the object from every collection it is in** before
linking it — a move, not a copy: `{"objects": ["Chair", "Table"], "collection":
"Furniture"}`.

### Instancing

`obj_instance_to_points` (`{"name": "Ground", "instance": "Tree", "instance_type":
"FACES"}`) parents `instance` to `name` and sets the base object's `instance_type`:
`VERTS` (default) or `FACES` — `NONE` and `COLLECTION` are the other identifiers this
Blender accepts. One copy appears per vertex or face, all sharing one mesh data-block.

The instance object is then hidden with the per-view-layer hide, not "Disable in
Viewports": the latter drops it from the depsgraph, so Blender generates no instances
and a later `obj_make_instances_real` succeeds while creating nothing.

`obj_make_instances_real` converts them into ordinary editable objects — do that only
when you need to vary them, since instances are far cheaper.

### Layout helpers

`util_align` and `util_distribute` work on **object origins**, not bounding boxes.

- `util_align` — `names`, `axis` (`X`/`Y`/`Z`), `align_to`: `MIN`, `MAX`, `CENTER`,
  `CURSOR`, or a number for an explicit coordinate. Omitted, it averages the current
  positions.
- `util_distribute` — `names`, `axis`, optional `spacing`. Sorts by current position
  on that axis, then lays them out from the lowest at `spacing`, or evenly between
  first and last. Needs at least two objects.
- `util_snap_to_ground` — `name`/`names`, optional `target_z` (default 0) and
  `align_bottom` (default true: the world-space bounding-box bottom lands on
  `target_z`, not the origin). It refreshes the dependency graph first, so it never
  snaps against a stale matrix — a real hazard headless, where nothing else forces
  that update.

### Scene settings

`scene_get_info` reads fps, frame range, resolution, engine, view transform and
counts; `scene_set` writes frame range, current frame and fps
(`{"frame_start": 1, "frame_end": 240, "fps": 24}`). `fps_base` is the divisor —
`fps: 30` with `fps_base: 1.001` is 29.97. A `frame_end` below `frame_start` is an error, not a silently swapped range. Set the
range before baking a simulation or keying animation; that is what those readers use.

### Cleanup

- `scene_list_unused` — orphan data-blocks with no users, by kind (meshes, materials,
  images, textures, actions, node groups, curves, armatures). Read it before purging.
- `scene_purge_unused` — drops them recursively, reporting `removed` and
  `remaining_data_blocks`. Deleting objects does not delete their meshes; this does,
  and it is what actually shrinks the file.
- `scene_detect_issues` — non-manifold edges, boundary edges and inverted normals per
  mesh, plus missing image files and orphan data; optional `objects` narrows it.
  Inverted normals are reported only for closed manifold meshes, where the test is
  valid, so open meshes are never falsely flagged.
- `scene_auto_fix` — `fix_normals`, `fix_scale`, `remove_doubles` (with
  `merge_distance`, default 0.0001), optional `objects`. **Every flag defaults to
  false**, so a bare call fixes nothing. Without `objects` it uses the selected
  meshes, falling back to every mesh in the scene. `fix_scale` is
  `obj_apply_transform` on scale under another name.

Usual order: `scene_detect_issues` → `scene_auto_fix` with the matching flags →
`scene_detect_issues` again → `scene_purge_unused`.

## Traps

- Rotations are radians in `obj_update`; `obj_shade`'s `angle` is the one in degrees,
  and `obj_update` with `visible: false` hides in viewport *and* render.
- `obj_join` deletes the sources and merges their material slots into the target's;
  slot indices shift, so re-read `mat_slots` afterwards.
- `obj_apply_transform` defaults to `location: true`, which moves the origin to the
  world origin.
- `AUTO_SMOOTH` is a modifier: applying or deleting the modifier stack takes the
  smoothing with it.
## See also

- `kdb_guide` `getting-started` — session shape, batching, the error contract.
- `kdb_search` `"mesh_edit"`, `"mod_add bevel"`, `"mat_slot_add"` — what happens
  inside the objects this guide creates.
- `kdb_read` `api/ops/object.md` (every `bpy.ops.object.*` signature and enum in this
  exact Blender) and `api/types/object.md` (the `Object` properties behind it).
