---
area: manual-editors
source: blender-manual
version: 5.2
---

*****
Track
*****

.. _bpy.ops.nla.tracks_add:

Add
===

Adds a new track below the Action Track.

Add Above Selected
==================

Adds a new track above each selected one.

.. _bpy.ops.nla.tracks_delete:

Delete Tracks
=============

Deletes the selected tracks and the strips they contain.

When using the keyboard shortcuts, make sure the mouse cursor is hovering over the track region,
as otherwise, Blender will only delete the selected strips.

Move
====

To Top Shift-PageUp
   Moves the selected tracks to the top.
Up PageUp
   Moves the selected tracks up by one.
Down PageDown
   Moves the selected tracks down by one.
To Bottom Shift-PageDown
   Moves the selected tracks to the bottom.

When using the keyboard shortcuts, make sure the mouse cursor is hovering over the track region,
as otherwise, Blender will move the selected strips to different tracks (rather than moving the tracks).

.. _bpy.ops.anim.channels_clean_empty:

Remove Empty Animation Data
===========================

Removes objects that don't have drivers, NLA tracks, or an active action
from the NLA editor to reduce clutter.
This essentially does the opposite of Add --> Selected Objects.
