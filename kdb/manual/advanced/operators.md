---
area: manual-advanced
source: blender-manual
version: 5.2
---

*********
Operators
*********

.. _bpy.ops.wm.operator_cheat_sheet:

Operator Cheat Sheet
====================

Creates a text file in the Text Editor that gives a list of all operators
and their default values in Python syntax, along with the generated docs.
This is a good way to get an overview of all Blender's operators.

**Seealso:**

   `Blender's API documentation <https://docs.blender.org/api/current/>`__

System Operators
================

.. _bpy.ops.script.reload:

Reload Scripts
--------------

Reloads all scripts found in the scripts data folder;
any changes that have been made in the Text Editor will be lost!

.. _bpy.ops.wm.memory_statistics:

Memory Statistics
-----------------

This operator which can be found by searching "Memory Statistics"
with the Operator Search <bpy.ops.wm.search_operator>
will print useful information about memory objects, their size and user count.

**Important:**

   To fully use this operator run Blender from the console with `--debug-memory`.

.. _bpy.ops.wm.debug_menu:

Debug Menu
----------

This operator brings up a menu to set Blender into a certain debug mode.

See the
`source code <https://projects.blender.org/blender/blender/src/branch/main/source/blender/blenkernel/BKE_global.hh>`__
for a description of what each value does.

**Tip:**

   Developers can search the code for `G.debug_value` to find other possible uses for this operator.

**Note:**

   Additional debug options are available by launching Blender in debug mode or setting `bpy.app.debug = True`.

.. _bpy.ops.wm.redraw_timer:

Redraw Timer
------------

This operator brings up a menu with a list of tests
to benchmark UI render times along with undo/redo functions.

.. _bpy.ops.screen.spacedata_cleanup:

Clean Up Space-Data
-------------------

Removes unused settings for invisible editors.
