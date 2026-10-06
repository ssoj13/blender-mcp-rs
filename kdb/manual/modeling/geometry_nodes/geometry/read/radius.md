---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputRadius:

***********
Radius Node
***********

The *Radius* node outputs the radius value at each point on the evaluated geometry.
For curves, this value is used for things like determining the size of the mesh created in
the Curve to Mesh </modeling/geometry_nodes/curve/operations/curve_to_mesh> node.
For point clouds, the value is used for the display size of the point in the viewport.

Inputs
======

This node has no inputs.

Outputs
=======

Radius
   Float value indicating radius at each point on the geometry.
