---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeMeshIcoSphere:

**************
Icosphere Node
**************

The *Icosphere* node generates a spherical mesh that consists of equally sized triangles.

Inputs
======

Radius
   Distance of the vertices from the origin.

Subdivisions
   Number of subdivisions on top of the most basic icosphere.
   The number of faces quadruple with every subdivision.

Outputs
=======

Mesh
   Standard geometry output.

UV Map
   A 2D vector representing the default X/Y coordinates of the UV Map for the primitive's shape.
   This can be connected to the /modeling/geometry_nodes/attribute/store_named_attribute,
   to be used once the Geometry Nodes Modifier get applied.
   The UV map must be stored on the face corner in order to be accessed.
