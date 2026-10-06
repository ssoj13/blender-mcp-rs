---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeOutputMaterial:

********************
Material Output Node
********************

The *Material Output* node is used to output surface material information to a surface object.

Inputs
======

Surface
   Shading for the surface </render/materials/components/surface> of the object.
Volume
   Shading for the volume </render/materials/components/volume> inside the object.
Displacement
   Used to create bump mapping or actual subdivided displacement </render/materials/components/displacement>.
Thickness EEVEE
   Used to approximate the inner geometry structure of the object without heavy computation.
   This is currently used for Subsurface Scattering </render/shader_nodes/shader/sss>,
   Translucent BSDF </render/shader_nodes/shader/translucent>,
   Refraction BSDF </render/shader_nodes/shader/refraction>, and the nodes containing these effects.

   If no value is plugged into the output node,
   a default thickness based on the smallest dimension of the object is used.
   If a value is connected it will be used as object space thickness (i.e. scaled by object transform).
   A value of zero will disable the thickness approximation and treat the object as having only one interface.

   This output is only used by the EEVEE render engine.

**Note:**

      - The thickness is used to skip the inner part of the object.
      - Refraction will not refract objects inside the thickness distance.
      - Shadow casting object will not cast shadow within the thickness distance.

**Tip:**

      - For large or compound meshes (e.g. vegetation),
        the thickness should be set to the thickness of individual parts (e.g. leaves, grass blades).
      - Thickness can be baked to textures or custom attributes for more accurate result.

**Seealso:**

      Thickness Mode <bpy.types.Material.thickness_mode> -- controls how the thickness value is used.

Properties
==========

Target
   Render engine the input shaders are used for.
   By default shaders are shared between Cycles and EEVEE,
   with multiple output nodes specialized shader setups can be created for each.

Outputs
=======

This node has no outputs.
