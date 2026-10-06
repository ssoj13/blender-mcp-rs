---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeMeshToVolume:

*******************
Mesh to Volume Node
*******************

The *Mesh to Volume* node creates a fog volumes based on the shape of a mesh.
The volume is created with a grid of the name `"density"`.

Inputs
======

Mesh
   Standard Mesh input.

Density
   Value of voxels inside the generated fog volume.

Resolution Mode
   How the voxel size is specified.

   :Amount:
      Specify the approximate number of voxels along the diagonal.
   :Size:
      Specify the voxel side length. It is recommended to be careful when tweaking this value,
      because small changes can have a large effect on the processing time.

Voxel Amount Amount
   Specify the approximate number of voxels along the diagonal.

Voxel Size Size
   Specify the voxel side length.

Interior Band Width
   The maximum distance of the included voxels to the surface on the inside of the mesh.

Outputs
=======

Volume
   The generated volume grid.
