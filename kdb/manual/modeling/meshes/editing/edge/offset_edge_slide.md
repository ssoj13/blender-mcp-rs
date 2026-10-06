---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.mesh.offset_edge_loops_slide:

*****************
Offset Edge Slide
*****************

Creates two new edges surrounding each selected edge.

Usage
=====

#. Select one or more edges.
#. Click the menu item or press Shift-Ctrl-R.
#. Move the mouse to the right to increase the distance or to the left to decrease it.
   Hold Shift to move more slowly for better precision.

   Alternatively, type a number.
#. Press LMB or Return to confirm.

.. list-table::
   :widths: 1 1 1

   * - .. figure:: /images/modeling_meshes_editing_edge_offset-edge-slide_before.png

          Edges selected.

     - .. figure:: /images/modeling_meshes_editing_edge_offset-edge-slide_after.png

          Surrounding edges created.

     - .. figure:: /images/modeling_meshes_editing_edge_offset-edge-slide_cap.png

          *Cap Endpoint* enabled.

Options
=======

While the tool is active, the *Factor* is shown at the top of the 3D Viewport and the shortcuts
are shown in the status bar </interface/window_system/status_bar>. After confirming
with LMB, the options can still be changed in the bpy.ops.screen.redo_last panel.

Cap Endpoint
   Creates triangles at the endpoints of each edge chain.

Factor
   Relative distance across the surrounding faces at which to create the new edges.

Even E
   When enabled, the new vertices will keep an absolute distance from the edge chain or the
   surrounding edges (instead of a relative distance).

Flipped F
   Switch between matching the shape of the edge chain or the surrounding edges when *Even*
   is enabled.

.. list-table::
   :widths: 1 1

   * - .. figure:: /images/modeling_meshes_editing_edge_offset-edge-slide_even.png

          *Even* enabled.

     - .. figure:: /images/modeling_meshes_editing_edge_offset-edge-slide_even-flip.png

          *Even* and *Flipped* enabled.

Clamp Alt or C
   Prevents the new edges from moving outside their neighboring faces.

Mirror Editing
   When checked, sliding the newly created edges will also slide any existing edges
   on the other side of the mesh. Mesh Symmetry
   needs to be enabled for this to work.

Correct UVs
   Set the UV coordinates </editors/uv/introduction> of the new vertices according
   to their position on the mesh. When unchecked, the newly created vertices will have the same
   UV coordinates as the originally selected ones.
