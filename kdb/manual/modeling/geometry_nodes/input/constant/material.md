---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputMaterial:

*************
Material Node
*************

The *Material* input node outputs a single material. It can be connected to other material sockets
to make using the same material name in multiple places more convenient.

**Tip:**

   The Material node can also be added by dragging and dropping a material data-block into the node editor.
   This will add the node and select the dropped material in the ui-data-block.

Inputs
======

This node has no inputs.

Properties
==========

- Material

Output
======

Material
   A reference to the selected material.
