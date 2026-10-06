---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.LineStyle*Modifier_CreaseAngle:
.. Editor's Note: This page gets copied into:

.. --- copy below this line ---

************
Crease Angle
************

A modifier based on the Crease Angle (angle between two adjacent faces).
If a stroke segment does not lie on a crease (i.e. the edge does not have the *Crease Angle nature*),
its properties are not touched by the modifier.

Angle Min, Max
   The range of input values to the mapping.
   Out-of-range crease angle values will be clamped by
   the Min and Max angles and their corresponding property values.
