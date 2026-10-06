---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeImportText:

****************
Import Text Node
****************

The *Import Text* node reads the contents of a plain text file and outputs it as a single string.
This can be useful in motion graphics or data-driven workflows where external text needs to be
processed or displayed.

**Note:**

   Currently, only files with the `.txt` extension are supported.

Inputs
======

Path
   The path to the txt file.

Outputs
=======

String
   The entire contents of the text file as a single string.
