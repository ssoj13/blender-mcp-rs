---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeVolumeAbsorption:

*****************
Volume Absorption
*****************

The *Volume Absorption* node allows light to be absorbed as it passes through the volume.
Typical usage for this node would be water and colored glass.

Inputs
======

Color
   Color of the volume.
Density
   The density of the absorption effect.

Properties
==========

This node has no properties.

Outputs
=======

Volume
   The Volume Shader output must be plugged into the *Volume Input*
   of the Material
   or World </render/shader_nodes/output/world> Output node.

Examples
========
