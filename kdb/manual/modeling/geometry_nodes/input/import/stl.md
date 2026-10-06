---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeImportSTL:

***************
Import STL Node
***************

The *Import STL* node imports mesh geometry from a `.stl` (Stereolithography) file.

STL files are commonly used for 3D printing and CAD applications and contain only surface geometry,
typically as a collection of triangles with no color, texture, or other attributes.

This node enables incorporating external STL assets into Geometry Nodes for procedural processing or visualization.

**Seealso:**

   STL Import Operator

Inputs
======

Path
   The path to the `.stl` file to be imported.

Outputs
=======

Mesh
   The mesh geometry imported from the STL file.
