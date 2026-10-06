---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSwitch:
.. --- copy below this line ---

***********
Switch Node
***********

The *Switch* node outputs one of two inputs depending on a condition.

When the inputs are regular values, only the input that is passed through
the node is evaluated. This means the unused input is not computed.

**Note:**

   For Geometry Nodes:

   When the inputs are /modeling/geometry_nodes/fields, both inputs may be evaluated
   before the switch is applied. The switch then selects which result is used
   for each element. Because of this, the node should not be relied on to avoid
   evaluating expensive field inputs.

**Seealso:**

   The menu_switch and index_switch
   can be used to switch between an arbitrary amount of inputs.

Inputs
======

Switch
   Determines which of the two inputs below will be passed through.

False
   Is passed through when the switch is set to false.

True
   Is passed through when the switch is set to true.

Properties
==========

Type
   Determines the type of the data that is handled by the node.

Outputs
=======

Output
   One of the two inputs without any modifications.
