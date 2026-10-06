---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

.. _object-convert-to:
.. _bpy.ops.object.convert:

*******
Convert
*******

Curve
=====

Converts the selected mesh or text object into a curve object.

- **For mesh objects**: Only loose edges (edges not part of any faces) will be included in the conversion.
- **For text objects**: The text is converted into curve outlines, preserving its shape.

The resulting curve will be Poly Curve <curve-poly> by default.
To create smooth segments, convert the curve to a Bézier Curve <curve-bezier>
using bpy.ops.curve.spline_type_set.

Mesh
====

Converts the selected curve, metaball, surface, or text object to a mesh object.
The actual defined resolution of these objects will be taken into account for the conversion.
Note that it also keeps the faces and volumes created by closed and extruded curves.

Grease Pencil
=============

Converts the selected curve, mesh, or text object to a Grease Pencil object, generating strokes
that follow the original shape. Basic materials are also created.
When multiple objects are selected, they are combined into a single Grease Pencil object.

Keep Original
   Keeps the original object by creating a duplicate before conversion.
Thickness
   Defines the stroke thickness.
Stroke Offset
   Adjusts the offset to separate strokes from filled areas.
Export Faces
   Converts mesh faces into filled strokes.

Trace Image to Grease Pencil
============================

See /grease_pencil/modes/object/trace_image.

.. _bpy.ops.image.convert_to_mesh_plane:

Convert to Mesh Plane
=====================

Converts the selected image empty <bpy.types.Object.empty_image> to a textured mesh plane.

For a description of the options see /modeling/meshes/import_images_as_planes.
