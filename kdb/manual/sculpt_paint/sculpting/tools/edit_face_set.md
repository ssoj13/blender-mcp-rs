---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

*************
Edit Face Set
*************

Edits the Face Set </sculpt_paint/sculpting/editing/face_sets> under the cursor.

Tool Settings
=============

Mode
   The operation to apply to the face set.

   :Grow Face Set:
      Grows the face sets boundary by one face based on mesh topology.
      This is also available as a shortcut operator
      via Ctrl-W.
   :Shrink Face Set:
      Shrinks the face sets boundary by one face based on mesh topology.
      This is also available as a shortcut operator
      via Ctrl-Alt-W.
   :Delete Geometry:
      Deletes the faces that are assigned to the face set.
   :Fair Positions:
      Creates a perfectly flat and smooth geometry patch from the face set.
      This is the ideal way to trim parts of your mesh
      if the vertex count is too high for other operations,
      or the vertex IDs must not be altered
      (Like when using Multires </modeling/modifiers/generate/multiresolution> sculpting).
   :Fair Tangency:
      Creates a smooth as possible geometry patch from the face set
      by minimizing changes in vertex tangents <Tangent>.
      This is ideal for creating smooth curved surfaces on complex topology,
      where just using the smooth brush will not lead to desired results

   .. list-table::

      * - .. figure:: /images/sculpt-paint_sculpt_fairing_none.png

            Before fairing.

        - .. figure:: /images/sculpt-paint_sculpt_fairing_positions.png

            After using Fair Positions.

        - .. figure:: /images/sculpt-paint_sculpt_fairing_tangency.png

            After using Fair Tangency.

Strength
   The amount of effect the filter has on the mesh.
   This setting is only available for the fairing operations.

Modify Hidden
   Apply the edit operation to hidden face sets.
