---
area: api-ops
group: info
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.info

- `bpy.ops.info.report_copy()` — Copy selected reports to clipboard
- `bpy.ops.info.report_delete()` — Delete selected reports
- `bpy.ops.info.report_replay()` — Replay selected reports
- `bpy.ops.info.reports_display_update()` — Update the display of reports in Blender UI (internal use)
- `bpy.ops.info.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Change selection of all visible reports
- `bpy.ops.info.select_box(xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean, mode: enum{SET, ADD, SUB})` — Toggle box selection
- `bpy.ops.info.select_pick(report_index: int, extend: boolean)` — Select reports by index