---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeMergeByDistance:

**********************
Merge by Distance Node
**********************

The *Merge by Distance* node merges selected mesh vertices or point cloud points within a given distance,
merging surrounding geometry where necessary. This operation is similar to the bpy.ops.mesh.remove_doubles
operator or the /modeling/modifiers/generate/weld.

**Tip:**

   For more control over which elements are merged, the
   Cluster by Distance </modeling/geometry_nodes/utilities/field/cluster_by_distance>,
   Cluster by Connected </modeling/geometry_nodes/mesh/read/cluster_by_connected>,
   and Merge Points
   nodes can be combined to build custom merge workflows.

   This makes it possible to merge elements based on arbitrary selections,
   groups, or connectivity without relying solely on distance-based merging.

Inputs
======

Geometry
   Standard geometry input.

Selection
   Boolean field that is true for parts of the geometry to be deleted.
   Unselected points will be completely unused for the operation--
   they will not be merged into other points, and no points will merge into them either.

**Tip:**

      When possible, using the selection input can be a simple way to speed up the node,
      since searching for nearby points is a relatively expensive operation that gets even
      more expensive when more points are involved.

Mode
   Method for choosing which vertices are merged.

   :All: Merge includes all geometry including loose parts.
   :Connected: Merge only includes attached geometry i.e. the modifier will not merge loose parts together.

Distance
    The distance to use for searching for nearby points.

Output
======

Geometry
   Standard geometry output.

Examples
========
