---
area: manual-compositing
source: blender-manual
version: 5.2
---

.. _bpy.types.CompositorNodeRLayers:

******************
Render Layers Node
******************

Renders a View Layer </render/layers/introduction> and reads its
Passes </render/layers/passes> into the compositing node graph.

Inputs
======

This node has no input sockets.

Properties
==========

Scene
   The scene for which to render a view layer.

View Layer
   The view layer to render. The button next to the dropdown re-renders it immediately.

**Hint:**

   To use the compositing output from another scene rather than its "raw" render output,
   first render that scene into a series of multi-layered images (using e.g. the OpenEXR format),
   then load those images into the Compositor of the current scene using the
   /compositing/types/input/image.

Outputs
=======

Image
   Rendered image.
Alpha
   Alpha channel.
Render pass sockets
   Additional outputs for any enabled render passes.

**Note:**

   The viewport compositor
   only supports render passes when using EEVEE.
   For other engines, the passes will be empty.
