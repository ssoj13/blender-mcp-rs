---
area: api-types
group: constraint
version: 5.2.2
---

# Blender 5.2.2 RNA types: constraint

## ActionConstraint

Map an action to the transform axes of a bone

- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `mix_mode` · enum · {REPLACE, BEFORE_FULL, BEFORE, BEFORE_SPLIT, AFTER_FULL, AFTER, AFTER_SPLIT} — Specify how existing transformations and the action channels are combined
- `transform_channel` · enum · {LOCATION_X, LOCATION_Y, LOCATION_Z, ROTATION_X, ROTATION_Y, ROTATION_Z, SCALE_X, SCALE_Y, SCALE_Z} — Transformation channel from the target that is used to key the Action
- `action` · pointer · -> Action — The constraining action
- `action_slot_handle` · int — A number that identifies which sub-set of the Action is considered to be for this Action Constraint
- `last_slot_identifier` · string — The identifier of the most recently assigned action slot. The slot identifies which sub-set of the Action is considered to be for this constraint, and its identifier is used to find the right slot when assigning an Action.
- `action_slot` · pointer · -> ActionSlot — The slot identifies which sub-set of the Action is considered to be for this strip, and its name is used to find the right slot when assigning another Action
- `action_suitable_slots` · collection · read-only — The list of action slots suitable for this NLA strip
- `use_bone_object_action` · boolean — Bones only: apply the object's transformation channels of the action to the constrained bone, instead of bone's channels
- `frame_start` · int — First frame of the Action to use
- `frame_end` · int — Last frame of the Action to use
- `max` · float — Maximum value for target channel range
- `min` · float — Minimum value for target channel range
- `eval_time` · float — Interpolates between Action Start and End frames
- `use_eval_time` · boolean — Interpolate between Action Start and End frames, with the Evaluation Time slider instead of the Target object/bone

## ArmatureConstraint

Applies transformations done by the Armature modifier

- `targets` · collection · [] of ArmatureConstraintTargets · read-only — Target Bones
- `use_deform_preserve_volume` · boolean — Deform rotation interpolation with quaternions
- `use_bone_envelopes` · boolean — Multiply weights by envelope for all bones, instead of acting like Vertex Group based blending. The specified weights are still used, and only the listed bones are considered.
- `use_current_location` · boolean — Use the current bone location for envelopes and choosing B-Bone segments instead of rest position

## CameraSolverConstraint

Lock motion to the reconstructed camera movement

- `clip` · pointer · -> MovieClip — Movie Clip to get tracking data from
- `use_active_clip` · boolean — Use active clip defined in scene

## ChildOfConstraint

Create constraint-based parent-child relationship

- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `use_location_x` · boolean — Use X Location of Parent
- `use_location_y` · boolean — Use Y Location of Parent
- `use_location_z` · boolean — Use Z Location of Parent
- `use_rotation_x` · boolean — Use X Rotation of Parent
- `use_rotation_y` · boolean — Use Y Rotation of Parent
- `use_rotation_z` · boolean — Use Z Rotation of Parent
- `use_scale_x` · boolean — Use X Scale of Parent
- `use_scale_y` · boolean — Use Y Scale of Parent
- `use_scale_z` · boolean — Use Z Scale of Parent
- `set_inverse_pending` · boolean — Set to true to request recalculation of the inverse matrix
- `inverse_matrix` · float · [16] — Transformation matrix to apply before

## ClampToConstraint

Constrain an object's location to the nearest point along the target path

- `target` · pointer · -> Object — Target Object (Curves only)
- `main_axis` · enum · {CLAMPTO_AUTO, CLAMPTO_X, CLAMPTO_Y, CLAMPTO_Z} — Main axis of movement
- `use_cyclic` · boolean — Treat curve as cyclic curve (no clamping to curve bounding box)

## Constraint

Constraint modifying the transformation of objects and bones

- `name` · string — Constraint name
- `type` · enum · {CAMERA_SOLVER, FOLLOW_TRACK, OBJECT_SOLVER, COPY_LOCATION, COPY_ROTATION, COPY_SCALE, COPY_TRANSFORMS, LIMIT_DISTANCE, LIMIT_LOCATION, LIMIT_ROTATION, LIMIT_SCALE, MAINTAIN_VOLUME, TRANSFORM, TRANSFORM_CACHE, CLAMP_TO, DAMPED_TRACK, IK, LOCKED_TRACK, SPLINE_IK, STRETCH_TO, TRACK_TO, ACTION, ARMATURE, CHILD_OF, FLOOR, FOLLOW_PATH, GEOMETRY_ATTRIBUTE, PIVOT, SHRINKWRAP} · read-only
- `is_override_data` · boolean · read-only — In a local override object, whether this constraint comes from the linked reference object, or is local to the override
- `owner_space` · enum · {WORLD, CUSTOM, POSE, LOCAL_WITH_PARENT, LOCAL} — Space that owner is evaluated in
- `target_space` · enum · {WORLD, CUSTOM, POSE, LOCAL_WITH_PARENT, LOCAL, LOCAL_OWNER_ORIENT} — Space that target is evaluated in
- `space_object` · pointer · -> Object — Object for Custom Space
- `space_subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `mute` · boolean — Enable/Disable Constraint
- `enabled` · boolean — Use the results of this constraint
- `show_expanded` · boolean — Constraint's panel is expanded in UI
- `is_valid` · boolean · read-only — Constraint has valid settings and can be evaluated
- `active` · boolean — Constraint is the one being edited
- `influence` · float — Amount of influence constraint will have on the final solution
- `error_location` · float · read-only — Amount of residual error in Blender space unit for constraints that work on position
- `error_rotation` · float · read-only — Amount of residual error in radians for constraints that work on orientation

## ConstraintTarget

Target object for multi-target constraints

- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...

## ConstraintTargetBone

Target bone for multi-target constraints

- `target` · pointer · -> Object — Target armature
- `subtarget` · string — Target armature bone
- `weight` · float — Blending weight of this bone

## CopyLocationConstraint

Copy the location of the target

- `head_tail` · float — Target along length of bone: Head is 0, Tail is 1
- `use_bbone_shape` · boolean — Follow shape of B-Bone segments when calculating Head/Tail position
- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `use_x` · boolean — Copy the target's X location
- `use_y` · boolean — Copy the target's Y location
- `use_z` · boolean — Copy the target's Z location
- `invert_x` · boolean — Invert the X location
- `invert_y` · boolean — Invert the Y location
- `invert_z` · boolean — Invert the Z location
- `use_offset` · boolean — Add original location into copied location

## CopyRotationConstraint

Copy the rotation of the target

- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `use_x` · boolean — Copy the target's X rotation
- `use_y` · boolean — Copy the target's Y rotation
- `use_z` · boolean — Copy the target's Z rotation
- `invert_x` · boolean — Invert the X rotation
- `invert_y` · boolean — Invert the Y rotation
- `invert_z` · boolean — Invert the Z rotation
- `euler_order` · enum · {AUTO, XYZ, XZY, YXZ, YZX, ZXY, ZYX} — Explicitly specify the euler rotation order
- `mix_mode` · enum · {REPLACE, ADD, BEFORE, AFTER, OFFSET} — Specify how the copied and existing rotations are combined
- `use_offset` · boolean — DEPRECATED: Add original rotation into copied rotation

## CopyScaleConstraint

Copy the scale of the target

- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `use_x` · boolean — Copy the target's X scale
- `use_y` · boolean — Copy the target's Y scale
- `use_z` · boolean — Copy the target's Z scale
- `power` · float — Raise the target's scale to the specified power
- `use_make_uniform` · boolean — Redistribute the copied change in volume equally between the three axes of the owner
- `use_offset` · boolean — Combine original scale with copied scale
- `use_add` · boolean — Use addition instead of multiplication to combine scale (2.7 compatibility)

## CopyTransformsConstraint

Copy all the transforms of the target

- `head_tail` · float — Target along length of bone: Head is 0, Tail is 1
- `use_bbone_shape` · boolean — Follow shape of B-Bone segments when calculating Head/Tail position
- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `remove_target_shear` · boolean — Remove shear from the target transformation before combining
- `mix_mode` · enum · {REPLACE, BEFORE_FULL, BEFORE, BEFORE_SPLIT, AFTER_FULL, AFTER, AFTER_SPLIT} — Specify how the copied and existing transformations are combined

## DampedTrackConstraint

Point toward target by taking the shortest rotation path

- `head_tail` · float — Target along length of bone: Head is 0, Tail is 1
- `use_bbone_shape` · boolean — Follow shape of B-Bone segments when calculating Head/Tail position
- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `track_axis` · enum · {TRACK_X, TRACK_Y, TRACK_Z, TRACK_NEGATIVE_X, TRACK_NEGATIVE_Y, TRACK_NEGATIVE_Z} — Axis that points to the target object

## FloorConstraint

Use the target object for location limitation

- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `floor_location` · enum · {FLOOR_X, FLOOR_Y, FLOOR_Z, FLOOR_NEGATIVE_X, FLOOR_NEGATIVE_Y, FLOOR_NEGATIVE_Z} — Location of target that object will not pass through
- `use_rotation` · boolean — Use the target's rotation to determine floor
- `offset` · float — Offset of floor from object origin

## FollowPathConstraint

Lock motion to the target path

- `target` · pointer · -> Object — Target Curve object
- `offset` · float — Offset from the position corresponding to the time frame
- `offset_factor` · float — Percentage value defining target position along length of curve
- `forward_axis` · enum · {FORWARD_X, FORWARD_Y, FORWARD_Z, TRACK_NEGATIVE_X, TRACK_NEGATIVE_Y, TRACK_NEGATIVE_Z} — Axis that points forward along the path
- `up_axis` · enum · {UP_X, UP_Y, UP_Z} — Axis that points upward
- `use_curve_follow` · boolean — Object will follow the heading and banking of the curve
- `use_fixed_location` · boolean — Object will stay locked to a single point somewhere along the length of the curve regardless of time
- `use_curve_radius` · boolean — Object is scaled by the curve radius

## FollowTrackConstraint

Lock motion to the target motion track

- `clip` · pointer · -> MovieClip — Movie Clip to get tracking data from
- `track` · string — Movie tracking track to follow
- `use_active_clip` · boolean — Use active clip defined in scene
- `use_3d_position` · boolean — Use 3D position of track to parent to
- `object` · string — Movie tracking object to follow (if empty, camera object is used)
- `camera` · pointer · -> Object — Camera to which motion is parented (if empty active scene camera is used)
- `depth_object` · pointer · -> Object — Object used to define depth in camera space by projecting onto surface of this object
- `frame_method` · enum · {STRETCH, FIT, CROP} — How the footage fits in the camera frame
- `use_undistorted_position` · boolean — Parent to undistorted position of 2D track

## GeometryAttributeConstraint

Create a constraint-based relationship with an attribute from geometry

- `target` · pointer · -> Object — Target geometry object
- `attribute_name` · string — Name of the attribute to retrieve the transform from
- `domain` · enum · {POINT, EDGE, FACE, FACE_CORNER, CURVE, INSTANCE} — Attribute domain
- `apply_target_transform` · boolean — Apply the target object's world transform on top of the attribute's transform
- `data_type` · enum · {VECTOR, QUATERNION, FLOAT4X4} — Select data type of attribute
- `sample_index` · int — Sample Index
- `mix_loc` · boolean — Mix Location
- `mix_rot` · boolean — Mix Rotation
- `mix_scl` · boolean — Mix Scale
- `mix_mode` · enum · {REPLACE, BEFORE_FULL, BEFORE_SPLIT, AFTER_FULL, AFTER_SPLIT} — Specify how the copied and existing transformations are combined

## KinematicConstraint

Inverse Kinematics

- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `iterations` · int — Maximum number of solving iterations
- `pole_target` · pointer · -> Object — Object for pole rotation
- `pole_subtarget` · string
- `pole_angle` · float — Pole rotation offset
- `weight` · float — For Tree-IK: Weight of position control for this target
- `orient_weight` · float — For Tree-IK: Weight of orientation control for this target
- `chain_count` · int — How many bones are included in the IK effect - 0 uses all bones
- `use_tail` · boolean — Include bone's tail as last element in chain
- `reference_axis` · enum · {BONE, TARGET} — Constraint axis Lock options relative to Bone or Target reference
- `use_location` · boolean — Chain follows position of target
- `lock_location_x` · boolean — Constraint position along X axis
- `lock_location_y` · boolean — Constraint position along Y axis
- `lock_location_z` · boolean — Constraint position along Z axis
- `use_rotation` · boolean — Chain follows rotation of target
- `lock_rotation_x` · boolean — Constraint rotation along X axis
- `lock_rotation_y` · boolean — Constraint rotation along Y axis
- `lock_rotation_z` · boolean — Constraint rotation along Z axis
- `use_stretch` · boolean — Enable IK Stretching
- `ik_type` · enum · {COPY_POSE, DISTANCE}
- `limit_mode` · enum · {LIMITDIST_INSIDE, LIMITDIST_OUTSIDE, LIMITDIST_ONSURFACE} — Distances in relation to sphere of influence to allow
- `distance` · float — Radius of limiting sphere

## LimitDistanceConstraint

Limit the distance from target object

- `head_tail` · float — Target along length of bone: Head is 0, Tail is 1
- `use_bbone_shape` · boolean — Follow shape of B-Bone segments when calculating Head/Tail position
- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `distance` · float — Radius of limiting sphere
- `limit_mode` · enum · {LIMITDIST_INSIDE, LIMITDIST_OUTSIDE, LIMITDIST_ONSURFACE} — Distances in relation to sphere of influence to allow
- `use_transform_limit` · boolean — Transforms are affected by this constraint as well

## LimitLocationConstraint

Limit the location of the constrained object

- `use_min_x` · boolean — Use the minimum X value
- `use_min_y` · boolean — Use the minimum Y value
- `use_min_z` · boolean — Use the minimum Z value
- `use_max_x` · boolean — Use the maximum X value
- `use_max_y` · boolean — Use the maximum Y value
- `use_max_z` · boolean — Use the maximum Z value
- `min_x` · float — Lowest X value to allow
- `min_y` · float — Lowest Y value to allow
- `min_z` · float — Lowest Z value to allow
- `max_x` · float — Highest X value to allow
- `max_y` · float — Highest Y value to allow
- `max_z` · float — Highest Z value to allow
- `use_transform_limit` · boolean — Transform tools are affected by this constraint as well

## LimitRotationConstraint

Limit the rotation of the constrained object

- `use_limit_x` · boolean — Use the minimum X value
- `use_limit_y` · boolean — Use the minimum Y value
- `use_limit_z` · boolean — Use the minimum Z value
- `min_x` · float — Lower X angle bound
- `min_y` · float — Lower Y angle bound
- `min_z` · float — Lower Z angle bound
- `max_x` · float — Upper X angle bound
- `max_y` · float — Upper Y angle bound
- `max_z` · float — Upper Z angle bound
- `euler_order` · enum · {AUTO, XYZ, XZY, YXZ, YZX, ZXY, ZYX} — Explicitly specify the euler rotation order
- `use_transform_limit` · boolean — Transform tools are affected by this constraint as well
- `use_legacy_behavior` · boolean — Use the old semi-broken behavior that does not understand that rotations loop around

## LimitScaleConstraint

Limit the scaling of the constrained object

- `use_min_x` · boolean — Use the minimum X value
- `use_min_y` · boolean — Use the minimum Y value
- `use_min_z` · boolean — Use the minimum Z value
- `use_max_x` · boolean — Use the maximum X value
- `use_max_y` · boolean — Use the maximum Y value
- `use_max_z` · boolean — Use the maximum Z value
- `min_x` · float — Lowest X value to allow
- `min_y` · float — Lowest Y value to allow
- `min_z` · float — Lowest Z value to allow
- `max_x` · float — Highest X value to allow
- `max_y` · float — Highest Y value to allow
- `max_z` · float — Highest Z value to allow
- `use_transform_limit` · boolean — Transform tools are affected by this constraint as well

## LockedTrackConstraint

Point toward the target along the track axis, while locking the other axis

- `head_tail` · float — Target along length of bone: Head is 0, Tail is 1
- `use_bbone_shape` · boolean — Follow shape of B-Bone segments when calculating Head/Tail position
- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `track_axis` · enum · {TRACK_X, TRACK_Y, TRACK_Z, TRACK_NEGATIVE_X, TRACK_NEGATIVE_Y, TRACK_NEGATIVE_Z} — Axis that points to the target object
- `lock_axis` · enum · {LOCK_X, LOCK_Y, LOCK_Z} — Axis that points upward

## MaintainVolumeConstraint

Maintain a constant volume along a single scaling axis

- `free_axis` · enum · {SAMEVOL_X, SAMEVOL_Y, SAMEVOL_Z} — The free scaling axis of the object
- `mode` · enum · {STRICT, UNIFORM, SINGLE_AXIS} — The way the constraint treats original non-free axis scaling
- `volume` · float — Volume of the bone at rest

## ObjectConstraints

Collection of object constraints

- `active` · pointer · -> Constraint — Active Object constraint

## ObjectSolverConstraint

Lock motion to the reconstructed object movement

- `clip` · pointer · -> MovieClip — Movie Clip to get tracking data from
- `use_active_clip` · boolean — Use active clip defined in scene
- `set_inverse_pending` · boolean — Set to true to request recalculation of the inverse matrix
- `object` · string — Movie tracking object to follow
- `camera` · pointer · -> Object — Camera to which motion is parented (if empty active scene camera is used)

## PivotConstraint

Rotate around a different point

- `head_tail` · float — Target along length of bone: Head is 0, Tail is 1
- `use_bbone_shape` · boolean — Follow shape of B-Bone segments when calculating Head/Tail position
- `target` · pointer · -> Object — Target Object, defining the position of the pivot when defined
- `subtarget` · string
- `use_relative_location` · boolean — Offset will be an absolute point in space instead of relative to the target
- `offset` · float · [3] — Offset of pivot from target (when set), or from owner's location (when Fixed Position is off), or the absolute pivot point
- `rotation_range` · enum · {ALWAYS_ACTIVE, NX, NY, NZ, X, Y, Z} — Rotation range on which pivoting should occur

## PoseBoneConstraints

Collection of pose bone constraints

- `active` · pointer · -> Constraint — Active PoseChannel constraint

## RigidBodyConstraint

Constraint influencing Objects inside Rigid Body Simulation

- `type` · enum · {FIXED, POINT, HINGE, SLIDER, PISTON, GENERIC, GENERIC_SPRING, MOTOR} — Type of Rigid Body Constraint
- `spring_type` · enum · {SPRING1, SPRING2} — Which implementation of spring to use
- `enabled` · boolean — Enable this constraint
- `disable_collisions` · boolean — Disable collisions between constrained rigid bodies
- `object1` · pointer · -> Object — First Rigid Body Object to be constrained
- `object2` · pointer · -> Object — Second Rigid Body Object to be constrained
- `use_breaking` · boolean — Constraint can be broken if it receives an impulse above the threshold
- `breaking_threshold` · float — Impulse threshold that must be reached for the constraint to break
- `use_override_solver_iterations` · boolean — Override the number of solver iterations for this constraint
- `solver_iterations` · int — Number of constraint solver iterations made per simulation step (higher values are more accurate but slower)
- `use_limit_lin_x` · boolean — Limit translation on X axis
- `use_limit_lin_y` · boolean — Limit translation on Y axis
- `use_limit_lin_z` · boolean — Limit translation on Z axis
- `use_limit_ang_x` · boolean — Limit rotation around X axis
- `use_limit_ang_y` · boolean — Limit rotation around Y axis
- `use_limit_ang_z` · boolean — Limit rotation around Z axis
- `use_spring_x` · boolean — Enable spring on X axis
- `use_spring_y` · boolean — Enable spring on Y axis
- `use_spring_z` · boolean — Enable spring on Z axis
- `use_spring_ang_x` · boolean — Enable spring on X rotational axis
- `use_spring_ang_y` · boolean — Enable spring on Y rotational axis
- `use_spring_ang_z` · boolean — Enable spring on Z rotational axis
- `use_motor_lin` · boolean — Enable linear motor
- `use_motor_ang` · boolean — Enable angular motor
- `limit_lin_x_lower` · float — Lower limit of X axis translation
- `limit_lin_x_upper` · float — Upper limit of X axis translation
- `limit_lin_y_lower` · float — Lower limit of Y axis translation
- `limit_lin_y_upper` · float — Upper limit of Y axis translation
- `limit_lin_z_lower` · float — Lower limit of Z axis translation
- `limit_lin_z_upper` · float — Upper limit of Z axis translation
- `limit_ang_x_lower` · float — Lower limit of X axis rotation
- `limit_ang_x_upper` · float — Upper limit of X axis rotation
- `limit_ang_y_lower` · float — Lower limit of Y axis rotation
- `limit_ang_y_upper` · float — Upper limit of Y axis rotation
- `limit_ang_z_lower` · float — Lower limit of Z axis rotation
- `limit_ang_z_upper` · float — Upper limit of Z axis rotation
- `spring_stiffness_x` · float — Stiffness on the X axis
- `spring_stiffness_y` · float — Stiffness on the Y axis
- `spring_stiffness_z` · float — Stiffness on the Z axis
- `spring_stiffness_ang_x` · float — Stiffness on the X rotational axis
- `spring_stiffness_ang_y` · float — Stiffness on the Y rotational axis
- `spring_stiffness_ang_z` · float — Stiffness on the Z rotational axis
- `spring_damping_x` · float — Damping on the X axis
- `spring_damping_y` · float — Damping on the Y axis
- `spring_damping_z` · float — Damping on the Z axis
- `spring_damping_ang_x` · float — Damping on the X rotational axis
- `spring_damping_ang_y` · float — Damping on the Y rotational axis
- `spring_damping_ang_z` · float — Damping on the Z rotational axis
- `motor_lin_target_velocity` · float — Target linear motor velocity
- `motor_lin_max_impulse` · float — Maximum linear motor impulse
- `motor_ang_target_velocity` · float — Target angular motor velocity
- `motor_ang_max_impulse` · float — Maximum angular motor impulse

## ShrinkwrapConstraint

Create constraint-based shrinkwrap relationship

- `target` · pointer · -> Object — Target Mesh object
- `shrinkwrap_type` · enum · {NEAREST_SURFACE, PROJECT, NEAREST_VERTEX, TARGET_PROJECT} — Select type of shrinkwrap algorithm for target position
- `wrap_mode` · enum · {ON_SURFACE, INSIDE, OUTSIDE, OUTSIDE_SURFACE, ABOVE_SURFACE} — Select how to constrain the object to the target surface
- `distance` · float — Distance to Target
- `project_axis` · enum · {POS_X, POS_Y, POS_Z, NEG_X, NEG_Y, NEG_Z} — Axis constrain to
- `project_axis_space` · enum · {WORLD, CUSTOM, POSE, LOCAL_WITH_PARENT, LOCAL} — Space for the projection axis
- `project_limit` · float — Limit the distance used for projection (zero disables)
- `use_project_opposite` · boolean — Project in both specified and opposite directions
- `cull_face` · enum · {OFF, FRONT, BACK} — Stop vertices from projecting to a face on the target when facing towards/away
- `use_invert_cull` · boolean — When projecting in the opposite direction invert the face cull mode
- `use_track_normal` · boolean — Align the specified axis to the surface normal
- `track_axis` · enum · {TRACK_X, TRACK_Y, TRACK_Z, TRACK_NEGATIVE_X, TRACK_NEGATIVE_Y, TRACK_NEGATIVE_Z} — Axis that is aligned to the normal

## SplineIKConstraint

Align 'n' bones along a curve

- `target` · pointer · -> Object — Curve that controls this relationship
- `chain_count` · int — How many bones are included in the chain
- `joint_bindings` · float · [32] — (EXPERIENCED USERS ONLY) The relative positions of the joints along the chain, as percentages
- `use_chain_offset` · boolean — Offset the entire chain relative to the root joint
- `use_even_divisions` · boolean — Ignore the relative lengths of the bones when fitting to the curve
- `use_curve_radius` · boolean — Average radius of the endpoints is used to tweak the X and Z Scaling of the bones, on top of XZ Scale mode
- `xz_scale_mode` · enum · {NONE, BONE_ORIGINAL, INVERSE_PRESERVE, VOLUME_PRESERVE} — Method used for determining the scaling of the X and Z axes of the bones
- `y_scale_mode` · enum · {NONE, FIT_CURVE, BONE_ORIGINAL} — Method used for determining the scaling of the Y axis of the bones, on top of the shape and scaling of the curve itself
- `use_original_scale` · boolean — Apply volume preservation over the original scaling
- `bulge` · float — Factor between volume variation and stretching
- `use_bulge_min` · boolean — Use lower limit for volume variation
- `use_bulge_max` · boolean — Use upper limit for volume variation
- `bulge_min` · float — Minimum volume stretching factor
- `bulge_max` · float — Maximum volume stretching factor
- `bulge_smooth` · float — Strength of volume stretching clamping

## StretchToConstraint

Stretch to meet the target object

- `head_tail` · float — Target along length of bone: Head is 0, Tail is 1
- `use_bbone_shape` · boolean — Follow shape of B-Bone segments when calculating Head/Tail position
- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `volume` · enum · {VOLUME_XZX, VOLUME_X, VOLUME_Z, NO_VOLUME} — Maintain the object's volume as it stretches
- `keep_axis` · enum · {PLANE_X, PLANE_Z, SWING_Y} — The rotation type and axis order to use
- `rest_length` · float — Length at rest position
- `bulge` · float — Factor between volume variation and stretching
- `use_bulge_min` · boolean — Use lower limit for volume variation
- `use_bulge_max` · boolean — Use upper limit for volume variation
- `bulge_min` · float — Minimum volume stretching factor
- `bulge_max` · float — Maximum volume stretching factor
- `bulge_smooth` · float — Strength of volume stretching clamping

## TrackToConstraint

Aim the constrained object toward the target

- `head_tail` · float — Target along length of bone: Head is 0, Tail is 1
- `use_bbone_shape` · boolean — Follow shape of B-Bone segments when calculating Head/Tail position
- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `track_axis` · enum · {TRACK_X, TRACK_Y, TRACK_Z, TRACK_NEGATIVE_X, TRACK_NEGATIVE_Y, TRACK_NEGATIVE_Z} — Axis that points to the target object
- `up_axis` · enum · {UP_X, UP_Y, UP_Z} — Axis that points upward
- `use_target_z` · boolean — Target's Z axis, not World Z axis, will constrain the Up direction

## TransformCacheConstraint

Look up transformation from an external file

- `cache_file` · pointer · -> CacheFile
- `object_path` · string — Path to the object in the Alembic archive used to lookup the transform matrix

## TransformConstraint

Map transformations of the target to the object

- `target` · pointer · -> Object — Target object
- `subtarget` · string — Armature bone, mesh or lattice vertex group, ...
- `map_from` · enum · {LOCATION, ROTATION, SCALE} — The transformation type to use from the target
- `map_to` · enum · {LOCATION, ROTATION, SCALE} — The transformation type to affect on the constrained object
- `map_to_x_from` · enum · {X, Y, Z} — The source axis constrained object's X axis uses
- `map_to_y_from` · enum · {X, Y, Z} — The source axis constrained object's Y axis uses
- `map_to_z_from` · enum · {X, Y, Z} — The source axis constrained object's Z axis uses
- `use_motion_extrapolate` · boolean — Extrapolate ranges
- `from_rotation_mode` · enum · {AUTO, XYZ, XZY, YXZ, YZX, ZXY, ZYX, QUATERNION, SWING_TWIST_X, SWING_TWIST_Y, SWING_TWIST_Z} — Specify the type of rotation channels to use
- `to_euler_order` · enum · {AUTO, XYZ, XZY, YXZ, YZX, ZXY, ZYX} — Explicitly specify the output euler rotation order
- `from_min_x` · float — Bottom range of X axis source motion
- `from_min_y` · float — Bottom range of Y axis source motion
- `from_min_z` · float — Bottom range of Z axis source motion
- `from_max_x` · float — Top range of X axis source motion
- `from_max_y` · float — Top range of Y axis source motion
- `from_max_z` · float — Top range of Z axis source motion
- `to_min_x` · float — Bottom range of X axis destination motion
- `to_min_y` · float — Bottom range of Y axis destination motion
- `to_min_z` · float — Bottom range of Z axis destination motion
- `to_max_x` · float — Top range of X axis destination motion
- `to_max_y` · float — Top range of Y axis destination motion
- `to_max_z` · float — Top range of Z axis destination motion
- `mix_mode` · enum · {REPLACE, ADD} — Specify how to combine the new location with original
- `from_min_x_rot` · float — Bottom range of X axis source motion
- `from_min_y_rot` · float — Bottom range of Y axis source motion
- `from_min_z_rot` · float — Bottom range of Z axis source motion
- `from_max_x_rot` · float — Top range of X axis source motion
- `from_max_y_rot` · float — Top range of Y axis source motion
- `from_max_z_rot` · float — Top range of Z axis source motion
- `to_min_x_rot` · float — Bottom range of X axis destination motion
- `to_min_y_rot` · float — Bottom range of Y axis destination motion
- `to_min_z_rot` · float — Bottom range of Z axis destination motion
- `to_max_x_rot` · float — Top range of X axis destination motion
- `to_max_y_rot` · float — Top range of Y axis destination motion
- `to_max_z_rot` · float — Top range of Z axis destination motion
- `mix_mode_rot` · enum · {REPLACE, ADD, BEFORE, AFTER} — Specify how to combine the new rotation with original
- `from_min_x_scale` · float — Bottom range of X axis source motion
- `from_min_y_scale` · float — Bottom range of Y axis source motion
- `from_min_z_scale` · float — Bottom range of Z axis source motion
- `from_max_x_scale` · float — Top range of X axis source motion
- `from_max_y_scale` · float — Top range of Y axis source motion
- `from_max_z_scale` · float — Top range of Z axis source motion
- `to_min_x_scale` · float — Bottom range of X axis destination motion
- `to_min_y_scale` · float — Bottom range of Y axis destination motion
- `to_min_z_scale` · float — Bottom range of Z axis destination motion
- `to_max_x_scale` · float — Top range of X axis destination motion
- `to_max_y_scale` · float — Top range of Y axis destination motion
- `to_max_z_scale` · float — Top range of Z axis destination motion
- `mix_mode_scale` · enum · {REPLACE, MULTIPLY} — Specify how to combine the new scale with original
