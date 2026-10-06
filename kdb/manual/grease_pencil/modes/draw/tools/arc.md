---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

.. _tool-grease-pencil-draw-arc:

********
Arc Tool
********

The Arc tool creates simple arcs using any of the *Draw* type brushes.

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
      When enabled, the stroke use a curve profile to control the thickness along the arc.

.. list-table:: Different thickness profile samples.

   * - .. figure:: /images/grease-pencil_modes_draw_tools_arc_thickness-profile-01.png
          :width: 200px

     - .. figure:: /images/grease-pencil_modes_draw_tools_arc_thickness-profile-02.png
          :width: 200px

     - .. figure:: /images/grease-pencil_modes_draw_tools_arc_thickness-profile-03.png
          :width: 200px

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
The Arc tool uses *Draw Brush* types.
See grease-pencil-draw-common-options for more information.

Creating Arcs
-------------

#. Click (LMB or the Pen tip) and drag the start point.
#. Release on the desired end point.
#. After releasing you can tweak the arc using a single cyan manipulator (hand icon).
#. Then confirm (Return/MMB) or cancel (Esc/RMB).

While dragging you can use Shift to make a perfect arc,
use Alt to create the arc from a center point or M to flip.

NumpadPlus and NumpadMinus or using the mouse Wheel
will increase or decrease the amount of points in the final arc.

F will adjust the line thickness and Shift-F will adjust the opacity of the strokes.

.. list-table::

   * - .. figure:: /images/grease-pencil_modes_draw_tools_arc_example-01.png
          :width: 200px

          click and dragging the start point.

     - .. figure:: /images/grease-pencil_modes_draw_tools_arc_example-02.png
          :width: 200px

          Tweaking arc with the manipulator.

     - .. figure:: /images/grease-pencil_modes_draw_tools_arc_example-03.png
          :width: 200px

          The arc after confirming.

Extruding
---------

Before confirming you can use E to extrude the end point of the arc
to generate multiple connected arcs.

.. list-table::

   * - .. figure:: /images/grease-pencil_modes_draw_tools_arc_extrude-01.png
          :width: 200px

          End point extruding.

     - .. figure:: /images/grease-pencil_modes_draw_tools_arc_extrude-02.png
          :width: 200px

          Tweaking the last arc with the manipulator.

     - .. figure:: /images/grease-pencil_modes_draw_tools_arc_extrude-03.png
          :width: 200px

          The connected arcs after confirming.
