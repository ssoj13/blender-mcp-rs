---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _dope-sheet-shape-key:

****************
Shape Key Editor
****************

The *Shape Key Editor* displays the shape keys </animation/shape_keys/introduction> of the
active object and allows you to create and adjust keyframes for their
influence values <bpy.types.ShapeKey.value>.
This editor is a specialized mode of the Dope Sheet </editors/dope_sheet/introduction>,
focused specifically on shape key animation.

Each row represents a shape key, and each keyframe marker corresponds to a point where the
shape key's value changes over time.
This provides an efficient way to time and refine facial expressions, morph targets,
and other mesh deformations driven by shape keys.

Additional shape key properties, such as names, values, and relative settings,
can be viewed and edited in the *Sidebar*.

Usage
=====

- Use the *Shape Key Editor* to keyframe shape key influences over time.
- Adjust timing by moving or scaling keyframes directly in the timeline.
- Combine with other Dope Sheet modes (such as the *Action Editor*) for complex character animation workflows.

See also: Shape Key Basics </animation/shape_keys/introduction> for details on creating
and managing shape keys.
