---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSDFGridBoolean:

*********************
SDF Grid Boolean Node
*********************

The *SDF Grid Boolean* node performs boolean operations between two or more
**Signed Distance Field (SDF)** grids.
This allows combining, subtracting, or intersecting volumetric shapes directly
in grid space, similar to mesh boolean operations but with smooth and continuous results.

The node computes the resulting signed distance field by applying mathematical
operations to the input grids, preserving the SDF property where each voxel
stores the shortest distance to the nearest surface.
This makes it useful for blending or sculpting complex volumes procedurally.

Inputs
======

Grid Intersect
   The input grid used for intersection operations.
   Only the regions common to all intersecting grids will remain active.

Grid 1 Union Difference
   The first input grid for boolean combination.
   Acts as the base grid to which the second grid is added or subtracted.

Grid 2 Union Difference
   The second input grid used to modify *Grid 1* according to the selected operation.

Properties
==========

Operation
   The boolean operation to perform between the input SDF grids:

   :Intersect:
      Keeps only the overlapping region of the two grids where both contain
      interior (negative) values.
   :Union:
      Combines both grids by taking the minimum distance at each voxel.
      The resulting grid contains the merged shape of both inputs.
   :Difference:
      Subtracts the second grid from the first by inverting its sign and then
      taking the maximum distance.
      Useful for cutting holes or carving one SDF out of another.

Outputs
=======

Grid
   The resulting SDF grid after the boolean operation is applied.
   The output maintains valid signed distance values and can be converted
   to a mesh using the Grid to Mesh
   node or used in further volumetric operations.
