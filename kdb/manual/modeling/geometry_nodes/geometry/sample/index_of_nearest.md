---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeIndexOfNearest:

****************
Index of Nearest
****************

The **Index of Nearest** node is a way to find other close elements in the same geometry.
If needed you can use Group ID to determine the group of neighbors to be analyzed together.

This is an alternative to the /modeling/geometry_nodes/geometry/sample/sample_nearest node.
The main difference is that this node does not require a geometry input, because the geometry
from the field context <field-context> is used.

**Tip:**

    This is often combined with the /modeling/geometry_nodes/utilities/field/evaluate_at_index or
    the /modeling/geometry_nodes/geometry/sample/sample_index node.

Inputs
======

Position
   The position for each element to search.
   By default, this is the same as if the /modeling/geometry_nodes/geometry/read/position was connected.

Group ID
   ID to group elements together.

Outputs
=======

Index
   The index
   of the closest element in the same geometry component.

Has Neighbor
   This is true when the group of the element has at least two elements.
   This is only relevant when using *Group ID*.
