---
area: manual-modeling
source: blender-manual
version: 5.2
---

*****
Split
*****

The *Split* menu can be accessed as a popover by pressing Alt-M.

These operators are about splitting meshes by disconnecting faces from each other.
For splitting faces into smaller ones, see the /modeling/meshes/editing/mesh/knife_topology_tool
or the /modeling/meshes/editing/vertex/connect_vertex_path operator.

.. _bpy.ops.mesh.split:

Split Selection
===============

Cuts the mesh along the border of the selected faces. The faces can then be moved elsewhere.

This operator can also disconnect "wires" (edges that are not part of a face) from
their surroundings. However, using it on edges that *are* part of a face will simply duplicate them.

**Seealso:**

   The /modeling/meshes/editing/mesh/separate operator disconnects the selection
   and moves it into a new mesh object.

.. _bpy.ops.mesh.edge_split:

Split Faces by Edges
====================

Cuts the mesh along the selected edges. The vertices can then be pulled apart to create a hole.

**Seealso:**

   /modeling/meshes/editing/vertex/rip_vertices also cuts neighboring unselected edges.

Split Faces & Edges by Vertices
===============================

Cuts the mesh along *all* neighboring edges of each selected vertex. (This is different from
*Rip Vertices* which only cuts two edges per vertex.)

The vertices can then be pulled apart to create a hole.
