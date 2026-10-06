---
area: api-ops
group: asset
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.asset

- `bpy.ops.asset.asset_download(asset_library_type: enum{ALL, LOCAL, ESSENTIALS, ONLINE_ESSENTIALS, CUSTOM}, asset_library_identifier: string, relative_asset_identifier: string)` — Make the asset available without internet access
- `bpy.ops.asset.assets_download()` — Download the selected asset(s)
- `bpy.ops.asset.assign_action()` — Set this pose Action as active Action on the active Object
- `bpy.ops.asset.browse_containing_blend_file()` — Open the system's file browser with the blend file that contains the active asset
- `bpy.ops.asset.bundle_install(asset_library_reference: enum{}, filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Copy the current .blend file into an Asset Library. Only works on standalone .blend files (i.e. when no other files are referenced)
- `bpy.ops.asset.catalog_delete(catalog_id: string)` — Remove an asset catalog from the asset library (contained assets will not be affected and show up as unassigned)
- `bpy.ops.asset.catalog_new(parent_path: string)` — Create a new catalog to put assets in
- `bpy.ops.asset.catalog_redo()` — Redo the last undone edit to the asset catalogs
- `bpy.ops.asset.catalog_undo()` — Undo the last edit to the asset catalogs
- `bpy.ops.asset.catalog_undo_push()` — Store the current state of the asset catalogs in the undo buffer
- `bpy.ops.asset.catalogs_save()` — Make any edits to any catalogs permanent by writing the current set up to the asset library
- `bpy.ops.asset.clear(set_fake_user: boolean)` — Delete all asset metadata and turn the selected asset data-blocks back into normal data-blocks
- `bpy.ops.asset.clear_single(set_fake_user: boolean)` — Delete all asset metadata and turn the asset data-block back into a normal data-block
- `bpy.ops.asset.library_refresh(use_remote_listing: boolean, use_shift_for_remote_listing: boolean)` — Reread assets and asset catalogs from the asset library on disk
- `bpy.ops.asset.library_reload_listing()` — Re-download the asset listing of a remote library. Only supported when the active asset library is remote or has a remote component (the Essentials library)
- `bpy.ops.asset.mark()` — Enable easier reuse of selected data-blocks through the Asset Browser, with the help of customizable metadata (like previews, descriptions and tags)
- `bpy.ops.asset.mark_single()` — Enable easier reuse of a data-block through the Asset Browser, with the help of customizable metadata (like previews, descriptions and tags)
- `bpy.ops.asset.open_containing_blend_file()` — Open the blend file that contains the active asset
- `bpy.ops.asset.screenshot_preview(p1: int, p2: int, force_square: boolean)` — Capture a screenshot to use as a preview for the selected asset
- `bpy.ops.asset.tag_add()` — Add a new keyword tag to the active asset
- `bpy.ops.asset.tag_remove()` — Remove an existing keyword tag from the active asset