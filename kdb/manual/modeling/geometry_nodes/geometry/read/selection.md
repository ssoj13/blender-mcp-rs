---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeToolSelection:

**************
Selection Node
**************

The *Selection* node outputs true for geometry that is selected </interface/selecting>, and false elsewhere.

The corresponding data flow node is the /modeling/geometry_nodes/geometry/write/set_selection.

**Note:**

   This node can only be used in the Tool context <tool_context>.

Inputs
======

This node has no inputs.

Outputs
=======

Selection
   Boolean field set to true for geometry that is selected in edit mode.
