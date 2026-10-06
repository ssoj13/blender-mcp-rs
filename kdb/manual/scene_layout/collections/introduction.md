---
area: manual-scene_layout
source: blender-manual
version: 5.2
---

************
Introduction
************

In Blender, objects are not directly part of the scenes.
Instead, they all get stored in a main database (basically the blend-file).

From there they are referenced into as many Scenes as you would like to see them.

When they are stored in a scene, they are part of a so-called *scene collection*.
So ultimately all the scene objects belong to this special collection.

Collections
===========

While the *scene collection* contains all the Scene's objects,
the user can also make their own collections to better organize these objects.

It works like a Venn diagram, where all the objects are part of the *scene collection*,
but can also be part of multiple collections.

The result is a clear and flexible way to arrange objects together on the Scene level.

Naming and Nesting
==================

Collections can be named and sorted hierarchically.
Just like folders can have subfolders in any operating system,
collections can have nested collections too.

For example: a *house* collection can contain a *bedroom* collection,
which in turn contains a *furniture* collection referencing a bed, a cabinet and other objects.

.. _scene_layout-collections-color-tagging:

Color Tagging
=============

Collections can have a color group assigned to them; helping organize
and group different collections. This color tag is displayed as part of
the collection icon in the Outliner </editors/outliner/index>
and various other menus. The available colors are defined by
Blender's interface Theme </editors/preferences/themes>.

To assign a color to a collection,
use the Set Color Tag <bpy.ops.outliner.collection_color_tag_set> tool in the Outliner.
