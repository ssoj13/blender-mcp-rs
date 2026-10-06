---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeToolMousePosition:

*******************
Mouse Position Node
*******************

The *Mouse Position* node returns information about the mouse cursor
such as its position and the region's </interface/window_system/regions> dimensions.

**Tip:**

   When using this node, enable Wait for Click <bpy.types.GeometryNodeTree.use_wait_for_click>.
   to wait for a mouse click input (LMB) before running the operator from a menu.

**Note:**

   This node can only be used in the Tool context <tool_context>.

Inputs
======

This node has no inputs.

Outputs
=======

Mouse X
   The region-space mouse X location, in pixels, increasing from 0 at the left.
Mouse Y
   The region-space mouse Y location, in pixels, increasing from 0 at the bottom.
Region Width
   The total X size of the region </interface/window_system/regions> in pixels.
Region Height
   The total Y size of the region </interface/window_system/regions> in pixels.
