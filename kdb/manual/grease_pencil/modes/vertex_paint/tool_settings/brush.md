---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

**************
Brush Settings
**************

Painting needs paint brushes and Blender provides a Brush panel within the Toolbar
when in *Vertex Paint Mode*.

Brush
   In the Data-Block menu <ui-data-block> you find predefined Brush presets.
   And you can create your own custom presets as needed.

Blend
   The blending mode used when applying the brush.

   See bpy.types.Brush.blend for more information.

Size
   This option controls the diameter of the brush, measured in pixels.
   F allows you to change the brush size interactively by
   dragging the mouse and then LMB (the texture of the brush should be visible inside the circle).
   Typing a number then enter while using F allows you to enter the size numerically.

   :bl-icon:`stylus_pressure` (Size Pressure)
      Adjusts the brush size based on the stylus pressure when using a Graphics Tablet <hardware-tablet>.

Strength
   How powerful the brush is when applied.

   :bl-icon:`stylus_pressure` (Strength Pressure)
      Adjusts the brush strength based on the stylus pressure when using a Graphics Tablet <hardware-tablet>.

Mode
   :Stroke: Only paint over strokes.
   :Fill: Only paint over fill areas.
   :Stroke & Fill: Paint over strokes and fill areas.

Cursor
   See the global brush settings for Cursor </sculpt_paint/brush/cursor> settings.

.. _grease-pencil-vertex-paint-brush-color:

Color Picker
============

The color of the brush. See ui-color-picker.

**Note:**

   Note that Vertex Paint works in sRGB space
   and the RGB representation of the same colors will be different between
   the paint tools and the materials that are in linear space.

Color Palette
=============

The active Color Palette. See bpy.types.PaletteColor.

Falloff
=======

See the global brush settings for Falloff </sculpt_paint/brush/falloff> settings.
