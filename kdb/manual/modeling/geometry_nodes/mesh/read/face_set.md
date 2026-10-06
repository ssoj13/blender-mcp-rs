---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeToolFaceSet:

*************
Face Set Node
*************

The *Face Set Node* outputs which face set <face_sets> a face is in,
and whether or not face sets exist in the mesh at all.

The corresponding data flow node is the /modeling/geometry_nodes/mesh/write/set_face_set.

**Note:**

   This node can only be used in the Tool context <tool_context>.

Inputs
======

This node has no inputs.

Output
======

Face Set
   Integer indicating which face set a face is in, or 0 when the mesh does not have face sets.
   When evaluated in the edge or point domain, outputs an interpolated value based on the connected faces.

Exists
   Boolean value that indicates whether the element's mesh has face sets.
