---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

.. _bpy.types.GreasePencilEnvelopeModifier:

*****************
Envelope Modifier
*****************

The *Envelope* modifier creates a shape known as envelope over the existing strokes
connecting all the points that have n points between them.

Options
=======

Mode
   :Deform: Replaces the original stroke with the envelope shape.
   :Segments: Add segments to create the envelope shape keeping the original stroke.
   :Fill: Add segments to create the envelope without the original stroke.

Spread Length
   The number of points to skip when creating the straight segments that define the envelope.

Thickness
   The thickness of the generated stroke segments.

Strength
   The Opacity of the generated stroke segments.

Material Index
   Defines the material to use on the generated stroke segments.

Skip Segments
   The Number of generated stroke segments to skip to reduce complexity.

Influence Filters
-----------------

See grease-pencil-modifier-influence-filters.
