---
area: manual-modeling
source: blender-manual
version: 5.2
---

*********************
Deleting & Dissolving
*********************

.. _bpy.ops.mesh.delete:

Delete
======

Deletes the selected mesh elements. The following menu items are available:

Vertices
   Deletes the selected vertices, as well as any edges and faces they are part of.
Edges
   Deletes the selected edges, as well as any faces they are part of.
   Orphaned vertices are deleted too.
Faces
   Deletes the selected faces. Orphaned vertices and edges are deleted too.
Only Edges & Faces
   Deletes the selected edges and faces, as well as any unselected faces that
   contain a selected edge. Orphaned vertices are left behind.
Only Faces
   Deletes the selected faces. Orphaned vertices and edges are left behind.

.. _bpy.ops.mesh.dissolve:

Dissolve
========

*Dissolving* is different from deleting in that it keeps the surface closed
rather than leaving a hole.

.. _bpy.ops.mesh.dissolve_verts:

Dissolve Vertices
-----------------

Deletes each selected vertex and joins its surrounding faces into one. (This often results in
n-gons <n-gon>.)

If a vertex is part of a wire -- that is, a chain of edges that are not part of any face --
its surrounding edges are merged instead.

The bpy.ops.screen.redo_last panel offers the following options:

Face Split
   Split surrounding faces where possible so that only a triangle in their corner is merged instead of
   the whole face. This reduces the size of the final "hole-filling" faces and can make them less uneven.
Tear Boundaries
   Delete faces at the mesh boundary instead of merging them. These faces are always split,
   even if *Face Split* is disabled.

.. list-table::
   :widths: 1 1 1 1

   * - .. figure:: /images/modeling_meshes_editing_mesh_delete_dissolve-example-1.png

          Original mesh

     - .. figure:: /images/modeling_meshes_editing_mesh_delete_dissolve-example-2.png

          Face Split: Off

          Tear Boundaries: Off

     - .. figure:: /images/modeling_meshes_editing_mesh_delete_dissolve-example-3.png

          Face Split: On

          Tear Boundaries: Off

     - .. figure:: /images/modeling_meshes_editing_mesh_delete_dissolve-example-4.png

          Face Split: On/Off

          Tear Boundaries: On

.. _bpy.ops.mesh.dissolve_edges:

Dissolve Edges
--------------

Deletes each selected edge and joins its surrounding faces into one. This is only done for edges that
have exactly two neighboring faces.

The Adjust Last Operation panel offers the following options:

Dissolve Vertices
   Also dissolve the vertices of the selected edges, not just the edges themselves.
Angle Threshold
   If *Dissolve Vertices* is enabled, this option allows preserving vertices at corners,
   only dissolving the ones in flattish areas. Faces are considered to form a corner if
   the angle between their *normals* exceeds the Angle Threshold -- that is, coplanar faces
   are considered to have an angle of 0°, not 180°.
Face Split
   Split surrounding faces where possible so that only a triangle in their corner is merged instead of
   the whole face. This reduces the size of the final "hole-filling" faces and can make them less uneven.
Preserve Quads
   Prevents vertices from being removed if doing so would break the quad structure
   when dissolving the edge between two triangles.

   Even if *Dissolve Vertices* is enabled, vertices shared by the two triangles
   are kept so the resulting geometry remains a quad rather than collapsing
   into a triangle or n-gon.

   This option helps maintain clean quad topology when simplifying meshes or
   removing unnecessary edges.

.. _bpy.ops.mesh.dissolve_faces:

Dissolve Faces
--------------

Merges each patch of connected faces into a single face.

**Note:**

   The F shortcut is normally used for creating faces, but when run on a selection of existing faces,
   it dissolves them instead. See modeling-mesh-make-face-edge-dissolve.

.. list-table::

   * - .. figure:: /images/modeling_meshes_editing_mesh_delete_dissolve-faces_before.png

          Before dissolving.

     - .. figure:: /images/modeling_meshes_editing_mesh_delete_dissolve-faces_after.png

          After dissolving.

.. _bpy.ops.mesh.dissolve_limited:

Limited Dissolve
================

Simplifies the selected geometry by dissolving vertices and edges in flattish areas.

.. list-table::

   * - .. figure:: /images/modeling_meshes_editing_mesh_delete_limited-dissolve-before.png
          :width: 320px

          Original mesh.

     - .. figure:: /images/modeling_meshes_editing_mesh_delete_limited-dissolve-after.png
          :width: 320px

          Result of Limited Dissolve.

Max Angle
   The maximum allowed angle between faces for the surface to be considered flat (and thus eligible
   for dissolving). Specifically, the angle is measured between face normals, meaning that coplanar
   faces are considered to have an angle of 0°, not 180°.
All Boundaries
   After dissolving the edges using *Max Angle*, dissolve all vertices at boundary corners --
   specifically, vertices that only have two neighboring edges.
Delimit
   Prevent merging faces that are discontinuous in some way. It's possible to activate multiple of
   these options by clicking them with Shift-LMB.

   :Normal: Don't merge faces that have opposite
            orientations <bpy.types.View3DOverlay.show_face_orientation>.
   :Material: Don't merge faces that have different materials.
   :Seam: Don't dissolve edges that are marked as UV Seams </modeling/meshes/uv/unwrapping/seams>.
   :Sharp: Don't dissolve edges that are marked as Sharp <bpy.ops.mesh.mark_sharp>.
   :UVs: Don't merge faces that are disconnected in any UV map.

.. _bpy.ops.mesh.edge_collapse:

Collapse Edges & Faces
======================

Collapses each patch of connected edges and faces into a single vertex.
This is useful for collapsing edge rings, for example.

.. list-table::
   :widths: 1 1

   * - .. figure:: /images/modeling_meshes_editing_mesh_delete_collapse-before.png

          Selected edge rings.

     - .. figure:: /images/modeling_meshes_editing_mesh_delete_collapse-after.png

          Edge rings collapsed.

Unlike the *Dissolve* operators, this operator doesn't create n-gons <n-gon>
and can add new vertices instead of only removing existing ones. In addition,
the new vertices automatically receive interpolated UV coordinates, color attributes, etc.
This makes it particularly useful for manually removing detail.

.. _bpy.ops.mesh.delete_edgeloop:

Edge Loops
==========

Dissolves the selected edge loops. This is essentially the same as
bpy.ops.mesh.dissolve_edges, except that it selects the surrounding faces afterwards.

.. list-table::

   * - .. figure:: /images/modeling_meshes_editing_mesh_delete_edge-loop-before.png
          :width: 320px

          Selected edge loop.

     - .. figure:: /images/modeling_meshes_editing_mesh_delete_edge-loop-after.png
          :width: 320px

          Edge loop dissolved.

**Note:**

   In Blender terminology, an *edge loop* is any chain of connected edges --
   it doesn't have to be a closed loop. See bpy.ops.mesh.loop_select.

**Seealso:**

   - Vertex merging
   - /modeling/meshes/editing/face/triangles_quads
   - /modeling/meshes/editing/edge/unsubdivide
