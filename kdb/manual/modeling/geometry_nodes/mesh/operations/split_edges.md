---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSplitEdges:

****************
Split Edges Node
****************

Like the /modeling/modifiers/generate/edge_split, the *Split Edges* node splits and duplicates edges
within a mesh, breaking 'links' between faces around those split edges.

Inputs
======

Mesh
   Standard geometry input.

Selection
   A standard Boolean selection input to determine which edges will be split.

**Note:**

   Because of mesh topology requirements, sometimes more or fewer edges than are selected will be split.

Outputs
=======

Mesh
   Standard geometry output.
