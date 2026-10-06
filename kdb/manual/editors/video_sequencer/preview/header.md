---
area: manual-editors
source: blender-manual
version: 5.2
---

******
Header
******

.. _bpy.types.SpaceSequenceEditor.show:

View Menu
=========

Toolbar T
   Show or hide the Toolbar <ui-region-toolbar>.
Sidebar N
   Show or hide the Sidebar <ui-region-sidebar>.
Tool Settings
   Show or hide the settings for the currently selected tool.
Footer
   Show or hide the Sidebar <ui-region-footer>.

----------

.. _bpy.types.SpaceSequenceEditor.show_transform_preview:

Preview During Transform
   Show a preview of the start or end frame of a strip while transforming its respective handle.

----------

Refresh All
   Reloads external files and refreshes the current frame preview.
   This is useful when you modified an external file or made a change in a scene that Blender
   didn't detect.

----------

Frame Selected
   Pan and zoom the view to focus on the selected image.
Fit Preview in Window Home
   Pan and zoom the view so that the entire video is visible.
   This enables *Zoom to Fit*.

Zoom
   Menu with convenient zoom levels and operations.
   The zoom levels are calculated based on the images resolution compared to the screen resolution.

   - 12.5% (1:8) Numpad8 zoom out to a factor of 12.5%.
   - 25% (1:4) Numpad4 zoom out to a factor of 25%.
   - 50% (1:2) Numpad2 zoom out to a factor of 50%.
   - 100% (1:1) Numpad1 resets the zoom to 100%.
   - 200% (2:1) Ctrl-Numpad2 zoom in to a factor of 200%.
   - 400% (4:1) Ctrl-Numpad4 zoom in to a factor of 400%.
   - 800% (8:1) Ctrl-Numpad8 zoom in to a factor of 800%.

   Zoom In/Out Wheel
      Zooms the view in or out.
   Zoom to Fit Shift-Home
      Like *Frame All*, but uses as much space in the editor as possible.
   Zoom Region Shift-B
      Zoom in the view to the nearest item contained in the border.

.. _bpy.types.SpaceSequenceEditor.use_zoom_to_fit:

Auto Zoom
   As long as this option is enabled, the preview will automatically zoom to keep the
   video size synchronized with the editor size.

----------

Proxy
   See /editors/video_sequencer/sequencer/sidebar/proxy.

----------

Use Timecode Ctrl-T
   Shows seconds instead of frames on the time axis.

----------

Sequence Render Image
   Show the current frame preview as a Render Result where you can save it as an image file.
Sequence Render Animation
   Save previews of the frames in the scene range (or the preview range, if active) to a video file
   or a series of image files. See the /render/output/properties/output panel for details.

**Note:**
   *Sequence Render Image* and *Sequence Render Animation* don't render the final video by default --
   specifically, they don't render Scene Strips, instead using the preview's
   shading mode </editors/3dview/display/shading> (which is initially Solid).

   To output a video where the Scene Strips are rendered, use the *Render* menu in the top-bar,
   or change Sidebar --> View --> Scene Strip Display --> Shading to *Rendered*.

----------

.. _bpy.ops.sequencer.export_subtitles:

Export Subtitles
   Exports Text strips </video_editing/edit/montage/strips/text>,
   which can act as subtitles, to a `SubRip <https://en.wikipedia.org/wiki/SubRip>`__ file (`.srt`).
   The exported file contains all Text strips in the video sequence.

----------

Toggle Sequencer/Preview Ctrl-Tab
   Switch the editor mode between *Sequencer* and *Preview*.

----------

Area
   Area controls. See the user interface
   documentation for more information.

Select Menu
===========

See /video_editing/edit/montage/selecting.

Strip Menu
==========

See /video_editing/edit/montage/editing for more information.

Mirror
   Interactive Mirror Ctrl-M
      Starts mirror operation based on mouse cursor position
   X Global
      Mirrors the image in global X coordinates
   Y Global
      Mirrors the image in global Y coordinates
   X Local
      Mirrors the image in local X coordinates
   Y Local
      Mirrors the image in local Y coordinates

.. _bpy.ops.sequencer.preview_duplicate_move:

Duplicate Shift-D
   The *Duplicate* operator creates a copy of the selected strip(s)
   and places them in the nearest available channel above the original.

   The duplicated content remains selected, allowing immediate repositioning.

**Seealso:**

   bpy.ops.sequencer.duplicate_move

Copy Ctrl-C
   Adds selected strips to copy-paste buffer.

Paste Ctrl-V
   Paste strips from copy-paste buffer.

Animation
   Insert Keyframe I
      See Insert Keyframe <bpy.ops.anim.keyframe_insert>.

   Insert Keyframe with Keying Set K
      See Insert Keyframe with Keying Set <bpy.ops.anim.keyframe_insert_menu>.

   Change Keying Set Shift-K
      See Set Active Keying Set <bpy.ops.anim.keying_set_active_set>.

   Delete Keyframes Alt-I
      See Delete Keyframes <bpy.ops.anim.keyframe_delete>.

   Clear Keyframes Shift-Alt-I
      See Clear Keyframes <bpy.ops.anim.keyframe_clear>.

**Seealso:**
   Editing Keyframes

Show/Hide
   Show Hidden Strips Alt-H
      Reveals all hidden/muted strips.
   Hide Selected H
      Mutes the selected strips.
   Hide Unselected Shift-H
      Mutes all strips except for the currently selected strips.

Delete
   Delete selected strips

Image Menu
==========

Transform
   Move Origin Ctrl-Period
      Moves the origin of the image strip without changing the strip's content position.

      This is useful for adjusting the reference point for transformations like rotation, scaling, or further
      positioning. For example, shifting the origin to a corner of the image allows rotations to pivot around that
      corner, rather than the default center.

Clear
   Resets the position, rotation, or scale of the selected images.
Apply
   Scale to Fit
      Resizes the selected images so that they're as large as possible while still
      fitting completely inside the video. They don't get cropped, and their aspect ratio
      stays the same.

   Scale to Fill
      Resizes the selected images to that they fill the entire video space.
      They may get cropped, but their aspect ratio stays the same.

   Stretch to Fill
      Resizes the selected images to match the video dimensions.
      They don't get cropped, but their aspect ratio may change.

Pivot Point
===========

See /editors/video_sequencer/preview/controls/pivot_point.

Display Mode
============

See /editors/video_sequencer/preview/display/display_mode.

Display Channels
================

Color & Alpha
   Display the preview image with transparency over a checkerboard pattern.
Color
   Ignore the transparency of the preview image (fully transparent areas will be black).

Gizmos
======

See /editors/video_sequencer/preview/display/gizmos.

Overlays
========

See /editors/video_sequencer/preview/display/overlays.
