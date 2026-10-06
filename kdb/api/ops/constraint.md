---
area: api-ops
group: constraint
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.constraint

- `bpy.ops.constraint.add_target()` — Add a target to the constraint
- `bpy.ops.constraint.apply(constraint: string, owner: enum{OBJECT, BONE}, report: boolean)` — Apply constraint and remove from the stack
- `bpy.ops.constraint.childof_clear_inverse(constraint: string, owner: enum{OBJECT, BONE})` — Clear inverse correction for Child Of constraint
- `bpy.ops.constraint.childof_set_inverse(constraint: string, owner: enum{OBJECT, BONE})` — Set inverse correction for Child Of constraint
- `bpy.ops.constraint.copy(constraint: string, owner: enum{OBJECT, BONE}, report: boolean)` — Duplicate constraint at the same position in the stack
- `bpy.ops.constraint.copy_to_selected(constraint: string, owner: enum{OBJECT, BONE})` — Copy constraint to other selected objects/bones
- `bpy.ops.constraint.delete(constraint: string, owner: enum{OBJECT, BONE}, report: boolean)` — Remove constraint from constraint stack
- `bpy.ops.constraint.disable_keep_transform()` — Set the influence of this constraint to zero while trying to maintain the object's transformation. Other active constraints can still influence the final transformation
- `bpy.ops.constraint.followpath_path_animate(constraint: string, owner: enum{OBJECT, BONE}, frame_start: int, length: int)` — Add default animation for path used by constraint if it isn't animated already
- `bpy.ops.constraint.limitdistance_reset(constraint: string, owner: enum{OBJECT, BONE})` — Reset limiting distance for Limit Distance Constraint
- `bpy.ops.constraint.move_down(constraint: string, owner: enum{OBJECT, BONE})` — Move constraint down in constraint stack
- `bpy.ops.constraint.move_to_index(constraint: string, owner: enum{OBJECT, BONE}, index: int)` — Change the constraint's position in the list so it evaluates after the set number of others
- `bpy.ops.constraint.move_up(constraint: string, owner: enum{OBJECT, BONE})` — Move constraint up in constraint stack
- `bpy.ops.constraint.normalize_target_weights()` — Normalize weights of all target bones
- `bpy.ops.constraint.objectsolver_clear_inverse(constraint: string, owner: enum{OBJECT, BONE})` — Clear inverse correction for Object Solver constraint
- `bpy.ops.constraint.objectsolver_set_inverse(constraint: string, owner: enum{OBJECT, BONE})` — Set inverse correction for Object Solver constraint
- `bpy.ops.constraint.remove_target(index: int)` — Remove the target from the constraint
- `bpy.ops.constraint.stretchto_reset(constraint: string, owner: enum{OBJECT, BONE})` — Reset original length of bone for Stretch To Constraint