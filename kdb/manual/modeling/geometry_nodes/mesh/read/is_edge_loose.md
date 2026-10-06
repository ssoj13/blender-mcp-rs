---
area: manual-modeling
source: blender-manual
version: 5.2
---

******************
Is Edge Loose Node
******************

The *Is Edge Loose* node outputs a boolean value indicating whether each edge in a mesh
is *loose*. A loose edge is not connected to any faces, meaning it exists independently
of the mesh surface.

This node can be used to detect or isolate wireframe elements, stray geometry, or edges
intentionally used for guides or supports.

Inputs
======

This node has no inputs.

Outputs
=======

Is Edge Loose
   Boolean value that indicates whether each edge is loose.
   True if the edge is not connected to any faces; false otherwise.
