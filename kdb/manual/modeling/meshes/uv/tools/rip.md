---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.uv.rip:

**********
Rip Region
**********

The *Rip Region* tool interactively separates selected UV elements
(vertices, edges, or faces) from connected components, creating a "rip" in the UV map.
After the separation, the selection can be moved in the direction of the mouse
pointer, allowing the detached elements to be repositioned interactively.

This is useful for isolating UV islands or unwrapping overlapping elements
without affecting surrounding geometry.

.. list-table::

   * - .. figure:: /images/modeling_meshes_uv_tools_rip_before.png

          Before.

     - .. figure:: /images/modeling_meshes_uv_tools_rip_after.png

          After.

**Note:**

   The *Rip Region* tool is not compatible with
   Sync Selection <bpy.types.ToolSettings.use_uv_select_sync>.
   To use this tool, make sure Sync Selection is disabled in the UV Editor.

**Seealso:**

   - Rip Move UV Operator <bpy.ops.uv.rip_move> -- operator version of the rip operator.
   - Mesh editing Rip <bpy.ops.mesh.rip_move> -- Similar functionality for mesh editing in the 3D Viewport.
