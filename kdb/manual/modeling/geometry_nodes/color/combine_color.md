---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.FunctionNodeCombineColor:
.. --- copy below this line ---

******************
Combine Color Node
******************

Combines four grayscale channels into one color image,
based on a particular Color Model.

Inputs
======

The inputs of this node depend on the Mode property (see below).

Alpha
   The opacity of the output color.

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
