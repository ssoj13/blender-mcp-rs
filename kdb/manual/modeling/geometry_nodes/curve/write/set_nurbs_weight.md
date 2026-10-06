---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSetNURBSWeight:

*********************
Set NURBS Weight Node
*********************

The *Set NURBS Weight* node sets the weight of control points
on NURBS curves by modifying the `nurbs_weight` attribute.

In NURBS curves, the weight influences how strongly a control point affects the
shape of the curve. Higher weights pull the curve closer to the control point,
while lower weights reduce its influence.

Inputs
======

Curve
   Input curve geometry. Only curves of type NURBS are affected; other curve
   types are passed through unchanged.

Selection
   The control points whose weights will be modified.

Weight
   The weight value assigned to the selected control points.

Outputs
=======

Curve
   The modified curve geometry with updated NURBS weights.
