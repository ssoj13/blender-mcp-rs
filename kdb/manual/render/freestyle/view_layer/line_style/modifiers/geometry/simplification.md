---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.LineStyleGeometryModifier_Simplification:

**************
Simplification
**************

The *Simplification* modifier merges stroke vertices that lie close to one another,
like the *Decimate* modifier for meshes.

Tolerance
   Measure for how close points have to be to each other to be merged.
   A higher tolerance means more vertices are merged.
