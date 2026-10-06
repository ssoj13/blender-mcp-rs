---
area: manual-compositing
source: blender-manual
version: 5.2
---

.. _bpy.types.CompositorNodeIDMask:

************
ID Mask Node
************

The *ID Mask Node* creates a mask for a particular object or material in the render.
It relies on the *Object Index* or *Material Index* render pass </render/layers/passes>,
which is only available when rendering with Cycles.

**Seealso:**

   This node is superseded by the /compositing/types/mask/cryptomatte.
   Cryptomatte is more versatile and is supported by both Cycles and EEVEE.

Inputs
======

ID Value
   Input for the *Object Index* or *Material Index* render pass.
   Once a pass is enabled, it can be accessed through the *IndexOB* or *IndexMA* slot of the
   /compositing/types/input/scene/render_layers.
Index
   The index for which to create a mask. This index can be configured for objects at
   Properties --> Object --> Relations --> Pass Index,
   and for materials at Properties --> Material --> Settings --> Pass Index.

Anti-Aliasing
   Whether to smooth the mask edges.

Outputs
=======

Alpha
   A grayscale image that's white where the object exists and black where it does not.

Example
=======

In the example below, the left and right cubes are assigned a *Pass Index* of 1 and 2 respectively.
We extract a mask for the left cube, then use it to turn that cube red with a
/compositing/types/color/mix_color. The masks for the other Pass Indexes are also shown.

Limitations
===========

- Volume Objects </modeling/volumes/introduction> are not supported.
