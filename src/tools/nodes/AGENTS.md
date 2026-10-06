# src/tools/nodes — the unified node layer

## What this is

One tool family for **all** Blender node trees: shader, geometry, compositor, world,
texture. It used to be three nearly identical families (`shader_*`, `comp_*`, `geo_*`)
with **three hand-written type tables** totalling 205 lines.

The tables are gone. There is **not a single hardcoded node or socket name** here —
everything is asked of Blender via RNA. If you feel tempted to add a `match` on node
types: don't, that is exactly what made everything rot.

## Numbers (measured on Blender 5.0)

| | |
|---|---|
| Node types in Blender 5.2 | 570 (shader 101, **geometry 267**, compositor 91, texture 37, function 56, generic 18) |
| Of these, actually creatable | **555** |
| Covered by the old tables | ~40 % |
| Covered now | **100 %** |

## Files

| File | What |
|---|---|
| `mod.rs` | Tree addressing + **all the guards**: `resolve_tree`, `find_socket`, `set_socket_value`, `set_node_prop`, `enabled_sockets` |
| `introspect.rs` | `node_types` (all 500), `node_schema` (sockets + properties + enums) |
| `graph.rs` | `node_tree_list/create`, `node_graph_get/build/edit` + auto-layout |
| `group.rs` | `node_group_make/add`, `node_interface_add`, `geo_modifier_set` |
| `preview.rs` | `mat_preview` — renders a material to an image |

## Tree addressing

One string for all five places where nodes live:

```
material:Rust          material shader tree
world                  world tree
compositor             scene compositor
group:MyGroup          node group (incl. geometry)
object:Cube/GeoNodes   geometry modifier tree
```

## The key point: the model must not guess

**`node_schema` is the central tool.** In Blender 5 the Principled BSDF sockets are
named differently from what any model remembers:

| Model remembers | Actually |
|---|---|
| `Specular` | `Specular IOR Level` |
| `Emission` | `Emission Color` + `Emission Strength` |
| `Transmission` | `Transmission Weight` |
| `Clearcoat` | `Coat Weight` |
| `Fac` (Noise output) | **`Factor`** |

That's why `find_socket` on a miss doesn't just say "not found", it **returns the
node's real socket list**. This is not politeness, it is the only way to break the
loop "model hits `Specular` → error → hits it again".

Sockets depend on node properties: `ShaderNodeMix` has four sockets named `A` — one
per data type, only the one matching `data_type` is visible. That's why `node_schema`
accepts `props` and shows the sockets **for that configuration**, and `enabled_sockets`
filters by the `enabled` flag everywhere.

## Traps (all verified on live Blender, not inferred)

- **A node's RNA class has no `bl_idname` class attribute.** The name lives in
  `cls.bl_rna.identifier`. `cls.bl_idname` → `AttributeError`.
- **Some RNA classes are registered but not creatable.** `TextureNodeTexMusgrave`
  exists in `bpy.types`, but lives only in the texture tree; the shader
  `ShaderNodeTexMusgrave` was removed in 4.1. So the catalog is built by **actually
  creating** every type in a scratch tree (235 ms once, cached in a `OnceLock`) —
  "advertised but not creatable" is impossible by construction.
- **`is` on bpy wrappers doesn't work.** Blender creates a new Python object on nearly
  every attribute access: `tree.nodes["A"] is tree.nodes["A"]` → `False`. Compare
  only via `as_pointer()`. This is used in the cycle detector and graph traversal.
- **A `Link` wrapper must not be held across node deletion** — you get a phantom loop.
  Save the socket itself before mutating.
- **`bpy.ops.node.group_make` doesn't work headless** (`poll()` = `False`, no
  `NODE_EDITOR`). So `node_group_make` assembles the group by hand.
- **The compositor moved.** In Blender 5 `scene.node_tree` is **removed**, the tree lives
  in `scene.compositing_node_group`. The old `comp_*` read `scene.node_tree` and were
  broken — they had simply never been run.
- **The engine name is a moving target.** EEVEE was `BLENDER_EEVEE`, became
  `BLENDER_EEVEE_NEXT` in 4.2 and `BLENDER_EEVEE` again in 5.0. Don't hardcode —
  `bpy::render_engine()` checks against the real enum.

## Feedback loop

`mat_preview` renders the material on a preview object **in a separate temporary scene**
and returns the PNG straight to the model. The user's scene is not touched (verified by
datablock counters before/after). Loop: `node_graph_build` → `mat_preview` → look →
`node_graph_edit`.
