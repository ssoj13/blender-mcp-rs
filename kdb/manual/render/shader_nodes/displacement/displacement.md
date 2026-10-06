---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeDisplacement:

*****************
Displacement Node
*****************

The *Displacement* node is used to displace the surface along the surface normal,
to add more detail to the geometry. Both procedural textures and baked displacement maps
can be used.

By default, Blender only uses Bump Mapping to render displacement.
However with true displacement, the rendered geometry will be physically displaced. To use true displacement
the Displacement method <bpy.types.Material.displacement_method> must be set accordingly.

**Tip:**

   For best results when using true displacement,
   the mesh must be subdivided finely to bring out the detail in the displacement texture.

**Seealso:**

   Material Displacement
   for more details on displacement workflows.

Inputs
======

Height
   Distance to displace the surface along the normal.
   This is where a texture node can be connected.
Midlevel
   Neutral displacement value that causes no displacement.
   With the default 0.5, any lower values will cause the surfaces to be pushed inwards,
   and any higher values will push them outwards.
Scale
   Increase or decrease the amount of displacement.
Normal
   Standard normal input.

Properties
==========

Space
   Object Space means the displacement scales along with the object.
   When using World Space the object scale is ignored.

Outputs
=======

Displacement
   Displacement offset to be connected into the Material Output.

Examples
========
