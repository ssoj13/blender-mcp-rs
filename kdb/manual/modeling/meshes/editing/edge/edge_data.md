---
area: manual-modeling
source: blender-manual
version: 5.2
---

*********
Edge Data
*********

The operators below change edge properties which in turn influence certain tools or modifiers.
To quickly access an operator that doesn't have its own shortcut, press Ctrl-E
to open the Edge menu as a popover at the mouse cursor.

.. _bpy.ops.transform.edge_bevelweight:

Edge Bevel Weight
=================

Interactively changes the Bevel Weight <modeling-edges-bevel-weight> of the selected edges.
After clicking the menu item, move the mouse to change the weight or type a (positive or negative)
number to add to the existing weight. (The value is shown at the top of the 3D Viewport.)
Then press LMB or Return to confirm, or RMB or Esc to cancel.

The Bevel Weight can also be seen and changed in the
Sidebar <modeling-mesh-transform-panel>.

.. _bpy.ops.transform.edge_crease:

Edge Crease
===========

Interactively changes the Crease <modeling-edges-bevel-weight> amount of the selected edges.
After clicking the menu item or pressing the shortcut, move the mouse to change the value
or type a (positive or negative) number to add to the existing value. (The value is shown
at the top of the 3D Viewport.) Then press LMB or Return to confirm,
or RMB or Esc to cancel.

The Crease value can also be seen and changed in the
Sidebar <modeling-mesh-transform-panel>.

.. _bpy.ops.mesh.mark_seam:

Mark/Clear Seam
===============

Mark or unmark the selected edges as being
UV Seams </modeling/meshes/uv/unwrapping/seams>.

.. _bpy.ops.mesh.mark_sharp:

Mark/Clear Sharp
================

Mark or unmark the selected edges as being Sharp. This property is used for the following:

- Making edges appear sharp, and faces appear flat, in meshes that are otherwise
  smoothly shaded <bpy.ops.object.shade_smooth>. Specifically, this is done by changing
  the normals <modeling-meshes-structure-normals> of vertices that are connected to
  at least two Sharp edges.
- Indicating which edges should be affected by the /modeling/modifiers/generate/edge_split.
- Indicating which edges should *not* be affected by the /modeling/modifiers/normals/smooth_by_angle
  and the /modeling/modifiers/normals/weighted_normal.
- Exporting smoothing groups to external file formats (e.g. FBX </files/import_export/fbx_legacy>,
  OBJ </files/import_export/obj>).

Internally, the Sharp state is stored in the `sharp_edge` attribute <geometry-nodes_builtin-attributes>.

**Seealso:**

   - /modeling/geometry_nodes/mesh/read/is_edge_smooth
   - /modeling/geometry_nodes/mesh/write/set_shade_smooth

.. _bpy.ops.mesh.set_sharpness_by_angle:

Set Sharpness by Angle
======================

Sets the sharp edge attribute <geometry-nodes_builtin-attributes>
based on the angle between neighboring faces.

Angle
   Maximum angle between face normals that will be considered as smooth.
Extend
   Add new sharp edges without clearing existing sharp edges.

.. _bpy.ops.mesh.mark_freestyle_edge:

Mark/Clear Freestyle Edge
=========================

Marks or unmarks the selected edges as requiring Freestyle lines.
See freestyle-edge-marks.
