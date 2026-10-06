---
area: manual-compositing
source: blender-manual
version: 5.2
---

.. _bpy.types.CompositorNodeRGB:
.. Editor's Note: This page gets copied into </render/cycles/nodes/types/input/rgb>

.. --- copy below this line ---

**********
Color Node
**********

The *Color* node outputs the color value chosen with the color picker widget.

**Tip:**

   Dragging colors from a color picker button into a node editor creates a Color node.
   Alpha values are preserved, if the source color has no alpha, a value of 1.0 is used.

Inputs
======

This node has no input sockets.

Properties
==========

The Color node uses the color picker widget <ui-color-picker>.

Outputs
=======

Color
   Outputs the chosen color as a single constant value.
