---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.types.SpaceProperties:

*****************
Properties Editor
*****************

The Properties editor displays settings for the active scene, object, material, and other data types.
It provides access to a wide range of context-sensitive properties used throughout Blender.

Navigation Bar
==============

Properties are grouped into tabs, shown as a vertical list of icons in the Navigation Bar region of the editor.

The Navigation Bar can be flipped to the left or right of the editor by RMB on the region and selecting
Navigation Bar --> Flip to Left/Right.

To hide the Navigation Bar, RMB on the region and select Navigation Bar --> Hide.
Use the region toggle icon as described in bpy.ops.screen.region_blend.

.. _properties-tool-tab:

Active Tool and Workspace Settings
----------------------------------

This first tab contains settings for the active tool </editors/3dview/toolbar/index> (in the 3D Viewport)
and the current workspace </interface/window_system/workspaces>.

Scene
-----

Tabs related to the active scene:

.. _properties-render-tab:

- Render: EEVEE </render/eevee/index>,
  Cycles </render/cycles/render_settings/index>, or
  Workbench </render/workbench/index> settings.
- Output </render/output/index>
- View Layer </render/layers/index>
- Scene </scene_layout/scene/properties>
- World </render/lights/world>

Collection
----------

Settings for the active Collection <scene-layout_collections_collections_tab>.

.. _properties-data-tabs:

Object
------

Tabs related to the active object. Some are only shown for certain object types.

- Object </scene_layout/object/properties/index>
- Modifiers </modeling/modifiers/index> (or Grease Pencil Modifiers </grease_pencil/modifiers/index>)
- Effects </grease_pencil/visual_effects/index>
- Particles </physics/particles/index>
- Physics </physics/index>
- Object Constraints </animation/constraints/index>

Object Data
-----------

The *Object Data* tab name remains constant, but its icon changes depending on the object type.

- Mesh </modeling/meshes/properties/object_data>
- Curve </modeling/curves/properties/index>
- Surface </modeling/surfaces/properties/index>
- Text </modeling/texts/properties>
- Metaball </modeling/metas/properties>
- Grease Pencil </grease_pencil/properties/index>

- Armature </animation/armatures/properties/index>

  - Bone </animation/armatures/bones/properties/index>
  - Bone Constraints </animation/armatures/posing/bone_constraints/index>

- Lattice </animation/lattice>

- Empty </modeling/empties>
- Speaker </render/output/audio/speaker>
- Camera </render/cameras>
- Light </render/lights/light_object>
- Light Probe </render/eevee/light_probes/index>

Object Shading
--------------

Tabs related to an object's visual appearance. Shown only when relevant.

- Material </render/materials/index>
- Texture </render/materials/legacy_textures/index>

Strips
------

Tabs related to video editing strips. Shown only when relevant.

- Strip Modifiers </video_editing/edit/montage/modifiers/index>
- Strip Properties </video_editing/edit/montage/strips/strip_properties>

.. _bpy.types.SpaceProperties.show_properties:

Visible Tabs
------------

Allows hiding specific tabs in the Properties editor.

This is especially useful for tailoring the editor to specific workflows. For example:
- In the *Video Editing* workspace, you may hide object and shading tabs to reduce clutter.
- In the *Modeling* workspace, you may hide strip-related tabs that are not relevant.

Hidden tabs can be restored at any time using this filter list.

Header
======

.. _bpy.types.SpaceProperties.search_filter:

Display Filter Ctrl-F
   Use this to search for properties by name. Matching properties remain visible; others are grayed out.
   The editor also highlights the first match and switches to the relevant tab automatically.

   Start a search with Ctrl-F, and clear it with Alt-F.

Data Context Path
   Displays the name and icon of the current data-block (e.g. Object, Material, Scene), along with its hierarchical
   path within the data structure.
   Example: *Cube* (Object) --> *Mesh* --> *Material*

.. _bpy.ops.buttons.toggle_pin:

Toggle Pin ID
   Click the pin icon to lock the editor to the current data-block, preventing it from changing when the selection
   updates. Click again to unlock and return to context-based display.

Options
-------

These options are accessible from the Options popover in the top-right corner of the editor.

.. _bpy.types.SpaceProperties.outliner_sync:

Sync with Outliner
   Controls whether clicking icons in the Outliner </editors/outliner/introduction> changes the active tab.

   :Always: Always follow the clicked Outliner icon.
   :Never: Ignore Outliner interaction.
   :Auto: Only follow when the Properties editor shares a border with the Outliner.
