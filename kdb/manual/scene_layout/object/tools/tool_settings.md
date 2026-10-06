---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

*************
Tool Settings
*************

Options
=======

Transform
---------

.. _bpy.types.ToolSettings.use_transform_data_origin:
.. _bpy.types.ToolSettings.use_transform_pivot_point_align:

Affect Only
   Origins Ctrl-Period
      Directly transforms the object's origin </scene_layout/object/origin>.
      This only works for objects with data which can be transformed;
      i.e. it will not work on object lights.

      When enabled, the object axes are displayed.

      Take care using this option since it transforms the object-data which may cause linked
      duplicates to be moved unintentionally.

**Hint:**

         Changing the object location and the object-data may impact
         modifiers, constraints and keyframe animation.

         If you are only temporarily setting the pivot point,
         use the 3D cursor <editors-3dview-3d_cursor> instead.

   Locations
      Changes the position of the object's origin relative to another point during transformation.
      In other words, the pivot point and the origin cannot share the same location.
      This will not affect the object local transforms, just its position in world space.

      In the examples below, a comparison of the scaling and rotation of objects,
      when *Location* is enabled (middle) and disabled (right).

   Parents
      Transforms Parent Objects
      while leaving their children objects unaffected.
