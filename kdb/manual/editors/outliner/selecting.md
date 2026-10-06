---
area: manual-editors
source: blender-manual
version: 5.2
---

************************
Selecting Outliner Items
************************

Selection is done with LMB (and/or the context menu <editors-outliner-editing-context_menu>)
on the row of a data-block. Single selections will also activate the data-block.
The rows of selected data-blocks are highlighted blue, with the active data-block highlighted in a lighter blue.

Clicking in empty space below the list of data-blocks will deselect all.

**Note:**

   By default, selecting data-blocks in the Outliner will select the respective objects,
   bones, and sequences in the 3D Viewport and Video Sequencer. Selections in the 3D Viewport
   and Video Sequencer will be synced to each Outliner. To disable selection syncing, turn off
   the toggle in the filter <editors-outliner-interface-filter> popover.

Children of a data-block can also be selected by clicking the icon that is displayed to
the right of the parent data-block's name.

Selecting Multiple Data-Blocks
==============================

Extend the selection one data-block at a time using Ctrl-LMB.
Each data-block added to the selection this way will be made the active data-block.

Select a range of elements from the active element using Shift-LMB.
To select a range without deselecting the previous selection, use Shift-Ctrl-LMB.

Select all children of an item by double clicking the item's icon.

A click and drag from any location in the Outliner other than a name or icon will begin a box selection.
Use Shift to add and Ctrl to subtract from existing selections with box select.
Box select can also be started with B.

To select all items use A; Alt-A will deselect all items.

The keyboard arrow keys can be used to navigate and select in the Outliner.
Keyboard selection and navigation starts from the active data-block.

.. list-table::
   :widths: 10 90

   * - Up
     - Select the previous element in the list.
   * - Down
     - Select the next element in the list.
   * - Shift-Up
     - Select the previous element without deselecting.
   * - Shift-Down
     - Select the next element without deselecting.
   * - Left
     - Close the data-block or select the parent.
   * - Right
     - Open the data-block to view children or select the first child.
   * - Shift-Left
     - Close this and all child data-blocks.
   * - Shift-Right
     - Open this and all child data-blocks.

.. _editors-outliner-properties-sync:

Properties Editor Sync
======================

When clicking the icon (not the name) of an item in the Outliner, the /editors/properties_editor
editor will automatically switch to the relevant tab. This feature can be disabled using
the Properties editor's Sync with Outliner <bpy.types.SpaceProperties.outliner_sync> option.
