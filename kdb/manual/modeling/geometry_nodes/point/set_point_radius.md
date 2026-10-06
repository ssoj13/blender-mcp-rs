---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSetPointRadius:

*********************
Set Point Radius Node
*********************

The *Set Point Radius* node controls the size each selected point cloud point should display with in the viewport.

The input node for this data is the /modeling/geometry_nodes/geometry/read/radius.

Inputs
======

Geometry
   Standard geometry input.

Radius
   Float value indicating the radius of the point geometry at each point.

Selection
   Boolean input for selecting which points will have the radius value applied.

Outputs
=======

Geometry
   Standard geometry output.
