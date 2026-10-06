---
area: manual-editors
source: blender-manual
version: 5.2
---

********************
Geometry Node Editor
********************

The Geometry Node editor is used to edit Node Groups </interface/controls/nodes/groups>
which are used by the Geometry Node Modifier </modeling/modifiers/geometry_nodes>.
Such a node group can define many operations to modify an object's geometry.

.. .. figure:: /images/editors_shader-editor_main.png
..    Geometry Node Editor with an example node setup.

A list of all Geometry Nodes </modeling/geometry_nodes/index> is available in the modeling section.
Also see the Nodes </interface/controls/nodes/index> page for information on working with
nodes in general.

Interface
=========

Header
------

.. _bpy.types.SpaceNodeEditor.node_tree_sub_type:

Node Tree Sub-Type
   Geometry Nodes can have multiple contexts depending on the intended function of the node group.
   Changing the context adjusts the user interface to best fit the needs of the selected context.

   :Modifier: Used to create node groups that will be used by the /modeling/modifiers/geometry_nodes.
   :Tool: Used to create node groups that will be used to create /modeling/geometry_nodes/tools.

View
   Standard view menu.
Select
   Menu for Selecting Nodes </interface/controls/nodes/selecting>.
Add
   Menu for adding new Geometry Nodes </modeling/geometry_nodes/index>.
Node
   Menu for Editing Nodes </interface/controls/nodes/editing>.

----

Geometry Node Group
   /interface/controls/templates/data_block for creating and selecting node groups.
:bl-icon:`unpinned` / :bl-icon:`pinned` Pinned
   The pin button will keep the current node group selection fixed,
   instead of using the Active Modifier <modifier-stack-active>.
   When a node group is pinned, it will remain visible in the Geometry Node editor
   even when another object or modifier is selected elsewhere.

----

Parent Node Tree
   Jumps up a node group level. See bpy.ops.node.tree_path_parent for details.
Snapping
   Snapping options. See /interface/controls/nodes/arranging for details.
Overlays
   See Overlays <bpy.types.SpaceNodeOverlay.show_overlays>.

Toolbar
-------

Select
   See Selecting Nodes </interface/controls/nodes/selecting>.
Annotate
   See /interface/annotate_tool.
Links Cut
   See Cut Links <bpy.ops.node.links_cut>.

Sidebar
-------

Node
^^^^

This tab gives access to the active node's properties.

Tool
^^^^

This tab gives access to the active tool's settings.

View
^^^^

This tab allows managing annotations.

Group
^^^^^

This tab allows you to edit the current node group's inputs and outputs.

**Tip:**

   In the Geometry Node Modifier </modeling/modifiers/geometry_nodes>,
   you can specify values for the root node group's inputs, as well as select destination
   /modeling/geometry_nodes/attributes_reference for its outputs.

.. _editors-geometry_nodes-tool_context:

Tool Context
============

These popover menus are displayed in the header when the tool context is enabled.
These properties determine where the tool is available in the user interface.

See modeling-geometry_nodes-tools_contexts for more information.

Types
-----

The /scene_layout/object/types the tool supports.

.. _bpy.types.GeometryNodeTree.is_type_mesh:

Mesh
   The node tree supports Mesh Objects </modeling/meshes/index>.

.. _bpy.types.GeometryNodeTree.is_type_curve:

Hair Curves
   The node tree supports Curve Objects </modeling/curves/index>.

.. _bpy.types.GeometryNodeTree.is_type_grease_pencil:

Grease Pencil
   The node tree supports Grease Pencil Objects </grease_pencil/index>.

.. _bpy.types.GeometryNodeTree.is_type_pointcloud:

Point Cloud
   The node tree supports Point Cloud Objects </modeling/point_cloud/index>.

Modes
-----

The /editors/3dview/modes the tool supports.

.. _bpy.types.GeometryNodeTree.is_mode_object:

Object Mode
   The node group can be used in Object mode.

.. _bpy.types.GeometryNodeTree.is_mode_edit:

Edit Mode
   The node group can be used in edit mode.

.. _bpy.types.GeometryNodeTree.is_mode_sculpt:

Sculpt Mode
   The node group can be used in Sculpt Mode </sculpt_paint/sculpting/index>.

.. _bpy.types.GeometryNodeTree.is_mode_paint:

Draw Mode Grease Pencil
   The node group can be used in Grease Pencil Draw Mode </grease_pencil/modes/draw/index>.

Options
-------

.. _bpy.types.GeometryNodeTree.use_wait_for_click:

Wait for Click
   Wait for a mouse click input (LMB) before running the operator from a menu.
   This is useful for the /modeling/geometry_nodes/input/scene/mouse_position.
