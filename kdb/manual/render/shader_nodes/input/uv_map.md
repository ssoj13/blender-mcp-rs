---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeUVMap:

***********
UV Map Node
***********

The *UV Map* node outputs texture coordinates from a specific UV map <uv-maps-panel>.
It is especially useful when working with multiple UV layouts in the same material.
The node is most commonly connected to the *Vector* input of texture nodes,
allowing textures to be placed or manipulated.

**Note:**

   Unlike the Texture Coordinate Node </render/shader_nodes/input/texture_coordinate>,
   which only outputs the active render UV map, this node can access any UV map assigned to the object.

Inputs
======

This node has no inputs.

Properties
==========

From Instancer Cycles Only
   See the From Instancer
   option of the Texture Coordinate Node </render/shader_nodes/input/texture_coordinate>.

UV Map
   The name of the UV map <uv-maps-panel> to use.
   This list is populated from the UV maps available on the object's mesh data-block.
   If left unset, the Active Render <bpy.types.MeshUVLoopLayer.active_render> UV map is used.

Outputs
=======

UV
   UV coordinates from the specified UV map.
