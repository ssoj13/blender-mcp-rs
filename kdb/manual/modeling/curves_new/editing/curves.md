---
area: manual-modeling
source: blender-manual
version: 5.2
---

******
Curves
******

Transform
=========

A curves object can be edited by transforming the locations of control points.

Move, Rotate, Scale
   Like other elements in Blender, control points can be moved, rotated, or scaled as described in
   Basic Transformations </scene_layout/object/editing/transform/introduction>.
To Sphere, Shear, Bend, Push/Pull
   The transform tools are described in
   the Transformations </modeling/meshes/editing/mesh/transform/index> sections.

.. _modeling-curves-radius:

Radius
------

The Radius allows you to directly control the width of the extrusion along the "spinal" curve.
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

.. _bpy.ops.curves.duplicate_move:

Duplicate
=========

This operator duplicates the selected control points,
along with the curve segments implicitly selected (if any).
.. If only a handle is selected, the full point will be duplicated too.
The copy is selected so you can move it to another place.

.. _bpy.ops.curves.extrude_move:

Extrude Curve and Move
======================

Extrudes points by duplicating the selected points, which then can be moved,
and connecting those points back to the original curve creating a continuous curve.

.. _bpy.ops.curves.attribute_set:

Set Attribute
=============

Opens a pop-up window showing the name of the active attribute
as well as the value of that attribute for the selected points
From there, you assign a new value to a selected attribute across all selected points.

This operator is useful for uniformly setting attribute values.

.. _bpy.ops.curves.curve_type_set:

Set Curve Type
==============

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
   :Catmull Rom: Converts the spline to a Catmull Rom type.

Handles
   Includes handle information during the conversion process.

Toggle Cyclic
=============

Toggles between an open curve and closed curve (Cyclic).
Only curves with at least one selected control point will be closed/open.
The shape of the closing segment is based on the start and end handles for Bézier curves,
and as usual on adjacent control points for NURBS.
The only time a handle is adjusted after closing is if the handle is an *Auto* one.
Fig. fig-curves-editing-open-close is the same Bézier curve open and closed.

This action only works on the original starting control point or the last control point added.
Deleting a segment(s) does not change how the action applies;
it still operates only on the starting and last control points. This means that
Alt-C may actually join two curves instead of closing a single curve!
Remember that when a 2D curve is closed, it creates a renderable flat face.

.. _fig-curves-editing-open-close:

.. _bpy.ops.curves.separate:

Separate
========

Separates the selected curve geometry into their own curve object.

.. _bpy.ops.curves.delete:

Delete
======

The Delete operator can remove Control Points or Segments.
Deleting can be used to make curves shorter or simplify
segments by deleting control points in the mid section of a segment.

.. _bpy.ops.curves.split:

Split
=====

The *Split* operator separates the selected portion of a curve from the rest,
creating a new, independent curve segment.
This curve can then be moved or altered without affecting the other curve.

- If a segment of the curve is selected, it will be split off
  as a new curve that can be moved or edited independently.
- If only a single control point is selected, it will be duplicated as a loose control point,
  while the original remains attached to the rest of the curve.
