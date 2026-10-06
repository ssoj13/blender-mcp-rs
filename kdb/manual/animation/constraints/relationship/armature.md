---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.ArmatureConstraint:

*******************
Armature Constraint
*******************

The *Armature* constraint transforms an "owner" object or bone based on the
weighted pose transformations of one or more "target" bones. It's similar to the
/modeling/modifiers/deform/armature, which performs this operation for
every vertex in a mesh.

Unlike the modifier, the constraint also uses bones whose
Deform </animation/armatures/bones/properties/deform> option is disabled.

**Seealso:**

   The /animation/constraints/relationship/child_of is an alternative if
   there's only one parent. Unlike with the *Armature* constraint, this parent
   can also be an object.

Options
=======

Preserve Volume
   Prevents the owner from shrinking when the target bones rotate relative to each other.

Use Envelopes
   Uses Envelopes <armature-bones-envelope> to weaken the influence of
   target bones that are further away. For best results, set the *Weight*
   of all bones to 1.0.

**Note:**

      Unlike the modifier, the constraint does not automatically detect nearby bones.
      Every bone has to be added manually.

Use Current Location Bone constraint only
   By default, the constraint uses the rest location of the owner bone to
   find nearby envelopes and B-Bone
   segments. This option uses the bone's current location instead (so including
   the pose transformation and prior constraints).

   Objects don't have a rest location, so for them, the constraint always uses
   the current location.

Add Target Bone
   Adds a new entry to the *Bones* list.

Normalize Weights
   Normalizes the *Weights* in the *Bones* list so they add up to 1.0.

bpy.types.constraint.influence
   How strongly the constraint affects the owner.

Bones
-----

The list of target bones used to transform the owner.

Target <rigging-constraints-interface-common-target>
   The armature object containing the bone. Unlike the modifier, the constraint can use bones
   from different armatures.

Sub-Target
   The name of the bone.

.. _bpy.ops.constraint.remove_target:

:bl-icon:`x` (Remove Target)
   Removes the entry from the list.

Weight
   Weight associated with the bone. With the modifier, this weight would come from a vertex
   group instead.
