---
area: api-types
group: world
version: 5.2.2
---

# Blender 5.2.2 RNA types: world

## World

World data-block describing the environment and ambient lighting of a scene

- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `use_eevee_finite_volume` · boolean — The world's volume used to be rendered by EEVEE Legacy. Conversion is needed for it to render properly.
- `color` · float · [3] — Color of the background
- `light_settings` · pointer · -> WorldLighting · read-only — World lighting settings
- `mist_settings` · pointer · -> WorldMistSettings · read-only — World mist settings
- `node_tree` · pointer · -> NodeTree · read-only — Node tree for node based worlds
- `use_nodes` · boolean — Use shader nodes to render the world
- `lightgroup` · string — Lightgroup that the world belongs to
- `probe_resolution` · enum · {128, 256, 512, 1024, 2048, 4096} — Resolution when baked to a texture
- `sun_threshold` · float — If non-zero, the maximum value for world contribution that will be recorded inside the world light probe. The excess contribution is converted to a sun light. This reduces the light bleeding caused by very bright light sources.
- `sun_angle` · float — Angular diameter of the Sun as seen from the Earth
- `use_sun_shadow` · boolean — Enable sun shadow casting
- `sun_shadow_maximum_resolution` · float — Maximum size of a shadow map pixel. Higher values use less memory at the cost of shadow quality.
- `sun_shadow_filter_radius` · float — Blur shadow aliasing using Percentage Closer Filtering
- `use_sun_shadow_jitter` · boolean — Enable jittered soft shadows to increase shadow precision (disabled in viewport unless enabled in the render settings). Has a high performance impact.
- `sun_shadow_jitter_overblur` · float — Apply shadow tracing to each jittered sample to reduce under-sampling artifacts
- `cycles` · pointer · -> CyclesWorldSettings · read-only — Cycles world settings
- `cycles_visibility` · pointer · -> CyclesVisibilitySettings · read-only — Cycles visibility settings

## WorldMistSettings

Mist settings for a World data-block

- `use_mist` · boolean — Occlude objects with the environment color as they are further away
- `intensity` · float — Overall minimum intensity of the mist effect
- `start` · float — Starting distance of the mist, measured from the camera
- `depth` · float — Distance over which the mist effect fades in
- `height` · float — Control how much mist density decreases with height
- `falloff` · enum · {QUADRATIC, LINEAR, INVERSE_QUADRATIC} — Type of transition used to fade mist
