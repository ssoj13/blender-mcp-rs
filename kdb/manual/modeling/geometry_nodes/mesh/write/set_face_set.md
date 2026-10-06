---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeToolSetFaceSet:

*****************
Set Face Set Node
*****************

The *Set Face Set* node controls which face set <face_sets> that faces are in.

The input node for this data is the /modeling/geometry_nodes/mesh/read/face_set.

**Note:**

   This node can only be used in the Tool context <tool_context>.

Inputs
======

Mesh
   Standard geometry input.

Selection
   Boolean field that controls which faces will have the Face Set value applied.

Face Set
   Integer field for specifying which face set each selected face should be moved to.
   Ignored for faces where the value of Selection is false.

Outputs
=======

Mesh
   Standard geometry output.
