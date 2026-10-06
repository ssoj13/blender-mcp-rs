---
area: guide
title: UVs and attributes — unwrapping, named data, weights
version: 5.2
---

# UVs and attributes

UV maps, named mesh attributes, colour attributes and vertex groups: the per-element
data layers a mesh carries alongside its geometry. This is what geometry nodes read
and write, what a shader's Color Attribute node samples, what a modifier's
`vertex_group` points at, and what `bake_texture` needs before it can put anything
into an image. Topology itself is the sibling guide `modelling`.

In Blender 5.2 all of these are one storage system: `mesh.attributes`. A UV map is a
`FLOAT2` attribute on the `CORNER` domain, a colour attribute is `FLOAT_COLOR` or
`BYTE_COLOR`, and `attr_list` shows every one of them, built-ins included.

## The tools

| Tool | What it is for |
|---|---|
| `uv_add`, `uv_list`, `uv_set_active`, `uv_remove` | UV maps as a set: create, inspect, choose the active/render one, delete |
| `mesh_unwrap` | actually generate UVs into the active map |
| `mesh_mark` | seams (and sharp/crease) on selected edges — what makes an unwrap usable |
| `mesh_get_uvs` | a summary only: `has_uvs`, active layer name, loop count. For real numbers use `attr_list` on that layer |
| `attr_add`, `attr_list`, `attr_set`, `attr_remove` | named attributes of any type/domain |
| `color_attr_add`, `color_attr_list`, `color_attr_set`, `color_attr_remove` | colour attributes (the 3.2+ replacement for vertex colours) |
| `vgroup_add`, `vgroup_list`, `vgroup_set`, `vgroup_get`, `vgroup_rename`, `vgroup_remove` | vertex groups and their weights |
| `rig_auto_weights` | fill vertex groups from an armature automatically |
| `bake_texture` | render shading into an image through a UV layout |
| `geo_modifier_set` | feed a Geometry Nodes modifier's inputs by socket name |

Note the parameter split: the UV/attribute/vgroup tools take `object`, while the
mesh-editing tools (`mesh_unwrap`, `mesh_get_uvs`, `mesh_mark`, `mesh_select`) take
`name`.

## How to do the usual things

### Unwrap a mesh
1. Select the edges that should be seams — `mesh_select` with `mode: "EDGE"`, then
   `mesh_mark` with `seam: true`.
2. `mesh_unwrap(name, method, angle)`. `method` is `angle_based` (default),
   `conformal`, or `smart`. The first two map to `bpy.ops.uv.unwrap`'s
   `ANGLE_BASED` / `CONFORMAL`; `smart` runs `smart_project` with `angle` as its
   angle limit in degrees (default 66).
3. `uv_list` to confirm which layer is active and which one renders.

Without seams, `angle_based` produces one stretched island and Blender warns
"Unwrap failed to solve 1 of 1 island(s)". `smart` needs no seams and is the safe
default for props and for bake targets.

```json
{"name": "Cube", "mode": "EDGE", "indices": [0, 2, 4, 6]}
{"name": "Cube", "seam": true}
{"name": "Cube", "method": "smart", "angle": 66}
```

### Several UV maps
`uv_add(object, name, active)` creates one — `active` defaults to true. A mesh holds
at most 8; the 9th returns a clear error rather than a silent failure.
`uv_set_active(object, name, render)` chooses the active layer (what unwrapping and
baking act on) and, with `render: true`, the one sampled at render time. These are
two independent flags, and a bake reading a different layer from the one you
unwrapped is the classic "why is my bake wrong".

Typical second-set workflow: `uv_add` a `Lightmap` layer, `uv_set_active` it,
`mesh_unwrap` with `smart`, then `uv_set_active` the original back for shading.

### Read or write actual UV coordinates
`mesh_get_uvs` only reports counts. The numbers live in the attribute of the same
name: `attr_list(object)` shows `UVMap` as `FLOAT2` / `CORNER` with a sample, and
`attr_set` writes it — one `[u, v]` per loop, indexed by **loop** (corner) index,
not vertex index.

```json
{"object": "Cube", "name": "UVMap", "indices": [0, 1, 2, 3],
 "values": [[0,0], [1,0], [1,1], [0,1]]}
```

### Named attributes
`attr_add(object, name, type, domain)`.

- `type` on 5.2: `FLOAT`, `INT`, `BOOLEAN`, `FLOAT_VECTOR`, `FLOAT_COLOR`,
  `QUATERNION`, `FLOAT4X4`, `STRING`, `INT8`, `INT16_2D`, `INT32_2D`, `FLOAT2`,
  `FLOAT4`, `BYTE_COLOR`. Default `FLOAT`.
- `domain` on a mesh: `POINT` (vertex), `EDGE`, `FACE`, `CORNER` (loop). Default
  `POINT`. Blender's enum also lists `CURVE`, `INSTANCE` and `LAYER`, but a Mesh
  rejects those with its own error.

A wrong value is checked against live RNA and comes back with this Blender's real
list, never guessed.

`attr_set(object, name, values, indices)` writes: `indices` addresses items in the
attribute's own domain (vertex for POINT, edge for EDGE, face for FACE, loop for
CORNER); omit it to write every item in order, in which case `values` must cover all
of them. A scalar type takes a plain number or bool, a vector/colour type a JSON
array of the right length, `STRING` a JSON string.

`attr_list(object)` is the full inventory with a sample of real values, and it
includes Blender's own built-ins — on 5.2 a plain cube reports `position`
(FLOAT_VECTOR/POINT), `.corner_vert` and `.corner_edge` (INT/CORNER), `.edge_verts`
(INT32_2D/EDGE), `sharp_face` (BOOLEAN/FACE), `.select_vert`/`.select_edge`/
`.select_poly`, plus `UVMap` once the mesh has UVs. Names beginning with a dot are
internal; do not write them. `attr_remove(object, name)` deletes one.

### Colour attributes
`color_attr_add(object, name, type, domain)` — `type` is `FLOAT_COLOR` (32-bit
linear, default) or `BYTE_COLOR` (8-bit sRGB storage); `domain` is `POINT` (per
vertex, default) or `CORNER` (per face corner, so a colour can differ across a UV
seam). `color_attr_set(object, name, colors, indices)` writes `[r, g, b, a]` per
item, indexed by vertex for POINT and by loop for CORNER. `color_attr_list` reports
name, data type, domain and a sample.

These are the layers a shader's Color Attribute node and geometry nodes' Named
Attribute read. They also show up in `attr_list`, since they are ordinary
attributes — `color_attr_*` is the narrower, colour-aware view.

### Vertex groups and weights
`vgroup_add(object, name)` creates a group (MESH and LATTICE objects only).
`vgroup_set(object, name, indices, weight | weights, mode)` assigns: one `weight`
for the whole batch, or a `weights` array parallel to `indices` for a painted
falloff. `mode` is `REPLACE` (default), `ADD` or `SUBTRACT` — `SUBTRACT` removes the
listed vertices from the group. `vgroup_get(object, name, indices)` reads weights
back; with `indices` omitted it returns every vertex actually in the group, and a
requested index that is not a member is simply absent rather than an error.

What reads a vertex group:

- `mod_add` with `properties: {"vertex_group": "..."}` — MASK, ARMATURE, HOOK,
  SOLIDIFY, DECIMATE, WELD, and the BEVEL modifier via `limit_method: "VGROUP"`.
  Without a group to point at, that falloff cannot be art-directed at all.
- `rig_auto_weights(mesh, armature)` — creates and fills one group per deform bone
  automatically; read the result back with `vgroup_list` / `vgroup_get`.

A vertex group is *not* an attribute: it does not appear in `attr_list`. To use a
weight inside geometry nodes, read it by the group's name with a Named Attribute
node, or mirror it into a real `FLOAT`/`POINT` attribute with `attr_add` +
`attr_set`.

### Attributes and geometry nodes
`GeometryNodeInputNamedAttribute` (Named Attribute) and
`GeometryNodeStoreNamedAttribute` (Store Named Attribute) address exactly the names
`attr_list` prints. Both nodes' `data_type` and `domain` enums on 5.2 are identical
to `attr_add`'s, so an attribute created here is directly addressable in a node
tree, and one stored by a node tree shows up in `attr_list` after the modifier is
applied. Build the tree with `node_graph_build`; set the modifier's exposed inputs
by socket name with `geo_modifier_set` (they live on the modifier, not the tree, so
`mod_update` cannot reach them).

### Baking
`bake_texture` needs three things in place first:

1. a UV map on the mesh — `mesh_unwrap`, and `uv_set_active` with `render: true` if
   there is more than one (or name it explicitly in `uv_layer`);
2. an image datablock — `img_new`, since resolution and colour space are its job;
3. an Image Texture node in the object's active material pointing at that image —
   `node_graph_build`.

Then call `bake_texture(object, image, type, ...)`. On 5.2 `type` is one of
`COMBINED`, `AO`, `SHADOW`, `POSITION`, `NORMAL`, `UV`, `ROUGHNESS`, `EMIT`,
`ENVIRONMENT`, `DIFFUSE`, `GLOSSY`, `TRANSMISSION`; the real list is read from RNA
at call time. Other useful params: `samples`, `margin` + `margin_type`
(`ADJACENT_FACES`/`EXTEND`), `use_clear`, `pass_filter`, and for high-to-low
`use_selected_to_active` + `source_objects` + `cage_extrusion` / `use_cage` +
`cage_object` / `max_ray_distance`. NORMAL bakes also take `normal_space`
(`OBJECT`/`TANGENT`) and the `normal_r`/`normal_g`/`normal_b` swizzles.

## Traps

- `mesh_unwrap` enters Edit Mode and selects the entire mesh. Any selection built
  with `mesh_select` is lost, so unwrap first and select afterwards.
- Active layer and render layer are separate flags. `uv_set_active` without
  `render: true` leaves the bake sampling the old one.
- A mesh accepts at most 8 UV maps.
- `mesh_get_uvs` returns no coordinates — read `UVMap` through `attr_list` /
  write it through `attr_set` instead.
- CORNER-domain indices are loop indices, not vertex indices. A cube has 8 vertices
  but 24 loops.
- Attributes whose name starts with `.` are Blender's internal bookkeeping
  (`.corner_vert`, `.select_vert`, `.uv_select_vert`); listing them is fine, writing
  them is not. The exception is `.sculpt_mask`, which `sculpt_mask` owns.
- `sculpt_remesh`, and any destructive retopology, throws UVs and vertex groups
  away. Unwrap and weight after remeshing, never before.
- `vgroup_add` only works on MESH and LATTICE objects; anything else raises
  Blender's own type error.
- `BYTE_COLOR` items expose both `color` and `color_srgb`; `color_attr_set` always
  writes the linear `color`, so pass linear values.

## See also

`kdb_guide`: `modelling` (selection, `mesh_mark`, modifiers, remesh).
`kdb_search` queries worth running: `bpy.ops.uv.unwrap`, `smart_project`,
`mesh.attributes`, `GeometryNodeStoreNamedAttribute`, `bpy.ops.object.bake`.
