---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeGizmoTransform:

***************
Transform Gizmo
***************

The *Transform Gizmo* node provides a compound gizmo that can control a position, rotation and scale.

Inputs
======

Value
   Special gizmo value socket.
   Everything that linked into this socket will be modified when the gizmo is modified.
Position
   Position of the gizmo in the local space of the object.
Rotation
   Local orientation of the gizmo.

**Note:**

   The rotation input is ignored by the 3D viewport if the transform orientation is set to global.

Properties
==========

The node has properties in the sidebar which allow disabling parts of the gizmo.
This can be useful when e.g. controlling only a translation or only a rotation.

Outputs
=======

Transform
   Should be joined into the geometry that is controlled by this gizmo.
