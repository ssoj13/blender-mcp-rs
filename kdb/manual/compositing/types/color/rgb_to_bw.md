---
area: manual-compositing
source: blender-manual
version: 5.2
---

.. _bpy.types.CompositorNodeRGBToBW:
.. Editor's Note: This page gets copied into:
.. - </render/shader_nodes/color/rgb_to_bw>
.. - </editors/texture_node/types/converter/rgb_to_bw>

.. --- copy below this line ---

**************
RGB to BW Node
**************

The *RGB to BW Node* makes a color image black-and-white by outputting its luminance.

**Note:**

   You can directly connect Color sockets to Value sockets in node graphs,
   which also converts the image to black-and-white. As such, this node is
   not always necessary.

Inputs
======

Image
   Color image input.

Outputs
=======

Value
   Grayscale value output.

.. (TODO add) examples of why this might be useful
