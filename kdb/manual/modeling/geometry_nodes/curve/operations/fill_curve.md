---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeFillCurve:

***************
Fill Curve Node
***************

The *Fill Curve* node generates a mesh using the constrained Delaunay triangulation algorithm
with the curves as boundaries. The mesh is only generated flat with a local Z of 0.

Inputs
======

Curve
   Standard geometry input with a curve component.

Group ID
   Value used to group curves together.
   Curves with different Group ID are treated separately.

Mode
   The type of geometry the output consists of.

   :Triangles: The output is made up of triangles.
   :N-gons: The output is made up of n-gons.

Outputs
=======

Mesh
   The filled-in curves.

Examples
========

Customized triangulation
------------------------

One or many "single point spline" can be used to customize the triangulation of the filled-in curves.

Group ID
--------

The following figures display diverse application of the Group ID.

.. The following example were made with some nodes "hidden" to illustrate the amount of mesh islands
.. and avoid Z fighting.
.. Mesh Island Index -> Set Position (as z offset)
.. Mesh Island Index -> Map Range (Mesh island count as input max) -> Color Ramp (distributed from left)
.. Colors Used : 5ABD9E, 85BD50, DDB72C, D26072
