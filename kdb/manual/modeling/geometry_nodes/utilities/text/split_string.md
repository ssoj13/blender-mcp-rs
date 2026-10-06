---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.FunctionNodeSplitString:

*****************
Split String Node
*****************

The *Split String* node splits a string into a list of substrings using a separator string.

Inputs
======

String
   The string value to split.

Separator
   The string used to determine where the input string is split.

   Each occurrence of the separator marks the boundary between list items.

Outputs
=======

List
   A list of strings resulting from splitting the input string at each occurrence
   of the separator.
