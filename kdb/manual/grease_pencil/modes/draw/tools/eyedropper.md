---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

.. _tool-grease-pencil-draw-eyedropper:

**********
Eyedropper
**********

The Eyedropper tool is used to create materials or palette color based on sampled colors in the 3D Viewport.

Tool Settings
=============

.. _bpy.ops.ui.eyedropper_grease_pencil_color:

:Material:
   Create a new material with the Stroke Base Color to be the sampled color.

   Material Mode
      The color transformation will be applied on the stroke and/or the fill color.

      :Stroke: Only paint over strokes.
      :Fill: Only paint over fill areas.
      :Both: Paint over strokes and fill areas
:Palette: Add a new color to the color palette based on the sampled color.
:Brush: Sets the brush color to the sampled color.

Usage
=====

- LMB Create a stroke material.
- Shift-LMB Create a fill material.
- Shift-Ctrl-LMB Create both a stroke and fill material.
- Holding LMB and dragging accumulates the average color under the mouse cursor.
