---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSelfObject:

****************
Self Object Node
****************

The *Self Object* node outputs the object that contains the geometry nodes modifier
currently being executed. This can be used to retrieve the original transforms.

When evaluated in the Tool context <tool_context>, this node returns the Active object.

**Note:**

   The geometry cannot be retrieved from this object with the
   /modeling/geometry_nodes/input/scene/object_info, since its final geometry is still
   being evaluated.

Inputs
======

This node has no inputs.

Outputs
=======

Self Object
   The object currently being evaluated.
