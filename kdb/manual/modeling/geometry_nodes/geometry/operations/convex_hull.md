---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeConvexHull:

****************
Convex Hull Node
****************

The *Convex Hull* node outputs a convex mesh that is enclosing all points in the input geometry.

**Note:**

   When the node is used on a geometry with instances, the algorithm will run once per instance,
   resulting in many convex hull meshes in the instance geometries. The *Realize Instances* node can
   be used to get a convex hull of an entire geometry.

**Important:**

   Volumes are not supported by this node, and attributes are not automatically transferred to the result.

**Note:**

   This node is affected by the limitations of floating point precision. If points are too close together,
   they may be merged in the result mesh.

Inputs
======

Geometry
   Standard geometry input.

Outputs
=======

Convex Hull
   Mesh that encloses all points in the input.
