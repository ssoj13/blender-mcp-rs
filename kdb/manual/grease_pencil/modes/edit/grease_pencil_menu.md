---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

******************
Grease Pencil Menu
******************

Transform
=========

Strokes can be edited by transforming the locations of points.

Move, Rotate & Scale
--------------------

Like other elements in Blender, points and strokes can be
moved G, rotated R or scaled S as described in
the Basic Transformations </scene_layout/object/editing/transform/introduction> section.
When in *Edit Mode*,
Proportional Editing </editors/3dview/controls/proportional_editing>
is also available for the transformation actions.

Transform Snapping
------------------

Basic move, rotate and scale transformations for selected points/strokes.
See Move, Rotate, Scale Basics </modeling/meshes/editing/mesh/transform/basic> for more information.

Tools
-----

The *Bend*, *Shear*, *To Sphere*, *Extrude* and *Shrink Fatten* transform tools are described
in the Editing tools </grease_pencil/modes/edit/tools/index> section.

Mirror
======

The *Mirror* tool is also available, behaving exactly the same as with
mesh vertices </modeling/meshes/editing/mesh/mirror>.

.. _bpy.ops.grease_pencil.snap_to_grid:
.. _bpy.ops.grease_pencil.snap_to_cursor:
.. _bpy.ops.grease_pencil.snap_cursor_to_selected:

Snap
====

Mesh snapping </editors/3dview/controls/snapping>
also works with Grease Pencil components.

Active Layer
============

Select the active layer.

Animation
=========

The stroke animation operations are described in the Animation </grease_pencil/animation/tools> section.

Interpolate Sequence
====================

See bpy.ops.grease_pencil.interpolate_sequence.

.. _bpy.ops.grease_pencil.duplicate_move:

Duplicate
=========

Duplicates the selected elements, without creating any connections
with the rest of the strokes (unlike *Extrude*, for example),
and places the duplicate at the location of the original elements.

.. _bpy.ops.grease_pencil.stroke_split:

Split
=====

The *Split* operator separates the selected portion of a curve from the rest,
creating a new, independent curve segment.
This curve can then be moved or altered without affecting the other curve.

- If a segment of the curve is selected, it will be split off
  as a new curve that can be moved or edited independently.
- If only a single control point is selected, it will be duplicated as a loose control point,
  while the original remains attached to the rest of the curve.

.. _bpy.ops.grease_pencil.copy:

Copy
====

Copy the selected points/strokes to the clipboard.

.. _bpy.ops.grease_pencil.paste:

Paste
=====

Paste Grease Pencil points or strokes from the internal clipboard to the active layer.

Paste on Back Shift-Ctrl-V
    Add pasted strokes behind all strokes.
Keep World Transform
   Keep the world transform of strokes from the clipboard unchanged.

Show/Hide
=========

Contains operators to adjust the visibility of points and strokes in the viewport.

Show All Layers
---------------

Shows all Grease Pencil layers </grease_pencil/properties/layers>.

.. _bpy.ops.grease_pencil.layer_hide:

Hide Active Layer
-----------------

Hides the active Grease Pencil layers </grease_pencil/properties/layers>.

Hide Inactive Layers
--------------------

Hides all Grease Pencil layers </grease_pencil/properties/layers> except the active layer.

.. _bpy.ops.grease_pencil.separate:

Separate
========

Separate different elements into new Grease Pencil objects based on specific criteria.

Selection
   Separates the selected points or strokes into a new object.
By Material
   Separates the geometry by creating a new object for each material.
By Layer
   Separates the geometry by creating a new object for each layer.
   See 2D Layers </grease_pencil/properties/layers> for more information.

Clean Up
========

These tools help to cleanup degenerate geometry on the strokes.

.. _bpy.ops.grease_pencil.clean_loose:

Clean Loose Points
------------------

Removes strokes with only a few points.

Limit
   The number of points to consider a stroke as loose.

.. _bpy.ops.grease_pencil.frame_clean_duplicate:

Delete Duplicate Frames
-----------------------

Removes any duplicate keyframes.

.. _bpy.ops.grease_pencil.stroke_merge_by_distance:

Merge by Distance
-----------------

Simplifies a stroke by merging the selected points that are closer than a specified distance to each other.
Note, unless using *Unselected*, selected points must be contiguous, else they will not be merged.

Merge Distance
   Sets the distance threshold for merging points.
Unselected
   Allows points in selection to be merged with unselected points.
   When disabled, selected points will only be merged with other selected ones.

.. _bpy.ops.grease_pencil.reproject:

Reproject Strokes
-----------------

Sometimes you may have drawn strokes unintentionally in different locations in the 3D space
but they look right from a certain plane or from the camera view.
You can use Reproject to flatten all the selected strokes from a certain viewpoint.

Reprojected Type
   :Front: Reproject selected strokes onto the front plane (XZ).
   :Side: Reproject selected strokes onto the side plane (YZ).
   :Top: Reproject selected strokes onto the top plane (XY).
   :View: Reproject selected strokes onto the current view.
   :Surface: Reproject selected strokes onto the mesh surfaces.

      Surface Offset
         When Surface Mode is activated controls the stroke offset from the object.
   :Cursor: Reproject selected strokes onto 3D cursor rotation.

Keep Original
   Maintains the original strokes after applying the tool.

.. list-table::

   * - .. figure:: /images/grease-pencil_modes_edit_grease-pencil-menu_reproject-strokes-1.png
          :width: 200px

          Original drawing from the front view.

     - .. figure:: /images/grease-pencil_modes_edit_grease-pencil-menu_reproject-strokes-2.png
          :width: 200px

          Original drawing in the 3D Viewport.

     - .. figure:: /images/grease-pencil_modes_edit_grease-pencil-menu_reproject-strokes-3.png
          :width: 200px

          Strokes reprojected onto the front plane to fix strokes misalignment.

     - .. figure:: /images/grease-pencil_modes_edit_grease-pencil-menu_reproject-strokes-1.png
          :width: 200px

          Drawing after reprojection operation from the front view.

.. _bpy.ops.grease_pencil.outline:

Outline
=======

The *Outline* operator converts selected Grease Pencil strokes into closed perimeter shapes.
It creates new strokes around the outer boundary of the original stroke,
effectively generating a filled outline with adjustable thickness.

View
   Defines the projection method used to generate the outline:

   :View: Use the current viewport perspective as the projection plane.
   :Front: Use the X-Z axes as the projection plane (front view).
   :Side: Use the Y-Z axes as the projection plane (side view).
   :Top: Use the X-Y axes as the projection plane (top view).
   :Camera: Use the active camera's perspective as the projection plane.
Radius
   Sets the thickness of the outline on both sides of the original stroke.
   Higher values result in a wider perimeter.
Offset Factor
   Scales the stroke outline inward or outward.
   - Positive values push the perimeter outward.
   - Negative values pull it inward.
   - A value of 0 centers the perimeter on the original stroke.
Corner Subdivisions
   Number of subdivisions used to smooth corners at stroke endpoints and joints.
   Higher values result in smoother corners but increase stroke complexity.

.. list-table::

   * - .. figure:: /images/grease-pencil_modes_edit_stroke-menu_outline-1.png
          :width: 320px

          Original stroke.

     - .. figure:: /images/grease-pencil_modes_edit_stroke-menu_outline-2.png
          :width: 320px

          Stroke after applying the *Outline* operator.

Delete
======

Opens a pop-up menu with operators to remove geometry from the Grease Pencil object.

.. _bpy.ops.grease_pencil.delete:

Strokes
-------

Deletes the selected points.

If deleting points leaves only a single point in a stroke,
the stroke will no longer be visible.
If all points of a stroke are removed, the entire stroke is deleted.

Only Strokes
------------

Deletes only the stroke (outline) component of the selected curves,
preserving any associated fill geometry.

Only Fills
----------

Deletes only the fill component of the selected curves,
leaving the stroke (outline) intact.

.. _bpy.ops.grease_pencil.dissolve:

Dissolve
--------

Dissolving removes points between other points and connect the remaining points.

Ctrl-X Opens a pop-up to choose the dissolve type.

Dissolve
   Deletes the selected points without splitting the stroke.
   The remaining points in the strokes stay connected.
Dissolve Between
   Deletes all the points between the selected points without splitting the stroke.
   The remaining points in the strokes stay connected.
Dissolve Unselect
   Deletes all the points that are not selected in the stroke without splitting the stroke.
   The remaining points in the strokes stay connected.

Delete Active Keyframe (Active Layer)
-------------------------------------

Deletes all the strokes at the current frame in the active layer.

Delete Active Keyframes (All Layers)
------------------------------------

Deletes all the strokes at the current frame in all layer.
