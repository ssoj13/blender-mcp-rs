---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputCurveHandlePositions:

***************************
Curve Handle Positions Node
***************************

Gets the two handle positions of each control point in a Bézier spline.

You can use the /modeling/geometry_nodes/curve/write/set_handle_positions
to change these positions.

Inputs
======

Relative
   Output the handle positions relative to the control point
   instead of in the local space of the geometry.

Outputs
=======

Left
   The position of the control point's left handle.

Right
   The position of the control point's right handle.
