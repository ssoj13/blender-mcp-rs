---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.MeshToVolumeModifier:

***********************
Mesh to Volume Modifier
***********************

The *Mesh to Volume* modifier uses a mesh to create a new volume grid.
All previously existing volume grids on the volume object are discarded.
So this modifier is usually added to an empty volume object.
The new volume grid is called "density".

**Tip:**

   To copy and move the generated volume separately from the mesh object,
   use a collection instance </scene_layout/object/properties/instancing/collection>.

Options
=======

Object
   The mesh object that determines where the volume data will be generated.

Density
   Makes the generated volume appear denser or less dense when rendering.

Interior Band Width
   The maximum distance of the included voxels to the surface on the inside of the mesh.

Resolution Mode
   Mode for how the voxel size is specified.

   :Voxel Amount:
      This allows setting an approximate number of voxels that will be used to represent mesh along its diagonal.
      When the dimensions of the mesh changes, the voxel size will change as well.
      For final rendering of animations, it's better to specify the voxel size explicitly to avoid artifacts.
   :Voxel Size:
      This allows setting the exact voxel size that will be used.
      This is ideal for rendering when the voxel size should not change between frames.

Example
=======
