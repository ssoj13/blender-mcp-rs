---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

.. _bpy.ops.object.duplicate_move:

*********
Duplicate
*********

This will create a visually-identical copy of the selected object(s).
The copy is created at the same position as the original object and
you are automatically placed in move mode. See the examples below.

This copy is a new object, which shares data-blocks with the original object
(by default, all the materials, textures, and F-Curves), but which has copied others,
like the mesh, for example. That is why this form of duplication is sometimes called "shallow link",
because not all data-blocks are shared; some of them are "hard copied"!

**Tip:**

   You can choose which types of data-block will be linked or copied when duplicating
   in the Preferences <bpy.types.PreferencesEdit.use_duplicate>.

Examples
========

The object `Cube` was duplicated, using Shift-D. Both these cubes have
separate meshes with unique names: `Cube` and `Cube.001`.

- The original left cube is being edited, the duplicated right cube remains unchanged.
  The mesh data has been copied, not linked.
- Likewise, if one cube is edited in Object Mode, the other cube remains
  unchanged. The new object's transform properties or data-block is a copy, not linked.
- When the cube was duplicated, it inherited the material of the original cube.
  The material properties were linked, not copied.

See above if you want separate copies of the data-blocks normally linked.
