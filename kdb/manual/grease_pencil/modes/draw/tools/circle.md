---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

.. _tool-grease-pencil-draw-circle:

***********
Circle Tool
***********

The Circle tool creates oval shapes using any of the *Draw* type brushes.

Tool Settings
=============

You can configure the brush main settings exposed on the Tool Settings for convenience.
For the draw brushes configuration and settings see:
Draw Brush </grease_pencil/modes/draw/brushes/draw>.

Subdivisions
   The number of stroke points between each stroke edge.

Thickness Profile
   Use a curve widget </interface/controls/templates/curve> to define the stroke thickness
   from the start (left) to end (right) of the stroke.

   Use Curve
      When enabled, the stroke use a curve profile to control the thickness along the line.

Brush Asset
-----------

Picks the brush asset used by the tool.

See /grease_pencil/modes/draw/tool_settings/brushes for more information.

See /grease_pencil/modes/draw/brushes/draw for a detailed list of all draw brushes and their options.

Brush Settings
--------------

Parameters to control to look of the stroke.

See /grease_pencil/modes/draw/brushes/draw for details.

Color
-----

Settings to determine the color of strokes.

See /grease_pencil/modes/draw/tool_settings/color

Usage
=====

Selecting a Brush and Material
------------------------------

In the Tool Settings select the brush, material and color type to use with the tool.
The Circle tool uses *Draw Brush* types.
See grease-pencil-draw-common-options for more information.

Creating Circles
----------------

#. Click (LMB or the Pen tip) and drag the start point.
#. Release on the desired end point.
#. After releasing you can move the start and end point by clicking and dragging on the yellow manipulators.
#. Then confirm (Return/MMB) or cancel (Esc/RMB).

While dragging you can use Shift to make a perfect circle
or use Alt to create the circle from a center point.

NumpadPlus and NumpadMinus or using the mouse Wheel
will increase or decrease the amount of points in the final circle.

F will adjust the line thickness and Shift-F will adjust the opacity of the strokes.

.. list-table::

   * - .. figure:: /images/grease-pencil_modes_draw_tools_circle_example-01.png
          :width: 200px

          Click and dragging the start point.

     - .. figure:: /images/grease-pencil_modes_draw_tools_circle_example-02.png
          :width: 200px

          Moving start and end points with manipulators.

     - .. figure:: /images/grease-pencil_modes_draw_tools_circle_example-03.png
          :width: 200px

          The circle after confirming.
