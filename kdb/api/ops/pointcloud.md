---
area: api-ops
group: pointcloud
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.pointcloud

- `bpy.ops.pointcloud.attribute_set(value_float: float, value_float_vector_2d: float, value_float_vector_3d: float, value_float_vector_4d: float, value_int: int, value_int_vector_2d: int, value_color: float, value_bool: boolean)` — Set values of the active attribute for selected elements
- `bpy.ops.pointcloud.delete()` — Remove selected points
- `bpy.ops.pointcloud.duplicate()` — Copy selected points
- `bpy.ops.pointcloud.duplicate_move(POINTCLOUD_OT_duplicate: pointer, TRANSFORM_OT_translate: pointer)` — Make copies of selected elements and move them
- `bpy.ops.pointcloud.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — (De)select all points
- `bpy.ops.pointcloud.select_random(seed: int, probability: float)` — Randomize existing selection or create new random selection
- `bpy.ops.pointcloud.separate()` — Separate selected geometry into a new point cloud