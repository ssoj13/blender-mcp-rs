---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. --- copy below this line ---

**********************
Combine Spherical Node
**********************

The *Combine Spherical* node converts spherical coordinates (R, Φ, Θ)
into a Cartesian vector (X, Y, Z). It is the inverse of the
Separate Spherical </modeling/geometry_nodes/utilities/vector/separate_spherical> node.

Spherical coordinates represent a point in 3D space by its distance from the origin,
horizontal rotation, and vertical inclination, which can be useful for constructing
radial or rotational patterns and generating coordinates on a sphere.

Inputs
======

R
   The radial distance from the origin.

Phi
   The azimuthal angle (horizontal rotation) around the Z axis.

Theta
   The polar angle (vertical inclination) from the ground plane.

Outputs
=======

Vector
   The resulting Cartesian vector corresponding to the input spherical coordinates.
