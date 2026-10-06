---
area: manual-modeling
source: blender-manual
version: 5.2
---

*************
Vertex Groups
*************

.. _bpy.ops.object.vertex_group_assign_new:

Assign to New Group
===================

Creates a new Vertex Group </modeling/meshes/properties/vertex_groups/introduction>
and assigns the selected vertices to it, with the *Weight* that's currently set
in the /modeling/meshes/properties/vertex_groups/vertex_groups.

Assign to Active Group
======================

Assigns the selected vertices to the vertex group that's selected in the Vertex Groups panel,
with the *Weight* that's currently set in that panel.

This is the same as clicking the *Assign* button in the panel itself.

Remove from Active Group
========================

Removes the selected vertices from the vertex group that's selected in the Vertex Groups panel.
This is the same as clicking the *Remove* button in the panel itself.

Remove from All
===============

Removes the selected vertices from all vertex groups.

.. _bpy.ops.object.vertex_group_set_active:

Set Active Group
================

Sets a specific vertex group (chosen from the menu) as the active one.
This is the same as selecting the group in the Vertex Groups panel.

Remove Active Group
===================

Deletes the vertex group that's currently selected in the Vertex Groups panel.
This is the same as clicking the :bl-icon:`remove` button in the panel.

Remove All Groups
=================

Deletes all vertex groups of the mesh. This can also be done by clicking the
:bl-icon:`downarrow_hlt` button in the Vertex Groups panel and selecting
*Delete All Groups* in the popover menu.
