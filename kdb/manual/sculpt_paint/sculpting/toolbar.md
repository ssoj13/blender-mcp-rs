---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

*******
Toolbar
*******

The amount of tools in sculpt mode is very extensive.
This is an overview of all of them, categorized by their general functions.

Sculpting Tools
===============

/sculpt_paint/sculpting/tools/brush_tool
   Tool to use for any of the *Sculpt* mode brushes.

Add Object Tools
================

General interactive tools to contruct primitive geometry directly in Sculpt Mode.

**Warning:**

   It's worth noting these tools do not work when Dynatopo is enabled or in the presence of an active Multires modifier.

Add Cube <tool-add-cube>
   Interactively add a cube mesh object.

   Add Cone
      Interactively add a cone mesh object.
   Add Cylinder
      Interactively add a cylinder mesh object.
   Add UV Sphere
      Interactively add a UV sphere mesh object.
   Add Icosphere
      Interactively add an icosphere mesh object.

Gesture Tools
=============

General gesture tools to apply an operation via box, lasso, line and polyline shapes.
See /sculpt_paint/sculpting/introduction/gesture_tools for more information.

/sculpt_paint/sculpting/tools/mask_tools
   Create a mask via a gesture.

/sculpt_paint/sculpting/tools/hide_tools
   Hides/Shows geometry via a gesture.

/sculpt_paint/sculpting/tools/face_set_tools
   Create a face set via a gesture.

/sculpt_paint/sculpting/tools/trim_tools
   Perform a Boolean operation via a gesture.

/sculpt_paint/sculpting/tools/line_project
   Flatten the geometry towards a drawn line.

Filter Tools
============

Tools for applying effects on the entire unmasked and visible mesh.

/sculpt_paint/sculpting/tools/mesh_filter
   Apply a deformation to all unmasked vertices.

/sculpt_paint/sculpting/tools/cloth_filter
   Applies a cloth simulation to all unmasked vertices.

/sculpt_paint/sculpting/tools/color_filter
   Changes the active color attribute on all unmasked vertices.

Single Click Tools
==================

Simpler tools that apply an operation on surfaces that are clicked on.

/sculpt_paint/sculpting/tools/edit_face_set
   Modifies the face set under the cursor.

/sculpt_paint/sculpting/tools/mask_by_color
   Create a mask from any color from the color attribute by clicking on it.

General Tools
=============

General transform and annotate tools like in other modes.

Move </sculpt_paint/sculpting/tools/transforms>
   Translation tool.

Rotate </sculpt_paint/sculpting/tools/transforms>
   Rotation tool.

Scale </sculpt_paint/sculpting/tools/transforms>
   Scale tool.

Transform </sculpt_paint/sculpting/tools/transforms>
   Adjust the objects translation, rotations and scale.

Annotate <tool-annotate-freehand>
   Draw free-hand annotation.

   Annotate Line
      Draw straight line annotation.
   Annotate Polygon
      Draw a polygon annotation.
   Annotate Eraser
      Erase previous drawn annotations.
