---
area: api-ops
group: console
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.console

- `bpy.ops.console.autocomplete()` — Evaluate the namespace up until the cursor and give a list of options or complete the name if there is only one
- `bpy.ops.console.banner()` — Print a message when the terminal initializes
- `bpy.ops.console.clear(scrollback: boolean, history: boolean)` — Clear text by type
- `bpy.ops.console.clear_line()` — Clear the line and store in history
- `bpy.ops.console.copy(delete: boolean)` — Copy selected text to clipboard
- `bpy.ops.console.copy_as_script()` — Copy the console contents for use in a script
- `bpy.ops.console.delete(type: enum{NEXT_CHARACTER, PREVIOUS_CHARACTER, NEXT_WORD, PREVIOUS_WORD})` — Delete text by cursor position
- `bpy.ops.console.execute(interactive: boolean)` — Execute the current console line as a Python expression
- `bpy.ops.console.history_append(text: string, current_character: int, remove_duplicates: boolean)` — Append history at cursor position
- `bpy.ops.console.history_cycle(reverse: boolean)` — Cycle through history
- `bpy.ops.console.indent()` — Add 4 spaces at line beginning
- `bpy.ops.console.indent_or_autocomplete()` — Indent selected text or autocomplete
- `bpy.ops.console.insert(text: string)` — Insert text at cursor position
- `bpy.ops.console.language(language: string)` — Set the current language for this console
- `bpy.ops.console.move(type: enum{LINE_BEGIN, LINE_END, PREVIOUS_CHARACTER, NEXT_CHARACTER, PREVIOUS_WORD, NEXT_WORD}, select: boolean)` — Move cursor position
- `bpy.ops.console.paste(selection: boolean)` — Paste text from clipboard
- `bpy.ops.console.scrollback_append(text: string, type: enum{OUTPUT, INPUT, INFO, ERROR})` — Append scrollback text by type
- `bpy.ops.console.select_all()` — Select all the text
- `bpy.ops.console.select_set()` — Set the console selection
- `bpy.ops.console.select_word()` — Select word at cursor position
- `bpy.ops.console.unindent()` — Delete 4 spaces from line beginning