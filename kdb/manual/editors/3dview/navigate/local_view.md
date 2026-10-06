---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.ops.view3d.localview:
.. _editors-3dview-navigate-views-local:

**********
Local View
**********

Toggle Local View
=================

Global view shows all 3D objects in the scene. Local view isolates the selected object(s)
so that they are the only ones visible in the viewport. This is useful for working on
objects that are obscured by others, or to speed up the viewport performance in heavy scenes.
Local view is contextual, meaning that it can be set per 3D Viewport.

You can toggle between *Global* and *Local View* by selecting the option
from the View menu or using the shortcut NumpadSlash.

.. list-table::

   * - .. figure:: /images/editors_3dview_navigate_local-view_local1.png
          :width: 320px

          Global View.

     - .. figure:: /images/editors_3dview_navigate_local-view_local2.png
          :width: 320px

          Local View.

**Note:**

   In Local View, the /editors/3dview/3d_cursor is not locked to the scene.
   Instead, each view has an independent cursor location.

**Tip:**

   Accidentally pressing NumpadSlash can happen rather often if you are new to Blender,
   so if a bunch of the objects in your scene seem to have mysteriously vanished,
   try pressing NumpadSlash again.

.. _bpy.ops.view3d.localview_remove_from:

Remove from Local View
----------------------

Objects can be removed from Local View by selecting them and using the *Remove from Local View* operator.
If the last remaining object is removed, Blender will automatically return to the Global View.

**Hint:**

   This is useful when working with objects in dense scenes where painstakingly selecting objects to include in
   the local view isn't practical, especially when they intersect or are obscured by objects you don't want
   to include. In this case it's simpler to select many objects in a region and enter local view, then
   remove the ones you don't need.
