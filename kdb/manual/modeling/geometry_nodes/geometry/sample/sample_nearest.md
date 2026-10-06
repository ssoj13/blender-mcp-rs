---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSampleNearest:

*******************
Sample Nearest Node
*******************

The *Sample Nearest* node retrieves the index </modeling/geometry_nodes/geometry/read/input_index>
of the geometry element in its input geometry that is closest to the input position.

This node is similar to the /modeling/geometry_nodes/geometry/sample/geometry_proximity,
but it outputs the index of the closest element instead of its distance from the current location.

**Tip:**

  If you want to find nearest to each point in same geometry, its better to use
  the /modeling/geometry_nodes/geometry/sample/index_of_nearest node.

Inputs
======

Geometry
   The geometry to sample.

**Note:**

      This node only supports point cloud and mesh inputs.

Sample Position
   The position to start from when finding the closest location on the target geometry.
   By default, this is the same as if the /modeling/geometry_nodes/geometry/read/position was connected.

Properties
==========

Domain
   The attribute domain <attribute-domains> to consider the distance from.

Outputs
=======

Index
   The index </modeling/geometry_nodes/geometry/read/input_index> of the closest geometry element of the
   chosen domain. For the *Face Corner* domain this is defined to be the closest corner on the closest face.

Examples
========
