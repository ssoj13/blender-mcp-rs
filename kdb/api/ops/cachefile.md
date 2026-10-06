---
area: api-ops
group: cachefile
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.cachefile

- `bpy.ops.cachefile.layer_add(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, relative_path: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Add an override layer to the archive
- `bpy.ops.cachefile.layer_move(direction: enum{UP, DOWN})` — Move layer in the list, layers further down the list will overwrite data from the layers higher up
- `bpy.ops.cachefile.layer_remove()` — Remove an override layer from the archive
- `bpy.ops.cachefile.open(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, relative_path: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Load a cache file
- `bpy.ops.cachefile.reload()` — Update objects paths list with new data from the archive