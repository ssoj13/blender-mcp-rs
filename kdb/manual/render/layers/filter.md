---
area: manual-render
source: blender-manual
version: 5.2
---

******
Filter
******

These options control which types of data are included in the final render for the current view layer.

.. _bpy.types.ViewLayer.use_sky:

Include
   Environment Cycles, EEVEE
      Disables rendering the *Environment* render pass in the final render.

   .. _bpy.types.ViewLayer.use_solid:

   Surfaces Cycles, EEVEE
      Disables rendering object materials in the final render.

   .. _bpy.types.ViewLayer.use_strand:

   Curves Cycles, EEVEE
      Disables rendering curve strands in the final render.

   .. _bpy.types.ViewLayer.use_volumes:

   Volume Cycles, EEVEE
      Disables rendering Volumes </modeling/volumes/index> in the final render.

   .. _bpy.types.ViewLayer.use_grease_pencil:

   Grease Pencil Cycles, EEVEE
      Render Grease Pencil objects on current layer

.. _bpy.types.ViewLayer.use_motion_blur:

Use
   Motion Blur Cycles, EEVEE
      Render motion blur for current layer,
      if enabled in the Render Settings <bpy.types.RenderSettings.use_motion_blur>.
