---
area: manual-interface
source: blender-manual
version: 5.2
---

************
Input Fields
************

.. _bpy.ops.buttons.start_filter:
.. _bpy.ops.buttons.clear_filter:

Text & Search Fields
====================

Text fields show a rounded rectangular border, and optionally an icon and/or text inside the border.
Text fields store text strings, and provide the means to edit text by standard text editing shortcuts:

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - Home
     - Go to the start of the line.
   * - End
     - Go to the end of the line.
   * - Left, Right
     - Move the cursor a single character.
   * - Ctrl-Left, Ctrl-Right
     - Move the cursor an entire word.
   * - Backspace, Delete
     - Delete characters.
   * - Ctrl-Backspace, Ctrl-Delete
     - Delete words.
   * - Shift
     - Select while holding the key and moving the cursor.
   * - Ctrl-A
     - Select all text.
   * - Ctrl-C
     - Copy the selected text.
   * - Ctrl-X
     - Cut the selected text.
   * - Ctrl-V
     - Paste text at the cursor position.

For text fields with an icon and pop-ups, see ui-data-id.

Text Boxes
----------

Text boxes allow the input of multi-line text.
Unlike single-line text fields, they support line breaks and paragraph-style editing.

Text boxes can be resized by dragging the handle in the bottom-right corner of the field.
This allows more text to be visible without scrolling.

Text boxes use the same editing shortcuts as regular text fields, with additional shortcuts
for navigating and editing across multiple lines.

.. list-table::
   :width: 100%
   :widths: 20 80

   * - Shift-Enter
     - Insert a new line.
   * - Up, Down
     - Move the cursor between lines.

Text boxes are commonly used for descriptions, notes, scripts,
and other longer text content throughout Blender's interface.

Number Fields
=============

Number fields store values and units.

The first type of number field shows triangles pointing left (<) and right (>)
on the sides of the field when mouse pointer is over the field.

Sliders, a second type of number field, have a colored bar in the background
to display values over a range, e.g. percentage values.

The value can be edited in several ways:

Incremental Steps
   To change the value in unit steps, click LMB on the small triangles (not available for sliders).
   You can also use Ctrl-Wheel while hovering over the field to edit the value.
Dragging
   To change the value with the mouse, hold down LMB and drag to left or right.

   Hold Ctrl to snap to the discrete steps while dragging or Shift for precision input.
Keyboard Input
   Press LMB or Return to enter value by typing it with keyboard.

   When entering values by keyboard, number fields work like text fields:

   - Press Return or LMB outside the field to apply the change.
   - Press Esc or RMB to cancel.
   - Press Tab to jump to the next field or Shift-Tab to go to the previous field.
   - Press Minus while hovering over a number field to negate the value.

Multi-Value Editing
-------------------

You can edit multiple number fields at once by pressing down
LMB on the first field, and then dragging vertically over
the fields you want to edit. Finally you can either drag left or right to
adjust value with the mouse, or release the LMB and type in a value.

Value Limits
------------

Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
Changing values by dragging with the mouse is restricted to the "soft limit" value range.
Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

Expressions
-----------

.. Do not use mathjax here

You can enter mathematical expressions into any number field.
For example, enter `3*2` or `10/5+4` instead of `6`.
Even constants like `pi` (3.142) or functions like `sqrt(2)` (square root of 2)
may be used.

**Seealso:**

   These expressions are evaluated by Python; for all available math expressions see:
   `Math module reference <https://docs.python.org/3/library/math.html>`__.

Expressions as Drivers
^^^^^^^^^^^^^^^^^^^^^^

You may want your expression to be re-evaluated after it is entered.
Blender supports this using Drivers </animation/drivers/index> (a feature of the animation system).

Expressions beginning with `#` have a special use.
Instead of evaluating the value and discarding the expression,
a driver is added to the property with the expression entered.

The expression `#frame` is a quick way to map a value to the current frame,
but more complex expressions like `#fmod(frame, 24) / 24` are also supported.

This is simply a convenient shortcut to add drivers which can also be added via the RMB menu.

Units
-----

As well as expressions, you can specify numbers and units.
If no unit is given, then a default unit is applied.
The unit system can be changed in scene settings <bpy.types.UnitSettings>.

You can use either the unit abbreviation or the full name after the value.

Examples of valid usage of length units include:

**Note:**

   - Decimal separator is optional.
   - You can mix units, e.g. metric and imperial even though you can only show one at a time.
   - Plurals of the names are recognized too, so `meter` and `meters` can both be used.

Color Fields
============

The color field stores a color value.
Clicking on it with LMB opens the /interface/controls/templates/color_picker.

Color fields with an alpha channel are divided in half: on the left, the color is shown without an alpha channel,
and on the right, it's shown with an alpha channel over a checker pattern.

Colors can be copied to other color fields by dragging and dropping.

Hovering over a color property will display a large swatch preview of the color
and the color's hexadecimal, RGBA, and HSVA values.
