---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeCurvePrimitiveCircle:

*****************
Curve Circle Node
*****************

The *Curve Circle* node generates a poly spline circle.

Inputs
======

Resolution
   Number of edges on the circle.

Radius
   The radius of the circle.

Point 1, Point 2, Point 3
   The three points on the circle.
   The order of the points determines the direction (clockwise or counterclockwise) of the circle.

**Note:**

   Because of the finite resolution, the three points do not necessarily lie on the generated curve.

Properties
==========

Mode
   :Points:
      The position and radius of the circle are determined by three points.
      The center of the circle is also given as an output.
      If the three points lie on one line, no geometry is generated.
   :Radius:
      The circle is determined by the radius.

Outputs
=======

Curve
   Poly spline generated from the inputs.

Center
   The center of the circle defined by the three points.
