---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.transform.skin_resize:

***********
Skin Resize
***********

Changes the X and Y skinning radii of the selected vertices for use with the
/modeling/modifiers/generate/skin. (For meshes that don't have this modifier,
activating this operator does nothing.)

To change only one of the radii, press X or Y while the operator is active,
or change a *Scale X/Y* property in the bpy.ops.screen.redo_last panel.

The radii can also be adjusted in the modeling-mesh-transform-panel in the Sidebar.

**Note:**

   If the modifier is deleted, the skinning radii will be lost.
