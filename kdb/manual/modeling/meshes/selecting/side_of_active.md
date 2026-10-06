---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.mesh.select_axis:

**************
Side of Active
**************

Selects all the vertices that are (for example) to the right of the active vertex.

Options
=======

Axis Mode
   The transform orientation </editors/3dview/controls/orientation> from which to take the *Axis*.

Axis Sign
   Select the vertices whose *Axis* coordinate is...

   Positive Axis
      ...greater than or equal to that of the active vertex.
   Negative Axis
      ...less than or equal to that of the active vertex.
   Aligned Axis
      ...equal to that of the active vertex.

Axis
   The axis along which to compare coordinates.

Threshold
   Tolerance for vertices that are slightly outside of the coordinate range.
