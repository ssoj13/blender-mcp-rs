---
area: api-ops
group: ed
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.ed

- `bpy.ops.ed.flush_edits()` — Flush edit data from active editing modes
- `bpy.ops.ed.lib_id_fake_user_toggle()` — Save this data-block even if it has no users
- `bpy.ops.ed.lib_id_generate_preview()` — Create an automatic preview for the selected data-block
- `bpy.ops.ed.lib_id_generate_preview_from_object()` — Create a preview for this asset by rendering the active object
- `bpy.ops.ed.lib_id_load_custom_preview(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, show_multiview: boolean, use_multiview: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Choose an image to help identify the data-block visually
- `bpy.ops.ed.lib_id_override_editable_toggle()` — Set if this library override data-block can be edited
- `bpy.ops.ed.lib_id_remove_preview()` — Remove the preview of this data-block
- `bpy.ops.ed.lib_id_unlink()` — Remove a usage of a data-block, clearing the assignment
- `bpy.ops.ed.redo()` — Redo previous action
- `bpy.ops.ed.undo()` — Undo previous action
- `bpy.ops.ed.undo_history(item: int)` — Undo or redo specific action in history
- `bpy.ops.ed.undo_push(message: string)` — Add an undo state (internal use only)
- `bpy.ops.ed.undo_redo()` — Undo and redo previous action