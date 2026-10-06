---
area: api-ops
group: font
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.font

- `bpy.ops.font.case_set(case: enum{LOWER, UPPER})` — Set font case
- `bpy.ops.font.case_toggle()` — Toggle font case
- `bpy.ops.font.change_character(delta: int)` — Change font character code
- `bpy.ops.font.change_spacing(delta: float)` — Change font spacing
- `bpy.ops.font.delete(type: enum{NEXT_CHARACTER, PREVIOUS_CHARACTER, NEXT_WORD, PREVIOUS_WORD, SELECTION, NEXT_OR_SELECTION, PREVIOUS_OR_SELECTION})` — Delete text by cursor position
- `bpy.ops.font.line_break()` — Insert line break at cursor position
- `bpy.ops.font.move(type: enum{LINE_BEGIN, LINE_END, TEXT_BEGIN, TEXT_END, PREVIOUS_CHARACTER, NEXT_CHARACTER, PREVIOUS_WORD, NEXT_WORD, PREVIOUS_LINE, NEXT_LINE, PREVIOUS_PAGE, NEXT_PAGE})` — Move cursor to position type
- `bpy.ops.font.move_select(type: enum{LINE_BEGIN, LINE_END, TEXT_BEGIN, TEXT_END, PREVIOUS_CHARACTER, NEXT_CHARACTER, PREVIOUS_WORD, NEXT_WORD, PREVIOUS_LINE, NEXT_LINE, PREVIOUS_PAGE, NEXT_PAGE})` — Move the cursor while selecting
- `bpy.ops.font.open(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, relative_path: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Load a new font from a file
- `bpy.ops.font.select_all()` — Select all text
- `bpy.ops.font.select_word()` — Select word under cursor
- `bpy.ops.font.selection_set()` — Set cursor selection
- `bpy.ops.font.style_set(style: enum{BOLD, ITALIC, UNDERLINE, SMALL_CAPS}, clear: boolean)` — Set font style
- `bpy.ops.font.style_toggle(style: enum{BOLD, ITALIC, UNDERLINE, SMALL_CAPS})` — Toggle font style
- `bpy.ops.font.text_copy()` — Copy selected text to clipboard
- `bpy.ops.font.text_cut()` — Cut selected text to clipboard
- `bpy.ops.font.text_insert(text: string, accent: boolean)` — Insert text at cursor position
- `bpy.ops.font.text_insert_unicode()` — Insert Unicode Character
- `bpy.ops.font.text_paste(selection: boolean)` — Paste text from clipboard
- `bpy.ops.font.text_paste_from_file(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Paste contents from file
- `bpy.ops.font.textbox_add()` — Add a new text box
- `bpy.ops.font.textbox_remove(index: int)` — Remove the text box
- `bpy.ops.font.unlink()` — Unlink active font data-block