---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.RenderSettings.use_simplify:

********
Simplify
********

.. _bpy.types.RenderSettings.simplify_subdivision:

Max Subdivision
   Maximum number of subdivision by the Subdivision Surface modifiers.

.. _bpy.types.RenderSettings.simplify_child_particles:

Child Particles
   Show only a subset of all child hairs and particles.

.. _bpy.types.CyclesRenderSettings.texture_resolution:

Texture Resolution
   Scale down image textures to reduce memory usage.

   With the Texture Cache
   and viewport rendering of complex scenes, a lower resolution can help ensure
   all textures fit in memory for interactive updates.

.. _bpy.types.CyclesRenderSettings.texture_limit:

Texture Size Limit
   Automatically scales textures down so that they are no larger than the values chosen.
   This can help reduce computer memory resources when rendering large scenes with huge textures.

Viewport
========

See Common Settings above.

.. _bpy.types.RenderSettings.simplify_volumes:

Volume Resolution
   Resolution percentage of volume objects </modeling/volumes/index> in the viewport.
   This mostly affects memory usage rather than computation times.

.. _bpy.types.RenderSettings.use_simplify_normals:

Normals
   Skip computing custom normals
   and face corner normals for displaying meshes in the viewport.

Render
======

See Common Settings above.

.. _render-cycles-settings-scene-simplify-culling:

Culling
=======

.. _bpy.types.CyclesRenderSettings.camera_cull_margin:
.. _bpy.types.CyclesRenderSettings.use_camera_cull:

Camera Cull
   Automatically culls objects based on the camera frustum defined by the *Margin*.

.. _bpy.types.CyclesRenderSettings.distance_cull_margin:
.. _bpy.types.CyclesRenderSettings.use_distance_cull:

Distance Cull
   Automatically culls objects based on their distance from the active camera.
   This is set via the *Distance* property.

Grease Pencil
=============

.. _bpy.types.RenderSettings.simplify_gpencil_onplay:

Playback Only
   Activates the simplification process only during animation playback.

.. _bpy.types.RenderSettings.simplify_gpencil_view_fill:

Fill
   Shows the fill component in Grease Pencil materials.

.. _bpy.types.RenderSettings.simplify_gpencil_modifier:

Modifiers
   Shows Grease Pencil modifiers </grease_pencil/modifiers/index>.

.. _bpy.types.RenderSettings.simplify_gpencil_shader_fx:

Shader Effects
   Shows Grease Pencil visual effects </grease_pencil/modifiers/index>.

.. _bpy.types.RenderSettings.simplify_gpencil_tint:

Layer Tinting
   Shows layers tint overrides.

.. _bpy.types.RenderSettings.simplify_gpencil_antialiasing:

Anti-Aliasing
   Use Anti-Aliasing to smooth stroke edges. The amount of anti-aliasing can be adjusted by
   the SMAA Threshold <bpy.types.SceneGpencil.antialias_threshold>.
