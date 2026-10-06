---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _armature-bone-properties:

**********
Properties
**********

Most bones' properties (except the transform ones) are regrouped in each bone's panels,
in the *Bones* tab in *Edit Mode*. Let us detail them.

Note that some of them are also available in the 3D Viewport,
through the three pop-up menus within the same entry:

- *Toggle Setting*: Shift-W or Armature --> Bone Settings --> Toggle a Setting
- *Enable Setting*: Shift-Ctrl-W or Armature --> Bone Settings --> Enable a Setting
- *Disable Setting*: Alt-W or Armature --> Bone Settings --> Disable a Setting

Display Wire
   Always display the bone as wireframe.
Deform
   (also Shift-W --> (Deform, ...)).
Multiply Vertex Group by Envelope
   (also Shift-W --> (Multiply Vertex Group by Envelope, ...)).

   These settings control how the bone influences its geometry, along with the bones' joints radius.
   This will be detailed in the skinning part </animation/armatures/skinning/index>.
Inherit Rotation
   The bone automatically rotates together with its parent in *Pose Mode*. For more details,
   see the relations page <bpy.types.EditBone.parent>.
Lock
   (also Shift-W --> (Locked, ...))
   This will prevent all editing of the bone in *Edit Mode*;
   see bone locking <animation_armatures_bones_locking>.
