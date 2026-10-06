---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

**********
Brush Tool
**********

Tool to free form draw Grease Pencil strokes using any of the *Draw* type brushes.

**Tip:**

   Activating a brush asset from an asset shelf or brush selector will also activate this tool for convenience.

Tool Settings
=============

Brush Asset
-----------

Picks the brush asset used by the tool.

See /grease_pencil/modes/draw/tool_settings/brushes for more information.

See /grease_pencil/modes/draw/brushes/draw for a detailed list of all draw brushes and their options.

Brush Settings
--------------

Parameters to control to look of the stroke.

See /grease_pencil/modes/draw/brushes/draw for details.

Eraser
------

.. _bpy.types.Paint.eraser_brush:

Default Eraser Brush
   Select a brush to use as eraser for quickly alternating with the main brush using
   Ctrl-LMB.

Color
-----

Settings to determine the color of strokes.

See /grease_pencil/modes/draw/tool_settings/color

Usage
=====

Selecting a Brush and Material
------------------------------

In the Tool Settings select the brush, material and color type to use with the tool.
The Draw tool uses *Draw Brush* types.
See grease-pencil-draw-common-options for more information.

Free-hand Drawing
-----------------

Click and hold LMB or use the pen tip to make free-hand drawing on the viewport.

.. list-table:: Drawing free-hand strokes.

   * - .. figure:: /images/grease-pencil_modes_draw_tools_draw_free-hand-01.png
          :width: 200px

     - .. figure:: /images/grease-pencil_modes_draw_tools_draw_free-hand-02.png
          :width: 200px

     - .. figure:: /images/grease-pencil_modes_draw_tools_draw_free-hand-03.png
          :width: 200px

Stabilize Stroke
----------------

Shift-LMB toggle the use of Stabilize Stroke <bpy.types.BrushGpencilSettings.use_settings_stabilizer>
on the brush to have more control while drawing and get smoother lines.

.. list-table:: Drawing strokes using *Stabilize Stroke*.

   * - .. figure:: /images/grease-pencil_modes_draw_tools_draw_stabilizer-01.png

     - .. figure:: /images/grease-pencil_modes_draw_tools_draw_stabilizer-02.png

     - .. figure:: /images/grease-pencil_modes_draw_tools_draw_stabilizer-03.png

Straight Lines
--------------

Alt-LMB Constrains the drawing of the strokes to horizontal or vertical straight lines.

Switching to the Erase Tool
---------------------------

Ctrl-LMB changes temporally to the active Erase tool.
See Erase Tool </grease_pencil/modes/draw/tools/erase> for more information.

You can also use B to delete all the points in the selected drawing area.
