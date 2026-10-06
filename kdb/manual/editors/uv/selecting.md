---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.ops.uv.select:

*************
Selecting UVs
*************

Much like the 3D Viewport, the UV Editor has selection mode buttons in the header,
as well as a *Select* menu.

.. _bpy.types.ToolSettings.use_uv_select_sync:

Sync Selection
==============

When enabled (the default), the UV Editor and 3D Viewport share a synchronized selection state.
Selecting components (vertices, edges, or faces) in one editor will automatically select the
corresponding elements in the other.

With *Sync Selection* enabled, all faces are visible in the UV Editor at all times,
since UV visibility follows mesh selection in the 3D Viewport.

Viewport Selection
   Selecting a vertex, edge, or face in the 3D Viewport selects all its corresponding UV elements.
UV Selection
   Selecting a UV will select vertices, edges and faces in the 3D viewport.

   With vertex & edge selection modes it is possible for only *some* of the UV's attached to a vertex to be selected.

When disabled, only the UVs belonging to the currently selected faces in the 3D Viewport are shown.
Selections in the UV Editor are independent, allowing individual UV vertices and edges to be selected
even if they correspond to the same mesh vertex or edge. Selecting in one editor no longer affects
the other.

**Note:**

   Currently, only some 3D Viewport selection operations preserve per-UV selection data, including
   basic picking, box, circle, and lasso selection. Other operators, such as *Select Random* or
   *Select Similar*, will reset the stored UV selection, causing all UVs connected to selected mesh
   elements to become selected. Support for additional operators may be added later.

**Hint:**

   Internally, UV selection data is stored per face corner and created on demand to avoid overhead.
   Python scripts that modify mesh selections can use the API functions to synchronize or clear the
   UV selection state as needed.

.. _bpy.ops.uv.select_mode:
.. _bpy.types.ToolSettings.uv_select_mode:

Selection Mode
==============

:Vertex: 1 Select vertices.
:Edge: 2 Select edges.
:Face: 3 Select faces.

If *Sync Selection* is enabled, you can hold Shift while clicking a selection mode
to activate multiple modes at the same time.

**Seealso:**

   Mesh Selection

.. _bpy.types.ToolSettings.use_uv_select_island:

UV Island Selection
===================

Select contiguous groups of faces that are connected in the UV map.

.. _bpy.types.ToolSettings.uv_sticky_select_mode:

Sticky Selection Mode
=====================

Options for automatically selecting additional UV vertices.

Disabled
   Each UV vertex can be selected independently of the others.

Shared Location
   Automatically select UV vertices that correspond to the same mesh vertex
   and have the same UV coordinates.
   This is the default and gives the illusion that multiple faces in a UV map
   can share the same vertex; in reality, they have separate vertices that overlap.

Shared Vertex
   Automatically select UV vertices that correspond to the same mesh vertex,
   even if they have different UV coordinates.

Select Menu
===========

.. _bpy.ops.uv.select_all:

All A
   Selects all UV elements.

None Alt-A
   Deselects all UV elements.

Invert Ctrl-I
   Inverts the current selection.

Box Select Alt-B
   See Box Select <bpy.ops.*.select_box>.

Box Select Pinned Ctrl-B
   Like *Box Select*, but only selects pinned <bpy.ops.uv.pin> UV vertices.

Circle Select
   See Circle Select <bpy.ops.*.select_circle>.

Lasso Select
   See Lasso Select <bpy.ops.*.select_lasso>.

More/Less Ctrl-NumpadPlus, Ctrl-NumpadMinus
   Expands/contracts the selection to/from adjacent elements.

All by Trait
   Tile
      See bpy.ops.uv.select_tile.
   Pinned Shift-P
      See bpy.ops.uv.select_pinned.
   Overlap
      See bpy.ops.uv.select_overlap.
   Winding
      See bpy.ops.uv.select_by_winding.

.. _bpy.ops.uv.select_similar:

Select Similar
==============

Selects UV elements that are similar to the active one in some way.
The bpy.ops.screen.redo_last panel provides several options:

Type
   The property to compare. Which properties are available depends on the
   Selection Mode <bpy.types.ToolSettings.uv_select_mode>.

   Vertex Selection Mode
      :Pinned:
         Selects vertices with the same pinned <bpy.ops.uv.pin> state.

   Edge Selection Mode
      :Length:
         Selects edges with a similar length in the UV map.
      :Length 3D:
         Selects edges with a similar length in the 3D mesh.
      :Pinned:
         Selects edges with the same pinned state.

   Face Selection Mode
      :Area:
         Selects faces with a similar area in the UV map.
      :Area 3D:
         Selects faces with a similar area in the 3D mesh.
      :Material:
         Selects faces that have the same Material </render/materials/index>.
      :Object:
         Selects faces that belong to the same object.
         This is useful when multiple objects are in Edit mode at once.
      :Polygon Sides:
         Selects faces with a similar number of edges.
      :Winding:
         Selects faces that have the same orientation
         (facing upwards or downwards in the UV map).

   Island Selection Mode
      :Area:
         Selects islands with a similar area in the UV map.
      :Area 3D:
         Selects islands with a similar area in the 3D mesh.
      :Amount of Faces in Island:
         Selects islands with a similar number of faces.

Compare
   The comparison operator.

   :Equal:
      Select elements whose value is equal.
   :Greater:
      Select elements whose value is greater or equal.
   :Less:
      Select elements whose value is less or equal.

Threshold
   Tolerance for values that are almost, but not quite the same.
   A higher threshold will select more elements.

Select Linked
=============

.. _bpy.ops.uv.select_linked:

Linked
------

Selects all UV elements that are connected to the currently selected ones.

This operation expands the selection to include all UVs that are part of the same
connected UV island. Connectivity is determined by shared UV edges, not by the
underlying mesh topology.

This is useful for quickly selecting an entire UV island when only a portion
of it is currently selected, such as when adjusting layout, packing, or applying
transforms to a whole island.

Delimit
   Restricts how linked UV selection expands.

   :Seam: Stop selection across edges marked as seams.
   :Sharp: Stop selection across sharp edges.
   :Material: Stop selection across faces using different materials.

.. _bpy.ops.uv.shortest_path_select:
.. _bpy.ops.uv.shortest_path_pick:

Shortest Path
-------------

Selects all the UV elements along the shortest path between two elements:
the two selected elements when activated using the menu,
or the active one and the clicked one when activated using the shortcut.

Face Stepping
   For vertices: allows the path to step across faces, following their diagonal
   rather than their edges.

   For edges: selects disconnected edges that are perpendicular to the path
   (edge ring), rather than connected edges along the path (edge loop).

   For faces: allows the path to go through faces that only share a vertex,
   rather than an edge.

Topology Distance
   Calculates the distance by simply counting edges rather than measuring their lengths.

Fill Region Shift-Ctrl-LMB
   Selects all shortest paths rather than just one.

The following settings allow selecting elements at regular intervals,
creating a "dashed line" rather than a continuous one.

Deselected
   The number of deselected elements in the repetitive sequence.

Selected
   The number of selected elements in the repetitive sequence.

Offset
   The number of elements to offset the sequence by.

**Seealso:**

   Mesh edit Select Shortest Path <bpy.ops.mesh.shortest_path_select>.

Select Split
============

"Detaches" the selected faces so they can be moved elsewhere without affecting
their neighbors.

**Hint:**

   Unlike Split Selection </modeling/meshes/editing/mesh/split> for meshes,
   which physically disconnects faces, this is a pure selection operator.
   In UV space, the faces were never connected to begin with;
   it only seemed that way because *Sticky Selection* automatically selected the
   vertices of neighboring faces. *Select Split* deselects those vertices again.

   As an alternative to *Select Split*, you can set the *Sticky Selection Mode*
   to *Disabled*.

Select All by Trait
===================

.. _bpy.ops.uv.select_tile:

Select Tile
-----------

Selects UV faces that lie within a specific UV tile, primarily intended for use
with UDIM workflows </modeling/meshes/uv/workflows/udims>.

The initial tile is determined by the position of the 2D Cursor.
All UV faces whose coordinates fall inside that tile are selected.

This operator is useful when working with multi-tile UV layouts, allowing you to
quickly isolate and operate on UVs assigned to a particular UDIM tile, such as for
packing, transforming, or baking textures.

To select a different tile, move the 2D Cursor to the desired tile location
and run the operator again.

.. _bpy.ops.uv.select_pinned:

Select Pinned
-------------

Selects all pinned UVs in the UV Editor.

Pinned UVs are constrained during unwrapping and certain transform operations,
allowing them to stay fixed while other UVs are adjusted.
This operator is useful for quickly identifying or modifying pinned regions,
such as when refining UV layouts or controlling unwrap behavior.

.. _bpy.ops.uv.select_overlap:

Select Overlap
--------------

Selects all UV faces that overlap with one another in the UV Editor. Selects the entire island when island selection mode is enabled

Overlapping UVs share the same or partially intersecting texture space,
which can cause rendering artifacts, texture bleeding, or incorrect baking results.
This operator helps identify problematic areas in a UV layout.

It is commonly used when preparing UVs for texture baking, lightmaps,
or game engine export, where overlapping UVs are often undesirable.
Once selected, overlapping faces can be moved, scaled, or repacked
to resolve the overlap.

.. _bpy.ops.uv.select_by_winding:

Select by Winding
-----------------

Selects faces by their winding - positive or negative,
where a negative winding would mirror the content of an image displayed on the mesh.

This can be useful to detect problems when flipping the UV mapping isn't desired.

.. _bpy.ops.uv.select_loop:

Select Edge Loop
================

Holding Alt while clicking an edge selects that edge and then expands the selection
as far as possible in the two directions parallel to it.
While this works for selecting edge loops that wrap around a mesh,
it also works for partial loops.

You can additionally hold Shift to extend the current selection
rather than replacing it.

**Seealso:**

   Mesh edit Select Edge Loops <bpy.ops.mesh.loop_select>.

Select Edge Ring
================

Holding Ctrl-Alt while clicking an edge selects that edge and then expands the selection
as far as possible in the two directions perpendicular to it.
While this works for selecting edge rings that wrap around a mesh,
it also works when a complete ring does not exist.

You can additionally hold Shift to extend the current selection
rather than replacing it.

**Seealso:**

   Mesh edit Select Edge Rings <bpy.ops.mesh.select_edge_ring_multi>.
