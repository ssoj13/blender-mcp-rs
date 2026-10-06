---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputEdgeSmooth:

*******************
Is Edge Smooth Node
*******************

The *Is Edge Smooth* node outputs true for each edge of the mesh that is *not* marked as sharp. Otherwise, if the edge
*is* marked as sharp, then the node outputs false.

**Seealso:**

   Mark Sharp & Clear Sharp

Inputs
======

This node has no inputs.

Outputs
=======

Smooth
   Boolean value that indicates whether the edges of the mesh are *not* marked as sharp.
