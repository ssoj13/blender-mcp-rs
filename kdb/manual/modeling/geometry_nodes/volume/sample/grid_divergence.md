---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeGridDivergence:

********************
Grid Divergence Node
********************

The *Grid Divergence* node computes the **divergence** of a vector field stored
in a voxel grid. Divergence measures how much a field is "spreading out" or
"converging" at each point, representing the net flow entering or leaving a
voxel.

A positive divergence value indicates that the field is expanding outward
from that voxel (acting as a source), while a negative value indicates that the
field is converging inward (acting as a sink). A divergence near zero means
the field is locally balanced, with equal flow in and out.

This operator is commonly used in fluid and smoke simulation workflows, where
it helps enforce incompressibility or visualize the flow behavior of vector
fields such as velocity grids.

Mathematically, for a 3D vector field \mathbf{F} = (F_x, F_y, F_z),
the divergence is defined as:

Inputs
======

Grid
   The input vector grid whose divergence will be calculated.
   The grid must store 3D vector values, such as a velocity or directional
   field.

Outputs
=======

Divergence
   A float grid representing the divergence of the input field.

   Positive values correspond to regions where vectors spread apart (sources),
   while negative values represent regions where vectors converge (sinks).
