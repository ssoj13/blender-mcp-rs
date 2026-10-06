---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.FunctionNodeProjectPoint:
.. --- copy below this line ---

******************
Project Point Node
******************

Applies a projection matrix to a point. Specifically, this node turns the given
Euclidean vector (X, Y, Z) into the homogeneous vector (X, Y, Z, 1),
multiplies the given projection matrix by it,
and turns the resulting homogeneous vector back into a Euclidean one by dividing
it by the absolute value of its W component. This last step is also known as
perspective division.

Inputs
======

Vector
   The position vector to project.
Transformation
   The projection matrix.

Outputs
=======

Vector
   The projected position vector.
