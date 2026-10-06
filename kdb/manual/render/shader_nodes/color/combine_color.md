---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeCombineColor:

******************
Combine Color Node
******************

Combines three color channels into one color,
based on a particular Color Model.

Inputs
======

The inputs of this node depend on the Mode property (see below).

Properties
==========

Mode
   The color model to use.

   :RGB: Red, Green, Blue.
   :HSV: Hue, Saturation, Value.
   :HSL: Hue, Saturation, Lightness.

Output
======

Color
   Standard color output.
