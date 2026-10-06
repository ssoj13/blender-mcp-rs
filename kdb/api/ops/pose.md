---
area: api-ops
group: pose
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.pose

- `bpy.ops.pose.armature_apply(selected: boolean)` — Apply the current pose as the new rest pose
- `bpy.ops.pose.autoside_names(axis: enum{XAXIS, YAXIS, ZAXIS})` — Automatically renames the selected bones according to which side of the target axis they fall on
- `bpy.ops.pose.blend_to_neighbor(factor: float, prev_frame: int, next_frame: int, channels: enum{ALL, LOC, ROT, SIZE, BBONE, CUSTOM}, axis_lock: enum{FREE, X, Y, Z})` — Blend from current position to previous or next keyframe
- `bpy.ops.pose.blend_with_rest(factor: float, prev_frame: int, next_frame: int, channels: enum{ALL, LOC, ROT, SIZE, BBONE, CUSTOM}, axis_lock: enum{FREE, X, Y, Z})` — Make the current pose more similar to, or further away from, the rest pose
- `bpy.ops.pose.breakdown(factor: float, prev_frame: int, next_frame: int, channels: enum{ALL, LOC, ROT, SIZE, BBONE, CUSTOM}, axis_lock: enum{FREE, X, Y, Z})` — Create a suitable breakdown pose on the current frame
- `bpy.ops.pose.constraint_add(type: enum{CAMERA_SOLVER, FOLLOW_TRACK, OBJECT_SOLVER, COPY_LOCATION, COPY_ROTATION, COPY_SCALE, COPY_TRANSFORMS, LIMIT_DISTANCE, LIMIT_LOCATION, LIMIT_ROTATION, LIMIT_SCALE, MAINTAIN_VOLUME, TRANSFORM, TRANSFORM_CACHE, CLAMP_TO, DAMPED_TRACK, IK, LOCKED_TRACK, SPLINE_IK, STRETCH_TO, TRACK_TO, ACTION, ARMATURE, CHILD_OF, FLOOR, FOLLOW_PATH, GEOMETRY_ATTRIBUTE, PIVOT, SHRINKWRAP})` — Add a constraint to the active bone
- `bpy.ops.pose.constraint_add_with_targets(type: enum{CAMERA_SOLVER, FOLLOW_TRACK, OBJECT_SOLVER, COPY_LOCATION, COPY_ROTATION, COPY_SCALE, COPY_TRANSFORMS, LIMIT_DISTANCE, LIMIT_LOCATION, LIMIT_ROTATION, LIMIT_SCALE, MAINTAIN_VOLUME, TRANSFORM, TRANSFORM_CACHE, CLAMP_TO, DAMPED_TRACK, IK, LOCKED_TRACK, SPLINE_IK, STRETCH_TO, TRACK_TO, ACTION, ARMATURE, CHILD_OF, FLOOR, FOLLOW_PATH, GEOMETRY_ATTRIBUTE, PIVOT, SHRINKWRAP})` — Add a constraint to the active bone, with target (where applicable) set to the selected Objects/Bones
- `bpy.ops.pose.constraints_clear()` — Clear all constraints from the selected bones
- `bpy.ops.pose.constraints_copy()` — Copy constraints to other selected bones
- `bpy.ops.pose.copy()` — Copy the current pose of the selected bones to the internal clipboard
- `bpy.ops.pose.flip_names(do_strip_numbers: boolean)` — Flips (and corrects) the axis suffixes of the names of selected bones
- `bpy.ops.pose.hide(unselected: boolean)` — Tag selected bones to not be visible in Pose Mode
- `bpy.ops.pose.ik_add(with_targets: boolean)` — Add an IK Constraint to the active Bone. The target can be a selected bone or object
- `bpy.ops.pose.ik_clear()` — Remove all IK Constraints from selected bones
- `bpy.ops.pose.loc_clear()` — Reset locations of selected bones to their default values
- `bpy.ops.pose.paste(flipped: boolean, selected_mask: boolean)` — Paste the stored pose on to the current pose
- `bpy.ops.pose.paths_calculate(display_type: enum{CURRENT_FRAME, RANGE}, range: enum{KEYS_ALL, KEYS_SELECTED, SCENE, MANUAL}, bake_location: enum{HEADS, TAILS})` — Calculate paths for the selected bones
- `bpy.ops.pose.paths_clear(only_selected: boolean)` — (undocumented operator)
- `bpy.ops.pose.paths_range_update()` — Update frame range for motion paths from the Scene's current frame range
- `bpy.ops.pose.paths_update()` — Recalculate paths for bones that already have them
- `bpy.ops.pose.propagate(mode: enum{NEXT_KEY, LAST_KEY, BEFORE_FRAME, BEFORE_END, SELECTED_KEYS, SELECTED_MARKERS}, end_frame: float)` — Copy selected aspects of the current pose to subsequent poses already keyframed
- `bpy.ops.pose.push(factor: float, prev_frame: int, next_frame: int, channels: enum{ALL, LOC, ROT, SIZE, BBONE, CUSTOM}, axis_lock: enum{FREE, X, Y, Z})` — Exaggerate the current pose in regards to the breakdown pose
- `bpy.ops.pose.quaternions_flip()` — Flip quaternion values to achieve desired rotations, while maintaining the same orientations
- `bpy.ops.pose.relax(factor: float, prev_frame: int, next_frame: int, channels: enum{ALL, LOC, ROT, SIZE, BBONE, CUSTOM}, axis_lock: enum{FREE, X, Y, Z})` — Make the current pose more similar to its breakdown pose
- `bpy.ops.pose.reveal(select: boolean)` — Reveal all bones hidden in Pose Mode
- `bpy.ops.pose.rot_clear()` — Reset rotations of selected bones to their default values
- `bpy.ops.pose.rotation_mode_set(type: enum{QUATERNION, XYZ, XZY, YXZ, YZX, ZXY, ZYX, AXIS_ANGLE})` — Set the rotation representation used by selected bones
- `bpy.ops.pose.scale_clear()` — Reset scaling of selected bones to their default values
- `bpy.ops.pose.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Toggle selection status of all bones
- `bpy.ops.pose.select_constraint_target()` — Select bones used as targets for the currently selected bones
- `bpy.ops.pose.select_grouped(extend: boolean, type: enum{COLLECTION, COLOR, KEYINGSET, CHILDREN, CHILDREN_IMMEDIATE, PARENT, SIBLINGS})` — Select all visible bones grouped by similar properties
- `bpy.ops.pose.select_hierarchy(direction: enum{PARENT, CHILD}, extend: boolean)` — Select immediate parent/children of selected bones
- `bpy.ops.pose.select_linked()` — Select all bones linked by connected parent/child relationships from the current selection
- `bpy.ops.pose.select_linked_pick(extend: boolean)` — Select bones linked by connected parent/child relationships under the mouse cursor
- `bpy.ops.pose.select_mirror(only_active: boolean, extend: boolean)` — Mirror the bone selection
- `bpy.ops.pose.select_parent()` — Select bones that are parents of the currently selected bones
- `bpy.ops.pose.selection_set_add()` — Create a new empty Selection Set
- `bpy.ops.pose.selection_set_add_and_assign()` — Create a new Selection Set with the currently selected bones
- `bpy.ops.pose.selection_set_assign()` — Add selected bones to Selection Set
- `bpy.ops.pose.selection_set_copy()` — Copy the selected Selection Set(s) to the clipboard
- `bpy.ops.pose.selection_set_delete_all()` — Remove all Selection Sets from this Armature
- `bpy.ops.pose.selection_set_deselect()` — Remove Selection Set bones from current selection
- `bpy.ops.pose.selection_set_move(direction: enum{UP, DOWN})` — Move the active Selection Set up/down the list of sets
- `bpy.ops.pose.selection_set_paste()` — Add new Selection Set(s) from the clipboard
- `bpy.ops.pose.selection_set_remove()` — Remove a Selection Set from this Armature
- `bpy.ops.pose.selection_set_remove_bones()` — Remove the selected bones from all Selection Sets
- `bpy.ops.pose.selection_set_select(selection_set_index: int)` — Select the bones from this Selection Set
- `bpy.ops.pose.selection_set_unassign()` — Remove selected bones from Selection Set
- `bpy.ops.pose.transforms_clear()` — Reset location, rotation, and scaling of selected bones to their default values
- `bpy.ops.pose.user_transforms_clear(only_selected: boolean)` — Reset pose bone transforms to keyframed state
- `bpy.ops.pose.visual_transform_apply()` — Apply final constrained position of pose bones to their transform