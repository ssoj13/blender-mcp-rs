---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeBump:

*********
Bump Node
*********

The *Bump* node generates a perturbed normal from a height texture, for bump mapping.
The height value will be sampled at the shading point and two nearby points
on the surface to determine the local direction of the normal.

Inputs
======

Strength
   Strength of the bump mapping effect, interpolating between no bump mapping and full bump mapping.
Distance
   Multiplier for the height value to control the overall distance for bump mapping.
Filter Width
   Filter width in pixels, used to compute the bump mapping direction. For most textures
   the default value of 0.1 enables subpixel filtering for stable results. For stepwise
   textures a larger filter width can be used to get a bevel like effect on edges.

   This input only supports a fixed value, it can not vary across the surface.
Height
   Scalar value giving the height offset from the surface at the shading point; this is where you plug in textures.
Normal
   Standard normal input.

Properties
==========

Invert
   Invert the bump mapping, to displace into the surface instead of out.

Outputs
=======

Normal
   Standard normal output.

**Tip:**

   If the *Height* input is not connected, the node becomes a no-op
   that outputs its *Normal* input as is, or defaults to the geometry normal if not connected. Routing a node
   group input via a no-op Bump node before doing math effectively makes it default to normal.

Examples
========

The above node setup will only bump the diffuse part of the shader,
simulating a bumpy diffuse surface coated with a smooth glossy "glaze" layer.
