---
area: manual-modeling
source: blender-manual
version: 5.2
---

************
Introduction
************

Text objects contain some text, and are in the same object type family as curves and surfaces ones,
as fonts are vector data (they are made of curves).

Blender uses a "Font System" to manage mapping letter codes to geometry representing them in the 3D Viewport.
This font system has its own *built-in* font, but it can use external fonts too,
including *PostScript Type 1*, *OpenType* and *TrueType* fonts.
And moreover, it can use any objects existing in the current blend-file as letters.

Text objects allow you to create and render 2D or 3D text,
with various advanced layout options, like justifying and frames.
By default, letters are just flat filled surfaces, exactly like any closed 2D curve.
But, just like curves, you can extrude them,
and apply modifiers </modeling/modifiers/introduction> to them
(e.g. to make them follow a curve </modeling/modifiers/deform/curve>).

Text in Blender can be laid out in some relatively advanced ways,
defining columns or blocks of text, using different alignments, and so on.

Those features are similar in concept to what you can find in DTP (DeskTop Publishing) software
(like `Scribus <https://www.scribus.net/>`__), although at a very basic level currently.

**Tip:**

   You can convert a text object, either to a curve, or directly to a mesh,
   using object-convert-to in Object Mode.

**Note:**

   A maximum of 50,000 characters is allowed per text object. However,
   be forewarned that the more characters a single text object has,
   the slower the object will respond interactively.
