---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

**********
Draw Sharp
**********

Similar to the Draw </sculpt_paint/sculpting/brushes/draw> brush,
but it always deforms the mesh from the original coordinates
and uses the *Sharper* Falloff by default.

Draw Sharp is useful on high density meshes for creating cloth wrinkles, stylized hair or hard surface edges.
To further sharpen or polish sharp edges in the case that the mesh density is not enough,
it's recommended to use the Pinch </sculpt_paint/sculpting/brushes/pinch>,
Crease </sculpt_paint/sculpting/brushes/crease> or
Multiplane Scrape </sculpt_paint/sculpting/brushes/multiplane_scrape> brushes.

A limitation is that the brush does not remesh the sculpted surfaces
with Dyntopo <bpy.ops.sculpt.dynamic_topology_toggle> enabled.
Because of that, a better brush to use with Dyntopo can be Crease </sculpt_paint/sculpting/brushes/crease>.

Brush Settings
==============

General
-------

Direction
   On *Subtract* by default to carve in creases. More info at Direction

**Note:**

   More info at sculpt-tool-settings-brush-settings-general brush settings
   and on sculpt-tool-settings-brush-settings-advanced brush settings.
