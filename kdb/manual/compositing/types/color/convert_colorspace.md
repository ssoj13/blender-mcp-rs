---
area: manual-compositing
source: blender-manual
version: 5.2
---

.. _bpy.types.CompositorNodeConvertColorSpace:

***********************
Convert Colorspace Node
***********************

The *Convert Colorspace* node converts images between color spaces <Color Space>.

**Note:**

    Images are already automatically converted into linear color space unless specified in the image's
    Color Space <bpy.types.ColorManagedInputColorspaceSettings.name> option.

Inputs
======

Image
   Standard color input.

Properties
==========

From, To
   The color space of the input image and the color space to convert it to.

   The list of color spaces depends on the active OCIO config <ocio-config>.
   The default supported color spaces are described in detail here:
   Default OpenColorIO Configuration

Outputs
=======

Image
   Standard color output.
