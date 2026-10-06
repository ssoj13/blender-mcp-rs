---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.FunctionNodeRotateEuler:

*****************
Rotate Euler Node
*****************

The *Rotate Euler* node rotates an Euler rotation.

**Important:**

   This node is deprecated, use the /modeling/geometry_nodes/utilities/rotation/rotate_rotation instead.

Inputs
======

Rotation
   The Euler rotation to rotate.

Rotate By
   Specifies how much an Euler rotation is rotated. This input is only available
   when the rotation type is set to *Euler*.

Axis
   The axis to rotate around. This input is only available when the rotation type is set to *Axis Angle*.

Angle
   The angle to rotate by around the specified axis. This input is only available
   when the rotation type is set to *Axis Angle*.

Properties
==========

Rotation Type
   :Axis Angle: Use separate axis and angle inputs to control the rotation.
   :Euler: Use an Euler input to control the rotation.

Space
   :Object: Rotate an Euler rotation in the evaluated object's space.
   :Local: Rotate an Euler rotation in local space.

Outputs
=======

Rotation
   The rotated Euler rotation.
