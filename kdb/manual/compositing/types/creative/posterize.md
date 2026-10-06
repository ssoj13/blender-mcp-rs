---
area: manual-compositing
source: blender-manual
version: 5.2
---

.. _bpy.types.CompositorNodePosterize:

*********
Posterize
*********

The *Posterize Node* reduces the number of colors in an image, converting
smooth gradients into sharp transitions.
This node is useful for generating masks in particular for rotoscoping.

Inputs
======

Image
   Standard color input.
Steps
   The number of colors per channel;
   A value of 8 will result in 8^3 = 512 total colors.

Outputs
=======

Image
   Standard color output.
