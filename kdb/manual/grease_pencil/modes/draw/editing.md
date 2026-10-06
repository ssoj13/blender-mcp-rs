---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

******************
Drawing Operations
******************

Active Layer
============

Select the active layer.

Animation
=========

The stroke animation operations are described in the Animation </grease_pencil/animation/tools> section.

Interpolate Sequence
====================

See bpy.ops.grease_pencil.interpolate_sequence.

.. _bpy.ops.grease_pencil.erase_lasso:

Erase Lasso
===========

The *Erase Lasso* operator erases all Grease Pencil strokes within a freeform selection.

- Press and hold Ctrl-Alt-RMB, then draw a lasso shape around the area you want to erase.
- Release the mouse button to apply the eraser.
- Only points within the lasso region are removed.

.. _bpy.ops.grease_pencil.erase_box:

Box Erase
=========

The *Box Erase* operator erases all Grease Pencil strokes within a rectangular selection.

- Press B, then click and drag to define a rectangular area.
- Release the mouse button to apply the eraser.
- All points of strokes within the box are removed.
