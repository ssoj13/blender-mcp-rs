---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

.. _tool-grease-pencil-draw-interpolate:

***********
Interpolate
***********

The *Interpolate* tool creates an in-between keyframe
by interpolating strokes between the previous and next Grease Pencil keyframes.
When used on a frame between two keyframes, clicking and dragging will add a new *breakdown* keyframe,
interpolating the stroke's shape based on the drag distance.

This allows artists to manually control how strokes evolve between poses or drawings,
providing fine control over the interpolation result.

**Note:**

   When interpolating between curves of different types, a priority system determines which curve type is used.
   The priority from highest to lowest is: *NURBS* --> *Bézier* --> *Catmull-Rom* --> *Polyline*.

Usage
=====

#. Place the timeline playhead between two existing Grease Pencil keyframes.
#. In the 3D Viewport, click and drag left to right to set the desired interpolation factor.
#. Release the mouse button to confirm and create a new *breakdown keyframe*.

The resulting keyframe contains strokes that are interpolated between the neighboring keyframes
according to the chosen factor.

Tool Settings
=============

Layer
   Restrict interpolation to either the *Active Layer* or *All Layers*.

Only Selected (Edit Mode)
   When enabled, only selected strokes will be interpolated.

Exclude Breakdowns
   Exclude existing Breakdown keyframes <keyframe-type> from being used as interpolation extremes.

Flip Mode
   Reverses the interpolation direction, swapping the start and end strokes.
   *Automatic* mode attempts to determine the correct direction for each stroke automatically.

Smooth
   The amount of smoothing applied to interpolated strokes to reduce jitter or visual noise.

Iterations
   The number of times smoothing is applied to newly created strokes.
   Higher values result in smoother interpolation but may reduce shape fidelity.
