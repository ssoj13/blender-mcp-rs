---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeOffsetCornerInFace:

**************************
Offset Corner in Face Node
**************************

Retrieves another corner in the same face as the input corner.
This is like "rotating" the input corner around in its face.

Conceptually the operation is similar to the
/modeling/geometry_nodes/curve/topology/offset_point_in_curve.

Inputs
======

Corner Index
   The index of the input face corner.

**Note:**

      If this input is not connected, it uses the
      index
      of the context item, which means it's important that the node is evaluated
      in the Face Corner domain.

Offset
   The number of corners to move around the face before finding the result,
   circling back to the first corner if necessary.

Outputs
=======

Corner Index
   The index of the offset face corner.
