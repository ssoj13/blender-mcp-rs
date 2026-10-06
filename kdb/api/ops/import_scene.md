---
area: api-ops
group: import_scene
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.import_scene

- `bpy.ops.import_scene.fbx(filepath: string, directory: string, filter_glob: string, files: collection, ui_tab: enum{MAIN, ARMATURE}, use_manual_orientation: boolean, global_scale: float, bake_space_transform: boolean, use_custom_normals: boolean, colors_type: enum{NONE, SRGB, LINEAR}, use_image_search: boolean, use_alpha_decals: boolean, decal_offset: float, use_anim: boolean, anim_offset: float, use_subsurf: boolean, use_custom_props: boolean, use_custom_props_enum_as_string: boolean, ignore_leaf_bones: boolean, force_connect_children: boolean, automatic_bone_orientation: boolean, primary_bone_axis: enum{X, Y, Z, -X, -Y, -Z}, secondary_bone_axis: enum{X, Y, Z, -X, -Y, -Z}, use_prepost_rot: boolean, mtl_name_collision_mode: enum{MAKE_UNIQUE, REFERENCE_EXISTING}, axis_forward: enum{X, Y, Z, -X, -Y, -Z}, axis_up: enum{X, Y, Z, -X, -Y, -Z})` — Load a FBX file
- `bpy.ops.import_scene.gltf(filepath: string, export_import_convert_lighting_mode: enum{SPEC, COMPAT, RAW}, filter_glob: string, directory: string, files: collection, loglevel: int, import_pack_images: boolean, merge_vertices: boolean, import_shading: enum{NORMALS, FLAT, SMOOTH}, bone_heuristic: enum{BLENDER, TEMPERANCE, FORTUNE}, disable_bone_shape: boolean, bone_shape_scale_factor: float, guess_original_bind_pose: boolean, import_webp_texture: boolean, import_unused_materials: boolean, import_select_created_objects: boolean, import_scene_extras: boolean, import_scene_as_collection: boolean, import_merge_material_slots: boolean, import_point_as_pointcloud: boolean)` — Load a glTF 2.0 file