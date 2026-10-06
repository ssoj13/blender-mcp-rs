---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeBsdfHair:

*********
Hair BSDF
*********

Cycles Only

The *Hair* BSDF (Bidirectional Scattering Distribution Function)
node is used to add shading for Hair </physics/particles/hair/index>.

Inputs
======

Color
   Color of the hair.
Offset
   Controls the way the light is rotated (angular shift) for the reflection/transmission.

Roughness U/V
   Controls the roughness in the direction light is skewed, and perpendicular to it.

   .. list-table::

      * - .. figure:: /images/render_shader-nodes_shader_hair_reflect-roughness-uv.png

             Roughness when using the Reflection Component.

        - .. figure:: /images/render_shader-nodes_shader_hair_trans-roughness-uv.png

             Roughness when using the Transmission Component.

Tangent
   Input tangent.

Properties
==========

Component
   There are two components that can be used to control the look of the hair.
   Usually you are going to want each of these and use a Mix Node </render/shader_nodes/shader/mix>.

   :Reflection: The light that bounces off the surface of the hair.
   :Transmission: The light that passes through the hair and exits on the other side.

Outputs
=======

BSDF
   Standard shader output.
