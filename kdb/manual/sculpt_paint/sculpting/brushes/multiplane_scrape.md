---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

*****************
Scrape Multiplane
*****************

Scrapes the mesh with two angled planes at the same time, creating a sharp edge between them.
This is useful for creating & polishing hard surface objects.

Brush Settings
==============

General
-------

**Note:**

   More info at sculpt-tool-settings-brush-settings-general brush settings
   and on sculpt-tool-settings-brush-settings-advanced brush settings.

Unique
------

.. _bpy.types.Brush.multiplane_scrape_angle:

Plane Angle
   The angle between the two planes of the brush, pressing Ctrl inverts the angle.

.. _bpy.types.Brush.use_multiplane_scrape_dynamic:

Dynamic Mode
   When enabled, the angle is dynamically updated based on the surface under the brush.
   The *Plane Angle* then controls how much the angle will increase when applying pen pressure.
   When pressing Ctrl, it locks the plane angle to 0 degrees.

.. _bpy.types.Brush.show_multiplane_scrape_planes_preview:

Show Cursor Preview
   Displays a preview of the two planes
   and the angle they form during the stroke.
