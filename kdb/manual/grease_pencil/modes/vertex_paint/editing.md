---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

***************************
Editing Vertex Paint Colors
***************************

.. _bpy.ops.grease_pencil.vertex_color_set:

Set Color Attribute
===================

Sets the active color <grease-pencil-vertex-paint-brush-color> to all selected vertices.

.. _bpy.ops.grease_pencil.stroke_reset_vertex_color:

Reset Vertex Color
==================

Removes the Color Attribute information of the active strokes,
if no strokes are selected, all strokes are reset.

.. _bpy.ops.grease_pencil.vertex_color_invert:

Invert
======

Invert RGB values.

.. _bpy.ops.grease_pencil.vertex_color_levels:

Levels
======

Adjust the levels of Color Attributes.

.. _bpy.ops.grease_pencil.vertex_color_hsv:

Hue/Saturation/Value
====================

Adjust the color's HSV values.

.. _bpy.ops.grease_pencil.vertex_color_brightness_contrast:

Brightness/Contrast
===================

Adjust the color's brightness/contrast.

Sample Color
============

Samples a color from the viewport and assigns it to the active
Paint </sculpt_paint/sculpting/brushes/paint> brush.

When invoking the operator from a menu (instead of using a shortcut),
you can click and drag across multiple colors to sample their average.

This allows quickly matching the brush color to existing painted details
directly on the model.

- Press Shift-X to sample a color at the mouse cursor position.

.. - Press Shift-Ctrl-X to sample the **merged viewport color**,
..   including lighting, shading, and all visible layers.

The sampled color becomes the primary color of the active Paint brush.
