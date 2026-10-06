---
area: manual-animation
source: blender-manual
version: 5.2
---

***************
Selecting Bones
***************

You can select and edit bones of armatures in *Edit Mode* and in *Pose Mode*.
Here, we will see how to select bones in *Edit Mode*.
Selecting bones in *Pose Mode* is similar to selecting in *Edit Mode*
with a few specific differences that will be detailed in
the posing part </animation/armatures/posing/selecting>.

Similar to vertex/edge selection </modeling/meshes/selecting/introduction> in meshes,
there are two ways to select whole bones in *Edit Mode*:

#. Directly, by selecting the bone's body.
#. Selecting both of its joints (root and tip).

This is an important point to understand,
because selecting bones' joints only might lead to non-obvious behavior,
with respect to which bone you actually select.

Note that unlike the mesh display type, the armature display type has no effect on selection behavior.
In other words, you can select a bone's joint or body the same way regardless of
the bone visualization chosen.

.. _bpy.ops.armature.select_all:

Selecting Bone Joints
=====================

To select bones' joints you have the standard selection </scene_layout/object/selecting> methods.

Inverse Selection
-----------------

As stated above, you have to remember that these selection tools are for bones' joints only,
not the bones' bodies.

For example, the *Inverse* selection option Ctrl-I
inverts the selection of bones' joints, not of bones (see *Inverse selection*).

Remember that a bone is selected only if both its joints are selected. So,
when the selection status of bones' joints is inverted, a new set of bones is selected.

.. list-table:: Inverse selection.

   * - .. figure:: /images/animation_armatures_bones_selecting_two-bones.png
          :width: 320px

          Two bones selected.

     - .. figure:: /images/animation_armatures_bones_selecting_three-ends.png
          :width: 320px

          The result of the inverse selection Ctrl-I:
          The bones joints selection has been inverted, and not the bones selection.

Selecting Connected Bone Joints
-------------------------------

Another example is: when you select the root of a bone connected to its parent,
you also implicitly select the tip of its parent (and vice versa).

**Note:**

   Remember that when selecting bones' joints,
   the tip of the parent bone is the "same thing" as the root of its children bones.

Selecting Bones
===============

By clicking on a bone's body, you will select it
(and hence you will implicitly select its root and tip).

Using Shift-click, you can add to/remove from the selection.

You also have some *advanced selection* options, based on their relations.

Pick Shortest Path Ctrl-click
   Selects the path from the active bone to the bone under the mouse.

Deselecting Connected Bones
---------------------------

There is a subtlety regarding connected bones.

When you have several connected bones selected, if you deselect one bone,
its tip will be deselected, but not its root, if it is also the tip of another selected bone.

To understand this, look at Fig. fig-rig-bone-select-deselect.

.. _fig-rig-bone-select-deselect:

.. list-table:: Bone deselection in a selected chain.

   * - .. figure:: /images/animation_armatures_bones_selecting_whole-chain.png
          :width: 320px

          A selected chain.

     - .. figure:: /images/animation_armatures_bones_selecting_two-bones.png
          :width: 320px

          Two selected bones.

After Shift-clicking "Bone.003":

- "Bone.003" 's tip (which is same as "Bone.004" 's root) is deselected.
- "Bone" is "Bone.003" 's parent. Therefore, "Bone.003" 's root is the same as the tip of "Bone".
  Since "Bone" is still selected, its tip is selected. Thus the root of "Bone.003" remains selected.

.. _bpy.ops.armature.select_mirror:

Select Mirror
=============

Flip the selection from one side to another.

.. _bpy.ops.armature.select_more:
.. _bpy.ops.armature.select_less:

More/Less
=========

More Ctrl-NumpadPlus
   Expand the current selection to the connected bones.
Less Ctrl-NumpadMinus
   Contrast the selection, deselect bones at the boundaries of each selection region.

.. _bpy.ops.armature.select_linked:

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

.. _bpy.ops.armature.select_similar:

Select Similar
==============

Children
   Extends the selection to all hierarchical descendant bones.
Immediate Children
   Extends the selection to all direct child bones.
Siblings
   Selects bones that have the same parent as the active bone.
Length
   Selects bones with a similar bone length <bpy.types.EditBone.length> under the specified *Threshold*.
Direction (Y axis)
   Select bones aligned on the Y axis (along the bone's length).
Prefix
   Select bones with matching name prefix (separated by `.`).
Suffix
   Select bones with matching name suffix (separated by `.`).
Bone Collection
   Select bones that share one or more bone collections with the active bone.
Color
   Select bones that have the same color as the active bone.
Shape
   Select bones using the same shape object (in Pose Mode).

Select Pattern
==============

Select bones by names, see Object Select Pattern <bpy.ops.object.select_pattern> for details.

.. _bpy.ops.armature.select_hierarchy:

Parent/Child
============

Parent [, Child ]
   Starting at the currently selected bones select their immediate parents or all of their children.
   The active bone is moved with the selection. When selecting children, the first connected
   child is chosen as the active. In case there are no connected bones, the first child will be active.

Extend Parent/Child
===================

Extend Parent Shift-[, Extend Child Shift-]
   Similar to *Parent*/*Child* but it keeps the current selection. The active bone is still moved.
