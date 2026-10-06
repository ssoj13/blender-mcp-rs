---
area: api-ops
group: curves
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.curves

- `bpy.ops.curves.add_bezier(radius: float, enter_editmode: boolean, align: enum{WORLD, VIEW, CURSOR}, location: float, rotation: float, scale: float)` — Add new Bézier curve
- `bpy.ops.curves.add_circle(radius: float, enter_editmode: boolean, align: enum{WORLD, VIEW, CURSOR}, location: float, rotation: float, scale: float)` — Add new circle curve
- `bpy.ops.curves.attribute_set(value_float: float, value_float_vector_2d: float, value_float_vector_3d: float, value_float_vector_4d: float, value_int: int, value_int_vector_2d: int, value_color: float, value_bool: boolean)` — Set values of the active attribute for selected elements
- `bpy.ops.curves.convert_from_particle_system()` — Add a new curves object based on the current state of the particle system
- `bpy.ops.curves.convert_to_particle_system()` — Add a new or update an existing hair particle system on the surface object
- `bpy.ops.curves.curve_type_set(type: enum{CATMULL_ROM, POLY, BEZIER, NURBS}, use_handles: boolean)` — Set type of selected curves
- `bpy.ops.curves.cyclic_toggle()` — Make active curve closed/open loop
- `bpy.ops.curves.delete()` — Remove selected control points or curves
- `bpy.ops.curves.draw(error_threshold: float, fit_method: enum{REFIT, SPLIT}, corner_angle: float, use_cyclic: boolean, stroke: collection, wait_for_input: boolean, is_curve_2d: boolean, bezier_as_nurbs: boolean)` — Draw a freehand curve
- `bpy.ops.curves.duplicate()` — Copy selected points or curves
- `bpy.ops.curves.duplicate_move(CURVES_OT_duplicate: pointer, TRANSFORM_OT_translate: pointer)` — Make copies of selected elements and move them
- `bpy.ops.curves.extrude()` — Extrude selected control point(s)
- `bpy.ops.curves.extrude_move(CURVES_OT_extrude: pointer, TRANSFORM_OT_translate: pointer)` — Extrude curve and move result
- `bpy.ops.curves.handle_type_set(type: enum{AUTO, VECTOR, ALIGN, FREE_ALIGN, TOGGLE_FREE_ALIGN})` — Set the handle type for Bézier curves
- `bpy.ops.curves.pen(extend: boolean, deselect: boolean, toggle: boolean, deselect_all: boolean, select_passthrough: boolean, extrude_point: boolean, extrude_handle: enum{AUTO, VECTOR}, delete_point: boolean, insert_point: boolean, move_segment: boolean, select_point: boolean, move_point: boolean, cycle_handle_type: boolean, size: float)` — Construct and edit Bézier curves
- `bpy.ops.curves.sculptmode_toggle()` — Enter/Exit sculpt mode for curves
- `bpy.ops.curves.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — (De)select all control points
- `bpy.ops.curves.select_ends(amount_start: int, amount_end: int)` — Select end points of curves
- `bpy.ops.curves.select_less()` — Shrink the selection by one point
- `bpy.ops.curves.select_linked()` — Select all points in curves with any point selection
- `bpy.ops.curves.select_linked_pick(deselect: boolean)` — Select all points in the curve under the cursor
- `bpy.ops.curves.select_more()` — Grow the selection by one point
- `bpy.ops.curves.select_random(seed: int, probability: float)` — Randomize existing selection or create new random selection
- `bpy.ops.curves.separate()` — Separate selected geometry into a new object
- `bpy.ops.curves.set_selection_domain(domain: enum{POINT, CURVE})` — Change the mode used for selection masking in curves sculpt mode
- `bpy.ops.curves.snap_curves_to_surface(attach_mode: enum{NEAREST, DEFORM})` — Move curves so that the first point is exactly on the surface mesh
- `bpy.ops.curves.split()` — Split selected points
- `bpy.ops.curves.subdivide(number_cuts: int)` — Subdivide selected curve segments
- `bpy.ops.curves.surface_set()` — Use the active object as surface for selected curves objects and set it as the parent
- `bpy.ops.curves.switch_direction()` — Reverse the direction of the selected curves
- `bpy.ops.curves.tilt_clear()` — Clear the tilt of selected control points