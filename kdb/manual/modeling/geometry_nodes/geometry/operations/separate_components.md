---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSeparateComponents:

************************
Separate Components Node
************************

The *Separate Components* node splits a geometry into a separate output
for each type of data in the geometry.

Inputs
======

Geometry
   Standard geometry input.

Outputs
=======

Mesh
   Mesh component of the input geometry.

Curve
   Curve component of the input geometry.

Point Cloud
   Point cloud component of the input geometry.

Volume
   Volume component of the input geometry.

Instances
   Instances component of the input geometry. Even if the instances contain geometry data with
   one of the other types, all instances will be added to this output.
   A /modeling/geometry_nodes/instances/realize_instances can be added to move the data from
   geometry instances to their corresponding outputs.
