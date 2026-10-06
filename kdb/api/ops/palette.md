---
area: api-ops
group: palette
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.palette

- `bpy.ops.palette.color_add()` — Add new color to active palette
- `bpy.ops.palette.color_delete()` — Remove active color from palette
- `bpy.ops.palette.color_move(type: enum{UP, DOWN})` — Move the active Color up/down in the list
- `bpy.ops.palette.extract_from_image(threshold: int)` — Extract all colors used in Image and create a Palette
- `bpy.ops.palette.join(palette: string)` — Join Palette Swatches
- `bpy.ops.palette.new()` — Add new palette
- `bpy.ops.palette.sort(type: enum{HSV, SVH, VHS, LUMINANCE})` — Sort Palette Colors