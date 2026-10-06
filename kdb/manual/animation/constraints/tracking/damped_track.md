---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.DampedTrackConstraint:

***********************
Damped Track Constraint
***********************

The *Damped Track* constraint makes an object or bone point towards a certain target.
It's typically called "Look At" or "Aim" in other 3D software.

The word "damped" means that the constraint uses a pure swing rotation
to minimize rolling around the tracking axis. This is in contrast to the
/animation/constraints/tracking/track_to which applies an "Up" axis
in addition to the tracking.

Options
=======

Target <rigging-constraints-interface-common-target>
   The object or bone to point towards.

Track Axis
   The local axis of the owner that should point at the target.
   For bones, this should typically be Y.

   A negative axis will make the owner point away from the target instead.

bpy.types.constraint.influence
   How strongly the constraint affects the owner.

Example
=======
