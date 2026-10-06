---
area: manual-editors
source: blender-manual
version: 5.2
---

**********
Navigating
**********

Panning can be done by dragging with MMB.

Zooming can be done using Wheel or NumpadPlus/NumpadMinus.

.. _editors-image-navigate-gizmos:

Gizmos
======

At the top of the Image Editor, next to the Sidebar region, navigation gizmos are available.
These provide on-screen controls for panning and zooming the view, which can be useful when
a mouse wheel is not available or when working on touch-based or pen-based devices.

The pan gizmo allows dragging the view in any direction, while the zoom gizmo adjusts the
view scale interactively.

When using the zoom gizmo, the current zoom level is displayed as a percentage, providing
immediate feedback about the scale of the image.

Holding Shift while dragging the zoom gizmo enables more precise control.
Holding Ctrl snaps the zoom level to 10% increments, allowing consistent and
predictable adjustments.

View Menu
=========

.. _bpy.types.SpaceImageEditor.show_region:

Toolbar T
   Show or hide the Toolbar <ui-region-toolbar>.
Sidebar N
   Show or hide the Sidebar <ui-region-sidebar>.
Tool Settings
   Show or hide the settings for the currently selected tool.
Asset Shelf
   Toggle the visibility of the ui-region-asset_shelf.
Adjust Last Operation
   Displays a pop-up panel to alter properties of the last
   completed operation. See bpy.ops.screen.redo_last.

Update Automatically
   Instantly update any other editors that are affected by changes in this Image Editor.
   When disabled, the other editors may display outdated information until they're manually refreshed
   (e.g. by orbiting for the 3D Viewport).
Show Metadata
   Displays metadata about the selected Render Result. See the Output tab's
   /render/output/properties/metadata panel to change what metadata to include.

.. _bpy.ops.image.view_zoom_ratio:

Zoom
   Menu with convenient zoom levels and operations.
   The zoom levels are calculated based on the images resolution compared to the screen resolution.

   - 12.5% (1:8) Numpad8 zoom out to a factor of 12.5%.
   - 25% (1:4) Numpad4 zoom out to a factor of 25%.
   - 50% (1:2) Numpad2 zoom out to a factor of 50%.
   - 100% (1:1) Numpad1 resets the zoom to 100%.
     (Ctrl-Numpad1 and Shift-Numpad1 also reset to 100%.)
   - 200% (2:1) Ctrl-Numpad2 zoom in to a factor of 200%.
   - 400% (4:1) Ctrl-Numpad4 zoom in to a factor of 400%.
   - 800% (8:1) Ctrl-Numpad8 zoom in to a factor of 800%.

   Zoom In/Out Wheel
      Zooms the view in or out.
   Zoom to Fit Shift-Home
      Like *Frame All*, but uses as much space in the editor as possible.
   Zoom Region Shift-B
      Zoom in the view to the nearest item contained in the border.
Frame All Home
   Pans and zooms the view so that the image is centered and fully visible.
Center View to Cursor
   Pan the view so that the 2D cursor is at the center of the editor.
Render Region Ctrl-B
   Only available when viewing the Render Result.
   See Render Region <editors-3dview-navigate-render-region>.
Clear Render Region Ctrl-Alt-B
   Only available when viewing the Render Result.
   See Render Region <editors-3dview-navigate-render-region>.
Render Slot Cycle Next/Previous J/Alt-J
   Switch to the next/previous render slot (that contains a render).
Area
   Adjust the area </interface/window_system/areas> the Image Editor is in.
