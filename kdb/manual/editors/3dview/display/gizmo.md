---
area: manual-editors
source: blender-manual
version: 5.2
---

***************
Viewport Gizmos
***************

.. _bpy.types.SpaceView3D.show_gizmo:

Clicking :bl-icon:`gizmo` (Show Gizmos) toggles all gizmos in the 3D Viewport.
The drop-down button displays a popover with more detailed settings,
which are described below.

Viewport Gizmos
===============

.. _bpy.types.SpaceView3D.show_gizmo_navigate:

Navigate
   Enable/disable the navigation gizmo <navigation-gizmo>.

.. _bpy.types.SpaceView3D.show_gizmo_tool:

Active Tool
   Enable/disable the gizmo of the active tool.

.. _bpy.types.SpaceView3D.show_gizmo_context:

Active Object
   Enable/disable the Object Gizmos for the active element (see below).

Object Gizmos
=============

Object Gizmos allow mouse-controlled translation, rotation and scaling in the 3D Viewport.
While they're called "object" gizmos in the popover, they also apply to other transformable
elements such as mesh vertices.

There is a separate gizmo for each operation.
Each gizmo can be used separately or in combination with the others.

A gizmo always has three color-coded axes: X (red), Y (green), and Z (blue).
You can drag an axis with LMB to transform along it. The Move and Scale gizmos
additionally have small colored squares for transforming along two axes in one go.

Various modifier keys can be used:

- Holding Ctrl at any time will toggle snapping </editors/3dview/controls/snapping>
  and also make rotation and scaling work in coarse increments.
- Holding Shift *after* pressing LMB will do the opposite of the above, "slowing down"
  the transformation relative to mouse movement to allow finer adjustments.
- Holding Shift *before* pressing LMB will perform the transformation in the plane
  that's perpendicular to the clicked axis. See view3d-transform-plane-lock.

The Gizmos popover has the following settings for object gizmos:

Orientation
   The orientation to use for the gizmo. *Default* means to use the viewport's
   Transform Orientation </editors/3dview/controls/orientation>.
   The other options override it.

.. _bpy.types.SpaceView3D.show_gizmo_object_translate:

Move
   Show the gizmo to control the location.
   Dragging the small white circle allows free movement in the viewing plane.

.. _bpy.types.SpaceView3D.show_gizmo_object_rotate:

Rotate
   Show the gizmo to control the rotation.
   Dragging the large white circle allows rotation around the viewing direction.
   Dragging the translucent white disc within that circle (only visible when
   hovering over the gizmo) allows trackball rotation <view3d-transform-trackball>.

.. _bpy.types.SpaceView3D.show_gizmo_object_scale:

Scale
   Show the gizmo to control the scale.
   Dragging the area between the small and large white circles scales along all three axes.

The latter three options are also available in a pie menu
if you have the *Grave Accent / Tilde Action* in the
Keymap Preferences </editors/preferences/keymap> set to *Gizmos*.

**Note:**

   If you're using a tool that's tied to a particular gizmo setup (the *Move*, *Rotate*,
   *Scale* and *Transform* tools), the Move/Rotate/Scale checkboxes won't have any effect.

.. list-table::
   :widths: 1 1 1 1

   * - .. figure:: /images/editors_3dview_display_gizmo_options-translate.png

          Move.

     - .. figure:: /images/editors_3dview_display_gizmo_options-rotate.png

          Rotate.

     - .. figure:: /images/editors_3dview_display_gizmo_options-scale.png

          Scale.

     - .. figure:: /images/editors_3dview_display_gizmo_options-all.png

          Combination.

**Seealso:**

   The Gizmo Preferences <bpy.types.PreferencesView.gizmo_size>.

Empty
=====

Gizmo settings for empties </modeling/empties>.

.. _bpy.types.SpaceView3D.show_gizmo_empty_image:

Image
   Show the gizmo to adjust the image size and position.

.. _bpy.types.SpaceView3D.show_gizmo_empty_force_field:

Force Field
   Show the gizmo to adjust the force field </physics/forces/force_fields/introduction>.

Light
=====

Gizmo settings for lights </render/lights/light_object>.

.. _bpy.types.SpaceView3D.show_gizmo_light_size:

Size
   Show the gizmo to adjust the Spot Size of spotlights.

.. _bpy.types.SpaceView3D.show_gizmo_light_look_at:

Look At
   Show the gizmo to adjust the direction of lights.

Camera
======

Gizmo settings for cameras </render/cameras>.

.. _bpy.types.SpaceView3D.show_gizmo_camera_lens:

Lens
   Show the gizmo to adjust the focal length (for Perspective cameras)
   or orthographic scale (for Orthographic cameras).

.. _bpy.types.SpaceView3D.show_gizmo_camera_dof_distance:

Focus Distance
   Enable the gizmo for adjusting the camera's bpy.types.CameraDOFSettings.focus_distance.
   To see this gizmo, you must also enable bpy.types.Camera.show_limits
   in the camera's viewport display properties.
