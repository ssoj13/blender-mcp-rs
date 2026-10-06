---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

************
Introduction
************

Draw Mode is the mode in Grease Pencil that allows you to draw in the 3D Viewport.
This mode is actually the only one in which new strokes can be created.

Already made strokes can not be selected in Draw Mode, for editing strokes you must use
the Edit Mode </grease_pencil/modes/edit/introduction> or
Sculpt Mode </grease_pencil/modes/sculpting/introduction>.

Draw Mode is selected with the *Mode* menu in the 3D Viewport header.
Once Draw Mode is activated, the Toolbar of the 3D Viewport will change to Draw Mode specific panels.
Also a circle with the same color as the active material will appear and
follow the location of the cursor in the 3D Viewport.

To create new strokes you have to select one of the drawing tools in the Toolbar.
The most common one is the Draw tool </grease_pencil/modes/draw/tools>
for free-hand drawings but there are many other tools for drawing, filling areas and erasing strokes.
There are also some tools to create primitives shapes like lines, arcs, curves, boxes and circles.

See Toolbar </grease_pencil/modes/draw/tools> for more details.

Strokes Location & Orientation Controls
=======================================

Drawing in a 3D space is not the same as drawing on a flat canvas.
When drawing with Grease Pencil you have to define
the location and orientation of the new strokes in the 3D space.

Stroke Placement
----------------

The Stroke Placement selector defines the new strokes location in 3D space.

See Stroke Placement </grease_pencil/modes/draw/stroke_placement> for more information.

Drawing Planes
--------------

The Drawing Planes selector defines the plane (orientation) to which the new strokes will be restricted.

See Drawing Planes </grease_pencil/modes/draw/drawing_planes> for more information.

Drawing Options
===============

Multiframe
   Allows to draw on several frames at the same time.

   See Multiframe </grease_pencil/multiframe> for more information.

.. _bpy.types.ToolSettings.use_gpencil_draw_additive:

Additive Drawing
   When creating new frames adding strokes with drawing tools,
   the strokes from the previous/active frame are include as a basis for the new one.
   When erasing existing strokes using Additive Drawing a new keyframe will be added.

.. _bpy.types.ToolSettings.use_gpencil_automerge_strokes:

AutoMerge
   Joins new strokes with the beginning or end of previously drawn strokes in the active layer.

.. _bpy.types.ToolSettings.use_gpencil_weight_data_add:

Add Weight Data
   When enabled, weight data is added to new strokes according to the current vertex group and weight.
   If there is no vertex group selected, no weight data is added.

   Useful for example in cut-out animation for adding new drawing
   on the same vertex group without the need to creating it afterwards.

   See Weight Paint Mode </grease_pencil/modes/weight_paint/introduction> for more information.

.. _bpy.types.ToolSettings.use_gpencil_draw_onback:

Draw on Back
   When enabled, new strokes are drawn below all strokes in the layer.
   For example when you want to paint with a fill material below line strokes on a character and
   they are on the same layer.
