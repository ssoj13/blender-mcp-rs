---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeToolActiveElement:

*******************
Active Element Node
*******************

The *Active Element* node outputs the index of the Active point, edge, face, or layer.

**Note:**

   This node can only be used in the Tool context <tool_context>.

Inputs
======

This node has no inputs.

Properties
==========

Domain
   Which domain <attribute-domains> to return the index of.

Outputs
=======

Index
   Index of the active element in the specified domain.

Exists
   True if an active element exists in the mesh, false otherwise
