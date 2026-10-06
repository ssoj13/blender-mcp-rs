---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSetGeometryBundle:

************************
Set Geometry Bundle Node
************************

The *Set Geometry Bundle* node stores a bundle on a geometry.

Bundles can store arbitrary data alongside geometry, allowing information to
be passed between node groups, modifiers, and objects without modifying the geometry itself.

The bundle stored on a geometry can later be retrieved with the
Get Geometry Bundle </modeling/geometry_nodes/geometry/read/get_geometry_bundle> node.

Inputs
======

Geometry
   Geometry on which to store the bundle.

Bundle
   Bundle to store on the geometry.

Outputs
=======

Geometry
   The input geometry with the bundle attached.
