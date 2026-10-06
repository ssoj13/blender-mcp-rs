---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

.. _bpy.ops.object.make_links_data:

*********
Link Data
*********

Performs various assignments: adding an object to a scene, giving an object the same
data or modifiers as another, and so on.

When you link two objects to the same data, changes made to one will also appear in the other.
Should you no longer want this, you can use data-system-datablock-make-single-user
to once again give each object its own data.

.. _bpy.ops.object.make_links_scene:

Link Objects to Scene
   Adds the selected objects to the specified scene. Objects can exist in multiple scenes
   at once and have the same position/animation in all of them.

----

Link Object Data
   Replaces the object data </scene_layout/object/introduction> of the
   selected objects by that of the active object <object-active>.
Link Materials
   Replaces the materials </render/materials/introduction> of the selected
   objects by those of the active object.
Link Animation Data
   Replaces the actions </animation/actions> and tracks
   of the selected objects by those of the active object.
Link Collections
   Moves the selected objects into the same
   collections </scene_layout/collections/introduction> as the active object.
Link Instance Collection
   Replaces the instance collection
   of the selected objects by that of the active object.
Link Fonts to Text
   Replaces the font of the selected text objects </modeling/texts/introduction> by that
   of the active text object.

----

Copy Modifiers
   Replaces the modifiers </modeling/modifiers/introduction> of the selected objects
   by those of the active object.
Copy Constraints
   Replaces the constraints </animation/constraints/introduction> of the selected objects
   by those of the active object.
Copy Grease Pencil Effects
   Replaces the visual effects </grease_pencil/visual_effects/introduction> of the selected
   Grease Pencil objects by those of the active object.
Copy Light Linking
   Replaces the assigned Light Linking
   collection of the selected objects with that of the active object.
Copy Shadow Linking
   Replaces the assigned Shadow Linking
   collection of the selected objects with that of the active object.

.. _bpy.ops.object.join_uvs:

Copy UV Maps
   Replaces the active UV map </editors/uv/introduction> of each selected mesh object
   by the active UV map of the active object. If a selected object doesn't have any UV maps,
   one is created.

   All objects must have matching geometry and face order. You can ensure the latter using
   /modeling/meshes/editing/mesh/sort_elements, but even then, this operator is really
   only useful if the destination is a deformed copy of the source. Use
   /scene_layout/object/editing/link_transfer/transfer_mesh_data for other cases.

----

Transfer Mesh Data
   See /scene_layout/object/editing/link_transfer/transfer_mesh_data.
Transfer Mesh Data Layout
   See /scene_layout/object/editing/link_transfer/transfer_mesh_data_layout.

----

.. _bpy.ops.object.light_linking_receivers_link:

Link Receivers to Emitter
   Adds the selected objects to the Light Linking
   collection of the active light object.

.. _bpy.ops.object.light_linking_blockers_link:

Link Blockers to Emitter
   Adds the selected objects to the Shadow Linking
   collection of the active light object.
