---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

************
Weights Menu
************

This page covers many of the tools in the *Weights* menu.

.. _bpy.ops.grease_pencil.vertex_group_normalize_all:

Normalize All
=============

For each point, this tool makes sure that the sum of the weights across all vertex groups is equal to 1.
It normalizes all of the vertex groups, except for locked groups, which keep their weight values untouched.

Lock Active
   Keep the values of the active group while normalizing all the others.

.. _bpy.ops.grease_pencil.vertex_group_normalize:

Normalize
=========

This tool only works on the active vertex group.
All points keep their relative weights, but the entire set of weights is scaled up
such that the highest weight value is 1.0.

.. _bpy.ops.grease_pencil.weight_invert:

Invert
======

Replaces each weight of the selected vertex group by × -1.0 weight.

Examples:

- Original 1.0 converts to 0.0
- Original 0.5 remains 0.5
- Original 0.0 converts to 1.0

.. _bpy.ops.grease_pencil.vertex_group_smooth:

Smooth
======

Smooths the weights of the active vertex group.

.. _bpy.ops.grease_pencil.weight_sample:

Sample Weight
=============

Adjust the Weight of the Draw </grease_pencil/modes/weight_paint/tools>
tool to the weight of the vertex under the mouse cursor.
