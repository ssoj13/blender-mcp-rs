---
area: manual-modeling
source: blender-manual
version: 5.2
---

*****
Curve
*****

This page covers the basics of curve editing.

.. _modeling-curves-transform:

Transform
=========

A Bézier curve can be edited by transforming the locations of both control points and handles.
NURBS curve on the other hand has only control points.

Move, Rotate, Scale
   Like other elements in Blender, curve control points and handles can be
   moved, rotated, or scaled as described in
   Basic Transformations </scene_layout/object/editing/transform/introduction>.
To Sphere, Shear, Bend, Push/Pull, Warp, Randomize
   The transform tools are described in
   the Transformations </modeling/meshes/editing/mesh/transform/index> sections.

.. _modeling-curve-radius:

Radius
------

Controls the width of the extrusion along the "spinal" curve.
The radius will be interpolated from point to point (you can check it with the normals).
The *Radius* of the points is set using the *Radius* transform tool. Or in the Sidebar *Transform* panel.

Mirror
======

The *Mirror* tool is also available, behaving exactly as with
mesh vertices </modeling/meshes/editing/mesh/mirror>.

Snap
====

Mesh snapping </editors/3dview/controls/snapping> also works with curve components.
Both control points and their handles will be affected by snapping,
except for within itself (other components of the active curve).
Snapping works with 2D curves but points will be constrained to the local XY axes.

Spin
====

The *Spin* operator only works for one dimensional surface </modeling/surfaces/index> objects.
Its use for curves is currently not possible,
the full feature is documented in Surface editing <bpy.ops.curve.spin>.

.. _bpy.ops.curve.duplicate_move:

Add Duplicate
=============

Duplicates the selected control points, along with the curve segments implicitly selected (if any).
If only a handle is selected, the full point will be duplicated too.
The copy is selected so you can move it to another place.

.. _bpy.ops.curve.split:

Split
=====

The *Split* operator separates the selected portion of a curve from the rest,
creating a new, independent curve segment.
This curve can then be moved or altered without affecting the other curve.

- If a segment of the curve is selected, it will be split off
  as a new curve that can be moved or edited independently.
- If only a single control point is selected, it will be duplicated as a loose control point,
  while the original remains attached to the rest of the curve.

.. _bpy.ops.curve.separate:

Separate
========

Separates curve objects that are made of multiple distinct curves into their own objects.

Note, if there is only one curve in a Curve object,
This operation will create a new Curve object with no control points.

.. _bpy.ops.curve.cyclic_toggle:

Toggle Cyclic
=============

Toggles between an open curve and closed curve (Cyclic).
Only curves with at least one selected control point will be closed/open.
The shape of the closing segment is based on the start and end handles for Bézier curves,
and as usual on adjacent control points for NURBS.
The only time a handle is adjusted after closing is if the handle is an *Auto* one.
Fig. fig-curve-editing-open-close is the same Bézier curve open and closed.

This action only works on the original starting control point or the last control point added.
Deleting a segment(s) does not change how the action applies;
it still operates only on the starting and last control points. This means that
Alt-C may actually join two curves instead of closing a single curve!
Remember that when a 2D curve is closed, it creates a renderable flat face.

.. _fig-curve-editing-open-close:

.. _bpy.ops.curve.spline_type_set:

Set Spline Type
===============

Converts splines in a curve object between Bézier, NURBS, Poly, and Catmull Rom types.
This is a basic conversion, meaning Blender does not preserve the exact shape or the number of control points.
For example, converting a NURBS spline to a Bézier spline maps each group of three NURBS control points
to a single Bézier control point with two handles.

Type
   Specifies the target spline type. For more details on spline types, see the
   Spline Types <curve-spline-types> documentation.

   :Bézier:
      Converts the spline to a Bézier type.
      - Poly splines are converted with vector handles.
      - NURBS or Catmull Rom splines are converted with automatic handles.

**Note:**

         When converting a NURBS spline to Bézier, at least six points are required.
         If the number of points is not a multiple of three, the spline will be truncated.

   :NURBS: Converts the spline to a NURBS type.
   :Poly: Converts the spline to a poly type.

Handles
   Includes handle information during the conversion process.

.. _bpy.ops.curve.reveal:
.. _bpy.ops.curve.hide:
.. _curves-show-hide:

Show/Hide
=========

When in *Edit Mode*, you can hide and reveal elements from the display.
You can only show or hide control points, as segments are always shown,
unless all control points of the connected curve are hidden,
in which case the curve is fully hidden.

See object-show-hide in *Object Mode*.
See also the /modeling/curves/curve_display panel.

Clean Up
========

.. _bpy.ops.curve.decimate:

Decimate Curve
--------------

The *Decimate Curve* operator reduces the number of control points
while trying to maintain the curves original shape.
This operator works similar to its mesh counterpart <bpy.ops.mesh.decimate>.

Ratio
   The percentage of control points to remove.

**Note:**

   This tool can only decimate Bézier curves.

.. _bpy.ops.curve.delete:

Delete
======

Options for the *Delete* pop-up menu:

Vertices
   This will delete the selected control points, *without* breaking the curve
   (i.e. the adjacent points will be directly linked, joined, once the intermediary ones are deleted).
   Remember that NURBS order cannot be higher than its number of control points,
   so it might decrease when you delete some control point.
   Of course, when only one point remains, there is no more visible curve,
   and when all points are deleted, the curve itself is deleted.
Segment
   Deletes the segment that connects the selected control points and disconnecting them.

.. _bpy.ops.curve.dissolve_verts:

Dissolve Vertices
-----------------

Deletes the selected control points, while the remaining segment
is fitted to the deleted curve by adjusting its handles.

.. list-table::

   * - .. figure:: /images/modeling_curves_editing_curve_make-segment.png

          Before deleting.

     - .. figure:: /images/modeling_curves_editing_curve_delete-vertices.png

          Deleting vertices.

   * - .. figure:: /images/modeling_curves_editing_curve_delete-segment.png

          Deleting segment.

     - .. figure:: /images/modeling_curves_editing_curve_dissolve-vertices.png

          Dissolve vertices.
