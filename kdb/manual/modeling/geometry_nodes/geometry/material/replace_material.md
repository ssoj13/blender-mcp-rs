---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeReplaceMaterial:

*********************
Replace Material Node
*********************

The *Replace Material* node swaps one material with another.
Replacing a material with this node is more efficient than creating a selection of all faces
with the old material with the /modeling/geometry_nodes/geometry/material/material_selection
and then using the /modeling/geometry_nodes/geometry/material/set_material.

**Note:**

   Currently this node only adjusts mesh data.

Inputs
======

Geometry
   Standard geometry input.

Old
   Material that is going to be replaced.

New
   Material that is replacing the old material.

Outputs
=======

Geometry
   Standard geometry output.
