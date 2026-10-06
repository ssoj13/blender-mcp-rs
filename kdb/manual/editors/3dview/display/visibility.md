---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.types.SpaceView3D.show_object_viewport:
.. _bpy.types.SpaceView3D.show_object_select:

**********************
Object Type Visibility
**********************

This popover lets you control the visibility and selectability of the various object types.
For example, you can hide all the Lights in the scene with one click.

The settings only apply to the current 3D Viewport. Object types marked as unselectable
can still be selected in other viewports and in the
Outliner </editors/outliner/introduction>, for example.

**Note:**

   The object types apply to the evaluated geometry type rather than the original.
   For example, a mesh object changed to a volume with geometry nodes will be visible
   even if the *Mesh* option is unchecked.

**Seealso:**

   Outliner Restriction Columns
   for making individual objects invisible or unselectable across all viewports.
