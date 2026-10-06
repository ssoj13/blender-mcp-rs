---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

.. _tool-scale-cage:

**********
Scale Cage
**********

The *Scale Cage* tool is a bounding box around the object(s) which scales objects from a particular point or axis.
The tool works by selecting a scale point and dragging inwards or outwards to adjust the scale accordingly.
The origin for the scale will be from the point on the cube directly opposite from the point selected.
Selecting points on the faces of the cube scales along one axis,
selecting points on the edges of the cube scales along two axes,
and selecting points on the vertices of the cube scales along all three axes.

Tool Settings
=============

Orientation
   Aligns the transformation axes to a specified orientation constraint.
   See Transform Orientations </editors/3dview/controls/orientation> for more information.

Options
=======

Scale X, Y, Z
   The amount to resize the selection on their respective axis.

Orientation
   Aligns the transformation axes to a specified orientation constraint.
   See Transform Orientations </editors/3dview/controls/orientation> for more information.

Proportional Editing
   The extruded face will affect nearby geometry.
   See Proportional Editing </editors/3dview/controls/proportional_editing> for a full reference.
