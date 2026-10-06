---
area: manual-editors
source: blender-manual
version: 5.2
---

******************
Individual Origins
******************

While the other pivot point modes transform the whole selection around one point,
Individual Origins transforms each item around itself.

In Object Mode
==============

Each object gets transformed around its origin </scene_layout/object/origin>,
which is a point that can be chosen freely and doesn't have to be in the center.
In the example below, the orange rectangle has it in a corner instead.

The images below compare Individual Origins to /editors/3dview/controls/pivot_point/median_point.

In Edit Mode
============

Each selected element is transformed around its own centerpoint.

When you transform adjacent faces or edges, they are treated as a single element
(meaning they don't become disconnected).
