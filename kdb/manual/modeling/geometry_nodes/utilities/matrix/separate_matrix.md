---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.FunctionNodeSeparateMatrix:
.. --- copy below this line ---

********************
Separate Matrix Node
********************

The *Separate Matrix* node splits a 4x4 matrix into its individual values.

Inputs
======

Matrix
   The matrix to split into individual values.

Outputs
=======

The outputs of this node are split into panels for each column of the matrix.
Each panel, has four value outputs for the four rows of the matrix.
