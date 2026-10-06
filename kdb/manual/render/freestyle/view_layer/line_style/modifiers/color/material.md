---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.LineStyle*Modifier_Material:
.. Editor's Note: This page gets copied into:

.. --- copy below this line ---

********
Material
********

The *Material* modifier alters the base property with a new one taken from a given range mapped on
the current material under the stroke.

You can use various properties of the materials, among which many are mono-component
(i.e. give black-and-white results). In this case for the color modifier, an optional color ramp can be used to
map these gray-scale values to colored ones.
In the reverse case properties of the materials, which are multi-components
(i.e. give RGB results) the mean value will be used for Alpha and Thickness modifiers.

If used with the *Split by Material* option in the *Stroke* tab,
the result will not be blurred between materials along the strokes.
