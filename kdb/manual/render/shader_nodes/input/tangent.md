---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeTangent:

************
Tangent Node
************

The *Tangent* node generates a tangent direction for the Anisotropic BSDF.

Inputs
======

This node has no inputs.

Properties
==========

Direction Type
   The tangent direction can be derived from a cylindrical projection around the X,
   Y, or Z axis (radial), or from a manually created UV Map for full control.

Outputs
=======

Tangent
   The tangent direction vector.
