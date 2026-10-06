---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.mesh.separate:
.. _object-separate:

********
Separate
********

Splits the mesh into two or more new, separate mesh objects.

Options
=======

Selection
   Disconnects the selected geometry from its surroundings and moves it into a new object.
By Material
   Splits the mesh into an object per material slot <bpy.types.MaterialSlot>,
   each containing the faces using that material slot. This is done for the whole mesh regardless
   of the selection.
By Loose Parts
   Creates an object for every independent (disconnected) piece of the original mesh.
   This is done for the whole mesh regardless of the selection.

**Seealso:**

   - bpy.ops.mesh.split for disconnecting geometry without moving it into a new object.
   - /scene_layout/object/editing/join for merging multiple objects into one.
