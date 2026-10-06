---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeBsdfPrincipled:

***************
Principled BSDF
***************

The *Principled* BSDF (Bidirectional Scattering Distribution Function)
that combines multiple layers into a single easy to use node.
It can model a wide variety of materials.

It is based on the OpenPBR Surface shading model, and provides parameters
compatible with similar PBR shaders found in other software,
such as the Disney and Standard Surface models.
Image textures painted or baked from software like Substance Painter
may be directly linked to the corresponding input in this shader.

Layers
======

The base layer is a mix between metal, diffuse, subsurface, and transmission components.
Most materials will use one of these components, though it is possible to smoothly mix
between them.

The metal component is opaque and only reflect lights. Diffuse is fully opaque, while
subsurface also involves light scattering just below the surface. Both diffuse and
subsurface sit below a specular layer. The transmission component includes both
specular reflection and refraction.

An optional thin film layer sits on top of metal, transmission and specular component,
to create iridescent effect.

On top of all base layers there is an optional glossy coat. And finally the sheen layer
sits on top of all other layers, to add fuzz or dust.

Light emission can also be added. Light emits from below the coat and sheen layers,
to model for example emissive displays with a coat or dust.

Thin Wall Mode
----------------
When the Thin Wall <bpy.types.ShaderNodePrincipled.thinWall> Boolean is enabled,
the surface is switched to thin-walled mode.
In this mode the surface is a slab with its layers mirrored around the base,
and the thickness is assumed to be zero.

**Note:**
   - The Subsurface Radius <bpy.types.ShaderNodePrincipled.Subsurface.Radius>,
     Subsurface Scale
     and Subsurface Method
     inputs are irrelevant in thin-walled mode.
   - When using thin-walled mode in EEVEE, the Thickness
     should be set to 0, and the Thickness Mode <bpy.types.Material.thickness_mode> should be set to Slab.

Inputs
======

Base Color
   Overall color of the material used for diffuse, subsurface, metal and transmission.

Roughness
   Specifies microfacet roughness of the surface for specular reflection and transmission.
   A value of 0.0 gives a perfectly sharp reflection, while 1.0 gives a diffuse reflection.

Metallic
   Blends between a dielectric and metallic material model.
   At 0.0 the material consists of a diffuse or transmissive base layer, with a specular reflection layer on top.
   A value of 1.0 gives a fully specular reflection tinted with the base color,
   without diffuse reflection or transmission.

IOR
   Index of refraction (IOR) for specular reflection and transmission.
   For most materials, the IOR is between 1.0 (vacuum and air) and 4.0 (germanium).
   The default value of 1.5 is a good approximation for glass.

Alpha
   Controls the transparency of the surface, with 1.0 fully opaque.
   Usually linked to the Alpha output of an Image Texture node.

.. _bpy.types.ShaderNodePrincipled.thinWall:

Thin Wall
   Switch the surface to thin-walled mode. Useful for modeling thin structures,
   such as papers, leaves, and window sheets.

   .. list-table::
      :widths: auto
      :align: center

      * - .. figure:: /images/render_shader-nodes_shader_principled_thin_subsurface.webp
             :width: 768px

             Subsurface Weight = 1, Anisotropy from -0.7 to 0.7

        - .. figure:: /images/render_shader-nodes_shader_principled_thin_glass.webp
             :width: 512px

             Transmission = 1, Thin Wall off and on

Normal
   Controls the normals of the base layers.

Diffuse
-------

Roughness Cycles Only
   Surface roughness; 0.0 gives standard Lambertian reflection, higher values activate the Oren-Nayar BSDF.

Subsurface
----------

Subsurface scattering is used to render materials such as skin, milk and wax.
Light scatters below the surface to create a soft appearance.

.. _bpy.types.ShaderNodePrincipled.Subsurface.Method:

Method
   Rendering method to simulate Subsurface scattering.

   :Christensen-Burley:
      An approximation to physically-based volume scattering.
      This method is less accurate than *Random Walk* however,
      in some situations this method will resolve noise faster.
   :Random Walk: Cycles Only
      Provides accurate results for thin and curved objects.
      Random Walk uses true volumetric scattering inside the mesh,
      which means that it works best for closed meshes.
      Overlapping faces and holes in the mesh can cause problems.
   :Random Walk (Skin): Cycles Only
      Random walk method optimized for skin rendering. The radius
      is automatically adjusted based on the color texture, and
      the subsurface entry direction uses a mix of diffuse and
      specular transmission with custom IOR. This tends to retain
      greater surface detail and color and matches measured skin
      more closely.
   :Random Walk (Legacy): Cycles Only
      The same as Random Walk, but uses a different mapping from albedo to
      scattering parameters. Will be deleted in the future.

Weight
   Blend between diffuse surface and subsurface scattering.
   Typically should be zero or one (either fully diffuse or subsurface).

.. _bpy.types.ShaderNodePrincipled.Subsurface.Radius:

Radius
   Average distance that light scatters below the surface.
   Higher radius gives a softer appearance, as light bleeds into shadows and through the object.
   The scattering distance is specified separately for the RGB channels,
   to render materials such as skin where red light scatters deeper.
   The X, Y and Z values are mapped to the R, G and B values, respectively.

.. _bpy.types.ShaderNodePrincipled.Subsurface.Scale:

Scale
   Scale applied to the radius.

IOR Cycles Only
   Index of refraction (IOR) used for rays that enter the subsurface component. This may be set to
   a different value than the global IOR to simulate different layers of skin.

Anisotropy Cycles Only
   Directionality of volume scattering within the subsurface medium. Zero scatters uniformly
   in all directions, positive values scatter more in the forward direction, and negative
   values scatter more backwards.
   For example, skin has been measured to have an anisotropy of 0.8.

Specular
--------

Controls for both the metallic component and specular layer on top of diffuse and subsurface.

Distribution
   Microfacet distribution to use.

   :GGX:
      A method that is faster than *Multiple-scattering GGX* but is less physically accurate.
   :Multiscatter GGX:
      Takes multiple scattering events between microfacets into account.
      This gives more energy conserving results,
      which would otherwise be visible as excessive darkening.

IOR Level
   Adjustment to the IOR to increase or decrease intensity of the specular layer.
   0.5 means no adjustment, 0 removes all reflections, 1 doubles them at normal incidence.

   This input is designed for conveniently texturing the IOR and amount of specular
   reflection.

Tint
   Color tint for specular and metallic reflection.

   For non-metallic tints provides artistic control over the color specular reflections at normal incidence,
   while grazing reflections remain white. In reality non-metallic specular reflection is fully white.

   For metallic materials tints the edges to simulate complex IOR as found in materials such as gold or copper.

Anisotropy Cycles Only
   Amount of anisotropy for specular reflection. Higher values give elongated highlights along the tangent direction;
   negative values give highlights shaped perpendicular to the tangent direction.

Anisotropic Rotation Cycles Only
   Rotates the direction of anisotropy, with 1.0 going full circle.

   Compared to the *Glossy BSDF* node, the direction of highlight elongation
   is rotated by 90°. Add 0.25 to the value to correct.

Tangent
   Controls the tangent direction for anisotropy.

Transmission
------------

Transmission is used to render materials like glass and liquids, where the surface both
reflects light and transmits it into the interior of the object

Weight
   Mix between fully opaque surface at zero and fully transmissive at one.

Coat
----

Coat on top of the materials, to simulate for example a clearcoat, lacquer or car paint.

Weight
   Controls the intensity of the coat layer, both the reflection and the tinting.
   Typically should be zero or one for physically-based materials, but may be textured
   to vary the amount of coating across the surface.

Roughness
   Roughness of the coat layer.

IOR
   Index of refraction (IOR) of the coat layer.
   Affects its reflectivity as well as the falloff of coat tinting.

Tint
   Adds a colored tint to the coat layer by modeling absorption in the layer.
   Saturation increases at shallower angles, as the light travels farther
   through the medium, depending on the IOR.

Normal
   Controls the normals of the *Coat* layer, for example to add a smooth coating on a rough surface.

Sheen
-----

Sheen simulates very small fibers on the surface.
For cloth this adds a soft velvet like reflection near edges.
It can also be used to simulate dust on arbitrary materials.

Weight
   Controls the intensity of the sheen layer.

Roughness
   Roughness of the sheen reflection.

Tint
   The color of the sheen reflection.

Emission
--------

Light emission from the surface.

Color
   Color of light emission from the surface.

Strength
   Strength of the emitted light. A value of 1.0 ensures that the object
   in the image has the exact same color as the *Emission Color*, i.e. make it 'shadeless'.

Thin Film Cycles Only
---------------------------------

Thin Film simulates the effect of interference in a thin film sitting on top of the material.
This causes the specular reflection to be colored in a way which strongly depends on the view
angle as well as the film thickness and the index of refraction (IOR) of the film and
the material itself.

This effect is commonly seen on e.g. oil films, soap bubbles or glass coatings. While its
influence is more obvious in specular highlights, it also affects transmission.

Thickness
   The thickness of the film in nanometers. A value of 0 disables the simulation.
   The interference effect is strongest between roughly 100 and 1000 nanometers, since this is
   near the wavelengths of visible light.

IOR
   Index of refraction (IOR) of the thin film.
   The common range for this value is between 1.0 (vacuum and air) and roughly 2.0,
   though some materials can reach higher values.
   The default value of 1.33 is a good approximation for water.
   Note that when the value is set to 1.0 or to the main IOR of the material, the thin film
   effect disappears since the film optically blends into the air or the material.

Outputs
=======

BSDF
   Standard shader output.
