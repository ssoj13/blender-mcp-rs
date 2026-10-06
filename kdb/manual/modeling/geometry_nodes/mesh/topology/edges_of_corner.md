---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeEdgesOfCorner:

********************
Edges of Corner Node
********************

The *Edges of Corner* node retrieves the edges on both sides of a face corner.

Inputs
======

Corner Index
   The index of the input face corner.

**Note:**

      By default this uses the index
      from the field context, which makes it important that the node is evaluated on
      the face corner domain.

Outputs
=======

Next Edge Index
   The index of the neighboring edge in the face, in the direction of increasing face corner indices.

Previous Edge Index
   The index of the neighboring edge in the face, in the direction of decreasing face corner indices.
