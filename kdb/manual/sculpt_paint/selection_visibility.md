---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

**********************
Selection & Visibility
**********************

Selection Masking
=================

If you have a complex mesh, it is sometimes not easy to paint on the intended vertices.
Suppose you only want to paint on a small area of the Mesh and keep the rest untouched.
This is where "selection masking" comes into play. When this mode is enabled,
a brush will only paint on the selected vertices or faces.
The option is available from the header of the 3D Viewport
(see icons surrounded by the yellow frame):

Selection masking has some advantages over the default paint mode:

- The original mesh edges are shown, even when modifiers are active.
- You can select and deselect faces instead without the need to switch to Edit Mode.

.. _bpy._paint.vert_select_loop:

Details About Selecting
-----------------------

The following standard selection operations are supported:

- Alt-LMB -- Single faces
- Shift-Alt-LMB -- Select more or remove them from the selection.
- A -- All faces, A A to deselect.
- B -- Box selection.
- C -- Circle select with brush.
- Ctrl-I -- Invert selection.
- L -- Pick linked (under the mouse cursor).
- Ctrl-L -- Select linked.
- Ctrl-NumpadPlus -- Extend Selection
- Ctrl-NumpadMinus -- Shrink Selection
- Alt-LMB -- Vertex Loop Select

Vertex Selection Masking
------------------------

In this mode you can select one or more vertices and then paint only on the selection.
All unselected vertices are protected from unintentional changes.

.. _bpy.types.Mesh.use_paint_mask:

Face Selection Masking
----------------------

The *Face Selection Masking* allows you to select faces and limit the paint
tool to those faces, very similar to Vertex selection masking.

.. The visual example needs to be updated. Ideally show hard edges on painted face corners.

.. _bpy.ops.paint.face_vert_reveal:
.. _bpy.ops.paint.face_select_hide:

Hide/Unhide Faces
-----------------

Faces can be temporarily hidden to limit painting to only the visible parts of the mesh.
Hidden faces cannot be painted or selected until they are revealed again.
This can be useful for isolating specific regions of a mesh while painting.

- H -- Hide the currently selected faces.
- Shift-H -- Hide all unselected faces, isolating the current selection.
- Alt-H -- Reveal all hidden faces.

Hide/Unhide Vertices
--------------------

You cannot specifically hide only selected faces in vertex mask selection mode.
However, the selection is converted when switching selection modes.
So a common trick is to:

#. Switch to Face selection mask mode to have the selection converted to faces.
#. Refine your selection next or just hide the faces.
#. Switch back to Vertex Selection mask mode.

Hiding faces will make sure that vertices that belong to visible faces remain visible.

.. _clipping_region:

The Clipping Region
-------------------

To constrain the paint area further you can use the *Clipping Region*.
Press Alt-B and LMB-drag a rectangular area.
The selected area will be "cut out" as the area of interest.
The rest of the 3D Viewport gets hidden.

.. This visual example could include the (Clipped) text info overlay

You make the entire mesh visible again by pressing Alt-B a second time.

All paint tools that use the view respect this clipping, including box select, and of course brush strokes.

There are two helpful reminders that a Clipping Region is used:

#. The clipping region is drawn as a gray box in the 3D Viewport
#. The Text Info overlay will state that the perspective is "Clipped"

.. _bpy.ops.paint.vert_select_linked:

Select Linked
=============

Select geometry connected to already selected elements.
This is often useful when a mesh has disconnected, overlapping parts,
where isolating it any other way would be tedious.
Pressing Shift-L will deselect linked any linked elements.

With L you can also select connected geometry directly under the cursor.
