---
area: api-types
group: image
version: 5.2.2
---

# Blender 5.2.2 RNA types: image

## CameraBackgroundImage

Image and settings for display in the 3D View background

- `is_override_data` · boolean · read-only — In a local override camera, whether this background image comes from the linked reference camera, or is local to the override
- `source` · enum · {IMAGE, MOVIE_CLIP} — Data source used for background
- `image` · pointer · -> Image — Image displayed and edited in this space
- `clip` · pointer · -> MovieClip — Movie clip displayed and edited in this space
- `image_user` · pointer · -> ImageUser · read-only — Parameters defining which layer, pass and frame of the image is displayed
- `clip_user` · pointer · -> MovieClipUser · read-only — Parameters defining which frame of the movie clip is displayed
- `offset` · float · [2]
- `scale` · float — Scale the background image
- `rotation` · float — Rotation for the background image (ortho view only)
- `use_flip_x` · boolean — Flip the background image horizontally
- `use_flip_y` · boolean — Flip the background image vertically
- `alpha` · float — Image opacity to blend the image against the background color
- `show_expanded` · boolean — Show the details in the user interface
- `use_camera_clip` · boolean — Use movie clip from active scene camera
- `show_background_image` · boolean — Show this image as background
- `show_on_foreground` · boolean — Show this image in front of objects in viewport
- `display_depth` · enum · {BACK, FRONT} — Display under or over everything
- `frame_method` · enum · {STRETCH, FIT, CROP} — How the image fits in the camera frame

## Image

Image data-block referencing an external or packed image

- `filepath` · string — Image/Movie file name
- `filepath_raw` · string — Image/Movie file name (without data refreshing)
- `file_format` · enum · {AVIF, JPEG, OPEN_EXR, PNG, WEBP, BMP, CINEON, DPX, IRIS, JPEG2000, HDR, TARGA, TARGA_RAW, TIFF, OPEN_EXR_MULTILAYER, FFMPEG} — Format used for re-saving this file
- `source` · enum · {FILE, SEQUENCE, MOVIE, GENERATED, VIEWER, TILED} — Where the image comes from
- `type` · enum · {IMAGE, MULTILAYER, UV_TEST, RENDER_RESULT, COMPOSITING} · read-only — How to generate the image
- `packed_file` · pointer · -> PackedFile · read-only — First packed file of the image
- `packed_files` · collection · read-only — Collection of packed images
- `use_view_as_render` · boolean — Apply render part of display transformation when displaying this image on the screen
- `use_deinterlace` · boolean — Deinterlace movie file on load
- `use_multiview` · boolean — Use Multiple Views (when available)
- `is_stereo_3d` · boolean · read-only — Image has left and right views
- `is_multiview` · boolean · read-only — Image has more than one view
- `is_dirty` · boolean · read-only — Image has changed and is not saved
- `generated_type` · enum · {BLANK, UV_GRID, COLOR_GRID} — Generated image type
- `generated_width` · int — Generated image width
- `generated_height` · int — Generated image height
- `use_generated_float` · boolean — Generate floating-point buffer
- `generated_color` · float · [4] — Fill color for the generated image
- `display_aspect` · float · [2] — Display Aspect for this image, does not affect rendering
- `render_slots` · collection · [] of RenderSlots · read-only — Render slots of the image
- `tiles` · collection · [] of UDIMTiles · read-only — Tiles of the image
- `has_data` · boolean · read-only — True if the image data is loaded into memory
- `depth` · int · read-only — Image bit depth
- `size` · int · [2] · read-only — Width and height of the image buffer in pixels, zero when image data cannot be loaded
- `resolution` · float · [2] — X/Y pixels per meter, for the image buffer
- `frame_duration` · int · read-only — Duration (in frames) of the image (1 when not a video/sequence)
- `pixels` · float — Image buffer pixels in floating-point values
- `channels` · int · read-only — Number of channels in pixels buffer
- `is_float` · boolean · read-only — True if this image is stored in floating-point buffer
- `colorspace_settings` · pointer · -> ColorManagedInputColorspaceSettings · read-only — Input color space settings
- `alpha_mode` · enum · {STRAIGHT, PREMUL, CHANNEL_PACKED, NONE} — Representation of alpha in the image file, to convert to and from when saving and loading the image
- `use_half_precision` · boolean — Use 16 bits per channel to lower the memory usage during rendering.
Note: Not supported by Cycles
- `seam_margin` · int — Margin to take into account when fixing UV seams during painting. Higher number would improve seam-fixes for mipmaps, but decreases performance.
- `views_format` · enum · {INDIVIDUAL, STEREO_3D} — Mode to load image views
- `stereo_3d_format` · pointer · -> Stereo3dFormat · read-only — Settings for stereo 3d

## ImageFormatSettings

Settings for image formats

- `media_type` · enum · {IMAGE, MULTI_LAYER_IMAGE, VIDEO} — The type of media to save
- `file_format` · enum · {AVIF, JPEG, OPEN_EXR, PNG, WEBP, BMP, CINEON, DPX, IRIS, JPEG2000, HDR, TARGA, TARGA_RAW, TIFF, OPEN_EXR_MULTILAYER, FFMPEG} — File format to save the rendered images as
- `color_mode` · enum · {BW, RGB, RGBA} — Choose BW for saving grayscale images, RGB for saving red, green and blue channels, and RGBA for saving red, green, blue and alpha channels
- `color_depth` · enum · {8, 10, 12, 16, 32} — Bit depth per channel
- `quality` · int — Quality for image formats that support lossy compression
- `compression` · int — Amount of time to determine best compression: 0 = no compression with fast file output, 100 = maximum lossless compression with slow file output
- `use_preview` · boolean — When rendering animations, save JPG preview images in same directory
- `exr_codec` · enum · {NONE, ZIP, PIZ, DWAA, DWAB, HTJ2K, ZIPS, RLE, PXR24, B44, B44A} — Compression codec settings for OpenEXR
- `use_exr_interleave` · boolean — Use legacy interleaved storage of views, layers and passes for compatibility with applications that do not support more efficient multi-part OpenEXR files.
- `use_jpeg2k_ycc` · boolean — Save luminance-chrominance-chrominance channels instead of RGB colors
- `use_jpeg2k_cinema_preset` · boolean — Use OpenJPEG Cinema Preset
- `use_jpeg2k_cinema_48` · boolean — Use OpenJPEG Cinema Preset (48fps)
- `jpeg2k_codec` · enum · {JP2, J2K} — Codec settings for JPEG 2000
- `tiff_codec` · enum · {NONE, DEFLATE, LZW, PACKBITS} — Compression mode for TIFF
- `use_cineon_log` · boolean — Convert to logarithmic color space
- `cineon_black` · int — Log conversion reference blackpoint
- `cineon_white` · int — Log conversion reference whitepoint
- `cineon_gamma` · float — Log conversion gamma
- `views_format` · enum · {INDIVIDUAL, STEREO_3D, MULTIVIEW} — Format of multiview media
- `stereo_3d_format` · pointer · -> Stereo3dFormat · read-only — Settings for stereo 3D
- `color_management` · enum · {FOLLOW_SCENE, OVERRIDE} — Which color management settings to use for file saving
- `view_settings` · pointer · -> ColorManagedViewSettings · read-only — Color management settings applied on image before saving
- `display_settings` · pointer · -> ColorManagedDisplaySettings · read-only — Settings of device saved image would be displayed on
- `linear_colorspace_settings` · pointer · -> ColorManagedInputColorspaceSettings · read-only — Output color space settings
- `has_linear_colorspace` · boolean · read-only — File format expects linear color space

## ImagePackedFile

- `packed_file` · pointer · -> PackedFile · read-only
- `filepath` · string
- `view` · int · read-only
- `tile_number` · int · read-only

## ImagePaint

Properties of image and texture painting mode

- `use_occlude` · boolean — Only paint onto the faces directly under the brush (slower)
- `use_backface_culling` · boolean — Ignore faces pointing away from the view (faster)
- `use_normal_falloff` · boolean — Paint most on faces pointing towards the view
- `use_stencil_layer` · boolean — Set the mask layer from the UV map buttons
- `invert_stencil` · boolean — Invert the stencil layer
- `stencil_image` · pointer · -> Image — Image used as stencil
- `canvas` · pointer · -> Image — Image used as canvas
- `clone_image` · pointer · -> Image — Image used as clone source
- `stencil_color` · float · [3] — Stencil color in the viewport
- `dither` · float — Amount of dithering when painting on byte images
- `use_clone_layer` · boolean — Use another UV map as clone source, otherwise use the 3D cursor as the source
- `seam_bleed` · int — Extend paint beyond the faces' UVs to reduce seams (in pixels, slower)
- `normal_angle` · int — Paint most on faces pointing towards the view according to this angle
- `screen_grab_size` · int · [2] — Size to capture the image for re-projecting
- `mode` · enum · {MATERIAL, IMAGE} — Mode of operation for projection painting
- `interpolation` · enum · {LINEAR, CLOSEST} — Texture filtering type
- `missing_uvs` · boolean · read-only — A UV layer is missing on the mesh
- `missing_materials` · boolean · read-only — The mesh is missing materials
- `missing_stencil` · boolean · read-only — Image Painting does not have a stencil
- `missing_texture` · boolean · read-only — Image Painting does not have a texture to paint on
- `clone_alpha` · float — Opacity of clone image display
- `clone_offset` · float · [2]

## ImagePreview

Preview image and icon

- `is_image_custom` · boolean — True if this preview image has been modified by py script, and is no more auto-generated by Blender
- `image_size` · int · [2] — Width and height in pixels
- `image_pixels` · int — Image pixels, as bytes (always 32-bit RGBA)
- `image_pixels_float` · float — Image pixels components, as floats (RGBA concatenated values)
- `is_icon_custom` · boolean — True if this preview icon has been modified by py script, and is no more auto-generated by Blender
- `icon_size` · int · [2] — Width and height in pixels
- `icon_pixels` · int — Icon pixels, as bytes (always 32-bit RGBA)
- `icon_pixels_float` · float — Icon pixels components, as floats (RGBA concatenated values)
- `icon_id` · int · read-only — Unique integer identifying this preview as an icon (zero means invalid)

## ImageTexture

- `use_interpolation` · boolean — Interpolate pixels using selected filter
- `use_flip_axis` · boolean — Flip the texture's X and Y axis
- `use_alpha` · boolean — Use the alpha channel information in the image
- `use_calculate_alpha` · boolean — Calculate an alpha channel based on RGB values in the image
- `invert_alpha` · boolean — Invert all the alpha values in the image
- `filter_size` · float — Multiply the filter size used by interpolation
- `extension` · enum · {EXTEND, CLIP, CLIP_CUBE, REPEAT, CHECKER} — How the image is extrapolated past its original bounds
- `repeat_x` · int — Repetition multiplier in the X direction
- `repeat_y` · int — Repetition multiplier in the Y direction
- `use_mirror_x` · boolean — Mirror the image repetition on the X direction
- `use_mirror_y` · boolean — Mirror the image repetition on the Y direction
- `use_checker_odd` · boolean — Odd checker tiles
- `use_checker_even` · boolean — Even checker tiles
- `checker_distance` · float — Distance between checker tiles
- `crop_min_x` · float — Minimum X value to crop the image
- `crop_min_y` · float — Minimum Y value to crop the image
- `crop_max_x` · float — Maximum X value to crop the image
- `crop_max_y` · float — Maximum Y value to crop the image
- `image` · pointer · -> Image
- `image_user` · pointer · -> ImageUser · read-only — Parameters defining which layer, pass and frame of the image is displayed
- `use_normal_map` · boolean — Use image RGB values for normal mapping

## ImageUser

Parameters defining how an Image data-block is used by another data-block

- `use_auto_refresh` · boolean — Always refresh image on frame changes
- `frame_current` · int — Current frame number in image sequence or movie
- `use_cyclic` · boolean — Cycle the images in the movie
- `frame_duration` · int — Number of images of a movie to use
- `frame_offset` · int — Offset the number of the frame to use in the animation
- `frame_start` · int — Global starting frame of the movie/sequence, assuming first picture has a #1
- `multilayer_layer` · int · read-only — Layer in multilayer image
- `multilayer_pass` · int · read-only — Pass in multilayer image
- `multilayer_view` · int · read-only — View in multilayer image
- `tile` · int — Tile in tiled image

## SpaceImageEditor

Image and UV editor space data

- `show_region_tool_header` · boolean
- `show_region_toolbar` · boolean
- `show_region_ui` · boolean
- `show_region_hud` · boolean
- `show_region_asset_shelf` · boolean — Display a region with assets that may currently be relevant (such as brushes in paint modes, or poses in Pose Mode)
- `image` · pointer · -> Image — Image displayed and edited in this space
- `image_user` · pointer · -> ImageUser · read-only — Parameters defining which layer, pass and frame of the image is displayed
- `scopes` · pointer · -> Scopes · read-only — Scopes to visualize image statistics
- `use_image_pin` · boolean — Display current image regardless of object selection
- `sample_histogram` · pointer · -> Histogram · read-only — Sampled colors along line
- `zoom` · float · [2] · read-only — Zoom factor
- `zoom_percentage` · float — Zoom percentage
- `show_repeat` · boolean — Display the image repeated outside of the main view
- `show_annotation` · boolean — Show annotations for this view
- `display_channels` · enum · {COLOR_ALPHA, COLOR, ALPHA, Z_BUFFER, RED, GREEN, BLUE} — Channels of the image to display
- `show_stereo_3d` · boolean — Display the image in Stereo 3D
- `show_sequencer_scene` · boolean — Display the render result for the sequencer scene instead of the active scene
- `uv_editor` · pointer · -> SpaceUVEditor · read-only — UV editor settings
- `mode` · enum · {VIEW, UV, PAINT, MASK} — Editing context being displayed
- `ui_mode` · enum · {VIEW, PAINT, MASK} — Editing context being displayed
- `cursor_location` · float · [2] — 2D cursor location for this view
- `pivot_point` · enum · {BOUNDING_BOX_CENTER, CURSOR, INDIVIDUAL_ORIGINS, MEDIAN_POINT, ACTIVE_ELEMENT} — Rotation/Scaling Pivot
- `annotation` · pointer · -> Annotation — Annotation data for this space
- `use_realtime_update` · boolean — Update other affected window spaces automatically to reflect changes during interactive operations such as transform
- `show_render` · boolean · read-only — Show render related properties
- `show_paint` · boolean · read-only — Show paint related properties
- `show_uvedit` · boolean · read-only — Show UV editing related properties
- `show_maskedit` · boolean · read-only — Show Mask editing related properties
- `show_gizmo` · boolean — Show gizmos of all types
- `show_gizmo_navigate` · boolean — Viewport navigation gizmo
- `show_gizmo_active_node` · boolean — Context sensitive gizmo for the active node
- `overlay` · pointer · -> SpaceImageOverlay · read-only — Settings for display of overlays in the UV/Image editor
- `mask` · pointer · -> Mask — Mask displayed and edited in this space
- `mask_display_type` · enum · {OUTLINE, DASH, BLACK, WHITE} — Display type for mask splines
- `show_mask_spline` · boolean
- `show_mask_overlay` · boolean
- `mask_overlay_mode` · enum · {ALPHACHANNEL, COMBINED} — Overlay mode of rasterized mask
- `blend_factor` · float — Overlay blending factor of rasterized mask

## SpaceImageOverlay

Settings for display of overlays in the UV/Image editor

- `show_overlays` · boolean — Display overlays like UV Maps and Metadata
- `show_grid_background` · boolean — Show the grid background and borders
- `show_render_size` · boolean — Display the region of the final render
- `show_text_info` · boolean — Display overlay text
- `passepartout_alpha` · float — Opacity of the darkened overlay outside the render region

## ThemeImageEditor

Theme settings for the Image Editor

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `grid` · float · [4]
- `vertex` · float · [3]
- `vertex_select` · float · [3]
- `vertex_size` · int
- `face` · float · [4]
- `face_select` · float · [4]
- `face_mode_select` · float · [4]
- `facedot_size` · int
- `editmesh_active` · float · [4]
- `wire_edit` · float · [3]
- `edge_width` · int
- `edge_select` · float · [3]
- `scope_back` · float · [4]
- `preview_stitch_face` · float · [4]
- `preview_stitch_edge` · float · [4]
- `preview_stitch_vert` · float · [4]
- `preview_stitch_stitchable` · float · [4]
- `preview_stitch_unstitchable` · float · [4]
- `preview_stitch_active` · float · [4]
- `uv_shadow` · float · [4]
- `metadatabg` · float · [3]
- `metadatatext` · float · [3]
