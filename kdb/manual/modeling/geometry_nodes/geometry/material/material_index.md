---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeInputMaterialIndex:

*******************
Material Index Node
*******************

The *Material Index* node outputs which material in the list of materials of the geometry
each element corresponds to. Currently the node supports mesh data, where `material_index`
is a built-in attribute on faces.

The node to set this data is
the Set Material Index </modeling/geometry_nodes/geometry/material/set_material_index> node.

Inputs
======

This node has no inputs.

Outputs
=======

Material Index
   Standard integer value, with a minimum value of zero.
