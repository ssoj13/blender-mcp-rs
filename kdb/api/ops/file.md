---
area: api-ops
group: file
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.file

- `bpy.ops.file.autopack_toggle()` — Automatically pack all external files into the .blend file
- `bpy.ops.file.bookmark_add()` — Add a bookmark for the selected/active directory
- `bpy.ops.file.bookmark_cleanup()` — Delete all invalid bookmarks
- `bpy.ops.file.bookmark_delete(index: int)` — Delete selected bookmark
- `bpy.ops.file.bookmark_move(direction: enum{TOP, UP, DOWN, BOTTOM})` — Move the active bookmark up/down in the list
- `bpy.ops.file.cancel()` — Cancel file operation
- `bpy.ops.file.delete()` — Move selected files to the trash or recycle bin
- `bpy.ops.file.directory_new(directory: string, open: boolean, confirm: boolean)` — Create a new directory
- `bpy.ops.file.edit_directory_path()` — Start editing directory field
- `bpy.ops.file.execute()` — Execute selected file
- `bpy.ops.file.external_operation(operation: enum{OPEN, FOLDER_OPEN, EDIT, NEW, FIND, SHOW, PLAY, BROWSE, PREVIEW, PRINT, INSTALL, RUNAS, PROPERTIES, FOLDER_FIND, CMD})` — Perform external operation on a file or folder
- `bpy.ops.file.filenum(increment: int)` — Increment number in filename
- `bpy.ops.file.filepath_drop(filepath: string)` — (undocumented operator)
- `bpy.ops.file.find_missing_files(find_all: boolean, directory: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Try to find missing external files
- `bpy.ops.file.hidedot()` — Toggle hide hidden dot files
- `bpy.ops.file.highlight()` — Highlight selected file(s)
- `bpy.ops.file.make_paths_absolute()` — Make all paths to external files absolute
- `bpy.ops.file.make_paths_relative()` — Make all paths to external files relative to current .blend
- `bpy.ops.file.mouse_execute()` — Perform the current execute action for the file under the cursor (e.g. open the file)
- `bpy.ops.file.next()` — Move to next folder
- `bpy.ops.file.pack_all()` — Pack all used external files into this .blend
- `bpy.ops.file.pack_libraries()` — Store all data-blocks linked from other .blend files in the current .blend file. Library references are preserved so the linked data-blocks can be unpacked again
- `bpy.ops.file.parent()` — Move to parent directory
- `bpy.ops.file.previous()` — Move to previous folder
- `bpy.ops.file.refresh()` — Refresh the file list
- `bpy.ops.file.rename()` — Rename file or file directory
- `bpy.ops.file.report_missing_files()` — Report all missing external files
- `bpy.ops.file.reset_recent()` — Reset recent files
- `bpy.ops.file.select(wait_to_deselect_others: boolean, use_select_on_click: boolean, mouse_x: int, mouse_y: int, extend: boolean, fill: boolean, open: boolean, deselect_all: boolean, only_activate_if_selected: boolean, pass_through: boolean)` — Handle mouse clicks to select and activate items
- `bpy.ops.file.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Select or deselect all files
- `bpy.ops.file.select_bookmark(dir: string)` — Select a bookmarked directory
- `bpy.ops.file.select_box(xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean, mode: enum{SET, ADD, SUB})` — Activate/select the file(s) contained in the border
- `bpy.ops.file.select_walk(direction: enum{UP, DOWN, LEFT, RIGHT}, extend: boolean, fill: boolean)` — Select/Deselect files by walking through them
- `bpy.ops.file.smoothscroll()` — Smooth scroll to make editable file visible
- `bpy.ops.file.sort_column_ui_context()` — Change sorting to use column under cursor
- `bpy.ops.file.start_filter()` — Start entering filter text
- `bpy.ops.file.unpack_all(method: enum{USE_LOCAL, WRITE_LOCAL, USE_ORIGINAL, WRITE_ORIGINAL, KEEP, REMOVE})` — Unpack all files packed into this .blend to external ones
- `bpy.ops.file.unpack_item(method: enum{USE_LOCAL, WRITE_LOCAL, USE_ORIGINAL, WRITE_ORIGINAL}, id_name: string, id_type: int)` — Unpack this file to an external file
- `bpy.ops.file.unpack_libraries()` — Restore all packed linked data-blocks to their original locations
- `bpy.ops.file.view_selected()` — Scroll the selected files into view