---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSetMaterial:

*****************
Set Material Node
*****************

The *Set Material* changes the material assignment in the specified selection,
by adjusting the `material_index` attribute. If the material is already used
on the geometry, the existing material index will be reused.

**Note:**

   This node adjusts mesh, point clouds, and volume data;
   other data types do not support materials.

Inputs
======

Geometry
   Standard geometry input containing a mesh.

Material
   The material to apply to the geometry.

Selection
   Whether to change the material of each face.
   True values mean the material will be changed, false values mean it will remain the same.

   Note, volumes and point clouds only support a single material,
   in these cases a field input will be ignored.

Outputs
=======

Geometry
   Standard geometry output.
