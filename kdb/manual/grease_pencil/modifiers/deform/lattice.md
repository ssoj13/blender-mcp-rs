---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

.. _bpy.types.GreasePencilLatticeModifier:

****************
Lattice Modifier
****************

The Lattice modifier deforms the base object according to
the shape of a Lattice </animation/lattice> object.

**Tip:**

   A Lattice Modifier can quickly be added to selected objects by selecting them all,
   then selecting the lattice object last and pressing Ctrl-P and choosing *Lattice Deform*.
   This will both add Lattice Modifiers to the selected objects and parent them to the lattice.

**Seealso:**

   This documentation refers to the Lattice Modifier specific to the Grease Pencil object.
   For uses with other object types refer to the general /modeling/modifiers/deform/lattice.

Options
=======

Object
   The Lattice </animation/lattice> object with which to deform the base object.

Strength
   A factor to control blending between original and deformed points positions.

Influence
---------

See grease-pencil-modifier-influence-filters.

Example
=======

.. list-table:: Lattice modifier example.

   * - .. figure:: /images/grease-pencil_modifiers_deform_lattice_original.png
          :width: 320px

          Original model.

     - .. figure:: /images/grease-pencil_modifiers_deform_lattice_edit.png
          :width: 320px

          After lattice editing.
