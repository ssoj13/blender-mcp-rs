---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputInstanceRotation:

**********************
Instance Rotation Node
**********************

The *Instance Rotation* outputs the XYZ Euler rotation of each top-level instance in the
local space of the modifier object.

The /modeling/geometry_nodes/instances page contains more information about geometry instances.

**Note:**

   Though rotations are often displayed in units of degrees in the spreadsheet or node editor,
   they are stored internally in radians, so this node outputs radians.

Inputs
======

This node has no inputs.

Outputs
=======

Rotation
   Vector that indicates the rotation of each top-level instance in radians.
