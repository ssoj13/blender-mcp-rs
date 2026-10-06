---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeStoreNamedAttribute:

**************************
Store Named Attribute Node
**************************

The *Store Named Attribute* node stores the result of a field on a geometry
as an attribute with the specified name. If the attribute already exists, the data
type and domain will be updated to the values chosen in the node.
However, keep in mind that the domain and data type of geometry-nodes_builtin-attributes
cannot be changed.

Compared with the /modeling/geometry_nodes/attribute/capture_attribute, this node basically
does the same thing, but the attribute gets a name instead of an anonymous reference.
For reusing the data in the same node tree, the *Capture Attribute* node might be preferable
since it does not create the chance for name conflicts in the input geometry.

**Note:**

   If the input geometry contains multiple geometry component types, the attribute will be
   created on each component that has the chosen domain.

Inputs
======

Geometry
   Standard geometry input.

Selection
   A boolean field input for each element indicating whether to store the attribute value for the given index.
   If the attribute does not exist, unselected parts are filled with zero values;
   otherwise, only the selected parts are filled.

Value
   The input field to evaluate.

Name
   The name to give the stored data.

Properties
==========

Data Type
   The data type <attribute-data-types> used for the evaluated data.

Domain
   Which attribute domain <attribute-domains> to store the evaluated data on.

Outputs
=======

Geometry
   Standard geometry output.
