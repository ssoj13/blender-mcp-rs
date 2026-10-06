---
area: api-ops
group: curve
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.curve

- `bpy.ops.curve.cyclic_toggle(direction: enum{CYCLIC_U, CYCLIC_V})` — Make active spline closed/open loop
- `bpy.ops.curve.de_select_first()` — (De)select first of visible part of each NURBS
- `bpy.ops.curve.de_select_last()` — (De)select last of visible part of each NURBS
- `bpy.ops.curve.decimate(ratio: float)` — Simplify selected curves
- `bpy.ops.curve.delete(type: enum{VERT, SEGMENT})` — Delete selected control points or segments
- `bpy.ops.curve.dissolve_verts()` — Delete selected control points, correcting surrounding handles
- `bpy.ops.curve.draw(error_threshold: float, fit_method: enum{REFIT, SPLIT}, corner_angle: float, use_cyclic: boolean, stroke: collection, wait_for_input: boolean)` — Draw a freehand spline
- `bpy.ops.curve.duplicate()` — Duplicate selected control points
- `bpy.ops.curve.duplicate_move(CURVE_OT_duplicate: pointer, TRANSFORM_OT_translate: pointer)` — Duplicate curve and move
- `bpy.ops.curve.extrude(mode: enum{INIT, DUMMY, TRANSLATION, ROTATION, RESIZE, SKIN_RESIZE, TOSPHERE, SHEAR, BEND, SHRINKFATTEN, TILT, TRACKBALL, PUSHPULL, CREASE, VERTEX_CREASE, MIRROR, BONE_SIZE, BONE_ENVELOPE, BONE_ENVELOPE_DIST, CURVE_SHRINKFATTEN, MASK_SHRINKFATTEN, BONE_ROLL, TIME_TRANSLATE, TIME_SLIDE, TIME_SCALE, TIME_EXTEND, BAKE_TIME, BWEIGHT, ALIGN, EDGESLIDE, SEQSLIDE, GPENCIL_OPACITY})` — Extrude selected control point(s)
- `bpy.ops.curve.extrude_move(CURVE_OT_extrude: pointer, TRANSFORM_OT_translate: pointer)` — Extrude curve and move result
- `bpy.ops.curve.handle_type_set(type: enum{AUTOMATIC, VECTOR, ALIGNED, FREE_ALIGN, TOGGLE_FREE_ALIGN})` — Set type of handles for selected control points
- `bpy.ops.curve.hide(unselected: boolean)` — Hide (un)selected control points
- `bpy.ops.curve.make_segment()` — Join two curves by their selected ends
- `bpy.ops.curve.match_texture_space()` — Match texture space to object's bounding box
- `bpy.ops.curve.normals_make_consistent(calc_length: boolean)` — Recalculate the direction of selected handles
- `bpy.ops.curve.pen(extend: boolean, deselect: boolean, toggle: boolean, deselect_all: boolean, select_passthrough: boolean, extrude_point: boolean, extrude_handle: enum{AUTO, VECTOR}, delete_point: boolean, insert_point: boolean, move_segment: boolean, select_point: boolean, move_point: boolean, close_spline: boolean, close_spline_method: enum{OFF, ON_PRESS, ON_CLICK}, toggle_vector: boolean, cycle_handle_type: boolean)` — Construct and edit splines
- `bpy.ops.curve.primitive_bezier_circle_add(radius: float, enter_editmode: boolean, align: enum{WORLD, VIEW, CURSOR}, location: float, rotation: float, scale: float)` — Construct a Bézier Circle
- `bpy.ops.curve.primitive_bezier_curve_add(radius: float, enter_editmode: boolean, align: enum{WORLD, VIEW, CURSOR}, location: float, rotation: float, scale: float)` — Construct a Bézier Curve
- `bpy.ops.curve.primitive_nurbs_circle_add(radius: float, enter_editmode: boolean, align: enum{WORLD, VIEW, CURSOR}, location: float, rotation: float, scale: float)` — Construct a Nurbs Circle
- `bpy.ops.curve.primitive_nurbs_curve_add(radius: float, enter_editmode: boolean, align: enum{WORLD, VIEW, CURSOR}, location: float, rotation: float, scale: float)` — Construct a Nurbs Curve
- `bpy.ops.curve.primitive_nurbs_path_add(radius: float, enter_editmode: boolean, align: enum{WORLD, VIEW, CURSOR}, location: float, rotation: float, scale: float)` — Construct a Path
- `bpy.ops.curve.radius_set(radius: float)` — Set per-point radius which is used for bevel tapering
- `bpy.ops.curve.reveal(select: boolean)` — Reveal hidden control points
- `bpy.ops.curve.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — (De)select all control points
- `bpy.ops.curve.select_less()` — Deselect control points at the boundary of each selection region
- `bpy.ops.curve.select_linked()` — Select all control points linked to the current selection
- `bpy.ops.curve.select_linked_pick(deselect: boolean)` — Select all control points linked to already selected ones
- `bpy.ops.curve.select_more()` — Select control points at the boundary of each selection region
- `bpy.ops.curve.select_next()` — Select control points following already selected ones along the curves
- `bpy.ops.curve.select_nth(skip: int, nth: int, offset: int)` — Deselect every Nth point starting from the active one
- `bpy.ops.curve.select_previous()` — Select control points preceding already selected ones along the curves
- `bpy.ops.curve.select_random(ratio: float, seed: int, action: enum{SELECT, DESELECT})` — Randomly select some control points
- `bpy.ops.curve.select_row()` — Select a row of control points including active one. Successive use on the same point switches between U/V directions
- `bpy.ops.curve.select_similar(type: enum{TYPE, RADIUS, WEIGHT, DIRECTION}, compare: enum{EQUAL, GREATER, LESS}, threshold: float)` — Select similar curve points by property type
- `bpy.ops.curve.separate()` — Separate selected points from connected unselected points into a new object
- `bpy.ops.curve.shade_flat()` — Set shading to flat
- `bpy.ops.curve.shade_smooth()` — Set shading to smooth
- `bpy.ops.curve.shortest_path_pick()` — Select shortest path between two selections
- `bpy.ops.curve.smooth()` — Flatten angles of selected points
- `bpy.ops.curve.smooth_radius()` — Interpolate radii of selected points
- `bpy.ops.curve.smooth_tilt()` — Interpolate tilt of selected points
- `bpy.ops.curve.smooth_weight()` — Interpolate weight of selected points
- `bpy.ops.curve.spin(center: float, axis: float)` — Extrude selected boundary row around pivot point and current view axis
- `bpy.ops.curve.spline_type_set(type: enum{POLY, BEZIER, NURBS}, use_handles: boolean)` — Set type of active spline
- `bpy.ops.curve.spline_weight_set(weight: float)` — Set softbody goal weight for selected points
- `bpy.ops.curve.split()` — Split off selected points from connected unselected points
- `bpy.ops.curve.subdivide(number_cuts: int)` — Subdivide selected segments
- `bpy.ops.curve.switch_direction()` — Switch direction of selected splines
- `bpy.ops.curve.tilt_clear()` — Clear the tilt of selected control points
- `bpy.ops.curve.vertex_add(location: float)` — Add a new control point (linked to only selected end-curve one, if any)