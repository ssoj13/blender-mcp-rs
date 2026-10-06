---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.object.hook:

*****
Hooks
*****

This menu works with the /modeling/modifiers/deform/hooks, which "parents" a set of vertices
to an external object. Transforming that object will then also transform those vertices.

The opposite operation -- parenting an external object to a vertex -- can be achieved using
/modeling/meshes/editing/vertex/make_vertex_parent.

.. _bpy.ops.object.hook_add_newob:

Hook to New Object
==================

Creates an Empty </modeling/empties> at the average position of the selected vertices,
then adds a Hook modifier that attaches the vertices to this Empty.

If no vertices are selected, the active
Vertex Group </modeling/meshes/properties/vertex_groups/introduction> is used instead.

.. _bpy.ops.object.hook_add_selob:

Hook to Selected Object
=======================

Adds a Hook modifier that attaches the selected vertices to the selected object (that is
not the currently active mesh). If no vertices are selected, the active Vertex Group is used instead.

To select another object while in Edit Mode, either click it in the
Outliner </editors/outliner/introduction> or click it with Ctrl-LMB in the 3D Viewport.

Hook to Selected Object Bone
============================

Adds a Hook modifier that attaches the selected vertices to the selected bone.
If no vertices are selected, the active Vertex Group is used instead.

To select the bone of another object while in Edit Mode, click it with Ctrl-LMB
in the 3D Viewport.

**Tip:**

   Attaching vertices to a bone is more commonly done using the
   /modeling/modifiers/deform/armature.

Assign to Hook
==============

Adds the selected vertices to an existing Hook modifier (as chosen from the menu)
and removes the unselected vertices from it.

Vertices can be assigned to multiple hooks at the same time.

.. _bpy.ops.object.hook_remove:

Remove Hook
===========

Removes a Hook modifier (as chosen from the menu) from the mesh.

Select Hook
===========

Selects the vertices that are assigned to a particular Hook modifier (as chosen from the menu).

Reset Hook
==========

Each Hook modifier stores a rest transformation for the "parent" object where the hooked
vertices stay in their original place. Clicking *Reset Hook* updates this rest transformation
to the parent object's current transformation (meaning the vertices will return to their original location).

Recenter Hook
=============

Sets the center of a particular Hook modifier (as chosen from the menu) to the position of the
/editors/3dview/3d_cursor. This is the centerpoint for calculating the *Falloff* distance
of the modifier.

The center is shown in the 3D Viewport as a black dot, and is connected to the "parent" object
with a dashed black line.
