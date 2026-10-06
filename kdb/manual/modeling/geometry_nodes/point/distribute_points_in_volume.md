---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeDistributePointsInVolume:

***************************
Distribute Points in Volume
***************************

The *Distribute Points in Volume* node creates points inside of volume grids.
The node has two basic modes of operation: distributing points randomly, or in
a regular grid. Both methods operate on all of the float grids in the volume.

Inputs
======

Volume
   Standard volume geometry input.

Mode
   :Random:
      Distribute points randomly inside of the volume. The local point count is implicitly
      defined as a product of the global from the *Density* input and the local voxel value.
      This method creates a distribution that is not stable as the input volume deforms.
   :Grid:
      Distribute the points in a grid pattern inside of the volume. At each grid point, the voxel
      value is used to determine whether to add a point.

Density Random
   Number of points to sample per unit volume.

Seed Random
   Seed used by the random number generator to generate random points

Spacing Grid
   Spacing between grid points.

Threshold Grid
   Minimum value of a volume cell to contain a grid point.

Outputs
=======

Points
   Standard point cloud geometry output.
