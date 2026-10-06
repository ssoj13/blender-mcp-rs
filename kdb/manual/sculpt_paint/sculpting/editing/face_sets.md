---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

.. _sculpting-editing-facesets:

*********
Face Sets
*********

This page details the face set related hotkey operators and menu operators in sculpt mode.

**Tip:**

   There is a face set pie menu that can be accessed with Alt-W.

.. _bpy.ops.sculpt.face_sets_create:

Face Set from Masked
====================

Creates a new face set from Masked Geometry <sculpt-mask-menu>.

Face Set from Visible
=====================

Creates a new face set from all visible geometry.

Face Set from Edit Mode Selection
=================================

Creates a new face set corresponding to the Edit Mode face selection.

.. _bpy.ops.sculpt.face_sets_init:

Initialize Face Sets
====================

Initializes all face sets on the mesh at once based off one of several mesh attribute properties.

Mode
   The mesh data attribute used to define the boundaries for the face sets.

   :By Loose Parts: Creates a new face set per discontinuous part of the mesh.
   :By Face Set Boundaries:
      Creates a face set for each isolated face set.
      This mode is useful for splitting the patterns created by Face Set Expand
      into individual Face Sets for further editing.
   :By Materials: Creates a face set per Material Slot <bpy.types.MaterialSlot>.
   :By Normals: Creates face sets for Faces that have similar Normals <modeling-meshes-structure-normals>.
   :By UV Seams: Creates face sets using UV Seams </modeling/meshes/uv/unwrapping/seams> as boundaries.
   :By Edge Creases: Creates face sets using Edge Creases <bpy.ops.transform.edge_crease> as boundaries.
   :By Edge Bevel Weight:
      Creates face sets using Bevel Weights <bpy.ops.transform.edge_bevelweight> as boundaries.
   :By Sharp Edges: Creates face sets using Sharp Edges <bpy.ops.mesh.mark_sharp> as boundaries.

Threshold
   The minimum value to consider a certain attribute a boundary when creating the face sets.

.. _bpy.ops.sculpt.face_set_edit:

Grow/Shrink Face Sets
=====================

Expands or contracts the face set under the cursor by adding or removing surrounding faces.

.. _bpy.ops.mesh.face_set_extract:

Expand Face Set
===============

**Note:**

   More info on Face Set Expand at the Expand page <bpy.ops.sculpt.expand>.

Extract Face Set
================

Creates a new mesh based on the selected face set.
Once the operator is initiated, hover over the face set and LMB to create the new mesh.
After the operator is finished the new mesh will be selected in Object Mode.

.. _bpy.ops.sculpt.face_sets_randomize_colors:

Randomize Colors
================

Generates a new set of random colors to render the face sets in the 3D Viewport.

Display Settings
================

The face sets display can be toggled as a viewport overlay </editors/3dview/display/overlays>.
In the overlay popover, the opacity of the face sets overlay can be adjusted
to make it more or less visible on the mesh.
