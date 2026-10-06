---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

.. _bpy.ops.view3d.snap:

****
Snap
****

The *Snap* menu provides tools for moving the selection or the
3D Cursor </editors/3dview/3d_cursor> to precise positions.

These operations are commonly used to align objects, place the cursor
for transformations, or position elements relative to the grid or other objects.

.. _bpy.ops.view3d.snap_selected_to_grid:

Selection to Grid
=================

Snaps the selected elements to the nearest grid point.

In *Object Mode*, each selected object origin is moved to the closest grid
intersection. In *Edit Mode*, the selected vertices, edges, or faces are moved
to the nearest grid location.

This is useful for quickly aligning objects or geometry to the grid.

.. _bpy.ops.view3d.snap_selected_to_cursor:

Selection to Cursor
===================

Moves the selected elements to the location of the 3D Cursor </editors/3dview/3d_cursor>.

Optionally, the selection can also be rotated to match the cursor's
orientation.

Selection to Cursor (Offset)
============================

Moves the selection so that its center aligns with the 3D Cursor,
while preserving the relative positions between selected elements.

Unlike *Selection to Cursor*, the selected objects are not stacked
directly on the cursor. Instead, the entire selection is offset so that
its center is placed at the cursor location.

.. _bpy.ops.view3d.snap_selected_to_active:

Selection to Active
===================

Moves the selected elements to the origin of the Active object.

The active object itself is not moved. All other selected objects
are aligned to its origin.

.. _bpy.ops.view3d.snap_cursor_to_selected:

Cursor to Selected
==================

Places the 3D Cursor </editors/3dview/3d_cursor> at the center of the current selection.

The exact location depends on the current pivot-point-index.

For example:

- With the *Bounding Box Center* pivot point active,
  *Cursor to Selected* places the cursor at the center of the
  bounding box surrounding the selected objects' origins.

- With the *Median Point* pivot point active,
  *Cursor to Selected* places the cursor at the
  median </editors/3dview/controls/pivot_point/median_point>
  of the selected object origins.

.. _bpy.ops.view3d.snap_cursor_to_center:

Cursor to World Origin
======================

Moves the 3D Cursor to the world origin at coordinates `(0, 0, 0)`.

**Seealso:**

   cursor-center-and-frame which moves the 3D Cursor to the world origin and reframes the 3D Viewport.

.. _bpy.ops.view3d.snap_cursor_to_grid:

Cursor to Grid
==============

Moves the 3D Cursor to the nearest grid overlay point.

.. _bpy.ops.view3d.snap_cursor_to_active:

Cursor to Active
================

Places the 3D Cursor at the origin of the Active object`
(the last selected object).
