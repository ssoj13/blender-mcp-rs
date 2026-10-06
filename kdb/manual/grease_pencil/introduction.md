---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

************
Introduction
************

Grease Pencil is a Blender object. It accepts the drawing information
from a mouse or pressure-sensitive stylus and places it in 3D space
as a collection of points, which are defined as a stroke.

The Grease Pencil object can be used to make traditional 2D animation, cut-out animation,
motion graphics, or use it as a storyboard tool, among other things.

Strokes are created in Draw Mode </grease_pencil/modes/draw/introduction>,
which requires a new keyframe <../animation/keyframes/editing>
in the animation timeline for the Grease Pencil object.
Existing strokes can then be adjusted in Edit Mode </grease_pencil/modes/edit/introduction>
and Sculpt Mode </grease_pencil/modes/sculpting/introduction>.
Finally, artists can apply materials, modifiers, lighting, and visual effects to strokes.

Quick Start
===========

Artists can add Grease Pencil to any existing Blender scene, or start with a 2D Animation template.
The template offers some pre-configured options that are helpful for animation and storyboarding.

Create and Use Grease Pencil
----------------------------

#. From Object Mode <../scene_layout/object/introduction>, Add --> Grease Pencil --> Blank.
#. Create a new keyframe or turn on Auto Key. (See Keyframe Editing <../animation/keyframes/editing>)
#. Switch to Draw Mode <./modes/draw/introduction>.
#. Click and drag across the viewport to add strokes to the Grease Pencil object.

2D Animation Template
---------------------

To create a new Blender file using the "2D Animation"
project template use: File --> New --> 2D Animation.

Note the following pre-configured setup for the 2D Animation template:

- 2D Animation is the default active workspace.
- World Properties --> Surface (Background) --> Color is set to white.
- Color management bpy.types.ColorManagedViewSettings.view_transform is set to *Standard*.
- The drawing plane <./modes/draw/drawing_planes> is set to Front (X-Z).
- Line and Fill layers, along with some stroke materials, are configured for Grease Pencil.
- The animation timeline will automatically create a new keyframe when Grease Pencil is used on empty frames.

**Tip:**

   Grease Pencil can read pressure-sensitivity information from a
   Graphics Tablet <hardware-tablet> or stylus.
