---
area: manual-modeling
source: blender-manual
version: 5.2
---

**************************
Selecting Surface Elements
**************************

When editing a Surface object, it's only possible to select control points.
There are no edges, faces, or Bézier handles.

**Seealso:**

   /interface/selecting for general information about selecting items in Blender.

Select All
==========

Select all control points.

Select None
===========

Deselect all control points.

Select Invert
=============

Select all the unselected control points and deselect the selected ones.

Box Select
==========

See bpy.ops.*.select_box.

Circle Select
=============

See bpy.ops.*.select_circle.

Lasso Select
============

See bpy.ops.*.select_lasso.

Select Random
=============

Adds random control points to the selection. The bpy.ops.screen.redo_last
panel offers the following options:

Ratio
   The ratio of control points that should end up selected, e.g. 0.5 to select half of all points
   (that are not hidden </scene_layout/object/editing/show_hide>).

   Note that the existing selection is ignored: if half of the control points are already selected,
   setting *Ratio* to 0.1 won't deselect anything, nor will it select 10% of the unselected
   points. Instead, it always picks 10% of *all* visible points and adds them to the selection.
Random Seed
   A number that influences which specific control points get picked.
Action
   :Select: Add random control points to the selection.
   :Deselect: Remove random control points from the selection.

Checker Deselect
================

Deselects control points according to a repeating pattern, starting at the active point.
For example, if *Deselected* is set to 2 and *Selected* to 3, the operator will
deselect the first two consecutive points, leave the next three unchanged, deselect
the two that come after those, and so on. This is done in both the "horizontal" and
"vertical" direction of the grid.

Deselected
   The number of control points to deselect in each pattern repetition.
Selected
   The number of control points to leave unchanged in each pattern repetition.
Offset
   Offset from the starting point.

Select More/Less
================

Expands or contracts the selection based on the currently selected control points.

More
   Adds the neighbors of the selected control points to the selection.
Less
   Removes the control points with at least one unselected neighbor from the selection.

Select Linked
=============

Press Ctrl-L to select all control points that are "linked to" (belong to the same
surface as) a control point that's already selected.

Alternatively, press L to select the control point under the mouse cursor and all
control points that are linked to it.

Select Similar
==============

Selects control points that are similar in some way to a selected control point.
The bpy.ops.screen.redo_last panel provides options for this:

Type
   Which property to compare for determining similarity.

   :Type:
      This option does not apply for Surfaces because, unlike Curves, they only support one handle type.
   :Radius:
      Selects control points that have a similar Radius value.
      See /modeling/surfaces/properties/transform_panel.
   :Weight:
      Selects control points that have a similar Weight value.
      See /modeling/surfaces/properties/transform_panel.
   :Direction: Selects control points that have a similar normal vector.

Compare
   How to compare unselected control points to selected ones.

   :Equal:
      Select control points whose property is equal to that of a selected point.
   :Greater:
      Select control points whose property is greater than or equal to the smallest property
      among the selected points.
   :Less:
      Select control points whose property is less than or equal to the greatest property
      among the selected points.
Threshold
   Tolerance value for also including control points whose property is close to the property
   of a selected point. For example, if a selected control point has a Weight of 0.6 and
   the *Threshold* is set to 0.1, *Select Similar* will include all control points that have
   a Weight between 0.5 and 0.7.

.. _bpy.ops.curve.select_row:

Select Control Point Row
========================

Selects the control points that are on the same grid line as the active one (similar to
edge loop selection <bpy.ops.mesh.loop_multi_select> for meshes).
Invoke the operator again to use the grid line in the other direction.
