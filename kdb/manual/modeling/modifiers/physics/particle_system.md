---
area: manual-modeling
source: blender-manual
version: 5.2
---

************************
Particle System Modifier
************************

The Particle System modifier is a container for Particle Systems </physics/particles/index>.

**Note:**

   By default the Particle System modifier does not take into account the modifier stack <modifier-stack>.
   Make sure to enable Use Modifier Stack
   in the Particle properties if you want Particle System modifier to take other modifiers into account.

Options
=======

As the modifier is only a container its actual options are configured in the *Particle Properties* tab.
See the Particle Systems Properties </physics/particles/index> for more information.

Converting Particle Systems
===========================

Make Instances Real
   Creates a new object of each instanced object <particle-object> or collection <particle-collection>.
   See bpy.ops.object.duplicates_make_real for more information.

Convert to Mesh
   Converts path <particle-path> particles to mesh objects.
   See bpy.ops.object.convert for more information.

Example
=======
