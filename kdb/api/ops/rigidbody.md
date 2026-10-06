---
area: api-ops
group: rigidbody
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.rigidbody

- `bpy.ops.rigidbody.bake_to_keyframes(frame_start: int, frame_end: int, step: int)` — Bake rigid body transformations of selected objects to keyframes
- `bpy.ops.rigidbody.connect(con_type: enum{FIXED, POINT, HINGE, SLIDER, PISTON, GENERIC, GENERIC_SPRING, MOTOR}, pivot_type: enum{CENTER, ACTIVE, SELECTED}, connection_pattern: enum{SELECTED_TO_ACTIVE, CHAIN_DISTANCE})` — Create rigid body constraints between selected rigid bodies
- `bpy.ops.rigidbody.constraint_add(type: enum{FIXED, POINT, HINGE, SLIDER, PISTON, GENERIC, GENERIC_SPRING, MOTOR})` — Add Rigid Body Constraint to active object
- `bpy.ops.rigidbody.constraint_remove()` — Remove Rigid Body Constraint from Object
- `bpy.ops.rigidbody.mass_calculate(material: enum{DEFAULT}, density: float)` — Automatically calculate mass values for Rigid Body Objects based on volume
- `bpy.ops.rigidbody.object_add(type: enum{ACTIVE, PASSIVE})` — Add active object as Rigid Body
- `bpy.ops.rigidbody.object_remove()` — Remove Rigid Body settings from Object
- `bpy.ops.rigidbody.object_settings_copy()` — Copy Rigid Body settings from active object to selected
- `bpy.ops.rigidbody.objects_add(type: enum{ACTIVE, PASSIVE})` — Add selected objects as Rigid Bodies
- `bpy.ops.rigidbody.objects_remove()` — Remove selected objects from Rigid Body simulation
- `bpy.ops.rigidbody.shape_change(type: enum{BOX, SPHERE, CAPSULE, CYLINDER, CONE, CONVEX_HULL, MESH, COMPOUND})` — Change collision shapes for selected Rigid Body Objects
- `bpy.ops.rigidbody.world_add()` — Add Rigid Body simulation world to the current scene
- `bpy.ops.rigidbody.world_remove()` — Remove Rigid Body simulation world from the current scene