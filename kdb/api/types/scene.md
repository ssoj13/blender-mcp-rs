---
area: api-types
group: scene
version: 5.2.2
---

# Blender 5.2.2 RNA types: scene

## Scene

Scene data-block, consisting in objects and defining time and render related settings

- `camera` · pointer · -> Object — Active camera, used for rendering the scene
- `background_set` · pointer · -> Scene — Background set scene
- `world` · pointer · -> World — World used for rendering the scene
- `objects` · collection · [] of SceneObjects · read-only
- `frame_current` · int — Current frame, to update animation data from Python frame_set() instead
- `frame_subframe` · float
- `frame_float` · float
- `frame_start` · int — First frame of the playback/rendering range
- `frame_end` · int — Final frame of the playback/rendering range
- `frame_step` · int — Number of frames to skip forward while rendering/playing back each frame
- `time_jump_unit` · enum · {FRAME, SECOND} — Which unit to use for time jumps in the timeline
- `time_jump_delta` · float — Number of frames or seconds to jump forward or backward
- `frame_current_final` · float · read-only — Current frame with subframe and time remapping applied
- `lock_frame_selection_to_range` · boolean — Don't allow frame to be selected with mouse outside of frame range
- `allow_preroll` · boolean — Allows playing back frames before the playback start frame
- `use_preview_range` · boolean — Use an alternative start/end frame range for animation playback and view renders
- `frame_preview_start` · int — Alternative start frame for UI playback
- `frame_preview_end` · int — Alternative end frame for UI playback
- `show_subframe` · boolean — Display and allow setting fractional frame values for the current frame
- `show_keys_from_selected_only` · boolean — Only include channels relating to selected objects and data
- `use_stamp_note` · string — User defined note for the render stamping
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `is_nla_tweakmode` · boolean · read-only — Whether there is any action referenced by NLA being edited (strictly read-only)
- `use_custom_simulation_range` · boolean — Use a simulation range that is different from the scene range for simulation nodes that don't override the frame range themselves
- `simulation_frame_start` · int — Frame at which simulations start
- `simulation_frame_end` · int — Frame at which simulations end
- `sync_mode` · enum · {NONE, FRAME_DROP, AUDIO_SYNC} — How to sync playback
- `playback_loop_mode` · enum · {INFINITE, STOP_END_FRAME, STOP_START_FRAME, RESTORE, BOUNCE} — What to do when playback reaches the last frame
- `compositing_node_group` · pointer · -> NodeTree — Compositor Nodes
- `use_nodes` · boolean — Enable the compositing node group.
- `sequence_editor` · pointer · -> SequenceEditor · read-only
- `keying_sets` · collection · [] of KeyingSets · read-only — Absolute Keying Sets for this Scene
- `keying_sets_all` · collection · [] of KeyingSetsAll · read-only — All Keying Sets available for use (Builtins and Absolute Keying Sets for this Scene)
- `rigidbody_world` · pointer · -> RigidBodyWorld · read-only
- `tool_settings` · pointer · -> ToolSettings · read-only
- `unit_settings` · pointer · -> UnitSettings · read-only — Unit editing settings
- `gravity` · float · [3] — Constant acceleration in a given direction
- `use_gravity` · boolean — Use global gravity for all dynamics
- `render` · pointer · -> RenderSettings · read-only
- `safe_areas` · pointer · -> DisplaySafeAreas · read-only
- `timeline_markers` · collection · [] of TimelineMarkers · read-only — Markers used in all timelines for the current scene
- `transform_orientation_slots` · collection · read-only
- `cursor` · pointer · -> View3DCursor · read-only
- `use_audio` · boolean — Play back of audio from Sequence Editor, otherwise mute audio
- `use_audio_scrub` · boolean — Play audio from Sequence Editor while scrubbing
- `audio_doppler_speed` · float — Speed of sound for Doppler effect calculation
- `audio_doppler_factor` · float — Pitch factor for Doppler effect calculation
- `audio_distance_model` · enum · {NONE, INVERSE, INVERSE_CLAMPED, LINEAR, LINEAR_CLAMPED, EXPONENT, EXPONENT_CLAMPED} — Distance model for distance attenuation calculation
- `audio_volume` · float — Audio volume
- `annotation` · pointer · -> Annotation — Data-block used for annotations in the 3D view
- `active_clip` · pointer · -> MovieClip — Active Movie Clip that can be used by motion tracking constraints or as a camera's background image
- `view_settings` · pointer · -> ColorManagedViewSettings · read-only — Color management settings applied on image before saving
- `display_settings` · pointer · -> ColorManagedDisplaySettings · read-only — Settings of device saved image would be displayed on
- `sequencer_colorspace_settings` · pointer · -> ColorManagedSequencerColorspaceSettings · read-only — Settings of color space sequencer is working in
- `view_layers` · collection · [] of ViewLayers · read-only
- `collection` · pointer · -> Collection · read-only — Scene root collection that owns all the objects and other collections instantiated in the scene
- `display` · pointer · -> SceneDisplay · read-only — Scene display settings for 3D viewport
- `eevee` · pointer · -> SceneEEVEE · read-only — EEVEE settings for the scene
- `grease_pencil_settings` · pointer · -> SceneGpencil · read-only — Grease Pencil settings for the scene
- `hydra` · pointer · -> SceneHydra · read-only — Hydra settings for the scene
- `cycles` · pointer · -> CyclesRenderSettings · read-only — Cycles render settings
- `cycles_curves` · pointer · -> CyclesCurveRenderSettings · read-only — Cycles curves rendering settings

## SceneDisplay

Scene display settings for 3D viewport

- `light_direction` · float · [3] — Direction of the light for shadows and highlights
- `shadow_shift` · float — Shadow termination angle
- `shadow_focus` · float — Shadow factor hardness
- `matcap_ssao_distance` · float — Distance of object that contribute to the cavity/edge effect
- `matcap_ssao_attenuation` · float — Attenuation constant
- `matcap_ssao_samples` · int — Number of samples
- `render_aa` · enum · {OFF, FXAA, 5, 8, 11, 16, 32} — Method of anti-aliasing when rendering final image
- `viewport_aa` · enum · {OFF, FXAA, 5, 8, 11, 16, 32} — Method of anti-aliasing when rendering 3d viewport
- `shading` · pointer · -> View3DShading · read-only — Shading settings for OpenGL render engine

## SceneEEVEE

Scene display settings for 3D viewport

- `gi_diffuse_bounces` · int — Number of times the light is reinjected inside light grids, 0 disable indirect diffuse light
- `gi_cubemap_resolution` · enum · {128, 256, 512, 1024, 2048, 4096} — Size of every cubemaps
- `gi_visibility_resolution` · enum · {8, 16, 32, 64} — Size of the shadow map applied to each irradiance sample
- `gi_glossy_clamp` · float — Clamp pixel intensity to reduce noise inside glossy reflections from reflection cubemaps (0 to disable)
- `gi_irradiance_pool_size` · enum · {16, 32, 64, 128, 256, 512, 1024} — Size of the irradiance pool, a bigger pool size allows for more irradiance grid in the scene but might not fit into GPU memory and decrease performance
- `taa_samples` · int — Number of samples, unlimited if 0
- `taa_render_samples` · int — Number of samples per pixel for rendering
- `use_taa_reprojection` · boolean — Denoise image using temporal reprojection (can leave some ghosting)
- `ray_tracing_method` · enum · {PROBE, SCREEN} — Select the tracing method used to find scene-ray intersections
- `use_shadow_jitter_viewport` · boolean — Enable jittered shadows on the viewport. (Jittered shadows are always enabled for final renders).
- `clamp_surface_direct` · float — If non-zero, the maximum value for lights contribution on a surface. Higher values will be scaled down to avoid too much noise and slow convergence at the cost of accuracy. Used by light objects.
- `clamp_surface_indirect` · float — If non-zero, the maximum value for indirect lighting on surface. Higher values will be scaled down to avoid too much noise and slow convergence at the cost of accuracy. Used by ray-tracing and light-probes.
- `clamp_volume_direct` · float — If non-zero, the maximum value for lights contribution in volumes. Higher values will be scaled down to avoid too much noise and slow convergence at the cost of accuracy. Used by light objects.
- `clamp_volume_indirect` · float — If non-zero, the maximum value for indirect lighting in volumes. Higher values will be scaled down to avoid too much noise and slow convergence at the cost of accuracy. Used by light-probes.
- `direct_light_intensity` · float — Scale the contribution of direct lighting
- `indirect_light_intensity` · float — Scale the contribution of indirect lighting
- `volumetric_start` · float — Start distance of the volumetric effect
- `volumetric_end` · float — End distance of the volumetric effect
- `volumetric_tile_size` · enum · {1, 2, 4, 8, 16} — Control the quality of the volumetric effects. Higher resolution uses more memory.
- `volumetric_samples` · int — Number of steps to compute volumetric effects. Higher step count increase VRAM usage and quality.
- `volumetric_sample_distribution` · float — Distribute more samples closer to the camera
- `volumetric_ray_depth` · int — Maximum surface intersection count used by the accurate volume intersection method. Will create artifact if it is exceeded. Higher count increases VRAM usage.
- `volumetric_light_clamp` · float — Maximum light contribution, reducing noise
- `use_volumetric_shadows` · boolean — Cast shadows from volumetric materials onto volumetric materials (Very expensive)
- `volumetric_shadow_samples` · int — Number of samples to compute volumetric shadowing
- `use_volume_custom_range` · boolean — Enable custom start and end clip distances for volume computation
- `use_fast_gi` · boolean — Use faster global illumination technique for high roughness surfaces
- `fast_gi_thickness_near` · float — Geometric thickness of the surfaces when computing fast GI and ambient occlusion. Reduces light leaking and missing contact occlusion.
- `fast_gi_quality` · float — Precision of the fast GI ray marching
- `fast_gi_step_count` · int — Amount of screen sample per GI ray
- `fast_gi_ray_count` · int — Amount of GI ray to trace for each pixel
- `fast_gi_method` · enum · {AMBIENT_OCCLUSION_ONLY, GLOBAL_ILLUMINATION} — Fast GI approximation method
- `fast_gi_distance` · float — If non-zero, the maximum distance at which other surfaces will contribute to the fast GI approximation
- `fast_gi_bias` · float — Bias the shading normal to reduce self intersection artifacts
- `fast_gi_resolution` · enum · {1, 2, 4, 8, 16} — Control the quality of the fast GI lighting. Higher resolution uses more memory.
- `bokeh_max_size` · float — Max size of the bokeh shape for the depth of field (lower is faster)
- `bokeh_threshold` · float — Brightness threshold for using sprite base depth of field
- `bokeh_neighbor_max` · float — Maximum brightness to consider when rejecting bokeh sprites based on neighborhood (lower is faster)
- `use_bokeh_jittered` · boolean — Jitter camera position to create accurate blurring using render samples (only for final render)
- `bokeh_overblur` · float — Apply blur to each jittered sample to reduce under-sampling artifacts
- `motion_blur_depth_scale` · float — Lower values will reduce background bleeding onto foreground elements
- `motion_blur_max` · int — Maximum blur distance a pixel can spread over
- `motion_blur_steps` · int — Controls accuracy of motion blur, more steps means longer render time
- `use_shadows` · boolean — Enable shadow casting from lights
- `shadow_pool_size` · enum · {16, 32, 64, 128, 256, 512, 1024, 1536, 2048} — Size of the shadow pool, a bigger pool size allows for more shadows in the scene but might not fit into GPU memory
- `shadow_ray_count` · int — Amount of shadow ray to trace for each light
- `shadow_step_count` · int — Amount of shadow map sample per shadow ray
- `light_threshold` · float — Minimum light intensity for a light to contribute to the lighting
- `use_overscan` · boolean — Internally render past the image border to avoid screen-space effects disappearing
- `overscan_size` · float — Percentage of render size to add as overscan to the internal render buffers
- `ray_tracing_options` · pointer · -> RaytraceEEVEE · read-only — EEVEE settings for tracing reflections
- `use_raytracing` · boolean — Enable the ray-tracing module
- `shadow_resolution_scale` · float — Resolution percentage of shadow maps

## SceneGpencil

Render settings

- `antialias_threshold` · float — Threshold for edge detection algorithm (higher values might over-blur some part of the image)
- `antialias_threshold_render` · float — Threshold for edge detection algorithm (higher values might over-blur some part of the image). Only applies to final render
- `aa_samples` · int — Number of supersampling anti-aliasing samples per pixel for final render
- `motion_blur_steps` · int — Controls accuracy of motion blur, more steps result in longer render time. Only used when Motion Blur is enabled. Set to 0 to disable motion blur for Grease Pencil

## SceneHydra

Scene Hydra render engine settings

- `export_method` · enum · {HYDRA, USD} — How to export the Blender scene to the Hydra render engine

## SceneRenderView

Render viewpoint for 3D stereo and multiview rendering

- `name` · string — Render view name
- `file_suffix` · string — Suffix added to the render images for this view
- `camera_suffix` · string — Suffix to identify the cameras to use, and added to the render images for this view
- `use` · boolean — Disable or enable the render view
