---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

***************************
Editing Vertex Paint Colors
***************************

.. _bpy.ops.paint.vertex_color_smooth:

Smooth Vertex Colors
====================

Smooth colors across vertices.

.. _bpy.ops.paint.vertex_color_dirt:

Dirty Vertex Colors
===================

Generate a dirt map gradient based on cavity.

Blur Strength
   Blur strength per iteration.
Blur Iterations
   Number of times to blur the colors (higher blurs more).
Highlight Angle
   Clamps the angle for convex areas of the mesh.
   Lower values increase the contrast but can result in clamping.
   90 means flat, 180 means infinitely pointed.
Dirt Angle
   Clamps the angle for concave areas of the mesh.
   Higher values increase the contrast but can result in clamping.
   90 means flat, 0 means infinitely deep.
Dirt Only
   When active it won't calculate cleans for convex areas.
Normalize
   Choose optimal contrast by effectively lowering
   *Highlight Angle* and increasing *Dirt Angle* automatically.
   Disabling *Normalize* allows getting consistent results across multiple objects.

.. _bpy.ops.paint.vertex_color_from_weight:

Vertex Color from Weight
========================

Converts the active weight into grayscale colors.

.. _bpy.ops.paint.vertex_color_invert:

Invert
======

Invert RGB values.

.. _bpy.ops.paint.vertex_color_levels:

Levels
======

Adjust the levels of the selected vertices.

.. _bpy.ops.paint.vertex_color_hsv:

Hue/Saturation/Value
====================

Adjust the HSV values of the selected vertices.

.. _bpy.ops.paint.vertex_color_brightness_contrast:

Brightness/Contrast
===================

Adjust the brightness/contrast of the selected vertices.

.. _bpy.ops.paint.vertex_color_set:

Set Vertex Colors
=================

Fill the active Color Attribute with the current paint color.

Affect Alpha
   Set color completely opaque instead of reusing existing alpha.

Sample Color
============

Samples a color from the viewport and assigns it to the active
Paint </sculpt_paint/sculpting/brushes/paint> brush.

When invoking the operator from a menu (instead of using a shortcut),
you can click and drag across multiple colors to sample their average.

This allows quickly matching the brush color to existing painted details
directly on the model.

- Press Shift-X to sample a color at the mouse cursor position.
- Press Shift-Ctrl-X to sample the **merged viewport color**,
  including lighting, shading, and all visible layers.

The sampled color becomes the primary color of the active Paint brush.
