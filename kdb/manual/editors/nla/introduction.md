---
area: manual-editors
source: blender-manual
version: 5.2
---

************
Introduction
************

The NonLinear Animation editor, or NLA editor for short, lets you animate on a higher level.
Instead of working with individual keyframes, it works with actions </animation/actions>,
which are named, reusable animation segments.

Main Region
===========

The editor displays a stack of tracks </editors/nla/tracks> which work like layers in an image editing
program. Higher tracks take precedence over lower ones, although you can also choose to blend them.

Each track can contain any number of strips </editors/nla/strips> -- typically Action Strips,
which are instances of actions.

The top track highlighted in orange is special: this is the Action Track. Unlike the other tracks,
it doesn't contain strips -- instead, it contains the object's *active action*,
which is where new keyframes are added to by default.

Editors like the Dope Sheet Editor </editors/dope_sheet/introduction>
normally only show the keyframes of this active action. If you want to edit another action,
you can select it in the NLA editor and press Tab to enter Tweak Mode.

Header
======

View Menu
---------

Sidebar N
   Shows or hides the Sidebar Region <ui-region-sidebar>.
Adjust Last Operation
   Displays a pop-up panel to alter properties of the last
   completed operation. See bpy.ops.screen.redo_last.
Channels
   Shows or hides the Track Region.
Playback Controls
   Show or hide the Playback Controls <animation-editors-footer>.

----------

Frame Selected NumpadPeriod
   Pans and zooms the view to focus on the selected strips.
Frame All Home
   Pans and zooms the view to show all strips.
Frame Scene/Preview Range
   Reset the horizontal view to the current scene frame range,
   taking the preview range into account if it is active.
Go to Current Frame Numpad0
   Centers the view on the Playhead.

----------

Realtime Updates
   Whether to update other views (such as the 3D Viewport) while you're moving strips around.
   If disabled, the other views only get updated once you finish the move.
Show Control F-Curves
   Shows a graph on top of each strip that uses Animated Influence <bpy.types.NlaStrip.influence>.

----------

Show Markers
   Shows the marker region (provided any markers have been defined).
   When disabled, the `Marker Menu`_ is also hidden and marker operators are not available in this editor.
Show Local Markers
   Shows action-local markers (which you can create in the Action Editor <dopesheet-marker-menu>).
   This can be useful to align strips to each other.

Use Timecode Ctrl-T
   Shows timing in seconds instead of frames.
Sync Visible Range
   Synchronizes the horizontal panning and scale of the editor
   with other time-based editors that also have this option enabled.
   That way, they always show the same section of time.

----------

Set Preview Range P
   Lets you drag a box to define a time range for previewing. As long as this range is active,
   playback will be limited to it, letting you repeatedly view a segment of the animation without
   having to manually rewind each time.

   You can change the start or end frame using the corresponding button in the
   Timeline editor's Playback <animation-editors-playback> popover.
   Alternatively, you can simply run *Set Preview Range* again.
Clear Preview Range Alt-P
   Clears the preview range.
Set Preview Range to Selected Ctrl-Alt-P
   Applies a preview range that encompasses the selected strips.

----------

Area
   Area controls. See the user interface
   documentation for more information.

Select Menu
-----------

All A
   Selects all strips.
None Alt-A
   Deselects all strips.
Invert Ctrl-I
   Inverts the current selection.

----------

Box Select <bpy.ops.*.select_box> B
   Lets you drag a box and selects the strips that are partially or completely inside it.
Box Select (Axis Range) Alt-B
   Lets you drag a box and selects the strips that overlap the corresponding time range,
   even if they're above or below the box.

----------

Before Current Frame [
   Selects all the strips that start before (or on) the current frame.
After Current Frame ]
   Selects all the strips that end after (or on) the current frame.

Marker Menu
-----------

Markers </animation/markers> are used to denote frames with key points or significant events
within an animation. Like with most animation editors, they're shown at the bottom.

For descriptions of the different marker tools, see Editing Markers <animation-markers-editing>.

Add Menu
--------

Action Shift-A
   Adds a strip referencing an action to the active track.
Transition Shift-T
   Adds a transition strip between the two selected action strips.
Sound Shift-K
   Adds a strip that controls when the /render/output/audio/speaker object plays its sound clip.

----------

.. _bpy.ops.nla.selected_objects_add:

Selected Objects
   Makes the selected objects appear in the NLA Editor without adding an action or track to them.

See /editors/nla/strips for details on the various strip types.

Track Menu
----------

Contains tools for working with NLA tracks.
See Editing Tracks </editors/nla/editing/track> for details.

Strip Menu
----------

Contains tools for working with NLA strips.
See Editing Strips </editors/nla/editing/strip> for details.

Filters
-------

Only Show Selected
   Only shows tracks belonging to objects that are selected.
Show Hidden
   Shows tracks from objects that are hidden.
Include Missing NLA
   Shows the Action Track even if there is no action in it.
Search
   Filters the track list by a search term.
Filtering Collection
   Select a collection to only show tracks from objects in that collection.
Filter by Type
   Filter tracks by target type.
Sort Data-Blocks
   Sorts data-blocks alphabetically to make them easier to find.

   If your playback speed suffers because of this
   (should only really be an issue when working with lots of objects),
   you can turn it off.

Snap
----

The toggle button enables/disables automatic strip snapping.
The dropdown button shows a popover with the following options:

Snap To
   Type of element to snap to.

   :Frame: Snap to full frames.
   :Second: Snap to seconds.
   :Nearest Marker: Snap to the nearest Marker </animation/markers>.

Absolute Time Snap
   When disabled, strips will move in increments of *Snap To*.
   For example, if you selected *Second* and have a strip that currently
   starts on 0:06+5, dragging it to the right will snap it to 0:07+5. Its time
   increases by a second, and its subsecond offset of 5 frames remains the same.

   When enabled, strips will snap to multiples of *Snap To*.
   Taking the above example, the strip would snap to 0:07+0,
   removing the subsecond offset.

Playback Controls
=================

The Playback Controls region contains controls and options
related to playback, keying, auto keyframing, and transport.

These settings allow you to:

- Control how animations are previewed and synchronized with audio.
- Insert and manage keyframes through keying sets and auto keying.
- Navigate the timeline using playback and transport controls.
- Adjust frame ranges and preview specific segments of the animation.

**Seealso:**

   For a detailed description of all properties and controls commonly found in the footer,
   see the Playback Controls <animation-editors-footer> documentation.
