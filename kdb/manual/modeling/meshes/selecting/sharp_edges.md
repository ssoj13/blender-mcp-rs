---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.mesh.edges_select_sharp:

******************
Select Sharp Edges
******************

Adds edges at sharp corners to the selection.

**Note:**
   This operator does not deal with edges that were manually marked as Sharp <bpy.ops.mesh.mark_sharp>.
   To select those, first select one Sharp edge by hand, then use
   Select --> Select Similar --> Sharpness to add the others.

Options
=======

Sharpness
   The minimum angle for an edge between two faces to be considered sharp. The angle is measured between
   the face normals, meaning that coplanar faces have an angle of 0° (not 180°) and sharp corners have
   greater angles than dull ones (not smaller).
