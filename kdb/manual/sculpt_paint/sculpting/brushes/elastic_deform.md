---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

**************
Elastic Deform
**************

Used to simulate realistic deformations such as grabbing or twisting of Elastic objects.
For example, this tool works great for modeling the shape of organic objects such as humans or animals.
When pressing Ctrl, the brush deforms vertices along the normal of the active vertex.

Brush Settings
==============

General
-------

**Note:**

   More info at sculpt-tool-settings-brush-settings-general brush settings
   and on sculpt-tool-settings-brush-settings-advanced brush settings.

Unique
------

.. _bpy.types.Brush.elastic_deform_type:

Deformation
   The surface alteration that is used in the brush.

   :Grab: Used to drag a group of vertices around.
   :Bi-scale Grab:
      Like *Grab* but the falloff is more localized to the center of the brush.
   :Tri-scale Grab:
      Like *Bi-scale Grab* but the falloff is more localized to the center of the brush.
   :Scale: Displaces vertices away from the active vertex.
   :Twist: Vertices are rotated around the active vertex.

.. _bpy.types.Brush.elastic_deform_volume_preservation:

Volume Preservation
   Higher values preserve volumes more, but also lead to more bulging.
   (This value determines the poisson ratio for elastic deformation)
