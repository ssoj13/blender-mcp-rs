---
area: manual-compositing
source: blender-manual
version: 5.2
---

.. _bpy.types.CompositorNodeExposure:

*************
Exposure Node
*************

The Exposure Node adjusts the brightness of an image using a camera exposure parameter.

**Seealso:**

   The exposure can also be adjusted in the scene Color Management <render-post-color-management>.

Inputs
======

Image
   Standard color input.
Exposure
   Scalar factor to adjust the exposure of the image.

Outputs
=======

Image
   Standard color output.

Examples
========

In the example below, the Exposure node is used to increase the brightness of the window area using a mask.
