---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.LockedTrackConstraint:

***********************
Locked Track Constraint
***********************

The *Locked Track* constraint makes an object or bone point towards a certain target.
It's typically called "Look At" or "Aim" in other 3D software.

The word "locked" means that the object or bone can only rotate around one axis.
The orientation of this axis always stays the same, and the other two axes always
stay in their original plane.

One example use case is distant tree billboards, which should turn to face the camera
while also staying upright. Another example is a compass on a table with a nearby magnet:
the needle spins horizontally to point at the magnet, but it can never point up or down.

**Tip:**

   This constraint can be used in combination with other *Track* constraints. For example,
   first add a Damped Track </animation/constraints/tracking/damped_track> constraint
   to orient the X/Y plane based on one target, then add a *Locked Track* constraint to rotate
   within that plane towards another target.

Options
=======

Target <rigging-constraints-interface-common-target>
   The object or bone to point towards.

Track Axis
   The local axis of the owner that should point at the target.
   For bones, this should typically be Y.

   A negative axis will make the owner point away from the target instead.

Locked Axis
   The local axis of the owner that should keep its orientation.
   In other words, the only axis which the owner can rotate around.

**Important:**

   The *Track Axis* and the *Locked Axis* must be different. If they are the same,
   the constraint will stop working and its icon will turn red.

bpy.types.constraint.influence
   How strongly the constraint affects the owner.

Example
=======
