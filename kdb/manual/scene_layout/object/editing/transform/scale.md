---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

.. _bpy.ops.transform.resize:

*****
Scale
*****

Scaling means changing proportions of objects. Pressing S will enter
the *Scale* transformation mode where the selected element is scaled inward or
outward according to the mouse pointer's location. The element's scale will
increase as the mouse pointer is moved away from the Pivot Point and decrease as
the pointer is moved towards it. If the mouse pointer crosses from the original side of
the Pivot Point </editors/3dview/controls/pivot_point/index>
to the opposite side, the scale will continue in the negative direction and flip the element.

The amount of scaling will be displayed in the header of the 3D Viewport.

**Seealso:**

   Using a combination of shortcuts gives you more control over your transformation.
   See Transform Control </scene_layout/object/editing/transform/control/index>.

Options
=======

Scale X, Y, Z
   The amount to resize the selection on the respective axis.

Orientation
   Aligns the transformation axes to a specified orientation constraint.
   See Transform Orientations </editors/3dview/controls/orientation> for more information.

Proportional Editing
   The extruded face will affect nearby geometry.
   See Proportional Editing </editors/3dview/controls/proportional_editing> for a full reference.
