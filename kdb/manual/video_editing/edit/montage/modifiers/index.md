---
area: manual-video_editing
source: blender-manual
version: 5.2
---

.. _bpy.types.StripModifier:

***************
Strip Modifiers
***************

Modifiers are used to make adjustments to the image, like contrast,
brightness, saturation, color balance and applying masks.

You can add these modifiers directly to a media strip,
or you can use them within an Adjustment Layer </video_editing/edit/montage/strips/adjustment>
strip, making them apply to several media strips in one go.

Modifier Assets
===============

Compositor node groups marked as Assets </editors/asset_browser>
can be used as strip modifiers.

Asset modifiers appear in the Add menu alongside Blender's
built-in modifiers. Assets assigned to :doc:`Catalogs
</files/asset_libraries/catalogs>` are organized into matching submenus.

This makes it possible to build reusable modifier libraries and share custom
effects across multiple projects.

**Tip:**

   To make a compositor node group available as a strip modifier,
   mark it as an asset by right-clicking its data-block selector and choosing
   Mark as Asset.

   After saving the file, the asset will appear in the modifier
   Add menu. Existing assets created in older Blender versions
   may need to be re-saved before they become available.

Common Options
==============

Each modifier provides several controls in its header:

:bl-icon:`restrict_view_on` / :bl-icon:`restrict_view_off` Viewport
   Toggles the modifier effect in the Preview region.
:bl-icon:`restrict_render_on` / :bl-icon:`restrict_render_off` Render
   Enables or disables the modifier during rendering and final output.
   This can be useful for comparing the strip with and without the modifier applied.

.. _bpy.ops.sequencer.strip_modifier_duplicate:

Extras
   Duplicate Shift-D
      Creates a duplicate of the modifier just below current one in the stack.

   .. _bpy.ops.sequencer.strip_modifier_copy:

   Copy to Selected
      Copies the modifier from the Active strip to all selected strips.

   .. _bpy.ops.sequencer.strip_modifier_move_to_index:

   Move to First/Last
      Moves the modifier to the first or last position in the modifier stack.

   .. _bpy.types.SequencerCompositorModifierData.show_group_selector:

   Show Node Group
      For Compositor Modifiers <bpy.types.SequencerCompositorModifierData>,
      shows or hides the node group selector.

:bl-icon:`x` Remove Strip Modifier
   Removes the modifier from the modifier stack.
:bl-icon:`grip` Move Strip Modifier
   Drag to reorder modifiers in the stack.

   Modifiers are evaluated from top to bottom, so changing their order affects the final result.

Types
=====

Strip modifiers include built-in image and sound modifiers as well as custom
compositor modifier assets.

Image Modifiers
---------------

-----

Sound Modifiers
---------------
