---
area: manual-modeling
source: blender-manual
version: 5.2
---

***************************
Editing Point Cloud Objects
***************************

In *Edit Mode*, you can apply basic editing operations to point cloud objects.

**Tip:**

   Point clouds can be converted to or from mesh objects
   for workflows that require geometry-based editing or further processing.

Transform
=========

Standard transformation operators such as **Move**, **Rotate**, and **Scale**
can be used to manipulate selected points within the point cloud.

These operators are useful for repositioning, aligning, or reshaping clusters of points manually.

.. _bpy.ops.pointcloud.duplicate_move:

Duplicate
=========

Creates a copy of the selected points and reposition the duplicated points.

Duplicated points inherit all attributes from the original points.
- Can be used for manually scattering point clusters or duplicating specific regions for further modification.

.. _bpy.ops.pointcloud.attribute_set:

Set Attribute
=============

Opens a pop-up window showing the name of the active attribute
as well as the value of that attribute for the selected points
From there, you assign a new value to a selected attribute across all selected points.

This tool is useful for uniformly setting attribute values such as size, color, or velocity across selected points.

.. _bpy.ops.pointcloud.delete:

Delete
======

Removes selected points from the point cloud.

.. _bpy.ops.pointcloud.separate:

Separate
========

Creates a new point cloud object from the currently selected points.

This is useful for breaking a larger scan or dataset into manageable segments
or organizing different regions of points into separate objects.
