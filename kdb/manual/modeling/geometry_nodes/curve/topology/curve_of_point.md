---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeCurveOfPoint:

*******************
Curve of Point Node
*******************

The *Curve of Point* node retrieves the index of the curve a control point is part of.
This node is conceptually similar to the /modeling/geometry_nodes/mesh/topology/face_of_corner.

Inputs
======

Point Index
   The index of the input control point.

**Note:**

      By default this uses the index
      from the field context, which makes it important that the node is evaluated on
      the point domain.

Outputs
=======

Curve Index
   The index of the curve the control point is part of.

Index in Curve
   How far along the control point is along its curve, with a value of 0 for the first point in each curve.
