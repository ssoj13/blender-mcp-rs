---
area: manual-editors
source: blender-manual
version: 5.2
---

****************
Contextual Views
****************

By default, the 3D Viewport displays the scene from a single viewpoint.
Using *Quad View* allows viewing the scene from multiple perspectives at the same time,
providing additional spatial context while modeling, sculpting, or editing.

.. _bpy.ops.screen.region_quadview:

Quad View
=========

Toggling *Quad View* splits the 3D Viewport into four regions:
three orthographic views (Top, Front, and Right by default)
and one perspective (User) view.

This layout makes it easier to understand the spatial relationships between objects
and to make precise adjustments along specific axes.

The size of each view can be adjusted by clicking and dragging from the center point
where the four regions meet.

**Note:**

   Quad View is different from splitting the area
   and aligning views manually. In Quad View, all four regions are part of the same
   3D Viewport and therefore share display settings such as shading mode, overlays,
   and visibility options.

Options
-------

Lock Rotation
   Prevents changes to the orientation and perspective of the orthographic side views.
   When enabled, navigation in one view will not rotate or switch the others.
   Note, enabling this option resets the three views to their default orientations (Top, Front, and Right).

.. _bpy.types.RegionView3D.show_sync_view:

Sync Zoom/Pan
   Synchronizes zooming and panning between the orthographic side views.
   This option requires *Lock Rotation* to be enabled.

.. _bpy.types.RegionView3D.use_box_clip:

Clip Contents
   Clips objects to the visible region defined by the orthographic views.
   Geometry outside the combined viewing volume is hidden, making it easier
   to inspect interior details or work on dense scenes.
