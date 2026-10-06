---
area: manual-modeling
source: blender-manual
version: 5.2
---

*******************
Select All by Trait
*******************

.. _bpy.ops.mesh.select_non_manifold:

Non Manifold
============

Selects the non-manifold geometry of a mesh.
This entry is only available in the *Vertex* and *Edge*
selection modes <bpy.types.ToolSettings.mesh_select_mode>.

Options
-------

Extend
   Adds to the current selection instead of replacing it.
Wire
   Selects edges that do not belong to any face.
Boundaries
   Selects edges at boundaries and holes.
Multiple Faces
   Selects edges that belong to three or more faces.
Non Contiguous
   Selects edges that belong to exactly two faces with opposite normals.
Vertices
   Selects vertices that are isolated, belong to an edge that's a *Wire*
   or has *Multiple Faces*, or are the only vertex connecting two faces.

.. _bpy.ops.mesh.select_loose:

Loose Geometry
==============

Depending on the selection mode <bpy.types.ToolSettings.mesh_select_mode>,
this operator selects vertices that are not part of any edge, edges that are not
part of any face, or faces that do not share an edge with any other face.

.. _bpy.ops.mesh.select_interior_faces:

Interior Faces
==============

Selects faces that may have been accidentally created inside the mesh instead of
on the mesh's surface. Specifically, it selects faces with "abnormal" neighbors
(multiple neighbors connected to the same edge), as well as any "normal" neighbors
of those faces.

.. _bpy.ops.mesh.select_face_by_sides:

Faces by Sides
==============

Selects all faces that have a certain number of edges.
The number can be specified in the bpy.ops.screen.redo_last panel.

.. _bpy.ops.mesh.select_by_pole_count:

Poles by Count
==============

Finds vertices that are a pole, namely vertices that are connected to an irregular
number of edges. Then, selects either those poles (in vertex selection mode) or the
edges/faces connected to them (in edge/face selection mode).

.. list-table::

   * - .. figure:: /images/modeling_meshes_editing_face_select_by_pole_count_before.webp

          Before selecting poles.

     - .. figure:: /images/modeling_meshes_editing_face_select_by_pole_count_after.webp

          After selecting poles.

Options
-------

Pole Count
   Specifies the number of edges a vertex must (not) have to be considered a pole.

Type
   Defines the comparison method for determining poles:

   :Less Than: Find vertices that have fewer edges than *Pole Count*.
   :Equal To: Find vertices that have exactly *Pole Count* edges.
   :Greater Than: Find vertices that have more edges than *Pole Count*.
   :Not Equal To: Find vertices that have more or fewer edges than *Pole Count*.

Extend
   Adds to the existing selection rather than replacing it.

Exclude Non Manifold
   Skips poles that are part of non-manifold geometry.

**Hint:**

   Poles can cause ugly pinching when using the /modeling/modifiers/generate/subdivision_surface,
   so this operator is useful for finding them.

.. _bpy.ops.mesh.select_ungrouped:

Ungrouped Vertices
==================

Selects all vertices that are not part of any
vertex group </modeling/meshes/properties/vertex_groups/introduction>.
