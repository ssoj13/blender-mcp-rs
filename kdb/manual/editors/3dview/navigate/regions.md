---
area: manual-editors
source: blender-manual
version: 5.2
---

************
View Regions
************

.. _bpy.ops.view3d.clip_border:

Clipping Region
===============

Allows you to define a clipping region to limit the 3D Viewport display to a portion of 3D space.
It can assist in the process of working with complex models and scenes.

Once activated, you have to draw a rectangle with the mouse.
It becomes a clipping volume of four planes:

- A right-angled `parallelepiped <https://en.wikipedia.org/wiki/Parallelepiped>`__
  (of infinite length) if your view is orthographic.
- A rectangular-based pyramid (of infinite height) if your view is in perspective.

Once clipping is used, you will only see what's inside the volume you defined.
Tools such as paint, sculpt, selection, transform snapping, etc.
will also ignore geometry outside the clipping bounds.

To delete this clipping, press Alt-B again.

Example
-------

.. list-table:: Region/Volume clipping.

   * - .. figure:: /images/editors_3dview_navigate_regions_border1.png
          :width: 320px

          Selecting a region.

     - .. figure:: /images/editors_3dview_navigate_regions_border2.png
          :width: 320px

          Region selected.

     - .. figure:: /images/editors_3dview_navigate_regions_border3.png
          :width: 320px

          View rotated.

The *Region/Volume clipping* image shows an example of using the clipping tool with a cube.
Start by activating the tool with Alt-B.
This will generate a dashed cross-hair cursor.
Click with the LMB and drag out a rectangular region.
Now clipping is applied against that region in 3D space.
Use the MMB to rotate
the view and you will see that only what is inside the clipping volume is visible.
All the editing tools still function as normal, but only within the clipping volume.

The dark gray area is the clipping volume itself.
Once clipping is deactivated with another Alt-B,
all of 3D space will become visible again.

.. _bpy.ops.view3d.render_border:
.. _bpy.ops.view3d.clear_render_border:
.. _editors-3dview-navigate-render-region:

Render Region
=============

Allows you to limit rendering to a 2D rectangular area. If you're busy tweaking
just a small part of the scene, it can be quite wasteful to have the whole viewport in
Rendered shading mode <view3d-viewport-shading> or make full-frame renders,
so this feature lets you save time.

You can define Render Regions in two different contexts:

- If you define one while in Camera View </editors/3dview/navigate/camera_view>,
  it will apply not just to the viewport, but also to the final render.
  If you want to temporarily disable this region rather than clearing it entirely,
  you can do so in the Output tab of the Properties <bpy.types.RenderSettings.use_border> editor.
- If you define one while not in Camera View, it will only apply to the viewport.
  If you want to temporarily disable this region rather than clearing it entirely,
  you can do so in the Sidebar <bpy.types.SpaceView3D.use_render_border>.

Both Render Regions can exist at the same time.

.. list-table:: Render region and associated render.
   :widths: 65 35

   * - .. figure:: /images/editors_3dview_navigate_regions_render-border-1.png

     - .. figure:: /images/editors_3dview_navigate_regions_render-border-2.png

**Note:**

   Render regions only apply to the viewport when using Cycles, not when using EEVEE.
   However, they always affect the final render.

**Seealso:**

   bpy.ops.view3d.zoom_border.
