---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. --- copy below this line ---

************************
Combine Cylindrical Node
************************

The *Combine Cylindrical* node converts cylindrical coordinates (R, Φ, Z)
into a Cartesian vector (X, Y, Z). It is the inverse of the
Separate Cylindrical </modeling/geometry_nodes/utilities/vector/separate_cylindrical> node.

This node is useful for constructing positions or directions based on radial distance,
rotation angle, and height, such as creating spiral shapes or cylindrical displacements.

Inputs
======

R
   The radial distance from the origin in the X-Y plane.

Phi
   The angle around the Z axis.

Z
   The vertical position or height component.

Outputs
=======

Vector
   The resulting Cartesian vector corresponding to the input cylindrical coordinates.
