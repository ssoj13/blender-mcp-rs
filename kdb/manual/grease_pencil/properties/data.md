---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

***************
Data Properties
***************

Grease Pencil
   The Grease Pencil data-block menu <ui-data-block> can be used to link the data between objects.

Layers </grease_pencil/properties/layers>
================================================

Strokes can be grouped in 2D layers, a special Grease Pencil layers
that help to organize the drawing order and visibility of the strokes.
Layers can be organized into layer groups.

Onion Skinning </grease_pencil/properties/onion_skinning>
================================================================

Onion skinning is used in animation to see several frames at once and make decisions or
edits based on how the previous/next frames are drawn.

Settings </grease_pencil/properties/strokes>
===================================================

General settings for Grease Pencil strokes.

.. _bpy.types.AttributeGroupGreasePencil:

Attributes
==========

Layers can store Custom Attributes </modeling/geometry_nodes/attributes_reference>.
The attributes are stored on the Layer <attribute-domains> domain.

For example, the Layer Adjustments <layer-adjustments> are stored as layer attributes.

Attributes
   List view <ui-list-view> of all the attributes stored on the layers.

   Name
      Name of the layer attribute.

   Data Type
      The Data Type <attribute-data-types> of the attribute.

Vertex Groups </modeling/meshes/properties/vertex_groups/index>
======================================================================

Vertex groups can be used to assign a group or weighted group to some operator.
An object can have several weight groups and can be assigned in
Weight Paint Mode </grease_pencil/modes/weight_paint/index>.

Custom Properties <files-data_blocks-custom-properties>
==============================================================

Create and manage your own properties to store data in the Grease Pencil's data-block.
