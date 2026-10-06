---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeCurvesToGreasePencil:

****************************
Curves to Grease Pencil Node
****************************

The *Curves to Grease Pencil* node Converts top-level curve instances into Grease Pencil layers

Inputs
======

Curves
   Either plain curves or curve instances.

Selection
    Either a curve or instance selection.

Instances as Layers
    Create a separate layer for each instance. If instances are used, realized curve geometry will be
    ignored. Layer names will use the instance geometry name. If real curve geometry is used, a single
    layer is created with the input geometry's name.

Outputs
=======

Grease Pencil
   Standard Grease Pencil geometry.

Examples
========
