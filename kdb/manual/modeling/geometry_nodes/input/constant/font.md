---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputFont:
.. --- copy below this line ---

*********
Font Node
*********

The *Font* node outputs a font data-block that can be used by other nodes
that require a font input.

This allows a font to be selected directly in a node tree and reused across
multiple nodes. For example, it can be connected to the
/modeling/geometry_nodes/utilities/text/string_to_curves
or used to control the font of a compositor /compositing/types/input/string_to_image.

Outputs
=======

Font
   The font data-block to output.

   By default, Blender's built-in font is selected.
