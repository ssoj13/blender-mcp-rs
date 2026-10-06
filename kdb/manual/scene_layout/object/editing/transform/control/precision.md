---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

*********
Precision
*********

Holding Ctrl during a transform operation (such as move, rotate or scale)
will toggle Transform Snapping </editors/3dview/controls/snapping>.
When using Increment Snap <bpy.types.ToolSettings.snap_elements_base>
this allows the transformation to be performed in discrete amounts.

Holding Shift during a transform operation will transform
the object at 1/10th the speed, allowing much finer control.

The magnitude of the transformation can be viewed in the 3D Viewport header.
Releasing Ctrl or Shift during the transformation will cause
the movement to revert back to its normal mode of operation.

**Note:**

   The snapping behaviors described on this page **only** apply
   when Increment Snap <bpy.types.ToolSettings.snap_elements_base> is selected.

**Tip:**

   It is possible to enable both snapping and precision mode,
   simply hold Ctrl and Shift. This has the following effects:

   Move
      Changes in 0.1 unit increments, regardless of zoom level.
   Rotation
      Changes in 1 unit increments.
   Scale
      Changes in 0.01 unit increments.

Usage
=====

With Hotkeys
------------

Press G, R or S and then hold either Ctrl,
Shift or Shift-Ctrl.

With the Transform Gizmo
------------------------

Select the gizmo handle then while moving the mouse hold Ctrl, Shift or Shift-Ctrl
to activate precision control or snapping.

**Seealso:**

   Read more about the Transform Gizmo </editors/3dview/display/gizmo>.

**Tip:**

   All of the precision controls detailed on the page can be combined with
   the Axis Locking </scene_layout/object/editing/transform/control/axis_locking> controls and
   used with the different Pivot Points </editors/3dview/controls/pivot_point/index>.

Snapping
========

Move
----

Snapping while moving objects changes the object location in 1 unit increments.
While in an aligned view </editors/3dview/navigate/align>,
The increment amount is changed based on the zoom level <bpy.ops.view3d.zoom>.
For example, at a base zoom level objects are moved in increments of 1 unit (i.e. between the two light gray lines).
Zooming in enough to see the next set of gray lines will snap in increments of 1/10 of a unit.
Zooming in further until will snap in increments of 1/100 of a unit and so on until the zoom limit is reached.
Zooming out will have the opposite effect and
cause movement to happen by increments of 10, 100 units, etc.

Rotation
--------

Holding Ctrl will cause rotations of 5 degrees.

Scale
-----

Holding Ctrl will cause size changes in increments of 0.1 units.

**Note:**

   Note that when you are Snapping To
   something other than *Increment*,
   holding Ctrl will cause the selection to snap to that nearest element.

   Read more about snapping </editors/3dview/controls/snapping>.

Precision
=========

Holding Shift during transformations allows for very fine control that does not
rely on fixed increments. Rather, large movements of the mouse across
the screen only result in small transformations of the selection.

In rotation mode the selected element will be rotate in 0.01 degree increments.
