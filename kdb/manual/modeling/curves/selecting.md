---
area: manual-modeling
source: blender-manual
version: 5.2
---

************************
Selecting Curve Elements
************************

This page discusses specific selecting tools for curve objects in Edit Mode.
The Curve Edit more also uses the general select tools used which are described
in the interface section </interface/selecting>.

Curve selection in *Edit Mode* has fewer options than with meshes.
Mainly this is, because there is only one selectable element type, the control points
(no select mode needed here...). These points are a bit more complex than simple vertices,
however, especially for Bézier curves, as there is the central vertex, and its two handles...

The basic tools are the same as with meshes </modeling/meshes/selecting/index>,
so you can select a simple control point with the LMB,
add to current selection with Shift-LMB, Box Select B, and so on.

One word about the Bézier control points: when you select the main central vertex,
the two handles are automatically selected too, so you can move it as a whole,
without creating an angle in the curve. However, when you select a handle,
only this vertex is selected, allowing you to modify this control vector...

Note that, unlike mesh edges, you cannot directly select a segment. Instead,
select all of the control points that make up the segment you want to edit.

Select Menu
===========

With curves, all "advanced" selection options are grouped
in the *Select* menu of the 3D Viewport header.

.. _bpy.ops.curve.select_all:

All
===

Select all selectable elements.

None
====

Deselect all elements, but the active element stays the same.

Invert
======

Selects all the geometry that are not selected, and deselect currently selected components.

Box Select
==========

Interactive box selection <tool-select-box>.

Circle Select
=============

Interactive circle selection <tool-select-circle>.

Lasso Select
============

See tool-select-lasso.

.. _bpy.ops.curve.select_random:

Select Random
=============

Select Random control points.

Percent
   Selects the defined percentage of control points.
Random Seed
   Seed used by the pseudo-random number generator.
Action
   Controls whether the operator *Selects* or *Deselects* control points.

.. _bpy.ops.curve.select_nth:

Checker Deselect
================

This tool applies an alternating selected/deselected checker pattern.
This only works if you already have more than one control point selected.

It works by changing the current selection so that only every Nth
control points will remain selected, starting from the active one.

Deselected
   The number of deselected elements in each pattern repetition.
Selected
   The number of selected elements in each pattern repetition.
Offset
   Offset from the starting point.

.. _bpy.ops.curve.select_more:
.. _bpy.ops.curve.select_less:

Select More/Less
================

Their purpose, based on the currently selected control points, is to reduce or enlarge this selection.

More
   For each selected control point, select *all* its linked points (i.e. one or two...).
Less
   For each selected control point, if *all* points linked to this point are selected, keep this one selected.
   Otherwise, deselect it.

This implies two points:

#. When *all* control points of a curve are selected, nothing will happen
   (as for *Less*, all linked points are always selected, and of course, *More* cannot add any).
   Conversely, the same goes when no control points are selected.
#. Second, these tools will never "go outside" of a curve
   (they will never "jump" to another curve in the same object).

.. _bpy.ops.curve.select_linked:

Select Linked
=============

Selects all control points connected to the active control point.

This operator makes it easier to select or deselect entire segments of a curve, especially in
dense or complex curves, where manually selecting each point would be time-consuming.

.. _bpy.ops.curve.select_linked_pick:

Select Linked Pick
------------------

Selects (L) or deselects (Shift-L) control points
connected to the control point nearest the mouse cursor.

**Note:**

   For Bézier curves with a handle selected,
   this selection operator will select the whole control point **and** all the linked ones.

.. _bpy.ops.curve.select_similar:

Select Similar
==============

Selects control points that have certain similar properties to the active one.
The bpy.ops.screen.redo_last panel provides several selection options:

Type
   Type
      Selects splines that have the same spline Type i.e. Bézier, NURBS or Poly.
   Radius
      Selects control points that have a similar Radius value.
   Weight
      Selects all points that have a similar Weight value.
   Direction
      Selects control points that have a similar handles direction.

Compare
   For quantitative properties, this property selects the type of comparison to between the two numerical values.

   :Equal: Select items with the same value as the active item's chosen property.
   :Greater: Select items with a larger value as the active item's chosen property.
   :Less: Select items with a smaller value as the active item's chosen property.
Threshold
   For quantitative properties, this property controls how
   close the property's values have to be in the comparison.

.. _bpy.ops.curve.de_select_first:
.. _bpy.ops.curve.de_select_last:

(De)select First/Last
=====================

These operators will toggle the selection of the first or last control point(s) of the curve(s)
in the object. This is useful to quickly find the start of a curve
(e.g. when using it as path...).

.. _bpy.ops.curve.select_next:
.. _bpy.ops.curve.select_previous:

Select Next/Previous
====================

These operators will select the next or previous control point(s),
based on the current selection
(i.e. the control points following or preceding the selected ones along the curve).
In case of a cyclic curve, the first and last points are not considered as neighbors.

Pick Shortest Path
==================

Selects the curve segments between two control points: the active and the one under the cursor.
In the case of a closed curve, the shortest path will be selected.
