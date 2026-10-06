---
area: manual-physics
source: blender-manual
version: 5.2
---

****************
Hinge Constraint
****************

The hinge permits one degree of freedom between two objects. Translation is completely constrained.
Rotation is permitted about the Z axis of the object hosting the Physics constraint
(usually an Empty, distinct from the two objects that are being linked).
Adjusting the position and rotation of the object hosting the constraint allows you to
control the anchor and axis of the hinge.

The Hinge is the only single-axis rotational constraint that uses the Z axis instead of the X axis.
If something is wrong with your hinge, check your other constraints to see if this might be the problem.

Options
=======

Limits
   Z Angle
      Enables/disables limit rotation around Z axis.

      Lower
         Lower limit of Z axis rotation.
      Upper
         Upper limit of Z axis rotation.
