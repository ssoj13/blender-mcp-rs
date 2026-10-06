---
area: api-ops
group: uilist
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.uilist

- `bpy.ops.uilist.entry_add(list_path: string, active_index_path: string)` — Add an entry to the list after the current active item
- `bpy.ops.uilist.entry_move(list_path: string, active_index_path: string, direction: enum{UP, DOWN})` — Move an entry in the list up or down
- `bpy.ops.uilist.entry_remove(list_path: string, active_index_path: string)` — Remove the selected entry from the list