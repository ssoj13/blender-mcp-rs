---
area: api-ops
group: view2d
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.view2d

- `bpy.ops.view2d.edge_pan(inside_padding: float, outside_padding: float, speed_ramp: float, max_speed: float, delay: float, zoom_influence: float)` — Pan the view when the mouse is held at an edge
- `bpy.ops.view2d.ndof()` — Use a 3D mouse device to pan/zoom the view
- `bpy.ops.view2d.pan(deltax: int, deltay: int)` — Pan the view
- `bpy.ops.view2d.reset()` — Reset the view
- `bpy.ops.view2d.scroll_down(deltax: int, deltay: int, page: boolean)` — Scroll the view down
- `bpy.ops.view2d.scroll_left(deltax: int, deltay: int)` — Scroll the view left
- `bpy.ops.view2d.scroll_right(deltax: int, deltay: int)` — Scroll the view right
- `bpy.ops.view2d.scroll_up(deltax: int, deltay: int, page: boolean)` — Scroll the view up
- `bpy.ops.view2d.scroller_activate()` — Scroll view by mouse click and drag
- `bpy.ops.view2d.smoothview(xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean)` — (undocumented operator)
- `bpy.ops.view2d.zoom(deltax: float, deltay: float, use_cursor_init: boolean)` — Zoom in/out the view
- `bpy.ops.view2d.zoom_border(xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean, zoom_out: boolean)` — Zoom in the view to the nearest item contained in the border
- `bpy.ops.view2d.zoom_in(zoomfacx: float, zoomfacy: float)` — Zoom in the view
- `bpy.ops.view2d.zoom_out(zoomfacx: float, zoomfacy: float)` — Zoom out the view