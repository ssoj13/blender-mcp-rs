---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeImportPLY:

***************
Import PLY Node
***************

The *Import PLY* node imports mesh data from a `.ply` (Polygon File Format or Stanford Triangle Format) file.

PLY files typically store geometry from 3D scanning or modeling software and can include
vertex attributes such as colors or normals. The geometry is imported as a single mesh.

This node allows procedural workflows to incorporate external static geometry directly into Geometry Nodes.

**Seealso:**

   PLY Import Operator

Inputs
======

Path
   The path to the `.ply` file to be imported.

Outputs
=======

Mesh
   The mesh geometry imported from the PLY file.
