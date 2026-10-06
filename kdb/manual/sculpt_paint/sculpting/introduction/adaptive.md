---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

*******************
Adaptive Resolution
*******************

In order for sculpting to give accurate and predictable results, Blender needs enough geometry.
Instead of starting out with a highly subdivided mesh, add geometry dynamically
by using either of the following adaptive sculpting methods.

Voxel Remesher
==============

"Voxel remeshing" rebuilds the geometry with a perfectly even distributed topology.
Depending on the set voxel size, this will lead to a lower or higher resolution.

This technique is especially useful to block out the initial shape of an object.
It also has the advantage of removing any overlapping geometry and creating a manifold volume as a result.

Any currently used mask, face sets and color attributes will be re-projected on the remeshed result.
Reaching high vertex counts should still be achievable with this technique, depending on the used hardware.

**Note:**

   This technique will not work on objects that do not have an enclosed volume.
   Make sure to fill any holes in the mesh before remeshing.
   Or avoid any holes in the mesh/volume that are larger than the defined voxel size.

**Tip:**

   If in doubt, you can fill all holes in edit mode or by using the
   Mask Slice and Fill Holes
   operation to fill all holes in the mesh. If nothing is masked, it only fills any holes.

      this is a useful workaround. But once the voxel remesher automatically checks for holes
      or a dedicated Fill Holes operation is added, this tip should be removed.

To more easily access this feature, use the shortcuts R to define the resolution,
and Ctrl-R to execute the remeshing.

**Seealso:**

   More information at Remesh </sculpt_paint/sculpting/tool_settings/remesh>.

.. _dyntopo_introduction:

Dyntopo
=======

Dynamic topology (aka Dyntopo) is a dynamic tessellation
sculpting method that automatically adds and removes topology under the brush.

Unlike the Voxel Remesher, this makes it possible to sculpt complex shapes
without thinking about the resolution or topology.
It also allows to define a different resolution wherever necessary.
Much more complex base mesh sculpting is especially useful with this technique.

The disadvantages of this technique are a slower performance and limited support for some sculpt mode features.
Custom attributes like Color Attributes, UV Maps and Face Sets are also lost or corrupted when using Dyntopo.

This feature shares the same shortcuts with voxel remeshing when enabled.
Use R to define the resolution and Ctrl-R to flood fill the resolution (if Constant Detail is used).

**Note:**

   Because Dyntopo and the Voxel Remesher are mutually exclusive and cannot be used at the same time,
   both use the same shortcut to define the remeshing resolution.

Brushes like Density </sculpt_paint/sculpting/brushes/simplify>,
Snake Hook </sculpt_paint/sculpting/brushes/snake_hook>
and Clay Strips </sculpt_paint/sculpting/brushes/clay_strips> work especially well with this feature.

**Seealso:**

   More information at Dyntopo </sculpt_paint/sculpting/tool_settings/dyntopo>.

Multiresolution
===============

The Multiresolution Modifier can be used for subdivision based sculpting.
This means the object will be subdivided, similar to the
Subdivision Surface Modifier </modeling/modifiers/generate/subdivision_surface>,
only that the subdivisions can be freely sculpted for very high resolution detailing.

**Note:**

   For this technique it is highly recommended to use on a clean topology base mesh.
   This means the base mesh should be only made of quads
   and avoid non-manifold faces, as well as poles with two connected edges.
   More information at bpy.ops.object.quadriflow_remesh Remeshing for an automatic retopology method.

This technique has the advantage of sculpting with multiple resolutions,
meaning you have the ability to sculpt on any level of subdivision.
This allows to add a much higher resolution of details for rendering and sculpting,
while displaying lower resolutions for better viewport performance.
It also allows sculpting on lower resolutions any time for broader changes.

As an example, you can sculpt general proportions in subdivision level 1,
add high resolution details in level 4 and switch back to subdivision 1 to correct the shape further.

The disadvantages are that you may end up with some mesh distortions
because the topology is not dynamic like voxel remeshing and dyntopo.
The topology should also not be changed once already subdivided,
since any edits to the base mesh will result in corrupted subdivision details.

**Tip:**

   Pay attention to the topology that you sculpt and how much it gets stretched.
   If more resolution is needed you can always subdivide another time,
   but there will be worse performance and slower level switching once more than 5 subdivisions are used.
   Alternatively use the Slide Relax
   brush to slide topology to where it is needed.

Additional brushes like the Multires Eraser </sculpt_paint/sculpting/brushes/multires_displacement_eraser> and
Multires Smear </sculpt_paint/sculpting/brushes/multires_displacement_smear> are recommended for adjustments.

Here are general shortcuts to use the feature.

- Step up one multires level Alt-2
- Step down one multires level Alt-1
- Set multires level / Create multires modifier Ctrl-0 to Ctrl-5

**Seealso:**

   More information at Multiresolution Modifier </modeling/modifiers/generate/multiresolution>.
