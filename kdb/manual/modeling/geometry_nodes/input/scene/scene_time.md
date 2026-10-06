---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputSceneTime:
.. --- copy below this line ---

***************
Scene Time Node
***************

The *Scene Time* node outputs the current time in the scene's animation in units of seconds or frames.

Inputs
======

This node has no inputs.

Outputs
=======

Seconds
   Current scene time in seconds.

Frames
   Current scene frame.
   As an input in geometry nodes, this output may also output non-round numbers,
   in order to support higher quality motion blur </render/eevee/render_settings/motion_blur>.
