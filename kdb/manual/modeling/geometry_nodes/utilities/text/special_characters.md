---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.FunctionNodeInputSpecialCharacters:
.. --- copy below this line ---

***********************
Special Characters Node
***********************

The *Special Characters* node is used to output string characters that can't be typed directly with the keyboard.

**Tip:**

   This node can be used to create a multi-line string for
   the /modeling/geometry_nodes/utilities/text/string_to_curves,
   when combined with the /modeling/geometry_nodes/utilities/text/join_strings
   or the /modeling/geometry_nodes/utilities/text/replace_string.

Inputs
======

This node has no inputs.

Outputs
=======

Line Break
   A new line character (escape character `\n`).

Tab
   A tab character used to add an indentation in the output.
