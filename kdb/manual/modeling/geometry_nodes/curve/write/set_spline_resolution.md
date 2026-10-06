---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSetSplineResolution:

**************************
Set Spline Resolution Node
**************************

The *Set Spline Resolution* node sets the value for how many evaluated points should be generated on the curve for
every control point. It only has an effect on NURBS, Bézier, and Catmull Rom splines. In case of
Bézier splines, the resolution does not have an effect on segments between vector handles.

The evaluated points are displayed in the viewport, used in the
/modeling/geometry_nodes/curve/operations/curve_to_mesh node,
and optionally used in the /modeling/geometry_nodes/curve/operations/resample_curve.

The input node for this data is the /modeling/geometry_nodes/curve/read/spline_resolution.

Inputs
======

Curve
   Standard geometry input.

Selection
   Whether or not to change the resolution value on each spline. True values mean the value will be changed,
   false values mean the resolution will remain the same.

Resolution
   The number of evaluated points generated for each control point in NURBS and Bézier splines.
   It controls the accuracy of operations like trimming or sampling a curve.
   Higher resolutions are more accurate, but slower.

Outputs
=======

Curve
   Standard geometry output.
