---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

**********
Visibility
**********

The Visibility panel controls how objects are interacted with in the viewport and in the final render.
These visibility options can also be set in the Outliner </editors/outliner/editing>.

.. _bpy.types.Object.hide_select:

Selectable
   The object is able to be selected in the 3D Viewport.

.. _bpy.types.Object.hide_viewport:
.. _bpy.types.Object.hide_render:

Show In
   Viewports
      The object will be displayed in the 3D Viewport.

   Renders
      The object is able to be in the final render, note that it will still be visible in rendered shading view.

**Seealso:**

   Cycles has additional Visibility properties
   and also Grease Pencil objects have additional Visibility properties <grease_pencil-object-visibility>.

.. _bpy.types.Object.is_holdout:

Mask
   Holdout
      Render objects as a holdout or matte, creating a hole in the image with zero Alpha <Alpha Channel>,
      to fill out in compositing </compositing/index> with real footage or another render.
