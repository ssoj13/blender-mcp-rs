---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeStringJoin:

*****************
Join Strings Node
*****************

The *Join Strings* node combines any number of input strings into the output string.
The order of the result depends on the vertical ordering of the inputs in the multi-input socket.

**Tip:**

   This node can be used to create a multi-line string for
   the /modeling/geometry_nodes/utilities/text/string_to_curves,
   when combined with the line break output from
   the /modeling/geometry_nodes/utilities/text/special_characters.

Inputs
======

Delimiter
   String value to place between each concatenated string.

Strings
   Multiple string values to be combined in connection order.

Outputs
=======

String
   String result of the concatenation.
