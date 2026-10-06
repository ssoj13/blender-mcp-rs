---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.ops.view3d.view_persportho:

************************
Perspective/Orthographic
************************

This operator changes the projection of the viewport camera.
Each 3D Viewport supports two different types of projection.
These are demonstrated in the Fig. below.

.. list-table::

   * - .. figure:: /images/editors_3dview_navigate_projections_view-orthographic.png
          :width: 320px

          Orthographic projection.

     - .. figure:: /images/editors_3dview_navigate_projections_view-perspective.png
          :width: 320px

          Perspective projection.

Our eyes are used to perspective viewing where distant objects appear smaller.
Orthographic projection often seems a bit odd at first,
because objects stay the same size regardless of their distance.
It is like viewing the scene from an infinitely distant point.
Nevertheless, orthographic viewing can be very useful,
because it provides a more "technical" insight into the scene,
making it easier to model and judge proportions.

Options
=======

To toggle between the two projections for the 3D Viewport, select
View --> Perspective/Orthographic or use the shortcut
Numpad5. Changing the projection for a 3D Viewport does not affect
the way the scene will be rendered. Rendering is in perspective by default.
If you need to create an orthographic rendering, select the camera, go to
the Camera tab in the Properties editor </editors/properties_editor>,
and set the Type in the Lens panel to *Orthographic*.

**Seealso:**

   - Auto-Perspective Preference
   - Camera Lens Type
   - Camera Projections
