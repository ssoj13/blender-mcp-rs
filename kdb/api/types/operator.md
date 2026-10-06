---
area: api-types
group: operator
version: 5.2.2
---

# Blender 5.2.2 RNA types: operator

## Operator

Storage of an operator being executed, or registered after execution

- `name` · string · read-only
- `properties` · pointer · -> OperatorProperties · read-only
- `has_reports` · boolean · read-only — Operator has a set of reports (warnings and errors) from last execution
- `bl_idname` · string
- `bl_label` · string
- `bl_translation_context` · string
- `bl_description` · string
- `bl_undo_group` · string
- `bl_options` · enum · {REGISTER, UNDO, UNDO_GROUPED, BLOCKING, MACRO, GRAB_CURSOR, GRAB_CURSOR_X, GRAB_CURSOR_Y, DEPENDS_ON_CURSOR, PRESET, INTERNAL, MODAL_PRIORITY} — Options for this operator type
- `bl_cursor_pending` · enum · {DEFAULT, NONE, WAIT, CROSSHAIR, MOVE_X, MOVE_Y, KNIFE, TEXT, PAINT_BRUSH, PAINT_CROSS, DOT, ERASER, HAND, HAND_POINT, HAND_CLOSED, SCROLL_X, SCROLL_Y, SCROLL_XY, EYEDROPPER, PICK_AREA, STOP, COPY, CROSS, MUTE, ZOOM_IN, ZOOM_OUT} — Cursor to use when waiting for the user to select a location to activate the operator (when ``bl_options`` has ``DEPENDS_ON_CURSOR`` set)
- `layout` · pointer · -> UILayout · read-only
- `options` · pointer · -> OperatorOptions · read-only — Runtime options
- `macros` · collection · read-only

## OperatorMacro

Storage of a sub operator in a macro after it has been added

- `properties` · pointer · -> OperatorProperties · read-only

## OperatorMousePath

Mouse path values for operators that record such paths

- `loc` · float · [2] — Mouse location
- `time` · float — Time of mouse location

## OperatorOptions

Runtime options

- `is_grab_cursor` · boolean · read-only — True when the cursor is grabbed
- `is_invoke` · boolean · read-only — True when invoked (even if only the execute callbacks available)
- `is_repeat` · boolean · read-only — True when run from the 'Adjust Last Operation' panel
- `is_repeat_last` · boolean · read-only — True when run from the operator 'Repeat Last'
- `use_cursor_region` · boolean — Enable to use the region under the cursor for modal execution

## OperatorStrokeElement

- `location` · float · [3]
- `mouse` · float · [2]
- `mouse_event` · float · [2]
- `pressure` · float — Tablet pressure
- `size` · float — Brush size in screen space
- `x_tilt` · float — Pen tilt from left (-1.0) to right (+1.0)
- `y_tilt` · float — Pen tilt from backward (-1.0) to forward (+1.0)
- `time` · float
- `is_start` · boolean
