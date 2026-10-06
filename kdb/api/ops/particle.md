---
area: api-ops
group: particle
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.particle

- `bpy.ops.particle.brush_edit(stroke: collection, pen_flip: boolean)` — Apply a stroke of brush to the particles
- `bpy.ops.particle.connect_hair(all: boolean)` — Connect hair to the emitter mesh
- `bpy.ops.particle.copy_particle_systems(space: enum{OBJECT, WORLD}, remove_target_particles: boolean, use_active: boolean)` — Copy particle systems from the active object to selected objects
- `bpy.ops.particle.delete(type: enum{PARTICLE, KEY})` — Delete selected particles or keys
- `bpy.ops.particle.disconnect_hair(all: boolean)` — Disconnect hair from the emitter mesh
- `bpy.ops.particle.duplicate_particle_system(use_duplicate_settings: boolean)` — Duplicate particle system within the active object
- `bpy.ops.particle.dupliob_copy()` — Duplicate the current instance object
- `bpy.ops.particle.dupliob_move_down()` — Move instance object down in the list
- `bpy.ops.particle.dupliob_move_up()` — Move instance object up in the list
- `bpy.ops.particle.dupliob_refresh()` — Refresh list of instance objects and their weights
- `bpy.ops.particle.dupliob_remove()` — Remove the selected instance object
- `bpy.ops.particle.edited_clear()` — Undo all edition performed on the particle system
- `bpy.ops.particle.hair_dynamics_preset_add(name: string, remove_name: boolean, remove_active: boolean)` — Add or remove a Hair Dynamics Preset
- `bpy.ops.particle.hide(unselected: boolean)` — Hide selected particles
- `bpy.ops.particle.mirror()` — Duplicate and mirror the selected particles along the local X axis
- `bpy.ops.particle.new()` — Add new particle settings
- `bpy.ops.particle.new_target()` — Add a new particle target
- `bpy.ops.particle.particle_edit_toggle()` — Toggle particle edit mode
- `bpy.ops.particle.particle_system_remove_all()` — Remove all particle system within the active object
- `bpy.ops.particle.rekey(keys_number: int)` — Change the number of keys of selected particles (root and tip keys included)
- `bpy.ops.particle.remove_doubles(threshold: float)` — Remove selected particles close enough to others
- `bpy.ops.particle.reveal(select: boolean)` — Show hidden particles
- `bpy.ops.particle.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — (De)select all particles' keys
- `bpy.ops.particle.select_less()` — Deselect boundary selected keys of each particle
- `bpy.ops.particle.select_linked()` — Select all keys linked to already selected ones
- `bpy.ops.particle.select_linked_pick(deselect: boolean, location: int)` — Select nearest particle from mouse pointer
- `bpy.ops.particle.select_more()` — Select keys linked to boundary selected keys of each particle
- `bpy.ops.particle.select_random(ratio: float, seed: int, action: enum{SELECT, DESELECT}, type: enum{HAIR, POINTS})` — Select a randomly distributed set of hair or points
- `bpy.ops.particle.select_roots(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Select roots of all visible particles
- `bpy.ops.particle.select_tips(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Select tips of all visible particles
- `bpy.ops.particle.shape_cut()` — Cut hair to conform to the set shape object
- `bpy.ops.particle.subdivide()` — Subdivide selected particles segments (adds keys)
- `bpy.ops.particle.target_move_down()` — Move particle target down in the list
- `bpy.ops.particle.target_move_up()` — Move particle target up in the list
- `bpy.ops.particle.target_remove()` — Remove the selected particle target
- `bpy.ops.particle.unify_length()` — Make selected hair the same length
- `bpy.ops.particle.weight_set(factor: float)` — Set the weight of selected keys