---
area: manual-animation
source: blender-manual
version: 5.2
---

***************
Selecting Bones
***************

Selection in *Pose Mode* is very similar to the one in Edit Mode </animation/armatures/bones/selecting>,
with a few deviations:
You can only select *whole bones* in *Pose Mode*, not roots/tips...

.. _bpy.ops.pose.select_all:

All
===

Select all selectable bones.

None
====

Deselect all bones, but the active bone stays the same.

Invert
======

Toggle the selection state of all visible bones.

Box Select
==========

Interactive box selection <tool-select-box>.

Circle Select
=============

Interactive circle selection <tool-select-circle>.

Lasso Select
============

See tool-select-lasso.

.. _bpy.ops.pose.select_mirror:

Select Mirror
=============

Flip the selection from one side to another.

.. _bpy.ops.pose.select_hierarchy:

Select More/Less
================

Parent [, Child ]
   You can deselect the active bone and select its immediate parent or one of its children.
Extend Parent Shift-[, Extend Child Shift-]
   Similar to *Parent*/*Child* but it keeps the active bone in the selection.

.. _bpy.ops.pose.select_grouped:

Select Grouped
==============

You can select bones, based on various properties, through the *Select Grouped* pop-up menu Shift-G:

Collection
   Selects all bones that share at least one bone collection with the active bone.
Color
   Selects all bones that have the same color as the active bone.
Keying Set
   All bones affected by active Keying Set
Children
   Select all children of currently selected bones.
Immediate Children
   Select direct children of currently selected bones.
Parents
   Select the parents of currently selected bones.
Siblings
   Select all bones that have the same parent as currently selected bones.

.. _bpy.ops.pose.select_linked:

Select Linked
=============

Selects all the bones in the chain which the active (last selected) bone belongs to.

All Forks
   Selects all bones connected to the active bone even if the branch off from the current bone.

.. list-table:: Linked bones selection.

   * - .. figure:: /images/animation_armatures_bones_selecting_single-bone.png
          :width: 320px

          A single selected bone.

     - .. figure:: /images/animation_armatures_bones_selecting_whole-chain.png
          :width: 320px

          Its whole chain selected with Linked.

Select Pattern
==============

Selects all bones whose name matches a given pattern.
Supported wild-cards: \* matches everything, ? matches any single character,
[abc] matches characters in "abc", and [!abc] match any character not in "abc".
As an example \*house\* matches any name that contains "house",
while floor\* matches any name starting with "floor".

Case Sensitive
   The matching can be chosen to be case sensitive or not.
Extend
   When *Extend* checkbox is checked the selection is extended instead of generating a new one.

.. _bpy.ops.pose.select_constraint_target:

Constraint Target
=================

Select bones used as targets for the currently selected bones
