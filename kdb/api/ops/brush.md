---
area: api-ops
group: brush
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.brush

- `bpy.ops.brush.asset_activate(asset_library_type: enum{ALL, LOCAL, ESSENTIALS, ONLINE_ESSENTIALS, CUSTOM}, asset_library_identifier: string, relative_asset_identifier: string, use_toggle: boolean)` — Activate a brush asset as current sculpt and paint tool
- `bpy.ops.brush.asset_delete()` — Delete the active brush asset
- `bpy.ops.brush.asset_edit_metadata(catalog_path: string, author: string, description: string)` — Edit asset information like the catalog, preview image, tags, or author
- `bpy.ops.brush.asset_load_preview(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, show_multiview: boolean, use_multiview: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Choose a preview image for the brush
- `bpy.ops.brush.asset_revert()` — Revert the active brush settings to the default values from the asset library
- `bpy.ops.brush.asset_save()` — Update the active brush asset in the asset library with current settings
- `bpy.ops.brush.asset_save_as(name: string, asset_library_reference: enum{}, catalog_path: string)` — Save a copy of the active brush asset into the default asset library, and make it the active brush
- `bpy.ops.brush.scale_size(scalar: float)` — Change brush size by a scalar
- `bpy.ops.brush.stencil_control(mode: enum{TRANSLATION, SCALE, ROTATION}, texmode: enum{PRIMARY, SECONDARY})` — Control the stencil brush
- `bpy.ops.brush.stencil_fit_image_aspect(use_repeat: boolean, use_scale: boolean, mask: boolean)` — When using an image texture, adjust the stencil size to fit the image aspect ratio
- `bpy.ops.brush.stencil_reset_transform(mask: boolean)` — Reset the stencil transformation to the default