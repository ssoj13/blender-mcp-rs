---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeMeshUVSphere:

**************
UV Sphere Node
**************

The *UV Sphere* node generates a spherical mesh mostly out of quads except for triangles at the top and bottom.

Inputs
======

Segments
   Horizontal resolution of the sphere.
   If this is smaller than three, no mesh is generated.

Rings
   Vertical resolution of the sphere.
   If this is smaller than two, no mesh is generated.

Radius
   Distance of vertices to the origin.

Outputs
=======

Mesh
   Standard geometry output.

UV Map
   A 2D vector representing the default X/Y coordinates of the UV Map for the primitive's shape.
   This can be connected to the /modeling/geometry_nodes/attribute/store_named_attribute,
   to be used once the Geometry Nodes Modifier get applied.
   The UV map must be stored on the face corner in order to be accessed.
