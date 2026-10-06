---
area: guide
title: Physics and simulation — how to work in it
version: 5.2
---

# Physics and simulation

Rigid bodies, soft bodies, cloth, collision, fluid, force fields, particles,
geometry-nodes simulation zones and hair curves — plus the three different bake
tools that each drive exactly one of those systems. An agent lands here as soon
as something has to fall, drape, blow, burn or grow, and leaves once the caches
are baked and the frames are ready to render.

## The tools

| Tool | What it is for |
|---|---|
| `phys_add` | Attach RIGID_BODY, SOFT_BODY, CLOTH, COLLISION or FLUID to an object. Single (`name`+`type`) or batch (`physics: [...]`) |
| `phys_remove` | Strip rigid body + the SOFT_BODY/CLOTH/FLUID/COLLISION modifiers off an object. `name` or `names` |
| `phys_add_force_field` | Create a force-field empty. See the type table — several obvious names are wrong |
| `phys_add_particles` | Add a particle system to a mesh and set count/lifetime/emission/physics/render |
| `phys_update_particles` | Change an existing particle system's settings |
| `phys_remove_particles` | Remove one particle system (by `ps_name`) or the active one |
| `phys_bake` | Bake **point caches** — rigid body, soft body, cloth, particles, fluid. The classic physics bake |
| `sim_create_zone` | Add a paired Simulation Input/Output zone into an object's geometry node tree |
| `sim_set_params` | Set the geometry-nodes modifier's `bake_directory` |
| `sim_bake` | Bake **one object's geometry-nodes simulation zones**. Not the same system as `phys_bake` |
| `sim_clear_cache` | Delete that object's geometry-nodes simulation bake |
| `hair_create` | Create a hair-curves object bound to a mesh surface |
| `hair_set_params` | Set `surface_uv_map`; returns the point/curve counts |
| `hair_add_modifier` | Add a modifier (normally NODES) to a hair-curves object |
| `scene_set` | `frame_start`/`frame_end` — every bake reads these. Set them first |

## How to do the usual things

### Which params matter per `phys_add` type

`phys_add` writes only the properties below; everything else keeps Blender's
default. An unknown `type` is a hard error, not a silent no-op.

| `type` | Params used | What it becomes |
|---|---|---|
| RIGID_BODY | `body_type` (ACTIVE/PASSIVE, default ACTIVE), `mass` (1.0), `friction` (0.5), `bounciness` (0.0 → `restitution`), `shape` (CONVEX_HULL) | `bpy.ops.rigidbody.object_add` + `object.rigid_body` |
| SOFT_BODY | `mass` (1.0), `friction` (0.5) | a SOFT_BODY modifier named `Softbody` |
| CLOTH | `mass` (0.3) | a CLOTH modifier named `Cloth` |
| COLLISION | none | a COLLISION modifier named `Collision` |
| FLUID | `fluid_type` (DOMAIN / FLOW / EFFECTOR, default FLOW) | a FLUID modifier named `Fluid` |

5.2 `shape` accepts more than the description lists: BOX, SPHERE, CAPSULE,
CYLINDER, CONE, CONVEX_HULL, MESH, COMPOUND. `fluid_type` also accepts NONE.
The FLUID modifier has no `.settings` — `fluid_type` is set on the modifier
itself, and the domain/flow settings groups appear only after that.

A falling object needs a floor with COLLISION *or* a PASSIVE rigid body — a
rigid-body sim only collides with other rigid bodies, so use PASSIVE there and
keep COLLISION for cloth, soft bodies and particles.

```json
{"physics": [
  {"name": "Ball",  "type": "RIGID_BODY", "body_type": "ACTIVE",  "mass": 2.0, "bounciness": 0.6, "shape": "SPHERE"},
  {"name": "Floor", "type": "RIGID_BODY", "body_type": "PASSIVE", "friction": 0.9}
]}
```

Batch mode reports each entry separately (`{"added": n, "results": [...]}`);
single mode turns a per-entry error into a hard failure.

### Force fields — the type names are not what you expect

Verified live on 5.2, `bpy.ops.object.effector_add` accepts exactly: NONE, BOID,
CHARGE, GUIDE, DRAG, FLUID_FLOW, FORCE, HARMONIC, LENNARDJ, MAGNET, TEXTURE,
TURBULENCE, VORTEX, WIND. **`MAGNETIC` and `SMOKE_FLOW` do not exist** — they
raise `enum "…" not found`. Use `MAGNET` and `FLUID_FLOW`.

`strength` and `flow` are set for every type; the rest are type-specific:
`noise` only for WIND, `inflow` only for VORTEX, `size` only for TURBULENCE. A
`size` sent to a WIND field is dropped without error.

```json
{"type": "TURBULENCE", "name": "Gust", "location": [0, 0, 2], "strength": 12.0, "size": 1.5}
```

### Particles

`phys_add_particles` adds the system with `bpy.ops.object.particle_system_add`,
renames it to `ps_name` (default "ParticleSystem") and writes the settings.
Defaults: `count` 1000, `lifetime` 50.0, `emit_from` FACE, `physics_type`
NEWTON.

- `emit_from`: VERT, FACE, VOLUME.
- `physics_type`: NO, NEWTON, KEYED, BOIDS, FLUID.
- `render_type`: NONE, HALO, LINE, PATH, OBJECT, COLLECTION.
- `instance_object`: naming it forces `render_type` to OBJECT, whatever
  `render_type` said in the same call. This is how you scatter a real mesh.

```json
{"name": "Emitter", "ps_name": "Sparks", "count": 5000,
 "emit_from": "VOLUME", "physics_type": "NEWTON",
 "instance_object": "Spark_Mesh", "lifetime": 24}
```

`phys_update_particles` targets `ps_name`, or the object's first system when
`ps_name` is omitted; it errors cleanly on an object with no particle systems.
Note that the emitter/hair distinction (`settings.type`) is not exposed by these
tools — for hair use `hair_create`, which is the modern Curves system.

### Baking: three tools, three different systems

This is the single most common mix-up here.

- **`phys_bake`** → `bpy.ops.ptcache.bake_all(bake=True)`. Bakes **every point
  cache in the scene**: rigid body, soft body, cloth, particles, fluid. It takes
  `frame_start`/`frame_end` and drives them through the scene frame range
  (which is what `bake_all` actually reads), then restores the scene. Takes no
  object name — it is all-or-nothing.
- **`sim_bake`** → `bpy.ops.object.simulation_nodes_cache_bake(selected=True)`.
  Bakes the **geometry-nodes simulation zones of one object**
  (`object` is required). It selects that object, bakes, and restores both the
  frame range and nothing else. Point caches are untouched.
- **`sim_clear_cache`** → `simulation_nodes_cache_delete`, again for **one
  object's geometry-nodes** cache. It is the counterpart to `sim_bake`, *not* to
  `phys_bake`; there is no tool that clears a point cache.

Use `phys_bake` for anything added with `phys_add` or `phys_add_particles`. Use
`sim_bake`/`sim_clear_cache` for anything built with `sim_create_zone`.

### Frame range comes first

Both bakes read the scene frame range. Passing a range to `phys_bake`/`sim_bake`
sets it for the duration of the bake and then puts it back — which means the
scene's own range still has to be right for playback and rendering afterwards.
Set it once with `scene_set` and let the bakes inherit it:

```json
{"frame_start": 1, "frame_end": 240, "fps": 24}
```

`scene_set` refuses `frame_end < frame_start` rather than silently clamping.

### Geometry-nodes simulation zones

1. The object needs a geometry-nodes modifier with a node group already; give
   `sim_create_zone` either `object` (it takes that object's first NODES
   modifier's group) or `node_group` directly. Blender 4.0+ is required.
2. `sim_create_zone` creates `GeometryNodeSimulationInput` +
   `GeometryNodeSimulationOutput`, **pairs them**, then links Geometry to
   Geometry by socket name. Pairing matters: verified on 5.2, an unpaired
   Simulation Input has an empty `inputs` and only `Delta Time` in `outputs`, so
   index-based linking wires a float into a boolean and produces two nodes that
   merely look like a zone.
3. Wire the zone into the rest of the tree with the node tools.
4. `sim_set_params` with `cache_path` sets the modifier's `bake_directory` — do
   this before `sim_bake` if the cache should not land in the default location.
5. `sim_bake`, then `sim_clear_cache` to invalidate.

### Hair curves

`hair_create` needs a **mesh** `surface`; it creates a `bpy.data.hair_curves`
datablock, links the object, binds `curves.surface`, and adds an empty NODES
modifier called "Generate". It does **not** generate any hair — the modifier's
node group is empty, so the object starts with zero curves. Populate it with the
node tools, or add further modifiers with `hair_add_modifier` (`type` is a
modifier type, normally `NODES`; `properties` is applied key by key and unknown
keys are skipped). `hair_set_params` only writes `surface_uv_map`, and is
useful mainly for the `point_count`/`curve_count` it reports back.

There is no `count` parameter anywhere — density comes from the node group.

## Traps

- **`MAGNETIC` and `SMOKE_FLOW` are not force-field types on 5.2.** Use `MAGNET`
  and `FLUID_FLOW`. Checked live.
- **`phys_bake` is scene-wide.** There is no per-object point-cache bake; if one
  object's sim is wrong, fix it and re-bake everything.
- **`sim_bake` will not bake a point cache and `phys_bake` will not bake a
  simulation zone.** They touch different caches entirely.
- **Rigid bodies ignore COLLISION modifiers.** Give the floor a PASSIVE rigid
  body instead.
- **Both bakes restore the frame range they changed.** A bake never leaves the
  scene range where you put it; use `scene_set` for the persistent range.
- **`phys_remove` removes physics wholesale**: the rigid body plus every
  SOFT_BODY, CLOTH, FLUID and COLLISION modifier on that object. It cannot
  remove just one.
- **`phys_add` FLUID only creates the modifier and sets `fluid_type`.** A
  working smoke or liquid sim still needs domain settings, a flow object inside
  the domain, and a `phys_bake`.
- **A particle system with `physics_type: "NO"`** renders where it is emitted
  and never moves — that is a valid scatter setup, not a broken sim.
- **`hair_create` produces an empty hair object.** Expect zero curves until the
  Generate node group is filled in.

## See also

`kdb_guide` areas: `rendering-and-cameras` (frame range, motion blur, the render
queue for the baked animation), `pipeline-io` (`file_save` before a long bake).
`kdb_search` queries worth running: "effector_add type enum", "ptcache bake_all",
"simulation_nodes_cache_bake", "particle settings physics_type", "hair curves
surface".
