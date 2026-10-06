---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

************
Cloth Filter
************

This tool works similar to the Cloth Brush </sculpt_paint/sculpting/brushes/cloth>,
however, it applies a cloth simulation to all vertices in the mesh at the same time.
Click and drag away from the object for a positive effect and towards for a negative effect.

**Tip:**

   Vertices can be "pinned" by masking </sculpt_paint/sculpting/brushes/mask> vertices
   that should remain stationary, or by using Face Sets <sculpting-editing-facesets>.

Tool Settings
=============

Filter Type
   Operation that is going to be applied to the mesh.

   :Gravity: Applies gravity to the simulation.
   :Inflate: Inflates the cloth.
   :Expand: Expands the cloth's dimensions.
   :Pinch: Pinches the cloth to the point where the cursor was when the filter started.
   :Scale:
      Scales the mesh as a Soft Body
      using the distance to the origin of the object as scale.
      This creates filter produces folds in the surface.
      The orientation of the folds can be controlled using the *Force Axis* and *Orientation*.

Strength
   The amount of effect the filter has on the mesh.

Force Axis
   Apply the force along the selected axis.

Orientation
   Orientation </editors/3dview/controls/orientation> of the axis to limit the filter force.

   :Local: Use the local axis to limit the force and set the gravity direction.
   :World: Use the world axis to limit the force and set the gravity direction.
   :View: Use the view axis to limit the force and set the gravity direction.

Cloth Mass
   Mass of each simulation particle.

Cloth Damping
   How much the applied forces are propagated through the cloth.

Use Face Sets
   Only applies the cloth forces to the vertices assigned to the Face Set
   that are under the mouse.

Use Collisions
   Enables the detection of collisions with other objects during the simulation.
   In order for the sculpt object to collide with object,
   the collision object must have Collision Physics </physics/collision> activated.
