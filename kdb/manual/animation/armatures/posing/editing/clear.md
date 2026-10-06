---
area: manual-animation
source: blender-manual
version: 5.2
---

***************
Clear Transform
***************

Once you have transformed some bones, if you want to return to their rest position,
just clear their transformations.

.. _bpy.ops.pose.transforms_clear:

All
   Resets location, rotation, and scaling of selected bones to their default values.

.. _bpy.ops.pose.loc_clear:
.. _bpy.ops.pose.rot_clear:
.. _bpy.ops.pose.scale_clear:

Location, Rotation, Scale Alt-G, Alt-R, Alt-S
   Clears individual transforms.

   Note that in *Envelope* visualization, Alt-S does not clear the scale,
   but rather scales the *Distance* influence area of the selected bones.
   (This is also available through the Pose --> Scale Envelope Distance menu entry,
   which is only effective in *Envelope* visualization, even though it is always available...)

.. _bpy.ops.pose.user_transforms_clear:

Reset Unkeyed
   Clears the transforms to their keyframe state.

   Only Selected
      Operate on just the selected or all bones.
