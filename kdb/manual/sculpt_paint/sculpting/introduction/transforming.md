---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

************
Transforming
************

Transform tools to move, rotate and scale </sculpt_paint/sculpting/tools/transforms>
are also available in Sculpt Mode,
but with an important difference to other modes. Sculpt Mode uses its own pivot point,
which can be manually positioned Shift-RMB
or automatically positioned with Mask Expand <bpy.ops.sculpt.mask_expand>.
This ensures that the pivot point can be more freely placed and always moves with the transformed geometry.

Optionally instead of keeping the transform tools active, you can enable the
viewport gizmos </editors/3dview/display/gizmo> to have access to the gizmo at all times.

**Note:**

   The gizmo can in some cases block areas from being sculpted on.
   In that case move the pivot point somewhere else to be able to click on the desired surface.

Apart from the transform tools there are also special brushes to move,
rotate and scale the topology like Pose </sculpt_paint/sculpting/brushes/pose>,
Boundary </sculpt_paint/sculpting/brushes/boundary>
and Elastic Deform </sculpt_paint/sculpting/brushes/elastic_deform>.
