---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.RenderSettings.use_motion_blur:

***********
Motion Blur
***********

Blender's animations are by default rendered as a sequence of *perfectly still* images.
While great for stop-motion and time-lapses, this is unrealistic, since fast-moving
objects do appear to be blurred in the direction of motion,
both in a movie frame and in a photograph from a real-world camera.

.. _bpy.types.RenderSettings.motion_blur_position:

Position
   Controls at what point the shutter opens in relation to the current frame.

   :Start on Frame: Shutter is starting to open at the current frame.
   :Center on Frame: Shutter is fully opened at the current frame.
   :End on Frame: Shutter is fully closed at the current frame.

.. _bpy.types.RenderSettings.motion_blur_shutter:

Shutter
   Time (in frames) between when the shutter starts to open and fully closed.
   For example, shutter time 1.0 blurs over the length of 1 frame.

.. _bpy.types.CyclesRenderSettings.rolling_shutter_type:

Rolling Shutter
   Creates a Rolling Shutter effect.

   :None: No rolling shutter effect.
   :Top-Bottom: Renders rolling shutter from the top of the image to the bottom.

.. _bpy.types.CyclesRenderSettings.rolling_shutter_duration:

Rolling Shutter Duration
   Controls balance between pure rolling shutter effect (if the value is zero)
   and pure motion blur effect (if the value is one).

**Note:**

   If there are particles or other physics system in a scene,
   be sure to bake them before rendering,
   otherwise you might not get correct or consistent motion blur.

**Seealso:**

   Each object has its own settings to control motion blur.
   These options can be found in the Object tab of the Properties.
   See object setting <bpy.types.CyclesObjectSettings.use_motion_blur> for more information.

.. _bpy.ops.render.shutter_curve_preset:

Shutter Curve
=============

Curve defining how the shutter opens and closes.
The X axis is time, Y values of 0 mean fully closed shutter, Y values of 1 mean fully opened shutter.
The default mapping is set to when shutter opens and closes instantly.

Limitations
===========

- Camera motion blur does not work for Orthographic Cameras </render/cameras>.
- Motion blur does not take into account the movement of Lights </render/lights/light_object>
  in world coordinates, which means lights are essentially stationary during rendering of this frame,
  and moving the camera together with lights will wrongly produce motion blur.
