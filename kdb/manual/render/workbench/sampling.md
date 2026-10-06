---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.SceneDisplay.render_aa:
.. _bpy.types.SceneDisplay.viewport_aa:

********
Sampling
********

The quality of the renders can be adjusted by changing the Anti-Aliasing method.
A different one can be selected for the 3D Viewport, viewport rendering and
for final rendering.

The setting for the 3D Viewport is a user preference to specify the anti-aliasing method
that runs best on the used system. The setting for viewport rendering
and final rendering is saved per scene.

No Anti-Aliasing
   With this option selected no anti-aliasing will be applied.

Single Pass Anti-Aliasing
   Scene will be rendered with a post-process anti-aliasing pass.

Multisample
   The scene will be rendered multiple times with a slight offset.
   The anti-aliasing will be gathered from the multiple renders.
   The number of samples are predefined so it uses the best distribution of the samples.

   5, 8, 11, 16, 32

**Tip:**

      Multisample anti-aliasing is well suited for rendering small details like hair.

   *Progressive Viewport Rendering*

   For the 3D Viewport, one sample is rendered at a time. When there are no changes
   to the scene or viewport the next sample will be rendered.
