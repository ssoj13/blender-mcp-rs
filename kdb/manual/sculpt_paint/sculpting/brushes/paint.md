---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

*****
Paint
*****

Paints on the active color attribute.
Hold Shift to blur painted colors instead.

Color attribute's can be managed in the palette pop-over in the middle of the header.

**Note:**

   More information in the
   Painting Introduction </sculpt_paint/sculpting/introduction/painting>.

Brush Settings
==============

General
-------

Strength
   This settings has a different effect on this brush.
   Instead of defining the strength of each individual step in the stroke,
   it determines the overall Opacity of the applied color.

   Use the *Flow* setting instead for faster increasing of strength.

**Note:**

   More info at sculpt-tool-settings-brush-settings-general brush settings
   and on sculpt-tool-settings-brush-settings-advanced brush settings.

Unique
------

.. _bpy.types.Brush.flow:

Flow
   Amount of paint that is applied per stroke sample.
   Used to create fast/slow accumulation effect.

.. _bpy.types.Brush.wet_mix:

Wet Mix
   Amount of paint that is picked from the surface into the brush color.
   Can achieve the effect of a wet canvas.

.. _bpy.types.Brush.wet_persistence:

Wet Persistence
   Amount of wet paint that stays in the brush after applying paint to the surface.

.. _bpy.types.Brush.wet_paint_radius_factor:

Wet Paint Radius
   Ratio between the brush radius and the radius that is going to be used to sample the color to blend in wet paint.

.. _bpy.types.Brush.density:

Density
   Amount of random elements that are going to be affected by this brush.
   Use this for a more detailed airbrush effect.
   This works best on a high resolution.

.. _bpy.types.Brush.tip_scale_x:

Tip Scale X
   Scale of the brush tip in the X axis.
   This is useful for a achieving a painting stroke like a marker or paint roller.
