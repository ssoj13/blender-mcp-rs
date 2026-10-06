---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

*************
Scene Project
*************

Projects vertices of the active object onto other visible meshes in the scene.

This provides more flexible and localized control than the
Shrinkwrap modifier </modeling/modifiers/deform/shrinkwrap>,
making it useful for manually conforming geometry to a target surface.

The brush projects geometry during the stroke, allowing surfaces to be
adjusted interactively while sculpting.

Brush Settings
==============

General
-------

**Note:**

   More info at sculpt-tool-settings-brush-settings-general brush settings
   and on sculpt-tool-settings-brush-settings-advanced brush settings.

Unique
------

.. _bpy.types.Brush.project_ray_direction_type:

Ray Direction Ctrl
   Direction used when projecting vertices onto scene geometry.

   :View Normal: Projects vertices along the view direction.
   :Plane Normal: Projects vertices along the brush plane normal.

.. _bpy.types.Brush.minimum_distance:

Minimum Distance
   Minimum distance maintained between the sculpted surface and the
   target geometry after projection.

.. _bpy.types.Brush.use_bidirectional:

Bidirectional
   Projects vertices both along the projection direction and its inverse,
   using the closest valid intersection.

   This can help when sculpting around thin or double-sided geometry.
