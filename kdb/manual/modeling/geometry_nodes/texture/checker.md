---
area: manual-modeling
source: blender-manual
version: 5.2
---

********************
Checker Texture Node
********************

**Note:**

   This node is ported from shader nodes. The manual and images are
   referencing the shader version of the node.
   This node accepts field inputs and outputs.
   When not connected the Vector input has an implicit `position` attribute value.

**Tip:**

   Texture nodes can produce details at a higher frequency
   than geometry can show. This may cause artifacts such
   as Moiré type patterns or a lack of detail due to
   insufficient sampling points.

The *Checker Texture* is used to add a checkerboard texture.

Inputs
======

Vector
   Texture coordinate to sample texture at;
   defaults to Generated texture coordinates if the socket is left unconnected.

**Warning:**

      This node can have precision issues with some vector inputs.
      See the notes for the White Noise Texture
      for ways to mitigate this issue.

Color1, Color 2
   Color of the checkers.
Scale
   Overall texture scale. The scale is a factor of the bounding box of the face divided by the scale.
   For example, a scale of 15 will result in 15 alternate patterns over the overall UV bounding box.
   Different patterns could be achieved using other nodes to give different input patterns to this socket.
   For example, using the Math node.

Outputs
=======

Color
   Texture color output.
Factor
   Checker 1 mask (1 = Checker 1).

Examples
========
