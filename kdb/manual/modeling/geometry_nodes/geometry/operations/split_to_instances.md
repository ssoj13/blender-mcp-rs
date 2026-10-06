---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeSplitToInstances:

***********************
Split To Instances Node
***********************

Splits a selection of geometry elements (such as faces) into groups,
then turns each group into an instance </modeling/geometry_nodes/instances>.

Inputs
======

Geometry
   Standard geometry input.

Selection
   Boolean field indicating which geometry elements to include.

Group ID
   Integer field indicating which group each element belongs to. Elements with the
   same ID will be moved into the same output instance.

Properties
==========

Domain
   The type of geometry to extract and split.
   This is also the domain on which the *Selection* and *Group ID* fields are evaluated.

   :Point:
      Points, spline control points, and vertices.
   :Edge:
      Mesh edges.
   :Face:
      Mesh faces.
   :Spline:
      Curve splines.
   :Instance:
      Top-level instances. Realized instances are ignored.
   :Layer:
      Grease Pencil layers.

**Note:**

      Geometry that doesn't match the selected domain will be removed.
      For example, if you choose *Edge*, any faces, splines, and instances in the input
      geometry will be lost.

Output
======

Instances
   The instances containing the grouped geometry elements.

Group ID
   Group ID of each instance.

Examples
========

In the example above, we start with a grid of 1000x1000 square faces serving as "pixels."
Then, we group these faces into patches by assigning them a group ID sampled from a Voronoi texture,
and move each resulting instance by a random amount along the Z axis.

Note that, because the texture outputs floating point values between 0 and 1 while the group ID
is an integer, all the values would be rounded to 0 or 1 and we would only get two groups.
To get more variation, we multiply the texture value by 1000.
