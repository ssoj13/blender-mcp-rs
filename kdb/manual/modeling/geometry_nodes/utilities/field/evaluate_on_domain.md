---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeFieldOnDomain:

***********************
Evaluate on Domain Node
***********************

The *Evaluate on Domain* allows evaluating a field for a different attribute domain <attribute-domains>
than the domain from the field context <field-context>. For example, the face index could be used instead
of the face corner index, when setting the values of a UV Map

**Note:**

   This node is not necessary to retrieve data from other attribute domains; that is done automatically.
   Its utility comes from the fact that it's possible to control *when* the domain interpolation happens.
   Normally, input nodes interpolate their data to the current context's domain as soon as they create
   their output.

**Tip:**

   It may be preferable to use this node over the /modeling/geometry_nodes/attribute/capture_attribute,
   since it allows using a specific attribute domain without requiring a geometry socket input,
   which allows creating more reusable node groups.

**Seealso:**

   The method of retrieving data from another domain is somewhat similar to the
   /modeling/geometry_nodes/utilities/field/evaluate_at_index.

Inputs
======

Value
   The field to evaluate on the chosen attribute domain.

Properties
==========

Domain
   The attribute domain <attribute-domains> used for evaluation of the *Value* input.

Output
======

Value
   The values from the input, evaluated on the chosen domain, then interpolated to the domain from
   the field context <field-context>.
