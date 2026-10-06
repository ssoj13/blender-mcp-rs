---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

****
Draw
****

Moves vertices inward or outward, based the average vertex normals within the brush radius.
This is a very default behavior for sculpting and can be used in most cases.

It is common to use this particular brush with heavy customization for creating many custom brushes.

Brush Settings
==============

**Note:**

   More info at sculpt-tool-settings-brush-settings-general brush settings
   and on sculpt-tool-settings-brush-settings-advanced brush settings.

VDM Displacement
================

*Vector Displacement Maps* are supported for the *Draw* brush to insert complex & overhanging shapes.
Unlike regular displacement, this uses all 3 color channels of the image
to displace geometry in three directions instead of just one.

`Download the demo file <https://www.blender.org/download/demo-files/#sculpting>`__
for more information and to try the feature out.

To use this feature, enable Vector Displacement <bpy.types.Brush.use_color_as_displacement> in the texture
panel. All stroke methods <bpy.types.Brush.stroke_method> are supported, but the recommended behavior is
*Anchored*.

Ideal images for vector displacement are open EXR files
with color clamping </render/materials/legacy_textures/colors> disabled.

**Note:**

   This feature is only supported with Area Plane mapping.
