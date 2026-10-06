---
area: api-ops
group: gpencil
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.gpencil

- `bpy.ops.gpencil.annotate(mode: enum{DRAW, DRAW_STRAIGHT, DRAW_POLY, ERASER}, arrowstyle_start: enum{NONE, ARROW, ARROW_OPEN, ARROW_OPEN_INVERTED, DIAMOND}, arrowstyle_end: enum{NONE, ARROW, ARROW_OPEN, ARROW_OPEN_INVERTED, DIAMOND}, use_stabilizer: boolean, stabilizer_factor: float, stabilizer_radius: int, stroke: collection, wait_for_input: boolean)` — Make annotations on the active data
- `bpy.ops.gpencil.annotation_active_frame_delete()` — Delete the active frame for the active Annotation Layer
- `bpy.ops.gpencil.annotation_add()` — Add new Annotation data-block
- `bpy.ops.gpencil.data_unlink()` — Unlink active Annotation data-block
- `bpy.ops.gpencil.layer_annotation_add()` — Add new Annotation layer or note for the active data-block
- `bpy.ops.gpencil.layer_annotation_move(type: enum{UP, DOWN})` — Move the active Annotation layer up/down in the list
- `bpy.ops.gpencil.layer_annotation_remove()` — Remove active Annotation layer