---
area: manual-render
source: blender-manual
version: 5.2
---

*****
Color
*****

In this tab you control the color of your strokes.

.. _bpy.types.FreestyleLineStyle.color:

Base Color
   The base color for this line style.

.. _bpy.ops.scene.freestyle_color_modifier_add:

Modifiers
=========

Common Options
--------------

Mix
   The modifier output can be mixed with the base property using the usual methods
   (see for example the Mix compositing node </compositing/types/color/mix_color>).
Influence
   How much the result of this modifier affects the current property.
Color Ramp
   Each modifier has color ramp <ui-color-ramp-widget> that maps the property to a stroke color.

Types
-----

- /render/freestyle/view_layer/line_style/modifiers/color/along_stroke
- /render/freestyle/view_layer/line_style/modifiers/color/crease_angle
- /render/freestyle/view_layer/line_style/modifiers/color/curvature_3d
- /render/freestyle/view_layer/line_style/modifiers/color/distance_from_camera
- /render/freestyle/view_layer/line_style/modifiers/color/distance_from_object
- /render/freestyle/view_layer/line_style/modifiers/color/material
- /render/freestyle/view_layer/line_style/modifiers/color/noise
- /render/freestyle/view_layer/line_style/modifiers/color/tangent
