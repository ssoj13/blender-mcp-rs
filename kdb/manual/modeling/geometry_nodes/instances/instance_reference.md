---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputInstanceReference:

***********************
Instance Reference Node
***********************

The *Instance Reference* node outputs the `reference_index` attribute as an integer field.

Each unique instanced geometry source is assigned a reference index.
Instances that originate from the same geometry or object share the same value,
allowing them to be grouped, compared, or processed together.

This is useful for identifying instances with shared geometry sets,
creating variation per referenced asset, or selecting instances based on
their original source geometry.

Inputs
======

This node has no inputs.

Outputs
=======

Reference Index
   Integer field representing the reference index of each instance source.

   Instances sharing the same referenced geometry output the same value.
