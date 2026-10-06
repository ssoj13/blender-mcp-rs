---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

.. _bpy.ops.sculpt.mask_expand:
.. _bpy.ops.sculpt.expand:

******
Expand
******

This is a multi-purpose modal operator to intuitively create and edit masks/face sets.
When executed, it uniformly expand outwards a pattern from the vertex under the cursor.

**Note:**

   These operators are meant to be used interactively through the shortcut.

There is also a `full showcase of the Expand features and use cases <https://www.youtube.com/watch?v=XT7h6lmE5bc>`__.

Expand Mask by Topology
=======================

Expands a mask from the active vertex.

Usage
-----

Mask from A to B
   Start the operator and expand a mask from an origin to your mouse cursor distance.
   Then confirm with LMB or Return

   By default the expansion will use a *Geodesic* falloff 1
   to create perfectly accurate distances along the surfaces.
   Use other falloff types via 2, 3 & 4 to expand via
   triangles, whole faces or scene distances instead.

   Start Expand while pointing at an open boundary to expand from the entire boundary loop.
   This will always use the *Topology* falloff mode.

   .. needs visual practical example

Fill Connected Meshes
   Move your cursor outside of the boundaries of the mesh to mask the entire connected mesh.
   This can be done repeatedly to quickly mask separate meshes.

Fill Face Sets
   Hold Ctrl to snap to face sets under your cursor and fill them.
   Any face set that was already fully covered in the expansion will be filled as well.

Automatically Set Transform Pivot
   While using any Transform tool, the pivot point
   will automatically snap the border of an Expand result.
   This way areas (Like limbs) can be masked and then immediately rotated or otherwise transformed.

Pattern Creation
   The different falloff types can be used for circular, triangular and square patterns.
   More loops can also be added/removed via W & Q to repeat the pattern across the mesh.

**Tip:**

      Mirror options
      can also be combined with the expansion.

   Linear gradients G or brush falloff gradients B will help to add slanted surfaces
   to the patterns.

   A "Recursions" with R or Alt-R will start
   a new expansion along the border of the current expansion.
   Doing this multiple times, can help for increasingly random patterns
   or advanced pattern creation.

**Tip:**

      Remember that Expand only affects visible geometry.
      So if a pattern should only be created on a part of the mesh,
      hide </sculpt_paint/sculpting/editing/sculpt> the other geometry first.

   Use the Mesh Filter
   to deform the geometry and the Color Filter
   to add colors, to apply the patterns on the sculpt.

Expanding Textures
   Textures can be used to affect the shape and gradients of the expanded mask.
   This feature can be combined with loops and recursion to create unique results.

   .. needs visual example with Michel fur patterns

   To use a texture, you need to assign it to your currently active brush in the
   Texture </sculpt_paint/brush/texture> Brush Settings. The texture can be edited/created
   in the Texture Properties </render/materials/legacy_textures/index>.

**Note:**

      This texture only works when the Mapping <bpy.types.BrushTextureSlot.map_mode> is set to *3D*.

   Use Y and T to increase or decrease the affect the texture has on the edge of the mask.

Controls
--------

:Invert: F
   Flips between expanding a positive mask (value of one) or a negative mask (value of zero).
   In the case of face sets, this option flips between including areas inside the masked area
   or areas outside the masked area.
   .. needs visual technical examples for both
:Toggle Preserve State: E
   Accumulate the new mask on top of the previous one or replace it.
   For Face Sets, this will toggle between creating Face Sets boundaries
   or replacing the existing Face Sets.
:Move Origin: Spacebar
   Moves the initial vertex used for calculating the falloff.
   .. needs visual technical example
:Geodesic Falloff: 1
   Uses a falloff based on the Geodesic distances from the edge boundary to the active vertex.
:Topology Falloff: 2
   Uses a falloff based on a flood fill using edges.
:Diagonals Falloff: 3
   Uses a falloff based on a flood fill using polygon diagonals and edges.
:Spherical Falloff: 4
   Uses a falloff based on the Euclidean distances from the edge boundary to the active vertex.
   .. needs visual technical examples
:Toggle Gradient: G
   Enables linear gradient of values from the origin to the current active vertex.
:Toggle Brush Gradient: B
   Similar to linear gradient but uses the current brush Falloff
   to define the shape of the falloff.
   .. needs visual technical examples
:Geodesic Recursive Step: R
   Start a new expansion with a Geodesic falloff from the boundary of the current falloff.
:Topology Recursive Step: Alt-R
   Start a new expansion with a topology falloff from the boundary of the current falloff.
   .. needs visual technical examples
:Snap Expanded to Face Sets: Ctrl
   Isolates the expanded region to the boundary of the face set under the cursor.
:Loop Count Increase: W
   Increase the number of loops or iterations the operator is run;
   using four loops will split the mask into four parts.
:Loop Count Decrease: Q
   Decrease the number of loops or iterations the operator is run;
   using four loops will split the mask into four parts.
   .. needs visual technical examples
:Texture Distortion Increase: Y
   Increases the falloff distance when using a texture to distort the mask shape.
:Texture Distortion Decrease: T
   Decreases the falloff distance when using a texture to distort the mask shape.
   ..needs visual technical examples

Expand Mask by Normals
======================

Expand a mask, following the curvature of the surface.
This operator uses the same internal operator as bpy.ops.sculpt.expand
meaning all the shortcuts and functionality works the same as that tool.

This operator is especially useful for hard surface sculpting.

**Tip:**

    If one expansion does not properly fill the entire desired surface,
    use the operator repeatedly with a different starting point.

.. needs visual practical example

**Note:**

   Using any of the Falloff shortcuts 1-4
   will replace the curvature falloff of this operator.

.. _face_set_expand:

Expand Face Set by Topology
===========================

Expands a face set from the active vertex.
This operator uses the same internal operator as bpy.ops.sculpt.expand
meaning all the hotkeys and functionality works the same as that tool,
with the gradient features as the exception.

Usage
-----

Saving Masks
   Expanding Face Sets has all the same use cases as expanding masks.
   The advantage for this one is that they will be saved for repeated usage.
   Face sets can be filled any time with a mask,
   so assigning areas face sets will save you time.
   (And of course face sets can be used to
   hide face sets </sculpt_paint/sculpting/editing/sculpt>)

Pivot Points for Pose Brush
   When using the Pose Brush </sculpt_paint/sculpting/brushes/pose> it is most predictable when using it with
   Face Sets to define the face set boundaries as pivot point locations.
   Face Sets can be expanded from a point or from a boundary between hidden face sets
   to create them quickly.
   Alternatively Grow/Shrink Face Sets
   or use the Expand Active Face Set <face_set_expand_active> to dynamically grow/shrink them.

   .. Visual examples for pose brush on fingers, expanding face set from finger segment boundary
   .. and growing face set segment.

Cloth Sculpting
   Tools like the Cloth Filter
   and Cloth Brush
   work especially well when only simulating small areas at a time.
   Face Sets can very easily be created with Expand to assign areas of action.

   .. needs visual examples with grids & a ribbon

.. _face_set_expand_active:

Expand Active Face Set
======================

Expands an existing face set with a geodesic falloff.
This operator uses the same internal operator as bpy.ops.sculpt.expand
meaning all the hotkeys and functionality works the same as that tool.

**Note:**

   Using any of the Falloff shortcuts 1-4
   the operator to switch to Expand Face Set by Topology <face_set_expand>.

Usage
-----

Resizing Face Sets
   Resize a Face Set along the surface distances.
   It is an alternative to Grow/Shrink Face Sets
   which follows the topology instead of geodesic distances.

   .. needs visual examples side by side

Joining Face Sets
   When holding Ctrl the expansion will instead snap to other Face Sets.
   This is a fast way of joining multiple face sets into one.

   .. needs visual examples side by side
