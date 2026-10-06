---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.mesh.solidify:

**************
Solidify Faces
**************

Extrudes the selected faces, turning them from an infinitely thin surface into
a manifold mesh with volume.

The thickness can be changed in the bpy.ops.screen.redo_last panel:

Thickness
   Amount to offset the newly created surface.
   Positive values move inward relative to the face normals.
   Negative values move outward.

.. list-table::

   * - .. figure:: /images/modeling_meshes_editing_face_solidify-faces_before.png
          :width: 200px

          Mesh before solidifying.

     - .. figure:: /images/modeling_meshes_editing_face_solidify-faces_after.png
          :width: 200px

          Solidify with a positive thickness.

     - .. figure:: /images/modeling_meshes_editing_face_solidify-faces_after2.png
          :width: 200px

          Solidify with a negative thickness.

**Seealso:**

   The /modeling/modifiers/generate/solidify does this non-destructively
   and has many more options.
