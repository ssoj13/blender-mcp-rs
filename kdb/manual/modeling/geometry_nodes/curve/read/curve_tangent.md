---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputTangent:

******************
Curve Tangent Node
******************

The *Curve Tangent* node outputs the direction that a curve points in at each control point,
depending on the direction of the curve (which can be controlled with
the /modeling/geometry_nodes/curve/operations/reverse_curve). The output values are normalized vectors.

**Warning:**

   For NURBS and Bézier spline curves, keep in mind that the value retrieved from this node is
   the value at every control point, which may not correspond to the visible *evaluated* points.
   For example, a Bézier spline might have 48 evaluated points, but only four control points,
   if its resolution is 12. For NURBS splines the difference may be even more pronounced and the result
   may not be as expected. A /modeling/geometry_nodes/curve/operations/resample_curve can be used to
   create a poly spline, where there is a control point for every evaluated point.

Inputs
======

This node has no inputs.

Outputs
=======

Tangent
   The direction of the curve at every control point.
