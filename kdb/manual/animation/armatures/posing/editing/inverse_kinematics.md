---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.ops.pose.ik_add:

**************
Add IK to Bone
**************

Adds an /animation/constraints/tracking/ik_solver to the active bone.
The operator shows a menu for selecting the target of the constraint:

Without Target
   Creates the constraint without a *Target*. Only available when no other bone
   or object is selected.
Target New Empty Object
   Creates an Empty </modeling/empties> at the bone's tail and uses it
   as the constraint target. Only available when no other bone or object is
   selected.
Target Selected Bone
   Sets the target to the selected bone (that's not active).
Target Selected Object
   Sets the target to the selected object (that's not active). Either select
   the object before selecting the Armature and entering Pose Mode,
   or select it in the Outliner </editors/outliner/introduction> when
   already in Pose Mode.

.. _bpy.ops.pose.ik_clear:

*********
Remove IK
*********

Removes the /animation/constraints/tracking/ik_solver from the selected bones.
