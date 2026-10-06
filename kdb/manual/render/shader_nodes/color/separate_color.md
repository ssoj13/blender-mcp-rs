---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeSeparateColor:

*******************
Separate Color Node
*******************

Splits a color into its channels,
based on a particular Color Model.

Inputs
======

Color
   Standard color input.

Properties
==========

Mode
   The color model to output.

   :RGB: Red, Green, Blue.
   :HSV: Hue, Saturation, Value.
   :HSL: Hue, Saturation, Lightness.

Outputs
=======

The outputs of this node depend on the Mode property (see above).
