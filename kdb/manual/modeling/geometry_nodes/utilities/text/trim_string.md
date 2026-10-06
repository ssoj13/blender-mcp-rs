---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.FunctionNodeTrimString:
.. --- copy below this line ---

****************
Trim String Node
****************

The *Trim String* node removes specified characters from the beginning and/or end
of a string. It does not affect characters in the middle of the string.

This node is useful for cleaning up input data, such as removing unwanted prefixes,
suffixes, or extra whitespace.

Inputs
======

String
   The input string to trim.

Characters
   A set of characters to remove from the start and/or end of the string.

   Each character is treated individually, and the order does not matter.
   Any matching characters found at the boundaries of the string are removed
   repeatedly until a non-matching character is encountered.

Whitespace
   Removes whitespace characters (such as spaces, tabs, and line breaks)
   in addition to the characters specified in *Characters*.

Limit
-----

Start
   Enables trimming from the beginning of the string.

End
   Enables trimming from the end of the string.

Outputs
=======

String
   The resulting string after trimming.
