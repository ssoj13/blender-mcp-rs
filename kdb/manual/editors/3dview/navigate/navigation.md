---
area: manual-editors
source: blender-manual
version: 5.2
---

**********
Navigation
**********

.. _bpy.ops.view3d.view_orbit:

Orbit
=====

Rotate the view around the point of interest by clicking and dragging
MMB on the viewport's area.

RMB cancels the orbit operation.

The Alt key has several effects on orbiting:

- Clicking a point with Alt-MMB will make it the point of interest:
  it becomes the central point which the view orbits around.
- Holding Alt and then dragging with MMB in a certain direction
  will align </editors/3dview/navigate/viewpoint> the view to an axis
  and make it orthographic.
- Dragging with MMB and then holding Alt
  will perform an orbit while also snapping to the world axes,
  as well as the diagonals between them.

To change the viewing angle in discrete steps, use Numpad8 and Numpad2
to go up and down, or Numpad4 and Numpad6 for left and right.
You can also press Numpad9 to switch to the opposite side of the view
(rotates the camera 180° around the Z axis).

**Seealso:**

   - Orbit Style Preference
   - Auto-Perspective Preference

.. _bpy.ops.view3d.view_roll:

Roll
====

Rotate the viewport camera around its viewing direction in 15° discrete steps by default.
See the rotation angle <bpy.types.PreferencesView.rotation_angle> preference to configure.

To reset the roll, you can first align the view to the global X axis
using Numpad3, then orbit to get back to the regular perspective view.

RMB cancels the roll operation.

.. _bpy.ops.view3d.view_pan:

Pan
===

Pans the 3D Viewport by moving the view left, right, up, or down without rotating it.
This is useful for repositioning your view without changing orientation.

- Shift-MMB -- Pan the view freely by dragging the mouse.
- Shift-WheelUp / Shift-WheelDown -- Pan the view vertically.
- Shift-WheelLeft / Shift-WheelRight -- Pan the view horizontally.
- Ctrl-Numpad8 / Ctrl-Numpad2 -- Pan the view up/down in fixed steps.
- Ctrl-Numpad4 / Ctrl-Numpad6 -- Pan the view left/right in fixed steps.

.. _bpy.ops.view3d.zoom:

Zoom In/Out
===========

Moves the view closer to, or further away from, the point of interest.
You can zoom in and out by rolling the Wheel or dragging with Ctrl-MMB.
To zoom with discrete steps, use the hotkeys NumpadPlus and NumpadMinus.

**Hint:**

   If you get lost in 3D space (which is not uncommon),
   bpy.ops.view3d.view_all and bpy.ops.view3d.view_selected
   can be used to show the contents of your scene.

RMB cancels the zoom operation.

.. _bpy.ops.view3d.zoom_border:

Zoom Region
===========

The *Zoom Region* tool allows you to specify a rectangular region
by dragging with LMB. The view will then zoom in on this region.

You can also drag with MMB to zoom out instead.

.. _bpy.ops.view3d.dolly:

Dolly View
==========

In most cases it's sufficient to zoom the view to get a closer look at something.
However, zooming only gets you up to the point of interest and no further.
If you hit this point where zooming no longer works, you can instead Dolly
by holding Shift-Ctrl and dragging up or down with MMB.
This will move the point of interest (and the view along with it).

RMB cancels the dolly operation.

**Hint:**

   Dolly changes orthographic views to a perspective projection.

   This is done because dolly doesn't work well with an orthographic projection because *moving* forwards/backwards
   with an orthographic projection doesn't have the effect of zooming.

.. NOTE(@campbellbarton): "Frame All" & "Frame Selected" could be documented elsewhere,
   however there doesn't seem to be an ideal location as only items in the View sub-menus have their own files.
   Loosely speaking these are navigation - so keep here.

.. _bpy.ops.view3d.view_all:

Frame All
=========

Changes the view so that you can see all objects.

.. _bpy.ops.view3d.view_selected:

Frame Selected
==============

Changes the view so that you can see the selected object(s).

Frame Last Stroke
=================

Centers the view on the region of the last brush stroke.
