---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.mesh.face_split_by_edges:

*********************
Weld Edges into Faces
*********************

Splits the selected faces along any selected wire edges (edges not part of any faces)
that share their vertices. The wire edges become part of the new faces.

.. list-table::

   * - .. figure:: /images/modeling_meshes_editing_face_weld-edges-faces_before.png

          Selecting a quad face and a wire edge.

     - .. figure:: /images/modeling_meshes_editing_face_weld-edges-faces_after.png

          Welding splits the quad into two triangles.

**Seealso:**

   Instead of creating edges and then running this operator, it may be easier to use
   /modeling/meshes/editing/vertex/connect_vertex_path or the
   /modeling/meshes/editing/mesh/knife_topology_tool to split the faces directly.
