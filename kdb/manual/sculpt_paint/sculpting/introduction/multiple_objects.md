---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

*****************************
Working with Multiple Objects
*****************************

Unlike Edit Mode, there is no multi-object editing supported for Sculpt Mode. Since sculpting often involves editing
many separate objects, it is recommended to use the shortcut Alt Q while pointing at other objects, for
bpy.ops.object.transfer_mode quickly.

The advantage of using multiple objects is that each can have its own origin and modifiers.
Splitting the geometry among multiple objects can also improve the sculpt mode performance.
Alternatively objects can also be joined </scene_layout/object/editing/join>
so there is no need to switch objects.

In the case that Face Sets were already used, joining objects or creating new geometry in Edit Mode will
automatically assign new Face Sets. This makes it immediately possible to target each new geometry, for example via
auto-masking. If no Face Sets are created, use the bpy.ops.sculpt.face_sets_init operator to create them.

Face Sets and Masked geometry can also be extracted via bpy.ops.sculpt.paint_mask_extract
or sliced into a new object via bpy.ops.sculpt.paint_mask_slice.
