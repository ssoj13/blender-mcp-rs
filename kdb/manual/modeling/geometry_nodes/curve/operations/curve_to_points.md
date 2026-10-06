---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeCurveToPoints:

********************
Curve to Points Node
********************

The *Curve to Points* node generates a point cloud from a curve.

Inputs
======

Curve
   Standard curve input.

Count
   Number of points generated. This input is only available for *Count* mode.

Length
   Length of the curve. This input is only available for *Length* mode.

Properties
==========

Mode
   :Evaluated:
      Creates points from the curve's evaluated points based on
      the resolution </modeling/geometry_nodes/curve/read/spline_resolution> attribute
      for NURBS and Bézier splines. This mode will generally be the fastest,
      since the second step of sampling equal lengths is not necessary.

   :Count:
      Sample each spline by evenly distributing the specified number of points.

   :Length:
      Sample each spline by splitting it into segments with specified length.
      The length will be rounded down so that a whole number of samples will fit in
      each input spline. To counteract jumping when the length of the spline changes,
      the /modeling/geometry_nodes/curve/operations/trim_curve can be used with
      a multiple of this length.

Outputs
=======

Points
   Generated point cloud.

Tangent
   The normalized curve tangent </modeling/geometry_nodes/curve/read/curve_tangent> at the sampled
   position, or the direct evaluated normal in *Evaluated* mode.

Normal
   The normal value from the evaluated curve at each result point.
   This is the same value from the /modeling/geometry_nodes/geometry/read/normal
   at those positions.

Rotation
   The Euler rotation build from the *Tangent* and *Normal* outputs, for convenience.
