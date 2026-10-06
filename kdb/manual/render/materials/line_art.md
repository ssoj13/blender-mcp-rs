---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.MaterialLineArt:

********
Line Art
********

.. _bpy.types.MaterialLineArt.use_material_mask:

Material Mask
   Material masks are a way to provide Line Art extra information about faces that caused the occlusion.
   So edges occluded by those faces can be selected to have different styles.

.. _bpy.types.MaterialLineArt.use_material_mask_bits:

Masks
   The layer to include faces of the current material.

.. _bpy.types.MaterialLineArt.mat_occlusion:

Levels
   Faces with this material will behave as if it has set number of layers in occlusion.

.. _bpy.types.MaterialLineArt.use_intersection_priority_override:
.. _bpy.types.MaterialLineArt.intersection_priority:

Intersection Priority
   Assigns an intersection priority value for this material.
   Note that this priority takes precedent over Object
   or Collection <bpy.types.Collection.lineart_intersection_priority> priority values.
   The intersection line will be included into the object with the higher intersection priority value.
