---
area: manual-editors
source: blender-manual
version: 5.2
---

************
Introduction
************

The 3D Viewport is used to interact with the 3D scene for a variety of purposes,
such as modeling, animating, texture painting, etc.

Header Region
=============

The header contains various menus and controls based on
the current mode </editors/3dview/modes>.
Its items are split into three groups:

Mode & Menus
------------

Mode Ctrl-Tab
   The 3D Viewport has several modes
   used for editing different kinds of data. For example, the default Object Mode
   would let you place a character in the scene, while Pose Mode would allow
   you to pose it.

   The shortcut Ctrl-Tab brings up a pie menu for quick mode switching.
   If you have an Armature </animation/armatures/introduction> selected,
   it'll instead switch between Object Mode and Pose Mode.

   Pressing Tab will switch between Object Mode and Edit Mode for objects
   that support it.

View
   This menu offers tools for navigating </editors/3dview/navigate/introduction> in 3D space.

The other menus depend on the current mode, Object Mode menus listed below:

Select
   Contains tools for selecting <object-select-menu> objects.
Add Shift-A
   Contains a list of different objects types <objects-types> that can be added to the scene.
Object
   Contains tools for operating on objects </scene_layout/object/editing/transform/introduction>,
   such as duplicating them. A subset of these tools can also be accessed by right-clicking
   in the 3D Viewport.

Transform Controls
------------------

Transform Orientation Comma
   Used to change the Transform Orientation </editors/3dview/controls/orientation>,
   which affects the rotation of the transform gizmo.
Pivot Point Period
   Used to change the Pivot Point </editors/3dview/controls/pivot_point/index>,
   which affects the location of the transform gizmo.
Snapping Shift-Tab
   Offers options for snapping </editors/3dview/controls/snapping> items
   to others that are nearby. You can hold Ctrl to toggle snapping on/off temporarily
   (as long as the key is held).
Proportional Editing O
   Used to smoothly transform unselected items that are near the selected ones.
   See /editors/3dview/controls/proportional_editing.

Display & Shading
-----------------

Object Type Visibility
   Change which types of objects are visible/selectable in the 3D Viewport.
   See Object Type Visibility </editors/3dview/display/visibility>.
Viewport Gizmos
   Change how gizmos </editors/3dview/display/gizmo> are
   displayed in the 3D Viewport.
Viewport Overlays
   Change how overlays </editors/3dview/display/overlays> are
   displayed in the 3D Viewport.

.. _bpy.ops.view3d.toggle_xray:

Toggle X-Ray Alt-Z
   Make the whole scene transparent, allowing you to see and select items that would otherwise be occluded.
   This is a shortcut to the X-Ray
   option which can be found inside the Viewport Shading popover (see below).

   In Pose Mode, this same button controls a different setting with its own separate on/off state.
   Rather than making the scene transparent, it shows the armature in front of any geometry.

Viewport Shading
   Change the shading </editors/3dview/display/shading> of the 3D Viewport.

Toolbar Region
==============

The Toolbar contains tools depending on the current mode
(for example, modeling tools in *Edit Mode*, brush tools in *Sculpt Mode*...).

See Tools </editors/3dview/toolbar/index> for more information.

Sidebar Region
==============

The Sidebar region contains properties of the active object and tool,
as well as of the viewport itself.

See Sidebar </editors/3dview/sidebar> for more information.

Asset Shelf Region
==================

Depending on the current mode, the asset shelf may be available, providing quick access to assets
for this specific mode (for example pose assets in *Pose Mode*, brush assets in *Sculpt Mode*).

See ui-region-asset_shelf for more information.
