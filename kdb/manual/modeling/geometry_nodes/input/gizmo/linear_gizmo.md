---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeGizmoLinear:

************
Linear Gizmo
************

The *Linear Gizmo* node provides the most widely applicable gizmo. It can e.g. be used to control the height of
something.

Inputs
======

Value
   Special gizmo value socket.
   Everything that linked into this socket will be modified when the gizmo is moved.
Position
   Position of the gizmo in the local space of the object.
Direction
   Specifies the direction in with the gizmo points or is moved.

Properties
==========

Color
   Controls which theme color is used for this gizmo.
Draw Style
   Allows choosing between different styles of the gizmo.

Outputs
=======

Transform
   Should be joined into the geometry that is controlled by this gizmo.
