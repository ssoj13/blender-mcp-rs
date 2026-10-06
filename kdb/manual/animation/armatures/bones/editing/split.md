---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.ops.armature.split:

*****
Split
*****

The *Split* operator disconnects selected bones from the rest of the armature,
creating a new, unconnected bone chain.

This is useful for restructuring rigs, separating limbs, or preparing bone chains to be transformed independently.

**Note:**

   This operator only affects bone connectivity; the bones remain within the same armature object.
   To move the split bone chain to a separate object, use bpy.ops.armature.separate.
