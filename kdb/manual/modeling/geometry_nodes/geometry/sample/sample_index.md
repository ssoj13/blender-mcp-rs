---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSampleIndex:

*****************
Sample Index Node
*****************

The *Sample Index* node retrieves values from a source geometry at a specific index.

**Tip:**

   If the *Geometry* used for the input is the same as the geometry from the field context <field-context>,
   this node is equivalent to the /modeling/geometry_nodes/utilities/field/evaluate_at_index. Using that node
   is usually preferable since avoiding the geometry socket makes the whole setup easier to use in other situations
   and share.

**Tip:**

   The Point domain is shared by different components (Mesh, Point Cloud, Curve).
   This node simply samples the *Value* from the first non-empty component,
   checked in the order: Mesh, Point Cloud, Curve.
   The /modeling/geometry_nodes/geometry/operations/separate_components can be used to sample directly from a
   specific component.

Inputs
======

Geometry
   The geometry to retrieve the attribute from.

Value
   A field to evaluate on the source *Geometry*. The values are then retrieved from specific
   indices for the output.

Index
   Which index to use when retrieving the data from the input *Value* field. Any index can be
   connected, resulting in a "shuffling" of the values.

Properties
==========

Data Type
   The data type <attribute-data-types> to use for the retrieved values.

Domain
   The attribute domain <attribute-domains> that the attribute is transferred from, or in other words,
   the domain used to evaluate the *Attribute* input. For example, it is possible to transfer data from the
   faces of one geometry to the points of another.

Clamp
   Clamp the indices to the size of the attribute domain instead of outputting a default value for invalid indices.

Outputs
=======

Value
   The data retrieved from the source *Geometry* input.

Examples
========
