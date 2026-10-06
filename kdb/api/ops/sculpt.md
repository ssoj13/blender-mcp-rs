---
area: api-ops
group: sculpt
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.sculpt

- `bpy.ops.sculpt.brush_stroke(stroke: collection, mode: enum{NORMAL, INVERT}, brush_toggle: enum{None, SMOOTH, ERASE, MASK}, pen_flip: boolean, override_location: boolean, ignore_background_click: boolean)` — Sculpt a stroke into the geometry
- `bpy.ops.sculpt.cloth_filter(start_mouse: int, area_normal_radius: float, strength: float, iteration_count: int, event_history: collection, type: enum{GRAVITY, INFLATE, EXPAND, PINCH, SCALE}, force_axis: enum{X, Y, Z}, orientation: enum{LOCAL, WORLD, VIEW}, cloth_mass: float, cloth_damping: float, use_face_sets: boolean, use_collisions: boolean)` — Applies a cloth simulation deformation to the entire mesh
- `bpy.ops.sculpt.color_filter(start_mouse: int, area_normal_radius: float, strength: float, iteration_count: int, event_history: collection, type: enum{FILL, HUE, SATURATION, VALUE, BRIGHTNESS, CONTRAST, SMOOTH, RED, GREEN, BLUE}, fill_color: float, use_immediate: boolean, use_secondary_color: boolean)` — Applies a filter to modify the active color attribute
- `bpy.ops.sculpt.detail_flood_fill()` — Flood fill the mesh with the selected detail setting
- `bpy.ops.sculpt.dynamic_topology_toggle()` — Dynamic topology alters the mesh topology while sculpting
- `bpy.ops.sculpt.dyntopo_detail_size_edit()` — Modify the detail size of dyntopo interactively
- `bpy.ops.sculpt.expand(target: enum{MASK, FACE_SETS, COLOR}, falloff_type: enum{GEODESIC, TOPOLOGY, TOPOLOGY_DIAGONALS, NORMALS, SPHERICAL, BOUNDARY_TOPOLOGY, BOUNDARY_FACE_SET, ACTIVE_FACE_SET}, invert: boolean, use_mask_preserve: boolean, use_falloff_gradient: boolean, use_modify_active: boolean, use_reposition_pivot: boolean, max_geodesic_move_preview: int, use_auto_mask: boolean, normal_falloff_smooth: int)` — Generic sculpt expand operator
- `bpy.ops.sculpt.face_set_box_gesture(xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean, use_front_faces_only: boolean)` — Add a face set in a rectangle defined by the cursor
- `bpy.ops.sculpt.face_set_change_visibility(mode: enum{TOGGLE, SHOW_ACTIVE, HIDE_ACTIVE}, active_face_set: int)` — Change the visibility of the face sets of the sculpt
- `bpy.ops.sculpt.face_set_edit(active_face_set: int, mode: enum{GROW, SHRINK, DELETE_GEOMETRY, FAIR_POSITIONS, FAIR_TANGENCY}, strength: float, modify_hidden: boolean)` — Edits the current active face set
- `bpy.ops.sculpt.face_set_extract(add_boundary_loop: boolean, smooth_iterations: int, apply_shrinkwrap: boolean, add_solidify: boolean)` — Create a new mesh object from the selected face set
- `bpy.ops.sculpt.face_set_lasso_gesture(path: collection, use_smooth_stroke: boolean, smooth_stroke_factor: float, smooth_stroke_radius: int, use_front_faces_only: boolean)` — Add a face set in a shape defined by the cursor
- `bpy.ops.sculpt.face_set_line_gesture(xstart: int, xend: int, ystart: int, yend: int, flip: boolean, cursor: int, use_front_faces_only: boolean, use_limit_to_segment: boolean)` — Add a face set to one side of a line defined by the cursor
- `bpy.ops.sculpt.face_set_polyline_gesture(path: collection, use_front_faces_only: boolean)` — Add a face set in a shape defined by the cursor
- `bpy.ops.sculpt.face_sets_create(mode: enum{MASKED, VISIBLE, ALL, SELECTION})` — Create a new face set
- `bpy.ops.sculpt.face_sets_init(mode: enum{LOOSE_PARTS, MATERIALS, NORMALS, UV_SEAMS, CREASES, BEVEL_WEIGHT, SHARP_EDGES, FACE_SET_BOUNDARIES}, threshold: float)` — Initializes all face sets in the mesh
- `bpy.ops.sculpt.face_sets_randomize_colors()` — Generates a new set of random colors to render the face sets in the viewport
- `bpy.ops.sculpt.mask_by_color(contiguous: boolean, invert: boolean, preserve_previous_mask: boolean, threshold: float, location: int)` — Creates a mask based on the active color attribute
- `bpy.ops.sculpt.mask_filter(filter_type: enum{SMOOTH, SHARPEN, GROW, SHRINK, CONTRAST_INCREASE, CONTRAST_DECREASE}, iterations: int, auto_iteration_count: boolean)` — Applies a filter to modify the current mask
- `bpy.ops.sculpt.mask_from_boundary(mix_mode: enum{MIX, MULTIPLY, DIVIDE, ADD, SUBTRACT}, mix_factor: float, settings_source: enum{OPERATOR, BRUSH, SCENE}, boundary_mode: enum{MESH, FACE_SETS}, propagation_steps: int)` — Creates a mask based on the boundaries of the surface
- `bpy.ops.sculpt.mask_from_cavity(mix_mode: enum{MIX, MULTIPLY, DIVIDE, ADD, SUBTRACT}, mix_factor: float, settings_source: enum{OPERATOR, BRUSH, SCENE}, factor: float, blur_steps: int, use_curve: boolean, invert: boolean)` — Creates a mask based on the curvature of the surface
- `bpy.ops.sculpt.mask_init(mode: enum{RANDOM_PER_VERTEX, RANDOM_PER_FACE_SET, RANDOM_PER_LOOSE_PART})` — Creates a new mask for the entire mesh
- `bpy.ops.sculpt.mesh_filter(start_mouse: int, area_normal_radius: float, strength: float, iteration_count: int, event_history: collection, type: enum{SMOOTH, SCALE, INFLATE, SPHERE, RANDOM, RELAX, RELAX_FACE_SETS, SURFACE_SMOOTH, SHARPEN, ENHANCE_DETAILS, ERASE_DISPLACEMENT}, deform_axis: enum{X, Y, Z}, orientation: enum{LOCAL, WORLD, VIEW}, surface_smooth_shape_preservation: float, surface_smooth_current_vertex: float, sharpen_smooth_ratio: float, sharpen_intensify_detail_strength: float, sharpen_curvature_smooth_iterations: int)` — Applies a filter to modify the current mesh
- `bpy.ops.sculpt.optimize()` — Recalculate the sculpt BVH to improve performance
- `bpy.ops.sculpt.paint_mask_extract(mask_threshold: float, add_boundary_loop: boolean, smooth_iterations: int, apply_shrinkwrap: boolean, add_solidify: boolean)` — Create a new mesh object from the current paint mask
- `bpy.ops.sculpt.paint_mask_slice(mask_threshold: float, fill_holes: boolean, new_object: boolean)` — Slices the paint mask from the mesh
- `bpy.ops.sculpt.project_line_gesture(xstart: int, xend: int, ystart: int, yend: int, flip: boolean, cursor: int, use_front_faces_only: boolean, use_limit_to_segment: boolean)` — Project the geometry onto a plane defined by a line
- `bpy.ops.sculpt.sample_detail_size(location: int, mode: enum{DYNTOPO, VOXEL})` — Sample the mesh detail on clicked point
- `bpy.ops.sculpt.sculptmode_toggle()` — Toggle sculpt mode in 3D view
- `bpy.ops.sculpt.set_persistent_base()` — Reset the copy of the mesh that is being sculpted on
- `bpy.ops.sculpt.set_pivot_position(mode: enum{ORIGIN, UNMASKED, BORDER, ACTIVE, SURFACE}, mouse_x: float, mouse_y: float)` — Sets the sculpt transform pivot position
- `bpy.ops.sculpt.symmetrize(merge_tolerance: float)` — Symmetrize the topology modifications
- `bpy.ops.sculpt.trim_box_gesture(xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean, use_front_faces_only: boolean, location: int, trim_mode: enum{DIFFERENCE, UNION, JOIN}, use_cursor_depth: boolean, trim_orientation: enum{VIEW, SURFACE}, trim_extrude_mode: enum{PROJECT, FIXED}, trim_solver: enum{EXACT, FLOAT, MANIFOLD})` — Execute a boolean operation on the mesh and a rectangle defined by the cursor
- `bpy.ops.sculpt.trim_lasso_gesture(path: collection, use_smooth_stroke: boolean, smooth_stroke_factor: float, smooth_stroke_radius: int, use_front_faces_only: boolean, location: int, trim_mode: enum{DIFFERENCE, UNION, JOIN}, use_cursor_depth: boolean, trim_orientation: enum{VIEW, SURFACE}, trim_extrude_mode: enum{PROJECT, FIXED}, trim_solver: enum{EXACT, FLOAT, MANIFOLD})` — Execute a boolean operation on the mesh and a shape defined by the cursor
- `bpy.ops.sculpt.trim_line_gesture(xstart: int, xend: int, ystart: int, yend: int, flip: boolean, cursor: int, use_front_faces_only: boolean, use_limit_to_segment: boolean, location: int, trim_mode: enum{DIFFERENCE, UNION, JOIN}, use_cursor_depth: boolean, trim_orientation: enum{VIEW, SURFACE}, trim_extrude_mode: enum{PROJECT, FIXED}, trim_solver: enum{EXACT, FLOAT, MANIFOLD})` — Remove a portion of the mesh on one side of a line
- `bpy.ops.sculpt.trim_polyline_gesture(path: collection, use_front_faces_only: boolean, location: int, trim_mode: enum{DIFFERENCE, UNION, JOIN}, use_cursor_depth: boolean, trim_orientation: enum{VIEW, SURFACE}, trim_extrude_mode: enum{PROJECT, FIXED}, trim_solver: enum{EXACT, FLOAT, MANIFOLD})` — Execute a boolean operation on the mesh and a polygonal shape defined by the cursor
- `bpy.ops.sculpt.uv_sculpt_grab(use_invert: boolean)` — Grab UVs
- `bpy.ops.sculpt.uv_sculpt_pinch(use_invert: boolean)` — Pinch UVs
- `bpy.ops.sculpt.uv_sculpt_relax(use_invert: boolean, relax_method: enum{LAPLACIAN, HC, COTAN})` — Relax UVs