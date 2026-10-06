---
area: manual-files
source: blender-manual
version: 5.2
---

**********************************
Import/Export SVG as Grease Pencil
**********************************

The Scalable Vector Graphics (SVG) format is used for interchanging vector-based illustrations between applications
and is supported by vector graphics editors such as Inkscape and modern browsers, among others.

**Warning:**

.. _bpy.ops.wm.grease_pencil_import_svg:

Import
======

Resolution
   Resolution for generated strokes.

Scale
   Generated strokes scale.

Scene Unit
   Apply current scene's unit (as defined by unit scale) to imported data.

Center on Bounds
   Center the imported SVG on its bounding box.

.. _bpy.ops.wm.grease_pencil_export_svg:

Export
======

Scene Options
-------------

Object
   Determine which objects will be included in the export.

   :Active: Export only the active Grease Pencil object.
   :Selected: Export all selected Grease Pencil objects.
   :Visible: Export all visible Grease Pencil object in the scene.

Export Options
--------------

Frame
   Determine which frames will be included in the export.

   :Active: Export only the active keyframe.
   :Selected: Export all selected keyframes as SVG animation.
   :Scene: Export all frames as SVG animation.

Sampling
   Precision for the stroke sampling. Low values mean a more accurate result.

Fill
   When enabled, export the Grease Pencil strokes fill.

Uniform Width
   When enabled, export strokes with constant thickness.

Clip Camera
   When enabled and camera view is active, export only the strokes clipped from camera view.

**Note:**
    workspace.
