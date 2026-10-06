---
area: api-ops
group: cycles
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.cycles

- `bpy.ops.cycles.denoise_animation(input_filepath: string, output_filepath: string)` — Denoise rendered animation sequence using current scene and view layer settings. Requires denoising data passes and output to OpenEXR multilayer files
- `bpy.ops.cycles.merge_images(input_filepath1: string, input_filepath2: string, output_filepath: string)` — Combine OpenEXR multi-layer images rendered with different sample ranges into one image with reduced noise
- `bpy.ops.cycles.use_shading_nodes()` — Enable nodes on a light