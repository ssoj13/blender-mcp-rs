---
area: api-ops
group: scene
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.scene

- `bpy.ops.scene.delete()` — Delete active scene
- `bpy.ops.scene.drop_scene_asset(session_uid: int)` — Import scene and set it as the active one in the window
- `bpy.ops.scene.freestyle_add_edge_marks_to_keying_set()` — Add the data paths to the Freestyle Edge Mark property of selected edges to the active keying set
- `bpy.ops.scene.freestyle_add_face_marks_to_keying_set()` — Add the data paths to the Freestyle Face Mark property of selected polygons to the active keying set
- `bpy.ops.scene.freestyle_alpha_modifier_add(type: enum{ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT})` — Add an alpha transparency modifier to the line style associated with the active lineset
- `bpy.ops.scene.freestyle_color_modifier_add(type: enum{ALONG_STROKE, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT})` — Add a line color modifier to the line style associated with the active lineset
- `bpy.ops.scene.freestyle_fill_range_by_selection(type: enum{COLOR, ALPHA, THICKNESS}, name: string)` — Fill the Range Min/Max entries by the min/max distance between selected mesh objects and the source object (either a user-specified object or the active camera)
- `bpy.ops.scene.freestyle_geometry_modifier_add(type: enum{2D_OFFSET, 2D_TRANSFORM, BACKBONE_STRETCHER, BEZIER_CURVE, BLUEPRINT, GUIDING_LINES, PERLIN_NOISE_1D, PERLIN_NOISE_2D, POLYGONIZATION, SAMPLING, SIMPLIFICATION, SINUS_DISPLACEMENT, SPATIAL_NOISE, TIP_REMOVER})` — Add a stroke geometry modifier to the line style associated with the active lineset
- `bpy.ops.scene.freestyle_lineset_add()` — Add a line set into the list of line sets
- `bpy.ops.scene.freestyle_lineset_copy()` — Copy the active line set to the internal clipboard
- `bpy.ops.scene.freestyle_lineset_move(direction: enum{UP, DOWN})` — Change the position of the active line set within the list of line sets
- `bpy.ops.scene.freestyle_lineset_paste()` — Paste the internal clipboard content to the active line set
- `bpy.ops.scene.freestyle_lineset_remove()` — Remove the active line set from the list of line sets
- `bpy.ops.scene.freestyle_linestyle_new()` — Create a new line style, reusable by multiple line sets
- `bpy.ops.scene.freestyle_modifier_copy()` — Duplicate the modifier within the list of modifiers
- `bpy.ops.scene.freestyle_modifier_move(direction: enum{UP, DOWN})` — Move the modifier within the list of modifiers
- `bpy.ops.scene.freestyle_modifier_remove()` — Remove the modifier from the list of modifiers
- `bpy.ops.scene.freestyle_module_add()` — Add a style module into the list of modules
- `bpy.ops.scene.freestyle_module_move(direction: enum{UP, DOWN})` — Change the position of the style module within in the list of style modules
- `bpy.ops.scene.freestyle_module_open(filepath: string, make_internal: boolean)` — Open a style module file
- `bpy.ops.scene.freestyle_module_remove()` — Remove the style module from the stack
- `bpy.ops.scene.freestyle_stroke_material_create()` — Create Freestyle stroke material for testing
- `bpy.ops.scene.freestyle_thickness_modifier_add(type: enum{ALONG_STROKE, CALLIGRAPHY, CREASE_ANGLE, CURVATURE_3D, DISTANCE_FROM_CAMERA, DISTANCE_FROM_OBJECT, MATERIAL, NOISE, TANGENT})` — Add a line thickness modifier to the line style associated with the active lineset
- `bpy.ops.scene.gltf2_action_filter_refresh()` — Refresh list of actions
- `bpy.ops.scene.gpencil_brush_preset_add(name: string, remove_name: boolean, remove_active: boolean)` — Add or remove Grease Pencil brush preset
- `bpy.ops.scene.gpencil_material_preset_add(name: string, remove_name: boolean, remove_active: boolean)` — Add or remove Grease Pencil material preset
- `bpy.ops.scene.new(type: enum{NEW, EMPTY, LINK_COPY, FULL_COPY})` — Add new scene by type
- `bpy.ops.scene.new_sequencer(type: enum{NEW, EMPTY, LINK_COPY, FULL_COPY})` — Add new scene by type in the sequence editor and assign to active strip
- `bpy.ops.scene.new_sequencer_scene(type: enum{NEW, EMPTY, LINK_COPY, FULL_COPY})` — Add new scene to be used by the sequencer
- `bpy.ops.scene.render_view_add()` — Add a render view
- `bpy.ops.scene.render_view_remove()` — Remove the selected render view
- `bpy.ops.scene.view_layer_add(type: enum{NEW, COPY, EMPTY})` — Add a view layer
- `bpy.ops.scene.view_layer_add_aov()` — Add a Shader AOV
- `bpy.ops.scene.view_layer_add_lightgroup(name: string)` — Add a Light Group
- `bpy.ops.scene.view_layer_add_used_lightgroups()` — Add all used Light Groups
- `bpy.ops.scene.view_layer_remove()` — Remove the selected view layer
- `bpy.ops.scene.view_layer_remove_aov()` — Remove Active AOV
- `bpy.ops.scene.view_layer_remove_lightgroup()` — Remove Active Lightgroup
- `bpy.ops.scene.view_layer_remove_unused_lightgroups()` — Remove all unused Light Groups