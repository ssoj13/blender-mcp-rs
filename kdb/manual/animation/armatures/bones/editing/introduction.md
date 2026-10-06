---
area: manual-animation
source: blender-manual
version: 5.2
---

************
Introduction
************

As with any other object, you edit your armature in *Edit Mode* Tab.

The set of bone editing tools is quite similar to the one for
mesh </modeling/meshes/editing/introduction> editing.

**Important:**

   One important thing to understand about armature editing is that you
   edit the *rest position* of your armature, i.e. its "default state".
   An armature in its *rest position* has all bones with *no* rotation and scaled to 1.0 in their own local space.

The different poses </animation/armatures/posing/index>
you might create afterwards are based on this rest position.
So if you modify it in *Edit Mode*, all the poses already existing will also be modified.
Thus you should in general be sure that your armature is definitive before starting to
skin </animation/armatures/skinning/index> and pose </animation/armatures/posing/index> it!

**Note:**

   Please note that some tools work on bones' joints, while others work on bones themselves.
   Be careful not to get confused.

Add Menu
========

In the 3D Viewport, Shift-A to add a new bone to your armature.
More information about adding bones can be found here </animation/armatures/bones/add_bones>.

.. _animation_armatures_bones_locking:

Locking Bones
=============

You can prevent a bone from being transformed in *Edit Mode* in several ways:

.. The active bone can be locked clicking on *Lock*
   in the *Transform* panel (3D Viewport Sidebar):

- All bones can be locked clicking on the *Lock* checkbox
  of their Transform panel in the *Bones* tab;
- Press Shift-W Toggle Bone Options --> Locked
- Select Armature --> Bone Settings --> Toggle a Setting.

*If the root of a locked bone is connected to the tip of an unlocked bone, it will not be locked*,
i.e. you will be able to move it to your liking.
This means that in a chain of connected bones, when you lock one bone,
you only really lock its tip. With unconnected bones, the locking is effective on both joints of the bone.
