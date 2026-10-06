---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSetInstanceTransform:

***************************
Set Instance Transform Node
***************************

The *Set Instance Transform* node Transforms <Transform> geometry instances
using a Transformation Matrix.

The /modeling/geometry_nodes/instances page contains more information about geometry instances.

Inputs
======

Instances
   Standard geometry input.

Selection
   Boolean field used to determine if an instance will be rotated.

Transform
   The transformation matrix to translate, rotate, and scale individual instances.

Outputs
=======

Instances
   Standard geometry output.
