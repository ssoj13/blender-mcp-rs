---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.ops.curves.add:

****************
Curve Primitives
****************

.. _bpy.ops.object.curves_empty_hair_add:

Empty Hair
==========

Adds an empty high-performance curves object and automatically:

- Assigns the active object as the Surface </sculpt_paint/curves_sculpting/introduction>.
- Set the surface object as the parent of the new object.
- Adds a Geometry Nodes modifier to deform the curves on the surface.

Fur
===

Adds a fur setup to the selected objects.
The fur setup is based on /modeling/geometry_nodes/index and built with
Hair Node Groups </modeling/geometry_nodes/hair/index> that come with Blender as bundled assets.

See bpy.ops.object.quick_fur for more information.
