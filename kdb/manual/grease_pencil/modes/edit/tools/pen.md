---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

.. _tool-grease-pencil-edit-pen:

***
Pen
***

The Pen tool allows you to construct and edit Bézier curves.

Usage
=====

Extrude Point
   LMB click to add a new point connected to the currently selected point.
   The new point will have handle type <curve-bezier-handle-type> of *Vector*.
   Shift-LMB click will add point as type *Auto*.
   However, the handle type switches to *Align* when handles are moved (See *Move Point*).

Delete Point
   Ctrl-LMB click on an existing point to delete it.

Insert Point
   Ctrl-LMB click on a Curve Segment to insert a new control point between the two
   adjacent control points. Ctrl-LMB click and drag to control the handles of the inserted points.

Move Segment
   LMB drag on a segment in between two control points to adjust the handles, changing the shape of the
   curve without affecting the location of any control points.

Select Point
   LMB click to select a single point or handle at a time.

Move Point
   LMB drag to move existing points or handles. With an endpoint of a stroke selected,
   click and drag on empty space to *Extrude Point* and move the handle at the same time.

Close Stroke
   Make the stroke Cyclic by clicking the endpoints.

Cycle Handle Type
   Double LMB click on the control point to cycle through all handle types.

Hotkeys
=======

Free-Align Toggle
   Hold LeftCtrl while dragging a handle to switch between `Free` and `Align` handle types.
   Can be used to create sharp corners along the curve.

Move Entire Point
   Hold LeftAlt while dragging a handle to move the entire point.

Snap Handle Angle
   Hold LeftShift while dragging a handle to limit the movement of the handle to multiples 45 degrees,
   so the handle can only be vertical, horizontal or diagonal.

Tool Settings
=============

Size
   Control the diameter of newly added strokes.
Stroke Placement
   The Stroke Placement </grease_pencil/modes/draw/stroke_placement> of newly added points.
Drawing Plane
   The Drawing Planes </grease_pencil/modes/draw/drawing_planes> of newly added points.
