---
area: api-ops
group: render
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.render

- `bpy.ops.render.clear_texture_cache()` — Delete Cycles texture cache files from disk
- `bpy.ops.render.color_management_white_balance_preset_add(name: string, remove_name: boolean, remove_active: boolean)` — Add or remove a white balance preset
- `bpy.ops.render.cycles_integrator_preset_add(name: string, remove_name: boolean, remove_active: boolean)` — Add an Integrator Preset
- `bpy.ops.render.cycles_performance_preset_add(name: string, remove_name: boolean, remove_active: boolean)` — Add an Performance Preset
- `bpy.ops.render.cycles_sampling_preset_add(name: string, remove_name: boolean, remove_active: boolean)` — Add a Sampling Preset
- `bpy.ops.render.cycles_viewport_sampling_preset_add(name: string, remove_name: boolean, remove_active: boolean)` — Add a Viewport Sampling Preset
- `bpy.ops.render.eevee_raytracing_preset_add(name: string, remove_name: boolean, remove_active: boolean)` — Add or remove an EEVEE ray-tracing preset
- `bpy.ops.render.generate_texture_cache(generate_sequences: boolean)` — Generate Cycles texture cache files for all images used in shader nodes
- `bpy.ops.render.opengl(animation: boolean, render_keyed_only: boolean, sequencer: boolean, write_still: boolean, view_context: boolean)` — Take a snapshot of the active viewport
- `bpy.ops.render.play_rendered_anim()` — Play back rendered frames/movies using an external player
- `bpy.ops.render.preset_add(name: string, remove_name: boolean, remove_active: boolean)` — Add or remove a Render Preset
- `bpy.ops.render.render(animation: boolean, write_still: boolean, use_viewport: boolean, use_sequencer_scene: boolean, layer: string, scene: string, frame_start: int, frame_end: int)` — (undocumented operator)
- `bpy.ops.render.shutter_curve_preset(shape: enum{SHARP, SMOOTH, MAX, LINE, ROUND, ROOT})` — Set shutter curve
- `bpy.ops.render.swap_dimensions()` — Flip X and Y resolutions
- `bpy.ops.render.view_cancel()` — Cancel showing the render view
- `bpy.ops.render.view_show()` — Toggle show render view