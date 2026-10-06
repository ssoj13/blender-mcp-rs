---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

***************
Cloth Sculpting
***************

Instead of sculpting cloth manually or creating complex physics simulation setups,
there are various tools directly in sculpt mode that offer a simplified
Cloth Physics Simulation </physics/cloth/index>.

This has various advantages but is especially useful for base mesh creation and larger clothing folds and draping.
Detailing is possible, but the slower performance on high resolution meshes and simplified cloth physics
might not lead to desirable results.

The resolution of the topology is mainly responsible for the size of the folds
and detail level of the simulation. So an optimal and evenly distributed topology is important.

Many sculpting features are supported, so for example Masked <sculpt-mask-menu> vertices
are pinned </physics/cloth/settings/shape> in the simulation.
Another example is with auto-masked </sculpt_paint/sculpting/controls> face set boundaries.
The sculpt mode gravity <bpy.types.Sculpt.gravity> factor is also applied on the cloth physics.

The main brushes and tools for this feature are the Cloth Brush </sculpt_paint/sculpting/brushes/cloth>
and Cloth Filter </sculpt_paint/sculpting/tools/cloth_filter>,
but other transform brushes like Pose </sculpt_paint/sculpting/brushes/pose> and
Boundary </sculpt_paint/sculpting/brushes/boundary> also support cloth sculpting in the brush settings.

A demo file for trying out the various brushes and tools is available
`here <https://www.blender.org/download/demo-files/#sculpting>`__.
