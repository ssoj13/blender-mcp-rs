---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _tool-mesh-extrude_cursor:

*****************
Extrude to Cursor
*****************

Interactively places new vertices with Ctrl-RMB at the mouse cursor position.

The most basic element, a vertex, can be added with a Ctrl-RMB click
when no other vertices are selected.
Because the camera space (computer screen) is two-dimensional,
Blender cannot determine all three vertex coordinates from a single mouse click,
so the new vertex is placed at the depth of the 3D cursor.

To create interconnected vertices, you can add a vertex and continuously make subsequent Ctrl-RMB
operations with the last vertex selected.
This will link the last selected vertex with the vertex created at the mouse position with an edge
(see Fig. fig-mesh-basics-add-one-tool),
and will continuously create and connect new vertices if you continue repeating this operation.

.. _fig-mesh-basics-add-one-tool:

Creating Faces
==============

If you have two vertices selected and already connected with an edge, Ctrl-RMB click
will create a planar face, also known as a quad. Blender will follow your mouse cursor
and will use the planar view from your viewport to create those quads.

For Ctrl-RMB, Blender will automatically rotate the last selected Edge (the source)
for the subsequent operations if you have at least one face created, dividing the angles created between
the newly created edge and the last two edges, creating a smooth angle between them. Blender will calculate
this angle using the last positive and negative position of the last X and Y coordinates
and the last connected unselected edge. If this angle exceeds a negative limit (following a quadrant rule)
between the recently created edge and the last two, Blender will wrap the faces.
But if you do not want Blender to rotate and smooth edges automatically when extruding from Ctrl-RMB,
you can also inhibit Blender from rotating sources using the shortcut Shift-Ctrl-RMB.
In this case, Blender will not rotate the source dividing the angle between those edges when creating a face.

If you have three or more vertices selected, and Ctrl-RMB click,
you will also create planar faces, but along the vertices selected, following the direction of the cursor.
This operation is similar to an extrude operation.

**Tip:**

   When adding objects with Ctrl-RMB, the extrusions of the selected elements,
   being vertices, edges and faces with the Ctrl-RMB, are viewport dependent.
   This means, once you change your viewport, for example, from top to left, bottom or right,
   the extrusion direction will also follow your viewport and align the extrusions with your planar view.
