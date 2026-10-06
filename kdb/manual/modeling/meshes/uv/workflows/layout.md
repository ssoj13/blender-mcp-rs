---
area: manual-modeling
source: blender-manual
version: 5.2
---

***************
Layout Workflow
***************

This page contains various unwrapping tips.

Unwrapping in Multiple Steps
============================

If you unwrap an entire mesh in one go, the resulting UV map may look rather messy.
In the example below, the ear and facial features are squashed and the neck is stretched
out way too far:

While you could of course start fixing this UV map by hand, it's probably a better
idea to drop it and unwrap the mesh differently. We'll divide it into pieces,
unwrapping each one separately with the most suitable projection.

We start by selecting just the head -- excluding the eyes, ears, and neck --
and unwrapping it using bpy.ops.uv.sphere_project:

Next, select the ear, align the 3D Viewport view to look straight at it,
and unwrap it using bpy.ops.uv.project_from_view.

The UV Editor only shows the ear at this point, but don't worry: the UV coordinates
for the head are still there, just hidden. To make them visible again, either
select the head in the 3D Viewport again or enable
bpy.types.ToolSettings.use_uv_select_sync in the UV Editor.

Next, you can unwrap the neck using bpy.ops.uv.cylinder_project.

**Hint:**

   Instead of selecting and unwrapping each piece individually, you can also
   use /modeling/meshes/uv/unwrapping/seams to mark cutting lines
   around the ear and neck, then run a single bpy.ops.uv.unwrap
   to map all three parts in one go.

Once everything is unwrapped, you can select the whole mesh in the 3D Viewport to
see the full UV map. Most likely, the different UV parts or "islands" will be
overlapping; you can fix this in the following ways:

- For each island, select a vertex and press Ctrl-L to select all the vertices
  it's connected (Linked) to. You can also hover over the vertex and press L,
  or simply set the bpy.ops.uv.select_mode to *Island*.
  Once the island is selected, scale it down and move it to a place where it no
  longer overlaps the others.
- Alternatively, you can simply click bpy.ops.uv.pack_islands in the menu
  to have Blender lay out the islands automatically.

As a final step, you can align the islands to each other by moving and scaling,
then connect them using bpy.ops.uv.weld -- or better yet, bpy.ops.uv.stitch.

Refining the Layout
===================

After using the unwrapping tools, you may want to manually tweak the UV map,
for example scaling up an area so it receives more pixels of the texture
and can thus be more detailed. You can also use bpy.ops.uv.minimize_stretch
if the UV map is too warped compared to the 3D mesh.

Multiple UV Maps
================

A mesh can have more than one UV map, where each map can assign different UV coordinates
to the same 3D vertex. This is useful if, say, you need one UV map for textures and another
for storing prebaked lighting <bpy.ops.uv.lightmap_pack>.

You can add and remove UV maps in the UV Maps panel <uv-maps-panel>.

Transferring UV Maps
====================

You can copy a UV map from one mesh to another in several ways. If the meshes have
the exact same topology and only differ in vertex positions, you can use
bpy.ops.object.join_uvs. If they have (approximately) the same surface
but different topology, you can instead use
/scene_layout/object/editing/link_transfer/transfer_mesh_data.
