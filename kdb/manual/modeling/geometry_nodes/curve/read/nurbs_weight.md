---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeNURBSWeight:

*****************
NURBS Weight Node
*****************

The *NURBS Weight* node outputs the NURBS weight of each control point in a NURBS curve.

NURBS weights influence how strongly each control point affects the shape of
the curve. Higher weights pull the curve closer to the corresponding control point.

**Note:**

   This node reads the built-in `nurbs_weight` attribute. The same values
   can be accessed, modified, or transferred using nodes that operate on attributes.

Inputs
======

This node has no inputs.

Outputs
=======

Weight
   The NURBS weight of each control point.

   For geometry that is not a NURBS curve, the output is zero.
