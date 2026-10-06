---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeGamma:
.. Editor's Note: The rest of the page gets copied into:
.. - </compositing/types/color/adjust/gamma
.. --- copy below this line ---

**********
Gamma Node
**********

Use this node to apply a gamma correction. The node is typically used to convert from gamma encoded to linear
color space, or in the reverse direction with 1/gamma.

Inputs
======

Color
   Standard color input.
Gamma
   An exponential brightness factor, applied as output\_value = input\_value ^ {\gamma}

Outputs
=======

Color
   Standard color output.

Examples
========
