---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.mesh.select_mirror:

*************
Select Mirror
*************

Flips the selection to the opposite side of the mesh.

Options
=======

Axis
   The local axis around which to mirror. The mesh's origin
   is used as the centerpoint.

Extend
   Add to the selection instead of replacing it.

It's possible to select multiple axes by holding Shift. If *Extend* is disabled,
this will mirror once across the combination of all the axes. If *Extend* is enabled,
this will mirror across each possible combination of the axes.
