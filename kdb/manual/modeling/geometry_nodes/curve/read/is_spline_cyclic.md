---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputSplineCyclic:

*********************
Is Spline Cyclic Node
*********************

The *Is Spline Cyclic* controls whether each of the curve splines start and endpoints form a connection.
Its output corresponds to the built-in `cyclic` attribute on the curve spline domain.

The node to set this data is the /modeling/geometry_nodes/curve/write/set_spline_cyclic.

Inputs
======

This node has no inputs.

Outputs
=======

Cyclic
   Whether the spline is cyclic.
