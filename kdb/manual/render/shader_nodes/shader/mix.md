---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeMixShader:

**********
Mix Shader
**********

The *Mix* node is used to mix two shaders together. Mixing can be used for material layering,
where the *Factor* input may, for example, be connected to a *Blend Weight* node.

Inputs
======

Shader
   Shaders to mix, such that incoming rays hit either with the specified probability in the *Factor* socket.
Factor
   Blend weight to use for mixing two shaders;
   at zero it uses the first shader entirely and at one the second shader.

Properties
==========

This node has no properties.

Outputs
=======

Shader
   Standard shader output.

Examples
========
