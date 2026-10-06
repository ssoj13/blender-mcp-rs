---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

.. _sculpt-mask-menu:
.. _bpy.ops.paint.mask:

****
Mask
****

This page details the mask related shortcut operators and menu operators in sculpt mode.
Other related information to masks can also be found at the bottom of the page.

Masks can be edited across all visible faces.
Using A opens a pie menu to choose the most common operations.

.. _mask_invert:

Invert Mask
===========

Inverts the visible mask.
This is often useful when the masked vertices are the surfaces you want to sculpt/paint.
In that case create a mask and then invert it.

.. _bpy.ops.paint.mask_flood_fill:

Fill Mask
=========

Fully masks the visible geometry.
Alternatively it is common to clear and then invert a mask via A
to achieve the same effect.

.. _mask_clear:

Clear Mask
==========

Removes the mask on all visible vertices.
To completely remove the mask data, see `Clear Sculpt-Mask Data`_.

.. _bpy.ops.paint.mask_box_gesture:

Box Mask
========

Works like the Box Mask <tool-box-mask> tool, it creates a rectangular mask region.
Hold Shift or press MMB to clear the mask of the selected region.

.. _bpy.ops.paint.mask_lasso_gesture:

Lasso Mask
==========

Can be used to create a free-form mask, similar to the Lasso Mask <tool-lasso-mask> tool.
This is very commonly used.

**Tip:**

   To clear the mask of areas with the *Lasso Mask*, first invert the mask,
   use *Lasso Mask*, and then invert the mask back.

.. _bpy.ops.sculpt.mask_filter:

Mask Filters
============

Similarly to other Filter Tools </sculpt_paint/sculpting/introduction/filters>,
mask filters are operations that are applied to the whole mask.

Type
   Smooth/Sharpen Mask
      Changes the sharpness of the mask edge.
      Using this can be faster and more consistent than smoothing the mask
      with the Mask </sculpt_paint/sculpting/brushes/mask> brush.
   Grow/Shrink Mask
      Further grow or shrink the mask along the surface of the mesh.
   Increase/Decrease Contrast
      Changes the contrast of the mask.

In the Adjust Last Operation <bpy.ops.screen.redo_last> panel there are further options
to add iterations for a stronger effect.

Iterations
   The number of times the filter is applied.

Auto Iteration Count
   Use an automatic number of iterations based on the number of vertices of the sculpt.
   Disable this option to set the Iterations manually.

**Tip:**

   An alternative to Iterations is to use Repeat Last
   via the shortcut Shift-R.

Expand Mask
===========

**Note:**

   More info on Mask Expand along Topology at the Expand page <bpy.ops.sculpt.expand>.

.. _bpy.ops.sculpt.paint_mask_extract:

Mask Extract
============

Creates a duplicate mesh object based on masked geometry.
The extracted geometry is also further processed by default for a cleaner result.

Threshold
   Minimum mask value to consider the vertex valid to extract a face from the original mesh.

Add Boundary Loop
   Creates and extra boundary loop on the edges of the geometry,
   making it easier smooth the boundaries and apply additional modifiers.

Smooth Iterations
   Smooth iterations applied to the extracted mesh.

Project to Sculpt
   Project the extracted mesh on to the original sculpt object.

Extract as Solid
   Adds a Solidify Modifier </modeling/modifiers/generate/solidify> to the newly created mesh object.

.. _bpy.ops.sculpt.paint_mask_slice:

Mask Slice
==========

Removes the masked vertices from the mesh.

Threshold
   Minimum mask value to consider the vertex valid to extract a face from the original mesh.

Fill Holes
   Fills concave holes with geometry that might have resulted from the *Mask Slice* operation.

**Tip:**

      If nothing is masked, this operation can be used to just fill all holes.
      Especially when using Trim Tools
      tools and the Voxel Remesher

      this is a useful workaround. But once the voxel remesher automatically checks for holes
      or a dedicated Fill Holes operation is added, this tip should be removed.

Slice to New Object
   Create a new object from the masked geometry.

.. _bpy.ops.sculpt.mask_from_cavity:

Mask From Cavity
================

Generates a mask based on the cavity of the surface. The settings of the operation can be changed
in the Adjust Last Operation </interface/undo_redo> panel.

Mode
   Choose how the newly created mask is mixed with the existing one. By default it will replace the old mask via
   "Mix".
Mix Factor
   The factor of the mix effect. Choose how strong the new mask is applied on the existing one.
Automask Settings
   The same settings as the Auto-Masking </sculpt_paint/sculpting/controls> settings are applied.
Factor
   Same as Auto-Masking </sculpt_paint/sculpting/controls>.
Blur
   Same as Auto-Masking </sculpt_paint/sculpting/controls>.
Invert
   Same as Auto-Masking </sculpt_paint/sculpting/controls>.
Custom Curve
   Same as Auto-Masking </sculpt_paint/sculpting/controls>.

.. _sculpt-mask_from_mesh_boundary:

Mask From Mesh Boundary
=======================

Generates a mask based on the topological islands of the mesh. The settings of the operation can be changed
in the Adjust Last Operation </interface/undo_redo> panel.

Mode
   Choose how the newly created mask is mixed with the existing one. By default it will replace the old mask via
   "Mix".
Mix Factor
   The factor of the mix effect. Choose how strong the new mask is applied on the existing one.
Automask Settings
   The same settings as the Auto-Masking </sculpt_paint/sculpting/controls> settings are applied.
Propagation Steps
   Same as Auto-Masking </sculpt_paint/sculpting/controls>.

.. _bpy.ops.sculpt.mask_from_boundary:

Mask From Face Sets Boundary
============================

Generates a mask based on the face set islands of the mesh. The settings of the operation can be changed
in the Adjust Last Operation </interface/undo_redo> panel.

Mode
   Choose how the newly created mask is mixed with the existing one. By default it will replace the old mask via
   "Mix".
Mix Factor
   The factor of the mix effect. Choose how strong the new mask is applied on the existing one.
Automask Settings
   The same settings as the Auto-Masking </sculpt_paint/sculpting/controls> settings are applied.
Propagation Steps
   Same as Auto-Masking </sculpt_paint/sculpting/controls>.

.. _bpy.ops.sculpt.mask_by_color:

Mask by Color
=============

Click on any color on the mesh to create a new mask (based on the active color attribute).

Threshold
   How much changes in color affect the mask generation. A smaller threshold includes fewer similar colors.
   A larger threshold includes much more similar colors.
Contiguous
   Mask only contiguous color areas. Colors that don't touch the one that you click on will not be masked.
Invert
   Invert the generated mask.
Preserve Previous Mask
   Preserve previous mask and add or subtract the new one generated by the colors.

**Seealso:**

   Mask by Color Tool

.. _bpy.ops.sculpt.mask_init:

Random Mask
===========

Generates a mask with random values for the entire object based on different mesh data.

Per Vertex
   Assigns a random mask value for each vertex.
Per Face Set
   Assigns a random mask value for each Face Set </sculpt_paint/sculpting/editing/face_sets>.
Per Loose Mask
   Assigns a random mask value for each disjoint part of the mesh.

.. _Mask Display Settings:

Display Settings
================

The mask display can be toggled as a viewport overlay </editors/3dview/display/overlays>.
In the overlay popover, the opacity of the mask overlay can be adjusted to make it more or less visible on the mesh.

.. _sculpt_mask_clear-data:

Clear Sculpt-Mask Data
======================

Completely frees the mask data layer from the mesh. While not a huge benefit,
this can speed-up sculpting if the mask is no longer being used.
