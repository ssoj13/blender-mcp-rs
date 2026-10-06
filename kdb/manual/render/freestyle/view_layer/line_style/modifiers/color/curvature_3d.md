---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.LineStyle*Modifier_Curvature_3D:
.. Editor's Note: This page gets copied into:

.. --- copy below this line ---

************
Curvature 3D
************

A modifier based on radial curvatures of the underlying 3D surface.
The `curvature <https://en.wikipedia.org/wiki/Curvature>`__ of a 2D curve
at a point is a measure of how quickly the curve turns at the point.
The quicker the turn is, the larger the curvature is at the point.
The curvature is zero if the curve is a straight line.
Radial curvatures are those computed for a 2D curve that appears at the cross section
between the 3D surface and a plane defined by the view point (camera location)
and the normal direction of the surface at the point.

For radial curvatures to be calculated (and therefore for this modifier to have any effect),
the *Face Smoothness* option has to be turned on and the object needs to have *Smooth Shading*.

Curvature Min, Max
   The limits of the mapping.
   If the current point of the stroke is at *Min Curvature* or less from the target,
   it will take the start point of the mapping. And conversely,
   if it is at *Max Curvature* or more from the target, it will take the end-point value of the mapping.
