---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeScaleElements:

*******************
Scale Elements Node
*******************

Scales the selected faces or edges, letting you specify a scaling factor and pivot point for each one.
Connected faces/edges are scaled together using their average factor and pivot point.

Inputs
======

Geometry
   Standard geometry input.

Selection
   Boolean field indicating which elements to scale.

Scale
   The scaling factor for each element.

Center
   The pivot point for each element.

Axis Single Axis Mode Only
   Axis along which to scale each element. This vector is normalized internally, so the length does not matter.

Scale Mode
   :Uniform: Scale elements by the same factor in every direction.
   :Single Axis: Scale elements in a single direction defined by the *Axis* input.

Properties
==========

Domain
   The element type to transform.

   :Face: Scale faces.
   :Edge: Scale edges.

Output
======

Geometry
   Standard geometry output.

Examples
========

The node is useful when combined with the /modeling/geometry_nodes/mesh/operations/extrude_mesh,
especially in *Individual* mode where connected faces aren't extruded together.
