---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodePointInfo:

**********
Point Info
**********

Cycles Only

The *Point Info* node can be used in the material node tree for point cloud objects
and gives access to the data of individual points.
It can be useful to give some variation to a single material assigned a point cloud object.

Inputs
======

This node has no inputs.

Properties
==========

This node has no properties.

Outputs
=======

Location
   Location of the particle.
Radius
   Size of the particle.
Random
   A random per-point value in the range from 0 to 1.
   It can for example be used in combination with a color ramp, to randomize the point color.
