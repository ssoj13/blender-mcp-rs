---
area: manual-interface
source: blender-manual
version: 5.2
---

.. _bpy.ops.ed:

***********
Undo & Redo
***********

The tools listed below will let you roll back an accidental action,
redo your last action, or let you choose to recover to a specific point,
by picking from a list of recent actions recorded by Blender.

.. _bpy.ops.ed.undo:

Undo
====

If you want to undo your last action, just press Ctrl-Z.

**Seealso:**

   Memory & Limits Preferences <prefs-system-memory-and-limits> to change undo settings.

.. _bpy.ops.ed.redo:

Redo
====

To roll back the Undo action, press Shift-Ctrl-Z.

.. _bpy.ops.screen.redo_last:

Adjust Last Operation
=====================

You can tweak the parameters of an operator </interface/operators> after running it.
In editors that support it, there is a "head-up display" panel in the bottom left
based on the last performed operation.
Alternatively, you can create a pop-up with F9 which does the same thing.

For example, if your last operation was a rotation in *Object Mode*,
Blender will show you the last value changed for the angle
(see Fig. fig-interface-redo-last-object-mode left),
where you can change your action back completely by typing Numpad0 in the *Angle* Field.
There are other useful options, based on the operator,
and you cannot only Undo actions, but change them completely using the available options.

If you are in *Edit Mode*,
Blender will also change its contents based on your last action taken.
In the second example (on the right), the last operation was a Move in Object Mode;
but a *Scale* on a Face in Edit Mode, and, as you can see,
the contents of *Adjust Last Operation* are different, because of the mode (Edit Mode)
(See Fig. fig-interface-redo-last-edit-mode right).

.. list-table:: Adjust Last Operation.

   * - .. _fig-interface-redo-last-object-mode:

     - .. _fig-interface-redo-last-edit-mode:

**Tip:**

   Some operations produce particularly useful results by using *Adjust Last Operation*.
   For example, adding a Circle in the 3D Viewport; if you reduce the *Vertices* to three,
   you get a perfect equilateral triangle.

**Tip:**

   The *Adjust Last Operation* region can be hidden by View --> Adjust Last Operation.

.. _bpy.ops.ed.undo_history:

Undo History
============

There is also an Undo History of the last actions taken, recorded by Blender.

The top of the list corresponds to the most recent actions.
A small icon of a dot next to one of the entries indicates the current status.
Rolling back actions using the *Undo History* feature will take you back to
the action you choose. Much like how you can alternate between going backward in
time with *Undo* and then forward with *Redo*,
you can hop around on the Undo timeline as much as you want as long as you do not make a new change.
Once you do make a new change, the Undo History is truncated at that point.
Selecting one of the entries in the list takes the current status to that position.

.. _bpy.ops.screen.repeat_last:

Repeat Last
===========

The Repeat Last feature will repeat your last action when you press Shift-R.

In the example images below, we duplicated a *Monkey* mesh and moved it a bit.
Using repeat Shift-R, the *Monkey* was duplicated and moved a second time.

.. list-table::

   * - .. figure:: /images/interface_undo-redo_repeat-last1.png

          Suzanne.

     - .. figure:: /images/interface_undo-redo_repeat-last2.png

          After a Shift-D and move.

     - .. figure:: /images/interface_undo-redo_repeat-last3.png

          After a Shift-R.

.. _bpy.ops.screen.repeat_history:

Repeat History
==============

The *Repeat History* feature will present you a list of the last repeated actions,
and you can choose the actions you want to repeat.
It works in the same way as the Undo History, explained above,
but the list contains only repeated actions.

**Important:**

   When you quit Blender, the complete list of user actions will be lost, even if you save your file before quitting.

**Seealso:**

   Troubleshooting section on Recovering your lost work </troubleshooting/recover>.
