---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.FunctionNodeRotateRotation:
.. --- copy below this line ---

********************
Rotate Rotation Node
********************

The *Rotate Rotation* node applies an additional rotation to a given one.

To rotate an Euler Rotation, first use the
/modeling/geometry_nodes/utilities/rotation/euler_to_rotation.

Inputs
======

Rotation
   The starting rotation.

Rotate By
   The additional rotation.

Properties
==========

Space
   :Global: Rotate in Global Space.
   :Local: Rotate in Local Space.

Outputs
=======

Rotation
   The resulting rotation.
