---
area: manual-editors
source: blender-manual
version: 5.2
---

*******************
Bounding Box Center
*******************

In this mode, the pivot point lies at the center of the bounding box, which is a box that's
wrapped as tightly as possible around the selection while still being aligned to the world axes.

In Object Mode
==============

The pivot point becomes the center of the bounding box *around the selected objects'
origin points*, not their geometry.

This means that, if you have a single object selected, the pivot point is the same
as the object's origin point </scene_layout/object/origin> -- which can be customized
and doesn't have to be in the center. In the example below, the orange rectangle
has it in a corner instead.

If you have multiple objects selected, the pivot point becomes the center
of an imaginary box around their origins.

The image below shows the difference between Bounding Box Center
and /editors/3dview/controls/pivot_point/median_point.
The latter calculates the average position of the origins,
meaning that the pivot point shifts towards the area with the most objects.

In Edit Mode
============

The pivot point becomes the center of the bounding box around the selected mesh elements.

Median Point may again give a different result.
