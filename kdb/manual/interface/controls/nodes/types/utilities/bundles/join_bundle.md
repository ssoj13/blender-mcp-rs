---
area: manual-interface
source: blender-manual
version: 5.2
---

.. _bpy.types.NodeJoinBundle:
.. --- copy below this line ---

****************
Join Bundle Node
****************

The *Join Bundle* node combines multiple bundles into a single output bundle.
Each connected input bundle is merged together, producing one bundle
that contains all items from the inputs.

Inputs
======

Bundle
   One or more input bundles to combine.
   Each bundle's contents are merged by name into the output bundle.

**Note:**

   If multiple bundles contain sockets with the same name (duplicate keys),
   the value from the first occurrence is used.
   An information icon in the node header will indicate if duplicate keys are detected.

Outputs
=======

Bundle
   The resulting bundle that contains all items from the input bundles.

   Use the separate_bundle to access individual items from the combined bundle if needed.
