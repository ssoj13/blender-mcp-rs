---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeVolumeScatter:

**************
Volume Scatter
**************

The *Volume Scatter* node allows light to be scattered as it passes through the volume.
Typical usage would be to add fog to a scene. It can also be used with
the Volume Absorption </render/shader_nodes/shader/volume_absorption> node to create smoke.

Inputs
======

Color
   Scattering coefficients per color channel.
Density
   The density of the scatter effect.
Anisotropy Henyey-Greenstein Draine
   Controls the relative amount of backward and forward scattering.
IOR Fournier-Forand
   Refractive index of the scattering particles relative to water. Common ocean waters range between
   1.0 and 1.2, while turbid waters with higher density of particles have higher IORs.
Backscatter Fournier-Forand
   Fraction of light that is scattered backwards. Most oceanic particles have backscatter
   values between 0.001 (e.g., very large phytoplankton) and 0.1 (e.g., very small mineral particles),
   pure water has a backscatter of 0.5. Values taken from
   `Ocean Optics Web Book <https://www.oceanopticsbook.info/view/scattering/the-fournier-forand-phase-function>`__.
Alpha Draine
   Blending factor between Henyey-Greenstein (\alpha = 0)
   and Cornette & Shanks (\alpha = 1) phase functions.
Diameter Mie
   Diameter of the scattering particles in µm.

Properties
==========

.. _bpy.types.ShaderNodeVolumeScatter.phase:

Phase
  Volume scattering phase function.

  :Henyey-Greenstein:
     Simple and widely used phase function, useful for approximating scattering in biological tissues.
  :Fournier-Forand:
     Cycles Only
     Suitable for modeling the scattering of light in underwater environments.
  :Draine:
     Cycles Only
     Suitable for modeling the scattering of interstellar dust.
  :Rayleigh:
     Cycles Only
     Describes the scattering by particles with a size smaller than the wavelength of light,
     such as the scattering of sunlight in earth's atmosphere.
  :Mie:
     Cycles Only
     Describes the scattering by particles with a size larger than the wavelength of light, such as cloud and fog.

**Tip:**
      These phase functions can be combined using a /render/shader_nodes/shader/mix.

Outputs
=======

Volume
   The Volume Shader output must be plugged into the *Volume Input*
   of the Material
   or World </render/shader_nodes/output/world> Output node.

Examples
========
