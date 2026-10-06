---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

***************************
Smear Multires Displacement
***************************

This tool deforms displacement information of
the Multires Modifier </modeling/modifiers/generate/multiresolution>,
moving the displaced vertices without affecting the base mesh.

Smearing effect can be used multiple times
over the same area without generating any artifacts in the topology.

**Tip:**

   This brush works best after using Apply Base <bpy.ops.object.multires_base_apply>.

Brush Settings
==============

General
-------

**Note:**

   More info at sculpt-tool-settings-brush-settings-general brush settings
   and on sculpt-tool-settings-brush-settings-advanced brush settings.

Unique
------

.. _bpy.types.Brush.smear_deform_type:

Deformation
   Deformation type that is used by the brush.

   :Drag: Pulls the displacement values in the direction of the brush.
   :Pinch: Pulls the displacement values towards the center of the brush,
           creating hard surface effects without pinching the topology.
   :Expand: Pushes the displacement values away from the brush center, smoothing the displacement.
