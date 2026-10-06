---
area: manual-editors
source: blender-manual
version: 5.2
---

********
Timeline
********

The *Timeline* editor is used to jump to different frames, manipulate keyframes,
and control animation playback.

Main View
=========

The X axis represents time, with the numbers 0/50/100/... being frame numbers.
The blue line is the *Playhead* indicating the current frame,
and the diamond shapes are *Keyframes*, points where you specified
a certain value for a certain property at a certain time.

Adjusting the View
------------------

Panning is done by dragging MMB.

Zooming is done by dragging Ctrl-MMB, rolling the mouse Wheel,
or pressing NumpadMinus/NumpadPlus.

You can also use the scrollbars located at the bottom and the right of the editor.

Frame Range
-----------

The *Frame Range* determines the length of the scene's animation.
By default, it's set to start at frame 1 and end at frame 250.
You can change this using the Start/End inputs in the Timeline header,
or in the Output Properties </render/output/properties/frame_range>.

Keyframes
---------

By default, the timeline only shows keyframes for selected items.
You can make it show all keyframes by unchecking
View --> Only Show Selected.

You can click a keyframe to select it (and deselect all others),
or click it while holding Shift to add it to the selection
(or remove it if it was already selected). You can also drag a box
to select multiple keyframes in one go.

To move the selected keyframes, simply drag one of them. Alternatively,
you can press G, move the mouse, and click LMB to confirm
(or RMB to cancel). You can also press S to scale the keyframes
in relation to the Playhead.

Markers
-------

See the Markers page </animation/markers> for more information.

Header
======

.. _animation-editors-timeline-headercontrols:

.. _timeline-view-menu:

View Menu
---------

Adjust Last Operation
   Displays a pop-up panel to alter properties of the last
   completed operation. See bpy.ops.screen.redo_last.
Channels
   Show or hide the Channels region (the tree of objects and animatable properties on the left).

----------

Frame All Home
   Pans and zooms the view so that all keyframes are visible.

.. _bpy.ops.anim.scene_range_frame:

Frame Scene/Preview Range
   Reset the horizontal view to the current scene frame range,
   taking the preview range into account if it is active.
Go to Current Frame Numpad0
   Centers the Timeline to the Playhead.

----------

Show Markers
   Shows the Markers region (if any markers are defined).
   When disabled, the `Marker Menu`_ is also hidden and marker operators are not
   available in this editor.
Use Timecode Ctrl-T
   Shows the time on the X axis and the *Playhead* as timestamps instead of frame numbers.
   A timestamp such as `01:03+02` means "1 minute, 3 seconds, 2 frames."
Sync Visible Range
   Synchronizes the horizontal panning and scale of the editor
   with other time-based editors that also have this option enabled.
   That way, they always show the same section of time.

----------

Only Show Selected
   Only show keyframes related to the selected items.
   This could be objects, bones, nodes, and so on.

**Note:**
   If this option is enabled, the Timeline may not show all material
   keyframes of the selected objects. Instead, it only shows the keyframes belonging to the selected nodes
   in the /editors/shader_editor.

Only Show Errors
   Only show curves and drivers that are disabled or have errors.
   Useful for debugging.

----------

Cache
   Which simulation caches to show on the timeline.
   See bpy.types.SpaceDopeSheetEditor.show_cache for more information.

----------

Area
   Area controls. See the user interface
   documentation for more information.

Marker Menu
-----------

Markers </animation/markers> are used to denote frames with key points or significant events
within an animation. Like in most animation editors, they're shown at the bottom of the Timeline.

For descriptions of the different marker tools, see Editing Markers <animation-markers-editing>.

----------

Playback Popover
   Properties for how animations are played.
   See animation-editors-playback for more information.

Keying Popover
   Properties for how keyframes are added.
   See animation-editors-keying for more information.

Auto Keying
   Options for automatically adding keyframes.
   See bpy.types.ToolSettings.use_keyframe_insert_auto for more information.

Transport Controls
   These buttons are used to control playback.
   See animation-editors-controls for more information.

Frame Controls
   Used to set the current frame and the start/end frame.
   See animation-editors-frame_controls for more information.
