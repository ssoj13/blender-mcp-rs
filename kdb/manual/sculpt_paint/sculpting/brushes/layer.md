---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

*****
Layer
*****

This brush is similar to Draw </sculpt_paint/sculpting/brushes/clay>,
except that the height capped.
This creates the appearance of a flat layer.

It is recommended to use the Persistent <bpy.types.Brush.use_persistent> setting
and regularly Set Persistent Base <bpy.ops.sculpt.set_persistent_base>,
so that multiple strokes to not add on top of each other.

Brush Settings
==============

General
-------

Hardness
   Higher by default to ensure the profile of layers is more noticeable.
   More info at Hardness

**Note:**

   More info at sculpt-tool-settings-brush-settings-general brush settings
   and on sculpt-tool-settings-brush-settings-advanced brush settings.

Unique
------

.. _bpy.types.Brush.height:

Height
   The fixed height of each stroke. This is measured using the scene scale <bpy.types.UnitSettings.system>,
   so it is consistent no matter the amount of zoom or object size.

.. _bpy.types.Brush.use_persistent:

Persistent
   This will ensure that multiple strokes use the same height, as if sculpting a single layer.

.. _bpy.ops.sculpt.set_persistent_base:

Set Persistent Base
   This button resets a new base so that you can sculpt new layer.
