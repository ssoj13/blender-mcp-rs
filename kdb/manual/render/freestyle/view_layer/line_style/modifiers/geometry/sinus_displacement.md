---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.LineStyleGeometryModifier_SinusDisplacement:

******************
Sinus Displacement
******************

The *Sinus Displacement* modifier adds a sinusoidal displacement to the stroke.

Wavelength
   How wide the undulations are along the stroke.
Amplitude
   How high the undulations are across the stroke.
Phase
   Allows "offsetting" ("moving") the undulations along the stroke.

**Tip:**

      The undulations this modifier produces look exactly the same at a Phase of `0`
      and any positive or negative multiple of the `Wavelength` set on the modifier.
      This can be used for rendering short video sequences with wavy lines
      that can then be seamlessly looped without any visual jumps in the undulations along the line.
