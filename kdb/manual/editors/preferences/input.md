---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.types.PreferencesInput:

*****
Input
*****

In the Input preferences, you can customize how Blender reacts to the mouse and keyboard
as well as define your own keymap.

Keyboard
========

.. _bpy.types.PreferencesInput.use_emulate_numpad:

Emulate Numpad
   The Numpad keys are used quite often in Blender and are not assigned to the same action as
   the regular number keys. If you have a keyboard without a Numpad (e.g. on a laptop),
   you can tell Blender to treat the standard number keys as Numpad keys by checking *Emulate Numpad*.

.. _bpy.types.PreferencesInput.use_numeric_input_advanced:

Default to Advanced Numeric Input
   For transform mode, default to transform-numeric-input-advanced,
   otherwise transform-numeric-input-simple is used.

Mouse
=====

.. _bpy.types.PreferencesInput.use_mouse_emulate_3_button:

Emulate 3 Button Mouse
   Blender can be configured to work with pointing devices which do not have an MMB.
   The functionality of the three mouse buttons by holding Alt-LMB.

   Mouse/Keyboard combinations referenced in this manual
   can be expressed with the combinations shown in the table. For example:

   MMB drag becomes Alt-LMB drag for example.

**Warning:**

      This option prevents certain features from being accessed,
      since Alt-LMB is used for some operations.

      - Modifying multiple items values at once (objects, bones... etc).
      - Deselecting edge/face rings in Edit Mode.
      - Detaching node links.
      - Moving the Compositor background image.

      Some touchpads support three-finger tap for middle mouse button,
      which may be an alternative to using this option.

   .. _bpy.types.PreferencesInput.mouse_emulate_3_button_modifier:

   Modifier
      The modifier key to press to emulate the middle mouse keybindings.
      This option is unsupported on Microsoft Windows.

      :Alt:
         Use the Alt key to emulate the middle mouse button.
      :OSKey:
         Use the OSKey to emulate the middle mouse button.

         This has the advantage that it doesn't conflict with existing Alt-MMB shortcuts,
         noted above.

.. _bpy.types.PreferencesInput.use_mouse_continuous:

Continuous Grab
   This feature is used to prevent the problem where an action such as moving objects or panning a view,
   is limited by your screen bounds.

   This is done by warping the mouse within the view.

**Note:**

      Cursor warping is only supported by *relative* input devices (mouse, trackball, trackpad).

      Graphics tablets, however, typically use *absolute* positioning,
      this feature is disabled when a tablet is being used.

      This is detected for each action,
      so the presence of a tablet will not disable *Continuous Grab* for mouse cursor input.

.. _bpy.types.PreferencesInput.use_drag_immediately:

Release Confirms
   Dragging LMB on an object will move it.
   To confirm this (and other) transform, an LMB is necessary by default.
   When this option is activated, the release of LMB acts as confirmation of the transform.

.. _bpy.types.PreferencesInput.mouse_double_click_time:

Double Click Speed
   The time in milliseconds to trigger a double click.

.. _bpy.types.PreferencesInput.drag_threshold_mouse:

Mouse Drag Threshold
   The number of pixels that a User Interface element has to be moved before it is recognized by Blender,
   values below this will be detected as click events.

.. _bpy.types.PreferencesInput.drag_threshold_tablet:

Tablet Drag Threshold
   The drag threshold for tablet events.

.. _bpy.types.PreferencesInput.drag_threshold:

Drag Threshold
   The drag threshold for non mouse/tablet events (keyboard or NDOF for example).

   This affects Pie Menu on Drag <keymap-pref-py_menu_on_drag> keymap preference.

.. _bpy.types.PreferencesInput.move_threshold:

Motion Threshold
   The number of pixels the cursor must be moved before the movement is registered.
   This is helpful for tablet pens that are a lot more difficult to keep still,
   then this could help to reduce stuttering of the cursor position.

**Note:**

      Unlike the click/drag distinction, this is used to detect small movements
      for example, picking selection cycles through elements near the cursor.
      Once the cursor moves past this threshold, selection stops cycling and picks the closest item.

Touchpad
========

**Note:**

   This panel is available on Windows, macOS, and Linux with Wayland.

.. _bpy.types.PreferencesInput.use_multitouch_gestures:

Multi-touch Gestures
   Use multi-touch gestures for navigation with touchpad, instead of scroll wheel emulation.
   For more detail on supported gestures,
   see Configuring Peripherals </getting_started/configuration/hardware>.

.. _bpy.types.PreferencesInput.touchpad_scroll_direction:

Scroll Direction
   The direction scrolling responds to the scroll gestures.

   Only available on Linux using Wayland.

   :Traditional: Scrolls content down when gestures move up.
   :Natural: Scrolls content up when gestures move up.

Tablet
======

.. _bpy.types.PreferencesInput.tablet_api:

Tablet API (Windows only)
   Select the native Windows Ink or older Wintab system for pressure sensitivity.
   Blender automatically selects the API for your operating system and tablet,
   however in case of problems this can be set manually.
   You may need to restart Blender for changes to take affect.

.. _bpy.types.PreferencesInput.pressure_threshold_max:

Max Threshold
   Amount of pressure required to achieve full intensity.

.. _bpy.types.PreferencesInput.pressure_softness:

Softness
   Controls how the softness of the low pressure response onset using a gamma curve.

.. _editors_preferences_input_ndof:

NDOF
====

These preferences control how an NDOF device <hardware-ndof> (3D mouse) interacts with the 3D Viewport.
These settings allow customization of navigation, orbit behavior, and motion sensitivity.
They can also be accessed using the NDOFMenu button on supported devices,
which opens a pop-up menu to adjust them directly in the viewport.

.. _bpy.types.PreferencesInput.ndof_navigation_mode:

Navigation Mode
   Sets how the 3D mouse navigates in the 3D Viewport.

   :Object:
      Feels like holding the object in your hand. Moving the 3D mouse moves the object in that direction.
   :Fly:
      Moves the camera through the scene, like flying or piloting a helicopter.
      For example, pushing the 3D mouse up moves the camera up.
   :Drone:
      Enables a Fly Mode navigation but pushing the cap forward
      while looking down will not change the altitude of the camera.

      "Lock Horizon" is always used in this mode.

.. _bpy.types.PreferencesInput.ndof_lock_horizon:

Lock Horizon
   Keeps the view level by preventing horizon tilt during navigation.

.. _bpy.types.PreferencesInput.ndof_orbit_center_auto:

Orbit Center -- Auto
   Automatically determines the rotation center. If the full model is visible,
   its center of volume is used. When zoomed in, the rotation center shifts to the nearest visible object.

.. _bpy.types.PreferencesInput.ndof_orbit_center_selected:

Orbit Center -- Selected Items
   Limits the orbit center to the center of the currently selected objects.

.. _bpy.types.PreferencesInput.ndof_show_guide_orbit_axis:

Show -- Orbit Axis
   Displays an axis overlay to indicate the current orbit rotation direction.

.. _bpy.types.PreferencesInput.ndof_show_guide_orbit_center:

Show -- Orbit Center
   Displays a marker showing the current orbit center point.

Advanced
--------

.. _bpy.types.PreferencesInput.ndof_translation_sensitivity:

Pan Sensitivity
   Controls how quickly the view pans in response to NDOF input.

.. _bpy.types.PreferencesInput.ndof_rotation_sensitivity:

Orbit Sensitivity
   Controls how quickly the view orbits in response to NDOF input.

.. _bpy.types.PreferencesInput.ndof_deadzone:

Deadzone
   Sets the minimum threshold for motion detection. Helps avoid unintended movement from slight touches.

.. _bpy.types.PreferencesInput.ndof_zoom_direction:

Zoom Direction
   Determines which direction on the 3D mouse triggers zooming.

   :Forward/Backward:
      Zooms in or out by pushing or pulling the 3D mouse forward/backward.
   :Up/Down:
      Zooms in or out by pushing or pulling the 3D mouse upward/downward.

.. _bpy.types.PreferencesInput.ndof_panx_invert_axis:
.. _bpy.types.PreferencesInput.ndof_pany_invert_axis:
.. _bpy.types.PreferencesInput.ndof_panz_invert_axis:

Invert Pan
   Inverts panning on the selected X, Y, or Z axis.

.. _bpy.types.PreferencesInput.ndof_rotx_invert_axis:
.. _bpy.types.PreferencesInput.ndof_roty_invert_axis:
.. _bpy.types.PreferencesInput.ndof_rotz_invert_axis:

Invert Rotate
   Inverts rotation direction on the selected X, Y, or Z axis.

.. _bpy.types.PreferencesInput.ndof_lock_camera_pan_zoom:

Pan / Zoom Camera View
   When in camera view, pans or zooms the camera instead of exiting the view during orbiting.
