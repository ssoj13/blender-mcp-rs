---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.curve.primitive*add:

****************
Curve Primitives
****************

**Seealso:**

   When adding curves there are some common options like other Objects <object-common-options>.

**Note:**

   Eventually all the primitive curves will be replaced to use the same curve system used for hair curves.
   Until this is done, their features will diverge.

   They can be converted interchangeably to access the full range of edit and sculpting functionalities.

In Object/Edit Mode, the *Add Curve* menu, provides a few different curve primitives:

Bézier Curve
============

Adds an open 2D Bézier curve with two control points.

Bézier Circle
=============

Adds a closed, circle-shaped 2D Bézier curve (made of four control points).

NURBS Curve
===========

Adds an open 2D NURBS curve, with four control points, with *Uniform* knots.

NURBS Circle
============

Adds a closed, circle-shaped 2D NURBS curve (made of eight control points).

Path
====

Adds a NURBS open 3D curve made of five aligned control points,
with *Endpoint* knots and the *Curve Path* setting enabled.
