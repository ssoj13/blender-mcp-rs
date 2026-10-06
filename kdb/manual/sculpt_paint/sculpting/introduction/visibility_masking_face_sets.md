---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

*******************************
Visibility, Masking & Face Sets
*******************************

Visibility Control
==================

Parts of the mesh can be hidden in Sculpt Mode.
Because hidden faces cannot be sculpted, hiding makes it easier to isolate what you want to work on.
Hiding geometry also improves the viewport performance.

Hiding is shared between all modes, except Object Mode
(i.e. hiding/showing of faces in one mode will hide the same faces in other modes too).

Unlike Selection Masking </sculpt_paint/selection_visibility> in other painting modes,
Sculpt Mode primarily uses Masks and Face Sets to easily control the mesh visibility
and which faces can currently be edited.
The exception is the Clipping Region <clipping_region>, which can be used in any mode.

The most common shortcuts are H to hide the face set under the cursor
and Shift-H to isolate the face set under the cursor (or show everything).

Inverting the visibility <bpy.ops.sculpt.face_set_change_visibility>
and showing all <bpy.ops.paint.hide_show_all> is also available in the Alt-W pie menu.

Modifying visibility can also be done via the /sculpt_paint/sculpting/tools/hide_tools.

**Seealso:**

   More information for controlling the visibility at Show & Hide </sculpt_paint/sculpting/editing/sculpt>.

.. _sculpt-masks:

Masks
=====

A mask is used to control which vertices of the mesh are influenced by sculpting and painting.
The mask can for example be created/edited via the /sculpt_paint/sculpting/tools/mask_tools
and Mask by Color </sculpt_paint/sculpting/tools/mask_by_color> tool.

Internally, masks are stored using the `sculpt_mask` /modeling/geometry_nodes/attributes_reference.

Clear & Invert
--------------

Creating masks follows a slightly different mental model than selecting in other modes.
For example Shift-LMB is used for smoothing instead of adding to a mask.

Masking is also conceptually inverted to selection
(i.e. You **cannot** edit masked vertices. But you **can** edit selected vertices).

Instead a mask is typically always added to the current mask with LMB and subtracted with Ctrl-LMB.
So if you wish to edit the masked surfaces, you'll need to use the Invert <mask_invert> operator,
In the case of masking everything that is visible,
the best workflow is to first Clear <mask_clear> and then Invert <mask_invert> the mask.

Both these operators can be quickly accessed in the A pie menu.

**Seealso:**

   More information about editing and using masks at the Mask Menu

.. _face_sets:

Face Sets
=========

Face sets are used to group your mesh into differently colored faces,
which can then be quickly hidden or shown like mentioned above.
They can also be used for fast mask creation via the Mask Expand <bpy.ops.sculpt.expand>.
Face Set Expand <face_set_expand> is also useful for creating, editing and joining face sets.

More options can be found in the Alt-W pie menu.

Otherwise Face Sets can be created/edited with the
Draw Face Sets </sculpt_paint/sculpting/brushes/draw_facesets> brush,
/sculpt_paint/sculpting/tools/mask_tools.
They can also be edited with the
Edit Face Set </sculpt_paint/sculpting/tools/edit_face_set> tool.

**Seealso:**

   More information about editing and using face sets at the
   Face Sets Menu

Internally, face sets are stored using the `sculpt_face_set` /modeling/geometry_nodes/attributes_reference.

Auto-Masking
============

Auto-Masking </sculpt_paint/sculpting/controls> is also a fast way of only editing specific geometry
without having to manually create a new mask or hide geometry.
This feature is especially useful in combination with face sets.

Display Settings
================

The mask and face sets display can be toggled and adjusted in the Mask Display Settings.
