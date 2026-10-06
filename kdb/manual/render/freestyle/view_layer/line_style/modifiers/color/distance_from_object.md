---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.LineStyle*Modifier_DistanceFromObject:
.. Editor's Note: This page gets copied into:

.. --- copy below this line ---

********************
Distance from Object
********************

The *Distance from Object* modifier alters the base property with a new one
from a given range using the distance to the active *camera* or to a given *object* as the parameter.

Target
   The object to measure distance from.

Range Min, Max
   The limits of the mapping from "distance to camera" to "property in mapping".
   If the current point of the stroke is at *Range Min* or less from the active camera or the object,
   it will take the start value. And conversely,
   if it is at *Max* or more from the camera/object, it will take the end value.
   These values are in the current scene's units, not in pixels!

Fill Range by Selection
   Set the min/max range values from the distances between the current selected mesh vertices and
   the camera or the target.
