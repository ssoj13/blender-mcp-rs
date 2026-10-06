---
area: manual-movie_clip
source: blender-manual
version: 5.2
---

*************
Editing Masks
*************

The tools and panels available to edit masks are the same in both editors.
Editing of mask splines happens in a way similar to editing Bézier curves or paths in GIMP or other curve editors.

**Tip:**

   To get interactive feedback on the resulting mask,
   a Mask node can be connected directly to a Viewer node in the Compositor,
   which will then keep updating the compositing result while editing.

Transform
=========

Move </scene_layout/object/editing/transform/move> G
   Change the location of control points. Control points can also be moved with LMB.
   The whole spline can be moved by dragging the center dot with LMB.
Rotate </scene_layout/object/editing/transform/rotate> R
   Change the location of control points by rotating about a pivot point.
Scale </scene_layout/object/editing/transform/scale> S
   Change the location of control points by expanding the distance between points.

To Sphere </modeling/meshes/editing/mesh/transform/to_sphere> Shift-Alt-S
   Morphs the control points to the shape of a circle.
Shear </modeling/meshes/editing/mesh/transform/shear> Shift-Ctrl-Alt-S
   Shifts control points along a defined axis so parallel control points move past one another.
Push/Pull </modeling/meshes/editing/mesh/transform/push_pull>
   Moves the control points closer together (Push) or further apart (Pull).

Scale Feather Alt-S
   Will scale the feather size.

.. _bpy.ops.mask.feather_weight_clear:

Clear Feather Weight
====================

Resets the feather weight to zero.

.. _bpy.ops.mask.cyclic_toggle:

Toggle Cyclic
=============

Toggle to create a closed curve or open it again.
Close the mask by joining the last control point to the first.

.. _bpy.ops.mask.handle_type_set:

Set Handle Type
===============

Set handle type for selected spline points.

.. _bpy.ops.mask.normals_make_consistent:

Recalculate Handles
===================

Make normals (handle directions) consistent.

.. _bpy.ops.mask.switch_direction:

Switch Direction
================

Switch Direction handle directions in/out.

.. _bpy.ops.mask.copy_splines:
.. _bpy.ops.mask.paste_splines:

Copy Paste
==========

.. _bpy.ops.mask.parent_clear:

Clear Parent
============

Clears any parenting relationship for the selected spline points.

.. _bpy.ops.mask.parent_set:

Make Parent
===========

Parents one or more selected spline points to the active motion tracker.

.. _bpy.ops.mask.shape_key_clear:
.. _bpy.ops.mask.shape_key_insert:

Animation
=========

Masks can be animated with the shape keying system.
This can be useful when there are not enough good feature points to track in the footage,
or the mask is not based on footage.
Mask animation timing can be edited from the *Dope Sheet's* Mask Mode <dope-sheet-mask>.

Insert Shape Key I
   Will insert a shape key for the active mask layer at the current frame.
   This works on the level of mask layers,
   so inserting a shape key will keyframe all the splines and points contained in it.
Clear Shape Key Alt-I
   Will clear the shape key for the active mask layer at the current frame.
Feather Reset Animation
   Resets the feather offset across all animated frames.
Re-Key Points of Selected Shapes
   Re-interpolate selected points on across the range of keys selected in the *Dope Sheet*.

.. _bpy.ops.mask.hide_view_clear:
.. _bpy.ops.mask.hide_view_set:

Show/Hide
=========

- Hide Selected H
- Hide Unselected Shift-H
- Clear Restricted View Alt-H

.. _bpy.ops.mask.delete:

Delete
======

Removes control points.

.. _bpy.ops.mask.slide_spline_curvature:
.. _bpy.ops.mask.add_vertex_slide:
.. _bpy.ops.mask.add_feather_vertex_slide:

Miscellaneous
=============

Slide Spline Curvature LMB
   Moves the curve and/or control points by clicking on them and dragging.

Add Vertex and Slide Ctrl-LMB
   Inserts new control points and defines handle orientations by a continued mouse drag.
   If the last point was selected, double-click will also close the curve.

Add Feather Vertex and Slide Shift-Ctrl-LMB
   Inserts new feather control points that can be transformed independently of the main spline curve.
   If no feather mask is in use this will create a basic feather mask to the curve.
