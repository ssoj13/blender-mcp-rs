---
area: api-ops
group: marker
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.marker

- `bpy.ops.marker.add()` — Add a new time marker
- `bpy.ops.marker.camera_bind()` — Bind the selected camera to a marker on the current frame
- `bpy.ops.marker.delete(confirm: boolean)` — Delete selected time marker(s)
- `bpy.ops.marker.duplicate(frames: int)` — Duplicate selected time marker(s)
- `bpy.ops.marker.make_links_scene(scene: enum{})` — Copy selected markers to another scene
- `bpy.ops.marker.move(frames: int, tweak: boolean)` — Move selected time marker(s)
- `bpy.ops.marker.rename(name: string)` — Rename first selected time marker
- `bpy.ops.marker.select(wait_to_deselect_others: boolean, use_select_on_click: boolean, mouse_x: int, mouse_y: int, extend: boolean, camera: boolean)` — Select time marker(s)
- `bpy.ops.marker.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Change selection of all time markers
- `bpy.ops.marker.select_box(xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean, mode: enum{SET, ADD, SUB}, tweak: boolean)` — Select all time markers using box selection
- `bpy.ops.marker.select_leftright(mode: enum{LEFT, RIGHT, CLICK_SIDE}, extend: boolean)` — Select markers on and left/right of the current frame