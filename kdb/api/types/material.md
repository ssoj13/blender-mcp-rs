---
area: api-types
group: material
version: 5.2.2
---

# Blender 5.2.2 RNA types: material

## Material

Material data-block to define the appearance of geometric objects for rendering

- `surface_render_method` · enum · {DITHERED, BLENDED} — Controls the blending and the compatibility with certain features
- `displacement_method` · enum · {BUMP, DISPLACEMENT, BOTH} — Method to use for the displacement
- `blend_method` · enum · {OPAQUE, CLIP, HASHED, BLEND} — Blend Mode for Transparent Faces (Deprecated: use 'surface_render_method')
- `alpha_threshold` · float — A pixel is rendered only if its alpha value is above this threshold
- `use_transparency_overlap` · boolean — Render multiple transparent layers (may introduce transparency sorting problems)
- `show_transparent_back` · boolean — Render multiple transparent layers (may introduce transparency sorting problems) (Deprecated: use 'use_tranparency_overlap')
- `use_backface_culling` · boolean — Use back face culling to hide the back side of faces
- `use_backface_culling_shadow` · boolean — Use back face culling when casting shadows
- `use_backface_culling_lightprobe_volume` · boolean — Consider material single sided for light probe volume capture. Additionally helps rejecting probes inside the object to avoid light leaks.
- `use_transparent_shadow` · boolean — Use transparent shadows for this material if it contains a Transparent BSDF, disabling will render faster but not give accurate shadows
- `use_raytrace_refraction` · boolean — Use raytracing to determine transmitted color instead of using only light probes. This prevents the surface from contributing to the lighting of surfaces not using this setting.
- `use_screen_refraction` · boolean — Use raytracing to determine transmitted color instead of using only light probes. This prevents the surface from contributing to the lighting of surfaces not using this setting. Deprecated: use 'use_raytrace_refraction'.
- `use_sss_translucency` · boolean — Add translucency effect to subsurface (Deprecated)
- `refraction_depth` · float — Approximate the thickness of the object to compute two refraction events (0 is disabled) (Deprecated)
- `thickness_mode` · enum · {SPHERE, SLAB} — Approximation used to model the light interactions inside the object
- `use_thickness_from_shadow` · boolean — Use the shadow maps from shadow casting lights to refine the thickness defined by the material node tree
- `volume_intersection_method` · enum · {FAST, ACCURATE} — Determines which inner part of the mesh will produce volumetric effect
- `max_vertex_displacement` · float — The max distance a vertex can be displaced. Displacements over this threshold may cause visibility issues.
- `preview_render_type` · enum · {FLAT, SPHERE, CUBE, HAIR, SHADERBALL, CLOTH, FLUID} — Type of preview render
- `use_preview_world` · boolean — Use the current world background to light the preview render
- `pass_index` · int — Index number for the "Material Index" render pass
- `node_tree` · pointer · -> NodeTree · read-only — Node tree for node based materials
- `use_nodes` · boolean — Use shader nodes to render the material
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `texture_paint_images` · collection · read-only — Texture images used for texture painting
- `texture_paint_slots` · collection · read-only — Texture slots defining the mapping and influence of textures
- `paint_active_slot` · int — Index of active texture paint slot
- `paint_clone_slot` · int — Index of clone texture paint slot
- `diffuse_color` · float · [4] — Diffuse color of the material
- `specular_color` · float · [3] — Specular color of the material
- `roughness` · float — Roughness of the material
- `specular_intensity` · float — How intense (bright) the specular reflection is
- `metallic` · float — Amount of mirror reflection for raytrace
- `line_color` · float · [4] — Line color used for Freestyle line rendering
- `line_priority` · int — The line color of a higher priority is used at material boundaries
- `grease_pencil` · pointer · -> MaterialGPencilStyle · read-only — Grease Pencil color settings for material
- `is_grease_pencil` · boolean · read-only — True if this material has Grease Pencil data
- `lineart` · pointer · -> MaterialLineArt · read-only — Line Art settings for material
- `cycles` · pointer · -> CyclesMaterialSettings · read-only — Cycles material settings

## MaterialGPencilStyle

- `color` · float · [4]
- `fill_color` · float · [4] — Color for filling region bounded by each stroke
- `mix_color` · float · [4] — Color for mixing with primary filling color
- `mix_factor` · float — Mix Factor
- `mix_stroke_factor` · float — Mix Stroke Factor
- `texture_angle` · float — Texture Orientation Angle
- `texture_scale` · float · [2] — Scale Factor for Texture
- `texture_offset` · float · [2] — Shift Texture in 2d Space
- `pixel_size` · float — Texture Pixel Size factor along the stroke
- `hide` · boolean — Set color Visibility
- `lock` · boolean — Protect color from further editing and/or frame changes
- `ghost` · boolean — Display strokes using this color when showing onion skins
- `texture_clamp` · boolean — Do not repeat texture and clamp to one instance only
- `flip` · boolean — Flip filling colors
- `use_overlap_strokes` · boolean — Disable stencil and overlap self intersections with alpha materials
- `use_stroke_holdout` · boolean — Remove the color from underneath this stroke by using it as a mask
- `use_fill_holdout` · boolean — Remove the color from underneath this stroke by using it as a mask
- `show_stroke` · boolean — Show stroke lines of this material
- `show_fill` · boolean — Show stroke fills of this material
- `alignment_mode` · enum · {PATH, OBJECT, FIXED} — Defines how align Dots and Boxes with drawing path and object rotation
- `alignment_rotation` · float — Additional rotation applied to dots and square texture of strokes
- `placement_mode` · enum · {COUNT, RADIUS, DENSITY} — Defines how Dots or Squares are placed along strokes
- `placement_count` · int — Number of dots placed per segment
- `placement_radius_spacing` · float — Spacing between dots as a percentage of the diameter
- `placement_density` · float — Density of dots along the stroke
- `use_randomization` · boolean — Use material randomization
- `random_size_factor` · float — Randomize the size
- `random_strength_factor` · float — Randomize strength
- `random_rotation_factor` · float — Randomize texture rotation
- `random_hue_factor` · float — Randomize color hue
- `random_saturation_factor` · float — Randomize color saturation
- `random_value_factor` · float — Randomize color value
- `random_noise_scale` · float — Scale the noise frequency
- `pass_index` · int — Index number for the "Color Index" pass
- `mode` · enum · {LINE, DOTS, BOX} — Select line type for strokes
- `stroke_style` · enum · {SOLID, TEXTURE} — Select style used to draw strokes
- `stroke_image` · pointer · -> Image
- `fill_style` · enum · {SOLID, GRADIENT, TEXTURE} — Select style used to fill strokes
- `gradient_type` · enum · {LINEAR, RADIAL} — Select type of gradient used to fill strokes
- `fill_image` · pointer · -> Image
- `is_stroke_visible` · boolean · read-only — True when opacity of stroke is set high enough to be visible
- `is_fill_visible` · boolean · read-only — True when opacity of fill is set high enough to be visible

## MaterialLineArt

- `use_material_mask` · boolean — Use material masks to filter out occluded strokes
- `use_material_mask_bits` · boolean
- `mat_occlusion` · int — Faces with this material will behave as if it has set number of layers in occlusion
- `intersection_priority` · int — The intersection line will be included into the object with the higher intersection priority value
- `use_intersection_priority_override` · boolean — Override object and collection intersection priority value

## MaterialSlot

Material slot in an object

- `link` · enum · {OBJECT, DATA} — Link material to object or the object's data
- `material` · pointer · -> Material — Material data-block used by this material slot
- `slot_index` · int · read-only
- `name` · string · read-only — Material slot name
