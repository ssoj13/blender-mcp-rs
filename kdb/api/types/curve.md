---
area: api-types
group: curve
version: 5.2.2
---

# Blender 5.2.2 RNA types: curve

## AttributeGroupCurves

Group of geometry attributes

- `active` · pointer · -> Attribute — Active attribute
- `active_index` · int — Active attribute index or -1 when none are active

## Curve

Curve data-block storing curves, splines and NURBS

- `shape_keys` · pointer · -> Key · read-only
- `splines` · collection · [] of CurveSplines · read-only — Collection of splines in this curve data object
- `path_duration` · int — The number of frames that are needed to traverse the path, defining the maximum value for the 'Evaluation Time' setting
- `use_path` · boolean — Enable the curve to become a translation path
- `use_path_follow` · boolean — Make curve path children rotate along the path
- `use_path_clamp` · boolean — Clamp the curve path children so they cannot travel past the start/end point of the curve
- `use_stretch` · boolean — Option for curve-deform: make deformed child stretch along entire path
- `use_deform_bounds` · boolean — Option for curve-deform: Use the mesh bounds to clamp the deformation
- `use_radius` · boolean — Option for paths and curve-deform: apply the curve radius to objects following it and to deformed objects
- `bevel_mode` · enum · {ROUND, OBJECT, PROFILE} — Determine how to build the curve's bevel geometry
- `bevel_profile` · pointer · -> CurveProfile · read-only — The path for the curve's custom profile
- `bevel_resolution` · int — The number of segments in each quarter-circle of the bevel
- `offset` · float — Distance to move the curve parallel to its normals
- `extrude` · float — Length of the depth added in the local Z direction along the curve, perpendicular to its normals
- `bevel_depth` · float — Radius of the bevel geometry, not including extrusion
- `resolution_u` · int — Number of computed points in the U direction between every pair of control points
- `resolution_v` · int — The number of computed points in the V direction between every pair of control points
- `render_resolution_u` · int — Surface resolution in U direction used while rendering (zero uses preview resolution)
- `render_resolution_v` · int — Surface resolution in V direction used while rendering (zero uses preview resolution)
- `eval_time` · float — Parametric position along the length of the curve that Objects 'following' it should be at (position is evaluated by dividing by the 'Path Length' value)
- `bevel_object` · pointer · -> Object — The name of the Curve object that defines the bevel shape
- `taper_object` · pointer · -> Object — Curve object name that defines the taper (width)
- `dimensions` · enum · {2D, 3D} — Select 2D or 3D curve type
- `fill_mode` · enum · {FULL, BACK, FRONT, HALF} — Mode of filling curve
- `fill_solver` · enum · {SWEEP_LINE, CDT} — Triangulation solver for filling 2D curves
- `fill_rule` · enum · {EVEN_ODD, NONZERO} — Fill rule for Delaunay fill solver
- `twist_mode` · enum · {Z_UP, MINIMUM, TANGENT} — The type of tilt calculation for 3D Curves
- `taper_radius_mode` · enum · {OVERRIDE, MULTIPLY, ADD} — Determine how the effective radius of the spline point is computed when a taper object is specified
- `bevel_factor_mapping_start` · enum · {RESOLUTION, SEGMENTS, SPLINE} — Determine how the geometry start factor is mapped to a spline
- `bevel_factor_mapping_end` · enum · {RESOLUTION, SEGMENTS, SPLINE} — Determine how the geometry end factor is mapped to a spline
- `twist_smooth` · float — Smoothing iteration for tangents
- `use_fill_caps` · boolean — Fill caps for beveled curves
- `use_map_taper` · boolean — Map effect of the taper object to the beveled part of the curve
- `use_auto_texspace` · boolean — Adjust active object's texture space automatically when transforming object
- `texspace_location` · float · [3]
- `texspace_size` · float · [3]
- `materials` · collection · [] of IDMaterials · read-only
- `bevel_factor_start` · float — Define where along the spline the curve geometry starts (0 for the beginning, 1 for the end)
- `bevel_factor_end` · float — Define where along the spline the curve geometry ends (0 for the beginning, 1 for the end)
- `is_editmode` · boolean · read-only — True when used in editmode
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `cycles` · pointer · -> CyclesMeshSettings · read-only — Cycles mesh settings

## CurveMap

Curve in a curve mapping

- `points` · collection · [] of CurveMapPoints · read-only

## CurveMapPoint

Point of a curve used for a curve mapping

- `location` · float · [2] — X/Y coordinates of the curve point
- `handle_type` · enum · {AUTO, AUTO_CLAMPED, VECTOR} — Curve interpolation at this point: Bézier or vector
- `select` · boolean — Selection state of the curve point

## CurveMapping

Curve mapping to map color, vector and scalar values to other values using a user defined curve

- `tone` · enum · {STANDARD, FILMLIKE} — Tone of the curve
- `use_clip` · boolean — Force the curve view to fit a defined boundary
- `clip_min_x` · float
- `clip_min_y` · float
- `clip_max_x` · float
- `clip_max_y` · float
- `extend` · enum · {HORIZONTAL, EXTRAPOLATED} — Extrapolate the curve or extend it horizontally
- `curves` · collection · read-only
- `black_level` · float · [3] — For RGB curves, the color that black is mapped to
- `white_level` · float · [3] — For RGB curves, the color that white is mapped to

## CurvePaintSettings

- `curve_type` · enum · {POLY, BEZIER} — Type of curve to use for new strokes
- `use_corners_detect` · boolean — Detect corners and use non-aligned handles
- `use_pressure_radius` · boolean — Map tablet pressure to curve radius
- `use_stroke_endpoints` · boolean — Use the start of the stroke for the depth
- `use_offset_absolute` · boolean — Apply a fixed offset (don't scale by the radius)
- `use_project_only_selected` · boolean — Project the strokes only onto selected objects
- `error_threshold` · int — Allow deviation for a smoother, less precise line
- `fit_method` · enum · {REFIT, SPLIT} — Curve fitting method
- `corner_angle` · float — Angles above this are considered corners
- `radius_min` · float — Minimum radius when the minimum pressure is applied (also the minimum when tapering)
- `radius_max` · float — Radius to use when the maximum pressure is applied (or when a tablet isn't used)
- `radius_taper_start` · float — Taper factor for the radius of each point along the curve
- `radius_taper_end` · float — Taper factor for the radius of each point along the curve
- `surface_offset` · float — Offset the stroke from the surface
- `depth_mode` · enum · {CURSOR, SURFACE} — Method of projecting depth
- `surface_plane` · enum · {NORMAL_VIEW, NORMAL_SURFACE, VIEW} — Plane for projected stroke

## CurvePoint

Curve control point

- `position` · float · [3]
- `radius` · float
- `index` · int · read-only — Index of this point

## CurveProfile

Profile Path editor used to build a profile path

- `preset` · enum · {LINE, SUPPORTS, CORNICE, CROWN, STEPS}
- `use_clip` · boolean — Force the path view to fit a defined boundary
- `use_sample_straight_edges` · boolean — Sample edges with vector handles
- `use_sample_even_lengths` · boolean — Sample edges with even lengths
- `points` · collection · [] of CurveProfilePoints · read-only — Profile control points
- `segments` · collection · read-only — Segments sampled from control points

## CurveProfilePoint

Point of a path used to define a profile

- `location` · float · [2] — X/Y coordinates of the path point
- `handle_type_1` · enum · {AUTO, VECTOR, FREE, ALIGN} — Path interpolation at this point
- `handle_type_2` · enum · {AUTO, VECTOR, FREE, ALIGN} — Path interpolation at this point
- `select` · boolean — Selection state of the path point

## CurveSlice

A single curve from a curves data-block

- `points` · collection · read-only — Control points of the curve
- `first_point_index` · int · read-only — The index of this curve's first control point
- `points_length` · int · read-only — Number of control points in the curve
- `index` · int · read-only — Index of this curve

## CurveSplines

Collection of curve splines

- `active` · pointer · -> Spline — Active curve spline

## Curves

Hair data-block for hair curves

- `curves` · collection · read-only — All curves in the data-block
- `points` · collection · read-only — Control points of all curves
- `position_data` · collection · read-only
- `curve_offset_data` · collection · read-only
- `normals` · collection · read-only — The curve normal value at each of the curve's control points
- `materials` · collection · [] of IDMaterials · read-only
- `surface` · pointer · -> Object — Mesh object that the curves can be attached to
- `surface_uv_map` · string — The name of the attribute on the surface mesh used to define the attachment of each curve
- `use_mirror_x` · boolean — Enable symmetry in the X axis
- `use_mirror_y` · boolean — Enable symmetry in the Y axis
- `use_mirror_z` · boolean — Enable symmetry in the Z axis
- `selection_domain` · enum · {POINT, CURVE}
- `use_sculpt_collision` · boolean — Enable collision with the surface while sculpting
- `surface_collision_distance` · float — Distance to keep the curves away from the surface
- `attributes` · collection · [] of AttributeGroupCurves · read-only — Geometry attributes
- `color_attributes` · collection · [] of AttributeGroupCurves · read-only — Geometry color attributes
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## EQCurveMappingData

EQCurveMappingData

- `curve_mapping` · pointer · -> CurveMapping · read-only

## FCurve

F-Curve defining values of a period of time

- `extrapolation` · enum · {CONSTANT, LINEAR} — Method used for evaluating value of F-Curve outside first and last keyframes
- `driver` · pointer · -> Driver · read-only — Channel Driver (only set for Driver F-Curves)
- `group` · pointer · -> ActionGroup — Action Group that this F-Curve belongs to
- `data_path` · string — RNA Path to property affected by F-Curve
- `array_index` · int — Index to the specific property affected by F-Curve if applicable
- `color_mode` · enum · {AUTO_RAINBOW, AUTO_RGB, AUTO_YRGB, CUSTOM} — Method used to determine color of F-Curve in Graph Editor
- `color` · float · [3] — Color of the F-Curve in the Graph Editor
- `select` · boolean — F-Curve is selected for editing
- `lock` · boolean — F-Curve's settings cannot be edited
- `mute` · boolean — Disable F-Curve evaluation
- `hide` · boolean — F-Curve and its keyframes are hidden in the Graph Editor graphs
- `auto_smoothing` · enum · {NONE, CONT_ACCEL} — Algorithm used to compute automatic handles
- `is_valid` · boolean — False when F-Curve could not be evaluated in past, so should be skipped when evaluating
- `is_empty` · boolean · read-only — True if the curve contributes no animation due to lack of keyframes or useful modifiers, and should be deleted
- `sampled_points` · collection · read-only — Sampled animation data
- `keyframe_points` · collection · [] of FCurveKeyframePoints · read-only — User-editable keyframes
- `modifiers` · collection · [] of FCurveModifiers · read-only — Modifiers affecting the shape of the F-Curve

## FCurveSample

Sample point for F-Curve

- `select` · boolean — Selection status
- `co` · float · [2] — Point coordinates

## ShapeKeyCurvePoint

Point in a shape key for curves

- `co` · float · [3]
- `tilt` · float — Tilt in 3D View
- `radius` · float — Radius for beveling

## TextCurve

Curve data-block used for storing text

- `align_x` · enum · {LEFT, CENTER, RIGHT, JUSTIFY, FLUSH} — Text horizontal alignment from the object or text box center
- `align_y` · enum · {TOP, TOP_BASELINE, CENTER, BOTTOM_BASELINE, BOTTOM} — Text vertical alignment from the object center
- `overflow` · enum · {NONE, SCALE, TRUNCATE} — Handle the text behavior when it does not fit in the text boxes
- `size` · float
- `small_caps_scale` · float — Scale of small capitals
- `space_line` · float
- `space_word` · float
- `space_character` · float
- `shear` · float — Italic angle of the characters
- `offset_x` · float — Horizontal offset from the object origin
- `offset_y` · float — Vertical offset from the object origin
- `underline_position` · float — Vertical position of underline
- `underline_height` · float
- `text_boxes` · collection · read-only
- `active_textbox` · int
- `family` · string — Use objects as font characters (give font objects a common name followed by the character they represent, eg. 'family-a', 'family-b', etc, set this setting to 'family-', and turn on Vertex Instancing)
- `body` · string — Content of this text object
- `body_format` · collection · read-only — Stores the style of each character
- `follow_curve` · pointer · -> Object — Curve deforming text object
- `font` · pointer · -> VectorFont
- `font_bold` · pointer · -> VectorFont
- `font_italic` · pointer · -> VectorFont
- `font_bold_italic` · pointer · -> VectorFont
- `edit_format` · pointer · -> TextCharacterFormat · read-only — Editing settings character formatting
- `use_fast_edit` · boolean — Don't fill polygons while editing
- `is_select_bold` · boolean · read-only — Whether the selected text is bold
- `is_select_italic` · boolean · read-only — Whether the selected text is italics
- `is_select_underline` · boolean · read-only — Whether the selected text is underlined
- `is_select_smallcaps` · boolean · read-only — Whether the selected text is small caps
- `has_selection` · boolean · read-only — Whether there is any text selected

## ThemeCommonCurves

Shared curve theme properties

- `handle_free` · float · [3]
- `handle_sel_free` · float · [3]
- `handle_auto` · float · [3]
- `handle_sel_auto` · float · [3]
- `handle_vect` · float · [3]
- `handle_sel_vect` · float · [3]
- `handle_align` · float · [3]
- `handle_sel_align` · float · [3]
- `handle_auto_clamped` · float · [3]
- `handle_sel_auto_clamped` · float · [3]
- `handle_vertex` · float · [3]
- `handle_vertex_select` · float · [3]
- `handle_vertex_size` · int
