---
area: manual-compositing
source: blender-manual
version: 5.2
---

*******
Sidebar
*******

View
====

.. _bpy.types.SpaceNodeEditor.show_backdrop:

Backdrop
--------

The backdrop displays the output of a *Viewer* node behind the node editor in the Compositor.
For example, pressing Shift-Ctrl-LMB on an Image node displays a preview of the image,
while pressing it on a Mix node shows the result of the mixing operation.

The backdrop can be toggled by enabling the checkbox in the *Backdrop* panel header
or by clicking the *Backdrop* button in the Compositor header.

The backdrop is useful for visually evaluating node results while editing,
allowing direct comparison between the node graph and its output.

.. _bpy.types.SpaceNodeEditor.backdrop_channels:

Channels
   Specifies which color channels of the backdrop image are displayed.

.. _bpy.types.SpaceNodeEditor.backdrop_zoom:

Zoom Alt-V, V
   Adjusts the display size of the backdrop image.

.. _bpy.types.SpaceNodeEditor.backdrop_offset:

Offset X, Y
   Offsets the backdrop image in screen space.

.. _bpy.ops.node.backimage_move:

Move Alt-MMB
   Interactively moves the backdrop image.

.. _bpy.ops.node.backimage_fit:

Fit
   Scales the backdrop image to fit the Compositor view.

Options
=======

Performance
-----------

This panel helps you tweak the performance of the Compositor.

.. Currently removed from UI see https://projects.blender.org/blender/blender/commit/a079875ffe
.. .. _bpy.types.CompositorNodeTree.use_viewer_border:

.. Viewer Region
..    This allows to set an area of interest for the backdrop.
..    Press Ctrl-B and select a rectangular area in the preview
..    which will become the next preview in the backdrop.
..    Ctrl-Alt-B discards the region back to a full preview.
..    This is only a preview option, final compositing during a render ignores this region.
