---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeGreasePencilToCurves:

****************************
Grease Pencil to Curves Node
****************************

The *Grease Pencil to Curves* node converts each Grease Pencil layer into an instance
that contains curves.

Inputs
======

Grease Pencil
   Standard Grease Pencil geometry.

Selection
   Selects the layers to convert.

Instances as Layers
   Create a separate curve instance for every layer.

**Tip:**
      This is equivalent to a Realize Instances
      operation after creating the curve geometry instances. This can simplify some logic later on, but
      typically keeping this operation off will be much better for performance.

Outputs
=======

Curves
   Standard curves geometry.
