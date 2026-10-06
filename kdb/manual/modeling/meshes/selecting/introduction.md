---
area: manual-modeling
source: blender-manual
version: 5.2
---

************
Introduction
************

This section describes the various tools for selecting mesh elements.

**Seealso:**

   The /interface/selecting page contains information about selecting in general.

.. _bpy.types.ToolSettings.mesh_select_mode:
.. _bpy.ops.mesh.select_mode:

Selection Modes
===============

In *Edit Mode*, there are three different selection modes: Vertex, Edge, and Face.
To switch to a different mode, click the button in the 3D Viewport's header or press
the shortcut key.

Vertex 1
   In this mode, vertices are shown as points. Unselected vertices are shown in black,
   selected vertices in orange, and the active vertex in white.
Edge 2
   In this mode, the vertices are not shown. Unselected edges are shown in black,
   selected edges in yellow, and the active edge in white.
Face 3
   In this mode, selected faces are shaded in orange, and the active face additionally has a
   white border.

.. list-table::
   :widths: 1 1 1

   * - .. figure:: /images/modeling_meshes_selecting_introduction_vertex-mode-example.png

          Vertex mode example.

     - .. figure:: /images/modeling_meshes_selecting_introduction_edge-mode-example.png

          Edge mode example.

     - .. figure:: /images/modeling_meshes_selecting_introduction_face-mode-example.png

          Face mode example.

Switching Selection Mode
------------------------

When switching modes in an "ascending" way -- from *Vertices* to *Edges* or from
*Edges* to *Faces* -- the selected elements from the old mode will only stay selected
if they form a complete element in the new mode.

For example, if all four edges of a face are selected, switching from *Edges* to *Faces*
will keep the face selected. However, any edges that do not form a complete face will be
deselected.

.. list-table::
   :widths: 1 1

   * - .. figure:: /images/modeling_meshes_selecting_introduction_edge-mode-example.png

          Selection in Edge mode.

     - .. figure:: /images/modeling_meshes_selecting_introduction_face-mode-switched-from-edge.png

          Switching to Face mode.

When switching in a "descending" way, on the other hand, all elements stay selected.
For example, if you selected a face and switch to *Edge* mode, all four of its edges
will be selected.

Multiple Selection Modes
------------------------

It's possible to enable multiple modes at the same time by holding Shift
while switching.

.. _modeling_meshes_selecting_switch-mode_expand-contract:

Expand/Contract Selection
-------------------------

When holding Ctrl while switching to a "higher" mode, the selection will be expanded,
fully including elements of the new mode that were only partially included before.

.. list-table::
   :widths: 1 1

   * - .. figure:: /images/modeling_meshes_selecting_introduction_vertex-mode-example.png

          Selection in Vertex mode.

     - .. figure:: /images/modeling_meshes_selecting_introduction_edge-mode-expanding-from-vertex.png

          Switching to Edge mode while holding Ctrl.

Holding Ctrl while switching to a "lower" mode does the opposite: it contracts the selection,
deselecting elements in the new mode that have any unselected neighbors.

X-Ray
=====

The X-Ray <bpy.types.View3DShading.show_xray> shading mode makes it possible to not just to see
occluded geometry, but also select it. As a side effect, it's no longer possible to select faces
by clicking anywhere on their surface; instead, click the selection dot in their center.

.. list-table::
   :widths: 1 1

   * - .. figure:: /images/modeling_meshes_selecting_introduction_limit-selection-to-visible-off.png

          X-ray enabled.

     - .. figure:: /images/modeling_meshes_selecting_introduction_limit-selection-to-visible-on.png

          X-ray disabled.

Select Menu
===========

All A
   Select all.
None Alt-A
   Select none.
Invert Ctrl-I
   Select the geometry that's not selected, and deselect the geometry that is.

----

Box Select <tool-select-box> B
   Interactive box selection.
Circle Select <tool-select-circle> C
   Interactive circle selection.
Lasso Select <tool-select-lasso>
   Interactive free-form selection.

----

Select Mirror </modeling/meshes/selecting/mirror> Shift-Ctrl-M
   Flips the selection to the opposite side of the mesh.
Select Random </modeling/meshes/selecting/random>
   Selects a random group of vertices, edges, or faces, based on a percentage value.
Checker Deselect </modeling/meshes/selecting/checker_deselect>
   Deselects alternating elements relative to the active one.

----

More/Less
   More <bpy.ops.mesh.select_more> Ctrl-NumpadPlus
      Grows the selection to include the elements at its border.
   Less <bpy.ops.mesh.select_less> Ctrl-NumpadMinus
      Shrinks the selection to exclude the elements at its border.
   Next Active <bpy.ops.mesh.select_next_item> Shift-Ctrl-NumpadPlus
      Selects the element that "logically follows" the two most recently selected ones.
   Previous Active <bpy.ops.mesh.select_prev_item> Shift-Ctrl-NumpadMinus
      Deselects the last selected element and makes the next-to-last one active again.

----

Select Similar </modeling/meshes/selecting/similar> Shift-G
   Selects elements similar to the current selection.

Select All by Trait </modeling/meshes/selecting/all_by_trait>
   Selects geometry that meets certain undesirable conditions.

Select Linked
   Linked
      Selects all elements that are connected directly or indirectly to the current selection.
   :reF:`Shortest Path <bpy.ops.mesh.shortest_path_select>`
      Selects the elements that lie on the shortest path between the two most recently selected ones.
   Linked Flat Faces
      Grows the selection to include the faces that are (approximately) coplanar with the currently
      selected ones.

Select Loops
   Edge Loops
      Selects the edges that lie in the same *edge loop* as the currently selected ones.
   Edge Rings
      Selects the edges that lie in the same *edge ring* as the currently selected ones.
   Select Loop Inner-Region
      Selects the faces that are enclosed by the currently selected boundary edges.
   Select Boundary Loop
      Replaces the current face selection by an edge selection of its boundaries.

Sharp Edges </modeling/meshes/selecting/sharp_edges>
   Selects edges at sharp corners.

Side of Active </modeling/meshes/selecting/side_of_active> Vertex Mode
   Selects all the vertices that are (for example) to the right of the active vertex.

By Attribute </modeling/meshes/selecting/by_attribute>
   Selects the elements for which the currently active :doc:`attribute
   </modeling/geometry_nodes/attributes_reference>` has a value of True.

Known Issues
============

Dense Meshes
------------

When selecting a region with *Box/Circle/Lasso Select* in a dense mesh with X-Ray disabled, not all vertices
in the region may get included due to overlapping. Should this happen, please zoom in or enable X-Ray as a workaround.

N-Gons in Face Select Mode
--------------------------

The face selection dot in the X-Ray and Wireframe shading modes is simply placed in the center of the face,
which may not be on the face's surface in the case of concave n-gons <n-gon>. This can make it harder
to find the dot, especially if another face sits in the cavity.
