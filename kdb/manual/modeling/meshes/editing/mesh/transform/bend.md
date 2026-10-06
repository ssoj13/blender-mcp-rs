---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.transform.bend:

****
Bend
****

This transformation bends part of the selection into a circle segment. It's similar to
/modeling/meshes/editing/mesh/transform/warp, except that here, the 3D Cursor
is on the circle instead of at the center.

.. list-table::
   :widths: 1 1 1

   * - .. figure:: /images/modeling_meshes_editing_mesh_transform_bend_example-clamp-1.png

          Before.

     - .. figure:: /images/modeling_meshes_editing_mesh_transform_bend_example-clamp-2.png

          Clamp on.

     - .. figure:: /images/modeling_meshes_editing_mesh_transform_bend_example-clamp-3.png

          Clamp off.

Usage
=====

#. Align the 3D Viewport's viewpoint </editors/3dview/navigate/viewpoint> to the plane
   in which the bend should happen.
#. Place the /editors/3dview/3d_cursor at the base of the bend. The geometry around
   this point will stay in place.
#. Place the mouse cursor so that the line between it and the 3D Cursor lies along the
   geometry to bend.
#. Press Shift-W to activate the tool and move the mouse to bend the geometry.

   - The distance between the mouse cursor and the 3D Cursor determines the bend radius
     and how much of the selected geometry is bent (instead of just rotated).
     Hold Alt to disable clamping and bend all selected geometry instead.
   - The angle between the original line and the new line determines the bend angle.

#. Press LMB or Return to confirm, or RMB or Esc to cancel.

**Note:**

   Unlike most other transform tools, *Bend* does not take into account the
   transform orientation </editors/3dview/controls/orientation> or the
   pivot point </editors/3dview/controls/pivot_point/index>, instead always
   using the view plane and the 3D Cursor.

**Hint:**

   Moving the mouse cursor around the 3D Cursor multiple times will correspondingly
   bend the geometry over multiple circle revolutions.

Options
=======

Unlike other operators, the Bend operator doesn't support the
bpy.ops.screen.redo_last panel.

Example
=======
