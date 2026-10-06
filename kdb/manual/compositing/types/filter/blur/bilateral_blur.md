---
area: manual-compositing
source: blender-manual
version: 5.2
---

.. _bpy.types.CompositorNodeBilateralblur:

*******************
Bilateral Blur Node
*******************

The Bilateral Blur node performs a high-quality adaptive blur, blurring
the image while retaining sharp edges.

It can be used for various purposes like: smoothing noisy render passes to avoid longer computation times
in example ray-traced ambient occlusion, blurry refractions/reflections, soft shadows,
or to make non-photorealistic compositing effects.

Inputs
======

Image
   Standard color input.
   If only the image input is connected, the node blurs the image depending on the edges present in the source image.
Determinator
   If connected, it serves as the source for defining edges/borders for the blur in the image.
   This has great advantage in case the source image is too noisy,
   but normals in combination with Z-buffer can still define exact borders/edges of objects.
Size
   The size of the blur in pixels.
Threshold
   Pixels are considered in the blur area if the average difference between their
   determinator and the determinator of the center pixel is less than this threshold.

Outputs
=======

Image
   Standard color output.

Example
=======

.. list-table::

   * - .. figure:: /images/compositing_types_filter_bilateral-blur_example-1-render.jpg
          :width: 320px

          Render result.

     - .. figure:: /images/compositing_types_filter_bilateral-blur_example-1-composite.jpg
          :width: 320px

          Composite.
