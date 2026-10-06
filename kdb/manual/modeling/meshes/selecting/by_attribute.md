---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.mesh.select_by_attribute:

************
By Attribute
************

Selects the vertices, edges, or faces for which the currently active
attribute </modeling/geometry_nodes/attributes_reference> has a value of True.

The active attribute is the one which is currently selected in the
Attributes list <bpy.types.AttributeGroup> of the /editors/properties_editor.

**Note:**

   The attribute must have a domain <attribute-domains> of Vertex, Edge, or Face,
   and a type <attribute-data-types> of Boolean.
