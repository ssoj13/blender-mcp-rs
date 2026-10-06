---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.CopyTransformsConstraint:

**************************
Copy Transforms Constraint
**************************

The *Copy Transforms* constraint forces an object or bone to match the location,
rotation, and scale of a target.

Options
=======

Target <rigging-constraints-interface-common-target>
   The object or bone whose transforms to copy.

Remove Target Shear
   Removes shearing from the Target's transformation before Mixing,
   ensuring it consists purely of translation, rotation, and scale.

Mix
   Specifies how the target's current transformation (from all its constraints) is combined with the
   owner's original transformation (from its preceding constraints).

   :Replace:
      The target's transformation replaces the owner's.
   :Before Original (Full):
      The target's transformation is applied before the owner's. The result is the same as
      the owner's transformation if it were a child of the target and there was no constraint.

      If the "parent" is non-uniformly scaled and the "child" was originally rotated,
      the constraint will cause shearing, just like the default
      Inherit Scale Full <bpy.types.EditBone.inherit_scale> setting for bones.
   :Before Original (Aligned):
      Prevents shearing by scaling the "child" along its own axes instead of the axes of
      the "parent," just like the Inherit Scale Aligned
      setting for bones.
   :Before Original (Split Channels):
      Calculates each transform "channel" -- location, rotation, and scale -- separately
      from the others. This is the same as having a
      Copy Location </animation/constraints/transform/copy_location> constraint,
      a Copy Rotation </animation/constraints/transform/copy_rotation> constraint,
      and a Copy Scale </animation/constraints/transform/copy_scale> constraint
      (all using *Offset*/*Before Original*). The result may be slightly different
      with sheared inputs, however.

      The difference with *Before Original (Aligned)* is that the child's location is only
      affected by the parent's location, not by its rotation and scale.
   :After Original (Full/Aligned/Split Channels):
      Like *Before Original*, except the result is the transformation of the target
      if it were a child of the owner.

   .. list-table::

      * - .. figure:: /images/animation_constraints_transform_copy-transforms_mix-none.png

             No constraint.

        - .. figure:: /images/animation_constraints_transform_copy-transforms_mix-before-original-full.png

             With *Before Original (Full)*, the cone is scaled along the cube's Y axis, causing shearing.

      * - .. figure:: /images/animation_constraints_transform_copy-transforms_mix-before-original-aligned.png

             With *Before Original (Aligned)*, the cone is scaled along its own Y axis, preventing shearing.

        - .. figure:: /images/animation_constraints_transform_copy-transforms_mix-before-original-split-channels.png

             With *Before Original (Split Channels)*, the cone's location is independent of the cube's
             rotation and scale.

Target/Owner <rigging-constraints-interface-common-space>
   The spaces for retrieving the transforms from the target and for applying them to the owner.

bpy.types.constraint.influence
   How strongly the constraint affects the owner.

Example
=======
