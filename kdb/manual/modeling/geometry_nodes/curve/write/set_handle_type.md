---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeCurveSetHandles:

********************
Set Handle Type Node
********************

Sets the handle type for the points on the Bézier curve that are in the selection.

A selection for a certain handle type can be retrieved with
the /modeling/geometry_nodes/curve/read/handle_type_selection.

Inputs
======

Curve
   Bézier curve.

Selection
   The points whose handle types will be changed.

Properties
==========

Mode
   Whether to update left or right or both handles.
   When both are selected, both the left and the right handles will be updated.

   :Left:
      Update the left handles.
   :Right:
      Update the right handles.

Handle Type
   The handle type to switch to. See the Bézier curves
   page for more details on the different handle types.

Outputs
=======

Curve
   Standard curve output.
