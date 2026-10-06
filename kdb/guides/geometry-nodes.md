---
area: guide
title: Geometry nodes — how to work in it
version: 5.2
---

# Geometry nodes

A geometry-nodes setup here is two things at once: a `GeometryNodeTree` node group,
and a `NODES` modifier on an object that points at it. `node_tree_create` makes both
in one call; from then on the graph is edited with the same generic `node_*` tools
that drive shader and compositor trees, while the modifier's own input values are set
by `geo_modifier_set`. Data flows between the graph and the mesh through **named
attributes**, which the `attr_*` tools read and write from outside.

## The tools

| Tool | What it is for |
|---|---|
| `node_tree_create` | `kind=geometry_modifier` + `object` — creates the tree **and** the `NODES` modifier, pre-wired Group Input → Group Output |
| `node_tree_list` | Every tree in the file with its address and node count |
| `node_types` / `node_schema` | The 237 geometry node types, and a type's real sockets/properties for a given `props` configuration |
| `node_graph_build` | Build a whole graph in one call, with zone pairing and auto-layout |
| `node_graph_edit` | Incremental edits against an existing tree |
| `node_graph_get` | Read the tree back — nodes, props, socket values, links |
| `node_interface_add` | Add a group input/output socket. Returns its `Socket_N` identifier |
| `node_group_make` / `node_group_add` | Collapse nodes into a reusable group / instance one (works headless) |
| `geo_modifier_set` | Set modifier input values **by socket name** on an object+modifier |
| `attr_add` / `attr_list` / `attr_set` / `attr_remove` | Named attributes on a mesh — the channel Named Attribute / Store Named Attribute address |
| `color_attr_add` / `color_attr_set` / `color_attr_list` / `color_attr_remove` | The colour-attribute flavour of the same |
| `sim_create_zone` | Drop a properly paired Simulation Input/Output zone into a tree |
| `sim_set_params` | Set the modifier's bake directory (that is all it writes) |
| `sim_bake` / `sim_clear_cache` | Bake the simulation to cache over a frame range / drop the cache |
| `mod_list` / `mod_delete` / `mod_apply` | The modifier stack the `NODES` modifier sits in |
| `obj_instance_to_points` / `obj_make_instances_real` | Non-geometry-nodes instancing, for comparison |

## How to do the usual things

### Create the setup
`node_tree_create` `{"kind": "geometry_modifier", "name": "Scatter", "object": "Ground"}`
returns `{"address": "object:Ground/Scatter", ...}` and has already:
- created a `GeometryNodeTree` named `Scatter`,
- given it `Geometry` input and output interface sockets,
- added `NodeGroupInput` and `NodeGroupOutput` and wired `Geometry` through, and
- added a `NODES` modifier named `Scatter` on `Ground` with that tree assigned.

So the modifier is a working pass-through immediately, not an empty shell. The
address `object:<Object>/<Modifier>` is what every `node_*` tool takes as `tree`;
`node_tree_list` shows it alongside `material:<name>`, `world`, `compositor` and
`group:<name>`. The same tree is also reachable as `group:Scatter` — the same
datablock, and the name `node_interface_add` wants.

### Build the graph
Call `node_types` `{"kind": "geometry"}` to find types, `node_schema` to get each
one's sockets, then build. Sockets that depend on a property are only visible for
that property's value — `GeometryNodeDistributePointsOnFaces` shows `Density` under
`distribute_method: "RANDOM"` but `Distance Min` / `Density Max` / `Density Factor`
under `"POISSON"` (verified live on 5.2.2), so pass `props` to `node_schema` too.

```json
{
  "tree": "object:Ground/Scatter",
  "clear": true,
  "nodes": [
    {"id": "gin", "type": "NodeGroupInput"},
    {"id": "dist", "type": "GeometryNodeDistributePointsOnFaces",
     "props": {"distribute_method": "POISSON"},
     "inputs": {"Distance Min": 0.15, "Density Max": 40.0, "Seed": 3}},
    {"id": "src", "type": "GeometryNodeObjectInfo", "props": {"transform_space": "RELATIVE"}},
    {"id": "iop", "type": "GeometryNodeInstanceOnPoints",
     "inputs": {"Scale": [0.4, 0.4, 0.4]}},
    {"id": "real", "type": "GeometryNodeRealizeInstances"},
    {"id": "gout", "type": "NodeGroupOutput"}
  ],
  "links": [
    ["gin.Geometry", "dist.Mesh"],
    ["dist.Points", "iop.Points"],
    ["dist.Rotation", "iop.Rotation"],
    ["src.Geometry", "iop.Instance"],
    ["iop.Instances", "real.Geometry"],
    ["real.Geometry", "gout.Geometry"]
  ]
}
```

`clear: true` wipes the Group Input/Output nodes too, which is why they are declared
again above. Guards (duplicate id, malformed link, unknown id, cycle) run before
anything is created, and any failure rolls back every node the call created.

### Expose inputs on the modifier
1. `node_interface_add` `{"group": "Scatter", "name": "Density", "in_out": "INPUT", "socket_type": "NodeSocketFloat", "default": 12.0, "min": 0.0, "max": 200.0}`
   → returns `{"identifier": "Socket_3", ...}`. `group` is the **node group name**,
   not the `object:.../...` address. The new socket appears on the existing
   `NodeGroupInput` node immediately, so `node_graph_edit` can link it by name.
2. Wire it inside the graph, then set the per-object value:

```json
{
  "object": "Ground",
  "modifier": "Scatter",
  "values": {"Density": 24.0, "Offset": [0.0, 0.0, 0.25], "Use Shadow": true}
}
```

`geo_modifier_set` resolves each **name** to its `Socket_N` identifier itself; the
caller never handles identifiers. An unknown name errors with the modifier's real
input list. Only value-type sockets are settable this way — a `NodeSocketGeometry`
input has no value to set and is rejected.

### Where a modifier input actually lives on 5.2
Blender 5.2 moved modifier inputs off ID-properties. They now live at
`modifier.properties.inputs.<identifier>` — a typed slot with `.value`, a `.type`
that is `VALUE` or `ATTRIBUTE`, and `.attribute_name` for the attribute case. The old
address `modifier["Socket_3"]` raises `bpy_struct.keys(): this type doesn't support
IDProperties`, and so does `modifier.keys()` (verified live on 5.2.2). `geo_modifier_set`
handles both addresses — it writes the typed slot on 5.2+, and falls back to the
ID-property on 5.0/5.1 — but any hand-written `util_execute_python` that pokes
`modifier["Socket_N"]` is broken on 5.2 and must be rewritten.

`geo_modifier_set` writes `.value`, i.e. the `VALUE` mode. Driving an input from an
attribute instead is not exposed; read the attribute inside the graph with a Named
Attribute node instead.

### Named attributes as the data channel
- `attr_add` `{"object": "Ground", "name": "wear", "type": "FLOAT", "domain": "POINT"}`,
  then `attr_set` with `values` (and optionally `indices`) to fill it. `attr_list`
  shows every attribute including Blender's built-ins (`position`, `sharp_face`,
  `UVMap`, …) — that is the exact set the graph can address.
- Inside the graph, `GeometryNodeInputNamedAttribute` takes `Name` (a string socket)
  and outputs `Attribute` + `Exists`; its `data_type` property must match how the
  attribute was stored. `GeometryNodeStoreNamedAttribute` takes
  `Geometry`, `Selection`, `Name`, `Value` and outputs `Geometry`, with `data_type`
  and `domain` properties (`POINT`, `EDGE`, `FACE`, `CORNER`, `CURVE`, `INSTANCE`,
  `LAYER`). `GeometryNodeRemoveAttribute` deletes one by `Name`.
- An attribute written by Store Named Attribute is only on the **evaluated** geometry
  until the modifier is applied — `attr_list` reads the underlying mesh, so it will
  not show it. `mod_apply` first if you need to read it back from outside.
- Colours go through `color_attr_*` and the shader-side `ShaderNodeVertexColor`
  (Color Attribute) node; see the `shading-and-materials` guide.

### Instancing
`GeometryNodeInstanceOnPoints` (in: `Points`, `Selection`, `Instance`,
`Pick Instance`, `Instance Index`, `Rotation`, `Scale`; out: `Instances`) is the core
of it. Feed `Points` from `GeometryNodeDistributePointsOnFaces` (out: `Points`,
`Normal`, `Rotation`) or from any point cloud; feed `Instance` from
`GeometryNodeObjectInfo` or a `GeometryNodeCollectionInfo`. Then:
`GeometryNodeRotateInstances` / `GeometryNodeScaleInstances` /
`GeometryNodeTranslateInstances` (each takes `Instances`, `Selection`, and a
`Local Space` boolean) to vary them, `GeometryNodeRealizeInstances` only when real
geometry is genuinely needed — realizing throws away the memory saving that made
instances worth using. `GeometryNodeSetMaterial` (in: `Geometry`, `Selection`,
`Material`) assigns a material inside the graph; the material must already exist
(`mat_create`).

### Simulation zones
1. `sim_create_zone` `{"object": "Ground"}` (or `node_group`) adds a
   `GeometryNodeSimulationInput` + `GeometryNodeSimulationOutput` pair, calls
   `pair_with_output` on them, and links their `Geometry` sockets. Rewire it into the
   graph with `node_graph_edit`.
2. To build a zone as part of a `node_graph_build`, declare both nodes and give the
   input node `"pair": "<id of the output node>"` (the pairing is inferred when
   exactly one matching Output node is in the same build).
3. `sim_set_params` `{"object": "Ground", "cache_path": "//cache/"}` writes the
   modifier's bake directory. It writes nothing else.
4. `sim_bake` `{"object": "Ground", "start_frame": 1, "end_frame": 120}` sets the
   scene's frame range for the duration of the bake, bakes, and restores the range.
   `sim_clear_cache` drops it again — do that before re-baking a changed graph.

State that must persist between frames goes through the zone's state items, which
appear as matching sockets on both nodes. After pairing, the Input node offers
`Delta Time` and `Geometry`; the Output node takes `Skip` and `Geometry`.

### Inspecting what you built
`node_graph_get` `{"tree": "object:Ground/Scatter"}` returns every node's name, type,
label, location, non-default properties and enabled sockets with their current
values, plus every link as `FromNode.Socket -> ToNode.Socket`. `node_tree_list` gives
the addresses of all trees in the file. Together they replace guessing at what state
a previous session left behind.

## Traps

- **A socket added after the modifier exists starts at zero, not at its default.**
  `node_interface_add` with `"default": 12.0` sets the *interface* default; the
  existing modifier's slot was created blank and reads `0.0` (verified live on
  5.2.2). Always follow `node_interface_add` with `geo_modifier_set`.
- **`modifier["Socket_N"]` is dead on 5.2.** Use `geo_modifier_set`, which knows
  about `modifier.properties.inputs.<identifier>.value`.
- **An unpaired zone node has no geometry sockets at all.** A fresh
  `GeometryNodeSimulationInput` has an empty `inputs` and a single `Delta Time`
  output until `pair_with_output` runs. Linking by index before pairing wires
  `Delta Time` into `Skip` — a float into a boolean — and `links.new` does not
  complain. Link by socket **name**, and let `sim_create_zone` or `node_graph_build`'s
  `pair` do the pairing.
- **`sim_bake` and `sim_set_params` take the object's *first* `NODES` modifier**, not
  a named one. On a stack with several geometry-nodes modifiers, reorder or
  temporarily remove the others (`mod_list` / `mod_delete`).
- **Interface sockets take base types only.** `NodeSocketFloatFactor` and
  `NodeSocketVectorTranslation` are socket *subtypes* that exist on nodes but are
  rejected by `node_interface_add`; use `NodeSocketFloat`, `NodeSocketVector`. An
  invalid value is rejected with the exact list this tree kind accepts.
- **`clear: true` removes Group Input and Group Output too.** Re-declare them in the
  same `node_graph_build`, or the modifier silently outputs nothing.
- **Geometry inputs are not modifier values.** A `NodeSocketGeometry` group input has
  no settable value; `geo_modifier_set` refuses it.
- **`node_graph_edit` is a script, not a transaction.** Its lists are applied in
  order (remove, mute, move, set_props, set_inputs, link, unlink) and a failure
  partway through leaves the earlier steps applied. `node_graph_build` does roll back.
- **Zone and group sockets show up with an empty name** in raw RNA dumps (the
  extend/"virtual" socket). Ignore those; `node_graph_get` reports the enabled,
  named ones.

## See also

`kdb_guide` areas: `shading-and-materials` (the same node layer, shader side),
`modelling`, `uv-and-attributes`, `physics-and-simulation`. Worthwhile `kdb_search` queries: "geometry nodes modifier
inputs 5.2", "simulation zone state item", "store named attribute domain",
"instance on points rotation". The generated socket lists: `kdb_read` on
`api/nodes/geometry.md`.
