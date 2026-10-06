---
area: manual-modeling
source: blender-manual
version: 5.2
---

.. _bpy.types.GeometryNodeGetAttributeNames:

************************
Get Attribute Names Node
************************

The *Get Attribute Names* node outputs the names of attributes stored on a
geometry as a list of strings.

The list can be filtered by attribute data type and domain.

Inputs
======

Geometry
   Geometry to inspect for attributes.

Filter Data Type
----------------

Data Type
   Data type of attributes to include in the output list.

Filter Domain
-------------

Domain
   Attribute domain to include in the output list.

Properties
==========

Component
   Geometry component to inspect for attributes.

Outputs
=======

Names
   List of attribute names matching the selected filters.
