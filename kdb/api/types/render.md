---
area: api-types
group: render
version: 5.2.2
---

# Blender 5.2.2 RNA types: render

## RenderEngine

Render engine

- `is_animation` · boolean
- `is_preview` · boolean
- `camera_override` · pointer · -> Object · read-only
- `layer_override` · boolean
- `resolution_x` · int · read-only
- `resolution_y` · int · read-only
- `temporary_directory` · string · read-only
- `render` · pointer · -> RenderSettings · read-only
- `use_highlight_tiles` · boolean
- `bl_idname` · string
- `bl_label` · string
- `bl_use_preview` · boolean — Render engine supports being used for rendering previews of materials, lights and worlds
- `bl_use_postprocess` · boolean — Apply compositing on render results
- `bl_use_eevee_viewport` · boolean — Uses EEVEE for viewport shading in Material Preview shading mode
- `bl_use_custom_freestyle` · boolean — Handles freestyle rendering on its own, instead of delegating it to EEVEE
- `bl_use_image_save` · boolean — Save images/movie to disk while rendering an animation. Disabling image saving is only supported when bl_use_postprocess is also disabled.
- `bl_use_gpu_context` · boolean — Enable OpenGL context for the render method, for engines that render using OpenGL
- `bl_use_shading_nodes_custom` · boolean — Don't expose Cycles and EEVEE shading nodes in the node editor user interface, so separate nodes can be used instead
- `bl_use_spherical_stereo` · boolean — Support spherical stereo camera models
- `bl_use_stereo_viewport` · boolean — Support rendering stereo 3D viewport
- `bl_use_materialx` · boolean — Use MaterialX for exporting materials to Hydra

## RenderLayer

- `name` · string · read-only — View layer name
- `use_solid` · boolean · read-only — Render Solid faces in this Layer
- `use_sky` · boolean · read-only — Render Sky in this Layer
- `use_ao` · boolean · read-only — Render Ambient Occlusion in this Layer
- `use_strand` · boolean · read-only — Render Strands in this Layer
- `use_volumes` · boolean · read-only — Render volumes in this Layer
- `use_motion_blur` · boolean · read-only — Render motion blur in this Layer, if enabled in the scene
- `use_grease_pencil` · boolean · read-only — Render Grease Pencil on this layer
- `use_pass_combined` · boolean · read-only — Deliver full combined RGBA buffer
- `use_pass_z` · boolean · read-only — Deliver depth values pass
- `use_pass_vector` · boolean · read-only — Deliver speed vector pass
- `use_pass_position` · boolean · read-only — Deliver position pass
- `use_pass_normal` · boolean · read-only — Deliver normal pass
- `use_pass_uv` · boolean · read-only — Deliver texture UV pass
- `use_pass_mist` · boolean · read-only — Deliver mist factor pass (0.0 to 1.0)
- `use_pass_object_index` · boolean · read-only — Deliver object index pass
- `use_pass_material_index` · boolean · read-only — Deliver material index pass
- `use_pass_shadow` · boolean · read-only — Deliver shadow pass
- `use_pass_ambient_occlusion` · boolean · read-only — Deliver Ambient Occlusion pass
- `use_pass_emit` · boolean · read-only — Deliver emission pass
- `use_pass_environment` · boolean · read-only — Deliver environment lighting pass
- `use_pass_diffuse_direct` · boolean · read-only — Deliver diffuse direct pass
- `use_pass_diffuse_indirect` · boolean · read-only — Deliver diffuse indirect pass
- `use_pass_diffuse_color` · boolean · read-only — Deliver diffuse color pass
- `use_pass_glossy_direct` · boolean · read-only — Deliver glossy direct pass
- `use_pass_glossy_indirect` · boolean · read-only — Deliver glossy indirect pass
- `use_pass_glossy_color` · boolean · read-only — Deliver glossy color pass
- `use_pass_transmission_direct` · boolean · read-only — Deliver transmission direct pass
- `use_pass_transmission_indirect` · boolean · read-only — Deliver transmission indirect pass
- `use_pass_transmission_color` · boolean · read-only — Deliver transmission color pass
- `use_pass_subsurface_direct` · boolean · read-only — Deliver subsurface direct pass
- `use_pass_subsurface_indirect` · boolean · read-only — Deliver subsurface indirect pass
- `use_pass_subsurface_color` · boolean · read-only — Deliver subsurface color pass
- `passes` · collection · [] of RenderPasses · read-only

## RenderPass

- `fullname` · string · read-only
- `name` · string · read-only
- `channel_id` · string · read-only
- `channels` · int · read-only
- `rect` · float
- `view_id` · int · read-only

## RenderResult

Result of rendering, including all layers and passes

- `resolution_x` · int · read-only
- `resolution_y` · int · read-only
- `layers` · collection · read-only
- `views` · collection · read-only

## RenderSettings

Rendering settings for a Scene data-block

- `image_settings` · pointer · -> ImageFormatSettings · read-only
- `resolution_x` · int — Number of horizontal pixels in the rendered image
- `resolution_y` · int — Number of vertical pixels in the rendered image
- `resolution_percentage` · int — Percentage scale for render resolution
- `preview_pixel_size` · enum · {AUTO, 1, 2, 4, 8} — Pixel size for viewport rendering
- `pixel_aspect_x` · float — Horizontal aspect ratio - for anamorphic or non-square pixel output
- `pixel_aspect_y` · float — Vertical aspect ratio - for anamorphic or non-square pixel output
- `ppm_factor` · float — The pixel density meta-data written to supported image formats. This value is multiplied by the PPM-base which defines the unit (typically inches or meters)
- `ppm_base` · float — The base unit for pixels per meter.
- `ffmpeg` · pointer · -> FFmpegSettings · read-only — FFmpeg related settings for the scene
- `fps` · int — Framerate, expressed in frames per second
- `fps_base` · float — Framerate base
- `frame_map_old` · int — Old mapping value in frames
- `frame_map_new` · int — How many frames the Map Old will last
- `dither_intensity` · float — Amount of dithering noise added to the rendered image to break up banding
- `filter_size` · float — Width over which the reconstruction filter combines samples
- `film_transparent` · boolean — World background is transparent, for compositing the render over another background
- `use_freestyle` · boolean — Draw stylized strokes using Freestyle
- `threads` · int — Maximum number of CPU cores to use simultaneously while rendering (for multi-core/CPU systems)
- `threads_mode` · enum · {AUTO, FIXED} — Determine the amount of render threads used
- `use_motion_blur` · boolean — Use multi-sampled 3D scene motion blur
- `motion_blur_shutter` · float — Time taken in frames between shutter open and close
- `motion_blur_position` · enum · {START, CENTER, END} — Offset for the shutter's time interval, allows to change the motion blur trails
- `motion_blur_shutter_curve` · pointer · -> CurveMapping · read-only — Curve defining the shutter's openness over time
- `hair_type` · enum · {STRAND, STRIP, CYLINDER} — Curves shape type
- `hair_subdiv` · int — Additional subdivision along the curves
- `use_high_quality_normals` · boolean — Use high quality tangent space at the cost of lower performance
- `anisotropic_filter` · enum · {FILTER_0, FILTER_2, FILTER_4, FILTER_8, FILTER_16} — Quality of anisotropic filtering in materials
- `use_border` · boolean — Render a user-defined render region, within the frame size
- `border_min_x` · float — Minimum X value for the render region
- `border_min_y` · float — Minimum Y value for the render region
- `border_max_x` · float — Maximum X value for the render region
- `border_max_y` · float — Maximum Y value for the render region
- `use_crop_to_border` · boolean — Crop the rendered frame to the defined render region size
- `use_placeholder` · boolean — Create empty placeholder files while rendering frames (similar to Unix 'touch')
- `use_overwrite` · boolean — Overwrite existing files while rendering
- `save_output` · boolean — Write frames to disk for animation renders
- `use_compositing` · boolean — Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene
- `use_sequencer` · boolean — Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist
- `use_file_extension` · boolean — Add the file format extensions to the rendered file name (eg: filename + .jpg)
- `file_extension` · string · read-only — The file extension used for saving renders
- `is_movie_format` · boolean · read-only — When true the format is a movie
- `use_lock_interface` · boolean — Lock interface during rendering in favor of giving more memory to the renderer
- `filepath` · string — Directory/name to save animations, # characters define the position and padding of frame numbers
- `use_render_cache` · boolean — Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)
- `use_stamp_time` · boolean — Include the rendered frame timecode as HH:MM:SS.FF in image metadata
- `use_stamp_date` · boolean — Include the current date in image/video metadata
- `use_stamp_frame` · boolean — Include the frame number in image metadata
- `use_stamp_frame_range` · boolean — Include the rendered frame range in image/video metadata
- `use_stamp_camera` · boolean — Include the name of the active camera in image metadata
- `use_stamp_lens` · boolean — Include the active camera's lens in image metadata
- `use_stamp_scene` · boolean — Include the name of the active scene in image/video metadata
- `use_stamp_note` · boolean — Include a custom note in image/video metadata
- `use_stamp_marker` · boolean — Include the name of the last marker in image metadata
- `use_stamp_filename` · boolean — Include the .blend filename in image/video metadata
- `use_stamp_sequencer_strip` · boolean — Include the name of the foreground sequence strip in image metadata
- `use_stamp_render_time` · boolean — Include the render time in image metadata
- `stamp_note_text` · string — Custom text to appear in the stamp note
- `use_stamp` · boolean — Render the stamp info text in the rendered image
- `use_stamp_labels` · boolean — Display stamp labels ("Camera" in front of camera name, etc.)
- `metadata_input` · enum · {SCENE, STRIPS} — Where to take the metadata from
- `use_stamp_memory` · boolean — Include the peak memory usage in image metadata
- `use_stamp_hostname` · boolean — Include the hostname of the machine that rendered the frame
- `stamp_font_size` · int — Size of the font used when rendering stamp text
- `stamp_foreground` · float · [4] — Color to use for stamp text
- `stamp_background` · float · [4] — Color to use behind stamp text
- `sequencer_gl_preview` · enum · {WIREFRAME, SOLID, MATERIAL, RENDERED} — Display method used in the sequencer view
- `use_sequencer_override_scene_strip` · boolean — Use Workbench render and world settings from the sequencer scene, instead of each strip's scene
- `use_single_layer` · boolean — Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line.
- `views` · collection · [] of RenderViews · read-only
- `stereo_views` · collection · read-only
- `use_multiview` · boolean — Use multiple views in the scene
- `views_format` · enum · {STEREO_3D, MULTIVIEW}
- `engine` · enum · {BLENDER_EEVEE} — Engine to use for rendering
- `has_multiple_engines` · boolean · read-only — More than one rendering engine is available
- `use_spherical_stereo` · boolean · read-only — Active render engine supports spherical stereo rendering
- `use_simplify` · boolean — Enable simplification of scene for quicker preview renders
- `simplify_subdivision` · int — Global maximum subdivision level
- `simplify_child_particles` · float — Global child particles percentage
- `simplify_subdivision_render` · int — Global maximum subdivision level during rendering
- `simplify_child_particles_render` · float — Global child particles percentage during rendering
- `simplify_volumes` · float — Resolution percentage of volume objects in viewport
- `use_simplify_normals` · boolean — Skip computing custom normals and face corner normals for displaying meshes in the viewport
- `use_texture_cache` · boolean — Load texture tiles at appropriate resolution on demand to reduce memory usage. This avoids loading all textures into memory, at the cost of extra disk space and some performance
- `use_auto_generate_texture_cache` · boolean — Automatically create tx files from image files when rendering, if the files do not exist or are outdated. The path to store the texture cache files is configured in the preferences
- `simplify_gpencil` · boolean — Simplify Grease Pencil drawing
- `simplify_gpencil_onplay` · boolean — Simplify Grease Pencil only during animation playback
- `simplify_gpencil_antialiasing` · boolean — Use Antialiasing to smooth stroke edges
- `simplify_gpencil_view_fill` · boolean — Display fill strokes in the viewport
- `simplify_gpencil_modifier` · boolean — Display modifiers
- `simplify_gpencil_shader_fx` · boolean — Display Shader Effects
- `simplify_gpencil_tint` · boolean — Display layer tint
- `use_persistent_data` · boolean — Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage
- `line_thickness_mode` · enum · {ABSOLUTE, RELATIVE} — Line thickness mode for Freestyle line drawing
- `line_thickness` · float — Line thickness in pixels
- `bake` · pointer · -> BakeSettings · read-only
- `compositor_device` · enum · {CPU, GPU} — Set how compositing is executed
- `compositor_precision` · enum · {AUTO, FULL} — The precision of compositor intermediate result
- `compositor_denoise_device` · enum · {AUTO, CPU, GPU} — The device to use to process the denoise nodes in the compositor
- `compositor_denoise_preview_quality` · enum · {HIGH, BALANCED, FAST} — The quality used by denoise nodes during viewport and interactive compositing if the nodes' quality option is set to Follow Scene
- `compositor_denoise_final_quality` · enum · {HIGH, BALANCED, FAST} — The quality used by denoise nodes during the compositing of final renders if the nodes' quality option is set to Follow Scene

## RenderSlot

Parameters defining the render slot

- `name` · string — Render slot name

## RenderSlots

Collection of render layers

- `active_index` · int — Active render slot of the image
- `active` · pointer · -> RenderSlot — Active render slot of the image

## RenderView

- `name` · string · read-only

## RenderViews

Collection of render views

- `active_index` · int — Active index in render view array
- `active` · pointer · -> SceneRenderView — Active Render View

## VolumeRender

Volume object render settings

- `precision` · enum · {FULL, HALF, VARIABLE} — Specify volume data precision. Lower values reduce memory consumption at the cost of detail.
- `space` · enum · {OBJECT, WORLD} — Specify volume density and step size in object or world space
- `step_size` · float — Distance between volume samples. Lower values render more detail at the cost of performance. If set to zero, the step size is automatically determined based on voxel size.
- `clipping` · float — Value under which voxels are considered empty space to optimize rendering
