---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeObjectInfo:

****************
Object Info Node
****************

The *Object Info* node gives information about the object instance.
This can be useful to give some variation to a single material assigned to multiple instances,
either manually controlled through the object index, based on the object location,
or randomized for each instance. For example a Noise texture can give random colors or a Color
Ramp can give a range of colors to be randomly picked from.

Inputs
======

This node has no inputs.

Properties
==========

This node has no properties.

Outputs
=======

Location
   Location of the object in world space.
Color
   Object color, same as *Color* in the Properties --> Object Properties --> Viewport Display.
Alpha
   The Alpha Channel component of the object's viewport display color (see the *Color* output for
   more details).
Object Index
   Object pass index, same as *Pass Index*
   in the Properties --> Object Properties --> Relations.
Material Index
   Material pass index, same as *Pass Index*
   in the Properties --> Material --> Settings.
Random
   Random number unique to a single object instance. Output is a Float between 0.0 and 1.0

**Note:**

   Note that this node only works for material shading nodes;
   it does nothing for light and world shading nodes.

Example
=======
