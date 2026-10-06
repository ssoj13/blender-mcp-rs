---
area: manual-interface
source: blender-manual
version: 5.2
---

**************
Default Keymap
**************

While this isn't a comprehensive list,
this page shows common keys used in Blender's default keymap.

.. Even though this is not intended to be comprehensive,
   it could be expanded.

Selecting
=========

Blender has two main selection modes: left-click select and right-click select.
See the Select with Mouse Button <keymap-blender_default-prefs-select_with> preference.

Hovering
========

The following shortcuts can be pressed while hovering the mouse cursor
over an editable field.

.. _keymap-common-properties:

Properties
----------

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - Ctrl-C
     - Copy the (single) value of the field.
   * - Ctrl-V
     - Paste the (single) value of the field.
   * - Ctrl-Alt-C
     - Copy the entire vector or color of the field.
   * - Ctrl-Alt-V
     - Paste the entire vector or color of the field.
   * - RMB
     - Open the context menu.
   * - Backspace
     - Reset the value to its default.
   * - Minus
     - Invert the value's sign (multiply by -1.0).
   * - Ctrl-Wheel
     - Change the value in incremental steps.

       For fields with a pop-up list of values, this cycles the value.
   * - Return
     - Activates menus and toggles checkboxes.
   * - Alt
     - Hold while editing values to apply the change to all selected items
       (objects, bones, sequence-strips).

       This can be used for number fields and toggles.

Dragging
========

The following shortcuts can be used while moving/rotating/scaling an object in the 3D Viewport,
dragging the slider of a value, and so on. Note that they should be pressed after starting
the drag, not before.

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - Ctrl
     - Snap to coarse increments, making it easier to (say) rotate an object by exactly 90°.
   * - Shift
     - Make the value change more slowly in response to mouse movement, giving you more precision.
   * - Shift-Ctrl
     - Snap to fine increments.

General
=======

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - Ctrl-O
     - Open file.
   * - Ctrl-S
     - Save file.
   * - Ctrl-N
     - New file.
   * - Ctrl-Z
     - Undo.
   * - Shift-Ctrl-Z
     - Redo.
   * - Ctrl-Q
     - Quit.
   * - F1
     - Help *(context sensitive)*.
   * - F2
     - Rename active item.
   * - F3
     - Menu Search <bpy.ops.wm.search_menu>.
   * - F4
     - File context menu.
   * - F5 - F8
     - *Reserved for user actions.*
   * - F9
     - Adjust Last Operation <bpy.ops.screen.redo_last>.
   * - F10
     - *Reserved for user actions.*
   * - F11
     - Show render window.
   * - F12
     - Render the current frame.
   * - Q
     - Quick access (favorites).
   * - Ctrl-Spacebar
     - Toggle Maximize Area </interface/window_system/areas>.
   * - Ctrl-PageUp / Ctrl-PageDown
     - Next/previous Workspace </interface/window_system/workspaces>.
   * - Spacebar
     - User configurable; see keymap-blender_default-spacebar_action.
   * - Shift-Ctrl-Spacebar
     - Playback animation (reverse).

Common Editing Keys
===================

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - X
     - Delete the selected item with a confirmation dialog.
   * - Delete
     - Delete the selected item without a confirmation dialog.

Common Editor Keys
==================

These keys are shared across editors such as the 3D Viewport, UV and Graph editor.

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - A
     - Select all.
   * - Alt-A /

       Double-tap A
     - Select none.
   * - Ctrl-I
     - Invert selection.
   * - H
     - Hide selected items.
   * - Shift-H
     - Hide unselected items.
   * - Alt-H
     - Reveal hidden items.
   * - T
     - Toggle Toolbar.
   * - N
     - Toggle Sidebar.

3D Viewport Keys
================

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - MMB
     - Orbit View
   * - Shift-MMB
     - Pan View
   * - Ctrl-MMB
     - Zoom View
   * - Tab
     - Toggle Edit mode.
   * - Ctrl-Tab
     - Toggle Pose mode for armatures, or show a mode </editors/3dview/modes> switching pie menu for others.
   * - 1 - 3
     - In Edit Mode, switch between editing vertices (1), edges (2), or faces (3).

       Hold Shift to toggle one of these without disabling the others.

       Hold Ctrl to alter how the selection is transformed from the old mode to the new.

       See Mesh Selection Modes </modeling/meshes/selecting/introduction> for details.
   * - AccentGrave
     - Show 3D Viewport navigation pie menu.
   * - Ctrl-AccentGrave
     - Toggle gizmos.
   * - Shift-AccentGrave
     - Start Fly/Walk Navigation <3dview-fly-walk>.

**Seealso:**

Animation
=========

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - I
     - Insert a keyframe </animation/keyframes/introduction>.
   * - Alt-I
     - Clear the keyframe.
   * - Shift-Alt-I
     - Clear all keyframes.
   * - Ctrl-D
     - Assign a driver </animation/drivers/introduction>.
   * - Ctrl-Alt-D
     - Clear the driver.
   * - K
     - Add the property to the current
       keying set </animation/keyframes/keying_sets>.
   * - Alt-K
     - Remove the property from the current keying set.

Python Scripting
================

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - Ctrl-C
     - When pressed while hovering over an operator button <ui-operator-buttons>,
       copies its Python command to the clipboard. This command can then be used in the
       Python Console </editors/python_console> or in the Text Editor
       when writing scripts.
   * - Shift-Ctrl-C
     - When pressed while hovering over a field, copies its relative data path (also available from
       the context menu). Useful when writing drivers or scripts.
   * - Shift-Ctrl-Alt-C
     - When pressed while hovering over a field, copies its full data path.

Platform Specific Keys
======================

macOS
-----

The Cmd key can be used instead of Ctrl on macOS
for all but a few exceptions which conflict with the operating system.

List of additional macOS specific keys:

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - Cmd-Comma
     - Preferences.
