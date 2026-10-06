---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeEdgePathsToSelection:

****************************
Edge Paths to Selection Node
****************************

The *Edge Paths to Selection* node follows paths across mesh edges and outputs a selection
of every visited edge.

**Seealso:**

   This node is meant to use the output of the /modeling/geometry_nodes/mesh/read/shortest_edge_paths.
   It can be combined with the /modeling/geometry_nodes/geometry/operations/separate_geometry to remove
   any unused edges.

Inputs
======

Start Vertices
   A selection of the vertices to start at when traveling along the next vertex indices.

Next Vertex Index
   Describes the path to follow at every vertex.

Outputs
=======

Selection
   A boolean field indicating all edges visited when traversing the mesh.
