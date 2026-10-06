---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.transform.tosphere:
.. _tool-transform-to_sphere:

*********
To Sphere
*********

The *To Sphere* transformation deforms the selected geometry to look more like a sphere.

.. _fig-mesh-deform-to-sphere-monkey:

Usage
=====

Click the menu item or press the keyboard shortcut, then move the mouse right or left
(or type a number between 0 and 1) to make the selection more or less spherical.
Finally, press LMB or Return to confirm, or RMB or Esc to cancel.

Options
=======

Factor
   A value between 0 and 1 indicating how strongly the selection is spherized.

Mirror Editing
   Whether to also affect matching geometry on the other side of the mesh.
   Mesh Symmetry
   needs to be enabled for this to work.

Proportional Editing
   See /editors/3dview/controls/proportional_editing.

Examples
========

Meshes with higher density will give better results:

The results are also different depending on the number and arrangement of the selected elements:
