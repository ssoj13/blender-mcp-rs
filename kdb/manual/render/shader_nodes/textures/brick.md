---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeTexBrick:
.. --- copy below this line ---

******************
Brick Texture Node
******************

The *Brick Texture* is used to add a procedural texture producing bricks.

Inputs
======

Color 1/2
   Color of the bricks.
Mortar
   The color of the area between bricks.
Scale
   Overall texture scale.
Mortar Size
   The size of the filling between the bricks known as "mortar"; 0 means no mortar.
Mortar Smooth
   Blurs/softens the edge between the mortar and the bricks.
   This can be useful with a texture and displacement textures.
Bias
   The color variation between *Color 1/2*.
   Values of -1 and 1 only use one of the two colors; values in between mix the colors.
Brick Width
   The ratio of brick's width relative to the texture scale.
Row Height
   The ratio of brick's row height relative to the texture scale.

Properties
==========

Offset
   Determines the brick offset of the various rows.
Frequency
   How often rows are offset; a value of 2 gives an even/uneven pattern of rows.

Squash
   Factor to adjust the brick's width for particular rows determined by the *Frequency*
Frequency
   How often rows consist of "squished" bricks.

Outputs
=======

Color
   Texture color output.
Factor
   Mortar mask (1 = mortar).

Examples
========
