---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.types.TextureNodeAt:

*******
At Node
*******

Returns the color of a texture at the specified coordinates.

Inputs
======

Texture
   Standard color input.
Coordinates
   The point at which to sample the color. For images, the space is between -1 and 1 for X and Y.
   If the coordinates are not spatially varying, the node will return a single color.

Properties
==========

This node has no properties.

Outputs
=======

Texture
   Standard color output.
