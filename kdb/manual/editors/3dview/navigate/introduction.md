---
area: manual-editors
source: blender-manual
version: 5.2
---

************
Introduction
************

To be able to work in the three-dimensional space that Blender uses,
you must be able to change your viewpoint as well as the viewing direction of the scene.
While we will describe the *3D Viewport* editor, most of the other editors have similar functions.
For example, it is possible to pan and zoom in the Image editor.

**Tip:**

   Some navigation tools require a middle mouse button or numpad.
   If you don't have one of these, see the Keyboard and Mouse
   page of the manual to learn how to work around this.

.. _navigation-gizmo:

Navigation Gizmo
================

The navigation gizmo can be found in the top right of the editor.

.. list-table:: Navigation Gizmo (left) and Navigation Gizmo in camera view (right).

   * - .. figure:: /images/editors_3dview_navigate_introduction_gizmo.png
          :width: 146px

     - .. figure:: /images/editors_3dview_navigate_introduction_gizmo_camera_view.png
          :width: 146px

The Orbit gizmo at the top shows the current orientation of the view.
Dragging it with LMB will orbit <bpy.ops.view3d.view_orbit> the view.
Clicking any of the axis labels will align </editors/3dview/navigate/viewpoint> the view to that axis.
Clicking the same axis again switches to the opposite side of that same axis.

The four buttons below the orbit gizmo do the following:

- Zoom the 3D Viewport <bpy.ops.view3d.zoom>
- Pan the 3D Viewport <bpy.ops.view3d.view_pan>
- Toggle the Camera View </editors/3dview/navigate/camera_view>
- Toggle the Projection </editors/3dview/navigate/projections> /
  Toggle Lock Camera to View <bpy.types.SpaceView3D.lock_camera> (in camera view)
