---
area: guide
title: Modelling — meshes, edits, modifiers, curves
version: 5.2
---

# Modelling

Everything that changes geometry: building a mesh from raw data, selecting parts of
it, running topology edits, stacking modifiers, booleans, curves and text, remeshing,
and shape keys. UVs, colour attributes and vertex groups live in the sibling guide
`uv-and-attributes`.

The mesh-editing tools (`mesh_create`, `mesh_select`, `mesh_edit`,
`mesh_transform_selection`, `mesh_mark`) never enter Edit Mode — they drive `bmesh`
on the mesh data directly, so they work headless and address elements by index.

## The tools

| Tool | What it is for |
|---|---|
| `mesh_create` | build a mesh from `verts`/`edges`/`faces` (`from_pydata`). For a cube/sphere/etc. use `obj_create_primitive` |
| `mesh_get_data` | counts, and optionally `vertex_positions` / `face_indices` / `vertex_normals` |
| `mesh_set_data` | overwrite every vertex position; count must match exactly. Moving *part* of a mesh is `mesh_transform_selection` |
| `mesh_select` | store a selection on the mesh (by index or predicate). Everything below reads it |
| `mesh_edit` | batch of `bmesh` topology ops against that selection |
| `mesh_transform_selection` | move/rotate/scale the selection; object-level transforms are `obj_update` |
| `mesh_mark` | seam / sharp / crease on the selected edges |
| `mesh_subdivide`, `mesh_extrude` | convenience wrappers over `bpy.ops`, whole-mesh only — prefer `mesh_edit` |
| `mod_add` / `mod_list` / `mod_update` / `mod_apply` / `mod_delete` | the modifier stack |
| `util_boolean`, `util_apply_booleans` | boolean modifier between two meshes, optionally applied |
| `sculpt_remesh`, `sculpt_symmetrize`, `sculpt_mask` | whole-mesh sculpt operations (no brush strokes exist headless) |
| `curve_create`, `curve_add_point`, `curve_update_point` | bezier/nurbs curves |
| `text_create`, `text_update` | 3D text objects. For script/driver text buffers see `text_datablock_create` |
| `mesh_convert_to` | curve/text → mesh |
| `shapekey_add` / `shapekey_list` / `shapekey_update` / `shapekey_remove` | shape keys |

## How to do the usual things

### Build a mesh from raw data
`mesh_create` takes `verts` (required, `[x,y,z]` each), plus optional `edges`
(`[a,b]` index pairs — only needed for loose edges) and `faces` (index lists, 3+ per
face, ngons fine). Winding order sets the face normal. Out-of-range indices are
rejected before Blender sees them; the result reports `had_invalid_geometry` when
`validate()` had to strip something.

```json
{"name": "Quad", "location": [0, 0, 0],
 "verts": [[0,0,0], [1,0,0], [1,1,0], [0,1,0]], "faces": [[0,1,2,3]]}
```

### Select, then edit
`mesh_select` writes select flags into the mesh data and they survive to a later
call — select in one call, edit in the next. `mode` is `VERT`, `EDGE` or `FACE`.
Give either `indices` (explicit, takes priority) or `by`:

| `by` | Domain | Extra params |
|---|---|---|
| `all`, `none`, `invert` | any | — |
| `material_index` | FACE only | `material_index` |
| `normal` | FACE only | `normal_axis` (required), `normal_angle` in degrees (default 45) |
| `bbox` | any | `bbox_min`, `bbox_max` — local space; an element needs *all* its verts inside |
| `non_manifold`, `boundary`, `loose` | VERT/EDGE only | — |

`extend: true` adds to the current selection instead of replacing it (ignored for
`invert`, which always flips whatever is there). After every change the selection is
flushed across all three domains exactly as interactive Edit Mode does — selecting
the four verts of a face also selects that face and its edges.

```json
{"name": "Cube", "mode": "FACE", "by": "normal", "normal_axis": [0,0,1], "normal_angle": 30}
```

### mesh_edit: the op language
`ops` is an ordered list; each op runs against the selection the previous one left,
and every op that creates geometry re-points the selection at what it created — so
extrude-then-inset-then-bevel is a single call.

| `op` | Needs selected | Params |
|---|---|---|
| `extrude` | faces | `extrude_mode`: `region` (default) / `individual`; `vector` `[x,y,z]` **or** `distance` (along the average normal, or each face's own normal when individual) |
| `inset` | faces | `thickness`, `depth`, `individual` |
| `bevel` | edges or verts | `affect`: `edges` (default) / `verts`; `thickness` (= offset), `segments` (default 1) |
| `subdivide` | edges | `cuts` (default 1), `smoothness` 0..1 (default 0) |
| `merge` | verts | `merge_distance` (default 0.0001) — merge by distance |
| `delete` | per mode | `delete_mode` (required): `verts`, `edges`, `faces`, `only_faces` |
| `duplicate` | anything | — |
| `spin` | anything | `vector` = axis (default `[0,0,1]`), `center` (default selection median), `angle` in degrees (default 360), `steps` (default 12), `duplicate` |
| `bridge` | edges | — two boundary edge loops |
| `triangulate` | faces | — |
| `recalc_normals`, `flip_normals` | faces | — |
| `dissolve` | per mode | `dissolve_mode` (required): `verts`, `edges`, `faces`, `limited` (+ `angle_limit` in degrees, default 5) |

Mind the vocabulary split: to extrude **faces** use `op: "extrude"`; to bevel
**edges** use `affect: "edges"`; to round **corners** use `affect: "verts"`; to cut
**edges** use `subdivide` (select faces or verts and the flush hands it their edges).

```json
{"name": "Cube", "ops": [
  {"op": "inset", "thickness": 0.1},
  {"op": "extrude", "distance": 0.5},
  {"op": "bevel", "affect": "edges", "thickness": 0.02, "segments": 3}
]}
```

### Move part of a mesh
`mesh_transform_selection` takes `translate`, `rotate_angle` + `rotate_axis`, and/or
`scale` — at least one is required — plus `pivot` (default: median of the selected
verts) and `space` (`local` default, or `global`, where the vectors are world-space
and converted through the object's `matrix_world`).

### Marking edges
`mesh_mark` sets `seam`, `sharp` and/or `crease` (0..1) on the selected edges; at
least one is required. Seams are what `mesh_unwrap` needs (see `uv-and-attributes`),
crease is what SUBSURF reads, sharp pairs with `obj_shade`.

### Convenience tools vs mesh_edit
`mesh_subdivide` (`cuts`) and `mesh_extrude` (`direction`) go through `bpy.ops` in
Edit Mode and act on the whole mesh — `mesh_subdivide` selects everything first,
destroying any stored selection. Use them for a quick global pass only; anything
targeted belongs in `mesh_edit`.

### Modifiers
`mod_add` takes `name` (the object), `type`, optional `mod_name`, and a free-form
`properties` object; or a batch under `modifiers: [{name, type, ...}, ...]`.
Useful types and their key properties on 5.2:

- `SUBSURF` — `levels`, `render_levels`, `subdivision_type` (`CATMULL_CLARK`/`SIMPLE`), `uv_smooth`, `boundary_smooth`
- `BEVEL` — `width`, `segments`, `affect` (`VERTICES`/`EDGES`), `limit_method` (`NONE`/`ANGLE`/`WEIGHT`/`VGROUP`), `angle_limit`
- `MIRROR` — `use_axis` (3 bools), `use_clip`, `mirror_object`, `use_mirror_u`/`use_mirror_v`, `merge_threshold`
- `ARRAY` — `count`, `fit_type`, `use_relative_offset`, `relative_offset_displace`, `constant_offset_displace`, `offset_object`, `use_merge_vertices`, `start_cap`/`end_cap`
- `SOLIDIFY` — `thickness`, `offset`, `use_rim`, `use_even_offset`, `vertex_group`
- `DECIMATE` — `decimate_type` (`COLLAPSE`/`UNSUBDIV`/`DISSOLVE`), `ratio`, `angle_limit`, `vertex_group`
- `WELD` — `merge_threshold`, `mode`, `vertex_group`
- `SCREW` — `axis`, `angle`, `steps`, `screw_offset`, `use_merge_vertices`
- `REMESH` — `mode` is `BLOCKS`/`SMOOTH`/`SHARP`/`VOXEL` (the destructive equivalent is `sculpt_remesh`)
- `NODES` — geometry nodes; its inputs are set with `geo_modifier_set`, not `mod_update`
- `MASK`, `ARMATURE`, `HOOK` — driven by a `vertex_group` (create it with `vgroup_add`)

`mod_update` takes `name`, `modifier`, `properties`. `mod_apply` and `mod_delete`
take `name` + `modifier`, or batches under `applications` / `deletions`. `mod_list`
reports each modifier's name, type, `show_viewport` and `show_render`.

### Booleans
`util_boolean(object, target, operation, solver, apply)` adds a BOOLEAN modifier to
`object` pointing at `target`. `operation` is `UNION` / `DIFFERENCE` (default) /
`INTERSECT`. `solver` defaults to `EXACT`; on 5.2 the RNA enum is `FLOAT`, `EXACT`,
`MANIFOLD`. Pass `apply: true`, or leave the stack intact and later collapse every
boolean on an object with `util_apply_booleans` (`name`, or `names` for a batch).

### Curves and text
`curve_create` — `type` (`BEZIER`/`NURBS`), `points` (list of `[x,y,z]`),
`dimensions` (`2D`/`3D`), `bevel_depth`, `extrude`, `location`, `name`.
`curve_add_point` appends to `spline_index` (default 0); `curve_update_point` edits
`point_index` — `position`, `handle_left`, `handle_right`, `handle_type`
(`AUTO`/`VECTOR`/`FREE`/`ALIGNED`, bezier splines only).

`text_create` — `body`, `size`, `extrude`, `bevel_depth`, `font` (a path),
`align_x`/`align_y` (`LEFT`/`CENTER`/`RIGHT`/`JUSTIFY`/`FLUSH`); `text_update`
changes the same fields on an existing FONT object. That is a 3D object; a script or
driver-expression buffer is `text_datablock_create` instead.

`mesh_convert_to` turns CURVE or FONT objects into real meshes (`name`, or `names`).
A curve with no `bevel_depth` and no `extrude` converts to edges only — give it one
of the two first if faces are wanted.

### Remesh and symmetry
`sculpt_remesh` runs in Object Mode. `method: "voxel"` (default) uses `voxel_size`
(smaller = denser; omitted keeps the mesh's own `remesh_voxel_size`) and
`voxel_adaptivity` (0..1, curvature-driven density). `method: "quadriflow"` does
quad retopology and takes exactly one of `target_faces`, `target_ratio`,
`target_edge_length`, plus `quad_preserve_symmetry`. Both are destructive: UVs,
vertex groups and shape keys do not survive either.

`sculpt_symmetrize(name, direction, merge_distance)` mirrors one half onto the
other; `direction` is `NEGATIVE_X` (default), `POSITIVE_X`, `NEGATIVE_Y`, …
`sculpt_mask(name, value)` flood-fills the `.sculpt_mask` attribute — 1.0 masks
everything, 0.0 clears it.

### Shape keys
`shapekey_add(name, key_name, from_mix, value)` — the first key added becomes the
Basis. `shapekey_update` sets `value`, `slider_min`/`slider_max`, `mute`;
`shapekey_list` reports them; `shapekey_remove` takes `key_name`. Animate a key by
keying its `value` with `anim_set_keyframe`.

## Traps

- `mesh_subdivide` and `mesh_unwrap` select the whole mesh as a side effect. Run them
  *before* building a `mesh_select` selection, never after.
- `subdivide` inside `mesh_edit` needs **edges**: "no edges selected" means the
  selected verts do not span a complete edge.
- `delete` and `dissolve` require their mode parameter and raise when that domain's
  selection is empty — there is no silent no-op. A failed op names its index in the
  batch, e.g. `mesh_edit: op 2 (bevel) failed: ...`.
- Boolean `solver: "FAST"` is a 4.x value; on 5.2 RNA rejects it outright — the enum
  is `FLOAT`, `EXACT`, `MANIFOLD`.
- Un-applied object scale silently breaks BEVEL/SOLIDIFY/ARRAY widths; run
  `obj_apply_transform` with `scale: true` first.
- Every primitive is flat-shaded; `obj_shade` is the only way to smooth it.
- `mesh_set_data` refuses a `vertex_positions` list whose length differs from the
  mesh's vertex count.
- `sculpt_remesh` discards UVs and vertex groups — unwrap and weight *after*
  remeshing.
- `mesh_edit` ops address only the active object's own mesh; to cut one mesh with
  another use `util_boolean`.

## See also

`kdb_guide`: `uv-and-attributes` (UV maps, named attributes, vertex groups, baking).
`kdb_search` queries worth running: `bmesh.ops`, `BevelModifier`, `quadriflow_remesh`,
`bpy.ops.object.convert`, `remesh_voxel_size`.
