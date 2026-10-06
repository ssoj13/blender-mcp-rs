---
area: api-ops
group: mask
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.mask

- `bpy.ops.mask.add_feather_vertex(location: float)` — Add vertex to feather
- `bpy.ops.mask.add_feather_vertex_slide(MASK_OT_add_feather_vertex: pointer, MASK_OT_slide_point: pointer)` — Add new vertex to feather and slide it
- `bpy.ops.mask.add_vertex(location: float)` — Add vertex to active spline
- `bpy.ops.mask.add_vertex_slide(MASK_OT_add_vertex: pointer, MASK_OT_slide_point: pointer)` — Add new vertex and slide it
- `bpy.ops.mask.copy_splines()` — Copy the selected splines to the internal clipboard
- `bpy.ops.mask.cyclic_toggle()` — Toggle cyclic for selected splines
- `bpy.ops.mask.delete(confirm: boolean)` — Delete selected control points or splines
- `bpy.ops.mask.duplicate()` — Duplicate selected control points and segments between them
- `bpy.ops.mask.duplicate_move(MASK_OT_duplicate: pointer, TRANSFORM_OT_translate: pointer)` — Duplicate mask and move
- `bpy.ops.mask.feather_weight_clear()` — Reset the feather weight to zero
- `bpy.ops.mask.handle_type_set(type: enum{AUTO, VECTOR, ALIGNED, ALIGNED_DOUBLESIDE, FREE})` — Set type of handles for selected control points
- `bpy.ops.mask.hide_view_clear(select: boolean)` — Reveal temporarily hidden mask layers
- `bpy.ops.mask.hide_view_set(unselected: boolean)` — Temporarily hide mask layers
- `bpy.ops.mask.layer_move(direction: enum{UP, DOWN})` — Move the active layer up/down in the list
- `bpy.ops.mask.layer_new(name: string)` — Add new mask layer for masking
- `bpy.ops.mask.layer_remove()` — Remove mask layer
- `bpy.ops.mask.move_to_layer(target_layer_name: string, add_new_layer: boolean)` — Move the active spline to layer
- `bpy.ops.mask.new(name: string)` — Create new mask
- `bpy.ops.mask.normals_make_consistent()` — Recalculate the direction of selected handles
- `bpy.ops.mask.parent_clear()` — Clear the mask's parenting
- `bpy.ops.mask.parent_set()` — Set the mask's parenting
- `bpy.ops.mask.paste_splines()` — Paste splines from the internal clipboard
- `bpy.ops.mask.primitive_circle_add(size: float, location: float)` — Add new circle-shaped spline
- `bpy.ops.mask.primitive_square_add(size: float, location: float)` — Add new square-shaped spline
- `bpy.ops.mask.select(extend: boolean, deselect: boolean, toggle: boolean, deselect_all: boolean, select_passthrough: boolean, location: float)` — Select spline points
- `bpy.ops.mask.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Change selection of all curve points
- `bpy.ops.mask.select_box(xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean, mode: enum{SET, ADD, SUB})` — Select curve points using box selection
- `bpy.ops.mask.select_circle(x: int, y: int, radius: int, wait_for_input: boolean, mode: enum{SET, ADD, SUB})` — Select curve points using circle selection
- `bpy.ops.mask.select_lasso(path: collection, use_smooth_stroke: boolean, smooth_stroke_factor: float, smooth_stroke_radius: int, mode: enum{SET, ADD, SUB})` — Select curve points using lasso selection
- `bpy.ops.mask.select_less()` — Deselect spline points at the boundary of each selection region
- `bpy.ops.mask.select_linked()` — Select all curve points linked to already selected ones
- `bpy.ops.mask.select_linked_pick(deselect: boolean)` — (De)select all points linked to the curve under the mouse cursor
- `bpy.ops.mask.select_more()` — Select more spline points connected to initial selection
- `bpy.ops.mask.shape_key_clear()` — Remove mask shape keyframe for active mask layer at the current frame
- `bpy.ops.mask.shape_key_feather_reset()` — Reset feather weights on all selected points animation values
- `bpy.ops.mask.shape_key_insert()` — Insert mask shape keyframe for active mask layer at the current frame
- `bpy.ops.mask.shape_key_rekey(location: boolean, feather: boolean)` — Recalculate animation data on selected points for frames selected in the dopesheet
- `bpy.ops.mask.slide_point(slide_feather: boolean, is_new_point: boolean)` — Slide control points
- `bpy.ops.mask.slide_spline_curvature()` — Slide a point on the spline to define its curvature
- `bpy.ops.mask.switch_direction()` — Switch direction of selected splines