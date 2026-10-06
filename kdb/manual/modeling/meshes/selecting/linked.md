---
area: manual-modeling
source: blender-manual
version: 5.2
---

*************
Select Linked
*************

.. _bpy.ops.mesh.select_linked:
.. _bpy.ops.mesh.select_linked_pick:

Linked
======

Selects all the geometry that's connected directly or indirectly to the current selection.
This is often useful when a mesh has disconnected but overlapping parts,
where isolating them any other way would be tedious.

Instead of selecting an element and pressing Ctrl-L, it's also possible to simply
hover the mouse over an element and press L. Alternatively, press Shift-L to deselect.

The bpy.ops.screen.redo_last panel has the following option:

Delimit
   Stop expanding the selection when it reaches certain boundaries.
   It's possible to activate multiple of these choices by clicking them with Shift-LMB.

   :Normal: Stop at faces that have an opposite
            orientation <bpy.types.View3DOverlay.show_face_orientation>.
   :Material: Stop at faces that have a different material.
   :Seam: Stop at edges that are marked as UV Seams </modeling/meshes/uv/unwrapping/seams>.
   :Sharp: Stop at edges that are marked as Sharp <bpy.ops.mesh.mark_sharp>.
   :UVs: Stop at island borders in any UV map.

.. _bpy.ops.mesh.shortest_path_select:
.. _bpy.ops.mesh.shortest_path_pick:

Shortest Path
=============

Selects the geometry along the shortest path between the two most recently
selected mesh elements.

To use the shortcut, first select the starting element normally,
then select the destination element with Ctrl-LMB.

Options
-------

Edge Tag Edge Mode
   What to do with the selected edges:

   :Select: Simply select the edges.
   :Tag Seam: Mark the edges as UV Seams </modeling/meshes/uv/unwrapping/seams>.
   :Tag Sharp: Mark the edges as Sharp <bpy.ops.mesh.mark_sharp>.
   :Tag Crease: Give the edges a Crease <modeling-edges-crease-subdivision> value of 1.0.
   :Tag Bevel: Give the edges a Bevel Weight <modeling-edges-bevel-weight> of 1.0.
   :Tag Freestyle Edge Mark: Mark the edges as Freestyle <freestyle-edge-marks> edges.

Face Stepping
   Allow diagonal paths for vertices and faces,
   and edge rings <bpy.ops.mesh.select_edge_ring_multi> for edges.
Topology Distance
   Pick the path that crosses the least edges instead of the smallest distance.
Fill Region Shift-Ctrl-LMB
   Select all possible shortest paths instead of just one.
Deselected, Selected, Offset
   Options for selecting a "dashed line" of elements instead of a continuous one.
   The path will contain a repeating pattern of a few unselected items followed
   by a few selected ones. See also /modeling/meshes/selecting/checker_deselect.

.. _bpy.ops.mesh.faces_select_linked_flat:

Linked Flat Faces
=================

Grows the selection to include connected faces until a sharp corner is reached.

Sharpness
   The maximum allowed angle between two faces for them to be considered part of
   the same flat surface.
   This angle is measured between the face normals, meaning that coplanar faces
   have an angle of 0° (not 180°) and sharp corners have a greater angle than
   dull ones (not smaller).

.. list-table::
   :widths: 1 1 1

   * - .. figure:: /images/modeling_meshes_selecting_linked_flat-faces_before.png

          Initial selection.

     - .. figure:: /images/modeling_meshes_selecting_linked_flat-faces_after.png

          After running Select Linked Flat Faces.

     - .. figure:: /images/modeling_meshes_selecting_linked_flat-faces_similar.png

          Comparison to Select Similar --> Normal.

To stop the selection at edges that were manually marked as Sharp <bpy.ops.mesh.mark_sharp>,
use bpy.ops.mesh.select_linked instead.
