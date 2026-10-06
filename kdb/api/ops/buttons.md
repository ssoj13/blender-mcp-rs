---
area: api-ops
group: buttons
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.buttons

- `bpy.ops.buttons.clear_filter()` — Clear the search filter
- `bpy.ops.buttons.context_menu()` — Display properties editor context_menu
- `bpy.ops.buttons.directory_browse(directory: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, relative_path: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Open a directory browser, hold Shift to open the file, Alt to browse containing directory
- `bpy.ops.buttons.file_browse(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, relative_path: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{}, filter_glob: string)` — Open a file browser, hold Shift to open the file, Alt to browse containing directory
- `bpy.ops.buttons.start_filter()` — Start entering filter text
- `bpy.ops.buttons.toggle_pin()` — Keep the current data-block displayed