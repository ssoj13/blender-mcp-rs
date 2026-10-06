---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeAttributeDomainSize:

****************
Domain Size Node
****************

The *Domain Size* outputs the size of an attribute domain on the selected geometry type,
for example, the number of edges in a mesh, or the number of points in a point cloud.

For more information about attribute domains, see the geometry attributes page <attribute-domains>.

Inputs
======

Geometry
   Standard geometry input.

Properties
==========

Component
    Which geometry type to retrieve the domain sizes from.

Outputs
=======

Point Count
   The size of the *Point* domain on any supported geometry.

Edge Count
   The size of the *Edge* domain on meshes.

Face Count
   The size of the *Face* domain on meshes.

Face Corner Count
   The size of the *Face Corner* domain on meshes.

Spline Count
   The size of the *Spline* domain on curves.

Instance Count
   The number of top-level instances in the geometry. Nested instances
   are not considered.
