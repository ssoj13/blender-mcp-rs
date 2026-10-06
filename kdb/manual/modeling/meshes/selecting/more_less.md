---
area: manual-modeling
source: blender-manual
version: 5.2
---

****************
Select More/Less
****************

Select More
===========

.. _bpy.ops.mesh.select_more:

Grows the selection to include the elements at its border.
Specifically, this adds the neighbors of the currently selected elements,
with the type of those neighbors (vertices, edges, or faces) depending on the
Selection Mode </modeling/meshes/selecting/introduction>.

Options
-------

Face Step
   Add whole neighboring faces even if they only share a single vertex.

.. _bpy.ops.mesh.select_less:

Select Less
===========

Shrinks the selection to exclude the elements at its border.
Specifically, this deselects elements that have any unselected neighbors,
with the type of those neighbors (vertices, edges, or faces) depending on the
Selection Mode </modeling/meshes/selecting/introduction>.

Options
-------

Face Step
   Deselect elements that share a vertex with an unselected face.

.. _bpy.ops.mesh.select_next_item:

Select Next Active
==================

Selects the element that "logically follows" the two most recently selected ones.

.. list-table::

   * - .. figure:: /images/modeling_meshes_selecting_more-less_select-active-1.png
          :width: 200px

          Initial selection.

     - .. figure:: /images/modeling_meshes_selecting_more-less_select-active-2.png
          :width: 200px

          Using Next Active once.

     - .. figure:: /images/modeling_meshes_selecting_more-less_select-active-3.png
          :width: 200px

          Using Next Active twice.

.. _bpy.ops.mesh.select_prev_item:

Select Previous Active
======================

Deselects the last selected element and makes the next-to-last one active again.
