---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

.. _bpy.ops.object.*clear:

*****
Clear
*****

Clearing transforms resets the transform values.
The objects location and rotation values are set to 0, and the scale to 1.

Clear Location Alt-G
   Clear (reset) the location of the selection.
   This will move the selection back to the coordinates (0, 0, 0).
Clear Scale Alt-S
   Clear (reset) the scale of the selection.
   This will change the scale to (1, 1, 1).
Clear Rotation Alt-R
   Clear (reset) the rotation of the selection.
   This will set the rotation of the selection to 0 degrees in each plane.

.. _bpy.ops.object.origin_clear:

Clear Origin
   Clears (resets) the offset of the child objects origin from
   the Parent </scene_layout/object/editing/parent>.
   This will cause child objects to move to the origin of the parent.
   The relationship between the parent and child is not affected,
   you can confirm the relationship is still intact by using the *Outliner* to
   verify that the child object is still parented.

Options
=======

Clear Delta
   Clear the delta transform <bpy.types.Object.delta> in addition to clearing the primary transforms.
   (Appears in the bpy.ops.screen.redo_last panel.)
