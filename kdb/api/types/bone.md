---
area: api-types
group: bone
version: 5.2.2
---

# Blender 5.2.2 RNA types: bone

## ArmatureBones

Collection of armature bones

- `active` · pointer · -> Bone — Armature's active bone

## ArmatureEditBones

Collection of armature edit bones

- `active` · pointer · -> EditBone — Armatures active edit bone

## Bone

Bone in an Armature data-block

- `parent` · pointer · -> Bone · read-only — Parent bone (in same Armature)
- `children` · collection · read-only — Bones which are children of this bone
- `collections` · collection · [] of BoneCollectionMemberships · read-only — Bone Collections that contain this bone
- `name` · string
- `color` · pointer · -> BoneColor · read-only
- `display_type` · enum · {ARMATURE_DEFINED, OCTAHEDRAL, STICK, BBONE, ENVELOPE, WIRE}
- `use_connect` · boolean · read-only — When bone has a parent, bone's head is stuck to the parent's tail
- `use_inherit_rotation` · boolean — Bone inherits rotation or scale from parent bone
- `use_envelope_multiply` · boolean — When deforming bone, multiply effects of Vertex Group weights with Envelope influence
- `use_deform` · boolean — Enable Bone to deform geometry
- `inherit_scale` · enum · {FULL, FIX_SHEAR, ALIGNED, AVERAGE, NONE, NONE_LEGACY} — Specifies how the bone inherits scaling from the parent bone
- `use_local_location` · boolean — Bone location is set in local space
- `use_relative_parent` · boolean — Object children will use relative transform, like deform
- `show_wire` · boolean — Bone is always displayed in wireframe regardless of viewport shading mode (useful for non-obstructive custom bone shapes)
- `use_cyclic_offset` · boolean — When bone does not have a parent, it receives cyclic offset effects (Deprecated)
- `hide_select` · boolean — Bone is able to be selected
- `envelope_distance` · float — Bone deformation distance (for Envelope deform only)
- `envelope_weight` · float — Bone deformation weight (for Envelope deform only)
- `head_radius` · float — Radius of head of bone (for Envelope deform only)
- `tail_radius` · float — Radius of tail of bone (for Envelope deform only)
- `bbone_segments` · int — Number of subdivisions of bone (for B-Bones only)
- `bbone_mapping_mode` · enum · {STRAIGHT, CURVED} — Selects how the vertices are mapped to B-Bone segments based on their position
- `bbone_x` · float — B-Bone X size
- `bbone_z` · float — B-Bone Z size
- `bbone_handle_type_start` · enum · {AUTO, ABSOLUTE, RELATIVE, TANGENT} — Selects how the start handle of the B-Bone is computed
- `bbone_custom_handle_start` · pointer · -> Bone — Bone that serves as the start handle for the B-Bone curve
- `bbone_handle_use_scale_start` · boolean — Multiply B-Bone Scale In channels by the local scale values of the start handle. This is done after the Scale Easing option and isn't affected by it.
- `bbone_handle_use_ease_start` · boolean — Multiply the B-Bone Ease In channel by the local Y scale value of the start handle. This is done after the Scale Easing option and isn't affected by it.
- `bbone_handle_type_end` · enum · {AUTO, ABSOLUTE, RELATIVE, TANGENT} — Selects how the end handle of the B-Bone is computed
- `bbone_custom_handle_end` · pointer · -> Bone — Bone that serves as the end handle for the B-Bone curve
- `bbone_handle_use_scale_end` · boolean — Multiply B-Bone Scale Out channels by the local scale values of the end handle. This is done after the Scale Easing option and isn't affected by it.
- `bbone_handle_use_ease_end` · boolean — Multiply the B-Bone Ease Out channel by the local Y scale value of the end handle. This is done after the Scale Easing option and isn't affected by it.
- `bbone_rollin` · float — Roll offset for the start of the B-Bone, adjusts twist
- `bbone_rollout` · float — Roll offset for the end of the B-Bone, adjusts twist
- `use_endroll_as_inroll` · boolean — Add Roll Out of the Start Handle bone to the Roll In value
- `bbone_curveinx` · float — X-axis handle offset for start of the B-Bone's curve, adjusts curvature
- `bbone_curveinz` · float — Z-axis handle offset for start of the B-Bone's curve, adjusts curvature
- `bbone_curveoutx` · float — X-axis handle offset for end of the B-Bone's curve, adjusts curvature
- `bbone_curveoutz` · float — Z-axis handle offset for end of the B-Bone's curve, adjusts curvature
- `bbone_easein` · float — Length of first Bézier Handle (for B-Bones only)
- `bbone_easeout` · float — Length of second Bézier Handle (for B-Bones only)
- `use_scale_easing` · boolean — Multiply the final easing values by the Scale In/Out Y factors
- `bbone_scalein` · float · [3] — Scale factors for the start of the B-Bone, adjusts thickness (for tapering effects)
- `bbone_scaleout` · float · [3] — Scale factors for the end of the B-Bone, adjusts thickness (for tapering effects)
- `hide` · boolean — Bone is not visible when it is in Edit Mode
- `matrix` · float · [9] · read-only — 3×3 bone matrix
- `matrix_local` · float · [16] · read-only — 4×4 bone matrix relative to armature
- `tail` · float · [3] · read-only — Location of tail end of the bone relative to its parent
- `tail_local` · float · [3] · read-only — Location of tail end of the bone relative to armature
- `head` · float · [3] · read-only — Location of head end of the bone relative to its parent
- `head_local` · float · [3] · read-only — Location of head end of the bone relative to armature
- `length` · float · read-only — Length of the bone

## BoneCollection

Bone collection in an Armature data-block

- `name` · string — Unique within the Armature
- `is_expanded` · boolean — This bone collection is expanded in the bone collections tree view
- `is_visible` · boolean — Bones in this collection will be visible in pose/object mode
- `is_visible_ancestors` · boolean · read-only — True when all of the ancestors of this bone collection are marked as visible; always True for root bone collections
- `is_visible_effectively` · boolean · read-only — Whether this bone collection is effectively visible in the viewport. This is True when this bone collection and all of its ancestors are visible, or when it is marked as 'solo'.
- `is_solo` · boolean — Show only this bone collection, and others also marked as 'solo'
- `is_local_override` · boolean · read-only — This collection was added via a library override in the current blend file
- `is_editable` · boolean · read-only — This collection is owned by a local Armature, or was added via a library override in the current blend file
- `bones` · collection · read-only — Bones assigned to this bone collection. In armature edit mode this will always return an empty list of bones, as the bone collection memberships are only synchronized when exiting edit mode.
- `children` · collection · read-only
- `parent` · pointer · -> BoneCollection — Parent bone collection. Note that accessing this requires a scan of all the bone collections to find the parent.
- `index` · int · read-only — Index of this bone collection in the armature.collections_all array. Note that finding this index requires a scan of all the bone collections, so do access this with care.
- `child_number` · int — Index of this collection into its parent's list of children. Note that finding this index requires a scan of all the bone collections, so do access this with care.

## BoneCollections

The Bone Collections of this Armature

- `active` · pointer · -> BoneCollection — Armature's active bone collection
- `active_index` · int — The index of the Armature's active bone collection; -1 when there is no active collection. Note that this is indexing the underlying array of bone collections, which may not be in the order you expect. Root collections are listed first, and siblings are always sequential. Apart from that, bone collections can be in any order, and thus incrementing or decrementing this index can make the active bone collection jump around in unexpected ways. For a more predictable interface, use ``active`` or ``active_name``.
- `active_name` · string — The name of the Armature's active bone collection; empty when there is no active collection
- `is_solo_active` · boolean · read-only — Read-only flag that indicates there is at least one bone collection marked as 'solo'

## BoneColor

Theme color or custom color of a bone

- `palette` · enum · {DEFAULT, THEME01, THEME02, THEME03, THEME04, THEME05, THEME06, THEME07, THEME08, THEME09, THEME10, THEME11, THEME12, THEME13, THEME14, THEME15, THEME16, THEME17, THEME18, THEME19, THEME20, CUSTOM} — Color palette to use
- `is_custom` · boolean · read-only — A color palette is user-defined, instead of using a theme-defined one
- `custom` · pointer · -> ThemeBoneColorSet · read-only — The custom bone colors, used when palette is 'CUSTOM'

## EditBone

Edit mode bone in an armature data-block

- `collections` · collection · read-only — Bone Collections that contain this bone
- `parent` · pointer · -> EditBone — Parent edit bone (in same Armature)
- `roll` · float — Bone rotation around head-tail axis
- `head` · float · [3] — Location of head end of the bone
- `tail` · float · [3] — Location of tail end of the bone
- `length` · float — Length of the bone. Changing moves the tail end.
- `name` · string
- `color` · pointer · -> BoneColor · read-only
- `display_type` · enum · {ARMATURE_DEFINED, OCTAHEDRAL, STICK, BBONE, ENVELOPE, WIRE}
- `use_connect` · boolean — When bone has a parent, bone's head is stuck to the parent's tail
- `use_inherit_rotation` · boolean — Bone inherits rotation or scale from parent bone
- `use_envelope_multiply` · boolean — When deforming bone, multiply effects of Vertex Group weights with Envelope influence
- `use_deform` · boolean — Enable Bone to deform geometry
- `inherit_scale` · enum · {FULL, FIX_SHEAR, ALIGNED, AVERAGE, NONE, NONE_LEGACY} — Specifies how the bone inherits scaling from the parent bone
- `use_local_location` · boolean — Bone location is set in local space
- `use_relative_parent` · boolean — Object children will use relative transform, like deform
- `show_wire` · boolean — Bone is always displayed in wireframe regardless of viewport shading mode (useful for non-obstructive custom bone shapes)
- `use_cyclic_offset` · boolean — When bone does not have a parent, it receives cyclic offset effects (Deprecated)
- `hide_select` · boolean — Bone is able to be selected
- `envelope_distance` · float — Bone deformation distance (for Envelope deform only)
- `envelope_weight` · float — Bone deformation weight (for Envelope deform only)
- `head_radius` · float — Radius of head of bone (for Envelope deform only)
- `tail_radius` · float — Radius of tail of bone (for Envelope deform only)
- `bbone_segments` · int — Number of subdivisions of bone (for B-Bones only)
- `bbone_mapping_mode` · enum · {STRAIGHT, CURVED} — Selects how the vertices are mapped to B-Bone segments based on their position
- `bbone_x` · float — B-Bone X size
- `bbone_z` · float — B-Bone Z size
- `bbone_handle_type_start` · enum · {AUTO, ABSOLUTE, RELATIVE, TANGENT} — Selects how the start handle of the B-Bone is computed
- `bbone_custom_handle_start` · pointer · -> EditBone — Bone that serves as the start handle for the B-Bone curve
- `bbone_handle_use_scale_start` · boolean — Multiply B-Bone Scale In channels by the local scale values of the start handle. This is done after the Scale Easing option and isn't affected by it.
- `bbone_handle_use_ease_start` · boolean — Multiply the B-Bone Ease In channel by the local Y scale value of the start handle. This is done after the Scale Easing option and isn't affected by it.
- `bbone_handle_type_end` · enum · {AUTO, ABSOLUTE, RELATIVE, TANGENT} — Selects how the end handle of the B-Bone is computed
- `bbone_custom_handle_end` · pointer · -> EditBone — Bone that serves as the end handle for the B-Bone curve
- `bbone_handle_use_scale_end` · boolean — Multiply B-Bone Scale Out channels by the local scale values of the end handle. This is done after the Scale Easing option and isn't affected by it.
- `bbone_handle_use_ease_end` · boolean — Multiply the B-Bone Ease Out channel by the local Y scale value of the end handle. This is done after the Scale Easing option and isn't affected by it.
- `bbone_rollin` · float — Roll offset for the start of the B-Bone, adjusts twist
- `bbone_rollout` · float — Roll offset for the end of the B-Bone, adjusts twist
- `use_endroll_as_inroll` · boolean — Add Roll Out of the Start Handle bone to the Roll In value
- `bbone_curveinx` · float — X-axis handle offset for start of the B-Bone's curve, adjusts curvature
- `bbone_curveinz` · float — Z-axis handle offset for start of the B-Bone's curve, adjusts curvature
- `bbone_curveoutx` · float — X-axis handle offset for end of the B-Bone's curve, adjusts curvature
- `bbone_curveoutz` · float — Z-axis handle offset for end of the B-Bone's curve, adjusts curvature
- `bbone_easein` · float — Length of first Bézier Handle (for B-Bones only)
- `bbone_easeout` · float — Length of second Bézier Handle (for B-Bones only)
- `use_scale_easing` · boolean — Multiply the final easing values by the Scale In/Out Y factors
- `bbone_scalein` · float · [3] — Scale factors for the start of the B-Bone, adjusts thickness (for tapering effects)
- `bbone_scaleout` · float · [3] — Scale factors for the end of the B-Bone, adjusts thickness (for tapering effects)
- `hide` · boolean — Bone is not visible when in Edit Mode
- `lock` · boolean — Bone is not able to be transformed when in Edit Mode
- `select` · boolean
- `select_head` · boolean
- `select_tail` · boolean
- `matrix` · float · [16] — Matrix combining location and rotation of the bone (head position, direction and roll), in armature space (does not include/support bone's length/size)

## PoseBone

Channel defining pose data for a bone in a Pose

- `constraints` · collection · [] of PoseBoneConstraints · read-only — Constraints that act on this pose channel
- `name` · string
- `motion_path` · pointer · -> MotionPath · read-only — Motion Path for this element
- `bone` · pointer · -> Bone · read-only — Bone associated with this PoseBone
- `parent` · pointer · -> PoseBone · read-only — Parent of this pose bone
- `child` · pointer · -> PoseBone · read-only — Child of this pose bone
- `location` · float · [3]
- `scale` · float · [3]
- `rotation_quaternion` · float · [4] — Rotation in Quaternions
- `rotation_axis_angle` · float · [4] — Angle of Rotation for Axis-Angle rotation representation
- `rotation_euler` · float · [3] — Rotation in Eulers
- `rotation_mode` · enum · {QUATERNION, XYZ, XZY, YXZ, YZX, ZXY, ZYX, AXIS_ANGLE} — The kind of rotation to apply, values from other rotation modes are not used
- `bbone_rollin` · float — Roll offset for the start of the B-Bone, adjusts twist
- `bbone_rollout` · float — Roll offset for the end of the B-Bone, adjusts twist
- `bbone_curveinx` · float — X-axis handle offset for start of the B-Bone's curve, adjusts curvature
- `bbone_curveinz` · float — Z-axis handle offset for start of the B-Bone's curve, adjusts curvature
- `bbone_curveoutx` · float — X-axis handle offset for end of the B-Bone's curve, adjusts curvature
- `bbone_curveoutz` · float — Z-axis handle offset for end of the B-Bone's curve, adjusts curvature
- `bbone_easein` · float — Length of first Bézier Handle (for B-Bones only)
- `bbone_easeout` · float — Length of second Bézier Handle (for B-Bones only)
- `bbone_scalein` · float · [3] — Scale factors for the start of the B-Bone, adjusts thickness (for tapering effects)
- `bbone_scaleout` · float · [3] — Scale factors for the end of the B-Bone, adjusts thickness (for tapering effects)
- `bbone_custom_handle_start` · pointer · -> PoseBone · read-only — Bone that serves as the start handle for the B-Bone curve
- `bbone_custom_handle_end` · pointer · -> PoseBone · read-only — Bone that serves as the end handle for the B-Bone curve
- `matrix_channel` · float · [16] · read-only — 4×4 matrix of the bone's location/rotation/scale channels (including animation and drivers) and the effect of bone constraints
- `matrix_basis` · float · [16] — Alternative access to location/scale/rotation relative to the parent and own rest bone
- `matrix` · float · [16] — Final 4×4 matrix after constraints and drivers are applied, in the armature object space
- `head` · float · [3] · read-only — Location of head of the channel's bone
- `tail` · float · [3] · read-only — Location of tail of the channel's bone
- `length` · float · read-only — Length of the bone
- `is_in_ik_chain` · boolean · read-only — Is part of an IK chain
- `lock_ik_x` · boolean — Disallow movement around the X axis
- `lock_ik_y` · boolean — Disallow movement around the Y axis
- `lock_ik_z` · boolean — Disallow movement around the Z axis
- `use_ik_limit_x` · boolean — Limit movement around the X axis
- `use_ik_limit_y` · boolean — Limit movement around the Y axis
- `use_ik_limit_z` · boolean — Limit movement around the Z axis
- `use_ik_rotation_control` · boolean — Apply channel rotation as IK constraint
- `use_ik_linear_control` · boolean — Apply channel size as IK constraint if stretching is enabled
- `ik_min_x` · float — Minimum angles for IK Limit
- `ik_max_x` · float — Maximum angles for IK Limit
- `ik_min_y` · float — Minimum angles for IK Limit
- `ik_max_y` · float — Maximum angles for IK Limit
- `ik_min_z` · float — Minimum angles for IK Limit
- `ik_max_z` · float — Maximum angles for IK Limit
- `ik_stiffness_x` · float — IK stiffness around the X axis
- `ik_stiffness_y` · float — IK stiffness around the Y axis
- `ik_stiffness_z` · float — IK stiffness around the Z axis
- `ik_stretch` · float — Allow scaling of the bone for IK
- `ik_rotation_weight` · float — Weight of rotation constraint for IK
- `ik_linear_weight` · float — Weight of scale constraint for IK
- `custom_shape` · pointer · -> Object — Object that defines custom display shape for this bone
- `custom_shape_scale_xyz` · float · [3] — Adjust the size of the custom shape
- `custom_shape_translation` · float · [3] — Adjust the location of the custom shape
- `custom_shape_rotation_euler` · float · [3] — Adjust the rotation of the custom shape
- `use_transform_at_custom_shape` · boolean — The location and orientation of the Custom Shape Transform bone will be used for transform gizmos and for other transform operators in the 3D Viewport. When disabled, the 3D Viewport will still use the actual bone transform for these, even when the custom bone shape transform is overridden.
- `use_transform_around_custom_shape` · boolean — Transform the bone as if it was a child of the Custom Shape Transform bone. This can be useful when combining shape-key and armature deformations.
- `use_custom_shape_bone_size` · boolean — Scale the custom object by the bone length
- `hide` · boolean — Bone is not visible except for Edit Mode
- `select` · boolean — Bone is selected in Pose Mode
- `custom_shape_transform` · pointer · -> PoseBone — Bone that defines the display transform of this custom shape
- `custom_shape_wire_width` · float — Adjust the line thickness of custom shapes
- `color` · pointer · -> BoneColor · read-only
- `lock_location` · boolean — Lock editing of location when transforming
- `lock_rotation` · boolean — Lock editing of rotation when transforming
- `lock_rotation_w` · boolean — Lock editing of 'angle' component of four-component rotations when transforming
- `lock_rotations_4d` · boolean — Lock editing of four component rotations by components (instead of as Eulers)
- `lock_scale` · boolean — Lock editing of scale when transforming

## ThemeBoneColorSet

Theme settings for bone color sets

- `normal` · float · [3] — Color used for the surface of bones
- `select` · float · [3] — Color used for selected bones
- `active` · float · [3] — Color used for active bones
- `show_colored_constraints` · boolean — Allow the use of colors indicating constraints/keyed status
