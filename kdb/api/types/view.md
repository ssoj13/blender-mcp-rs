---
area: api-types
group: view
version: 5.2.2
---

# Blender 5.2.2 RNA types: view

## ColorManagedViewSettings

Color management settings used for displaying images on the display

- `look` · enum · {NONE} — Additional transform applied before view transform for artistic needs
- `view_transform` · enum · {NONE} — View used when converting image to a display space
- `exposure` · float — Exposure (stops) applied before display transform, multiplying by 2^exposure
- `gamma` · float — Additional gamma encoding after display transform, for output with custom gamma
- `curve_mapping` · pointer · -> CurveMapping · read-only — Color curve mapping applied before display transform
- `use_curve_mapping` · boolean — Use RGB curved for pre-display transformation
- `use_white_balance` · boolean — Perform chromatic adaption from a different white point
- `white_balance_temperature` · float — Color temperature of the scene's white point
- `white_balance_tint` · float — Color tint of the scene's white point (the default of 10 matches daylight)
- `white_balance_whitepoint` · float · [3] — The color which gets mapped to white (automatically converted to/from temperature and tint)
- `is_hdr` · boolean · read-only — The display and view transform supports high dynamic range colors
- `support_emulation` · boolean · read-only — The display and view transform supports automatic emulation for another display device, using the display color spaces mechanism in OpenColorIO v2 configurations

## ForeachGeometryElementZoneViewerPathElem

- `zone_output_node_id` · int

## IDViewerPathElem

- `id` · pointer · -> ID · read-only

## PreferencesView

Preferences related to viewing data

- `ui_scale` · float — Changes the size of the fonts and widgets in the interface
- `border_width` · int — Size of the padding around each editor.
- `ui_line_width` · enum · {THIN, AUTO, THICK} — Changes the thickness of widget outlines, lines and dots in the interface
- `show_tooltips` · boolean — Display tooltips (when disabled, hold Alt then hover to force display)
- `show_tooltips_python` · boolean — Show Python references in tooltips
- `show_developer_ui` · boolean — Display advanced settings and tools for developers
- `show_area_handle` · boolean — Show visible area maintenance corner handles
- `show_number_arrows` · boolean — Display arrows in numeric input fields for increasing or decreasing values
- `show_object_info` · boolean — Include the name of the active object and the current frame number in the text info overlay
- `show_view_name` · boolean — Include the name of the view orientation in the text info overlay
- `show_splash` · boolean — Display splash screen on startup
- `show_playback_fps` · boolean — Include the number of frames displayed per second in the text info overlay while animation is played back
- `playback_fps_samples` · int — The number of frames to use for calculating FPS average. Zero to calculate this automatically, where the number of samples matches the target FPS.
- `use_fresnel_edit` · boolean — Enable a fresnel effect on edit mesh overlays.
It improves shape readability of very dense meshes, but increases eye fatigue when modeling lower poly
- `show_addons_enabled_only` · boolean — Only show enabled add-ons. Un-check to see all installed add-ons.
- `factor_display_type` · enum · {FACTOR, PERCENTAGE} — How factor values are displayed
- `use_weight_color_range` · boolean — Enable color range used for weight visualization in weight painting mode
- `weight_color_range` · pointer · -> ColorRamp · read-only — Color range used for weight visualization in weight painting mode
- `show_navigate_ui` · boolean — Show navigation controls in 2D and 3D views which do not have scroll bars
- `use_mouse_over_open` · boolean — Open menu buttons and pull-downs automatically when the mouse is hovering
- `menu_close_leave` · boolean — Close menus when the mouse is moved out of the region.
- `open_toplevel_delay` · int — Time delay in 1/10 seconds before automatically opening top level menus
- `open_sublevel_delay` · int — Time delay in 1/10 seconds before automatically opening sub level menus
- `color_picker_type` · enum · {CIRCLE_HSV, CIRCLE_HSL, SQUARE_SV, SQUARE_HS, SQUARE_HV} — Different styles of displaying the color picker widget
- `pie_initial_timeout` · int — Pie menus will use the initial mouse position as center for this amount of time (in 1/100ths of sec)
- `pie_tap_timeout` · int — Pie menu button held longer than this will dismiss menu on release (in 1/100ths of sec)
- `pie_animation_timeout` · int — Time needed to fully animate the pie to unfolded state (in 1/100ths of sec)
- `pie_menu_radius` · int — Pie menu size in pixels
- `pie_menu_threshold` · int — Distance from center needed before a selection can be made
- `pie_menu_confirm` · int — Distance threshold after which selection is made (zero to disable)
- `use_save_prompt` · boolean — Ask for confirmation when quitting with unsaved changes
- `show_column_layout` · boolean — Use a column layout for toolbox
- `use_filter_brushes_by_tool` · boolean — Only show brushes applicable for the currently active tool in the asset shelf. Stored in the Preferences, which may have to be saved manually if Auto-Save Preferences is disabled
- `asset_access` · enum · {ALL, ONLY_ONLINE, ONLY_OFFLINE} — Choose the visibility of online and offline assets
- `header_align` · enum · {NONE, TOP, BOTTOM} — Default header position for new space-types
- `render_display_type` · enum · {NONE, SCREEN, AREA, WINDOW} — Default location where rendered images will be displayed in
- `filebrowser_display_type` · enum · {SCREEN, WINDOW} — Default location where the File Editor will be displayed in
- `preferences_display_type` · enum · {SCREEN, WINDOW} — Default location where the Preferences will be displayed in
- `mini_axis_type` · enum · {NONE, MINIMAL, GIZMO} — Show small rotating 3D axes in the top right corner of the 3D viewport
- `mini_axis_size` · int — The axes icon's size
- `mini_axis_brightness` · int — Brightness of the icon
- `smooth_view` · int — Time to animate the view in milliseconds, zero to disable
- `rotation_angle` · float — Rotation step for numerical pad keys (2 4 6 8)
- `show_gizmo` · boolean — Use transform gizmos by default
- `gizmo_size` · int — Diameter of the gizmo
- `gizmo_size_navigate_v3d` · int — The Navigate Gizmo size
- `lookdev_sphere_size` · int — Diameter of the HDRI reference spheres
- `view2d_grid_spacing_min` · int — Minimum number of pixels between each gridline in 2D Viewports
- `timecode_style` · enum · {MINIMAL, SMPTE, SMPTE_COMPACT, MILLISECONDS, SECONDS_ONLY} — Format of timecode displayed when not displaying timing in terms of frames
- `view_frame_type` · enum · {KEEP_RANGE, SECONDS, KEYFRAMES} — How zooming to frame focuses around current frame
- `view_frame_keyframes` · int — Keyframes around cursor that we zoom around
- `view_frame_seconds` · float — Seconds around cursor that we zoom around
- `use_text_antialiasing` · boolean — Smooth jagged edges of user interface text
- `use_text_render_subpixelaa` · boolean — Render text for optimal horizontal placement
- `text_hinting` · enum · {AUTO, NONE, SLIGHT, FULL} — Method for making user interface text render sharp
- `font_path_ui` · string — Path to interface font
- `font_path_ui_mono` · string — Path to interface monospaced Font
- `language` · enum · {DEFAULT, ab, ar_EG, eu_EU, be, bg_BG, ca_AD, zh_HANS, zh_HANT, hr, cs_CZ, da, nl_NL, en_GB, en_US, eo, fi_FI, fr_FR, ka, de_DE, el_GR, he_IL, hi_IN, hu_HU, id_ID, it_IT, ja_JP, ko_KR, ky_KG, lt, ml, nb, fa_IR, pl_PL, pt_BR, pt_PT, ro_RO, ru_RU, sr_RS, sr_RS@latin, ... (51 total)} — Language used for translation
- `use_translate_tooltips` · boolean — Translate the descriptions when hovering UI elements (recommended)
- `use_translate_interface` · boolean — Translate all labels in menus, buttons and panels (note that this might make it hard to follow tutorials or the manual)
- `use_translate_reports` · boolean — Translate additional information, such as error messages
- `use_translate_new_dataname` · boolean — Translate the names of new data-blocks (objects, materials...)
- `date_format` · enum · {DEFAULT, LE_SLASH, LE_DOT, LE_DASH, ME_SLASH, BE_SLASH, BE_DOT, BE_DASH} — Format for displaying date strings
- `time_format` · enum · {H24, H12} — Format for displaying time strings
- `show_statusbar_memory` · boolean — Show Blender memory usage
- `show_statusbar_vram` · boolean — Show GPU video memory usage
- `show_statusbar_version` · boolean — Show Blender version string
- `show_statusbar_stats` · boolean — Show scene statistics
- `show_statusbar_scene_duration` · boolean — Show scene duration
- `show_extensions_updates` · boolean — Show Extensions Update Count
- `use_reduce_motion` · boolean — Avoid animations and other motion effects in the interface

## RegionView3D

3D View region data

- `lock_rotation` · boolean — Lock view rotation of side views to Top/Front/Right
- `show_sync_view` · boolean — Sync view position between side views
- `use_box_clip` · boolean — Clip view contents based on what is visible in other side views
- `perspective_matrix` · float · [16] · read-only — Current perspective matrix (``window_matrix * view_matrix``)
- `window_matrix` · float · [16] · read-only — Current window matrix
- `view_matrix` · float · [16] — Current view matrix
- `view_perspective` · enum · {PERSP, ORTHO, CAMERA} — View Perspective
- `is_perspective` · boolean
- `is_orthographic_side_view` · boolean — Whether the current view is aligned to an axis (does not check whether the view is orthographic, use "is_perspective" for that). Setting this will rotate the view to the closest axis
- `use_clip_planes` · boolean
- `clip_planes` · float · [24]
- `view_location` · float · [3] — View pivot location
- `view_rotation` · float · [4] — Rotation in quaternions (keep normalized)
- `view_distance` · float — Distance to the view location
- `view_camera_zoom` · float — Zoom factor in camera view
- `view_camera_offset` · float · [2] — View shift in camera view

## RepeatZoneViewerPathElem

- `repeat_output_node_id` · int

## SimulationZoneViewerPathElem

- `sim_output_node_id` · int

## SpaceView3D

3D View space data

- `show_region_tool_header` · boolean
- `show_region_toolbar` · boolean
- `show_region_ui` · boolean
- `show_region_hud` · boolean
- `show_region_asset_shelf` · boolean — Display a region with assets that may currently be relevant (such as brushes in paint modes, or poses in Pose Mode)
- `camera` · pointer · -> Object — Active camera used in this view (when unlocked from the scene's active camera)
- `use_render_border` · boolean — Use a region within the frame size for rendered viewport (when not viewing through the camera)
- `render_border_min_x` · float — Minimum X value for the render region
- `render_border_min_y` · float — Minimum Y value for the render region
- `render_border_max_x` · float — Maximum X value for the render region
- `render_border_max_y` · float — Maximum Y value for the render region
- `lock_object` · pointer · -> Object — 3D View center is locked to this object's position
- `lock_bone` · string — 3D View center is locked to this bone's position
- `lock_cursor` · boolean — 3D View center is locked to the cursor's position
- `local_view` · pointer · -> SpaceView3D · read-only — Display an isolated subset of objects, apart from the scene visibility
- `lens` · float — Viewport lens angle
- `clip_start` · float — 3D View near clipping distance (perspective view only)
- `clip_end` · float — 3D View far clipping distance
- `lock_camera` · boolean — Enable view navigation within the camera view
- `show_gizmo` · boolean — Show gizmos of all types
- `show_gizmo_navigate` · boolean — Viewport navigation gizmo
- `show_gizmo_context` · boolean — Context sensitive gizmos for the active item
- `show_gizmo_modifier` · boolean — Gizmos for the active modifier
- `show_gizmo_tool` · boolean — Active tool gizmo
- `show_gizmo_object_translate` · boolean — Gizmo to adjust location
- `show_gizmo_object_rotate` · boolean — Gizmo to adjust rotation
- `show_gizmo_object_scale` · boolean — Gizmo to adjust scale
- `show_gizmo_empty_image` · boolean — Gizmo to adjust image size and position
- `show_gizmo_empty_force_field` · boolean — Gizmo to adjust the force field
- `show_gizmo_light_size` · boolean — Gizmo to adjust spot and area size
- `show_gizmo_light_look_at` · boolean — Gizmo to adjust the direction of the light
- `show_gizmo_camera_lens` · boolean — Gizmo to adjust camera focal length or orthographic scale
- `show_gizmo_camera_dof_distance` · boolean — Gizmo to adjust camera focus distance (depends on limits display)
- `use_local_camera` · boolean — Use a local camera in this view, rather than scene's active camera
- `region_3d` · pointer · -> RegionView3D · read-only — 3D region for this space. When the space is in quad view, the camera region
- `region_quadviews` · collection · read-only — 3D regions (the third one defines quad view settings, the fourth one is same as 'region_3d')
- `show_reconstruction` · boolean — Display reconstruction data from active movie clip
- `tracks_display_size` · float — Display size of tracks from reconstructed data
- `tracks_display_type` · enum · {PLAIN_AXES, ARROWS, SINGLE_ARROW, CIRCLE, CUBE, SPHERE, CONE} — Viewport display style for tracks
- `show_camera_path` · boolean — Show reconstructed camera path
- `show_bundle_names` · boolean — Show names for reconstructed tracks objects
- `use_local_collections` · boolean — Display a different set of collections in this viewport
- `stereo_3d_eye` · enum · {LEFT_EYE, RIGHT_EYE} · read-only — Current stereo eye being displayed
- `stereo_3d_camera` · enum · {LEFT, RIGHT, S3D}
- `show_stereo_3d_cameras` · boolean — Show the left and right cameras
- `show_stereo_3d_convergence_plane` · boolean — Show the stereo 3D convergence plane
- `stereo_3d_convergence_plane_alpha` · float — Opacity (alpha) of the convergence plane
- `show_stereo_3d_volume` · boolean — Show the stereo 3D frustum volume
- `stereo_3d_volume_alpha` · float — Opacity (alpha) of the cameras' frustum volume
- `mirror_xr_session` · boolean — Synchronize the viewer perspective of virtual reality sessions with this 3D viewport
- `show_object_viewport_mesh` · boolean — Show mesh objects
- `show_object_viewport_curve` · boolean — Show curves
- `show_object_viewport_surf` · boolean — Show surfaces
- `show_object_viewport_meta` · boolean — Show metaballs
- `show_object_viewport_font` · boolean — Show text objects
- `show_object_viewport_curves` · boolean — Show hair curves
- `show_object_viewport_pointcloud` · boolean — Show point clouds
- `show_object_viewport_volume` · boolean — Show volumes
- `show_object_viewport_armature` · boolean — Show armatures
- `show_object_viewport_lattice` · boolean — Show lattices
- `show_object_viewport_empty` · boolean — Show empties
- `show_object_viewport_grease_pencil` · boolean — Show Grease Pencil objects
- `show_object_viewport_camera` · boolean — Show cameras
- `show_object_viewport_light` · boolean — Show lights
- `show_object_viewport_speaker` · boolean — Show speakers
- `show_object_viewport_light_probe` · boolean — Show light probes
- `show_object_select_mesh` · boolean — Allow selection of mesh objects
- `show_object_select_curve` · boolean — Allow selection of curves
- `show_object_select_surf` · boolean — Allow selection of surfaces
- `show_object_select_meta` · boolean — Allow selection of metaballs
- `show_object_select_font` · boolean — Allow selection of text objects
- `show_object_select_curves` · boolean — Allow selection of hair curves
- `show_object_select_pointcloud` · boolean — Allow selection of point clouds
- `show_object_select_volume` · boolean — Allow selection of volumes
- `show_object_select_armature` · boolean — Allow selection of armatures
- `show_object_select_lattice` · boolean — Allow selection of lattices
- `show_object_select_empty` · boolean — Allow selection of empties
- `show_object_select_grease_pencil` · boolean — Allow selection of Grease Pencil objects
- `show_object_select_camera` · boolean — Allow selection of cameras
- `show_object_select_light` · boolean — Allow selection of lights
- `show_object_select_speaker` · boolean — Allow selection of speakers
- `show_object_select_light_probe` · boolean — Allow selection of light probes
- `icon_from_show_object_viewport` · int · read-only
- `show_viewer` · boolean — Display non-final geometry from viewer nodes
- `shading` · pointer · -> View3DShading · read-only — Settings for shading in the 3D viewport
- `overlay` · pointer · -> View3DOverlay · read-only — Settings for display of overlays in the 3D viewport

## ThemeView3D

Theme settings for the 3D viewport

- `space` · pointer · -> ThemeSpaceGradient · read-only — Settings for space
- `grid` · float · [4]
- `grid_major` · float · [4]
- `grid_axis_brightness` · float — Brightness of the grid axis lines
- `clipping_border_3d` · float · [4]
- `wire` · float · [3]
- `wire_edit` · float · [3] — Color for wireframe when in edit mode, but edge selection is active
- `edge_width` · int
- `gp_wire_edit` · float · [4] — Grease Pencil wireframe color when in edit mode
- `gp_vertex` · float · [3]
- `gp_vertex_select` · float · [3]
- `gp_vertex_size` · int
- `text_grease_pencil` · float · [3] — Color for indicating Grease Pencil keyframes
- `object_selected` · float · [3]
- `object_active` · float · [3]
- `camera` · float · [3]
- `empty` · float · [3]
- `light` · float · [4]
- `speaker` · float · [3]
- `vertex` · float · [3]
- `vertex_select` · float · [3]
- `vertex_size` · int
- `edge_select` · float · [3]
- `edge_mode_select` · float · [3]
- `face` · float · [4]
- `face_select` · float · [4]
- `face_mode_select` · float · [4]
- `facedot_size` · int
- `face_back` · float · [4]
- `face_front` · float · [4]
- `bevel` · float · [3]
- `seam` · float · [3]
- `sharp` · float · [3]
- `crease` · float · [3]
- `freestyle` · float · [4]
- `extra_edge_len` · float · [3]
- `extra_edge_angle` · float · [3]
- `extra_face_angle` · float · [3]
- `extra_face_area` · float · [3]
- `editmesh_active` · float · [4]
- `normal` · float · [3]
- `vertex_normal` · float · [3]
- `split_normal` · float · [3]
- `vertex_unreferenced` · float · [3]
- `face_retopology` · float · [4]
- `nurb_uline` · float · [3]
- `nurb_vline` · float · [3]
- `nurb_sel_uline` · float · [3]
- `nurb_sel_vline` · float · [3]
- `bone_pose` · float · [3] — Outline color of selected pose bones
- `bone_pose_active` · float · [3] — Outline color of active pose bones
- `bone_solid` · float · [3] — Default color of the solid shapes of bones
- `bone_locked_weight` · float · [4] — Shade for bones corresponding to a locked weight group during painting
- `before_current_frame` · float · [3] — The color for things before the current frame (for onion skinning, motion paths, etc.)
- `after_current_frame` · float · [3] — The color for things after the current frame (for onion skinning, motion paths, etc.)
- `bundle_solid` · float · [3]
- `camera_path` · float · [3]
- `camera_passepartout` · float · [3]
- `skin_root` · float · [3]
- `view_overlay` · float · [3]
- `transform` · float · [3]
- `outline_width` · int
- `object_origin_size` · int — Diameter in pixels for object/light origin display

## View3DCursor

- `location` · float · [3]
- `rotation_quaternion` · float · [4] — Rotation in quaternions (keep normalized)
- `rotation_axis_angle` · float · [4] — Angle of Rotation for Axis-Angle rotation representation
- `rotation_euler` · float · [3] — 3D rotation
- `rotation_mode` · enum · {QUATERNION, XYZ, XZY, YXZ, YZX, ZXY, ZYX, AXIS_ANGLE} — The kind of rotation to apply, values from other rotation modes are not used
- `matrix` · float · [16] — Matrix combining location and rotation of the cursor

## View3DOverlay

Settings for display of overlays in the 3D viewport

- `show_overlays` · boolean — Display overlays like gizmos and outlines
- `show_ortho_grid` · boolean — Show grid in orthographic side view
- `show_floor` · boolean — Show the ground plane grid
- `show_axis_x` · boolean — Show the X axis line
- `show_axis_y` · boolean — Show the Y axis line
- `show_axis_z` · boolean — Show the Z axis line
- `grid_scale` · float — Multiplier for the distance between 3D View grid lines
- `grid_lines` · int — Number of grid lines to display in perspective view
- `grid_subdivisions` · int — Number of subdivisions between grid lines
- `grid_scale_unit` · float · read-only — Grid cell size scaled by scene unit system settings
- `show_outline_selected` · boolean — Show an outline highlight around selected objects
- `show_object_origins` · boolean — Show object center dots
- `show_object_origins_all` · boolean — Show the object origin center dot for all (selected and unselected) objects
- `show_relationship_lines` · boolean — Show dashed lines indicating parent or constraint relationships
- `show_cursor` · boolean — Display 3D Cursor Overlay
- `show_text` · boolean — Display overlay text
- `show_stats` · boolean — Display scene statistics overlay text
- `show_performance` · boolean — Display viewport performance timings:
 • Evaluation: Time to evaluate the dependency graph.
 • Synchronization: Time to build the GPU buffers.
- `show_camera_guides` · boolean — Show camera composition guides
- `show_camera_passepartout` · boolean — Show camera passepartout
- `show_extras` · boolean — Object details, including empty wire, cameras and other visual guides
- `show_light_colors` · boolean — Show light colors
- `show_bones` · boolean — Display bones (disable to show motion paths only)
- `show_face_orientation` · boolean — Show the Face Orientation Overlay
- `show_fade_inactive` · boolean — Fade inactive geometry using the viewport background color
- `fade_inactive_alpha` · float — Strength of the fade effect
- `show_xray_bone` · boolean — Show the bone selection overlay
- `xray_alpha_bone` · float — Opacity to use for bone selection
- `bone_wire_alpha` · float — Maximum opacity of bones in wireframe display mode
- `show_motion_paths` · boolean — Show the Motion Paths Overlay
- `show_onion_skins` · boolean — Show the Onion Skinning Overlay
- `show_look_dev` · boolean — Show reference spheres with neutral shading that react to lighting to assist in look development
- `show_wireframes` · boolean — Show face edges wires
- `wireframe_threshold` · float — Adjust the angle threshold for displaying edges (1.0 for all)
- `wireframe_opacity` · float — Opacity of the displayed edges (1.0 for opaque)
- `show_viewer_attribute` · boolean — Show attribute overlay for active viewer node
- `viewer_attribute_opacity` · float — Opacity of the attribute that is currently visualized
- `show_viewer_text` · boolean — Show attribute values as text in viewport
- `show_paint_wire` · boolean — Use wireframe display in painting modes
- `show_wpaint_contours` · boolean — Show contour lines formed by points with the same interpolated weight
- `show_weight` · boolean — Display weights in editmode
- `show_retopology` · boolean — Hide the solid mesh and offset the overlay towards the view. Selection is occluded by inactive geometry, unless X-Ray is enabled
- `retopology_offset` · float — Offset used to draw edit mesh in front of other geometry
- `show_face_normals` · boolean — Display face normals as lines
- `show_vertex_normals` · boolean — Display vertex normals as lines
- `show_split_normals` · boolean — Display vertex-per-face normals as lines
- `show_faces` · boolean — Display a face selection overlay
- `show_face_center` · boolean — Display face center when face selection is enabled in solid shading modes
- `show_edge_crease` · boolean — Display creases created for Subdivision Surface modifier
- `show_edge_bevel_weight` · boolean — Display weights created for the Bevel modifier
- `show_edge_seams` · boolean — Display UV unwrapping seams
- `show_edge_sharp` · boolean — Display sharp edges, used with the Edge Split modifier
- `show_freestyle_edge_marks` · boolean — Display Freestyle edge marks, used with the Freestyle renderer
- `show_freestyle_face_marks` · boolean — Display Freestyle face marks, used with the Freestyle renderer
- `show_statvis` · boolean — Display statistical information about the mesh
- `show_extra_edge_length` · boolean — Display selected edge lengths, using global values when set in the transform panel
- `show_extra_edge_angle` · boolean — Display selected edge angle, using global values when set in the transform panel
- `show_extra_face_angle` · boolean — Display the angles in the selected edges, using global values when set in the transform panel
- `show_extra_face_area` · boolean — Display the area of selected faces, using global values when set in the transform panel
- `show_extra_indices` · boolean — Display the index numbers of selected vertices, edges, and faces
- `display_handle` · enum · {NONE, SELECTED, ALL} — Limit the display of curve handles in Edit Mode
- `show_curve_normals` · boolean — Display 3D curve normals in Edit Mode
- `normals_length` · float — Display size for normals in the 3D view
- `normals_constant_screen_size` · float — Screen size for normals in the 3D view
- `use_normals_constant_screen_size` · boolean — Keep size of normals constant in relation to 3D view
- `texture_paint_mode_opacity` · float — Opacity of the texture paint mode stencil mask overlay
- `vertex_paint_mode_opacity` · float — Opacity of the texture paint mode stencil mask overlay
- `weight_paint_mode_opacity` · float — Opacity of the weight paint mode overlay
- `sculpt_mode_mask_opacity` · float
- `show_sculpt_curves_cage` · boolean — Show original curves that are currently being edited
- `sculpt_curves_cage_opacity` · float — Opacity of the cage overlay in curves sculpt mode
- `sculpt_mode_face_sets_opacity` · float
- `show_sculpt_mask` · boolean
- `show_sculpt_face_sets` · boolean
- `show_annotation` · boolean — Show annotations for this view
- `use_gpencil_fade_objects` · boolean — Fade all viewport objects with a full color layer to improve visibility
- `use_gpencil_grid` · boolean — Display a grid over Grease Pencil paper
- `use_gpencil_fade_layers` · boolean — Toggle fading of Grease Pencil layers except the active one
- `use_gpencil_fade_gp_objects` · boolean — Fade Grease Pencil Objects, except the active one
- `use_gpencil_canvas_xray` · boolean — Show Canvas grid in front
- `use_gpencil_show_directions` · boolean — Show stroke drawing direction with a bigger green dot (start) and smaller red dot (end) points
- `use_gpencil_show_material_name` · boolean — Show material name assigned to each stroke
- `gpencil_grid_opacity` · float — Canvas grid opacity
- `gpencil_grid_color` · float · [3] — Canvas grid color
- `gpencil_grid_scale` · float · [2] — Canvas grid scale
- `gpencil_grid_offset` · float · [2] — Canvas grid offset
- `gpencil_grid_subdivisions` · int — Canvas grid subdivisions
- `gpencil_fade_objects` · float — Fade factor
- `gpencil_fade_layer` · float — Fade layer opacity for Grease Pencil layers except the active one
- `use_gpencil_edit_lines` · boolean — Show Edit Lines when editing strokes
- `use_gpencil_multiedit_line_only` · boolean — Show Edit Lines only in multiframe
- `use_gpencil_onion_skin` · boolean — Show ghosts of the keyframes before and after the current frame
- `use_gpencil_onion_skin_active_object` · boolean — Show only the onion skins of the active object
- `vertex_opacity` · float — Opacity for edit vertices
- `gpencil_vertex_paint_opacity` · float — Vertex Paint mix factor
- `use_debug_freeze_view_culling` · boolean — Freeze view culling bounds

## View3DShading

Settings for shading in the 3D viewport

- `type` · enum · {WIREFRAME, SOLID, MATERIAL, RENDERED} — Method to display/shade objects in the 3D View
- `light` · enum · {STUDIO, MATCAP, FLAT} — Lighting Method for Solid/Texture Viewport Shading
- `show_object_outline` · boolean — Show Object Outline
- `studio_light` · enum · {DEFAULT} — Studio lighting setup
- `use_world_space_lighting` · boolean — Make the lighting fixed and not follow the camera
- `show_backface_culling` · boolean — Use back face culling to hide the back side of faces
- `show_cavity` · boolean — Show Cavity
- `cavity_type` · enum · {WORLD, SCREEN, BOTH} — Way to display the cavity shading
- `curvature_ridge_factor` · float — Factor for the curvature ridges
- `curvature_valley_factor` · float — Factor for the curvature valleys
- `cavity_ridge_factor` · float — Factor for the cavity ridges
- `cavity_valley_factor` · float — Factor for the cavity valleys
- `selected_studio_light` · pointer · -> StudioLight · read-only — Selected StudioLight
- `studiolight_rotate_z` · float — Rotation of the studiolight around the Z-Axis
- `studiolight_intensity` · float — Strength of the studiolight
- `studiolight_background_alpha` · float — Show the studiolight in the background
- `studiolight_background_blur` · float — Blur the studiolight in the background
- `use_studiolight_view_rotation` · boolean — Make the HDR rotation fixed and not follow the camera
- `color_type` · enum · {MATERIAL, OBJECT, RANDOM, VERTEX, TEXTURE, SINGLE} — Color Type
- `wireframe_color_type` · enum · {THEME, OBJECT, RANDOM} — Wire Color Type
- `single_color` · float · [3] — Color for single color mode
- `background_type` · enum · {THEME, WORLD, VIEWPORT} — Way to display the background
- `background_color` · float · [3] — Color for custom background color
- `show_shadows` · boolean — Show Shadow
- `show_xray` · boolean — Show whole scene transparent
- `show_xray_wireframe` · boolean — Show whole scene transparent
- `xray_alpha` · float — Amount of opacity to use
- `xray_alpha_wireframe` · float — Amount of opacity to use
- `use_dof` · boolean — Use depth of field on viewport using the values from the active camera
- `use_scene_lights` · boolean — Render lights and light probes of the scene
- `use_scene_world` · boolean — Use scene world for lighting
- `use_scene_lights_render` · boolean — Render lights and light probes of the scene
- `use_scene_world_render` · boolean — Use scene world for lighting
- `show_specular_highlight` · boolean — Render specular highlights
- `object_outline_color` · float · [3] — Color for object outline
- `shadow_intensity` · float — Darkness of shadows
- `render_pass` · enum · {COMBINED, EMISSION, ENVIRONMENT, AO, SHADOW, TRANSPARENT, DIFFUSE_LIGHT, DIFFUSE_COLOR, SPECULAR_LIGHT, SPECULAR_COLOR, VOLUME_LIGHT, POSITION, NORMAL, MIST, CryptoObject, CryptoAsset, CryptoMaterial, AOV} — Render Pass to show in the viewport
- `aov_name` · string — Name of the active Shader AOV
- `use_compositor` · enum · {DISABLED, CAMERA, ALWAYS} — When to preview the compositor output inside the viewport
- `cycles` · pointer · -> CyclesView3DShadingSettings · read-only

## ViewLayer

View layer

- `name` · string — View layer name
- `material_override` · pointer · -> Material — Material to override all other materials in this view layer
- `world_override` · pointer · -> World — Override world in this view layer
- `samples` · int — Override number of render samples for this view layer, 0 will use the scene setting
- `pass_alpha_threshold` · float — Z, Index, normal, UV and vector passes are only affected by surfaces with alpha transparency equal to or higher than this threshold
- `eevee` · pointer · -> ViewLayerEEVEE · read-only — View layer settings for EEVEE
- `aovs` · collection · [] of AOVs · read-only
- `active_aov` · pointer · -> AOV · read-only — Active AOV
- `active_aov_index` · int — Index of active AOV
- `lightgroups` · collection · [] of Lightgroups · read-only
- `active_lightgroup` · pointer · -> Lightgroup · read-only — Active Lightgroup
- `active_lightgroup_index` · int — Index of active lightgroup
- `use_pass_cryptomatte_object` · boolean — Render cryptomatte object pass, for isolating objects in compositing
- `use_pass_cryptomatte_material` · boolean — Render cryptomatte material pass, for isolating materials in compositing
- `use_pass_cryptomatte_asset` · boolean — Render cryptomatte asset pass, for isolating groups of objects with the same parent
- `pass_cryptomatte_depth` · int — Sets how many unique objects can be distinguished per pixel
- `use_pass_cryptomatte_accurate` · boolean — Generate a more accurate cryptomatte pass
- `use_solid` · boolean — Render Solid faces in this Layer
- `use_sky` · boolean — Render Sky in this Layer
- `use_ao` · boolean — Render Ambient Occlusion in this Layer
- `use_strand` · boolean — Render Strands in this Layer
- `use_volumes` · boolean — Render volumes in this Layer
- `use_motion_blur` · boolean — Render motion blur in this Layer, if enabled in the scene
- `use_grease_pencil` · boolean — Render Grease Pencil on this layer
- `use_pass_combined` · boolean — Deliver full combined RGBA buffer
- `use_pass_z` · boolean — Deliver depth values pass
- `use_pass_vector` · boolean — Deliver speed vector pass
- `use_pass_position` · boolean — Deliver position pass
- `use_pass_normal` · boolean — Deliver normal pass
- `use_pass_uv` · boolean — Deliver texture UV pass
- `use_pass_mist` · boolean — Deliver mist factor pass (0.0 to 1.0)
- `use_pass_object_index` · boolean — Deliver object index pass
- `use_pass_material_index` · boolean — Deliver material index pass
- `use_pass_shadow` · boolean — Deliver shadow pass
- `use_pass_ambient_occlusion` · boolean — Deliver Ambient Occlusion pass
- `use_pass_emit` · boolean — Deliver emission pass
- `use_pass_environment` · boolean — Deliver environment lighting pass
- `use_pass_diffuse_direct` · boolean — Deliver diffuse direct pass
- `use_pass_diffuse_indirect` · boolean — Deliver diffuse indirect pass
- `use_pass_diffuse_color` · boolean — Deliver diffuse color pass
- `use_pass_glossy_direct` · boolean — Deliver glossy direct pass
- `use_pass_glossy_indirect` · boolean — Deliver glossy indirect pass
- `use_pass_glossy_color` · boolean — Deliver glossy color pass
- `use_pass_transmission_direct` · boolean — Deliver transmission direct pass
- `use_pass_transmission_indirect` · boolean — Deliver transmission indirect pass
- `use_pass_transmission_color` · boolean — Deliver transmission color pass
- `use_pass_subsurface_direct` · boolean — Deliver subsurface direct pass
- `use_pass_subsurface_indirect` · boolean — Deliver subsurface indirect pass
- `use_pass_subsurface_color` · boolean — Deliver subsurface color pass
- `layer_collection` · pointer · -> LayerCollection · read-only — Root of collections hierarchy of this view layer, its 'collection' pointer property is the same as the scene's master collection
- `active_layer_collection` · pointer · -> LayerCollection — Active layer collection in this view layer's hierarchy
- `objects` · collection · [] of LayerObjects · read-only — All the objects in this layer
- `use` · boolean — Enable or disable rendering of this View Layer
- `has_export_collections` · boolean · read-only — At least one Collection in this View Layer has an exporter
- `use_freestyle` · boolean — Render stylized strokes in this Layer
- `freestyle_settings` · pointer · -> FreestyleSettings · read-only
- `use_pass_grease_pencil` · boolean — Deliver Grease Pencil render result in a separate pass
- `depsgraph` · pointer · -> Depsgraph · read-only — Dependencies in the scene data
- `cycles` · pointer · -> CyclesRenderLayerSettings · read-only — Cycles ViewLayer Settings

## ViewLayerEEVEE

View Layer settings for EEVEE

- `use_pass_volume_direct` · boolean — Deliver volume direct light pass
- `use_pass_bloom` · boolean — Deliver bloom pass (deprecated)
- `use_pass_transparent` · boolean — Deliver alpha blended surfaces in a separate pass
- `ambient_occlusion_distance` · float — Distance of object that contribute to the ambient occlusion effect

## ViewerPath

Path to data that is viewed

- `path` · collection · read-only

## ViewerPathElem

Element of a viewer path

- `type` · enum · {ID, MODIFIER, GROUP_NODE, SIMULATION_ZONE, VIEWER_NODE, REPEAT_ZONE, FOREACH_GEOMETRY_ELEMENT_ZONE, EVALUATE_CLOSURE} · read-only — Type of the path element
- `ui_name` · string · read-only — Name that can be displayed in the UI for this element

## XrViewfinderState

Runtime state information about the VR Location Scouting Viewfinder

- `location` · float · [3] · read-only — Last known location of the viewfinder in world space
- `orientation` · float · [4] · read-only — Last known orientation of the viewfinder in world space
- `capture_dof_enabled` · boolean — Enable viewfinder capture depth of field
- `capture_lens_focal` · float — Viewfinder capture focal length value in millimeters
- `capture_dof_distance` · float — Viewfinder capture distance to the focus point for depth of field
- `capture_dof_fstop` · float — Viewfinder capture f-stop ratio
- `playback_show_active_capture_in_space_enabled` · boolean — Display active capture in space when in Viewfinder Playback mode
- `active_mode` · enum · {LIVE, PLAYBACK, CONFIRM} — Active viewfinder mode, live or playback
- `active_action_live` · enum · {LENS, DOF, FOCUS, APERTURE} — Active viewfinder live action
- `active_action_playback` · enum · {BROWSE, PREVIEW, DELETE} — Active viewfinder playback action
- `active_action_confirm` · enum · {CANCEL, CONFIRM} — Active viewfinder confirm action
