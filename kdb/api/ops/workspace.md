---
area: api-ops
group: workspace
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.workspace

- `bpy.ops.workspace.add()` — Add a new workspace by duplicating the current one or appending one from the user configuration
- `bpy.ops.workspace.append_activate(idname: string, filepath: string)` — Append a workspace and make it the active one in the current window
- `bpy.ops.workspace.delete()` — Delete the active workspace
- `bpy.ops.workspace.delete_all_others()` — Delete all workspaces except this one
- `bpy.ops.workspace.duplicate()` — Add a new workspace
- `bpy.ops.workspace.reorder_to_back()` — Reorder workspace to be last in the list
- `bpy.ops.workspace.reorder_to_front()` — Reorder workspace to be first in the list
- `bpy.ops.workspace.scene_pin_toggle()` — Remember the last used scene for the current workspace and switch to it whenever this workspace is activated again