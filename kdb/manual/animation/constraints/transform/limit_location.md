---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.LimitLocationConstraint:

*************************
Limit Location Constraint
*************************

The *Limit Location* constraint forces an object or bone to stay above a coordinate,
below a coordinate, or between two coordinates along one or more axes.

Like other constraints, *Limit Location* only looks at the origin </scene_layout/object/origin>
for objects; their geometry is not taken into account. This means that, if a cube's origin
is at its center and a *Limit Location* constraint is added for the coordinates of a
floor plane, the cube will still be able to sink halfway into the plane.

**Important:**

   This constraint has no effect on connected bones
   as their position is determined by their parent bone.

**Seealso:**

   /animation/constraints/relationship/floor

Options
=======

Minimum/Maximum X, Y, Z
   The number fields determine the boundaries for each axis.
   The checkboxes determine whether each boundary applies.

**Note:**

      If a *Minimum* is higher than the corresponding *Maximum*,
      the constraint uses the *Maximum* for both.

Affect Transform
   When the owner is manually moved around, restrict not just its
   visual <constraint-visual-transform> position but also its
   *Location* in the /editors/properties_editor's *Transform* panel.

Owner <rigging-constraints-interface-common-space>
   The space for determining and limiting the position of the owner.

bpy.types.constraint.influence
   How strongly the constraint affects the owner.

Example
=======
