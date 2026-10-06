---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

************************
Mesh Sculpt Brush Assets
************************

This is a list of all provided 'Essentials' brush assets that come with Blender.
These are based on various Brush Types <sculpt-tool-settings-brush-type> which are mentioned for each brush..

Add/Subtract Brushes
====================

These brushes generally push vertices outwards and inwards and are the most customizable to achieve a wide variety of
effects. They typically don't use a color in their thumbnail.

Draw
   | Brush Type: draw
   | Shortcut:  V

   The standard brush for pushing vertices inwards and outwards from the surface direction.

Draw Sharp
   | Brush Type: draw_sharp
   | Shortcut:  Shift V

   Same as *Draw* but with a much sharper Falloff </sculpt_paint/brush/falloff>.
   Useful for creating creases and sharp angles.

Clay
   Brush Type: clay

   Similar to the *Draw* brush but with a flattening effect and subtle smoothing.
   Useful for polishing and building volumes.

Clay Strips
   | Brush Type: clay_strips
   | Shortcut:  C

   The same as the *Clay* brush, but more aggressive with a square falloff.
   A common standard for building rough volumes.

Clay Thumb
   | Brush Type: clay_thumb

   The same as the *Clay* brush, but specifically for emulating the effect of running your thumb over surfaces.
   Pushes geometry in and sideways.

Layer
   Brush Type: layer

   Draw with a fixed height. Useful for adding flat layers to a surface.

Inflate/Deflate
   | Brush Type: inflate
   | Shortcut:  I

   Moves the mesh in multiple direction. Useful for inflating or shrinking surfaces and volumes.

Blob
   Brush Type: blob

   Magnifies the mesh as you draw. Useful for an additional inflation effect on the stroke.

Crease Polish
   | Brush Type: crease
   | Shortcut:  Shift C

   A Draw brush with a pinching effect. Useful for polishing existing creases or carefully creating new ones.

Crease Sharp
   Brush Type: crease

   Much sharper and stronger Crease brush. Great for creating thin and deep pinches.

Contrast Brushes
================

Recognizable by their red thumbnail and cursor.
These brushes generally flatten or heighten the contrast of the surface.

Smooth
   | Brush Type: smooth
   | Shortcut:  S

   Smooths out irregularities in the surface and shrinks volumes by averaging the vertices positions.
   An essential brush that is frequently used.

Flatten/Contrast
   Brush Type: plane

   Pushes vertices to an average height to create a flat surfaces. Alternatively pushes them away from the center for
   more contrast.

Plateau
   Brush Type: plane

   Similar to Flatten but with a locked orientation and depth to create a consistently flat surface.

Fill/Deepen
   Brush Type: plane

   Pushes surfaces upwards towards a flat plane. Useful for filling in holes and crevices. Alternatively deepens
   existing holes when holding 'Ctrl'.

Scrape/Fill
   | Brush Type: plane
   | Shortcut:  Shift T

   Pushes surfaces inwards. Alternatively fills surfaces while holding 'Ctrl'. This is the most common brush for
   flattening meshes.

Trim
   Brush Type: plane

   Pushes surfaces inwards toward a locked direction. The depth can be defined by going deeper towards surfaces along
   the stroke.

Scrape Multiplane
   Brush Type: multiplane_scrape

   Scrapes the mesh with two angled planes at the same time, producing a sharp edge between them.

Transform Brushes
=================

Recognizable by their yellow icon and cursor.
These brushes generally move, pinch and magnify the mesh.

Pinch/Magnify
   | Brush Type: pinch
   | Shortcut:  P

   Pulls vertices towards the center of the brush. Useful for polishing angles and creases. Alternatively pushes them
   away from the center.

Grab
   | Brush Type: grab
   | Shortcut:  G

   Moves vertices along with the mouse. An essential brush for building shapes and adjusting proportions.

Grab 2D
   Brush Type: grab

   Similar to Grab but with an infinitely projected falloff. Useful for grabbing broader shapes and giving a similar
   feel to using Liquify tools in image painting applications.

Grab Silhouette
   Brush Type: grab

   Similar to Grab but only affects vertices with the normal facing sideways away from the view. Very useful for
   adjusting outer silhouettes of thin objects.

Elastic Grab
   Brush Type: elastic_deform

   Used to simulate realistic deformations from grabbing of Elastic objects.

Elastic Snake Hook
   Brush Type: snake_hook

   Similar to Elastic Grab but rotates affected geometry based on the stroke direction.

Scene Project
   | Brush Type: scene_project

   Projects vertices of the active object onto other visible meshes in the scene.

   This provides more flexible and localized control than the
   Shrinkwrap modifier </modeling/modifiers/deform/shrinkwrap>,
   making it useful for manually conforming geometry to a target surface.

Snake Hook
   | Brush Type: snake_hook
   | Shortcut:  K

   Pulls vertices along with the stroke to create long, snake-like forms. Geometry is rotated and magnified to allow
   continuous pulling. Much more useful while having Dyntopo <dyntopo_introduction> enabled.

Pull
   Brush Type: snake_hook

   Iteratively picks up and lets go of geometry like the Snake Hook, but much softer. Useful for subtle small scale
   deforming over longer strokes.

Thumb
   Brush Type: thumb

   Same as *Grab* but moves vertices along the surface direction. Useful for preserving specific surfaces.

Pose
   Brush Type: pose

   Simulating an armature-like deformations. Useful for quick posing and transformations.

Nudge
   Brush Type: nudge

   Similar as *Thumb* but dynamically picks up vertices like the *Snake Hook*.
   Useful for nudging something along the mesh surface.

Twist
   Brush Type: rotate

   Rotates vertices within the brush in the direction the cursor is moved.

Relax Slide
   Brush Type: slide_relax

   Slides the topology of the mesh in the direction of the stroke
   while preserving the geometrical shape of the mesh. Alternatively smooths the mesh on 'Shift'.
   Also useful for redistributing topology where it is needed.

Relax Pinch
   Brush Type: slide_relax

   Similar to the Relax Slide brush but pinches/relaxes geometry instead.

Boundary
   Brush Type: boundary

   Transform specifically mesh boundaries with various deformations.

Utility Brushes
===============

No clear color assignment.
These brushes are general purpose brushes or specific.

Density
   Brush Type: simplify

   Cleans up geometry by collapsing short edges. Specifically for use with Dyntopo <dyntopo_introduction>.

Mask
   | Brush Type: mask
   | Shortcut:  M

   Paints a selection on parts of the mesh to be unaffected by other brushes.
   It is possible to temporarily toggle this brush with Alt-LMB,
   to temporarily toggle mask erasing use Ctrl-Alt-LMB,

Draw Face Sets
   Brush Type: draw_facesets

   Paint new, smooth or extend existing Face Sets.

Erase Multires Displacement
   Brush Type: multires_displacement_eraser

   Remove displacement information on a Multiresolution modifier.

Smear Multires Displacement
   Brush Type: multires_displacement_smear

   Smear displacement information on a Multiresolution modifier.

Painting Brushes
================

Recognizable by their blue thumbnails.
These brushes are used for painting color attributes within sculpt mode.

Airbrush
   Brush Type: paint

   A soft round brush that builds up over time instead of stroke distance.

Blend Hard
   Brush Type: paint

   Similar to Average brushes in other modes with a hard round falloff. Used to blend colors along the stroke.

Blend Soft
   Brush Type: paint

   Same as Blend Hard but with a soft round falloff.

Blend Square
   Brush Type: paint

   Same as Blend Hard but with a hard square falloff.

Blur
   Brush Type: paint

   Smooths painted colors; softening transitions and reducing sharp color differences.

Paint Blend
   Brush Type: paint

   A mix of a Paint and Blend brush. On low pen pressure the brush averages colors and with high pen pressure it
   paints colors.

Paint Hard
   Brush Type: paint

   A simple hard round falloff.

Paint Hard Pressure
   Brush Type: paint

   A hard round falloff with pressure sensitivity for the brush size.

Paint Soft
   Brush Type: paint

   A soft round falloff with pressure sensitivity for only the strength.

Paint Soft Pressure
   Brush Type: paint

   A soft round falloff with pressure sensitivity for both size and strength.

Paint Square
   Brush Type: paint

   A hard square brush falloff.

Sharpen
   Brush Type: smear

   Pinches the colors inwards to create sharp edges or points.

Smear
   Brush Type: smear

   Smears colors along the stroke.

Simulation Brushes
==================

These brushes are similar to regular brushes but with an additional cloth simulation applied.
These are ideally used on a relatively low resolution, since the mesh density defines the size of cloth dynamics.

Drag Cloth
   Brush Type: cloth

   Nudges the geometry along the surface while minimally affecting the overall shape of the object.

Push Cloth
   Brush Type: cloth

   Pushes geometry inwards or outwards.

Grab Cloth
   Brush Type: cloth

   Grabs geometry within the brush radius firmly, while surrounding geometry is being simulated to follow.

Grab Planar Cloth
   Brush Type: cloth

   Similar to Grab Cloth but with a line as the brush radius instead of a circle.

Grab Random Cloth
   Brush Type: cloth

   Similar to Grab Cloth but with a noise texture applied to create more random variation.

Inflate Cloth
   Brush Type: cloth

   Inflates the geometry outwards or inwards.

Expand/Contract Cloth
   Brush Type: cloth

   Creates compression or stretching on geometry.

Pinch Point Cloth
   Brush Type: cloth

   Pinches geometry to the center point of the radius, creating folds from all sides.

Pinch Folds Cloth
   Brush Type: cloth

   Pinches only from two perpendicular sides along the stroke direction, creating parallel folds along the stroke.

Bend/Twist Cloth
   Brush Type: pose

   A pose brush that rotates geometry.

Stretch/Move Cloth
   Brush Type: pose

   A pose brush that translates and scales geometry.

Bend Boundary Cloth
   Brush Type: boundary

   Bend only open boundaries of the mesh, folding the surrounding geometry in the process.

Twist Boundary Cloth
   Brush Type: boundary

   Twist open boundaries of the mesh, creating twisting folds.
