---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

.. _bpy.types.Brush:
.. _bpy.ops.brush:
.. _bpy.types.UnifiedPaintSettings:

************
Introduction
************

Brushes are the main way of interacting with any painting and sculpting mode. By click & dragging in the 3D Viewport
(or the Image Editor when using Texture Paint </sculpt_paint/texture_paint/index>), the active brush creates a
stroke </sculpt_paint/brush/stroke> with a certain effect, depending on the used brush settings. Brushes are
used as brush assets and stored in asset libraries, which makes it easy to reuse and share them. Typically they have a
preview image and a name that indicate the effect they create.

**Tip:**

   It is highly recommended to use a Graphics Tablet
   for a better brush feel and additional features.

.. _brush-introduction-accessing-brushes:

Accessing Brushes
=================

In modes that use painting or sculpting functionality, the Asset Shelf <ui-region-asset_shelf> of the 3D
Viewport and Image Editor displays brush assets that can be used in that mode. Clicking a brush asset will activate
the Brush Tool if necessary, with the clicked brush set.

This asset shelf is also available as popup in the Tool Settings <ui-region-tool-settings>, the
Sidebar<ui-region-sidebar>, Properties </editors/properties_editor> and using a shortcut.

Brush Control
=============

These are the most common hotkeys for controlling the brush.

- Set brush size F
- Set brush strength Shift-F
- Rotate brush texture / Set brush weight Ctrl-F

After pressing these hotkeys, you can then either adjust the value interactively or by typing in numbers.
Move the mouse right or left to increase/reduce the value
(additionally with precision (Shift) and/or snapping (Ctrl) activated).
Finally confirm (LMB, Return) or cancel (RMB, Esc).

You can also invert the brush direction/effect by holding Ctrl.

Custom Brush Shortcuts
======================

To give a brush a shortcut, simply right click it in the asset shelf or brush selector popup, and
select *Assign Shortcut*. To modify or remove an existing shortcut, select *Change Shortcut* or
*Remove Shortcut* accordingly.

Brush Assets
============

Brushes are used as assets, and stored in asset libraries </files/asset_libraries/introduction>. This makes the
brushes shared across project files. All available brush assets can be displayed in the
Asset Browser </editors/asset_browser>, which also provides ways to organize them.

Blender comes bundled with a number of brushes in the `Essentials` asset library. These can be customized into all
kinds of custom brushes by duplicating them (see Brush Editing<brush_management>).

While it's possible to have brush data-blocks that are local to the file and not marked as assets, such brushes cannot
be activated for actual painting or sculpting. Use the `Mark as Asset` operator to make them brush assets that can be
activated.

Brush Tool
==========

Painting or sculpting with brushes requires the brush tool to be active. Activating a brush from an
asset shelf or brush selector also activates the brush tool for convenience.
