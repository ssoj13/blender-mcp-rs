---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSetMaterialIndex:

***********************
Set Material Index Node
***********************

The *Set Material Index* node sets the material index for a geometry.

The node to get this data is the Material Index </modeling/geometry_nodes/geometry/material/material_index>
node.

Inputs
======

Geometry
   Standard geometry input.

Selection
   Whether to change the material index for each face.
   True values mean the material index will be changed, false values mean it will remain the same.

Material Index
   The new material index.

Outputs
=======

Geometry
   Standard geometry output.
