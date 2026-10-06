---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputMeshFaceArea:

**************
Face Area Node
**************

The *Face Area* node outputs the surface area of a mesh's faces.
The units are in Blender units no matter the unit system,
equivalent to meters-squared at the default unit scale.

**Note:**

   For quads <Quad> and N-gons <N-gon>, when the face's vertices are not planar,
   the output is not necessarily the same as the sum of every one of the face's triangles visible
   in the viewport. In this case it should only be used an approximation. In some cases,
   the /modeling/geometry_nodes/mesh/operations/triangulate can be used to get an exact value.

Inputs
======

This node has no inputs.

Outputs
=======

Area
   The surface area of each of the mesh's faces.

Examples
========

Combined with the /modeling/geometry_nodes/attribute/attribute_statistic,
this node can be used to calculate the total surface area of a mesh.
