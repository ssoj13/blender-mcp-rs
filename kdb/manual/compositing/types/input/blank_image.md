---
area: manual-compositing
source: blender-manual
version: 5.2
---

.. _bpy.types.CompositorNodeBlankImage:

****************
Blank Image Node
****************

The *Blank Image* node generates an image of a specified size filled with a constant color.

This node is commonly used for procedural workflows, such as generating masks, creating
custom kernels for convolution or bokeh effects, and building images for compositing operations.

Inputs
======

Color
   The color of all pixels in the image.
Size
   The size of the image.

Outputs
=======

Image
   The generated image of the given size and constant color.
