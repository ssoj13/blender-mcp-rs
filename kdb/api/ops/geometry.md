---
area: api-ops
group: geometry
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.geometry

- `bpy.ops.geometry.attribute_add(name: string, domain: enum{POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER}, data_type: enum{FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR})` — Add attribute to geometry
- `bpy.ops.geometry.attribute_convert(mode: enum{GENERIC, VERTEX_GROUP}, domain: enum{POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER}, data_type: enum{FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR})` — Change how the attribute is stored
- `bpy.ops.geometry.attribute_remove()` — Remove attribute from geometry
- `bpy.ops.geometry.color_attribute_add(name: string, domain: enum{POINT, CORNER}, data_type: enum{FLOAT_COLOR, BYTE_COLOR}, color: float)` — Add color attribute to geometry
- `bpy.ops.geometry.color_attribute_convert(domain: enum{POINT, CORNER}, data_type: enum{FLOAT_COLOR, BYTE_COLOR})` — Change how the color attribute is stored
- `bpy.ops.geometry.color_attribute_duplicate()` — Duplicate color attribute
- `bpy.ops.geometry.color_attribute_remove()` — Remove color attribute from geometry
- `bpy.ops.geometry.color_attribute_render_set(name: string)` — Set default color attribute used for rendering
- `bpy.ops.geometry.geometry_randomization(value: boolean)` — Toggle geometry randomization for debugging purposes