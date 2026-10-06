---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeCurvePrimitiveLine:

***************
Curve Line Node
***************

The *Curve Line* node generates poly spline line.

Inputs
======

Start
   Position of the first control point.

End
   Position of the second control point.
   This is only available in the *Points* mode.

Direction
   Direction the line is going in.
   The length of this vector does not matter.
   This is only available in the *Direction* mode.

Length
   Length of the line.
   This is only available in the *Direction* mode.

Properties
==========

Mode:
   :Points:
      Define the spline with start and end points.
   :Direction:
      Define the spline with a start, direction and length.

Outputs
=======

Curve
   Standard geometry output.
