---
area: api-ops
group: text
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.text

- `bpy.ops.text.autocomplete()` — Show a list of used text in the open document
- `bpy.ops.text.comment_toggle(type: enum{TOGGLE, COMMENT, UNCOMMENT})` — (undocumented operator)
- `bpy.ops.text.convert_whitespace(type: enum{SPACES, TABS})` — Convert whitespaces by type
- `bpy.ops.text.copy()` — Copy selected text to clipboard
- `bpy.ops.text.cursor_set(x: int, y: int)` — Set cursor position
- `bpy.ops.text.cut()` — Cut selected text to clipboard
- `bpy.ops.text.delete(type: enum{NEXT_CHARACTER, PREVIOUS_CHARACTER, NEXT_WORD, PREVIOUS_WORD})` — Delete text by cursor position
- `bpy.ops.text.duplicate_line()` — Duplicate the current line
- `bpy.ops.text.find()` — Find specified text
- `bpy.ops.text.find_set_selected()` — Find specified text and set as selected
- `bpy.ops.text.indent()` — Indent selected text
- `bpy.ops.text.indent_or_autocomplete()` — Indent selected text or autocomplete
- `bpy.ops.text.insert(text: string)` — Insert text at cursor position
- `bpy.ops.text.jump(line: int)` — Jump cursor to line
- `bpy.ops.text.jump_to_file_at_point(filepath: string, line: int, column: int)` — Jump to a file for the text editor
- `bpy.ops.text.line_break()` — Insert line break at cursor position
- `bpy.ops.text.line_number()` — The current line number
- `bpy.ops.text.make_internal()` — Make active text file internal
- `bpy.ops.text.move(type: enum{LINE_BEGIN, LINE_END, FILE_TOP, FILE_BOTTOM, PREVIOUS_CHARACTER, NEXT_CHARACTER, PREVIOUS_WORD, NEXT_WORD, PREVIOUS_LINE, NEXT_LINE, PREVIOUS_PAGE, NEXT_PAGE})` — Move cursor to position type
- `bpy.ops.text.move_lines(direction: enum{UP, DOWN})` — Move the currently selected line(s) up/down
- `bpy.ops.text.move_select(type: enum{LINE_BEGIN, LINE_END, FILE_TOP, FILE_BOTTOM, PREVIOUS_CHARACTER, NEXT_CHARACTER, PREVIOUS_WORD, NEXT_WORD, PREVIOUS_LINE, NEXT_LINE, PREVIOUS_PAGE, NEXT_PAGE})` — Move the cursor while selecting
- `bpy.ops.text.new()` — Create a new text data-block
- `bpy.ops.text.open(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, relative_path: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{DEFAULT, FILE_SORT_ALPHA, FILE_SORT_EXTENSION, FILE_SORT_TIME, FILE_SORT_SIZE, ASSET_CATALOG}, internal: boolean)` — Open a new text data-block
- `bpy.ops.text.overwrite_toggle()` — Toggle overwrite while typing
- `bpy.ops.text.paste(selection: boolean)` — Paste text from clipboard
- `bpy.ops.text.reload()` — Reload active text data-block from its file
- `bpy.ops.text.replace(all: boolean)` — Replace text with the specified text
- `bpy.ops.text.replace_set_selected()` — Replace text with specified text and set as selected
- `bpy.ops.text.resolve_conflict(resolution: enum{IGNORE, RELOAD, SAVE, MAKE_INTERNAL})` — When external text is out of sync, resolve the conflict
- `bpy.ops.text.run_script()` — Run active script
- `bpy.ops.text.save()` — Save active text data-block
- `bpy.ops.text.save_as(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Save active text file with options
- `bpy.ops.text.scroll(lines: int)` — (undocumented operator)
- `bpy.ops.text.scroll_bar(lines: int)` — (undocumented operator)
- `bpy.ops.text.select_all()` — Select all text
- `bpy.ops.text.select_line()` — Select text by line
- `bpy.ops.text.select_word()` — Select word under cursor
- `bpy.ops.text.selection_set()` — Set text selection
- `bpy.ops.text.start_find()` — Start searching text
- `bpy.ops.text.to_3d_object(split_lines: boolean)` — Create 3D text object from active text data-block
- `bpy.ops.text.unindent()` — Unindent selected text
- `bpy.ops.text.unlink()` — Unlink active text data-block
- `bpy.ops.text.update_shader()` — Update users of this shader, such as custom cameras and script nodes, with its new sockets and options