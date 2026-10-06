---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

******************
Hide Gesture Tools
******************

Hide gesture tools hide all selected vertices within the selection area and any of their connected
edges and faces.
Holding Ctrl while performing the selection reveals the vertices, edges, and faces.

Pressing LMB with any of these tools without also dragging reveals all elements of a mesh.

All hide gesture tools can be activated in the Toolbar and are comprised of the following:

.. _tool-box-hide:

Box Hide
========

Hides vertices and connected edges and faces based on a
box gesture <gesture-tool-box>.

.. _tool-lasso-hide:

Lasso Hide
==========

Hides vertices and connected edges and faces based on a
lasso gesture <gesture-tool-lasso>.

.. _tool-line-hide:

Line Hide
=========

Hides vertices and connected edges and faces based on a
line gesture <gesture-tool-line>.

.. _tool-polyline-hide:

Polyline Hide
=============

Hides vertices and connected edges and faces based on a
polyline gesture <gesture-tool-polyline>.

**Note:**

    The Polyline Hide tool does not support showing all vertices via pressing LMB.

Tool Settings
=============

Visibility Area
   Determines whether all vertices inside or outside the selected area should be affected.

   :Inside:
      All vertices and connected elements inside the selection area will be hidden.
   :Outside:
      All vertices and connected elements outside the selection area will be hidden.
