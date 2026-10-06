---
area: manual-interface
source: blender-manual
version: 5.2
---

.. _ui-list-view:

*********
List View
*********

This control is useful for managing lists of items.
In addition to the main list, there is a Filtering panel on the bottom
(hidden by default) and modification buttons on the right.

Select
   To select an item, click LMB on it.
Rename
   By double-clicking on an item, you can edit its name via a text field.
   This can also be achieved by clicking it with Ctrl-LMB.
Resize
   The list view can be resized to show more or fewer items.
   Hover the mouse over the handle (::::), then click and drag to expand or shrink the list.

.. _bpy.ops.ui.list_start_filter:

Filter
   Click the *Show filtering options* button (triangle on bottom left) to show or hide the filter option panel.

   Search Ctrl-F
      Filters the list to only show items containing a certain term.
   :bl-icon:`arrow_leftright` Invert
      Toggle between including items that match the search term and those that do not contain the search term.

   Sort by Name
      This button switches between alphabetical and non-alphabetical ordering.
   Reverse
      Sort objects in ascending or descending order. This also applies to alphabetical sorting, if selected.

On the right of the list view are list modification buttons:

:bl-icon:`add` (Add)
   Adds a new item.
:bl-icon:`remove` (Remove)
   Removes the selected item.
:bl-icon:`downarrow_hlt` (Specials)
   A menu with operators to edit list entries.
:bl-icon:`tria_up` / :bl-icon:`tria_down` Move
   Moves the selected item up/down one position.
