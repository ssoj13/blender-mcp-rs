---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodePoints:

***********
Points Node
***********

The *Points* node generate a point cloud with positions and radii defined by
fields </modeling/geometry_nodes/fields>.

Inputs
======

Count
   The number of points to create.

Position
   The position of each generated point.

Radius
   The radius </modeling/geometry_nodes/geometry/read/radius> of each point.

**Note:**

   Since the point cloud is created from scratch, the *Position* and *Radius* inputs can only depend on
   the index </modeling/geometry_nodes/geometry/read/input_index> node. Regular input nodes like
   the position </modeling/geometry_nodes/geometry/read/position> won't work.

Outputs
=======

Points
   Standard geometry output.
