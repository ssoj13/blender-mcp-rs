---
area: api-types
group: brush
version: 5.2.2
---

# Blender 5.2.2 RNA types: brush

## Brush

Brush data-block for storing brush settings for painting and sculpting

- `has_unsaved_changes` · boolean · read-only — Indicates that there are any user visible changes since the brush has been imported or read from the file
- `blend` · enum · {MIX, DARKEN, MUL, COLORBURN, LINEARBURN, LIGHTEN, SCREEN, COLORDODGE, ADD, OVERLAY, SOFTLIGHT, HARDLIGHT, VIVIDLIGHT, LINEARLIGHT, PINLIGHT, DIFFERENCE, EXCLUSION, SUB, HUE, SATURATION, COLOR, LUMINOSITY, ERASE_ALPHA, ADD_ALPHA} — Brush blending mode
- `sculpt_brush_type` · enum · {DRAW, DRAW_SHARP, CLAY, CLAY_STRIPS, CLAY_THUMB, LAYER, INFLATE, BLOB, CREASE, SMOOTH, PLANE, MULTIPLANE_SCRAPE, PINCH, GRAB, ELASTIC_DEFORM, SNAKE_HOOK, THUMB, POSE, NUDGE, ROTATE, TOPOLOGY, BOUNDARY, SCENE_PROJECT, CLOTH, SIMPLIFY, MASK, DRAW_FACE_SETS, DISPLACEMENT_ERASER, DISPLACEMENT_SMEAR, PAINT, SMEAR, BLUR}
- `vertex_brush_type` · enum · {DRAW, BLUR, AVERAGE, SMEAR}
- `weight_brush_type` · enum · {DRAW, BLUR, AVERAGE, SMEAR}
- `image_brush_type` · enum · {DRAW, SOFTEN, SMEAR, CLONE, FILL, MASK}
- `gpencil_brush_type` · enum · {DRAW, FILL, ERASE, TINT}
- `gpencil_vertex_brush_type` · enum · {DRAW, BLUR, AVERAGE, SMEAR, REPLACE}
- `gpencil_sculpt_brush_type` · enum · {SMOOTH, THICKNESS, STRENGTH, RANDOMIZE, GRAB, PUSH, TWIST, PINCH, CLONE}
- `gpencil_weight_brush_type` · enum · {WEIGHT, BLUR, AVERAGE, SMEAR}
- `curves_sculpt_brush_type` · enum · {SELECTION_PAINT, ADD, DELETE, DENSITY, COMB, SNAKE_HOOK, GROW_SHRINK, PINCH, PUFF, SMOOTH, SLIDE}
- `direction` · enum · {ADD, SUBTRACT}
- `stroke_method` · enum · {DOTS, DRAG_DOT, SPACE, AIRBRUSH, ANCHORED, LINE, CURVE}
- `sculpt_plane` · enum · {AREA, VIEW, X, Y, Z}
- `mask_tool` · enum · {DRAW, SMOOTH}
- `curve_distance_falloff_preset` · enum · {CUSTOM, SMOOTH, SMOOTHER, SPHERE, ROOT, SHARP, LIN, POW4, INVSQUARE, CONSTANT}
- `deform_target` · enum · {GEOMETRY, CLOTH_SIM} — How the deformation of the brush will affect the object
- `elastic_deform_type` · enum · {GRAB, GRAB_BISCALE, GRAB_TRISCALE, SCALE, TWIST} — Deformation type that is used in the brush
- `snake_hook_deform_type` · enum · {FALLOFF, ELASTIC} — Deformation type that is used in the brush
- `plane_inversion_mode` · enum · {INVERT_DISPLACEMENT, SWAP_DEPTH_AND_HEIGHT} — Inversion Mode
- `project_ray_direction_type` · enum · {VIEW_NORMAL, PLANE_NORMAL} — Ray Direction
- `minimum_distance` · float — Minimum distance to other scene objects after projecting onto them
- `cloth_deform_type` · enum · {DRAG, PUSH, PINCH_POINT, PINCH_PERPENDICULAR, INFLATE, GRAB, EXPAND, SNAKE_HOOK} — Deformation type that is used in the brush
- `cloth_force_falloff_type` · enum · {RADIAL, PLANE} — Shape used in the brush to apply force to the cloth
- `cloth_simulation_area_type` · enum · {LOCAL, GLOBAL, DYNAMIC} — Part of the mesh that is going to be simulated when the stroke is active
- `boundary_falloff_type` · enum · {CONSTANT, RADIUS, LOOP, LOOP_INVERT} — How the brush falloff is applied across the boundary
- `smooth_deform_type` · enum · {LAPLACIAN, SURFACE} — Deformation type that is used in the brush
- `smear_deform_type` · enum · {DRAG, PINCH, EXPAND} — Deformation type that is used in the brush
- `slide_deform_type` · enum · {DRAG, PINCH, EXPAND} — Deformation type that is used in the brush
- `boundary_deform_type` · enum · {BEND, EXPAND, INFLATE, GRAB, TWIST, SMOOTH} — Deformation type that is used in the brush
- `pose_deform_type` · enum · {ROTATE_TWIST, SCALE_TRANSLATE, SQUASH_STRETCH} — Deformation type that is used in the brush
- `pose_origin_type` · enum · {TOPOLOGY, FACE_SETS, FACE_SETS_FK} — Method to set the rotation origins for the segments of the brush
- `jitter_unit` · enum · {VIEW, BRUSH} — Jitter in screen space or relative to brush size
- `falloff_shape` · enum · {SPHERE, PROJECTED} — Use projected or spherical falloff
- `size` · int — Diameter of the brush in pixels
- `unprojected_size` · float — Diameter of brush in Blender units
- `input_samples` · int — Number of input samples to average together to smooth the brush stroke
- `jitter` · float — Jitter the position of the brush while painting
- `jitter_absolute` · int — Jitter the position of the brush in pixels while painting
- `spacing` · int — Spacing between brush daubs as a percentage of brush diameter
- `grad_spacing` · int — Spacing before brush gradient goes full circle
- `use_color_jitter` · boolean — Jitter brush color
- `hue_jitter` · float — Color jitter effect on hue
- `saturation_jitter` · float — Color jitter effect on saturation
- `value_jitter` · float — Color jitter effect on value
- `use_stroke_random_hue` · boolean — Use randomness at stroke level
- `use_stroke_random_sat` · boolean — Use randomness at stroke level
- `use_stroke_random_val` · boolean — Use randomness at stroke level
- `use_random_press_hue` · boolean — Use pressure to modulate randomness
- `use_random_press_sat` · boolean — Use pressure to modulate randomness
- `use_random_press_val` · boolean — Use pressure to modulate randomness
- `curve_random_hue` · pointer · -> CurveMapping · read-only — Curve used for modulating effect
- `curve_random_saturation` · pointer · -> CurveMapping · read-only — Curve used for modulating effect
- `curve_random_value` · pointer · -> CurveMapping · read-only — Curve used for modulating effect
- `curve_size` · pointer · -> CurveMapping · read-only — Curve used to map pressure to brush size
- `curve_strength` · pointer · -> CurveMapping · read-only — Curve used to map pressure to brush strength
- `curve_jitter` · pointer · -> CurveMapping · read-only — Curve used to map pressure to brush jitter
- `smooth_stroke_radius` · int — Minimum distance from last point before stroke continues
- `smooth_stroke_factor` · float — Higher values give a smoother stroke
- `rate` · float — Interval between paints for Airbrush
- `color` · float · [3]
- `secondary_color` · float · [3]
- `weight` · float — Vertex weight when brush is applied
- `strength` · float — How powerful the effect of the brush is when applied
- `flow` · float — Amount of paint that is applied per stroke sample
- `wet_mix` · float — Amount of paint that is picked from the surface into the brush color
- `wet_persistence` · float — Amount of wet paint that stays in the brush after applying paint to the surface
- `density` · float — Amount of random elements that are going to be affected by the brush
- `tip_scale_x` · float — Scale of the brush tip in the X axis
- `use_hardness_pressure` · boolean — Use pressure to modulate hardness
- `invert_hardness_pressure` · boolean — Invert the modulation of pressure in hardness
- `use_flow_pressure` · boolean — Use pressure to modulate flow
- `invert_flow_pressure` · boolean — Invert the modulation of pressure in flow
- `use_wet_mix_pressure` · boolean — Use pressure to modulate wet mix
- `invert_wet_mix_pressure` · boolean — Invert the modulation of pressure in wet mix
- `use_wet_persistence_pressure` · boolean — Use pressure to modulate wet persistence
- `invert_wet_persistence_pressure` · boolean — Invert the modulation of pressure in wet persistence
- `use_density_pressure` · boolean — Use pressure to modulate density
- `invert_density_pressure` · boolean — Invert the modulation of pressure in density
- `dash_ratio` · float — Ratio of samples in a cycle that the brush is enabled
- `dash_samples` · int — Length of a dash cycle measured in stroke samples
- `plane_offset` · float — Adjust plane on which the brush acts towards or away from the object surface
- `plane_trim` · float — If a vertex is further away from offset plane than this, then it is not affected
- `height` · float — Affectable height of brush (i.e. the layer height for the layer tool)
- `plane_depth` · float — The maximum distance below the plane for affected vertices. Increasing the depth affects vertices farther below the plane.
- `plane_height` · float — The maximum distance above the plane for affected vertices. Increasing the height affects vertices farther above the plane.
- `stabilize_normal` · float — How stable the plane normal is over the course of the stroke. A value of 0 corresponds to using the current normal, and a value of 1 corresponds to using the initial normal.
- `stabilize_plane` · float — How stable the plane center is over the course of the stroke. A value of 0 corresponds to using the current center, and a value of 1 corresponds to using the initial center.
- `texture_sample_bias` · float — Value added to texture samples
- `use_color_as_displacement` · boolean — Handle each pixel color as individual vector for displacement (area plane mapping only)
- `normal_weight` · float — How much grab will pull vertices out of surface during a grab
- `elastic_deform_volume_preservation` · float — Poisson ratio for elastic deformation. Higher values preserve volume more, but also lead to more bulging.
- `rake_factor` · float — How much grab will follow cursor rotation
- `crease_pinch_factor` · float — How much the crease brush pinches
- `pose_offset` · float — Offset of the pose origin in relation to the brush radius
- `disconnected_distance_max` · float — Maximum distance to search for disconnected loose parts in the mesh
- `boundary_offset` · float — Offset of the boundary origin in relation to the brush radius
- `surface_smooth_shape_preservation` · float — How much of the original shape is preserved when smoothing
- `surface_smooth_current_vertex` · float — How much the position of each individual vertex influences the final result
- `surface_smooth_iterations` · int — Number of smoothing iterations per brush step
- `multiplane_scrape_angle` · float — Angle between the planes of the crease
- `pose_smooth_iterations` · int — Smooth iterations applied after calculating the pose factor of each vertex
- `pose_ik_segments` · int — Number of segments of the inverse kinematics chain that will deform the mesh
- `tip_roundness` · float — Roundness of the brush tip
- `cloth_mass` · float — Mass of each simulation particle
- `cloth_damping` · float — How much the applied forces are propagated through the cloth
- `cloth_sim_limit` · float — Factor added relative to the size of the radius to limit the cloth simulation effects
- `cloth_sim_falloff` · float — Area to apply deformation falloff to the effects of the simulation
- `cloth_constraint_softbody_strength` · float — How much the cloth preserves the original shape, acting as a soft body
- `hardness` · float — How close the brush falloff starts from the edge of the brush
- `auto_smooth_factor` · float — Amount of smoothing to automatically apply to each stroke
- `topology_rake_factor` · float — Automatically align edges to the brush direction to generate cleaner topology and define sharp features. Best used on low-poly meshes as it has a performance impact.
- `tilt_strength_factor` · float — How much the tilt of the pen will affect the brush. Negative values indicate inverting the tilt directions.
- `normal_radius_factor` · float — Ratio between the brush radius and the radius that is going to be used to sample the normal
- `area_radius_factor` · float — Ratio between the brush radius and the radius that is going to be used to sample the area center
- `wet_paint_radius_factor` · float — Ratio between the brush radius and the radius that is going to be used to sample the color to blend in wet paint
- `stencil_pos` · float · [2] — Position of stencil in viewport
- `stencil_dimension` · float · [2] — Dimensions of stencil in viewport
- `mask_stencil_pos` · float · [2] — Position of mask stencil in viewport
- `mask_stencil_dimension` · float · [2] — Dimensions of mask stencil in viewport
- `sharp_threshold` · float — Threshold below which, no sharpening is done
- `fill_threshold` · float — Threshold above which filling is not propagated
- `blur_kernel_radius` · int — Radius of kernel used for soften and sharpen in pixels
- `blur_mode` · enum · {BOX, GAUSSIAN}
- `falloff_angle` · float — Paint most on faces pointing towards the view according to this angle
- `use_original_normal` · boolean — When locked keep using normal of surface where stroke was initiated
- `use_original_plane` · boolean — When locked keep using the plane origin of surface where stroke was initiated
- `use_scene_spacing` · enum · {VIEW, SCENE} — Calculate the brush spacing using view or scene distance
- `use_grab_active_vertex` · boolean — Apply the maximum grab strength to the active vertex instead of the cursor location
- `use_grab_silhouette` · boolean — Grabs trying to automask the silhouette of the object
- `use_paint_antialiasing` · boolean — Smooths the edges of the strokes
- `use_multiplane_scrape_dynamic` · boolean — The angle between the planes changes during the stroke to fit the surface under the cursor
- `show_multiplane_scrape_planes_preview` · boolean — Preview the scrape planes in the cursor during the stroke
- `use_pose_ik_anchored` · boolean — Keep the position of the last segment in the IK chain fixed
- `use_pose_lock_rotation` · boolean — Do not rotate the segment when using the scale deform mode
- `use_connected_only` · boolean — Affect only topologically connected elements
- `use_cloth_pin_simulation_boundary` · boolean — Lock the position of the vertices in the simulation falloff area to avoid artifacts and create a softer transition with unaffected areas
- `use_cloth_collision` · boolean — Collide with objects during the simulation
- `invert_to_scrape_fill` · boolean — Use Scrape or Fill brush when inverting this brush instead of inverting its displacement direction
- `use_pressure_strength` · boolean — Enable tablet pressure sensitivity for strength
- `use_offset_pressure` · boolean — Enable tablet pressure sensitivity for offset
- `use_pressure_area_radius` · boolean — Enable tablet pressure sensitivity for area radius
- `use_pressure_size` · boolean — Enable tablet pressure sensitivity for size
- `use_pressure_jitter` · boolean — Enable tablet pressure sensitivity for jitter
- `use_pressure_spacing` · boolean — Enable tablet pressure sensitivity for spacing
- `use_pressure_masking` · enum · {NONE, RAMP, CUTOFF} — Pen pressure makes texture influence smaller
- `use_inverse_smooth_pressure` · boolean — Lighter pressure causes more smoothing to be applied
- `use_plane_trim` · boolean — Limit the distance from the offset plane that a vertex can be affected
- `use_frontface` · boolean — Brush only affects vertices that face the viewer
- `use_frontface_falloff` · boolean — Blend brush influence by how much they face the front
- `use_smooth_stroke` · boolean — Brush lags behind mouse and follows a smoother path
- `use_persistent` · boolean — Sculpt on a persistent layer of the mesh
- `use_bidirectional` · boolean — Project vertices both along the projection direction and its inverse, choosing the closest intersection.
- `use_accumulate` · boolean — Accumulate stroke daubs on top of each other
- `use_space_attenuation` · boolean — Automatically adjust strength to give consistent results for different spacings
- `use_adaptive_space` · boolean — Space daubs according to surface orientation instead of screen space
- `use_locked_size` · enum · {VIEW, SCENE} — Measure brush size relative to the view or the scene
- `color_type` · enum · {COLOR, GRADIENT} — Use single color or gradient when painting
- `use_edge_to_edge` · boolean — Drag anchor brush from edge-to-edge
- `use_alpha` · boolean — When this is disabled, lock alpha while painting
- `curve_distance_falloff` · pointer · -> CurveMapping · read-only — Editable falloff curve
- `paint_curve` · pointer · -> PaintCurve — Active paint curve
- `gradient` · pointer · -> ColorRamp · read-only
- `gradient_stroke_mode` · enum · {PRESSURE, SPACING_REPEAT, SPACING_CLAMP}
- `gradient_fill_mode` · enum · {LINEAR, RADIAL}
- `use_primary_overlay` · boolean — Show texture in viewport
- `use_secondary_overlay` · boolean — Show texture in viewport
- `use_cursor_overlay` · boolean — Show cursor in viewport
- `use_cursor_overlay_override` · boolean — Don't show overlay during a stroke
- `use_primary_overlay_override` · boolean — Don't show overlay during a stroke
- `use_secondary_overlay_override` · boolean — Don't show overlay during a stroke
- `use_paint_sculpt` · boolean — Use this brush in sculpt mode
- `use_paint_uv_sculpt` · boolean — Use this brush in UV sculpt mode
- `use_paint_vertex` · boolean — Use this brush in vertex paint mode
- `use_paint_weight` · boolean — Use this brush in weight paint mode
- `use_paint_image` · boolean — Use this brush in texture paint mode
- `use_paint_grease_pencil` · boolean — Use this brush in Grease Pencil drawing mode
- `use_vertex_grease_pencil` · boolean — Use this brush in Grease Pencil vertex color mode
- `use_paint_sculpt_curves` · boolean — Use this brush in sculpt curves mode
- `texture_slot` · pointer · -> BrushTextureSlot · read-only
- `texture` · pointer · -> Texture
- `mask_texture_slot` · pointer · -> BrushTextureSlot · read-only
- `mask_texture` · pointer · -> Texture
- `texture_overlay_alpha` · int
- `mask_overlay_alpha` · int
- `cursor_overlay_alpha` · int
- `cursor_color_add` · float · [4] — Color of cursor when adding
- `cursor_color_subtract` · float · [4] — Color of cursor when subtracting
- `brush_capabilities` · pointer · -> BrushCapabilities · read-only — Brush's capabilities
- `sculpt_capabilities` · pointer · -> BrushCapabilitiesSculpt · read-only
- `image_paint_capabilities` · pointer · -> BrushCapabilitiesImagePaint · read-only
- `vertex_paint_capabilities` · pointer · -> BrushCapabilitiesVertexPaint · read-only
- `weight_paint_capabilities` · pointer · -> BrushCapabilitiesWeightPaint · read-only
- `gpencil_settings` · pointer · -> BrushGpencilSettings · read-only
- `curves_sculpt_settings` · pointer · -> BrushCurvesSculptSettings · read-only
- `mesh_automasking_settings` · pointer · -> MeshAutomaskingSettings · read-only

## BrushCapabilities

Read-only indications of supported operations

- `has_overlay` · boolean · read-only
- `has_random_texture_angle` · boolean · read-only
- `has_spacing` · boolean · read-only
- `has_smooth_stroke` · boolean · read-only

## BrushCapabilitiesImagePaint

Read-only indications of supported operations

- `has_accumulate` · boolean · read-only
- `has_space_attenuation` · boolean · read-only
- `has_radius` · boolean · read-only
- `has_color` · boolean · read-only

## BrushCapabilitiesSculpt

Read-only indications of which brush operations are supported by the current sculpt tool

- `has_accumulate` · boolean · read-only
- `has_auto_smooth` · boolean · read-only
- `has_normal_radius` · boolean · read-only
- `has_hardness` · boolean · read-only
- `has_topology_rake` · boolean · read-only
- `has_height` · boolean · read-only
- `has_plane_depth` · boolean · read-only
- `has_plane_height` · boolean · read-only
- `has_jitter` · boolean · read-only
- `has_normal_weight` · boolean · read-only
- `has_rake_factor` · boolean · read-only
- `has_persistence` · boolean · read-only
- `has_pinch_factor` · boolean · read-only
- `has_plane_offset` · boolean · read-only
- `has_random_texture_angle` · boolean · read-only
- `has_sculpt_plane` · boolean · read-only
- `has_color` · boolean · read-only
- `has_secondary_color` · boolean · read-only
- `has_smooth_stroke` · boolean · read-only
- `has_space_attenuation` · boolean · read-only
- `has_strength_pressure` · boolean · read-only
- `has_size_pressure` · boolean · read-only
- `has_auto_smooth_pressure` · boolean · read-only
- `has_hardness_pressure` · boolean · read-only
- `has_direction` · boolean · read-only
- `has_gravity` · boolean · read-only
- `has_tilt` · boolean · read-only
- `has_dyntopo` · boolean · read-only

## BrushCapabilitiesVertexPaint

Read-only indications of supported operations

- `has_color` · boolean · read-only

## BrushCapabilitiesWeightPaint

Read-only indications of supported operations

- `has_weight` · boolean · read-only

## BrushCurvesSculptSettings

- `add_amount` · int — Number of curves added by the Add brush
- `points_per_curve` · int — Number of control points in a newly added curve
- `use_uniform_scale` · boolean — Grow or shrink curves by changing their size uniformly instead of using trimming or extrapolation
- `minimum_length` · float — Avoid shrinking curves shorter than this length
- `use_length_interpolate` · boolean — Use length of the curves in close proximity
- `use_radius_interpolate` · boolean — Use radius of the curves in close proximity
- `use_point_count_interpolate` · boolean — Use the number of points from the curves in close proximity
- `use_shape_interpolate` · boolean — Use shape of the curves in close proximity
- `curve_length` · float — Length of newly added curves when it is not interpolated from other curves
- `minimum_distance` · float — Goal distance between curve roots for the Density brush
- `curve_radius` · float — Radius of newly added curves when it is not interpolated from other curves
- `density_add_attempts` · int — How many times the Density brush tries to add a new curve
- `density_mode` · enum · {AUTO, ADD, REMOVE} — Determines whether the brush adds or removes curves
- `curve_parameter_falloff` · pointer · -> CurveMapping · read-only — Falloff that is applied from the tip to the root of each curve

## BrushGpencilSettings

Settings for Grease Pencil brush

- `pen_strength` · float — Color strength for new strokes (affect alpha factor of color)
- `pen_jitter` · float — Jitter factor of brush radius for new strokes
- `random_pressure` · float — Randomness factor for pressure in new strokes
- `random_strength` · float — Randomness factor strength in new strokes
- `angle` · float — Direction of the stroke at which brush gives maximal thickness (0° for horizontal)
- `angle_factor` · float — Reduce brush thickness by this factor when stroke is perpendicular to 'Angle' direction
- `pen_smooth_factor` · float — Amount of smoothing to apply after finish newly created strokes, to reduce jitter/noise
- `pen_smooth_steps` · int — Number of times to smooth newly created strokes
- `pen_subdivision_steps` · int — Number of times to subdivide newly created strokes, for less jagged strokes
- `simplify_factor` · float — Factor of Simplify using adaptive algorithm
- `simplify_pixel_threshold` · float — Threshold in screen space used for the simplify algorithm. Points within this threshold are treated as if they were in a straight line.
- `curve_sensitivity` · pointer · -> CurveMapping · read-only — Curve used for the sensitivity
- `curve_strength` · pointer · -> CurveMapping · read-only — Curve used for the strength
- `curve_jitter` · pointer · -> CurveMapping · read-only — Curve used for the jitter effect
- `curve_random_pressure` · pointer · -> CurveMapping · read-only — Curve used for modulating effect
- `curve_random_strength` · pointer · -> CurveMapping · read-only — Curve used for modulating effect
- `curve_random_uv` · pointer · -> CurveMapping · read-only — Curve used for modulating effect
- `curve_random_hue` · pointer · -> CurveMapping · read-only — Curve used for modulating effect
- `curve_random_saturation` · pointer · -> CurveMapping · read-only — Curve used for modulating effect
- `curve_random_value` · pointer · -> CurveMapping · read-only — Curve used for modulating effect
- `fill_threshold` · float — Threshold to consider color transparent for filling
- `fill_factor` · float — Factor for fill boundary accuracy, higher values are more accurate but slower
- `fill_simplify_level` · int — Number of simplify steps (large values reduce fill accuracy)
- `uv_random` · float — Random factor for auto-generated UV rotation
- `hardness` · float — Gradient from the center of Dot and Box strokes (set to 1 for a solid stroke)
- `aspect` · float · [2]
- `input_samples` · int — Generated intermediate points for very fast mouse movements (Set to 0 to disable)
- `active_smooth_factor` · float — Amount of smoothing while drawing
- `eraser_strength_factor` · float — Amount of erasing for strength
- `eraser_thickness_factor` · float — Amount of erasing for thickness
- `vertex_mode` · enum · {STROKE, FILL, BOTH} — Defines how vertex color affect to the strokes
- `vertex_color_factor` · float — Factor used to mix vertex color to get final color
- `random_hue_factor` · float — Random factor to modify original hue
- `random_saturation_factor` · float — Random factor to modify original saturation
- `random_value_factor` · float — Random factor to modify original value
- `extend_stroke_factor` · float — Strokes end extension for closing gaps, use zero to disable
- `fill_extend_mode` · enum · {EXTEND, RADIUS} — Types of stroke extensions used for closing gaps
- `dilate` · int — Number of pixels to expand or contract fill area
- `outline_thickness_factor` · float — Thickness of the outline stroke relative to current brush thickness
- `conversion_threshold` · float — Threshold distance between points for conversion
- `use_pressure` · boolean — Use tablet pressure
- `use_strength_pressure` · boolean — Use tablet pressure for color strength
- `use_jitter_pressure` · boolean — Use tablet pressure for jitter
- `use_stroke_random_hue` · boolean — Use randomness at stroke level
- `use_stroke_random_sat` · boolean — Use randomness at stroke level
- `use_stroke_random_val` · boolean — Use randomness at stroke level
- `use_stroke_random_radius` · boolean — Use randomness at stroke level
- `use_stroke_random_strength` · boolean — Use randomness at stroke level
- `use_stroke_random_uv` · boolean — Use randomness at stroke level
- `use_random_press_hue` · boolean — Use pressure to modulate randomness
- `use_random_press_sat` · boolean — Use pressure to modulate randomness
- `use_random_press_val` · boolean — Use pressure to modulate randomness
- `use_random_press_radius` · boolean — Use pressure to modulate randomness
- `use_random_press_strength` · boolean — Use pressure to modulate randomness
- `use_random_press_uv` · boolean — Use pressure to modulate randomness
- `stroke_type` · enum · {STROKE, FILL, BOTH} — Mode to use when creating strokes
- `use_settings_stabilizer` · boolean — Draw lines with a delay to allow smooth strokes (press Shift key to override while drawing)
- `eraser_mode` · enum · {SOFT, HARD, STROKE} — Eraser Mode
- `caps_type` · enum · {ROUND, FLAT} — The shape of the start and end of the stroke
- `fill_draw_mode` · enum · {BOTH, STROKE, CONTROL} — Mode to draw boundary limits
- `fill_layer_mode` · enum · {VISIBLE, ACTIVE, ABOVE, BELOW, ALL_ABOVE, ALL_BELOW} — Layers used as boundaries
- `fill_direction` · enum · {NORMAL, INVERT} — Direction of the fill
- `fill_solver` · enum · {DELAUNAY, PIXEL} — Method used for when filling
- `fill_gap_factor` · float — The sensitivity of the gap detection. Higher values results in more gaps detected and as such can create smaller fills
- `pin_draw_mode` · boolean — Pin the mode to the brush
- `brush_draw_mode` · enum · {ACTIVE, MATERIAL, VERTEXCOLOR} — Preselected mode when using this brush
- `use_trim` · boolean — Trim intersecting stroke ends
- `use_settings_outline` · boolean — Convert stroke to outline
- `use_edit_position` · boolean — The brush affects the position of the point
- `use_edit_strength` · boolean — The brush affects the color strength of the point
- `use_edit_thickness` · boolean — The brush affects the thickness of the point
- `use_edit_uv` · boolean — The brush affects the UV rotation of the point
- `material` · pointer · -> Material — Material used for strokes drawn using this brush
- `material_alt` · pointer · -> Material — Material used for secondary uses for this brush
- `show_fill_boundary` · boolean — Show help lines for filling to see boundaries
- `show_fill_extend` · boolean — Show help lines for stroke extension
- `use_collide_strokes` · boolean — Check if extend lines collide with strokes
- `show_fill` · boolean — Show transparent lines to use as boundary for filling
- `use_auto_remove_fill_guides` · boolean — Automatically remove fill guide strokes after fill operation
- `fill_internal_gaps` · boolean — Stop at internal gaps
- `curve_type` · enum · {CATMULL_ROM, POLY, BEZIER, NURBS} — Type of curves
- `use_fill_limit` · boolean — Fill only visible areas in viewport
- `use_settings_postprocess` · boolean — Additional post processing options for new strokes
- `use_settings_random` · boolean — Random brush settings
- `use_material_pin` · boolean — Keep material assigned to brush
- `show_lasso` · boolean — Display fill color while drawing the stroke
- `use_occlude_eraser` · boolean — Erase only strokes visible and not occluded
- `use_keep_caps_eraser` · boolean — Keep the caps as they are and don't flatten them when erasing
- `use_active_layer_only` · boolean — Only edit the active layer of the object

## BrushTextureSlot

Texture slot for textures in a Brush data-block

- `angle` · float — Brush texture rotation
- `map_mode` · enum · {VIEW_PLANE, AREA_PLANE, TILED, 3D, RANDOM, STENCIL}
- `mask_map_mode` · enum · {VIEW_PLANE, TILED, RANDOM, STENCIL}
- `use_rake` · boolean
- `use_random` · boolean
- `random_angle` · float — Brush texture random angle
- `has_texture_angle_source` · boolean · read-only
- `has_random_texture_angle` · boolean · read-only
- `has_texture_angle` · boolean · read-only

## DynamicPaintBrushSettings

Brush settings

- `paint_color` · float · [3] — Color of the paint
- `paint_alpha` · float — Paint alpha
- `use_absolute_alpha` · boolean — Only increase alpha value if paint alpha is higher than existing
- `paint_wetness` · float — Paint wetness, visible in wetmap (some effects only affect wet paint)
- `use_paint_erase` · boolean — Erase / remove paint instead of adding it
- `wave_type` · enum · {CHANGE, DEPTH, FORCE, REFLECT}
- `wave_factor` · float — Multiplier for wave influence of this brush
- `wave_clamp` · float — Maximum level of surface intersection used to influence waves (use 0.0 to disable)
- `use_smudge` · boolean — Make this brush to smudge existing paint as it moves
- `smudge_strength` · float — Smudge effect strength
- `velocity_max` · float — Velocity considered as maximum influence (Blender units per frame)
- `use_velocity_alpha` · boolean — Multiply brush influence by velocity color ramp alpha
- `use_velocity_depth` · boolean — Multiply brush intersection depth (displace, waves) by velocity ramp alpha
- `use_velocity_color` · boolean — Replace brush color by velocity color ramp
- `paint_source` · enum · {PARTICLE_SYSTEM, POINT, DISTANCE, VOLUME_DISTANCE, VOLUME}
- `paint_distance` · float — Maximum distance from brush to mesh surface to affect paint
- `use_proximity_ramp_alpha` · boolean — Only read color ramp alpha
- `proximity_falloff` · enum · {SMOOTH, CONSTANT, RAMP} — Proximity falloff type
- `use_proximity_project` · boolean — Brush is projected to canvas from defined direction within brush proximity
- `ray_direction` · enum · {CANVAS, BRUSH, Z_AXIS} — Ray direction to use for projection (if brush object is located in that direction it's painted)
- `invert_proximity` · boolean — Proximity falloff is applied inside the volume
- `use_negative_volume` · boolean — Negate influence inside the volume
- `particle_system` · pointer · -> ParticleSystem — The particle system to paint with
- `use_particle_radius` · boolean — Use radius from particle settings
- `solid_radius` · float — Radius that will be painted solid
- `smooth_radius` · float — Smooth falloff added after solid radius
- `paint_ramp` · pointer · -> ColorRamp · read-only — Color ramp used to define proximity falloff
- `velocity_ramp` · pointer · -> ColorRamp · read-only — Color ramp used to define brush velocity effect

## ParticleBrush

Particle editing brush

- `size` · int — Radius of the brush in pixels
- `strength` · float — Brush strength
- `count` · int — Particle count
- `steps` · int — Brush steps
- `puff_mode` · enum · {ADD, SUB}
- `use_puff_volume` · boolean — Apply puff to unselected end-points (helps maintain hair volume when puffing root)
- `length_mode` · enum · {GROW, SHRINK}
- `curve` · pointer · -> CurveMapping · read-only
