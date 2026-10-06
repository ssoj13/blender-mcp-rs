---
area: manual-compositing
source: blender-manual
version: 5.2
---

.. _bpy.types.CompositorNodePremulKey:

******************
Alpha Convert Node
******************

The *Alpha Convert Node* converts the alpha channel format of an image.

For compositing and rendering, Premultiplied Alpha is the standard in Blender.
Render layers will be premultiplied alpha, and images loaded into rendering
or compositing will be converted to this.

If you want to do a compositing operation with straight alpha,
the *Alpha Convert* node can be used. Typically this would be a color correction operation
where it might give better results working on RGB channels without alpha.
If the alpha is converted to straight in the Compositor,
it should be converted back to premultiplied before the *Group Output* node,
otherwise some artifacts might occur.

Inputs
======

Image
   Standard color input.
Type
   The direction of convert alpha.
   For details on the difference between both ways to store alpha values see Alpha Channel.

   :To Premultiplied:
      Converts from Straight Alpha to Premultiplied Alpha.
   :To Straight:
      Converts from Premultiplied Alpha to Straight Alpha.

Outputs
=======

Image
   Standard color output.
