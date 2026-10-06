---
area: manual-render
source: blender-manual
version: 5.2
---

*********************
Open Shading Language
*********************

`Open Shading Language (OSL) <https://github.com/AcademySoftwareFoundation/OpenShadingLanguage>`__
is a programmable shading system developed for advanced rendering engines.
It allows technical artists and developers to write custom shader code using a C-like scripting language.

In Blender, OSL can be used within Cycles to define custom surface, volume, and displacement shaders.
This gives users full control over shading behavior, enabling procedural effects, advanced lighting models,
and custom geometry-based material logic that may not be possible with built-in shader nodes alone.

Unlike node-based materials, OSL shaders are authored as text scripts using Blender's internal
Text Editor </editors/text_editor> or loaded from external `.osl` or `.oso` files.
These scripts are then compiled and used in the Shader Editor
through the Script Node </render/shader_nodes/utilities/script>.

**Tip:**

   OSL is especially useful for generating procedural textures, custom BRDFs, or implementing research prototypes.
   It also allows sharing shaders across compatible rendering applications that support the OSL standard.

.. _osl-usage:

Usage
=====

To use Open Shading Language (OSL) in Blender, follow these steps:

#. **Enable OSL Rendering**

   In the Render Properties enable
   Open Shading Language <bpy.types.CyclesRenderSettings.shading_system>.

#. **Add a Script Node**

   In the Shader Editor add Script Node </render/shader_nodes/utilities/script> then in the node's properties:

   - Set the *Mode* to *Internal* to use a Blender text data-block, or
   - Set it to *External* to load a shader file from disk (either `.osl` or compiled `.oso`).

   For the internal mode, create a new text data-block in the Text Editor,
   then write or paste your OSL code there.

   Blender will compile the OSL source file automatically.
   If the source is `.osl`, it will be compiled into `.oso` bytecode.
   Compilation errors will be shown in the system console.

#. **Use Shader Outputs**

   Once compiled, the node's outputs will reflect the `output` parameters defined in the OSL code.
   These outputs can be connected to any part of the material node tree.

Writing Shaders
===============

For more details on how to write shaders, see the
`OSL Documentation <https://open-shading-language.readthedocs.io/en/latest/>`__.

Here is a simple example:

.. code-block:: cpp

   shader simple_material(
       color Diffuse_Color = color(0.6, 0.8, 0.6),
       float Noise_Factor = 0.5,
       output closure color BSDF = diffuse(N))
   {
       color material_color = Diffuse_Color * mix(1.0, noise(P * 10.0), Noise_Factor);
       BSDF = material_color * diffuse(N);
   }

Closures
========

OSL is different from, for example, RSL or GLSL, in that it does not have a light loop.
There is no access to lights in the scene,
and the material must be built from closures that are implemented in the renderer itself.
This is more limited, but also makes it possible for the renderer to do optimizations and
ensure all shaders can be importance sampled.

The available closures in Cycles correspond to the shader nodes and their sockets;
for more details on what they do and the meaning of the parameters,
see the shader nodes manual </render/shader_nodes/shader/index>.

**Seealso:**

   Documentation on OSL's `built-in closures
   <https://open-shading-language.readthedocs.io/en/latest/stdlib.html#material-closures>`__.

BSDF
----

- `diffuse(N)`
- `oren_nayar(N, roughness)`
- `diffuse_ramp(N, colors[8])`
- `phong_ramp(N, exponent, colors[8])`
- `diffuse_toon(N, size, smooth)`
- `glossy_toon(N, size, smooth)`
- `translucent(N)`
- `reflection(N)`
- `refraction(N, ior)`
- `transparent()`
- `microfacet_ggx(N, roughness)`
- `microfacet_ggx_aniso(N, T, ax, ay)`
- `microfacet_ggx_refraction(N, roughness, ior)`
- `microfacet_beckmann(N, roughness)`
- `microfacet_beckmann_aniso(N, T, ax, ay)`
- `microfacet_beckmann_refraction(N, roughness, ior)`
- `ashikhmin_shirley(N, T, ax, ay)`
- `ashikhmin_velvet(N, roughness)`

Hair
----

- `hair_reflection(N, roughnessu, roughnessv, T, offset)`
- `hair_transmission(N, roughnessu, roughnessv, T, offset)`
- `principled_hair(N, absorption, roughness, radial_roughness, coat, offset, IOR)`

BSSRDF
------

Used to simulate subsurface scattering.

Volume
------

- `henyey_greenstein(g)`
- `absorption()`

Other
-----

- `emission()`
- `ambient_occlusion()`
- `holdout()`
- `background()`

Attributes
==========

Geometry attributes can be read through the `getattribute()` function.
This includes UV maps, color attributes and any attributes output from geometry nodes.

The following built-in attributes are available through `getattribute()` as well.

`geom:generated`
   Automatically generated texture coordinates, from non-deformed mesh.
`geom:uv`
   Default render UV map.
`geom:tangent`
   Default tangent vector along surface, in object space.
`geom:undisplaced`
   Position before displacement, in object space.
`geom:dupli_generated`
   For instances, generated coordinate from instancer object.
`geom:dupli_uv`
   For instances, UV coordinate from instancer object.
`geom:trianglevertices`
   Three vertex coordinates of the triangle.
`geom:numpolyvertices`
   Number of vertices in the polygon (always returns three currently).
`geom:polyvertices`
   Vertex coordinates array of the polygon (always three vertices currently).
`geom:name`
   Name of the object.
`geom:is_smooth`
   Is mesh face smooth or flat shaded.
`geom:is_curve`
   Is object a curve or not.
`geom:curve_intercept`
   0..1 coordinate for point along the curve, from root to tip.
`geom:curve_thickness`
   Thickness of the curve in object space.
`geom:curve_length`
   Length of the curve in object space.
`geom:curve_tangent_normal`
   Tangent Normal of the strand.
`geom:is_point`
   Is point in a point cloud or not.
`geom:point_radius`
   Radius of point in point cloud.
`geom:point_position`
   Center position of point in point cloud.
`geom:point_random`
   Random number, different for every point in point cloud.
`path:ray_length`
   Ray distance since last hit.
`object:random`
   Random number, different for every object instance.
`object:index`
   Object unique instance index.
`object:location`
   Object location.
`material:index`
   Material unique index number.
`particle:index`
   Particle unique instance number.
`particle:age`
   Particle age in frames.
`particle:lifetime`
   Total lifespan of particle in frames.
`particle:location`
   Location of the particle.
`particle:size`
   Size of the particle.
`particle:velocity`
   Velocity of the particle.
`particle:angular_velocity`
   Angular velocity of the particle.

.. _render-shader-nodes-osl-trace:

Trace
=====

CPU Only

We support the `trace(point pos, vector dir, ...)` function,
to trace rays from the OSL shader. The "shade" parameter is not supported currently,
but attributes can be retrieved from the object that was hit using the
`getmessage("trace", ..)` function. See the OSL specification for details on how to use this.

This function cannot be used instead of lighting;
the main purpose is to allow shaders to "probe" nearby geometry,
for example to apply a projected texture that can be blocked by geometry,
apply more "wear" to exposed geometry, or make other ambient occlusion-like effects.

Metadata
========

Metadata on parameters controls how they are displayed in the user interface.
The following metadata entries are supported:

`[[ string label = "My Label" ]]`
   Custom display name of the parameter in the user interface.

`[[ string widget = "null" ]]`
   Hides the parameter from the user interface.

`[[ string widget = "boolean" ]]` or `[[ string widget = "checkBox" ]]`
   Displays an integer parameter as a boolean checkbox.

Limitations
===========

**Important:**

   OSL is not supported with GPU rendering unless using the OptiX backend.

Some OSL features are not available when using the OptiX backend. Examples include:

- Texture lookups require OSL to be able to determine a constant image file path for each
   texture call.
- Some noise functions are not available. Examples include *Cell*, *Simplex*, and *Gabor*.
- The trace <render-shader-nodes-osl-trace> function is not functional.
  As a result of this, the Ambient Occlusion <bpy.types.ShaderNodeAmbientOcclusion>
  and Bevel <bpy.types.ShaderNodeBevel> nodes do not work.
