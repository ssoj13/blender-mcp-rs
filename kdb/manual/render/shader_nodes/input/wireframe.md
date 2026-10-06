---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.ShaderNodeWireframe:

**************
Wireframe Node
**************

The Wireframe node is used to retrieve the edges of an object as it appears to Cycles.
As meshes are triangulated before being processed by Cycles,
topology will always appear triangulated when viewed with the Wireframe node.

Inputs
======

This node has no inputs.

Properties
==========

Pixel Size
   When enabled, the size of edge lines is set in screen space.
Size
   Thickness of the edge lines.

Outputs
=======

Factor
   Black-and-white mask showing white lines representing edges according to the object's Topology.

Examples
========
