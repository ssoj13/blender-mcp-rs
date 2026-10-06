---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeVertexOfCorner:

*********************
Vertex of Corner Node
*********************

Outputs the index of the vertex that a face corner is attached to.

Inputs
======

Corner Index
   The index of the face corner.

**Note:**

      If this input is not connected, it uses the
      index
      of the context item, which means it's important that the node is evaluated
      in the Face Corner domain.

Outputs
=======

Vertex Index
   The index of the vertex that the face corner is attached to.
