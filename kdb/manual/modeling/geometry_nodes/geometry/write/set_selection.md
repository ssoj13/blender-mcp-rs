---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeToolSetSelection:

******************
Set Selection Node
******************

The *Set Selection* node controls which geometry is selected </interface/selecting>.

The input node for this data is the /modeling/geometry_nodes/geometry/read/selection.

**Note:**

   This node can only be used in the Tool context <tool_context>.

Inputs
======

Geometry
   Standard geometry input.

Selection
   Boolean field for specifying which elements should be selected in the output geometry.
   Elements for which this field evaluates to false are implicitly de-selected.

Properties
==========

Domain
   Which domain <attribute-domains> to set the selection on.

Outputs
=======

Geometry
   Standard geometry output.
