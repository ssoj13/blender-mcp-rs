---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputMeshFaceIsPlanar:

*******************
Is Face Planar Node
*******************

The *Is Face Planar* node outputs whether every triangle of a
quads <Quad> or N-gons <N-gon> is on the same plane as all of the others, in
other words, if they have the same normal </modeling/meshes/editing/mesh/normals>.

For example, a non-planar face can be created by moving a single vertex in a face but not
the others. Triangles will always be planar.

Inputs
======

Threshold
    The distance a point can be from the surface before the face is no longer
    considered planar.

Outputs
=======

Planar
   Whether each mesh face is planar.

Examples
========

Combined with the /modeling/geometry_nodes/geometry/material/set_material,
this node is used to visualize all non-planar faces in a mesh.
