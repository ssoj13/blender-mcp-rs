---
area: manual-movie_clip
source: blender-manual
version: 5.2
---

****************
Selecting Tracks
****************

.. _bpy.ops.clip.select_all:

All A
   Selects all items.

None Alt-A
   Deselects all items.

Inverse Ctrl-I
   Selects non-selected items and deselects existing selection.

.. _bpy.ops.clip.select_box:

Box Select
==========

See tool-select-box.

.. _bpy.ops.clip.select_circle:

Circle Select
=============

See tool-select-circle.

.. _bpy.ops.clip.select_lasso:

Lasso Select
============

See tool-select-lasso.

.. _bpy.ops.clip.select_grouped:

Select Grouped
==============

Select all tracks from specified group.

Action
   The group of tracks to select.

   :Keyframed Tracks: Select all keyframed tracks.
   :Estimated Tracks: Select all estimated tracks.
   :Tracked Tracks: Select all tracked tracks.
   :Locked Tracks: Select all locked tracks.
   :Disabled Tracks: Select all disabled tracks.
   :Track with Same Color: Select all tracks with same color as active track.
   :Failed Tracks: Select all tracks which failed to be reconstructed.

.. _bpy.ops.clip.stabilize_2d_select:

Select Stabilization Tracks
===========================

Select tracks which are used for translation stabilization.

.. _bpy.ops.clip.stabilize_2d_rotation_select:

Select Stabilization Rotation Tracks
====================================

Select tracks which are used for rotation stabilization.
