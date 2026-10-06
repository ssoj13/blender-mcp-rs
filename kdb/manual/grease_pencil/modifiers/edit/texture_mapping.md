---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

.. _bpy.types.GreasePencilTextureModifier:

************************
Texture Mapping Modifier
************************

The *Texture Mapping* Modifier change the strokes texture UV position.

Options
=======

Mode
   The texture transformation will be applied to the stroke/fill or stroke UVs.

   :Stroke:
      Stroke Fit Method
         Selects the texture fitting method.

         :Constant Length: The texture keep a consistent length along the strokes.
         :Stroke Length: The texture is normalized to fit the stroke length.

      UV Offset
         Moves the texture along the strokes.

      Rotation
         Rotates the points of the strokes.

**Note:**

         The *Rotation* option is limited to a range of -90 to 90 degrees.

      Scale
         Factor for the texture scale.

   :Fill:
      Fill Rotation
         Sets the texture angle.

      Offset
         Moves the texture origin.

         X, Y

      Scale
         Factor for the texture scale.

Influence
---------

See grease-pencil-modifier-influence-filters.

Example
=======

.. list-table:: Opacity Factor samples.

   * - .. figure:: /images/grease-pencil_modifiers_color_texture-mapping_1.png
          :width: 200px

          Rotation: 0°.

     - .. figure:: /images/grease-pencil_modifiers_color_texture-mapping_2.png
          :width: 200px

          Rotation: 45°.

     - .. figure:: /images/grease-pencil_modifiers_color_texture-mapping_3.png
          :width: 200px

          Rotation: 90°.
