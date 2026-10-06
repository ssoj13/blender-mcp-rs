---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeGeometryToInstance:

*************************
Geometry to Instance Node
*************************

The *Geometry to Instance* node turns every connected input geometry into an instance.
Visually, the node has a similar result as the /modeling/geometry_nodes/geometry/join_geometry,
but it outputs the result as separate instances instead. The geometry data itself isn't actually
joined.

The node can be used in combination with the *Pick Instances* option in the
/modeling/geometry_nodes/instances/instance_on_points, as a way to pick
between geometry generated in the node tree (as opposed to picking from separate
instances from the /modeling/geometry_nodes/input/scene/collection_info, for example).

**Tip:**

   This node can be much faster than the join geometry node when the inputs are large geometries.
   This is because the join geometry node must actually create a larger mesh, or a larger curve.
   Even though the operation is simple, just creating a large mesh can have a significant cost.
   This node can be better, because instead of merging large geometries, it just groups them
   together as instances.

Inputs
======

Geometry
   Geometry that will be joined. Multiple inputs are allowed.
   When the node is muted, only the first link will be passed through.

Output
======

Geometry
   Standard geometry output.

Examples
========

The node used in combination with the /modeling/geometry_nodes/instances/instance_on_points
to choose between multiple primitives for instancing.
