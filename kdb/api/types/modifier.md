---
area: api-types
group: modifier
version: 5.2.2
---

# Blender 5.2.2 RNA types: modifier

## ArmatureModifier

Armature deformation modifier

- `object` · pointer · -> Object — Armature object to deform with
- `use_bone_envelopes` · boolean — Bind Bone envelopes to armature modifier
- `use_vertex_groups` · boolean — Bind vertex groups to armature modifier
- `use_deform_preserve_volume` · boolean — Deform rotation interpolation with quaternions
- `use_multi_modifier` · boolean — Use same input as previous modifier, and mix results using overall vgroup
- `vertex_group` · string — Name of Vertex Group which determines influence of modifier per point
- `invert_vertex_group` · boolean — Invert vertex group influence

## ArrayModifier

Array duplication modifier

- `fit_type` · enum · {FIXED_COUNT, FIT_LENGTH, FIT_CURVE} — Array length calculation method
- `count` · int — Number of duplicates to make
- `fit_length` · float — Length to fit array within
- `curve` · pointer · -> Object — Curve object to fit array length to
- `use_constant_offset` · boolean — Add a constant offset
- `constant_offset_displace` · float · [3] — Value for the distance between arrayed items
- `use_relative_offset` · boolean — Add an offset relative to the object's bounding box
- `relative_offset_displace` · float · [3] — The size of the geometry will determine the distance between arrayed items
- `use_merge_vertices` · boolean — Merge vertices in adjacent duplicates
- `use_merge_vertices_cap` · boolean — Merge vertices in first and last duplicates
- `merge_threshold` · float — Limit below which to merge vertices
- `use_object_offset` · boolean — Add another object's transformation to the total offset
- `offset_object` · pointer · -> Object — Use the location and rotation of another object to determine the distance and rotational change between arrayed items
- `start_cap` · pointer · -> Object — Mesh object to use as a start cap
- `end_cap` · pointer · -> Object — Mesh object to use as an end cap
- `offset_u` · float — Amount to offset array UVs on the U axis
- `offset_v` · float — Amount to offset array UVs on the V axis

## BevelModifier

Bevel modifier to make edges and vertices more rounded

- `width` · float — Bevel amount
- `width_pct` · float — Bevel amount for percentage method
- `segments` · int — Number of segments for round edges/verts
- `affect` · enum · {VERTICES, EDGES} — Affect edges or vertices
- `limit_method` · enum · {NONE, ANGLE, WEIGHT, VGROUP}
- `edge_weight` · string — Attribute name for edge weight
- `vertex_weight` · string — Attribute name for vertex weight
- `angle_limit` · float — Angle above which to bevel edges
- `vertex_group` · string — Vertex group name
- `invert_vertex_group` · boolean — Invert vertex group influence
- `use_clamp_overlap` · boolean — Clamp the width to avoid overlap
- `offset_type` · enum · {OFFSET, WIDTH, DEPTH, PERCENT, ABSOLUTE} — What distance Width measures
- `profile_type` · enum · {SUPERELLIPSE, CUSTOM} — The type of shape used to rebuild a beveled section
- `profile` · float — The profile shape (0.5 = round)
- `material` · int — Material index of generated faces, -1 for automatic
- `loop_slide` · boolean — Prefer sliding along edges to having even widths
- `mark_seam` · boolean — Mark Seams along beveled edges
- `mark_sharp` · boolean — Mark beveled edges as sharp
- `harden_normals` · boolean — Match normals of new faces to adjacent faces
- `face_strength_mode` · enum · {FSTR_NONE, FSTR_NEW, FSTR_AFFECTED, FSTR_ALL} — Whether to set face strength, and which faces to set it on
- `miter_outer` · enum · {MITER_SHARP, MITER_PATCH, MITER_ARC} — Pattern to use for outside of miters
- `miter_inner` · enum · {MITER_SHARP, MITER_ARC} — Pattern to use for inside of miters
- `spread` · float — Spread distance for inner miter arcs
- `custom_profile` · pointer · -> CurveProfile · read-only — The path for the custom profile
- `vmesh_method` · enum · {ADJ, CUTOFF} — The method to use to create the mesh at intersections

## BooleanModifier

Boolean operations modifier

- `object` · pointer · -> Object — Mesh object to use for Boolean operation
- `collection` · pointer · -> Collection — Use mesh objects in this collection for Boolean operation
- `operation` · enum · {INTERSECT, UNION, DIFFERENCE}
- `operand_type` · enum · {OBJECT, COLLECTION}
- `double_threshold` · float — Threshold for checking overlapping geometry
- `solver` · enum · {FLOAT, EXACT, MANIFOLD} — Method for calculating booleans
- `use_self` · boolean — Allow self-intersection in operands
- `use_hole_tolerant` · boolean — Better results when there are holes (slower)
- `material_mode` · enum · {INDEX, TRANSFER} — Method for setting materials on the new faces
- `debug_options` · enum · {SEPARATE, NO_DISSOLVE, NO_CONNECT_REGIONS} — Debugging options, only when started with '-d'

## BrightContrastModifier

Bright/contrast modifier data for sequence strip

- `bright` · float — Adjust the luminosity of the colors
- `contrast` · float — Adjust the difference in luminosity between pixels
- `open_mask_input_panel` · boolean

## BuildModifier

Build effect modifier

- `frame_start` · float — Start frame of the effect
- `frame_duration` · float — Total time the build effect requires
- `use_reverse` · boolean — Deconstruct the mesh instead of building it
- `use_random_order` · boolean — Randomize the faces or edges during build
- `seed` · int — Seed for random if used

## CastModifier

Modifier to cast to other shapes

- `cast_type` · enum · {SPHERE, CYLINDER, CUBOID} — Target object shape
- `object` · pointer · -> Object — Control object: if available, its location determines the center of the effect
- `invert_vertex_group` · boolean — Invert vertex group influence
- `use_x` · boolean
- `use_y` · boolean
- `use_z` · boolean
- `use_radius_as_size` · boolean — Use radius as size of projection shape (0 = auto)
- `use_transform` · boolean — Use object transform to control projection shape
- `factor` · float
- `radius` · float — Only deform vertices within this distance from the center of the effect (leave as 0 for infinite.)
- `size` · float — Size of projection shape (leave as 0 for auto)
- `vertex_group` · string — Vertex group name

## ClothModifier

Cloth simulation modifier

- `settings` · pointer · -> ClothSettings · read-only
- `collision_settings` · pointer · -> ClothCollisionSettings · read-only
- `solver_result` · pointer · -> ClothSolverResult · read-only
- `point_cache` · pointer · -> PointCache · read-only
- `hair_grid_min` · float · [3] · read-only
- `hair_grid_max` · float · [3] · read-only
- `hair_grid_resolution` · int · [3] · read-only

## CollisionModifier

Collision modifier defining modifier stack position used for collision

- `settings` · pointer · -> CollisionSettings · read-only

## ColorBalanceModifier

Color balance modifier for sequence strip

- `color_balance` · pointer · -> StripColorBalanceData · read-only
- `color_multiply` · float — Multiply the intensity of each pixel
- `open_mask_input_panel` · boolean

## CorrectiveSmoothModifier

Correct distortion caused by deformation

- `factor` · float — Smooth effect factor
- `iterations` · int
- `scale` · float — Compensate for scale applied by other modifiers
- `rest_source` · enum · {ORCO, BIND} — Select the source of rest positions
- `smooth_type` · enum · {SIMPLE, LENGTH_WEIGHTED} — Method used for smoothing
- `invert_vertex_group` · boolean — Invert vertex group influence
- `vertex_group` · string — Name of Vertex Group which determines influence of modifier per point
- `is_bind` · boolean · read-only
- `use_only_smooth` · boolean — Apply smoothing without reconstructing the surface
- `use_pin_boundary` · boolean — Excludes boundary vertices from being smoothed

## CurveModifier

Curve deformation modifier

- `object` · pointer · -> Object — Curve object to deform with
- `vertex_group` · string — Name of Vertex Group which determines influence of modifier per point
- `invert_vertex_group` · boolean — Invert vertex group influence
- `deform_axis` · enum · {POS_X, POS_Y, POS_Z, NEG_X, NEG_Y, NEG_Z} — The axis that the curve deforms along

## CurvesModifier

RGB curves modifier for sequence strip

- `curve_mapping` · pointer · -> CurveMapping · read-only
- `open_mask_input_panel` · boolean

## DataTransferModifier

Modifier transferring some data from a source mesh

- `object` · pointer · -> Object — Object to transfer data from
- `use_object_transform` · boolean — Evaluate source and destination meshes in global space
- `use_vert_data` · boolean — Enable vertex data transfer
- `use_edge_data` · boolean — Enable edge data transfer
- `use_loop_data` · boolean — Enable face corner data transfer
- `use_poly_data` · boolean — Enable face data transfer
- `data_types_verts` · enum · {VGROUP_WEIGHTS, BEVEL_WEIGHT_VERT, COLOR_VERTEX} — Which vertex data layers to transfer
- `data_types_edges` · enum · {SHARP_EDGE, SEAM, CREASE, BEVEL_WEIGHT_EDGE, FREESTYLE_EDGE} — Which edge data layers to transfer
- `data_types_loops` · enum · {CUSTOM_NORMAL, COLOR_CORNER, UV} — Which face corner data layers to transfer
- `data_types_polys` · enum · {SMOOTH, FREESTYLE_FACE} — Which face data layers to transfer
- `vert_mapping` · enum · {TOPOLOGY, NEAREST, EDGE_NEAREST, EDGEINTERP_NEAREST, POLY_NEAREST, POLYINTERP_NEAREST, POLYINTERP_VNORPROJ} — Method used to map source vertices to destination ones
- `edge_mapping` · enum · {TOPOLOGY, VERT_NEAREST, NEAREST, POLY_NEAREST, EDGEINTERP_VNORPROJ} — Method used to map source edges to destination ones
- `loop_mapping` · enum · {TOPOLOGY, NEAREST_NORMAL, NEAREST_POLYNOR, NEAREST_POLY, POLYINTERP_NEAREST, POLYINTERP_LNORPROJ} — Method used to map source faces' corners to destination ones
- `poly_mapping` · enum · {TOPOLOGY, NEAREST, NORMAL, POLYINTERP_PNORPROJ} — Method used to map source faces to destination ones
- `use_max_distance` · boolean — Source elements must be closer than given distance from destination one
- `max_distance` · float — Maximum allowed distance between source and destination element, for non-topology mappings
- `ray_radius` · float — 'Width' of rays (especially useful when raycasting against vertices or edges)
- `islands_precision` · float — Factor controlling precision of islands handling (typically, 0.1 should be enough, higher values can make things really slow)
- `layers_vgroup_select_src` · enum · {ACTIVE, ALL, BONE_SELECT, BONE_DEFORM} — Which layers to transfer, in case of multi-layers types
- `layers_vcol_vert_select_src` · enum · {ACTIVE, ALL, BONE_SELECT, BONE_DEFORM} — Which layers to transfer, in case of multi-layers types
- `layers_vcol_loop_select_src` · enum · {ACTIVE, ALL, BONE_SELECT, BONE_DEFORM} — Which layers to transfer, in case of multi-layers types
- `layers_uv_select_src` · enum · {ACTIVE, ALL, BONE_SELECT, BONE_DEFORM} — Which layers to transfer, in case of multi-layers types
- `layers_vgroup_select_dst` · enum · {ACTIVE, NAME, INDEX} — How to match source and destination layers
- `layers_vcol_vert_select_dst` · enum · {ACTIVE, NAME, INDEX} — How to match source and destination layers
- `layers_vcol_loop_select_dst` · enum · {ACTIVE, NAME, INDEX} — How to match source and destination layers
- `layers_uv_select_dst` · enum · {ACTIVE, NAME, INDEX} — How to match source and destination layers
- `mix_mode` · enum · {REPLACE, ABOVE_THRESHOLD, BELOW_THRESHOLD, MIX, ADD, SUB, MUL} — How to affect destination elements with source values
- `mix_factor` · float — Factor to use when applying data to destination (exact behavior depends on mix mode, multiplied with weights from vertex group when defined)
- `vertex_group` · string — Vertex group name for selecting the affected areas
- `invert_vertex_group` · boolean — Invert vertex group influence

## DecimateModifier

Decimation modifier

- `decimate_type` · enum · {COLLAPSE, UNSUBDIV, DISSOLVE}
- `ratio` · float — Ratio of triangles to reduce to (collapse only)
- `iterations` · int — Number of times reduce the geometry (unsubdivide only)
- `angle_limit` · float — Only dissolve angles below this (planar only)
- `vertex_group` · string — Vertex group name (collapse only)
- `invert_vertex_group` · boolean — Invert vertex group influence (collapse only)
- `use_collapse_triangulate` · boolean — Keep triangulated faces resulting from decimation (collapse only)
- `use_symmetry` · boolean — Maintain symmetry on an axis
- `symmetry_axis` · enum · {X, Y, Z} — Axis of symmetry
- `vertex_group_factor` · float — Vertex group strength
- `use_dissolve_boundaries` · boolean — Dissolve all vertices in between face boundaries (planar only)
- `delimit` · enum · {NORMAL, MATERIAL, SEAM, SHARP, UV} — Limit merging geometry
- `face_count` · int · read-only — The current number of faces in the decimated mesh

## DisplaceModifier

Displacement modifier

- `vertex_group` · string — Name of Vertex Group which determines influence of modifier per point
- `mid_level` · float — Material value that gives no displacement
- `strength` · float — Amount to displace geometry
- `direction` · enum · {X, Y, Z, NORMAL, CUSTOM_NORMAL, RGB_TO_XYZ}
- `space` · enum · {LOCAL, GLOBAL}
- `invert_vertex_group` · boolean — Invert vertex group influence
- `texture` · pointer · -> Texture
- `texture_coords` · enum · {LOCAL, GLOBAL, OBJECT, UV}
- `uv_layer` · string — UV map name
- `texture_coords_object` · pointer · -> Object — Object to set the texture coordinates
- `texture_coords_bone` · string — Bone to set the texture coordinates

## DynamicPaintModifier

Dynamic Paint modifier

- `canvas_settings` · pointer · -> DynamicPaintCanvasSettings · read-only
- `brush_settings` · pointer · -> DynamicPaintBrushSettings · read-only
- `ui_type` · enum · {CANVAS, BRUSH}

## EchoModifier

Tooltip

- `delay` · float — The delay of the effect in seconds
- `feedback` · float — The feedback of the effect
- `mix` · float — The wet/dry mix of the effect

## EdgeSplitModifier

Edge splitting modifier to create sharp edges

- `split_angle` · float — Angle above which to split edges
- `use_edge_angle` · boolean — Split edges with high angle between faces
- `use_edge_sharp` · boolean — Split edges that are marked as sharp

## ExplodeModifier

Explosion effect modifier based on a particle system

- `vertex_group` · string
- `protect` · float — Clean vertex group edges
- `use_edge_cut` · boolean — Cut face edges for nicer shrapnel
- `show_unborn` · boolean — Show mesh when particles are unborn
- `show_alive` · boolean — Show mesh when particles are alive
- `show_dead` · boolean — Show mesh when particles are dead
- `use_size` · boolean — Use particle size for the shrapnel
- `particle_uv` · string — UV map to change with particle age
- `invert_vertex_group` · boolean — Invert vertex group influence

## FCurveModifiers

Collection of F-Curve Modifiers

- `active` · pointer · -> FModifier — Active F-Curve Modifier

## FModifier

Modifier for values of F-Curve

- `name` · string — F-Curve Modifier name
- `type` · enum · {NULL, GENERATOR, FNGENERATOR, ENVELOPE, CYCLES, NOISE, LIMITS, STEPPED, SMOOTH} · read-only — F-Curve Modifier Type
- `show_expanded` · boolean — F-Curve Modifier's panel is expanded in UI
- `mute` · boolean — Enable F-Curve modifier evaluation
- `is_valid` · boolean · read-only — F-Curve Modifier has invalid settings and will not be evaluated
- `active` · boolean — F-Curve modifier will show settings in the editor
- `use_restricted_range` · boolean — F-Curve Modifier is only applied for the specified frame range to help mask off effects in order to chain them
- `frame_start` · float — Frame that modifier's influence starts (if Restrict Frame Range is in use)
- `frame_end` · float — Frame that modifier's influence ends (if Restrict Frame Range is in use)
- `blend_in` · float — Number of frames from start frame for influence to take effect
- `blend_out` · float — Number of frames from end frame for influence to fade out
- `use_influence` · boolean — F-Curve Modifier's effects will be tempered by a default factor
- `influence` · float — Amount of influence F-Curve Modifier will have when not fading in/out

## FModifierCycles

Repeat the values of the modified F-Curve

- `mode_before` · enum · {NONE, REPEAT, REPEAT_OFFSET, MIRROR} — Cycling mode to use before first keyframe
- `cycles_before` · int — Maximum number of cycles to allow before first keyframe (0 = infinite)
- `mode_after` · enum · {NONE, REPEAT, REPEAT_OFFSET, MIRROR} — Cycling mode to use after last keyframe
- `cycles_after` · int — Maximum number of cycles to allow after last keyframe (0 = infinite)

## FModifierEnvelope

Scale the values of the modified F-Curve

- `control_points` · collection · [] of FModifierEnvelopeControlPoints · read-only — Control points defining the shape of the envelope
- `reference_value` · float — Value that envelope's influence is centered around / based on
- `default_min` · float — Lower distance from Reference Value for 1:1 default influence
- `default_max` · float — Upper distance from Reference Value for 1:1 default influence

## FModifierEnvelopeControlPoint

Control point for envelope F-Modifier

- `min` · float — Lower bound of envelope at this control-point
- `max` · float — Upper bound of envelope at this control-point
- `frame` · float — Frame this control-point occurs on

## FModifierFunctionGenerator

Generate values using a built-in function

- `amplitude` · float — Scale factor determining the maximum/minimum values
- `phase_multiplier` · float — Scale factor determining the 'speed' of the function
- `phase_offset` · float — Constant factor to offset time by for function
- `value_offset` · float — Constant factor to offset values by
- `use_additive` · boolean — Values generated by this modifier are applied on top of the existing values instead of overwriting them
- `function_type` · enum · {SIN, COS, TAN, SQRT, LN, SINC} — Type of built-in function to use

## FModifierGenerator

Deterministically generate values for the modified F-Curve

- `use_additive` · boolean — Values generated by this modifier are applied on top of the existing values instead of overwriting them
- `mode` · enum · {POLYNOMIAL, POLYNOMIAL_FACTORISED} — Type of generator to use
- `poly_order` · int — The highest power of 'x' for this polynomial (number of coefficients - 1)
- `coefficients` · float · [32] — Coefficients for 'x' (starting from lowest power of x^0)

## FModifierLimits

Limit the time/value ranges of the modified F-Curve

- `use_min_x` · boolean — Use the minimum X value
- `use_min_y` · boolean — Use the minimum Y value
- `use_max_x` · boolean — Use the maximum X value
- `use_max_y` · boolean — Use the maximum Y value
- `min_x` · float — Lowest X value to allow
- `min_y` · float — Lowest Y value to allow
- `max_x` · float — Highest X value to allow
- `max_y` · float — Highest Y value to allow

## FModifierNoise

Give randomness to the modified F-Curve

- `blend_type` · enum · {REPLACE, ADD, SUBTRACT, MULTIPLY} — Method of modifying the existing F-Curve
- `scale` · float — Scaling (in time) of the noise
- `strength` · float — Amplitude of the noise - the amount that it modifies the underlying curve
- `phase` · float — A random seed for the noise effect
- `offset` · float — Time offset for the noise effect
- `lacunarity` · float — Gap between successive frequencies. Depth needs to be greater than 0 for this to have an effect
- `roughness` · float — Amount of high frequency detail. Depth needs to be greater than 0 for this to have an effect
- `depth` · int — Amount of fine level detail present in the noise
- `use_legacy_noise` · boolean — Use the legacy way of generating noise. Has the issue that it can produce values outside of -1/1

## FModifierSmooth

Smooth curve using Gaussian smoothing

- `sigma` · float — The shape of the Gaussian distribution in frames. Lower values will increase sharpness across the Filter Width.
- `filter_width` · int — The number of frames to average around each keyframe. Higher values allow more smoothing, but will decrease performance.

## FModifierStepped

Hold each interpolated value from the F-Curve for several frames without changing the timing

- `frame_step` · float — Number of frames to hold each value
- `frame_offset` · float — Reference number of frames before frames get held (use to get hold for '1-3' vs '5-7' holding patterns)
- `use_frame_start` · boolean — Restrict modifier to only act after its 'start' frame
- `use_frame_end` · boolean — Restrict modifier to only act before its 'end' frame

## FluidModifier

Fluid simulation modifier

- `domain_settings` · pointer · -> FluidDomainSettings · read-only
- `flow_settings` · pointer · -> FluidFlowSettings · read-only
- `effector_settings` · pointer · -> FluidEffectorSettings · read-only
- `fluid_type` · enum · {NONE, DOMAIN, FLOW, EFFECTOR}

## GreasePencilArmatureModifier

Deform stroke points using armature object

- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `open_influence_panel` · boolean
- `object` · pointer · -> Object — Armature object to deform with
- `use_bone_envelopes` · boolean — Bind Bone envelopes to armature modifier
- `use_vertex_groups` · boolean — Bind vertex groups to armature modifier
- `use_deform_preserve_volume` · boolean — Deform rotation interpolation with quaternions

## GreasePencilArrayModifier

Create grid of duplicate instances

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `open_constant_offset_panel` · boolean
- `open_relative_offset_panel` · boolean
- `open_object_offset_panel` · boolean
- `open_randomize_panel` · boolean
- `open_influence_panel` · boolean
- `count` · int — Number of items
- `offset_object` · pointer · -> Object — Use the location and rotation of another object to determine the distance and rotational change between arrayed items
- `constant_offset` · float · [3] — Value for the distance between items
- `relative_offset` · float · [3] — The size of the geometry will determine the distance between arrayed items
- `random_offset` · float · [3] — Value for changes in location
- `random_rotation` · float · [3] — Value for changes in rotation
- `random_scale` · float · [3] — Value for changes in scale
- `seed` · int — Random seed
- `replace_material` · int — Index of the material used for generated strokes (0 keep original material)
- `use_constant_offset` · boolean — Enable offset
- `use_object_offset` · boolean — Add another object's transformation to the total offset
- `use_relative_offset` · boolean — Add an offset relative to the object's bounding box
- `use_uniform_random_scale` · boolean — Use the same random seed for each scale axis for a uniform scale

## GreasePencilBuildModifier

Animate strokes appearing and disappearing

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `open_influence_panel` · boolean
- `open_frame_range_panel` · boolean
- `open_fading_panel` · boolean
- `mode` · enum · {SEQUENTIAL, CONCURRENT, ADDITIVE} — How strokes are being built
- `transition` · enum · {GROW, SHRINK, FADE} — How are strokes animated (i.e. are they appearing or disappearing)
- `start_delay` · float — Number of frames after each GP keyframe before the modifier has any effect
- `length` · float — Maximum number of frames that the build effect can run for (unless another GP keyframe occurs before this time has elapsed)
- `concurrent_time_alignment` · enum · {START, END} — How should strokes start to appear/disappear
- `time_mode` · enum · {DRAWSPEED, FRAMES, PERCENTAGE} — Use drawing speed, a number of frames, or a manual factor to build strokes
- `speed_factor` · float — Multiply recorded drawing speed by a factor
- `speed_maxgap` · float — The maximum gap between strokes in seconds
- `use_restrict_frame_range` · boolean — Only modify strokes during the specified frame range
- `use_percentage` · boolean — Use a percentage factor to determine the visible points
- `percentage_factor` · float — Defines how much of the stroke is visible
- `frame_start` · float — Start Frame (when Restrict Frame Range is enabled)
- `frame_end` · float — End Frame (when Restrict Frame Range is enabled)
- `use_fading` · boolean — Fade out strokes instead of directly cutting off
- `fade_factor` · float — Defines how much of the stroke is fading in/out
- `target_vertex_group` · string — Output Vertex group
- `fade_opacity_strength` · float — How much strength fading applies on top of stroke opacity
- `fade_thickness_strength` · float — How much strength fading applies on top of stroke thickness
- `object` · pointer · -> Object — Object used as build starting position

## GreasePencilColorModifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `use_custom_curve` · boolean — Use a custom curve to define a factor along the strokes
- `custom_curve` · pointer · -> CurveMapping · read-only — Custom curve to apply effect
- `open_influence_panel` · boolean
- `color_mode` · enum · {BOTH, STROKE, FILL} — Attributes to modify
- `hue` · float — Color hue offset
- `saturation` · float — Color saturation factor
- `value` · float — Color value factor

## GreasePencilDashModifierData

Create dot-dash effect for strokes

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `open_influence_panel` · boolean
- `segments` · collection · read-only
- `segment_active_index` · int — Active index in the segment list
- `dash_offset` · int — Offset into each stroke before the beginning of the dashed segment generation

## GreasePencilDashModifierSegment

Configuration for a single dash segment

- `name` · string — Name of the dash segment
- `dash` · int — The number of consecutive points from the original stroke to include in this segment
- `gap` · int — The number of points skipped after this segment
- `radius` · float — The factor to apply to the original point's radius for the new points
- `opacity` · float — The factor to apply to the original point's opacity for the new points
- `material_index` · int — Use this index on generated segment. -1 means using the existing material.
- `use_cyclic` · boolean — Enable cyclic on individual stroke dashes

## GreasePencilEnvelopeModifier

Envelope stroke effect modifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `open_influence_panel` · boolean
- `mode` · enum · {DEFORM, SEGMENTS, FILLS} — Algorithm to use for generating the envelope
- `spread` · int — The number of points to skip to create straight segments
- `mat_nr` · int — The material to use for the new strokes
- `thickness` · float — Multiplier for the thickness of the new strokes
- `strength` · float — Multiplier for the strength of the new strokes
- `skip` · int — The number of generated segments to skip to reduce complexity

## GreasePencilHookModifier

Hook modifier to modify the location of stroke points

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `use_custom_curve` · boolean — Use a custom curve to define a factor along the strokes
- `custom_curve` · pointer · -> CurveMapping · read-only — Custom curve to apply effect
- `open_influence_panel` · boolean
- `open_falloff_panel` · boolean
- `object` · pointer · -> Object — Parent Object for hook, also recalculates and clears offset
- `subtarget` · string — Name of Parent Bone for hook (if applicable), also recalculates and clears offset
- `strength` · float — Relative force of the hook
- `falloff_type` · enum · {NONE, CURVE, SMOOTH, SPHERE, ROOT, INVERSE_SQUARE, SHARP, LINEAR, CONSTANT}
- `falloff_radius` · float — If not zero, the distance from the hook where influence ends
- `center` · float · [3]
- `matrix_inverse` · float · [16] — Reverse the transformation between this object and its target
- `use_falloff_uniform` · boolean — Compensate for non-uniform object scale

## GreasePencilLatticeModifier

Deform strokes using a lattice object

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `open_influence_panel` · boolean
- `object` · pointer · -> Object — Lattice object to deform with
- `strength` · float — Strength of modifier effect

## GreasePencilLengthModifier

Stretch or shrink strokes

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `open_random_panel` · boolean
- `open_curvature_panel` · boolean
- `open_influence_panel` · boolean
- `start_factor` · float — Added length to the start of each stroke relative to its length
- `end_factor` · float — Added length to the end of each stroke relative to its length
- `start_length` · float — Absolute added length to the start of each stroke
- `end_length` · float — Absolute added length to the end of each stroke
- `random_start_factor` · float — Size of random length added to the start of each stroke
- `random_end_factor` · float — Size of random length added to the end of each stroke
- `random_offset` · float — Smoothly offset each stroke's random value
- `use_random` · boolean — Use random values over time
- `seed` · int — Random seed
- `step` · int — Number of frames between randomization steps
- `overshoot_factor` · float — Defines what portion of the stroke is used for the calculation of the extension
- `mode` · enum · {RELATIVE, ABSOLUTE} — Mode to define length
- `use_curvature` · boolean — Follow the curvature of the stroke
- `invert_curvature` · boolean — Invert the curvature of the stroke's extension
- `point_density` · float — Multiplied by Start/End for the total added point count
- `segment_influence` · float — Factor to determine how much the length of the individual segments should influence the final computed curvature. Higher factors makes small segments influence the overall curvature less.
- `max_angle` · float — Ignore points on the stroke that deviate from their neighbors by more than this angle when determining the extrapolation shape

## GreasePencilLineartModifier

Generate Line Art strokes from selected source

- `use_custom_camera` · boolean — Use custom camera instead of the active camera
- `use_fuzzy_intersections` · boolean — Treat intersection and contour lines as if they were the same type so they can be chained together
- `use_fuzzy_all` · boolean — Treat all lines as the same line type so they can be chained together
- `use_object_instances` · boolean — Allow particle objects and face/vertex instances to show in Line Art
- `use_edge_overlap` · boolean — Allow edges in the same location (i.e. from edge split) to show properly. May run slower.
- `use_clip_plane_boundaries` · boolean — Allow lines generated by the near/far clipping plane to be shown
- `crease_threshold` · float — Angles smaller than this will be treated as creases. Crease angle priority: object Line Art crease override > mesh auto smooth angle > Line Art default crease.
- `split_angle` · float — Angle in screen space below which a stroke is split in two
- `smooth_tolerance` · float — Strength of smoothing applied on jagged chains
- `use_loose_as_contour` · boolean — Loose edges will have contour type
- `invert_source_vertex_group` · boolean — Invert source vertex group values
- `use_output_vertex_group_match_by_name` · boolean — Match output vertex group based on name
- `use_face_mark` · boolean — Filter feature lines using Freestyle face marks
- `use_face_mark_invert` · boolean — Invert face mark filtering
- `use_face_mark_boundaries` · boolean — Filter feature lines based on face mark boundaries
- `use_face_mark_keep_contour` · boolean — Preserve contour lines while filtering
- `chaining_image_threshold` · float — Segments with an image distance smaller than this will be chained together
- `use_loose_edge_chain` · boolean — Allow loose edges to be chained together
- `use_geometry_space_chain` · boolean — Use geometry distance for chaining instead of image space
- `use_detail_preserve` · boolean — Keep the zig-zag "noise" in initial chaining
- `use_overlap_edge_type_support` · boolean — Allow an edge to have multiple overlapping types. This will create a separate stroke for each overlapping type.
- `stroke_depth_offset` · float — Move strokes slightly towards the camera to avoid clipping while preserve depth for the viewport
- `use_offset_towards_custom_camera` · boolean — Offset strokes towards selected camera instead of the active camera
- `source_camera` · pointer · -> Object — Use specified camera object for generating Line Art strokes
- `light_contour_object` · pointer · -> Object — Use this light object to generate light contour
- `source_type` · enum · {COLLECTION, OBJECT, SCENE} — Line Art stroke source type
- `source_object` · pointer · -> Object — Generate strokes from this object
- `source_collection` · pointer · -> Collection — Generate strokes from the objects in this collection
- `use_contour` · boolean — Generate strokes from contours lines
- `use_loose` · boolean — Generate strokes from loose edges
- `use_crease` · boolean — Generate strokes from creased edges
- `use_material` · boolean — Generate strokes from borders between materials
- `use_edge_mark` · boolean — Generate strokes from Freestyle marked edges
- `use_intersection` · boolean — Generate strokes from intersections
- `use_light_contour` · boolean — Generate light/shadow separation lines from a reference light object
- `use_shadow` · boolean — Project contour lines using a light source object
- `shadow_region_filtering` · enum · {NONE, ILLUMINATED, SHADED, ILLUMINATED_ENCLOSED} — Select feature lines that comes from lit or shaded regions. Will not affect cast shadow and light contour since they are at the border.
- `silhouette_filtering` · enum · {NONE, GROUP, INDIVIDUAL} — Select contour or silhouette
- `fill_strokes` · boolean — Generate filled strokes instead of only outline
- `use_multiple_levels` · boolean — Generate strokes from a range of occlusion levels
- `level_start` · int — Minimum number of occlusions for the generated strokes
- `level_end` · int — Maximum number of occlusions for the generated strokes
- `target_layer` · string — Grease Pencil layer to which assign the generated strokes
- `target_material` · pointer · -> Material — Grease Pencil material assigned to the generated strokes
- `source_vertex_group` · string — Match the beginning of vertex group names from mesh objects, match all when left empty
- `vertex_group` · string — Vertex group name for selected strokes
- `is_baked` · boolean — This modifier has baked data
- `use_cache` · boolean — Use cached scene data from the first Line Art modifier in the stack. Certain settings will be unavailable.
- `overscan` · float — A margin to prevent strokes from ending abruptly at the edge of the image
- `radius` · float — The radius for the generated strokes
- `opacity` · float — The strength value for the generate strokes
- `use_material_mask` · boolean — Use material masks to filter out occluded strokes
- `use_material_mask_match` · boolean — Require matching all material masks instead of just one
- `use_material_mask_bits` · boolean — Mask bits to match from Material Line Art settings
- `use_intersection_match` · boolean — Require matching all intersection masks instead of just one
- `use_intersection_mask` · boolean — Mask bits to match from Collection Line Art settings
- `use_crease_on_smooth` · boolean — Allow crease edges to show inside smooth surfaces
- `use_crease_on_sharp` · boolean — Allow crease to show on sharp edges
- `use_image_boundary_trimming` · boolean — Trim all edges right at the boundary of image (including overscan region)
- `use_back_face_culling` · boolean — Remove all back faces to speed up calculation, this will create edges in different occlusion levels than when disabled
- `shadow_camera_near` · float — Near clipping distance of shadow camera
- `shadow_camera_far` · float — Far clipping distance of shadow camera
- `shadow_camera_size` · float — Represents the "Orthographic Scale" of an orthographic camera. If the camera is positioned at the light's location with this scale, it will represent the coverage of the shadow "camera".
- `use_invert_collection` · boolean — Select everything except lines from specified collection
- `use_invert_silhouette` · boolean — Select anti-silhouette lines

## GreasePencilMirrorModifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `open_influence_panel` · boolean
- `object` · pointer · -> Object — Object used as center
- `use_axis_x` · boolean — Mirror the X axis
- `use_axis_y` · boolean — Mirror the Y axis
- `use_axis_z` · boolean — Mirror the Z axis

## GreasePencilMultiplyModifier

Generate multiple strokes from one stroke

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `open_fading_panel` · boolean
- `open_influence_panel` · boolean
- `use_fade` · boolean — Fade the stroke thickness for each generated stroke
- `duplicates` · int — How many copies of strokes be displayed
- `distance` · float — Distance of duplications
- `offset` · float — Offset of duplicates, -1 to 1 (inner to outer)
- `fading_thickness` · float — Fade influence of stroke's thickness
- `fading_opacity` · float — Fade influence of stroke's opacity
- `fading_center` · float — Fade center

## GreasePencilNoiseModifier

Noise effect modifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `use_custom_curve` · boolean — Use a custom curve to define a factor along the strokes
- `custom_curve` · pointer · -> CurveMapping · read-only — Custom curve to apply effect
- `open_influence_panel` · boolean
- `open_random_panel` · boolean
- `factor` · float — Amount of noise to apply
- `factor_strength` · float — Amount of noise to apply to opacity
- `factor_thickness` · float — Amount of noise to apply to thickness
- `factor_uvs` · float — Amount of noise to apply to UV rotation
- `use_random` · boolean — Use random values over time
- `seed` · int — Random seed
- `noise_scale` · float — Scale the noise frequency
- `noise_offset` · float — Offset the noise along the strokes
- `step` · int — Number of frames between randomization steps
- `random_mode` · enum · {STEP, KEYFRAME} — Where to perform randomization

## GreasePencilOffsetModifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `open_general_panel` · boolean
- `open_influence_panel` · boolean
- `offset_mode` · enum · {RANDOM, LAYER, STROKE, MATERIAL}
- `location` · float · [3] — Values for change location
- `rotation` · float · [3] — Values for changes in rotation
- `scale` · float · [3] — Values for changes in scale
- `stroke_location` · float · [3] — Value for changes in location
- `stroke_rotation` · float · [3] — Value for changes in rotation
- `stroke_scale` · float · [3] — Value for changes in scale
- `seed` · int — Random seed
- `stroke_step` · int — Number of elements that will be grouped
- `stroke_start_offset` · int — Offset starting point
- `use_uniform_random_scale` · boolean — Use the same random seed for each scale axis for a uniform scale

## GreasePencilOpacityModifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `use_custom_curve` · boolean — Use a custom curve to define a factor along the strokes
- `custom_curve` · pointer · -> CurveMapping · read-only — Custom curve to apply effect
- `open_influence_panel` · boolean
- `color_mode` · enum · {BOTH, STROKE, FILL, HARDNESS} — Attributes to modify
- `color_factor` · float — Factor of opacity
- `hardness_factor` · float — Factor of stroke hardness
- `use_weight_as_factor` · boolean — Use vertex group weight as factor instead of influence
- `use_uniform_opacity` · boolean — Replace the stroke opacity instead of modulating each point

## GreasePencilOutlineModifier

Outline of Strokes modifier from camera view

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `open_influence_panel` · boolean
- `thickness` · int — Thickness of the perimeter stroke
- `sample_length` · float
- `subdivision` · int — Number of subdivisions
- `use_keep_shape` · boolean — Try to keep global shape
- `outline_material` · pointer · -> Material — Material used for outline strokes
- `object` · pointer · -> Object — Target object to define stroke start

## GreasePencilShrinkwrapModifier

Shrink wrapping modifier to shrink wrap an object to a target

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `open_influence_panel` · boolean
- `wrap_method` · enum · {NEAREST_SURFACEPOINT, PROJECT, NEAREST_VERTEX, TARGET_PROJECT}
- `wrap_mode` · enum · {ON_SURFACE, INSIDE, OUTSIDE, OUTSIDE_SURFACE, ABOVE_SURFACE} — Select how vertices are constrained to the target surface
- `cull_face` · enum · {OFF, FRONT, BACK} — Stop vertices from projecting to a face on the target when facing towards/away
- `target` · pointer · -> Object — Mesh target to shrink to
- `auxiliary_target` · pointer · -> Object — Additional mesh target to shrink to
- `offset` · float — Distance to keep from the target
- `project_limit` · float — Limit the distance used for projection (zero disables)
- `use_project_x` · boolean
- `use_project_y` · boolean
- `use_project_z` · boolean
- `subsurf_levels` · int — Number of subdivisions that must be performed before extracting vertices' positions and normals
- `use_negative_direction` · boolean — Allow vertices to move in the negative direction of axis
- `use_positive_direction` · boolean — Allow vertices to move in the positive direction of axis
- `use_invert_cull` · boolean — When projecting in the negative direction invert the face cull mode
- `smooth_factor` · float — Amount of smoothing to apply
- `smooth_step` · int — Number of times to apply smooth (high numbers can reduce FPS)

## GreasePencilSimplifyModifier

Simplify Stroke modifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `open_influence_panel` · boolean
- `factor` · float — Factor of Simplify
- `mode` · enum · {FIXED, ADAPTIVE, SAMPLE, MERGE} — How to simplify the stroke
- `step` · int — Number of times to apply simplify
- `length` · float — Length of each segment
- `sharp_threshold` · float — Preserve corners that have sharper angle than this threshold
- `distance` · float — Distance between points

## GreasePencilSmoothModifier

Smooth effect modifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `use_custom_curve` · boolean — Use a custom curve to define a factor along the strokes
- `custom_curve` · pointer · -> CurveMapping · read-only — Custom curve to apply effect
- `open_influence_panel` · boolean
- `factor` · float — Amount of smooth to apply
- `use_edit_position` · boolean — The modifier affects the position of the point
- `use_edit_strength` · boolean — The modifier affects the color strength of the point
- `use_edit_thickness` · boolean — The modifier affects the thickness of the point
- `use_edit_uv` · boolean — The modifier affects the UV rotation factor of the point
- `step` · int — Number of times to apply smooth (high numbers can reduce fps)
- `use_keep_shape` · boolean — Smooth the details, but keep the overall shape
- `use_smooth_ends` · boolean — Smooth ends of strokes

## GreasePencilSubdivModifier

Subdivide Stroke modifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `open_influence_panel` · boolean
- `level` · int — Level of subdivision
- `subdivision_type` · enum · {CATMULL_CLARK, SIMPLE} — Select type of subdivision algorithm

## GreasePencilTextureModifier

Transform stroke texture coordinates Modifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `open_influence_panel` · boolean
- `uv_offset` · float — Offset value to add to stroke UVs
- `uv_scale` · float — Factor to scale the UVs
- `alignment_rotation` · float — Additional rotation applied to dots and square strokes
- `fill_rotation` · float — Additional rotation of the fill UV
- `fill_offset` · float · [2] — Additional offset of the fill UV
- `fill_scale` · float — Additional scale of the fill UV
- `fit_method` · enum · {CONSTANT_LENGTH, FIT_STROKE}
- `mode` · enum · {STROKE, FILL, STROKE_AND_FILL}

## GreasePencilThickModifierData

Adjust stroke thickness

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `use_custom_curve` · boolean — Use a custom curve to define a factor along the strokes
- `custom_curve` · pointer · -> CurveMapping · read-only — Custom curve to apply effect
- `open_influence_panel` · boolean
- `thickness` · float — Absolute thickness to apply everywhere
- `thickness_factor` · float — Factor to multiply the thickness with
- `use_weight_factor` · boolean — Use weight to modulate effect
- `use_uniform_thickness` · boolean — Replace the stroke thickness

## GreasePencilTimeModifier

Offset keyframes

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `open_influence_panel` · boolean
- `open_custom_range_panel` · boolean
- `segments` · collection · read-only
- `segment_active_index` · int — Active index in the segment list
- `mode` · enum · {NORMAL, REVERSE, FIX, PINGPONG, CHAIN}
- `offset` · int — Number of frames to offset original keyframe number or frame to fix
- `frame_scale` · float — Evaluation time in seconds
- `frame_start` · int — First frame of the range
- `frame_end` · int — Final frame of the range
- `use_keep_loop` · boolean — Retiming end frames and move to start of animation to keep loop
- `use_custom_frame_range` · boolean — Define a custom range of frames to use in modifier

## GreasePencilTimeModifierSegment

Configuration for a single dash segment

- `name` · string — Name of the dash segment
- `segment_start` · int — First frame of the segment
- `segment_end` · int — Last frame of the segment
- `segment_repeat` · int — Number of cycle repeats
- `segment_mode` · enum · {NORMAL, REVERSE, PINGPONG}

## GreasePencilTintModifier

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `use_custom_curve` · boolean — Use a custom curve to define a factor along the strokes
- `custom_curve` · pointer · -> CurveMapping · read-only — Custom curve to apply effect
- `open_influence_panel` · boolean
- `color_mode` · enum · {BOTH, STROKE, FILL} — Attributes to modify
- `factor` · float — Factor for tinting
- `tint_mode` · enum · {UNIFORM, GRADIENT}
- `color` · float · [3] — Color used for tinting
- `color_ramp` · pointer · -> ColorRamp · read-only — Gradient tinting colors
- `object` · pointer · -> Object — Object used for the gradient direction
- `radius` · float — Influence distance from the object
- `use_weight_as_factor` · boolean — Use vertex group weight as factor instead of influence

## GreasePencilWeightAngleModifier

Calculate Vertex Weight dynamically

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `open_influence_panel` · boolean
- `target_vertex_group` · string — Output Vertex group
- `use_multiply` · boolean — Multiply the calculated weights with the existing values in the vertex group
- `use_invert_output` · boolean — Invert output weight values
- `angle` · float — Angle
- `axis` · enum · {X, Y, Z}
- `space` · enum · {LOCAL, WORLD} — Coordinates space
- `minimum_weight` · float — Minimum value for vertex weight

## GreasePencilWeightProximityModifier

Calculate Vertex Weight dynamically

- `tree_node_filter` · string — Layer name
- `use_layer_pass_filter` · boolean — Use layer pass filter
- `layer_pass_filter` · int — Layer pass filter
- `invert_layer_filter` · boolean — Invert layer filter
- `invert_layer_pass_filter` · boolean — Invert layer pass filter
- `use_layer_group_filter` · boolean — Filter by layer group name
- `material_filter` · pointer · -> Material — Material used for filtering
- `use_material_pass_filter` · boolean — Use material pass filter
- `material_pass_filter` · int — Material pass
- `invert_material_filter` · boolean — Invert material filter
- `invert_material_pass_filter` · boolean — Invert material pass filter
- `vertex_group_name` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group weights
- `open_influence_panel` · boolean
- `use_multiply` · boolean — Multiply the calculated weights with the existing values in the vertex group
- `use_invert_output` · boolean — Invert output weight values
- `target_vertex_group` · string — Output Vertex group
- `object` · pointer · -> Object — Object used as distance reference
- `distance_start` · float — Distance mapping to 0.0 weight
- `minimum_weight` · float — Minimum value for vertex weight
- `distance_end` · float — Distance mapping to 1.0 weight

## HookModifier

Hook modifier to modify the location of vertices

- `strength` · float — Relative force of the hook
- `falloff_type` · enum · {NONE, CURVE, SMOOTH, SPHERE, ROOT, INVERSE_SQUARE, SHARP, LINEAR, CONSTANT}
- `falloff_radius` · float — If not zero, the distance from the hook where influence ends
- `falloff_curve` · pointer · -> CurveMapping · read-only — Custom falloff curve
- `center` · float · [3] — Center of the hook, used for falloff and display
- `matrix_inverse` · float · [16] — Reverse the transformation between this object and its target
- `object` · pointer · -> Object — Parent Object for hook, also recalculates and clears offset
- `subtarget` · string — Name of Parent Bone for hook (if applicable), also recalculates and clears offset
- `use_falloff_uniform` · boolean — Compensate for non-uniform object scale
- `vertex_group` · string — Name of Vertex Group which determines influence of modifier per point
- `vertex_indices` · int · [64] · read-only — Indices of vertices bound to the modifier. For Bézier curves, handles count as additional vertices.
- `invert_vertex_group` · boolean — Invert vertex group influence

## HueCorrectModifier

Hue correction modifier for sequence strip

- `curve_mapping` · pointer · -> CurveMapping · read-only
- `open_mask_input_panel` · boolean

## LaplacianDeformModifier

Mesh deform modifier

- `vertex_group` · string — Name of Vertex Group which determines Anchors
- `iterations` · int
- `is_bind` · boolean · read-only — Whether geometry has been bound to anchors
- `invert_vertex_group` · boolean — Invert vertex group influence

## LaplacianSmoothModifier

Smoothing effect modifier

- `use_x` · boolean — Smooth object along X axis
- `use_y` · boolean — Smooth object along Y axis
- `use_z` · boolean — Smooth object along Z axis
- `use_volume_preserve` · boolean — Apply volume preservation after smooth
- `use_normalized` · boolean — Improve and stabilize the enhanced shape
- `lambda_factor` · float — Smooth effect factor
- `lambda_border` · float — Lambda factor in border
- `iterations` · int
- `vertex_group` · string — Name of Vertex Group which determines influence of modifier per point
- `invert_vertex_group` · boolean — Invert vertex group influence

## LatticeModifier

Lattice deformation modifier

- `object` · pointer · -> Object — Lattice object to deform with
- `vertex_group` · string — Name of Vertex Group which determines influence of modifier per point
- `invert_vertex_group` · boolean — Invert vertex group influence
- `strength` · float — Strength of modifier effect

## LineStyleAlphaModifier

Base type to define alpha transparency modifiers

- `name` · string — Name of the modifier

## LineStyleAlphaModifier_AlongStroke

Change alpha transparency along stroke

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping

## LineStyleAlphaModifier_CreaseAngle

Alpha transparency based on the angle between two adjacent faces

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `angle_min` · float — Minimum angle to modify thickness
- `angle_max` · float — Maximum angle to modify thickness

## LineStyleAlphaModifier_Curvature_3D

Alpha transparency based on the radial curvature of 3D mesh surfaces

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `curvature_min` · float — Minimum Curvature
- `curvature_max` · float — Maximum Curvature

## LineStyleAlphaModifier_DistanceFromCamera

Change alpha transparency based on the distance from the camera

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `range_min` · float — Lower bound of the input range the mapping is applied
- `range_max` · float — Upper bound of the input range the mapping is applied

## LineStyleAlphaModifier_DistanceFromObject

Change alpha transparency based on the distance from an object

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `range_min` · float — Lower bound of the input range the mapping is applied
- `range_max` · float — Upper bound of the input range the mapping is applied
- `target` · pointer · -> Object — Target object from which the distance is measured

## LineStyleAlphaModifier_Material

Change alpha transparency based on a material attribute

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `material_attribute` · enum · {LINE, LINE_R, LINE_G, LINE_B, LINE_A, DIFF, DIFF_R, DIFF_G, DIFF_B, SPEC, SPEC_R, SPEC_G, SPEC_B, SPEC_HARD, ALPHA} — Specify which material attribute is used
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping

## LineStyleAlphaModifier_Noise

Alpha transparency based on random noise

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `amplitude` · float — Amplitude of the noise
- `period` · float — Period of the noise
- `seed` · int — Seed for the noise generation

## LineStyleAlphaModifier_Tangent

Alpha transparency based on the direction of the stroke

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping

## LineStyleColorModifier

Base type to define line color modifiers

- `name` · string — Name of the modifier

## LineStyleColorModifier_AlongStroke

Change line color along stroke

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `color_ramp` · pointer · -> ColorRamp · read-only — Color ramp used to change line color

## LineStyleColorModifier_CreaseAngle

Change line color based on the underlying crease angle

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `color_ramp` · pointer · -> ColorRamp · read-only — Color ramp used to change line color
- `angle_min` · float — Minimum angle to modify thickness
- `angle_max` · float — Maximum angle to modify thickness

## LineStyleColorModifier_Curvature_3D

Change line color based on the radial curvature of 3D mesh surfaces

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `color_ramp` · pointer · -> ColorRamp · read-only — Color ramp used to change line color
- `curvature_min` · float — Minimum Curvature
- `curvature_max` · float — Maximum Curvature

## LineStyleColorModifier_DistanceFromCamera

Change line color based on the distance from the camera

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `color_ramp` · pointer · -> ColorRamp · read-only — Color ramp used to change line color
- `range_min` · float — Lower bound of the input range the mapping is applied
- `range_max` · float — Upper bound of the input range the mapping is applied

## LineStyleColorModifier_DistanceFromObject

Change line color based on the distance from an object

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `color_ramp` · pointer · -> ColorRamp · read-only — Color ramp used to change line color
- `range_min` · float — Lower bound of the input range the mapping is applied
- `range_max` · float — Upper bound of the input range the mapping is applied
- `target` · pointer · -> Object — Target object from which the distance is measured

## LineStyleColorModifier_Material

Change line color based on a material attribute

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `material_attribute` · enum · {LINE, LINE_R, LINE_G, LINE_B, LINE_A, DIFF, DIFF_R, DIFF_G, DIFF_B, SPEC, SPEC_R, SPEC_G, SPEC_B, SPEC_HARD, ALPHA} — Specify which material attribute is used
- `color_ramp` · pointer · -> ColorRamp · read-only — Color ramp used to change line color
- `use_ramp` · boolean — Use color ramp to map the BW average into an RGB color

## LineStyleColorModifier_Noise

Change line color based on random noise

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `color_ramp` · pointer · -> ColorRamp · read-only — Color ramp used to change line color
- `amplitude` · float — Amplitude of the noise
- `period` · float — Period of the noise
- `seed` · int — Seed for the noise generation

## LineStyleColorModifier_Tangent

Change line color based on the direction of a stroke

- `type` · enum · {ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `color_ramp` · pointer · -> ColorRamp · read-only — Color ramp used to change line color

## LineStyleGeometryModifier

Base type to define stroke geometry modifiers

- `name` · string — Name of the modifier

## LineStyleGeometryModifier_2DOffset

Add two-dimensional offsets to stroke backbone geometry

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `start` · float — Displacement that is applied from the beginning of the stroke
- `end` · float — Displacement that is applied from the end of the stroke
- `x` · float — Displacement that is applied to the X coordinates of stroke vertices
- `y` · float — Displacement that is applied to the Y coordinates of stroke vertices

## LineStyleGeometryModifier_2DTransform

Apply two-dimensional scaling and rotation to stroke backbone geometry

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `pivot` · enum · {CENTER, START, END, PARAM, ABSOLUTE} — Pivot of scaling and rotation operations
- `scale_x` · float — Scaling factor that is applied along the X axis
- `scale_y` · float — Scaling factor that is applied along the Y axis
- `angle` · float — Rotation angle
- `pivot_u` · float — Pivot in terms of the stroke point parameter u (0 <= u <= 1)
- `pivot_x` · float — 2D X coordinate of the absolute pivot
- `pivot_y` · float — 2D Y coordinate of the absolute pivot

## LineStyleGeometryModifier_BackboneStretcher

Stretch the beginning and the end of stroke backbone

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `backbone_length` · float — Amount of backbone stretching

## LineStyleGeometryModifier_BezierCurve

Replace stroke backbone geometry by a Bézier curve approximation of the original backbone geometry

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `error` · float — Maximum distance allowed between the new Bézier curve and the original backbone geometry

## LineStyleGeometryModifier_Blueprint

Produce a blueprint using circular, elliptic, and square contour strokes

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `shape` · enum · {CIRCLES, ELLIPSES, SQUARES} — Select the shape of blueprint contour strokes
- `rounds` · int — Number of rounds in contour strokes
- `backbone_length` · float — Amount of backbone stretching
- `random_radius` · int — Randomness of the radius
- `random_center` · int — Randomness of the center
- `random_backbone` · int — Randomness of the backbone stretching

## LineStyleGeometryModifier_GuidingLines

Modify the stroke geometry so that it corresponds to its main direction line

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `offset` · float — Displacement that is applied to the main direction line along its normal

## LineStyleGeometryModifier_PerlinNoise1D

Add one-dimensional Perlin noise to stroke backbone geometry

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `frequency` · float — Frequency of the Perlin noise
- `amplitude` · float — Amplitude of the Perlin noise
- `octaves` · int — Number of octaves (i.e., the amount of detail of the Perlin noise)
- `angle` · float — Displacement direction
- `seed` · int — Seed for random number generation (if negative, time is used as a seed instead)

## LineStyleGeometryModifier_PerlinNoise2D

Add two-dimensional Perlin noise to stroke backbone geometry

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `frequency` · float — Frequency of the Perlin noise
- `amplitude` · float — Amplitude of the Perlin noise
- `octaves` · int — Number of octaves (i.e., the amount of detail of the Perlin noise)
- `angle` · float — Displacement direction
- `seed` · int — Seed for random number generation (if negative, time is used as a seed instead)

## LineStyleGeometryModifier_Polygonalization

Modify the stroke geometry so that it looks more 'polygonal'

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `error` · float — Maximum distance between the original stroke and its polygonal approximation

## LineStyleGeometryModifier_Sampling

Specify a new sampling value that determines the resolution of stroke polylines

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `sampling` · float — New sampling value to be used for subsequent modifiers

## LineStyleGeometryModifier_Simplification

Simplify the stroke set

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `tolerance` · float — Distance below which segments will be merged

## LineStyleGeometryModifier_SinusDisplacement

Add sinus displacement to stroke backbone geometry

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `wavelength` · float — Wavelength of the sinus displacement
- `amplitude` · float — Amplitude of the sinus displacement
- `phase` · float — Phase of the sinus displacement

## LineStyleGeometryModifier_SpatialNoise

Add spatial noise to stroke backbone geometry

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `amplitude` · float — Amplitude of the spatial noise
- `scale` · float — Scale of the spatial noise
- `octaves` · int — Number of octaves (i.e., the amount of detail of the spatial noise)
- `smooth` · boolean — If true, the spatial noise is smooth
- `use_pure_random` · boolean — If true, the spatial noise does not show any coherence

## LineStyleGeometryModifier_TipRemover

Remove a piece of stroke at the beginning and the end of stroke backbone

- `type` · enum · {2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER} · read-only — Type of the modifier
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `tip_length` · float — Length of tips to be removed

## LineStyleThicknessModifier

Base type to define line thickness modifiers

- `name` · string — Name of the modifier

## LineStyleThicknessModifier_AlongStroke

Change line thickness along stroke

- `type` · enum · {ALONG_STROKE, CALLIGRAPHY, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `value_min` · float — Minimum output value of the mapping
- `value_max` · float — Maximum output value of the mapping

## LineStyleThicknessModifier_Calligraphy

Change line thickness so that stroke looks like made with a calligraphic pen

- `type` · enum · {ALONG_STROKE, CALLIGRAPHY, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `orientation` · float — Angle of the main direction
- `thickness_min` · float — Minimum thickness in the direction perpendicular to the main direction
- `thickness_max` · float — Maximum thickness in the main direction

## LineStyleThicknessModifier_CreaseAngle

Line thickness based on the angle between two adjacent faces

- `type` · enum · {ALONG_STROKE, CALLIGRAPHY, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `angle_min` · float — Minimum angle to modify thickness
- `angle_max` · float — Maximum angle to modify thickness
- `thickness_min` · float — Minimum thickness
- `thickness_max` · float — Maximum thickness

## LineStyleThicknessModifier_Curvature_3D

Line thickness based on the radial curvature of 3D mesh surfaces

- `type` · enum · {ALONG_STROKE, CALLIGRAPHY, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `thickness_min` · float — Minimum thickness
- `thickness_max` · float — Maximum thickness
- `curvature_min` · float — Minimum Curvature
- `curvature_max` · float — Maximum Curvature

## LineStyleThicknessModifier_DistanceFromCamera

Change line thickness based on the distance from the camera

- `type` · enum · {ALONG_STROKE, CALLIGRAPHY, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `range_min` · float — Lower bound of the input range the mapping is applied
- `range_max` · float — Upper bound of the input range the mapping is applied
- `value_min` · float — Minimum output value of the mapping
- `value_max` · float — Maximum output value of the mapping

## LineStyleThicknessModifier_DistanceFromObject

Change line thickness based on the distance from an object

- `type` · enum · {ALONG_STROKE, CALLIGRAPHY, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `range_min` · float — Lower bound of the input range the mapping is applied
- `range_max` · float — Upper bound of the input range the mapping is applied
- `value_min` · float — Minimum output value of the mapping
- `value_max` · float — Maximum output value of the mapping
- `target` · pointer · -> Object — Target object from which the distance is measured

## LineStyleThicknessModifier_Material

Change line thickness based on a material attribute

- `type` · enum · {ALONG_STROKE, CALLIGRAPHY, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `material_attribute` · enum · {LINE, LINE_R, LINE_G, LINE_B, LINE_A, DIFF, DIFF_R, DIFF_G, DIFF_B, SPEC, SPEC_R, SPEC_G, SPEC_B, SPEC_HARD, ALPHA} — Specify which material attribute is used
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `value_min` · float — Minimum output value of the mapping
- `value_max` · float — Maximum output value of the mapping

## LineStyleThicknessModifier_Noise

Line thickness based on random noise

- `type` · enum · {ALONG_STROKE, CALLIGRAPHY, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `amplitude` · float — Amplitude of the noise
- `period` · float — Period of the noise
- `seed` · int — Seed for the noise generation
- `use_asymmetric` · boolean — Allow thickness to be assigned asymmetrically

## LineStyleThicknessModifier_Tangent

Thickness based on the direction of the stroke

- `type` · enum · {ALONG_STROKE, CALLIGRAPHY, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT} · read-only — Type of the modifier
- `blend` · enum · {MIX, ADD, SUBTRACT, MULTIPLY, DIVIDE, DIFFERENCE, MINIMUM, MAXIMUM} — Specify how the modifier value is blended into the base value
- `influence` · float — Influence factor by which the modifier changes the property
- `use` · boolean — Enable or disable this modifier during stroke rendering
- `expanded` · boolean — True if the modifier tab is expanded
- `mapping` · enum · {LINEAR, CURVE} — Select the mapping type
- `invert` · boolean — Invert the fade-out direction of the linear mapping
- `curve` · pointer · -> CurveMapping · read-only — Curve used for the curve mapping
- `thickness_min` · float — Minimum thickness
- `thickness_max` · float — Maximum thickness

## MaskModifier

Mask modifier to hide parts of the mesh

- `mode` · enum · {VERTEX_GROUP, ARMATURE}
- `armature` · pointer · -> Object — Armature to use as source of bones to mask
- `vertex_group` · string — Vertex group name
- `invert_vertex_group` · boolean — Use vertices that are not part of region defined
- `use_smooth` · boolean — Use vertex group weights to cut faces at the weight contour
- `threshold` · float — Weights over this threshold remain

## MeshCacheModifier

Cache Mesh

- `cache_format` · enum · {MDD, PC2}
- `interpolation` · enum · {NONE, LINEAR}
- `time_mode` · enum · {FRAME, TIME, FACTOR} — Method to control playback time
- `play_mode` · enum · {SCENE, CUSTOM}
- `deform_mode` · enum · {OVERWRITE, INTEGRATE}
- `filepath` · string — Path to external displacements file
- `factor` · float — Influence of the deformation
- `vertex_group` · string — Name of the Vertex Group which determines the influence of the modifier per point
- `invert_vertex_group` · boolean — Invert vertex group influence
- `forward_axis` · enum · {POS_X, POS_Y, POS_Z, NEG_X, NEG_Y, NEG_Z}
- `up_axis` · enum · {POS_X, POS_Y, POS_Z, NEG_X, NEG_Y, NEG_Z}
- `flip_axis` · boolean
- `frame_start` · float — Add this to the start frame
- `frame_scale` · float — Evaluation time in seconds
- `eval_frame` · float — The frame to evaluate (starting at 0)
- `eval_time` · float — Evaluation time in seconds
- `eval_factor` · float — Evaluation time in seconds

## MeshDeformModifier

Mesh deformation modifier to deform with other meshes

- `object` · pointer · -> Object — Mesh object to deform with
- `is_bound` · boolean · read-only — Whether geometry has been bound to control cage
- `invert_vertex_group` · boolean — Invert vertex group influence
- `vertex_group` · string — Vertex group name
- `precision` · int — The grid size for binding
- `use_dynamic_bind` · boolean — Recompute binding dynamically on top of other deformers (slower and more memory consuming)

## MeshSequenceCacheModifier

Cache Mesh

- `cache_file` · pointer · -> CacheFile
- `object_path` · string — Path to the object in the Alembic archive used to lookup geometric data
- `read_data` · enum · {VERT, POLY, UV, COLOR, ATTRIBUTES} — Data to read from the cache
- `use_vertex_interpolation` · boolean — Allow interpolation of vertex positions
- `velocity_scale` · float — Multiplier used to control the magnitude of the velocity vectors for time effects

## MeshToVolumeModifier

- `object` · pointer · -> Object — Object
- `resolution_mode` · enum · {VOXEL_AMOUNT, VOXEL_SIZE} — Mode for how the desired voxel size is specified
- `voxel_size` · float — Smaller values result in a higher resolution output
- `voxel_amount` · int — Approximate number of voxels along one axis
- `interior_band_width` · float — Width of the gradient inside of the mesh
- `density` · float — Density of the new volume

## MirrorModifier

Mirroring modifier

- `use_axis` · boolean — Enable axis mirror
- `use_bisect_axis` · boolean — Cuts the mesh across the mirror plane
- `use_bisect_flip_axis` · boolean — Flips the direction of the slice
- `use_clip` · boolean — Prevent vertices from going through the mirror during transform
- `use_mirror_vertex_groups` · boolean — Mirror vertex groups (e.g. .R->.L)
- `use_mirror_merge` · boolean — Merge vertices within the merge threshold
- `use_mirror_u` · boolean — Mirror the U texture coordinate around the flip offset point
- `use_mirror_v` · boolean — Mirror the V texture coordinate around the flip offset point
- `use_mirror_udim` · boolean — Mirror the texture coordinate around each tile center
- `mirror_offset_u` · float — Amount to offset mirrored UVs flipping point from the 0.5 on the U axis
- `mirror_offset_v` · float — Amount to offset mirrored UVs flipping point from the 0.5 point on the V axis
- `offset_u` · float — Mirrored UV offset on the U axis
- `offset_v` · float — Mirrored UV offset on the V axis
- `merge_threshold` · float — Distance within which mirrored vertices are merged
- `bisect_threshold` · float — Distance from the bisect plane within which vertices are removed
- `mirror_object` · pointer · -> Object — Object to use as mirror

## Modifier

Modifier affecting the geometry data of an object

- `name` · string — Modifier name
- `type` · enum · {GREASE_PENCIL_VERTEX_WEIGHT_PROXIMITY, DATA_TRANSFER, MESH_CACHE, MESH_SEQUENCE_CACHE, NORMAL_EDIT, WEIGHTED_NORMAL, UV_PROJECT, UV_WARP, VERTEX_WEIGHT_EDIT, VERTEX_WEIGHT_MIX, VERTEX_WEIGHT_PROXIMITY, GREASE_PENCIL_COLOR, GREASE_PENCIL_TINT, GREASE_PENCIL_OPACITY, GREASE_PENCIL_VERTEX_WEIGHT_ANGLE, GREASE_PENCIL_TIME, GREASE_PENCIL_TEXTURE, ARRAY, BEVEL, BOOLEAN, BUILD, DECIMATE, EDGE_SPLIT, NODES, MASK, MIRROR, MESH_TO_VOLUME, MULTIRES, REMESH, SCREW, SKIN, SOLIDIFY, SUBSURF, TRIANGULATE, VOLUME_TO_MESH, WELD, WIREFRAME, GREASE_PENCIL_ARRAY, GREASE_PENCIL_BUILD, GREASE_PENCIL_LENGTH, ... (83 total)} · read-only
- `show_viewport` · boolean — Display modifier in viewport
- `show_render` · boolean — Use modifier during render
- `show_in_editmode` · boolean — Display modifier in Edit mode
- `show_on_cage` · boolean — Adjust edit cage to modifier result
- `show_expanded` · boolean — Set modifier expanded in the user interface
- `is_active` · boolean — The active modifier in the list
- `use_pin_to_last` · boolean — Keep the modifier at the end of the list
- `is_override_data` · boolean · read-only — In a local override object, whether this modifier comes from the linked reference object, or is local to the override
- `use_apply_on_spline` · boolean — Apply this and all preceding deformation modifiers on splines' points rather than on filled curve/surface
- `execution_time` · float · read-only — Time in seconds that the modifier took to evaluate. This is only set on evaluated objects. If multiple modifiers run in parallel, execution time is not a reliable metric.
- `persistent_uid` · int · read-only — Uniquely identifies the modifier within the modifier stack that it is part of

## ModifierViewerPathElem

- `modifier_uid` · int — The persistent UID of the modifier

## MultiresModifier

Multiresolution mesh modifier

- `uv_smooth` · enum · {NONE, PRESERVE_CORNERS, PRESERVE_CORNERS_AND_JUNCTIONS, PRESERVE_CORNERS_JUNCTIONS_AND_CONCAVE, PRESERVE_BOUNDARIES, SMOOTH_ALL} — Controls how smoothing is applied to UVs
- `quality` · int — Accuracy of vertex positions, lower value is faster but less precise
- `boundary_smooth` · enum · {PRESERVE_CORNERS, ALL} — Controls how open boundaries are smoothed
- `levels` · int — Number of subdivisions to use in the viewport
- `sculpt_levels` · int — Number of subdivisions to use in sculpt mode
- `render_levels` · int — The subdivision level visible at render time
- `total_levels` · int · read-only — Number of subdivisions for which displacements are stored
- `is_external` · boolean · read-only — Store multires displacements outside the .blend file, to save memory
- `filepath` · string — Path to external displacements file
- `show_only_control_edges` · boolean — Skip drawing/rendering of interior subdivided edges
- `use_creases` · boolean — Use mesh crease information to sharpen edges or corners
- `use_custom_normals` · boolean — Interpolates existing custom normals to resulting mesh
- `use_sculpt_base_mesh` · boolean — Make Sculpt Mode tools deform the base mesh while previewing the displacement of higher subdivision levels

## NormalEditModifier

Modifier affecting/generating custom normals

- `mode` · enum · {RADIAL, DIRECTIONAL} — How to affect (generate) normals
- `offset` · float · [3] — Offset from object's center
- `mix_mode` · enum · {COPY, ADD, SUB, MUL} — How to mix generated normals with existing ones
- `mix_factor` · float — How much of generated normals to mix with existing ones
- `mix_limit` · float — Maximum angle between old and new normals
- `no_polynors_fix` · boolean — Do not flip polygons when their normals are not consistent with their newly computed custom vertex normals
- `vertex_group` · string — Vertex group name for selecting/weighting the affected areas
- `invert_vertex_group` · boolean — Invert vertex group influence
- `target` · pointer · -> Object — Target object used to affect normals
- `use_direction_parallel` · boolean — Use same direction for all normals, from origin to target's center (Directional mode only)

## ObjectModifiers

Collection of object modifiers

- `active` · pointer · -> Modifier — The active modifier in the list

## OceanModifier

Simulate an ocean surface

- `geometry_mode` · enum · {GENERATE, DISPLACE} — Method of modifying geometry
- `size` · float — Surface scale factor (does not affect the height of the waves)
- `repeat_x` · int — Repetitions of the generated surface in X
- `repeat_y` · int — Repetitions of the generated surface in Y
- `use_normals` · boolean — Output normals for bump mapping - disabling can speed up performance if it's not needed
- `use_foam` · boolean — Generate foam mask as a vertex color channel
- `use_spray` · boolean — Generate map of spray direction as a vertex color channel
- `invert_spray` · boolean — Invert the spray direction map
- `spray_layer_name` · string — Name of the vertex color layer used for the spray direction map
- `resolution` · int — Resolution of the generated surface for rendering and baking
- `viewport_resolution` · int — Viewport resolution of the generated surface
- `spatial_size` · int — Size of the simulation domain (in meters), and of the generated geometry (in BU)
- `wind_velocity` · float — Wind speed
- `damping` · float — Damp reflected waves going in opposite direction to the wind
- `wave_scale_min` · float — Shortest allowed wavelength
- `wave_alignment` · float — How much the waves are aligned to each other
- `wave_direction` · float — Main direction of the waves when they are (partially) aligned
- `wave_scale` · float — Scale of the displacement effect
- `depth` · float — Depth of the solid ground below the water surface
- `foam_coverage` · float — Amount of generated foam
- `bake_foam_fade` · float — How much foam accumulates over time (baked ocean only)
- `foam_layer_name` · string — Name of the vertex color layer used for foam
- `choppiness` · float — Choppiness of the wave's crest (adds some horizontal component to the displacement)
- `time` · float — Current time of the simulation
- `spectrum` · enum · {PHILLIPS, PIERSON_MOSKOWITZ, JONSWAP, TEXEL_MARSEN_ARSLOE} — Spectrum to use
- `fetch_jonswap` · float — This is the distance from a lee shore, called the fetch, or the distance over which the wind blows with constant velocity. Used by 'JONSWAP' and 'TMA' models.
- `sharpen_peak_jonswap` · float — Peak sharpening for 'JONSWAP' and 'TMA' models
- `random_seed` · int — Seed of the random generator
- `frame_start` · int — Start frame of the ocean baking
- `frame_end` · int — End frame of the ocean baking
- `is_cached` · boolean · read-only — Whether the ocean is using cached data or simulating
- `filepath` · string — Path to a folder to store external baked images

## ParticleInstanceModifier

Particle system instancing modifier

- `object` · pointer · -> Object — Object that has the particle system
- `particle_system_index` · int
- `particle_system` · pointer · -> ParticleSystem
- `axis` · enum · {X, Y, Z} — Pole axis for rotation
- `space` · enum · {LOCAL, WORLD} — Space to use for copying mesh data
- `use_normal` · boolean — Create instances from normal particles
- `use_children` · boolean — Create instances from child particles
- `use_path` · boolean — Create instances along particle paths
- `show_unborn` · boolean — Show instances when particles are unborn
- `show_alive` · boolean — Show instances when particles are alive
- `show_dead` · boolean — Show instances when particles are dead
- `use_preserve_shape` · boolean — Don't stretch the object
- `use_size` · boolean — Use particle size to scale the instances
- `position` · float — Position along path
- `random_position` · float — Randomize position along path
- `rotation` · float — Rotation around path
- `random_rotation` · float — Randomize rotation around path
- `particle_amount` · float — Amount of particles to use for instancing
- `particle_offset` · float — Relative offset of particles to use for instancing, to avoid overlap of multiple instances
- `index_layer_name` · string — Custom data layer name for the index
- `value_layer_name` · string — Custom data layer name for the randomized value

## ParticleSystemModifier

Particle system simulation modifier

- `particle_system` · pointer · -> ParticleSystem · read-only — Particle System that this modifier controls

## PitchModifier

Shift Audio Pitch

- `mode` · enum · {SEMITONES, RATIO} — Mode of the pitch shift
- `semitones` · int — Number of semitones to shift the pitch.
- `cents` · int — A cent is one one-hundredth of a semi-tone.
- `ratio` · float — Factor by which the audio pitch is scaled.
- `preserve_formant` · boolean — Whether to preserve the vocal formants when shifting the pitch.
- `quality` · enum · {HIGH, FAST, CONSISTENT} — Quality of the pitch shifting

## RemeshModifier

Generate a new surface with regular topology that follows the shape of the input mesh

- `mode` · enum · {BLOCKS, SMOOTH, SHARP, VOXEL}
- `scale` · float — The ratio of the largest dimension of the model over the size of the grid
- `threshold` · float — If removing disconnected pieces, minimum size of components to preserve as a ratio of the number of polygons in the largest component
- `octree_depth` · int — Resolution of the octree; higher values give finer details
- `sharpness` · float — Tolerance for outliers; lower values filter noise while higher values will reproduce edges closer to the input
- `voxel_size` · float — Size of the voxel in object space used for volume evaluation. Lower values preserve finer details.
- `adaptivity` · float — Reduces the final face count by simplifying geometry where detail is not needed, generating triangles. A value greater than 0 disables Fix Poles.
- `use_remove_disconnected` · boolean
- `use_smooth_shade` · boolean — Output faces with smooth shading rather than flat shaded

## ScrewModifier

Revolve edges

- `object` · pointer · -> Object — Object to define the screw axis
- `steps` · int — Number of steps in the revolution
- `render_steps` · int — Number of steps in the revolution
- `iterations` · int — Number of times to apply the screw operation
- `axis` · enum · {X, Y, Z} — Screw axis
- `angle` · float — Angle of revolution
- `screw_offset` · float — Offset the revolution along its axis
- `merge_threshold` · float — Limit below which to merge vertices
- `use_normal_flip` · boolean — Flip normals of lathed faces
- `use_normal_calculate` · boolean — Calculate the order of edges (needed for meshes, but not curves)
- `use_object_screw_offset` · boolean — Use the distance between the objects to make a screw
- `use_merge_vertices` · boolean — Merge adjacent vertices (screw offset must be zero)
- `use_smooth_shade` · boolean — Output faces with smooth shading rather than flat shaded
- `use_stretch_u` · boolean — Stretch the U coordinates between 0 and 1 when UVs are present
- `use_stretch_v` · boolean — Stretch the V coordinates between 0 and 1 when UVs are present

## SequencerCompositorModifierData

Compositor Modifier

- `node_group` · pointer · -> NodeTree — Node group that controls what this modifier does
- `show_group_selector` · boolean
- `open_mask_input_panel` · boolean
- `properties` · pointer · -> SequencerCompositorModifierProperties · read-only

## SequencerTonemapModifierData

Tone mapping modifier

- `tonemap_type` · enum · {RD_PHOTORECEPTOR, RH_SIMPLE} — Tone mapping algorithm
- `key` · float — The value the average luminance is mapped to
- `offset` · float — Normally always 1, but can be used as an extra control to alter the brightness curve
- `gamma` · float — If not used, set to 1
- `intensity` · float — If less than zero, darkens image; otherwise, makes it brighter
- `contrast` · float — Set to 0 to use estimate from input image
- `adaptation` · float — If 0, global; if 1, based on pixel intensity
- `correction` · float — If 0, same for all channels; if 1, each independent
- `open_mask_input_panel` · boolean

## ShrinkwrapModifier

Shrink wrapping modifier to shrink wrap and object to a target

- `wrap_method` · enum · {NEAREST_SURFACEPOINT, PROJECT, NEAREST_VERTEX, TARGET_PROJECT}
- `wrap_mode` · enum · {ON_SURFACE, INSIDE, OUTSIDE, OUTSIDE_SURFACE, ABOVE_SURFACE} — Select how vertices are constrained to the target surface
- `cull_face` · enum · {OFF, FRONT, BACK} — Stop vertices from projecting to a face on the target when facing towards/away
- `target` · pointer · -> Object — Mesh target to shrink to
- `auxiliary_target` · pointer · -> Object — Additional mesh target to shrink to
- `vertex_group` · string — Vertex group name
- `offset` · float — Distance to keep from the target
- `project_limit` · float — Limit the distance used for projection (zero disables)
- `use_project_x` · boolean
- `use_project_y` · boolean
- `use_project_z` · boolean
- `subsurf_levels` · int — Number of subdivisions that must be performed before extracting vertices' positions and normals
- `use_negative_direction` · boolean — Allow vertices to move in the negative direction of axis
- `use_positive_direction` · boolean — Allow vertices to move in the positive direction of axis
- `use_invert_cull` · boolean — When projecting in the negative direction invert the face cull mode
- `invert_vertex_group` · boolean — Invert vertex group influence

## SimpleDeformModifier

Simple deformation modifier to apply effects such as twisting and bending

- `deform_method` · enum · {TWIST, BEND, TAPER, STRETCH}
- `vertex_group` · string — Vertex group name
- `deform_axis` · enum · {X, Y, Z} — Deform around local axis
- `origin` · pointer · -> Object — Offset the origin and orientation of the deformation
- `factor` · float — Amount to deform object
- `angle` · float — Angle of deformation
- `limits` · float · [2] — Lower/Upper limits for deform
- `lock_x` · boolean — Do not allow deformation along the X axis
- `lock_y` · boolean — Do not allow deformation along the Y axis
- `lock_z` · boolean — Do not allow deformation along the Z axis
- `invert_vertex_group` · boolean — Invert vertex group influence

## SkinModifier

Generate Skin

- `branch_smoothing` · float — Smooth complex geometry around branches
- `use_smooth_shade` · boolean — Output faces with smooth shading rather than flat shaded
- `use_x_symmetry` · boolean — Avoid making unsymmetrical quads across the X axis
- `use_y_symmetry` · boolean — Avoid making unsymmetrical quads across the Y axis
- `use_z_symmetry` · boolean — Avoid making unsymmetrical quads across the Z axis

## SmoothModifier

Smoothing effect modifier

- `use_x` · boolean — Smooth object along X axis
- `use_y` · boolean — Smooth object along Y axis
- `use_z` · boolean — Smooth object along Z axis
- `factor` · float — Strength of modifier effect
- `iterations` · int
- `vertex_group` · string — Name of Vertex Group which determines influence of modifier per point
- `invert_vertex_group` · boolean — Invert vertex group influence

## SoftBodyModifier

Soft body simulation modifier

- `settings` · pointer · -> SoftBodySettings · read-only
- `point_cache` · pointer · -> PointCache · read-only

## SolidifyModifier

Create a solid skin, compensating for sharp angles

- `solidify_mode` · enum · {EXTRUDE, NON_MANIFOLD} — Selects the used algorithm
- `thickness` · float — Thickness of the shell
- `thickness_clamp` · float — Offset clamp based on geometry scale
- `use_thickness_angle_clamp` · boolean — Clamp thickness based on angles
- `thickness_vertex_group` · float — Thickness factor to use for zero vertex group influence
- `offset` · float — Offset the thickness from the center
- `edge_crease_inner` · float — Assign a crease to inner edges
- `edge_crease_outer` · float — Assign a crease to outer edges
- `edge_crease_rim` · float — Assign a crease to the edges making up the rim
- `material_offset` · int — Offset material index of generated faces
- `material_offset_rim` · int — Offset material index of generated rim faces
- `vertex_group` · string — Vertex group name
- `shell_vertex_group` · string — Vertex group that the generated shell geometry will be weighted to
- `rim_vertex_group` · string — Vertex group that the generated rim geometry will be weighted to
- `use_rim` · boolean — Create edge loops between the inner and outer surfaces on face edges (slow, disable when not needed)
- `use_even_offset` · boolean — Maintain thickness by adjusting for sharp corners (slow, disable when not needed)
- `use_quality_normals` · boolean — Calculate normals which result in more even thickness (slow, disable when not needed)
- `invert_vertex_group` · boolean — Invert the vertex group influence
- `use_flat_faces` · boolean — Make faces use the minimal vertex weight assigned to their vertices (ensures new faces remain parallel to their original ones, slow, disable when not needed)
- `use_flip_normals` · boolean — Invert the face direction
- `use_rim_only` · boolean — Only add the rim to the original data
- `nonmanifold_thickness_mode` · enum · {FIXED, EVEN, CONSTRAINTS} — Selects the used thickness algorithm
- `nonmanifold_boundary_mode` · enum · {NONE, ROUND, FLAT} — Selects the boundary adjustment algorithm
- `nonmanifold_merge_threshold` · float — Distance within which degenerated geometry is merged
- `bevel_convex` · float — Edge bevel weight to be added to outside edges

## SoundEqualizerModifier

Equalize audio

- `graphics` · collection · read-only — Graphical definition equalization

## StripModifier

Modifier for sequence strip

- `name` · string
- `type` · enum · {BRIGHT_CONTRAST, COLOR_BALANCE, COMPOSITOR, CURVES, HUE_CORRECT, MASK, TONEMAP, WHITE_BALANCE, SOUND_EQUALIZER, PITCH, ECHO} · read-only
- `mute` · boolean — Mute this modifier
- `enable` · boolean — Use modifier during render
- `show_preview` · boolean — Display modifier in preview
- `show_expanded` · boolean — Mute expanded settings for the modifier
- `input_mask_type` · enum · {STRIP, ID} — Type of input data used for mask
- `mask_time` · enum · {RELATIVE, ABSOLUTE} — Time to use for the Mask animation
- `input_mask_strip` · pointer · -> Strip — Strip used as mask input for the modifier
- `input_mask_id` · pointer · -> Mask — Mask ID used as mask input for the modifier
- `is_active` · boolean — This modifier is active

## StripModifiers

Collection of strip modifiers

- `active` · pointer · -> StripModifier — The active strip modifier in the list

## SubsurfModifier

Subdivision surface modifier

- `uv_smooth` · enum · {NONE, PRESERVE_CORNERS, PRESERVE_CORNERS_AND_JUNCTIONS, PRESERVE_CORNERS_JUNCTIONS_AND_CONCAVE, PRESERVE_BOUNDARIES, SMOOTH_ALL} — Controls how smoothing is applied to UVs
- `quality` · int — Accuracy of vertex positions, lower value is faster but less precise
- `boundary_smooth` · enum · {PRESERVE_CORNERS, ALL} — Controls how open boundaries are smoothed
- `subdivision_type` · enum · {CATMULL_CLARK, SIMPLE} — Select type of subdivision algorithm
- `levels` · int — Number of subdivisions to perform in the 3D viewport
- `render_levels` · int — Number of subdivisions to perform when rendering
- `show_only_control_edges` · boolean — Skip displaying interior subdivided edges
- `use_creases` · boolean — Use mesh crease information to sharpen edges or corners
- `use_custom_normals` · boolean — Interpolates existing custom normals to resulting mesh
- `use_limit_surface` · boolean — Place vertices at the surface that would be produced with infinite levels of subdivision (smoothest possible shape)
- `use_adaptive_subdivision` · boolean — Adaptively subdivide mesh based on camera distance
- `adaptive_space` · enum · {PIXEL, OBJECT} — How to adaptively subdivide the mesh
- `adaptive_pixel_size` · float — Target polygon pixel size for adaptive subdivision
- `adaptive_object_edge_length` · float — Target object space edge length for adaptive subdivision
- `open_adaptive_subdivision_panel` · boolean
- `open_advanced_panel` · boolean

## SurfaceDeformModifier

- `target` · pointer · -> Object — Mesh object to deform with
- `falloff` · float — Controls how much nearby polygons influence deformation
- `is_bound` · boolean · read-only — Whether geometry has been bound to target mesh
- `vertex_group` · string — Vertex group name for selecting/weighting the affected areas
- `invert_vertex_group` · boolean — Invert vertex group influence
- `use_sparse_bind` · boolean — Only record binding data for vertices matching the vertex group at the time of bind
- `strength` · float — Strength of modifier deformations

## TriangulateModifier

Triangulate Mesh

- `quad_method` · enum · {BEAUTY, FIXED, FIXED_ALTERNATE, SHORTEST_DIAGONAL, LONGEST_DIAGONAL} — Method for splitting the quads into triangles
- `ngon_method` · enum · {BEAUTY, CLIP} — Method for splitting the n-gons into triangles
- `min_vertices` · int — Triangulate only polygons with vertex count greater than or equal to this number
- `keep_custom_normals` · boolean — Try to preserve custom normals.
Warning: Depending on chosen triangulation method, shading may not be fully preserved, "Fixed" method usually gives the best result here

## UVProjectModifier

UV projection modifier to set UVs from a projector

- `uv_layer` · string — UV map name
- `projector_count` · int — Number of projectors to use
- `projectors` · collection · read-only
- `aspect_x` · float — Horizontal aspect ratio (only used for camera projectors)
- `aspect_y` · float — Vertical aspect ratio (only used for camera projectors)
- `scale_x` · float — Horizontal scale (only used for camera projectors)
- `scale_y` · float — Vertical scale (only used for camera projectors)

## UVWarpModifier

Add target position to UV coordinates

- `axis_u` · enum · {X, Y, Z} — Pole axis for rotation
- `axis_v` · enum · {X, Y, Z} — Pole axis for rotation
- `center` · float · [2] — Center point for rotate/scale
- `offset` · float · [2] — 2D Offset for the warp
- `scale` · float · [2] — 2D Scale for the warp
- `rotation` · float — 2D Rotation for the warp
- `object_from` · pointer · -> Object — Object defining offset
- `bone_from` · string — Bone defining offset
- `object_to` · pointer · -> Object — Object defining offset
- `bone_to` · string — Bone defining offset
- `vertex_group` · string — Vertex group name
- `invert_vertex_group` · boolean — Invert vertex group influence
- `uv_layer` · string — UV map name

## VertexWeightEditModifier

Edit the weights of vertices in a group

- `vertex_group` · string — Vertex group name
- `falloff_type` · enum · {LINEAR, CURVE, SHARP, SMOOTH, ROOT, ICON_SPHERECURVE, RANDOM, STEP} — How weights are mapped to their new values
- `invert_falloff` · boolean — Invert the resulting falloff weight
- `normalize` · boolean — Normalize the resulting weights (otherwise they are only clamped within 0.0 to 1.0 range)
- `map_curve` · pointer · -> CurveMapping · read-only — Custom mapping curve
- `use_add` · boolean — Add vertices with weight over threshold to vgroup
- `use_remove` · boolean — Remove vertices with weight below threshold from vgroup
- `default_weight` · float — Default weight a vertex will have if it is not in the vgroup
- `add_threshold` · float — Lower (inclusive) bound for a vertex's weight to be added to the vgroup
- `remove_threshold` · float — Upper (inclusive) bound for a vertex's weight to be removed from the vgroup
- `mask_constant` · float — Global influence of current modifications on vgroup
- `mask_vertex_group` · string — Masking vertex group name
- `invert_mask_vertex_group` · boolean — Invert vertex group mask influence
- `mask_texture` · pointer · -> Texture — Masking texture
- `mask_tex_use_channel` · enum · {INT, RED, GREEN, BLUE, HUE, SAT, VAL, ALPHA} — Which texture channel to use for masking
- `mask_tex_mapping` · enum · {LOCAL, GLOBAL, OBJECT, UV} — Which texture coordinates to use for mapping
- `mask_tex_uv_layer` · string — UV map name
- `mask_tex_map_object` · pointer · -> Object — Which object to take texture coordinates from
- `mask_tex_map_bone` · string — Which bone to take texture coordinates from

## VertexWeightMixModifier

Mix the weights of two vertex groups

- `vertex_group_a` · string — First vertex group name
- `vertex_group_b` · string — Second vertex group name
- `invert_vertex_group_a` · boolean — Invert the influence of vertex group A
- `invert_vertex_group_b` · boolean — Invert the influence of vertex group B
- `default_weight_a` · float — Default weight a vertex will have if it is not in the first A vgroup
- `default_weight_b` · float — Default weight a vertex will have if it is not in the second B vgroup
- `mix_mode` · enum · {SET, ADD, SUB, MUL, DIV, DIF, AVG, MIN, MAX} — How weights from vgroup B affect weights of vgroup A
- `mix_set` · enum · {ALL, A, B, OR, AND} — Which vertices should be affected
- `normalize` · boolean — Normalize the resulting weights (otherwise they are only clamped within 0.0 to 1.0 range)
- `mask_constant` · float — Global influence of current modifications on vgroup
- `mask_vertex_group` · string — Masking vertex group name
- `invert_mask_vertex_group` · boolean — Invert vertex group mask influence
- `mask_texture` · pointer · -> Texture — Masking texture
- `mask_tex_use_channel` · enum · {INT, RED, GREEN, BLUE, HUE, SAT, VAL, ALPHA} — Which texture channel to use for masking
- `mask_tex_mapping` · enum · {LOCAL, GLOBAL, OBJECT, UV} — Which texture coordinates to use for mapping
- `mask_tex_uv_layer` · string — UV map name
- `mask_tex_map_object` · pointer · -> Object — Which object to take texture coordinates from
- `mask_tex_map_bone` · string — Which bone to take texture coordinates from

## VertexWeightProximityModifier

Set the weights of vertices in a group from a target object's distance

- `vertex_group` · string — Vertex group name
- `proximity_mode` · enum · {OBJECT, GEOMETRY} — Which distances to target object to use
- `proximity_geometry` · enum · {VERTEX, EDGE, FACE} — Use the shortest computed distance to target object's geometry as weight
- `target` · pointer · -> Object — Object to calculate vertices distances from
- `min_dist` · float — Distance mapping to weight 0.0
- `max_dist` · float — Distance mapping to weight 1.0
- `falloff_type` · enum · {LINEAR, CURVE, SHARP, SMOOTH, ROOT, ICON_SPHERECURVE, RANDOM, STEP} — How weights are mapped to their new values
- `invert_falloff` · boolean — Invert the resulting falloff weight
- `normalize` · boolean — Normalize the resulting weights (otherwise they are only clamped within 0.0 to 1.0 range)
- `map_curve` · pointer · -> CurveMapping · read-only — Custom mapping curve
- `mask_constant` · float — Global influence of current modifications on vgroup
- `mask_vertex_group` · string — Masking vertex group name
- `invert_mask_vertex_group` · boolean — Invert vertex group mask influence
- `mask_texture` · pointer · -> Texture — Masking texture
- `mask_tex_use_channel` · enum · {INT, RED, GREEN, BLUE, HUE, SAT, VAL, ALPHA} — Which texture channel to use for masking
- `mask_tex_mapping` · enum · {LOCAL, GLOBAL, OBJECT, UV} — Which texture coordinates to use for mapping
- `mask_tex_uv_layer` · string — UV map name
- `mask_tex_map_object` · pointer · -> Object — Which object to take texture coordinates from
- `mask_tex_map_bone` · string — Which bone to take texture coordinates from

## VolumeDisplaceModifier

- `strength` · float — Strength of the displacement
- `texture` · pointer · -> Texture
- `texture_map_mode` · enum · {LOCAL, GLOBAL, OBJECT}
- `texture_map_object` · pointer · -> Object — Object to use for texture mapping
- `texture_mid_level` · float · [3] — Subtracted from the texture color to get a displacement vector
- `texture_sample_radius` · float — Smaller values result in better performance but might cut off the volume

## VolumeToMeshModifier

- `object` · pointer · -> Object — Object
- `threshold` · float — Voxels with a larger value are inside the generated mesh
- `adaptivity` · float — Reduces the final face count by simplifying geometry where detail is not needed
- `use_smooth_shade` · boolean — Output faces with smooth shading rather than flat shaded
- `grid_name` · string — Grid in the volume object that is converted to a mesh
- `resolution_mode` · enum · {GRID, VOXEL_AMOUNT, VOXEL_SIZE} — Mode for how the desired voxel size is specified
- `voxel_size` · float — Smaller values result in a higher resolution output
- `voxel_amount` · int — Approximate number of voxels along one axis

## WarpModifier

Warp modifier

- `object_from` · pointer · -> Object — Object to transform from
- `bone_from` · string — Bone to transform from
- `object_to` · pointer · -> Object — Object to transform to
- `bone_to` · string — Bone defining offset
- `strength` · float
- `falloff_type` · enum · {NONE, CURVE, SMOOTH, SPHERE, ROOT, INVERSE_SQUARE, SHARP, LINEAR, CONSTANT}
- `falloff_radius` · float — Radius to apply
- `falloff_curve` · pointer · -> CurveMapping · read-only — Custom falloff curve
- `use_volume_preserve` · boolean — Preserve volume when rotations are used
- `vertex_group` · string — Vertex group name for modulating the deform
- `invert_vertex_group` · boolean — Invert vertex group influence
- `texture` · pointer · -> Texture
- `texture_coords` · enum · {LOCAL, GLOBAL, OBJECT, UV}
- `uv_layer` · string — UV map name
- `texture_coords_object` · pointer · -> Object — Object to set the texture coordinates
- `texture_coords_bone` · string — Bone to set the texture coordinates

## WaveModifier

Wave effect modifier

- `use_x` · boolean — X axis motion
- `use_y` · boolean — Y axis motion
- `use_cyclic` · boolean — Cyclic wave effect
- `use_normal` · boolean — Displace along normals
- `use_normal_x` · boolean — Enable displacement along the X normal
- `use_normal_y` · boolean — Enable displacement along the Y normal
- `use_normal_z` · boolean — Enable displacement along the Z normal
- `time_offset` · float — Either the starting frame (for positive speed) or ending frame (for negative speed)
- `lifetime` · float — Lifetime of the wave in frames, zero means infinite
- `damping_time` · float — Number of frames in which the wave damps out after it dies
- `falloff_radius` · float — Distance after which it fades out
- `start_position_x` · float — X coordinate of the start position
- `start_position_y` · float — Y coordinate of the start position
- `start_position_object` · pointer · -> Object — Object which defines the wave center
- `vertex_group` · string — Vertex group name for modulating the wave
- `invert_vertex_group` · boolean — Invert vertex group influence
- `speed` · float — Speed of the wave, towards the starting point when negative
- `height` · float — Height of the wave
- `width` · float — Distance between the waves
- `narrowness` · float — Distance between the top and the base of a wave, the higher the value, the more narrow the wave
- `texture` · pointer · -> Texture
- `texture_coords` · enum · {LOCAL, GLOBAL, OBJECT, UV}
- `uv_layer` · string — UV map name
- `texture_coords_object` · pointer · -> Object — Object to set the texture coordinates
- `texture_coords_bone` · string — Bone to set the texture coordinates

## WeightedNormalModifier

- `weight` · int — Corrective factor applied to faces' weights, 50 is neutral, lower values increase weight of weak faces, higher values increase weight of strong faces
- `mode` · enum · {FACE_AREA, CORNER_ANGLE, FACE_AREA_WITH_ANGLE} — Weighted vertex normal mode to use
- `thresh` · float — Threshold value for different weights to be considered equal
- `keep_sharp` · boolean — Keep sharp edges as computed for default custom normals, instead of setting a single weighted normal for each vertex
- `vertex_group` · string — Vertex group name for modifying the selected areas
- `invert_vertex_group` · boolean — Invert vertex group influence
- `use_face_influence` · boolean — Use influence of face for weighting

## WeldModifier

Weld modifier

- `mode` · enum · {ALL, CONNECTED} — Mode defines the merge rule
- `merge_threshold` · float — Limit below which to merge vertices
- `vertex_group` · string — Vertex group name for selecting the affected areas
- `invert_vertex_group` · boolean — Invert vertex group influence
- `loose_edges` · boolean — Collapse edges without faces, cloth sewing edges

## WhiteBalanceModifier

White balance modifier for sequence strip

- `white_value` · float · [3] — This color defines white in the strip
- `open_mask_input_panel` · boolean

## WireframeModifier

Wireframe effect modifier

- `thickness` · float — Thickness factor
- `thickness_vertex_group` · float — Thickness factor to use for zero vertex group influence
- `offset` · float — Offset the thickness from the center
- `use_replace` · boolean — Remove original geometry
- `use_boundary` · boolean — Support face boundaries
- `use_even_offset` · boolean — Scale the offset to give more even thickness
- `use_relative_offset` · boolean — Scale the offset by surrounding geometry
- `use_crease` · boolean — Crease hub edges for improved subdivision surface
- `crease_weight` · float — Crease weight (if active)
- `material_offset` · int — Offset material index of generated faces
- `vertex_group` · string — Vertex group name for selecting the affected areas
- `invert_vertex_group` · boolean — Invert vertex group influence
