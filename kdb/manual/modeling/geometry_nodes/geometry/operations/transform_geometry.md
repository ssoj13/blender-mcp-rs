---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeTransform:

***********************
Transform Geometry Node
***********************

The *Transform Geometry Node* allows you to move, rotate or scale the geometry.
The transformation is applied to the entire geometry, and not per element.
The /modeling/geometry_nodes/geometry/write/set_position is used for moving
individual points of a geometry. For transforming instances individually, the instance
translate </modeling/geometry_nodes/instances/translate_instances>,
rotate </modeling/geometry_nodes/instances/rotate_instances>, or
scale </modeling/geometry_nodes/instances/scale_instances> nodes can be used.

Inputs
======

Geometry
   Standard geometry input.

Mode
   How the transformation is specified.

   :Components: Provide separate inputs for location, rotation and scale.
   :Matrix: Use a transformation matrix.

Translation
   Translation of the entire geometry in the local space of the modified object.

Rotation
   Euler rotation in the local space of the modified object.

Scale
   Scale for the geometry in the local space of the modified object.

Transform
   A Transformation Matrix, available when using *Matrix* mode.

Output
======

Geometry
   Standard geometry output.
