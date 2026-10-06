---
area: manual-modeling
source: blender-manual
version: 5.2
---

******************
Magic Texture Node
******************

**Note:**

   This node is ported from shader nodes. The manual and images are
   referencing the shader version of the node.
   This node accepts field inputs and outputs.
   When not connected the Vector input has an implicit `position` attribute value.

The *Magic Texture* node is used to add a psychedelic color texture.

Inputs
======

Vector
   Texture coordinate to sample texture at;
   defaults to Generated texture coordinates if the socket is left unconnected.
Scale
   Scale of the texture.
Distortion
   Amount of distortion.

Properties
==========

Depth
   Number of iterations.

Outputs
=======

Color
   Texture color output.
Factor
   Texture intensity output.

Examples
========
