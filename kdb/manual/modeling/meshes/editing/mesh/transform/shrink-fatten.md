---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.transform.shrink_fatten:
.. _tool-mesh-shrink-fatten:

*************
Shrink/Fatten
*************

Moves the selected vertices "inwards" or "outwards" along their normal,
all by the same distance.

After clicking the menu item or pressing the keyboard shortcut, move the mouse down to
shrink or up to fatten. (Alternatively, type a distance on the keyboard.) Then
press LMB or Return to confirm or RMB or Esc to cancel.

.. list-table::
   :widths: 1 1 1

   * - .. figure:: /images/modeling_meshes_editing_mesh_transform_shrink-fatten_before.png

          Mesh before shrink/fatten.

     - .. figure:: /images/modeling_meshes_editing_mesh_transform_shrink-fatten_inflate-negative.png

          Shrunk using a negative value.

     - .. figure:: /images/modeling_meshes_editing_mesh_transform_shrink-fatten_inflate-positive.png

          Fattened using a positive value.

Options
=======

These settings can be changed in the bpy.ops.screen.redo_last panel.

Offset
   How far to move each vertex.

Offset Even
   When disabled, each vertex is offset by the same distance.
   When enabled, each *edge* is offset by the same distance.

   This option can also be toggled before confirming by pressing S or holding Alt.
   (These shortcuts are shown in the status bar.)

.. list-table::

   * - .. figure:: /images/modeling_meshes_editing_face_extrude-faces-normal_offset-even-off.png

          Fattening with Offset Even disabled.

     - .. figure:: /images/modeling_meshes_editing_face_extrude-faces-normal_offset-even-on.png

          Fattening with Offset Even enabled.

Mirror Editing
   Whether to also affect matching geometry on the other side of the mesh.
   Mesh Symmetry
   needs to be enabled for this to work.

Proportional Editing
   See Proportional Editing </editors/3dview/controls/proportional_editing>.
