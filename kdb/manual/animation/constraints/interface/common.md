---
area: manual-animation
source: blender-manual
version: 5.2
---

******
Common
******

.. _rigging-constraints-interface-common-target:

Target
======

The *Target* is the object which the constraint should refer to.
For example, the /animation/constraints/transform/copy_location copies the location of the
target object to the constrained object.

Initially, the *Target* is empty and the constraint's icon is red, indicating it's inactive:

Once a target is selected, the icon will turn gray and the constraint will start working.

By default, the constraint uses the location of the target's Origin <Object Origin>,
but if the target is a Mesh or Lattice, it's possible to change this by
selecting a Vertex Group. The constraint will then use the weighted average of
the target's vertex positions.

Alternatively, if the target is an Armature, it's possible to select a Bone
and choose an interpolated position between its Head and Tail.
The :bl-icon:`ipo_bezier` button enables following the curved shape of
/animation/armatures/bones/properties/bendy_bones.

.. _rigging-constraints-interface-common-space:
.. _bpy.types.constraint.owner_space:
.. _bpy.types.constraint.target_space:

Space
=====

The *Target Space* is the reference frame for retrieving the location coordinates,
rotation angles, and scale factors of the *Target*. The *Owner Space* is the
reference frame for applying those numbers to the object or bone that owns the constraint.

.. _rigging-constraints-interface-common-space-types:

Space Types
-----------

World Space
   Use the transformation relative to the world axes.

Custom Space
   Use the transformation relative to an arbitrary object or bone.

Pose Space Bones Only
   Use the transformation relative to the armature object.

Local Space
   For an object, use the transformation relative to its parent.

   For a bone, use the transformation relative to its rest state, after that rest state was
   transformed by the bone's ancestors. (If the bone has no other constraints, this is its
   transformation as shown in the /editors/properties_editor while in Pose Mode.)

**Warning:**

      For objects without a parent, *Local Space* has a special meaning that's kept for backwards
      compatibility. This behavior may be removed in the future and shouldn't be relied on;
      use *World Space* instead.

Local with Parent Bones Only
   Use the transformation relative to the bone's rest state. Unlike *Local Space*, this includes
   the rotation and location difference caused by rotating any ancestor bone.

Local Space (Owner Orientation) Bone Targets Only
   This space is intended to be used with the *Owner* set to *Local Space*. It retrieves the *Local Space*
   of the Target bone, adjusted to work with the rest orientation of the Owner bone. The result is that,
   if the parent bones of both the Target and the Owner are in their rest pose, the Owner will
   undergo the same transformation as the Target *in armature space*. The following image
   demonstrates this:

   - The left hand armature contains the Target bone which we rotate manually.
   - The middle armature has a constraint that copies the *Local Space* rotation of the Target bone.
     If the Target bone is rotated around its Y axis, this Owner bone rotates by the same amount around
     *its own* Y axis.
   - The right hand armature has a constraint that copies the *Local Space (Owner Orientation)* rotation
     of the Target bone. If the Target bone is rotated around its Y axis, the Owner bone rotates around
     *that same* axis in armature space. Initially this is the same as using *Pose Space* or even
     *World Space*, but if the parent bones are rotated as well, the result will be different.

.. _bpy.types.constraint.influence:

Influence
=========

The *Influence* is a strength multiplier for the constraint.

By default, it's 1, meaning that (for example) a *Copy Location* constraint fully overrides
the object's original location.

Setting it to 0 is the same as disabling or even deleting the constraint.

Setting it between 0 and 1 will give an interpolated result. For example, a value of 0.5 would
place the object halfway between its previous location and the location of the *Target*.

The *Influence* can be keyframed, meaning constraints can be turned on and off over the course
of an animation.

.. _bpy.ops.constraint.disable_keep_transform:

:bl-icon:`cancel` Disable and Keep Transform
   Applies <bpy.ops.constraint.apply> the result of the constraint to the object's/bone's
   own transformation, but instead of deleting the constraint, disables it by setting the *Influence* to 0.
   As with applying, this may not work perfectly if the constraint is not the first in the stack.
