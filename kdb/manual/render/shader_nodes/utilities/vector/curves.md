---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeVectorCurve:
.. Editor's Note: This page gets copied into:
.. - </compositing/types/utilities/vector/vector_curves>
.. - </modeling/geometry_nodes/utilities/vector/vector_curves>
.. --- copy below this line ---

******************
Vector Curves Node
******************

The Vector Curves node maps an input vector components to a curve.

Use this curve node to slow things down or speed them up from the original scene.

Inputs
======

In the shader context the node also has an additional Factor property.

Factor
   Controls the amount of influence the node exerts on the output vector.
Vector
   Standard vector input.

Properties
==========

Channel
   X, Y, Z
Curve
   For the curve controls see: Curve widget <ui-curve-widget>.

Outputs
=======

Vector
   Standard vector output.
