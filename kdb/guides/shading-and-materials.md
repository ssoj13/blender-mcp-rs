---
area: guide
title: Shading and materials — how to work in it
version: 5.2
---

# Shading and materials

Everything that decides how a surface looks: material datablocks and their slots,
the shader node graph behind them, the images those nodes sample, baking a graph
down to a texture, the world/HDRI that lights it, and the view transform the result
is displayed through. Two families meet here — the `mat_*` tools, which drive a
Principled BSDF by name, and the `node_*` tools, which drive any node tree
generically. Reach for `mat_*` for a plain PBR surface; drop to `node_*` the moment
the material needs more than one node.

## The tools

| Tool | What it is for |
|---|---|
| `mat_create` | New material with a Principled BSDF. Sets only Base Color / Metallic / Roughness. `materials` makes a batch in one call |
| `mat_update` | Change an existing material's Principled inputs. Fails if the tree has no node named `Principled BSDF` — then use `node_graph_edit` |
| `mat_list` | Every material with its Principled values |
| `mat_get_object` | Materials on one object |
| `mat_get_objects_by` | Reverse lookup: which objects use these materials |
| `mat_assign` | Put a material in **slot 0** (appends one if the object has none). Never creates a second slot |
| `mat_slot_add` / `mat_slot_set` / `mat_slot_remove` / `mat_slots` | The slot list itself — how an object gets a second material |
| `mat_assign_faces` | Point a face range or an explicit face list at a slot index. This is what actually makes a mesh multi-material |
| `mat_settings` | EEVEE flags: `blend_method`, `displacement_method`, `backface_culling`, `use_nodes` |
| `mat_preview` | Render the material on a sphere/cube/plane/cloth/monkey in a throwaway scene and hand back the image |
| `node_tree_create` | `kind=material` — a new material whose shader tree is addressable as `material:<name>` |
| `node_types` / `node_schema` | Which node types exist, and a type's **real** socket and property names |
| `node_graph_build` | Build (or wipe and rebuild) a whole graph in one call |
| `node_graph_edit` | Incremental: remove / mute / move / set_props / set_inputs / link / unlink |
| `node_graph_get` | Read a tree back: nodes, non-default props, socket values, every link |
| `img_new` / `img_load` / `img_list` / `img_save` | Image datablocks — bake targets, textures, results |
| `bake_texture` | Bake a shader onto an image with Cycles |
| `uv_add` / `uv_list` / `uv_set_active` / `mesh_unwrap` | The UV layout a bake writes into |
| `world_set_hdri` / `world_set_background` / `world_get` | Environment lighting |
| `color_set` | View transform, look, exposure, gamma — display, not shading |

## How to do the usual things

### A plain PBR material
1. `mat_create` with `name`, `color` (RGB or RGBA), `metallic`, `roughness`.
   Defaults if omitted: colour `0.8, 0.8, 0.8, 1.0`, metallic `0.0`, roughness `0.5`.
2. `mat_assign` it to the object (slot 0).
3. Tweak with `mat_update`. Its parameters map onto Blender 5 Principled sockets:

   | `mat_update` parameter | Principled BSDF socket |
   |---|---|
   | `color` | `Base Color` |
   | `metallic` | `Metallic` |
   | `roughness` | `Roughness` |
   | `emission` | `Emission Color` |
   | `emission_strength` | `Emission Strength` (defaults to 1.0 whenever `emission` is given) |
   | `alpha` | `Alpha` — and it sets `blend_method` to `BLEND` below 1.0, `OPAQUE` at 1.0 |
   | `ior` | `IOR` |
   | `transmission` | `Transmission Weight` |

   Anything else on the Principled node — `Specular IOR Level`, `Coat Weight`,
   `Sheen Weight`, `Subsurface Weight`, `Thin Film Thickness` — has no `mat_update`
   parameter. Set it with `node_graph_edit` `set_inputs` on the `Principled BSDF`
   node instead.

### A multi-material mesh
`mat_assign` only ever fills slot 0, so:
1. `mat_slot_add` once per extra material (or `mat_slot_set` to swap one in place).
2. `mat_slots` to see index → material → link mode (`OBJECT` or `DATA`).
3. `mat_assign_faces` with `slot_index` plus either `face_indices` (explicit list)
   or `face_start` + `face_end` (range). `mesh_select` with `by: "material_index"`
   selects those faces back afterwards.
4. `mat_slot_remove` by index to drop one.

### Building a shader graph
1. `node_tree_create` `{"kind": "material", "name": "Rust"}` → address `material:Rust`,
   already holding `Principled BSDF` + `Material Output`. A material made by
   `mat_create` has the same tree and the same address.
2. Find node types with `node_types` (`kind: "shader"`, `search`), then call
   `node_schema` for each one you intend to use. **Do not recall socket names** —
   Blender 5 renamed most of the Principled inputs, and `node_schema` takes `props`
   so it reports the sockets for a given configuration (`ShaderNodeMix` exposes
   different `A`/`B` sockets per `data_type`).
3. `node_graph_build`:

```json
{
  "tree": "material:Rust",
  "clear": true,
  "nodes": [
    {"id": "coord", "type": "ShaderNodeTexCoord"},
    {"id": "noise", "type": "ShaderNodeTexNoise",
     "inputs": {"Scale": 12.0, "Detail": 6.0, "Roughness": 0.6}},
    {"id": "ramp", "type": "ShaderNodeValToRGB"},
    {"id": "bsdf", "type": "ShaderNodeBsdfPrincipled",
     "inputs": {"Metallic": 1.0, "Roughness": 0.45, "Specular IOR Level": 0.4}},
    {"id": "out", "type": "ShaderNodeOutputMaterial"}
  ],
  "links": [
    ["coord.Object", "noise.Vector"],
    ["noise.Factor", "ramp.Factor"],
    ["ramp.Color", "bsdf.Base Color"],
    ["bsdf.BSDF", "out.Surface"]
  ]
}
```

   `props` is applied before `inputs`, because sockets depend on properties. Node
   `id`s are local handles: Blender may rename on collision, and the response's
   `created` reports the real names. With `clear: false` a link endpoint may also be
   an existing node's real Blender name.
4. Look at it: `mat_preview` `{"material": "Rust", "shape": "sphere", "engine": "EEVEE"}`
   renders in a disposable scene and returns the PNG. Use `engine: "CYCLES"` and a
   higher `samples` when the EEVEE preview lies. Loop: `node_graph_build` →
   `mat_preview` → `node_graph_edit`.
5. `node_graph_get` prints the tree back, including every link as
   `FromNode.Socket -> ToNode.Socket`.

### Textures
1. `img_load` `{"path": ..., "colorspace": "Non-Color"}` for any **data** map —
   roughness, metallic, normal, displacement. Leave it at `sRGB` only for colour.
   `pack: true` embeds the pixels so the .blend travels alone.
2. Feed the returned datablock name to a `ShaderNodeTexImage` as a **property**, not
   an input: `{"id": "rough", "type": "ShaderNodeTexImage", "props": {"image": "rock_rough.png"}}`.
   Its sockets are `Vector` in, `Color` + `Alpha` out. Other useful props (verified
   on 5.2.2): `interpolation` (`Linear`, `Closest`, `Cubic`, `Smart`) and `extension`
   (`REPEAT`, `EXTEND`, `CLIP`, `MIRROR`).
3. UVs reach the texture through `ShaderNodeUVMap` or `ShaderNodeTexCoord`'s `UV`
   output; `uv_list` says which maps the mesh has.
4. A normal map goes through `ShaderNodeNormalMap` (`Color` in → `Normal` out) into
   the Principled `Normal` socket — never straight into it.

### Baking a graph down to a texture
`bake_texture` runs Cycles and checks its prerequisites before touching anything.
All four must hold:
1. The image exists: `img_new` `{"name": "T_Rock_BC", "width": 2048, "height": 2048}`
   (defaults 1024×1024, alpha on). `bake_texture` will not create one — resolution
   and colour space are `img_new`'s business.
2. The mesh has UVs: `mesh_unwrap`, or `uv_add` for a dedicated bake layout followed
   by `uv_set_active`. Pass `uv_layer` to bake into a specific one.
3. The object's **active** material has a node tree containing a
   `ShaderNodeTexImage` whose `image` property is that datablock. The tool selects
   and activates that node itself (and restores the previous selection afterwards);
   the node does not need to be wired to anything.
4. Cycles is available in this build. The tool switches `render.engine` to `CYCLES`
   for the bake and puts the old engine back.

Then: `bake_texture` `{"object": "Rock", "image": "T_Rock_BC", "type": "DIFFUSE", "pass_filter": ["COLOR"], "samples": 64, "margin": 16}`.
`type` is validated against this Blender's own list (on 5.2.2: `COMBINED`, `AO`,
`SHADOW`, `POSITION`, `NORMAL`, `UV`, `ROUGHNESS`, `EMIT`, `ENVIRONMENT`, `DIFFUSE`,
`GLOSSY`, `TRANSMISSION`). High-to-low: `use_selected_to_active: true` plus
`source_objects`, with `cage_extrusion` or `use_cage` + `cage_object` controlling the
rays. Finally `img_save` to write the result to disk — a baked image lives only in
memory until then.

### World and HDRI
- `world_set_hdri` `{"path": "studio.hdr", "strength": 1.0, "rotation": [0, 0, 1.57]}`
  — rotation is radians. It **rebuilds the world tree from scratch**, so a custom
  world graph is lost; build one with `node_graph_build` on the `world` address
  instead if you need more than an environment texture.
- `world_set_background` sets colour/strength on the world's `Background` node,
  creating the world if the scene has none.
- `world_get` reads back what is there.

### Colour management
`color_set` sets `view_transform` (AgX is the 5.2 default), `look`, `exposure`,
`gamma`. This is display, not shading: it changes every render, not the material. The
valid `view_transform` and `look` values come from the OCIO config, so they are not a
fixed list — an invalid value is rejected with this Blender's real options.

## Traps

- **Blender 5 renamed the Principled sockets.** `Specular` → `Specular IOR Level`,
  `Emission` → `Emission Color` + `Emission Strength`, `Transmission` →
  `Transmission Weight`, `Clearcoat` → `Coat Weight`. A noise texture's factor output
  is `Factor`, not `Fac`. Call `node_schema` rather than remembering.
- **`Material.shadow_method` no longer exists on 5.2** (verified live). `mat_settings`
  still advertises the parameter and reports honestly that this Blender has no such
  property. `blend_method` does exist: `OPAQUE`, `CLIP`, `HASHED`, `BLEND`;
  `displacement_method` is `BUMP`, `DISPLACEMENT`, `BOTH`.
- **`mat_update` needs the node to still be called `Principled BSDF`.** After a
  `node_graph_build` with `clear: true` that name is usually gone and `mat_update`
  errors. Use `node_graph_edit` `set_inputs` from then on.
- **`mat_assign` cannot make a second slot.** Multi-material means `mat_slot_add` +
  `mat_assign_faces`.
- **An `img_load` of a missing file does not raise** — Blender hands back an image
  with no pixels. `img_list` reports whether the pixels actually loaded; check it.
- **A data map loaded as sRGB is silently wrong.** Roughness/metallic/normal maps
  need `colorspace: "Non-Color"`.
- **`mat_preview` renders in a temporary scene** and changes nothing in the user's
  file; there is nothing to clean up after it. `scene_screenshot`, by contrast, needs
  a GUI Blender and does nothing under `--background`.
- **A bake result is not saved.** `img_save` it, or it dies with the session.

## See also

`kdb_guide` areas: `geometry-nodes` (the same node layer, geometry side),
`rendering-and-cameras`, `uv-and-attributes`. Worthwhile `kdb_search` queries: "Principled BSDF sockets",
"bake selected to active", "AgX view transform", "EEVEE blend method 5.2". The
generated socket lists: `kdb_read` on `api/nodes/shader.md`.
