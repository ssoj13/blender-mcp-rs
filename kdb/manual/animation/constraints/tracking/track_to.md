---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.TrackToConstraint:

*******************
Track To Constraint
*******************

The *Track To* constraint makes an object or bone point towards a certain target,
then roll to make a certain local axis point upwards.
It's typically called "Look At" or "Aim" in other 3D software.

If the constraint owner is almost directly above the target, the roll around
the tracking axis will become unstable. For this scenario, it's recommended to
use the /animation/constraints/tracking/damped_track instead.

Options
=======

Target <rigging-constraints-interface-common-target>
   The object or bone to point towards.

Track Axis
   The local axis of the owner that should point at the target.
   For bones, this should typically be Y.

   A negative axis will make the owner point away from the target instead.

Up
   The local axis of the owner that should point upwards in the world.

**Important:**

   The *Track Axis* and the *Up* axis must be different. If they are the same,
   the constraint will stop working and its icon will turn red.

Target Z
   Align the owner's *Up* axis as closely as possible to the target's Z axis
   instead of the global one.

Target/Owner <rigging-constraints-interface-common-space>
   The spaces for evaluating the target and owner transforms.

bpy.types.constraint.influence
   How strongly the constraint affects the owner.

Example
=======
