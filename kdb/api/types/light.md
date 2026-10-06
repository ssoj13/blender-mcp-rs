---
area: api-types
group: light
version: 5.2.2
---

# Blender 5.2.2 RNA types: light

## AreaLight

Directional area Light

- `energy` · float — Light energy emitted over the entire area of the light in all directions, in units of radiant power (W)
- `shadow_buffer_clip_start` · float — Shadow map clip start, below which objects will not generate shadows
- `shadow_soft_size` · float — Light size for ray shadow sampling (Raytraced shadows)
- `shadow_filter_radius` · float — Blur shadow aliasing using Percentage Closer Filtering
- `shadow_maximum_resolution` · float — Minimum size of a shadow map pixel. Higher values use less memory at the cost of shadow quality.
- `use_shadow_jitter` · boolean — Enable jittered soft shadows to increase shadow precision (disabled in viewport unless enabled in the render settings). Has a high performance impact.
- `shadow_jitter_overblur` · float — Apply shadow tracing to each jittered sample to reduce under-sampling artifacts
- `use_absolute_resolution` · boolean — Limit the resolution at 1 unit from the light origin instead of relative to the shadowed pixel
- `shape` · enum · {SQUARE, RECTANGLE, DISK, ELLIPSE} — Shape of the area Light
- `size` · float — Size of the area of the area light, X direction size for rectangle shapes
- `size_y` · float — Size of the area of the area light in the Y direction for rectangle shapes
- `spread` · float — How widely the emitted light fans out, as in the case of a gridded softbox

## CollectionLightLinking

Light linking settings of objects and children collections of a collection

- `link_state` · enum · {INCLUDE, EXCLUDE} — Light or shadow receiving state of the object or collection

## Light

Light data-block for lighting a scene

- `type` · enum · {POINT, SUN, SPOT, AREA} — Type of light
- `use_temperature` · boolean — Use blackbody temperature to define a natural light color
- `color` · float · [3] — Light color
- `temperature` · float — Light color temperature in Kelvin
- `temperature_color` · float · [3] · read-only — Color from Temperature
- `specular_factor` · float — Specular reflection multiplier
- `diffuse_factor` · float — Diffuse reflection multiplier
- `transmission_factor` · float — Transmission light multiplier
- `volume_factor` · float — Volume light multiplier
- `use_custom_distance` · boolean — Use custom attenuation distance instead of global light threshold
- `cutoff_distance` · float — Distance at which the light influence will be set to 0
- `use_shadow` · boolean
- `exposure` · float — Scales the power of the light exponentially, multiplying the intensity by 2^exposure
- `normalize` · boolean — Normalize intensity by light area, for consistent total light output regardless of size and shape
- `node_tree` · pointer · -> NodeTree · read-only — Node tree for node based lights
- `use_nodes` · boolean — Use shader nodes to render the light
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `cycles` · pointer · -> CyclesLightSettings · read-only — Cycles light settings

## LightProbe

Light Probe data-block for lighting capture objects

- `type` · enum · {SPHERE, PLANE, VOLUME} · read-only — Type of light probe
- `clip_start` · float — Probe clip start, below which objects will not appear in reflections
- `show_clip` · boolean — Show the clipping distances in the 3D view
- `show_influence` · boolean — Show the influence volume in the 3D view
- `influence_distance` · float — Influence distance of the probe
- `visibility_buffer_bias` · float — Bias for reducing self shadowing (Deprecated)
- `visibility_bleed_bias` · float — Bias for reducing light-bleed on variance shadow maps (Deprecated)
- `visibility_blur` · float — Filter size of the visibility blur (Deprecated)
- `visibility_collection` · pointer · -> Collection — Restrict objects visible for this probe (Deprecated)
- `invert_visibility_collection` · boolean — Invert visibility collection (Deprecated)
- `show_data` · boolean — Deprecated, use use_data_display instead
- `use_data_display` · boolean — Display sampled data in the viewport to debug captured light
- `data_display_size` · float — Viewport display size of the sampled data
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## LightProbePlane

Light probe that captures incoming light from a single direction on a plane

- `parallax_distance` · float — Amount of parallax to use for reflections on Blended materials or Shader To RGB evaluation

## LightProbeSphere

Light probe that captures precise lighting from all directions at a single point in space

- `influence_type` · enum · {ELIPSOID, BOX} — Type of influence volume
- `falloff` · float — Control how fast the probe influence decreases
- `clip_end` · float — Probe clip end, beyond which objects will not appear in reflections
- `use_custom_parallax` · boolean — Enable custom settings for the parallax correction volume
- `show_parallax` · boolean — Show the parallax correction volume in the 3D view
- `parallax_type` · enum · {ELIPSOID, BOX} — Type of parallax volume
- `parallax_distance` · float — Lowest corner of the parallax bounding box

## LightProbeVolume

Light probe that captures low frequency lighting inside a volume

- `intensity` · float — Modify the intensity of the lighting captured by this probe
- `resolution_x` · int — Number of samples along the x axis of the volume
- `resolution_y` · int — Number of samples along the y axis of the volume
- `resolution_z` · int — Number of samples along the z axis of the volume
- `capture_distance` · float — Distance around the probe volume that will be considered during the bake
- `normal_bias` · float — Offset sampling of the irradiance grid in the surface normal direction to reduce light bleeding
- `view_bias` · float — Offset sampling of the irradiance grid in the viewing direction to reduce light bleeding
- `facing_bias` · float — Smoother irradiance interpolation but introduce light bleeding
- `bake_samples` · int — Number of ray directions to evaluate when baking
- `surface_bias` · float — Moves capture points away from surfaces to prevent artifacts
- `escape_bias` · float — Distance to search for valid capture positions to prevent lighting artifacts
- `surfel_density` · int — Number of surfels to spawn in one local unit distance (higher values improve quality)
- `validity_threshold` · float — Ratio of front-facing surface hits under which a grid sample will not be considered for lighting
- `dilation_threshold` · float — Ratio of front-facing surface hits under which a grid sample will reuse neighbors grid sample lighting
- `dilation_radius` · float — Radius in grid sample to search valid grid samples to copy into invalid grid samples
- `capture_world` · boolean — Bake incoming light from the world instead of just the visibility for more accurate lighting, but lose correct blending to surrounding irradiance volumes
- `capture_indirect` · boolean — Bake light bounces from light sources for more accurate lighting
- `capture_emission` · boolean — Bake emissive surfaces for more accurate lighting
- `clamp_direct` · float — Clamp the direct lighting intensity to reduce noise (0 to disable)
- `clamp_indirect` · float — Clamp the indirect lighting intensity to reduce noise (0 to disable)

## Lightgroup

- `name` · string — Name of the Lightgroup

## PointLight

Omnidirectional point Light

- `use_soft_falloff` · boolean — Apply falloff to avoid sharp edges when the light geometry intersects with other objects
- `energy` · float — Light energy emitted over the entire area of the light in all directions, in units of radiant power (W)
- `shadow_buffer_clip_start` · float — Shadow map clip start, below which objects will not generate shadows
- `shadow_soft_size` · float — Light size for ray shadow sampling (Raytraced shadows)
- `shadow_filter_radius` · float — Blur shadow aliasing using Percentage Closer Filtering
- `shadow_maximum_resolution` · float — Minimum size of a shadow map pixel. Higher values use less memory at the cost of shadow quality.
- `use_shadow_jitter` · boolean — Enable jittered soft shadows to increase shadow precision (disabled in viewport unless enabled in the render settings). Has a high performance impact.
- `shadow_jitter_overblur` · float — Apply shadow tracing to each jittered sample to reduce under-sampling artifacts
- `use_absolute_resolution` · boolean — Limit the resolution at 1 unit from the light origin instead of relative to the shadowed pixel

## SpotLight

Directional cone Light

- `energy` · float — The energy this light would emit over its entire area if it wasn't limited by the spot angle, in units of radiant power (W)
- `shadow_buffer_clip_start` · float — Shadow map clip start, below which objects will not generate shadows
- `shadow_soft_size` · float — Light size for ray shadow sampling (Raytraced shadows)
- `shadow_filter_radius` · float — Blur shadow aliasing using Percentage Closer Filtering
- `shadow_maximum_resolution` · float — Minimum size of a shadow map pixel. Higher values use less memory at the cost of shadow quality.
- `use_shadow_jitter` · boolean — Enable jittered soft shadows to increase shadow precision (disabled in viewport unless enabled in the render settings). Has a high performance impact.
- `shadow_jitter_overblur` · float — Apply shadow tracing to each jittered sample to reduce under-sampling artifacts
- `use_absolute_resolution` · boolean — Limit the resolution at 1 unit from the light origin instead of relative to the shadowed pixel
- `use_square` · boolean — Cast a square spot light shape
- `spot_blend` · float — The softness of the spotlight edge
- `spot_size` · float — Angular diameter of the spotlight beam
- `show_cone` · boolean — Display transparent cone in 3D view to visualize which objects are contained in it
- `use_soft_falloff` · boolean — Apply falloff to avoid sharp edges when the light geometry intersects with other objects

## StudioLight

Studio light

- `index` · int · read-only
- `is_user_defined` · boolean · read-only
- `has_specular_highlight_pass` · boolean · read-only — Studio light image file has separate "diffuse" and "specular" passes
- `type` · enum · {STUDIO, WORLD, MATCAP} · read-only
- `name` · string · read-only
- `path` · string · read-only
- `solid_lights` · collection · read-only — Lights used to display objects in solid draw mode
- `light_ambient` · float · [3] · read-only — Color of the ambient light that uniformly lit the scene

## SunLight

Constant direction parallel ray Light

- `angle` · float — Angular diameter of the Sun as seen from the Earth
- `energy` · float — Sunlight strength in watts per meter squared (W/m²)
- `shadow_buffer_clip_start` · float — Shadow map clip start, below which objects will not generate shadows
- `shadow_soft_size` · float — Light size for ray shadow sampling (Raytraced shadows)
- `shadow_filter_radius` · float — Blur shadow aliasing using Percentage Closer Filtering
- `shadow_maximum_resolution` · float — Minimum size of a shadow map pixel. Higher values use less memory at the cost of shadow quality.
- `use_shadow_jitter` · boolean — Enable jittered soft shadows to increase shadow precision (disabled in viewport unless enabled in the render settings). Has a high performance impact.
- `shadow_jitter_overblur` · float — Apply shadow tracing to each jittered sample to reduce under-sampling artifacts
- `shadow_cascade_max_distance` · float — End distance of the cascaded shadow map (only in perspective view)
- `shadow_cascade_count` · int — Number of texture used by the cascaded shadow map
- `shadow_cascade_exponent` · float — Higher value increase resolution towards the viewpoint
- `shadow_cascade_fade` · float — How smooth is the transition between each cascade

## UserSolidLight

Light used for Studio lighting in solid shading mode

- `use` · boolean — Enable this light in solid shading mode
- `smooth` · float — Smooth the lighting from this light
- `direction` · float · [3] — Direction that the light is shining
- `specular_color` · float · [3] — Color of the light's specular highlight
- `diffuse_color` · float · [3] — Color of the light's diffuse highlight

## WorldLighting

Lighting for a World data-block

- `ao_factor` · float — Factor for ambient occlusion blending
- `distance` · float — Length of rays, defines how far away other faces give occlusion effect
