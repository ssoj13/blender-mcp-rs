---
area: api-types
group: texture
version: 5.2.2
---

# Blender 5.2.2 RNA types: texture

## BlendTexture

Procedural color blending texture

- `progression` · enum · {LINEAR, QUADRATIC, EASING, DIAGONAL, SPHERICAL, QUADRATIC_SPHERE, RADIAL} — Style of the color blending
- `use_flip_axis` · enum · {HORIZONTAL, VERTICAL} — Flip the texture's X and Y axis

## CloudsTexture

Procedural noise texture

- `noise_scale` · float — Scaling for noise input
- `noise_depth` · int — Depth of the cloud calculation
- `noise_basis` · enum · {BLENDER_ORIGINAL, ORIGINAL_PERLIN, IMPROVED_PERLIN, VORONOI_F1, VORONOI_F2, VORONOI_F3, VORONOI_F4, VORONOI_F2_F1, VORONOI_CRACKLE, CELL_NOISE} — Noise basis used for turbulence
- `noise_type` · enum · {SOFT_NOISE, HARD_NOISE}
- `cloud_type` · enum · {GRAYSCALE, COLOR} — Determine whether Noise returns grayscale or RGB values
- `nabla` · float — Size of derivative offset used for calculating normal

## DistortedNoiseTexture

Procedural distorted noise texture

- `distortion` · float — Amount of distortion
- `noise_scale` · float — Scaling for noise input
- `noise_basis` · enum · {BLENDER_ORIGINAL, ORIGINAL_PERLIN, IMPROVED_PERLIN, VORONOI_F1, VORONOI_F2, VORONOI_F3, VORONOI_F4, VORONOI_F2_F1, VORONOI_CRACKLE, CELL_NOISE} — Noise basis used for turbulence
- `noise_distortion` · enum · {BLENDER_ORIGINAL, ORIGINAL_PERLIN, IMPROVED_PERLIN, VORONOI_F1, VORONOI_F2, VORONOI_F3, VORONOI_F4, VORONOI_F2_F1, VORONOI_CRACKLE, CELL_NOISE} — Noise basis for the distortion
- `nabla` · float — Size of derivative offset used for calculating normal

## LineStyleTextureSlot

Texture slot for textures in a LineStyle data-block

- `mapping_x` · enum · {NONE, X, Y, Z}
- `mapping_y` · enum · {NONE, X, Y, Z}
- `mapping_z` · enum · {NONE, X, Y, Z}
- `mapping` · enum · {FLAT, CUBE, TUBE, SPHERE}
- `use_map_color_diffuse` · boolean — The texture affects basic color of the stroke
- `use_map_alpha` · boolean — The texture affects the alpha value
- `texture_coords` · enum · {WINDOW, GLOBAL, ALONG_STROKE, ORCO} — Texture coordinates used to map the texture onto the background
- `alpha_factor` · float — Amount texture affects alpha
- `diffuse_color_factor` · float — Amount texture affects diffuse color

## MagicTexture

Procedural noise texture

- `turbulence` · float — Turbulence of the noise
- `noise_depth` · int — Depth of the noise

## MarbleTexture

Procedural noise texture

- `noise_scale` · float — Scaling for noise input
- `turbulence` · float — Turbulence of the bandnoise and ringnoise types
- `noise_depth` · int — Depth of the cloud calculation
- `noise_type` · enum · {SOFT_NOISE, HARD_NOISE}
- `marble_type` · enum · {SOFT, SHARP, SHARPER}
- `noise_basis` · enum · {BLENDER_ORIGINAL, ORIGINAL_PERLIN, IMPROVED_PERLIN, VORONOI_F1, VORONOI_F2, VORONOI_F3, VORONOI_F4, VORONOI_F2_F1, VORONOI_CRACKLE, CELL_NOISE} — Noise basis used for turbulence
- `noise_basis_2` · enum · {SIN, SAW, TRI}
- `nabla` · float — Size of derivative offset used for calculating normal

## MusgraveTexture

Procedural musgrave texture

- `musgrave_type` · enum · {MULTIFRACTAL, RIDGED_MULTIFRACTAL, HYBRID_MULTIFRACTAL, FBM, HETERO_TERRAIN} — Fractal noise algorithm
- `dimension_max` · float — Highest fractal dimension
- `lacunarity` · float — Gap between successive frequencies
- `octaves` · float — Number of frequencies used
- `offset` · float — The fractal offset
- `gain` · float — The gain multiplier
- `noise_intensity` · float — Intensity of the noise
- `noise_scale` · float — Scaling for noise input
- `noise_basis` · enum · {BLENDER_ORIGINAL, ORIGINAL_PERLIN, IMPROVED_PERLIN, VORONOI_F1, VORONOI_F2, VORONOI_F3, VORONOI_F4, VORONOI_F2_F1, VORONOI_CRACKLE, CELL_NOISE} — Noise basis used for turbulence
- `nabla` · float — Size of derivative offset used for calculating normal

## StucciTexture

Procedural noise texture

- `turbulence` · float — Turbulence of the noise
- `noise_basis` · enum · {BLENDER_ORIGINAL, ORIGINAL_PERLIN, IMPROVED_PERLIN, VORONOI_F1, VORONOI_F2, VORONOI_F3, VORONOI_F4, VORONOI_F2_F1, VORONOI_CRACKLE, CELL_NOISE} — Noise basis used for turbulence
- `noise_scale` · float — Scaling for noise input
- `noise_type` · enum · {SOFT_NOISE, HARD_NOISE}
- `stucci_type` · enum · {PLASTIC, WALL_IN, WALL_OUT}

## Texture

Texture data-block used by materials, lights, worlds and brushes

- `type` · enum · {NONE, BLEND, CLOUDS, DISTORTED_NOISE, IMAGE, MAGIC, MARBLE, MUSGRAVE, NOISE, STUCCI, VORONOI, WOOD}
- `use_clamp` · boolean — Set negative texture RGB and intensity values to zero, for some uses like displacement this option can be disabled to get the full range
- `use_color_ramp` · boolean — Map the texture intensity to the color ramp. Note that the alpha value is used for image textures, enable "Calculate Alpha" for images without an alpha channel.
- `color_ramp` · pointer · -> ColorRamp · read-only
- `intensity` · float — Adjust the brightness of the texture
- `contrast` · float — Adjust the contrast of the texture
- `saturation` · float — Adjust the saturation of colors in the texture
- `factor_red` · float
- `factor_green` · float
- `factor_blue` · float
- `use_preview_alpha` · boolean — Show Alpha in Preview Render
- `use_nodes` · boolean — Make this a node-based texture
- `node_tree` · pointer · -> NodeTree · read-only — Node tree for node-based textures
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## TextureSlot

Texture slot defining the mapping and influence of a texture

- `texture` · pointer · -> Texture — Texture data-block used by this texture slot
- `name` · string · read-only — Texture slot name
- `offset` · float · [3] — Fine tune of the texture mapping X, Y and Z locations
- `scale` · float · [3] — Set scaling for the texture's X, Y and Z sizes
- `color` · float · [3] — Default color for textures that don't return RGB or when RGB to intensity is enabled
- `blend_type` · enum · {MIX, DARKEN, MULTIPLY, LIGHTEN, SCREEN, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE} — Mode used to apply the texture
- `default_value` · float — Value to use for Ref, Spec, Amb, Emit, Alpha, RayMir, TransLu and Hard
- `output_node` · enum · {DEFAULT} — Which output node to use, for node-based textures

## VoronoiTexture

Procedural voronoi texture

- `weight_1` · float — Voronoi feature weight 1
- `weight_2` · float — Voronoi feature weight 2
- `weight_3` · float — Voronoi feature weight 3
- `weight_4` · float — Voronoi feature weight 4
- `minkovsky_exponent` · float — Minkowski exponent
- `distance_metric` · enum · {DISTANCE, DISTANCE_SQUARED, MANHATTAN, CHEBYCHEV, MINKOVSKY_HALF, MINKOVSKY_FOUR, MINKOVSKY} — Algorithm used to calculate distance of sample points to feature points
- `color_mode` · enum · {INTENSITY, POSITION, POSITION_OUTLINE, POSITION_OUTLINE_INTENSITY}
- `noise_intensity` · float — Scales the intensity of the noise
- `noise_scale` · float — Scaling for noise input
- `nabla` · float — Size of derivative offset used for calculating normal

## WoodTexture

Procedural noise texture

- `noise_scale` · float — Scaling for noise input
- `turbulence` · float — Turbulence of the bandnoise and ringnoise types
- `noise_basis` · enum · {BLENDER_ORIGINAL, ORIGINAL_PERLIN, IMPROVED_PERLIN, VORONOI_F1, VORONOI_F2, VORONOI_F3, VORONOI_F4, VORONOI_F2_F1, VORONOI_CRACKLE, CELL_NOISE} — Noise basis used for turbulence
- `noise_type` · enum · {SOFT_NOISE, HARD_NOISE}
- `wood_type` · enum · {BANDS, RINGS, BANDNOISE, RINGNOISE}
- `noise_basis_2` · enum · {SIN, SAW, TRI}
- `nabla` · float — Size of derivative offset used for calculating normal
