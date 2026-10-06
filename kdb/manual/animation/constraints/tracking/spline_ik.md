---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.SplineIKConstraint:

********************
Spline IK Constraint
********************

The *Spline IK* constraint fits a chain of bones to the shape of a
Curve </modeling/curves/introduction>.
It's particularly well suited for rigging flexible body parts such as tails, tentacles,
and spines, as well as inorganic objects such as ropes.

**Note:**

   The IK constraints are special in that they modify multiple bones.
   For this reason, they ignore their position in the stack and always run after
   all other constraints on the affected bones. To apply constraints after IK,
   it is necessary to first copy the final transformation to a new bone chain,
   e.g. using Copy Transforms </animation/constraints/transform/copy_transforms>.

Options
=======

Target <rigging-constraints-interface-common-target>
   The Curve whose shape to match.

bpy.types.constraint.influence
   How strongly the constraint affects the bone chain.

Fitting
-------

Chain Length
   The number of bones affected by the constraint, starting with the owner bone
   and walking up its chain of ancestors. A value of 1 will only fit the bone
   itself, a value of 2 will fit the bone and its parent, and so on.

Even Divisions
   When disabled, each bone will cover a distance along the curve relative to its
   rest length. When enabled, each bone will cover the same distance regardless
   of its rest length.

Chain Offset
   When disabled, the bone chain will move to match the position of the curve.
   When enabled, the bone chain will stay at its original starting location and
   mimic the shape of the curve from there.

Chain Scaling
-------------

Use Curve Radius
   Whether to use the radii <modeling-curve-radius> of the curve control points
   as additional X and Z scaling factors for the bones.

Y Scale Mode
   How to stretch the bones along the length of the curve.

   :None: Reset the bones to their rest length.
   :Fit Curve: Stretch the bones to cover the entire length of the curve.
   :Bone Original: Keep the bones' original length (including their Pose Mode scale).

XZ Scale Mode
   How to scale the bones along the normals of the curve (or in other words, how to determine
   the thickness of the bones).

   :None: Reset the bones' X and Z scales to 1.
   :Bone Original: Keep the bones' original X and Z scales from Pose Mode.
   :Inverse Scale: Set the X and Z scales to the inverse of the Y scale.
   :Volume Preservation: Similar to the Stretch To <constraints-stretch-to-volume-preservation> constraint.

Use Original Scale
   Apply *Inverse Scale* or *Volume Preservation* on top of the Pose Mode bone scales,
   like in the Stretch To constraint.

**Seealso:**

   This subject is seen in-depth in
   the Armature Posing section </animation/armatures/posing/bone_constraints/inverse_kinematics/spline_ik>.

Example
=======
