---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeNURBSOrder:

****************
NURBS Order Node
****************

The *NURBS Order* node outputs the order of each NURBS spline.

The order determines how many control points influence each segment of the curve.
Higher orders produce smoother curves with broader control over the curve shape.

**Note:**

   This node reads the built-in `nurbs_order` attribute. The same values
   can be accessed, modified, or transferred using nodes that operate on attributes.

Inputs
======

This node has no inputs.

Outputs
=======

Order
   The order of the NURBS spline.

   For geometry that is not a NURBS curve, the output is zero.
