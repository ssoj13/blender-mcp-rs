---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeCurveHandleTypeSelection:

**************************
Handle Type Selection Node
**************************

Creates a selection based on the handle types of the control points.

The handle type of each control point can be changed with the
/modeling/geometry_nodes/curve/write/set_handle_type.

Inputs
======

This node has no inputs.

Properties
==========

Mode
   Whether to consider left and/or right handles.
   When both are selected, the output value is true
   if either of the handles are of the chosen type.

   :Left:
      Consider the left handle.
   :Right:
      Consider the right handle.

Handle Type
   Handle type for which the selection will be true.
   See the Bézier curves <curve-bezier-handle-type> page for more details
   on the different handle types.

Outputs
=======

Selection
   Boolean field set to true wherever the handle type matches.
