---
area: api-ops
group: image
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.image

- `bpy.ops.image.add_render_slot()` — Add a new render slot
- `bpy.ops.image.change_frame(frame: int)` — Interactively change the current frame number
- `bpy.ops.image.clear_render_border()` — Clear the boundaries of the render region and disable render region
- `bpy.ops.image.clear_render_slot()` — Clear the currently selected render slot
- `bpy.ops.image.clipboard_copy()` — Copy the image to the clipboard
- `bpy.ops.image.clipboard_paste()` — Paste new image from the clipboard
- `bpy.ops.image.convert_to_mesh_plane(interpolation: enum{Linear, Closest, Cubic, Smart}, extension: enum{CLIP, EXTEND, REPEAT}, use_auto_refresh: boolean, relative: boolean, shader: enum{PRINCIPLED, SHADELESS, EMISSION}, emit_strength: float, use_transparency: boolean, render_method: enum{DITHERED, BLENDED}, use_backface_culling: boolean, show_transparent_back: boolean, overwrite_material: boolean, name_from: enum{OBJECT, IMAGE}, delete_ref: boolean)` — Convert selected reference images to textured mesh plane
- `bpy.ops.image.curves_point_set(point: enum{BLACK_POINT, WHITE_POINT}, size: int)` — Set black point or white point for curves
- `bpy.ops.image.cycle_render_slot(reverse: boolean)` — Cycle through all non-void render slots
- `bpy.ops.image.external_edit(filepath: string)` — Edit image in an external application
- `bpy.ops.image.file_browse(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, relative_path: boolean, show_multiview: boolean, use_multiview: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Open an image file browser, hold Shift to open the file, Alt to browse containing directory
- `bpy.ops.image.flip(use_flip_x: boolean, use_flip_y: boolean)` — Flip the image
- `bpy.ops.image.import_as_mesh_planes(interpolation: enum{Linear, Closest, Cubic, Smart}, extension: enum{CLIP, EXTEND, REPEAT}, use_auto_refresh: boolean, relative: boolean, shader: enum{PRINCIPLED, SHADELESS, EMISSION}, emit_strength: float, use_transparency: boolean, render_method: enum{DITHERED, BLENDED}, use_backface_culling: boolean, show_transparent_back: boolean, overwrite_material: boolean, filepath: string, align: enum{WORLD, VIEW, CURSOR}, location: float, rotation: float, files: collection, directory: string, filter_image: boolean, filter_movie: boolean, filter_folder: boolean, force_reload: boolean, image_sequence: boolean, offset: boolean, offset_axis: enum{+X, +Y, +Z, -X, -Y, -Z}, offset_amount: float, align_axis: enum{+X, +Y, +Z, -X, -Y, -Z, CAM, CAM_AX}, prev_align_axis: enum{+X, +Y, +Z, -X, -Y, -Z, CAM, CAM_AX, NONE}, align_track: boolean, size_mode: enum{ABSOLUTE, CAMERA, DPI, DPBU}, fill_mode: enum{FILL, FIT}, height: float, factor: float)` — Create mesh plane(s) from image files with the appropriate aspect ratio
- `bpy.ops.image.invert(invert_r: boolean, invert_g: boolean, invert_b: boolean, invert_a: boolean)` — Invert image's channels
- `bpy.ops.image.match_movie_length()` — Set the image's frame range to match the video's duration
- `bpy.ops.image.new(name: string, width: int, height: int, color: float, alpha: boolean, generated_type: enum{BLANK, UV_GRID, COLOR_GRID}, float: boolean, use_stereo_3d: boolean, tiled: boolean)` — Create a new image
- `bpy.ops.image.open(allow_path_tokens: boolean, filepath: string, directory: string, files: collection, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, relative_path: boolean, show_multiview: boolean, use_multiview: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{}, use_sequence_detection: boolean, use_udim_detecting: boolean)` — Open image
- `bpy.ops.image.open_images(directory: string, files: collection, relative_path: boolean, use_sequence_detection: boolean, use_udim_detection: boolean)` — (undocumented operator)
- `bpy.ops.image.pack()` — Pack an image as embedded data into the .blend file
- `bpy.ops.image.project_apply()` — Project edited image back onto the object
- `bpy.ops.image.project_edit()` — Edit a snapshot of the 3D Viewport in an external image editor
- `bpy.ops.image.read_viewlayers()` — Read all the current scene's view layers from cache, as needed
- `bpy.ops.image.reload()` — Reload current image from disk
- `bpy.ops.image.remove_render_slot()` — Remove the current render slot
- `bpy.ops.image.render_border(xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean)` — Set the boundaries of the render region and enable render region
- `bpy.ops.image.replace(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, relative_path: boolean, show_multiview: boolean, use_multiview: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Replace current image by another one from disk
- `bpy.ops.image.resize(size: int, all_udims: boolean)` — Resize the image
- `bpy.ops.image.rotate_orthogonal(degrees: enum{90, 180, 270})` — Rotate the image
- `bpy.ops.image.sample(size: int)` — Use mouse to sample a color in current image
- `bpy.ops.image.sample_line(xstart: int, xend: int, ystart: int, yend: int, flip: boolean, cursor: int)` — Sample a line and show it in Scope panels
- `bpy.ops.image.save()` — Save the image with current name and settings
- `bpy.ops.image.save_all_modified()` — Save all modified images
- `bpy.ops.image.save_as(save_as_render: boolean, copy: boolean, allow_path_tokens: boolean, filepath: string, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, relative_path: boolean, show_multiview: boolean, use_multiview: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Save the image with another name and/or settings
- `bpy.ops.image.save_sequence()` — Save a sequence of images
- `bpy.ops.image.tile_add(number: int, count: int, label: string, fill: boolean, color: float, generated_type: enum{BLANK, UV_GRID, COLOR_GRID}, width: int, height: int, float: boolean, alpha: boolean)` — Adds a tile to the image
- `bpy.ops.image.tile_fill(color: float, generated_type: enum{BLANK, UV_GRID, COLOR_GRID}, width: int, height: int, float: boolean, alpha: boolean)` — Fill the current tile with a generated image
- `bpy.ops.image.tile_remove()` — Removes a tile from the image
- `bpy.ops.image.unpack(method: enum{REMOVE, USE_LOCAL, WRITE_LOCAL, USE_ORIGINAL, WRITE_ORIGINAL}, id: string)` — Save an image packed in the .blend file to disk
- `bpy.ops.image.view_all(fit_view: boolean)` — View the entire image
- `bpy.ops.image.view_center_cursor()` — Center the view so that the cursor is in the middle of the view
- `bpy.ops.image.view_cursor_center(fit_view: boolean)` — Set 2D cursor to center view location
- `bpy.ops.image.view_ndof()` — Use a 3D mouse device to pan/zoom the view
- `bpy.ops.image.view_pan(offset: float)` — Pan the view
- `bpy.ops.image.view_selected()` — View all selected UVs
- `bpy.ops.image.view_zoom(factor: float, use_cursor_init: boolean)` — Zoom in/out the image
- `bpy.ops.image.view_zoom_border(xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean, zoom_out: boolean)` — Zoom in the view to the nearest item contained in the border
- `bpy.ops.image.view_zoom_in(location: float)` — Zoom in the image (centered around 2D cursor)
- `bpy.ops.image.view_zoom_out(location: float)` — Zoom out the image (centered around 2D cursor)
- `bpy.ops.image.view_zoom_ratio(ratio: float)` — Set zoom ratio of the view