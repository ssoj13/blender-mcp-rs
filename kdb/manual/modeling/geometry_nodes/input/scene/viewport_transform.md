---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeViewportTransform:

***********************
Viewport Transform Node
***********************

The *Viewport Transform* node retrieves the view direction and location of the /editors/3dview/index.

**Note:**

   This node can only be used in the Tool context <tool_context>.

Inputs
======

This node has no inputs.

Outputs
=======

Projection
   The 3D Viewport's perspective or orthographic projection matrix.
View
   The view direction and location of the 3D viewport.
Is Orthographic
   Whether the viewport is using orthographic projection.
