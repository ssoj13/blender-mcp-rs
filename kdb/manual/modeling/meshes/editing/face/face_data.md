---
area: manual-modeling
source: blender-manual
version: 5.2
---

*********
Face Data
*********

.. _bpy.ops.mesh.colors_rotate:

Rotate Colors
=============

Rotates the colors for the active
Color Attribute <modeling-meshes-properties-object_data-color-attributes>
inside each selected face either clockwise or counterclockwise.

.. _bpy.ops.mesh.colors_reverse:

Reverse Colors
==============

Mirrors the colors for the active Color Attribute inside each selected face.

Rotate UVs
==========

See bpy.ops.mesh.uvs_rotate.

Reverse UVs
===========

See bpy.ops.mesh.uvs_reverse.

.. _bpy.ops.mesh.flip_quad_tessellation:

Flip Quad Tessellation
======================

Internally, every quad is tessellated <Tessellation> into two triangles.
This operator swaps the tessellation direction of the selected quads.

.. list-table::

   * - .. figure:: /images/modeling_meshes_editing_face_flip-tessellation_before.png
     - .. figure:: /images/modeling_meshes_editing_face_flip-tessellation_after.png

**Seealso:**

   /modeling/meshes/editing/edge/rotate_edge

.. _bpy.ops.mesh.mark_freestyle_face:

Mark/Clear Freestyle Face
=========================

Marks or unmarks the selected faces as requiring special Freestyle behavior.
See freestyle-face-marks.
