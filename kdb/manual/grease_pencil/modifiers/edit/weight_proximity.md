---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

.. _bpy.types.GreasePencilWeightProximityModifier:

********************************
Vertex Weight Proximity Modifier
********************************

This modifier sets the weights of the given vertex group,
based on the distance between the object (or its vertices),
and another target object (or its geometry).

**Warning:**

   This modifier does implicit clamping of weight values in the standard (0.0 to 1.0) range.
   All values below 0.0 will be set to 0.0, and all values above 1.0 will be set to 1.0.

**Seealso:**

   This documentation refers to the Vertex Weight Proximity Modifier specific to the Grease Pencil object.
   For uses with other object types refer to the general /modeling/modifiers/modify/weight_proximity.

Options
=======

Vertex Group
   The vertex group to affect.

   :bl-icon:`arrow_leftright` Invert
      Inverts the influence of the selected vertex group. The setting reverses the weight values of the group.

Target Object
   The object from which to compute distances.

Lowest
   Distance mapping to 0.0 weight.
Highest
   Distance mapping to 1.0 weight.

Minimum
   Minimum value for vertex weight.

Multiply Weights
   Multiply the calculated weights with the existing values in the vertex group.

Influence
---------

See grease-pencil-modifier-influence-filters.
