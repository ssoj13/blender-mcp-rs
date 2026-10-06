---
area: api-ops
group: action
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.action

- `bpy.ops.action.bake_keys()` — Add keyframes on every frame between the selected keyframes
- `bpy.ops.action.clean(threshold: float, channels: boolean)` — Simplify F-Curves by removing closely spaced keyframes
- `bpy.ops.action.clickselect(wait_to_deselect_others: boolean, use_select_on_click: boolean, mouse_x: int, mouse_y: int, extend: boolean, deselect_all: boolean, column: boolean, channel: boolean)` — Select keyframes by clicking on them
- `bpy.ops.action.copy()` — Copy selected keyframes to the internal clipboard
- `bpy.ops.action.delete(confirm: boolean)` — Remove all selected keyframes
- `bpy.ops.action.duplicate()` — Make a copy of all selected keyframes
- `bpy.ops.action.duplicate_move(ACTION_OT_duplicate: pointer, TRANSFORM_OT_transform: pointer)` — Make a copy of all selected keyframes and move them
- `bpy.ops.action.easing_type(type: enum{AUTO, EASE_IN, EASE_OUT, EASE_IN_OUT})` — Set easing type for the F-Curve segments starting from the selected keyframes
- `bpy.ops.action.extrapolation_type(type: enum{CONSTANT, LINEAR, MAKE_CYCLIC, CLEAR_CYCLIC})` — Set extrapolation mode for selected F-Curves
- `bpy.ops.action.frame_jump()` — Set the current frame to the average frame value of selected keyframes
- `bpy.ops.action.handle_type(type: enum{FREE, ALIGNED, VECTOR, AUTO, AUTO_CLAMPED})` — Set type of handle for selected keyframes
- `bpy.ops.action.interpolation_type(type: enum{CONSTANT, LINEAR, BEZIER, SINE, QUAD, CUBIC, QUART, QUINT, EXPO, CIRC, BACK, BOUNCE, ELASTIC})` — Set interpolation mode for the F-Curve segments starting from the selected keyframes
- `bpy.ops.action.keyframe_insert(type: enum{ALL, SEL, GROUP})` — Insert keyframes for the specified channels
- `bpy.ops.action.keyframe_type(type: enum{KEYFRAME, BREAKDOWN, MOVING_HOLD, EXTREME, JITTER, GENERATED})` — Set type of keyframe for the selected keyframes
- `bpy.ops.action.markers_make_local()` — Move selected scene markers to the active Action as local 'pose' markers
- `bpy.ops.action.mirror(type: enum{CFRA, MARKER, XAXIS})` — Flip selected keyframes over the selected mirror line
- `bpy.ops.action.new()` — Create new action
- `bpy.ops.action.paste(offset: enum{START, END, RELATIVE, NONE}, merge: enum{MIX, OVER_ALL, OVER_RANGE, OVER_RANGE_ALL}, flipped: boolean)` — Paste keyframes from the internal clipboard for the selected channels, starting on the current frame
- `bpy.ops.action.previewrange_set()` — Set Preview Range based on extents of selected Keyframes
- `bpy.ops.action.push_down()` — Push action down on to the NLA stack as a new strip
- `bpy.ops.action.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Toggle selection of all keyframes
- `bpy.ops.action.select_box(axis_range: boolean, xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean, mode: enum{SET, ADD, SUB}, tweak: boolean)` — Select all keyframes within the specified region
- `bpy.ops.action.select_by_type(extend: boolean, type: enum{KEYFRAME, BREAKDOWN, MOVING_HOLD, EXTREME, JITTER, GENERATED})` — Select all keyframes of the given type
- `bpy.ops.action.select_circle(x: int, y: int, radius: int, wait_for_input: boolean, mode: enum{SET, ADD, SUB})` — Select keyframe points using circle selection
- `bpy.ops.action.select_column(mode: enum{KEYS, CFRA, MARKERS_COLUMN, MARKERS_BETWEEN})` — Select all keyframes on the specified frame(s)
- `bpy.ops.action.select_lasso(path: collection, use_smooth_stroke: boolean, smooth_stroke_factor: float, smooth_stroke_radius: int, mode: enum{SET, ADD, SUB})` — Select keyframe points using lasso selection
- `bpy.ops.action.select_leftright(mode: enum{CHECK, LEFT, RIGHT}, extend: boolean)` — Select keyframes to the left or the right of the current frame
- `bpy.ops.action.select_less()` — Deselect keyframes on ends of selection islands
- `bpy.ops.action.select_linked()` — Select keyframes occurring in the same F-Curves as selected ones
- `bpy.ops.action.select_more()` — Select keyframes beside already selected ones
- `bpy.ops.action.snap(type: enum{CFRA, NEAREST_FRAME, NEAREST_SECOND, NEAREST_MARKER})` — Snap selected keyframes to the times specified
- `bpy.ops.action.stash(create_new: boolean)` — Store this action in the NLA stack as a non-contributing strip for later use
- `bpy.ops.action.stash_and_create()` — Store this action in the NLA stack as a non-contributing strip for later use, and create a new action
- `bpy.ops.action.unlink(force_delete: boolean)` — Unlink this action from the active action slot (and/or exit Tweak Mode)
- `bpy.ops.action.view_all()` — Reset viewable area to show full keyframe range
- `bpy.ops.action.view_frame()` — Move the view to the current frame
- `bpy.ops.action.view_selected()` — Reset viewable area to show selected keyframes range