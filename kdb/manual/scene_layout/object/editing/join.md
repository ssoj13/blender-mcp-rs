---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

.. _bpy.ops.object.join:
.. _object-join:

****
Join
****

Join merges all selected objects into the last selected *Active* object.
All object data is linked to the active object (which must be selected).
All objects must be of the same type: mesh, curve, surface or armature.
If several curves are joined, each one will keep its subtype (NURBS or Bézier).

**Note:**

   Object data has many attributes which may be handled when joining.

   Materials, vertex groups, UV and Vertex layers will be merged.

   Modifiers, constraints, groups and parent relationships are ignored
   when joining and will not be applied to the active object.
