---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.LimitScaleConstraint:

**********************
Limit Scale Constraint
**********************

The *Limit Scale* constraint applies a lower and/or upper bound to an
object or bone's scale along each axis.

**Important:**

   While it's possible to set limits of 0 or less, these will not work correctly
   and should be avoided.

Options
=======

Minimum/Maximum X, Y, Z
   The number fields determine the boundaries for each axis.
   The checkboxes determine whether each boundary applies.

**Note:**

      If a *Minimum* is higher than the corresponding *Maximum*,
      the constraint uses the *Maximum* for both.

Affect Transform
   When the owner is manually scaled, restrict not just its
   visual <constraint-visual-transform> scale but also its
   *Scale* in the /editors/properties_editor's *Transform* panel.

Owner <rigging-constraints-interface-common-space>
   The space for determining and limiting the scale of the owner.

bpy.types.constraint.influence
   How strongly the constraint affects the owner.

Example
=======
