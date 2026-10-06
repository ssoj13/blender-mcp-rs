---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

****
Clay
****

Similar to the Draw </sculpt_paint/sculpting/brushes/draw> brush,
but includes settings to adjust the sculpt plane <bpy.types.Brush.sculpt_plane>
on which the brush acts. That's because it behaves like a combination of the
Plane </sculpt_paint/sculpting/brushes/plane> and *Draw* brushes.

This brush is useful for building and removing volumes and shapes like real clay,
because it flattens details as you add/subtract from the surfaces.

If used together with Dyntopo <bpy.ops.sculpt.dynamic_topology_toggle>
it's easy to continuously build shapes, even in a single stroke.

Brush Settings
==============

General
-------

Hardness
   Slightly higher by default. This makes the profile of the brush more noticeable.
   More info at Hardness

Auto-Smooth
   Enabled by default for a consistent smoothing effect.
   With lower brush strength (for example with lower pen pressure) the smoothing effect will be more noticeable
   and can be used to create and then blend/polish shapes in a single stroke.
   Enable Pressure to modulate the use of auto-smooth even more with pen inputs.
   More info at Auto-Smooth

**Note:**

   More info at sculpt-tool-settings-brush-settings-general brush settings
   and on sculpt-tool-settings-brush-settings-advanced brush settings.
