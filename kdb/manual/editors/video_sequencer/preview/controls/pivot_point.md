---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.types.SequencerToolSettings.pivot_point:

***********
Pivot Point
***********

The Pivot Point is the point around which images are rotated and scaled.
It's indicated by the position of the selected tool's gizmo.

**Seealso:**
   The /editors/3dview/controls/pivot_point/index of the 3D Viewport

Bounding Box Center (:bl-icon:`pivot_boundbox`)
   Use the center of the rectangle that's wrapped as tightly as possible around the selected
   images' origin points.
Median Point (:bl-icon:`pivot_median`)
   Use the averaged-out position of the selected images' origin points.
2D Cursor (:bl-icon:`pivot_cursor`)
   Use the location of the 2D Cursor <editors_sequencer_preview_2d-cursor>,
   for when you want to specify the pivot point by hand.
Individual Origins (:bl-icon:`pivot_individual`)
   Rotate/scale each image around its own origin, rather than rotating/scaling
   all of them around the same single point like the other options do.
