---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.ops.render.play_rendered_anim:

.. |numsp| unicode:: U+2007

****************
Animation Player
****************

The animation player is a utility typically used for previewing rendered animations,
supporting all image and video formats also supported by Blender.
This is a convenient way to play back image sequences at the correct frame rate.

Launching the animation player opens a new window,
playing back images or a video located at the render output of the current scene.
You can also drop images or movie files in a running animation player.
It will then restart the player with the new data.

**Tip:**

   An external player can also be used instead of the one included in Blender.
   To do this, select it in the Preferences </editors/preferences/file_paths>.

Player Options
==============

Ping Pong
   When enabled, playback loops forwards than backwards.
X/Y Flip
   Flip the image horizontally or vertically.

   *Viewing the animation from a different perspective can help you see the animation with "fresh eyes".*

Hotkeys
=======

The following table shows the available hotkeys for the animation player.

.. list-table::
   :header-rows: 1

   * - Action
     - Hotkey
   * - Start/Pause:
     - Spacebar
   * - Start playback (when paused):
     - Return
   * - Quit:
     - Esc

.. list-table::
   :header-rows: 1

   * - Action
     - Hotkey
   * - Scrub in time:
     - LMB
   * - Step back one frame:
     - Left
   * - Step forward one frame:
     - Right
   * - Step back 10 frames:
     - Down
   * - Step forward 10 frames:
     - Up
   * - Manual frame stepping:
     - NumpadPeriod

.. list-table::
   :header-rows: 1

   * - Action
     - Hotkey
   * - Backward playback:
     - Shift-Down
   * - Forward playback
     - Shift-Up
   * - Slow down playback:
     - NumpadMinus
   * - Speed up playback:
     - NumpadPlus
   * - Toggle looping:
     - Numpad0
   * - Toggle frame skipping:
     - A
   * - Toggle ping-pong:
     - P

.. list-table::
   :header-rows: 1

   * - Action
     - Hotkey
   * - Toggle Playhead (Indicator):
     - I
   * - Flip image on the X axis:
     - F
   * - Flip image on the Y axis:
     - Shift-F
   * - Hold to show frame numbers:
     - Shift
   * - Zoom in:
     - Ctrl-NumpadPlus
   * - Zoom out:
     - Ctrl-NumpadMinus

.. list-table::
   :header-rows: 1

   * - Action
     - Hotkey
   * - 60 fps
     - Numpad1
   * - 50 fps
     - Numpad2
   * - 30 fps
     - Numpad3
   * - 25 fps
     - Numpad4
   * - 24 fps
     - Shift-Numpad4
   * - 20 fps
     - Numpad5
   * - 15 fps
     - Numpad6
   * - 12 fps
     - Numpad7
   * - 10 fps
     - Numpad8
   * - |numsp|\ 6 fps
     - Numpad9
   * - |numsp|\ 5 fps
     - NumpadSlash

Frame Cache
===========

Image files are cached during playback for faster access.

While loading images is rarely a bottleneck,
there are situations where high resolution images may slow down playback causing frame skipping.

**Seealso:**

   Memory Cache Limit <bpy.types.PreferencesSystem.memory_cache_limit> preference to control this limit,
   which may be increased to cache more images during playback.
   command-line-args-animation-playback-options to specify this value when launching from the command line.
