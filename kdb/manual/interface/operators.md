---
area: manual-interface
source: blender-manual
version: 5.2
---

*********
Operators
*********

Operators execute an action the moment they're activated,
which makes them different from tools (which require some sort of input).
Operators can be started from ui-operator-buttons,
bpy.types.UIPopupMenu, or bpy.ops.wm.search_menu.
Examples of operators include adding a new object,
deleting it, or setting its shading to smooth.

Operator Properties
===================

Most operators have properties that can be adjusted to refine their result.
First run the operator (which will use its default settings),
then adjust the properties in the bpy.ops.screen.redo_last region.

Modal Operators
===============

Modal operators exist as a concept in between Tools </interface/tool_system>
and regular operators.
They require some sort of interactive input.

.. list-table::
   :align: left
   :width: 95%
   :widths: 20 80

   * - Esc, RMB
     - Cancels a modal operator.
   * - Return, LMB
     - Confirms the action of a modal operator.

Slider Operators
----------------

Slider operators are used to interactively adjust a percentage value
in the editor's ui-region-header.

You can adjust the percentage by dragging the slider left or right.
This can be made coarser (snapping in 10% increments) by holding Ctrl
and more precise by holding Shift.
For some sliders, you can toggle "overshoot" with E, which lets
you go beyond the 0-100% range.

Searching for Operators
=======================

.. _bpy.ops.wm.search_menu:

Menu Search
-----------

The Menu Search pop-up lets you search Blender's interface for a certain
operator </interface/operators> and execute it.
First narrow down the list by typing (part of) the operator's name,
then either click the operator with LMB, or navigate to it with
Down and Up and activate it with Return.

Apart from the operator names, the pop-up also shows the menus
where they're located.

**Seealso:**

   The Spacebar Action
   option in the Preferences.

.. _bpy.ops.wm.search_operator:

Operator Search
---------------

When Developer Extras <bpy.types.PreferencesView.show_developer_ui> are activated,
the Operator Search can be accessed from the Edit menu in the Topbar.
This menu searches all /interface/operators
within Blender, even if they are not exposed in a menu.
This is useful for Python developers for testing purposes.
Blender might also include a few advanced operators that are not
exposed in a menu and can only be accessed via this search menu.

**Seealso:**

   The User Preferences
   has an option to change how the search results are scored.
