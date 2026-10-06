---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeGridCurl:

**************
Grid Curl Node
**************

The *Grid Curl* node computes the curl of a vector field stored in a voxel grid.
Curl represents the local amount of rotational motion or circulation within
a vector field—essentially, how much and in what direction the field "spins"
around each point.

In mathematical terms, the curl of a 3D vector field F = (Fx, Fy, Fz) is a
vector that describes the infinitesimal rotation of the field, defined as:

The resulting vector points along the axis of rotation, and its magnitude
indicates the strength of that rotation.

This operation is useful for generating turbulence or analyzing the rotational
behavior of flow fields in simulations and procedural effects.

Inputs
======

Grid
   The input vector grid from which to calculate the curl.
   The grid must store 3D vector values (e.g. velocity or direction fields).

Outputs
=======

Curl
   A vector grid representing the curl (local rotation) of the input field.
   The direction of the output vectors indicates the axis of rotation,
   and their magnitude represents the rotational strength at each voxel.
