---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.WeldModifier:

*************
Weld Modifier
*************

The *Weld* modifier looks for groups of vertices within a threshold and merges them,
collapsing the surrounding geometry.

Options
=======

Mode
   Method for choosing which vertices are merged.

   :All: Merge includes all geometry including loose parts.
   :Connected: Merge only includes attached geometry i.e. the modifier will not merge loose parts together.

Distance
   Maximum distance that the vertices must have each other to be merged.

Only Loose Edges Connected Mode
   Only collapse short edges which are not adjacent to any face.
   This is useful for example to stitch the seams used in cloth simulations.

Vertex Group
   When the *Vertex Group* option is selected, only vertices with weight above zero will be affected by the modifier.

   :bl-icon:`arrow_leftright` Invert
      Inverts the influence of the selected vertex group, meaning that the group
      now represents vertices that will not be merged by the modifier.

      The setting reverses the weight values of the group.
