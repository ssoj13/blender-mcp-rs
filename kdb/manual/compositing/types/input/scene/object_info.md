---
area: manual-compositing
source: blender-manual
version: 5.2
---

****************
Object Info Node
****************

The *Object Info* node outputs transformation information from an object in the scene.
This can be used to drive compositor effects using the position, rotation, or scale
of objects without manually entering values.

An *Object Info* node can be added quickly by dragging an object into the node editor.

Inputs
======

Object
   Object to get the properties from.

Outputs
=======

Transform
   Transformation Matrix containing the location, rotation and scale of the object.
Location
   Location of the object in world space.
Rotation
   Rotation of the object in world space.
Scale
   Scale of the object in world space.
