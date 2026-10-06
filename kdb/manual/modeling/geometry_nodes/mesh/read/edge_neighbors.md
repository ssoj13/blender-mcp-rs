---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputMeshEdgeNeighbors:

*******************
Edge Neighbors Node
*******************

The *Edge Neighbors* node outputs topology information relating to each edge of a mesh.

Inputs
======

This node has no inputs.

Outputs
=======

Face Count
    The number of faces that use the edge as one of their sides.
    When the value is one, the edge is a non-manifold <Non-manifold> boundary edge.
    Alternatively, when the value is zero, the edge is a loose edge, not used by any faces.

Examples
========
