---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.EdgeSplitModifier:

*******************
Edge Split Modifier
*******************

The *Edge Split* modifier splits, duplicates edges within a mesh,
breaking 'links' between faces around those split edges.

The edges to split can be determined from the edge angle (i.e. angle between faces forming that edge),
and/or edges marked as Sharp <bpy.ops.mesh.mark_sharp>.

Splitting an edge affects vertex normal generation at that edge, making the edge appear sharp.
It can also be used for manual control of the smoothing process,
where the user defines which edges should appear smooth or sharp
(see Mesh Smoothing <modeling-meshes-editing-normals-shading> for other ways to do this).
If desired, both modes can be active at once.

**Note:**

   This modifier is kept mostly for historical/compatibility reasons.
   Everything it can do in shading, and much more,
   can now be achieved using custom normals <modeling_meshes_normals_custom>.

   Unless you really need the topology changes it generates, it is not advised to use it in new projects.

**Note:**

   Splitting edges can also be performed manually <bpy.ops.mesh.edge_split> in Edit Mode.

Options
=======

Edge Angle
   When enabled, an edge will be split if the angle between its
   two adjacent faces is greater than the *Split Angle*.

   Split Angle
      On 0: all edges are split. On 180: no edges are split.

Sharp Edges
   When enabled, edges will be split if they were marked as sharp <bpy.ops.mesh.mark_sharp>.

**Note:**

   Non-manifold edges will always be split.

Examples
========

.. list-table::

   * - .. figure:: /images/modeling_modifiers_generate_edge-split_example-1.png

          Flat shading.

     - .. figure:: /images/modeling_modifiers_generate_edge-split_example-2.png

          Smooth shading.

   * - .. figure:: /images/modeling_modifiers_generate_edge-split_example-3.png

          Smooth shading with Edge Split.

     - .. figure:: /images/modeling_modifiers_generate_edge-split_example-4.png

          Smooth shading with Edge Split and Subdivision Surface.
