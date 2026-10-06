---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

.. _bpy.types.GreasePencilMirrorModifier:

***************
Mirror Modifier
***************

The *Mirror* modifier mirrors the strokes along its local X, Y and/or Z axes, across the Object Origin.
It can also use another object as the mirror center, then use that object's local axes instead of its own.

**Seealso:**

   This documentation refers to the Mirror Modifier specific to the Grease Pencil object.
   For uses with other object types refer to the general /modeling/modifiers/generate/mirror.

Options
=======

Axis
   The X, Y, Z axis along which to mirror, i.e. the axis perpendicular to the mirror plane of symmetry.

   To understand how the axis applies to the mirror direction, if you were to mirror on the X axis,
   the positive X values of the original stroke would become the negative X values on the mirrored side.

   You can select more than one of these axes. And will then get more mirrored copies.
   With one axis you get a single mirror, with two axes four mirrors, and with all three axes eight mirrors.

Object
   A ui-data-id to select an object (usually an empty),
   which position and rotation will be used to define mirror planes
   (instead of using the ones from the modified object).

Influence
---------

See grease-pencil-modifier-influence-filters.
