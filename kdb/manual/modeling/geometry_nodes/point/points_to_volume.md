---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodePointsToVolume:

*********************
Points to Volume Node
*********************

The *Points to Volume* node generates a fog volume sphere around every point in the input geometry.
The new volume grid is named "density".

It usually makes sense to combine this node with the
/modeling/geometry_nodes/volume/operations/volume_to_mesh.

**Warning:**

   This node expects that point positions are not extremely large.
   For position values of many billions, the behavior isn't guaranteed, and it may be unstable.

Inputs
======

Points
   Standard geometry input.

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

Radius
   Specify the radius of the sphere generated at each point.

Outputs
=======

Volume
   Standard geometry output.
