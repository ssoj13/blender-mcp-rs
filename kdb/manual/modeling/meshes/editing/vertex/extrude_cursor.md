---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.mesh.dupli_extrude_cursor:

************************
Extrude to Cursor or Add
************************

Either creates a new vertex at the mouse cursor, or extrudes the selected vertices/edges
up to the mouse cursor.

If nothing is selected, pressing Ctrl-RMB will create a vertex at the depth of the
/editors/3dview/3d_cursor.

If there are one or more selected vertices that are isolated (not connected to any edge or face),
pressing Ctrl-RMB will extrude each one into an edge. The new vertices will be placed at
the same depth from the current viewpoint as the original ones. In addition, the original vertices
will be deselected and the new ones will be selected, so that this operation can be repeated to
create chains of edges.

Finally, if there are two or more selected vertices that are connected by edges, pressing
Ctrl-RMB will extrude them into faces.

The previously selected edges will be rotated to get a smoother result. If this is not desired,
press Shift-Ctrl-RMB instead.

**Tip:**

   When extruding, use a top or side view </editors/3dview/navigate/viewpoint> to
   stay within a global plane. For example, using the top view will ensure that the new
   vertices have the same global Z coordinate as the previous ones.
