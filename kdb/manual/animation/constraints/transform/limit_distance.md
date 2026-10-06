---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.LimitDistanceConstraint:

*************************
Limit Distance Constraint
*************************

The *Limit Distance* constraint forces an object or bone to stay further from,
nearer to, or exactly at a given distance from a target. In other words,
the owner's location is constrained to be either outside, inside,
or on the surface of a sphere centered on the target.

**Important:**

   This constraint has no effect on connected bones
   as their position is determined by their parent bone.

Options
=======

Target <rigging-constraints-interface-common-target>
   The object or bone to stay close to/keep away from.

Distance
   The limit distance, i.e. the radius of the constraining sphere. It's calculated
   automatically when selecting the first target.

   :bl-icon:`x` (Reset Distance)
      Set the *Distance* to the current distance between owner and target.

Clamp Region
   How to constrain the owner relative to the spherical boundary:

   :Inside: Keep the owner trapped within the sphere.
   :Outside: Prevent the owner from entering the sphere.
   :On Surface: Constrain the owner to the surface of the sphere.

Affect Transform
   When the owner is manually moved around, restrict not just its
   visual <constraint-visual-transform> position but also its
   *Location* in the /editors/properties_editor's *Transform* panel.

Target/Owner <rigging-constraints-interface-common-space>
   The spaces for retrieving the position of the target and for applying the constrained result to the owner.

bpy.types.constraint.influence
   How strongly the constraint affects the owner.

**Tip:**

   Evaluating both owner and target in a Custom Space
   will automatically scale the *Distance* according to the scale of the space's object/bone.

Example
=======
