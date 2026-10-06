---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.FunctionNodeAxesToRotation:
.. --- copy below this line ---

*********************
Axes to Rotation Node
*********************

Creates a rotation based on two axis directions.

**Tip:**

   In many cases, these directions are a normal and tangent on a mesh or curve.

Inputs
======

Primary Axis
   The desired direction of the primary axis.

Secondary Axis
   The desired direction of the secondary axis. Ideally, this is orthogonal to the primary direction.

Properties
==========

Primary Axis
   The axis (X, Y or Z) that should be aligned exactly to the primary direction.

Secondary Axis
   The axis that should be aligned as closely as possible to the secondary direction.

Outputs
=======

Rotation
    The rotation that results in the given axes being aligned to the given directions.
