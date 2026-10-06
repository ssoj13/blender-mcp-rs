---
area: api-types
group: preferences
version: 5.2.2
---

# Blender 5.2.2 RNA types: preferences

## AddonPreferences

- `bl_idname` · string

## Preferences

Global preferences

- `active_section` · enum · {INTERFACE, VIEWPORT, LIGHTS, EDITING, ANIMATION, EXTENSIONS, ADDONS, THEMES, ASSETS, INPUT, NAVIGATION, KEYMAP, SYSTEM, SAVE_LOAD, FILE_PATHS, DEVELOPER_TOOLS, EXPERIMENTAL} — Preferences
- `app_template` · string
- `themes` · collection · read-only
- `ui_styles` · collection · read-only
- `addons` · collection · [] of Addons · read-only
- `autoexec_paths` · collection · [] of PathCompareCollection · read-only
- `use_recent_searches` · boolean — Sort the recently searched items at the top
- `show_hidden_ids` · boolean — Show data-blocks with dot-prefixed names in search menus
- `view` · pointer · -> PreferencesView · read-only — Preferences related to viewing data
- `edit` · pointer · -> PreferencesEdit · read-only — Settings for interacting with Blender data
- `inputs` · pointer · -> PreferencesInput · read-only — Settings for input devices
- `keymap` · pointer · -> PreferencesKeymap · read-only — Shortcut setup for keyboards and other input devices
- `filepaths` · pointer · -> PreferencesFilePaths · read-only — Default paths for external files
- `asset_libraries` · pointer · -> PreferencesAssetLibraries · read-only — Setup for custom and builtin asset libraries
- `extensions` · pointer · -> PreferencesExtensions · read-only — Settings for extensions
- `system` · pointer · -> PreferencesSystem · read-only — Graphics driver and operating system settings
- `apps` · pointer · -> PreferencesApps · read-only — Preferences that work only for apps
- `experimental` · pointer · -> PreferencesExperimental · read-only — Settings for features that are still early in their development stage
- `version` · int · [3] · read-only — Version of Blender the userpref.blend was saved with
- `studio_lights` · collection · [] of StudioLights · read-only
- `use_preferences_save` · boolean — Save preferences on exit when modified (unless factory settings have been loaded)
- `is_dirty` · boolean — Preferences have changed

## PreferencesApps

Preferences that work only for apps

- `show_corner_split` · boolean — Split and join editors by dragging from corners
- `show_edge_resize` · boolean — Resize editors by dragging from the edges
- `show_regions_visibility_toggle` · boolean — Header and side bars visibility toggles

## PreferencesAssetLibraries

- `use_online_essentials` · boolean — Include remote assets in the Essentials asset library. Downloading requires Online Access to be enabled under System > Network in the Preferences

## PreferencesEdit

Settings for interacting with Blender data

- `material_link` · enum · {OBDATA, OBJECT} — Toggle whether the material is linked to object data or the object block
- `object_align` · enum · {WORLD, VIEW, CURSOR} — The default alignment for objects added from a 3D viewport menu
- `use_enter_edit_mode` · boolean — Enter edit mode automatically after adding a new object
- `collection_instance_empty_size` · float — Display size of the empty when new collection instances are created
- `use_text_edit_auto_close` · boolean — Automatically close relevant character pairs when typing in the text editor
- `undo_steps` · int — Number of undo steps available (smaller values conserve memory)
- `undo_memory_limit` · int — Maximum memory usage in megabytes (0 means unlimited)
- `use_global_undo` · boolean — Global undo works by keeping a full copy of the file itself in memory, so takes extra memory
- `use_auto_keying` · boolean — Automatic keyframe insertion for Objects and Bones (default setting used for new Scenes)
- `auto_keying_mode` · enum · {ADD_REPLACE_KEYS, REPLACE_KEYS} — Mode of automatic keyframe insertion for Objects and Bones (default setting used for new Scenes)
- `use_keyframe_insert_available` · boolean — Insert Keyframes only for properties that are already animated
- `use_auto_keying_warning` · boolean — Show warning indicators when transforming objects and bones if auto keying is enabled
- `key_insert_channels` · enum · {LOCATION, ROTATION, SCALE, ROTATE_MODE, CUSTOM_PROPS} — Which channels to insert keys at when no keying set is active
- `use_auto_keyframe_insert_needed` · boolean — Auto-Keying will skip inserting keys that don't affect the animation
- `use_keyframe_insert_needed` · boolean — When keying manually, skip inserting keys that don't affect the animation
- `use_visual_keying` · boolean — Use Visual keying automatically for constrained objects
- `use_insertkey_xyz_to_rgb` · boolean — Color for newly added transformation F-Curves (Location, Rotation, Scale) and also Color is based on the transform axis
- `use_anim_channel_group_colors` · boolean — Use animation channel group colors; generally this is used to show bone group colors
- `fcurve_new_auto_smoothing` · enum · {NONE, CONT_ACCEL} — Auto Handle Smoothing mode used for newly added F-Curves
- `keyframe_new_interpolation_type` · enum · {CONSTANT, LINEAR, BEZIER, SINE, QUAD, CUBIC, QUART, QUINT, EXPO, CIRC, BACK, BOUNCE, ELASTIC} — Interpolation mode used for first keyframe on newly added F-Curves (subsequent keyframes take interpolation from preceding keyframe)
- `keyframe_new_handle_type` · enum · {FREE, ALIGNED, VECTOR, AUTO, AUTO_CLAMPED} — Handle type for handles of new keyframes
- `use_negative_frames` · boolean — Current frame number can be manually set to a negative value
- `fcurve_unselected_alpha` · float — The opacity of unselected F-Curves against the background of the Graph Editor
- `show_only_selected_curve_keyframes` · boolean — Only keyframes of selected F-Curves are visible and editable
- `use_fcurve_high_quality_drawing` · boolean — Draw F-Curves using Anti-Aliasing (disable for better performance)
- `grease_pencil_manhattan_distance` · int — Pixels moved by mouse per axis when drawing stroke
- `grease_pencil_euclidean_distance` · int — Distance moved by mouse when drawing stroke to include
- `grease_pencil_eraser_radius` · int — Radius of eraser 'brush'
- `grease_pencil_default_color` · float · [4] — Color of new annotation layers
- `sculpt_paint_overlay_color` · float · [3] — Color of texture overlay
- `connect_strips_by_default` · boolean — Connect newly added movie strips by default if they have multiple channels
- `use_duplicate_mesh` · boolean — Causes mesh data to be duplicated with the object
- `use_duplicate_surface` · boolean — Causes surface data to be duplicated with the object
- `use_duplicate_curve` · boolean — Causes curve data to be duplicated with the object
- `use_duplicate_lattice` · boolean — Causes lattice data to be duplicated with the object
- `use_duplicate_text` · boolean — Causes text data to be duplicated with the object
- `use_duplicate_metaball` · boolean — Causes metaball data to be duplicated with the object
- `use_duplicate_armature` · boolean — Causes armature data to be duplicated with the object
- `use_duplicate_camera` · boolean — Causes camera data to be duplicated with the object
- `use_duplicate_speaker` · boolean — Causes speaker data to be duplicated with the object
- `use_duplicate_light` · boolean — Causes light data to be duplicated with the object
- `use_duplicate_material` · boolean — Causes material data to be duplicated with the object
- `use_duplicate_action` · boolean — Causes actions to be duplicated with the data-blocks
- `use_duplicate_particle` · boolean — Causes particle systems to be duplicated with the object
- `use_duplicate_lightprobe` · boolean — Causes light probe data to be duplicated with the object
- `use_duplicate_grease_pencil` · boolean — Causes Grease Pencil data to be duplicated with the object
- `use_duplicate_curves` · boolean — Causes curves data to be duplicated with the object
- `use_duplicate_pointcloud` · boolean — Causes point cloud data to be duplicated with the object
- `use_duplicate_volume` · boolean — Causes volume data to be duplicated with the object
- `use_duplicate_node_tree` · boolean — Make copies of node groups when duplicating nodes in the node editor
- `node_use_insert_offset` · boolean — Automatically offset the following or previous nodes in a chain when inserting a new node
- `node_margin` · int — Minimum distance between nodes for Auto-offsetting nodes
- `node_preview_resolution` · int — Resolution used for Shader node previews (should be changed for performance convenience)
- `use_cursor_lock_adjust` · boolean — Place the cursor without 'jumping' to the new location (when lock-to-cursor is used)
- `use_mouse_depth_cursor` · boolean — Use the surface depth for cursor placement

## PreferencesExperimental

Experimental features

- `use_undo_legacy` · boolean — Use legacy undo (slower than the new default one, but may be more stable in some cases)
- `override_auto_resync` · boolean — Disable library overrides automatic resync detection and process on file load (can be useful to help fixing broken files). Also see the "--disable-liboverride-auto-resync" command line option
- `use_new_curves_tools` · boolean — Enable additional features for the new curves data block
- `use_cycles_debug` · boolean — Enable Cycles debugging options for developers
- `use_eevee_debug` · boolean — Enable EEVEE debugging options for developers
- `use_sculpt_texture_paint` · boolean — Use texture painting in Sculpt Mode
- `use_extended_asset_browser` · boolean — Enable Asset Browser editor and operators to manage regular data-blocks as assets, not just poses
- `show_asset_debug_info` · boolean — Enable some extra fields in the Asset Browser to aid in debugging
- `use_asset_indexing` · boolean — Disable the asset indexer, to force every asset library refresh to completely reread assets from disk
- `use_viewport_debug` · boolean — Enable viewport debugging options for developers in the overlays pop-over
- `write_legacy_blend_file_format` · boolean — Use file format used before Blender 5.0. This format is more limited but it may have better compatibility with tools that don't support the new format yet
- `no_data_block_packing` · boolean — Fall-back to appending instead of packing data-blocks
- `use_all_linked_data_direct` · boolean — Forces all linked data to be considered as directly linked. Workaround for current issues/limitations in BAT (Blender studio pipeline tool)
- `use_shader_node_previews` · boolean — Enables previews in the shader node editor
- `use_remote_asset_libraries` · boolean · read-only — Enable asset libraries served over HTTP/HTTPS
- `use_collection_importer` · boolean — Enables a file importer to be configured on a Collection
- `use_extensions_debug` · boolean — Extra debugging information & developer support utilities for extensions
- `use_recompute_usercount_on_save_debug` · boolean — Recompute all ID user-counts before saving to a blend-file. Allows to work around invalid user-count handling in code that may lead to loss of data due to wrongly detected unused data-blocks
- `use_paint_debug` · boolean — Enable paint & sculpt debugging options for developers

## PreferencesExtensions

Settings for extensions

- `use_online_access_handled` · boolean — The user has been shown the "Online Access" prompt and made a choice
- `repos` · collection · [] of UserExtensionRepoCollection · read-only
- `active_repo` · int — Index of the extensions repository being edited in the Preferences UI

## PreferencesFilePaths

Default paths for external files

- `show_hidden_files_datablocks` · boolean — Show files and data-blocks that are normally hidden
- `use_filter_files` · boolean — Enable filtering of files in the File Browser
- `show_recent_locations` · boolean — Show Recent locations list in the File Browser
- `show_system_bookmarks` · boolean — Show System locations list in the File Browser
- `use_relative_paths` · boolean — Default relative path option for the file selector, when no path is defined yet
- `use_file_compression` · boolean — Enable file compression when saving .blend files
- `use_load_ui` · boolean — Load user interface setup when loading .blend files
- `use_scripts_auto_execute` · boolean — Allow any .blend file to run scripts automatically (unsafe with blend files from an untrusted source)
- `use_tabs_as_spaces` · boolean — Automatically convert all new tabs into spaces for new and loaded text files
- `use_extension_online_access_handled` · boolean — The user has been shown the "Online Access" prompt and made a choice
- `font_directory` · string — The default directory to search for loading fonts
- `texture_directory` · string — The default directory to search for textures
- `render_output_directory` · string — The default directory for rendering output, for new scenes
- `script_directories` · collection · [] of ScriptDirectoryCollection · read-only
- `i18n_branches_directory` · string — The path to the '/branches' directory of your local svn-translation copy, to allow translating from the UI
- `sound_directory` · string — The default directory to search for sounds
- `temporary_directory` · string — The directory for storing temporary save files. The path must reference an existing directory or it will be ignored
- `render_cache_directory` · string — Where to cache raw render results
- `texture_cache_directory` · string — The directory for storing tx files generated from image files, for more efficient rendering. Paths may be absolute, or relative to the image file. Leave blank to store tx files in the same directory as image files
- `image_editor` · string — Path to an image editor
- `text_editor` · string — Command to launch the text editor, either a full path or a command in $PATH.
Use the internal editor when left blank
- `text_editor_args` · string — Defines the specific format of the arguments with which the text editor opens files. The supported expansions are as follows:

$filepath The absolute path of the file.
$line The line to open at (Optional).
$column The column to open from the beginning of the line (Optional).
$line0 & column0 start at zero.
Example: -f $filepath -l $line -c $column
- `animation_player` · string — Path to a custom animation/frame sequence player
- `animation_player_preset` · enum · {INTERNAL, DJV, FRAMECYCLER, RV, MPLAYER, CUSTOM} — Preset configs for external animation players
- `save_version` · int — The number of old versions to maintain in the current directory, when manually saving
- `use_auto_save_temporary_files` · boolean — Automatic saving of temporary files in temp directory, uses process ID.
Warning: Sculpt and edit mode data won't be saved
- `save_modified_images` · enum · {ASK, ALWAYS_SAVE, NEVER_SAVE} — How modified images should be handled when saving the .blend file
- `auto_save_time` · int — The time (in minutes) to wait between automatic temporary saves
- `recent_files` · int — Maximum number of recently opened files to remember
- `file_preview_type` · enum · {NONE, AUTO, SCREENSHOT, CAMERA} — What type of blend preview to create
- `asset_libraries` · collection · [] of AssetLibraryCollection · read-only
- `active_asset_library` · int — Index of the asset library being edited in the Preferences UI

## PreferencesInput

Settings for input devices

- `view_zoom_method` · enum · {CONTINUE, DOLLY, SCALE} — Which style to use for viewport scaling
- `view_zoom_axis` · enum · {VERTICAL, HORIZONTAL} — Axis of mouse movement to zoom in or out on
- `use_multitouch_gestures` · boolean — Use multi-touch gestures for navigation with touchpad, instead of scroll wheel emulation
- `invert_mouse_zoom` · boolean — Invert the axis of mouse movement for zooming
- `use_mouse_depth_navigate` · boolean — Use the depth under the mouse to improve view pan/rotate/zoom functionality
- `use_zoom_to_mouse` · boolean — Zoom in towards the mouse pointer's position in the 3D view, rather than the 2D window center
- `use_auto_perspective` · boolean — Automatically switch between orthographic and perspective when changing from top/front/side views
- `use_rotate_around_active` · boolean — Use the selection (or the last stroke center in Paint modes) as the pivot point for orbiting
- `view_rotate_method` · enum · {TURNTABLE, TRACKBALL} — Orbit method in the viewport
- `use_mouse_continuous` · boolean — Let the mouse wrap around the view boundaries so mouse movements are not limited by the screen size (used by transform, dragging of UI controls, etc.)
- `use_drag_immediately` · boolean — Moving things with a mouse drag confirms when releasing the button
- `use_numeric_input_advanced` · boolean — When entering numbers while transforming, default to advanced mode for full math expression evaluation
- `navigation_mode` · enum · {WALK, FLY} — Which method to use for viewport navigation
- `walk_navigation` · pointer · -> WalkNavigation · read-only — Settings for walk navigation mode
- `view_rotate_sensitivity_turntable` · float — Rotation amount per pixel to control how fast the viewport orbits
- `view_rotate_sensitivity_trackball` · float — Scale trackball orbit sensitivity
- `drag_threshold_mouse` · int — Number of pixels to drag before a drag event is triggered for mouse/trackpad input (otherwise click events are detected)
- `drag_threshold_tablet` · int — Number of pixels to drag before a drag event is triggered for tablet input (otherwise click events are detected)
- `drag_threshold` · int — Number of pixels to drag before a drag event is triggered for keyboard and other non mouse/tablet input (otherwise click events are detected)
- `move_threshold` · int — Number of pixels to before the cursor is considered to have moved (used for cycling selected items on successive clicks)
- `pressure_threshold_max` · float — Raw input pressure value that is interpreted as 100% by Blender
- `pressure_softness` · float — Adjusts softness of the low pressure response onset using a gamma curve
- `tablet_api` · enum · {AUTOMATIC, WINDOWS_INK, WINTAB} — Select the tablet API to use for pressure sensitivity (may require restarting Blender for changes to take effect)
- `show_tablet_debug_values` · boolean — Show pressure values when using a paint operator
- `xr_navigation` · pointer · -> XrNavigation · read-only — Settings for navigation in XR
- `ndof_translation_sensitivity` · float — Overall sensitivity of the 3D Mouse for translation
- `ndof_rotation_sensitivity` · float — Overall sensitivity of the 3D Mouse for rotation
- `ndof_deadzone` · float — Threshold of initial movement needed from the device's rest position
- `ndof_zoom_direction` · enum · {NDOF_ZOOM_FORWARD, NDOF_ZOOM_UP} — Which axis of the 3D Mouse cap zooms the view
- `ndof_show_guide_orbit_axis` · boolean — Display the center and axis during rotation
- `ndof_show_guide_orbit_center` · boolean — Display the orbit center during rotation
- `ndof_navigation_mode` · enum · {OBJECT, FLY, DRONE} — 3D Mouse Navigation Mode
- `ndof_lock_horizon` · boolean — Lock Horizon forces the horizon to be kept leveled as it currently is
- `ndof_fly_speed_auto` · boolean — Automatically adjusts fly navigation speed based on the distance of objects near the center of the viewport, making it easier to navigate complex scenes. Speed is recalculated each time movement starts.
- `ndof_orbit_center_auto` · boolean — Auto sets the orbit center dynamically. When the complete model is in view, the center of volume of the whole model is used as the rotation point. When you move closer, the orbit center will be set on an object close to your center of the view.
- `ndof_orbit_center_selected` · boolean — Selected Item forces the orbit center to only take the currently selected objects into account.
- `ndof_rotx_invert_axis` · boolean
- `ndof_roty_invert_axis` · boolean
- `ndof_rotz_invert_axis` · boolean
- `ndof_panx_invert_axis` · boolean
- `ndof_pany_invert_axis` · boolean
- `ndof_panz_invert_axis` · boolean
- `ndof_fly_helicopter` · boolean — Device up/down directly controls the Z position of the 3D viewport
- `ndof_lock_camera_pan_zoom` · boolean — Pan/zoom the camera view instead of leaving the camera view when orbiting
- `mouse_double_click_time` · int — Time/delay (in ms) for a double click
- `use_mouse_emulate_3_button` · boolean — Emulate Middle Mouse with Alt+Left Mouse
- `mouse_emulate_3_button_modifier` · enum · {ALT, OSKEY} — Hold this modifier to emulate the middle mouse button
- `use_emulate_numpad` · boolean — Main 1 to 0 keys act as the numpad ones (useful for laptops)
- `invert_zoom_wheel` · boolean — Swap the Mouse Wheel zoom direction
- `touchpad_scroll_direction` · enum · {TRADITIONAL, NATURAL} — Scroll direction (Wayland only)

## PreferencesSystem

Graphics driver and operating system settings

- `ui_scale` · float · read-only — Size multiplier to use when displaying custom user interface elements, so that they are scaled correctly on screens with different DPI. This value is based on operating system DPI settings and Blender display scale.
- `ui_line_width` · float · read-only — Suggested line thickness and point size in pixels, for add-ons displaying custom user interface elements, based on operating system settings and Blender UI scale
- `dpi` · int · read-only
- `pixel_size` · float · read-only
- `memory_cache_limit` · int — Memory cache limit (in megabytes)
- `geometry_nodes_stack_limit` · int — Approximate maximum size of the call stack used by Geometry Nodes. For example, this corresponds to the number of allowed nested node groups. Setting this too high can result in crashes caused by running out of stack memory.
- `sequencer_proxy_setup` · enum · {MANUAL, AUTOMATIC} — When and how proxies are created
- `scrollback` · int — Maximum number of lines to store for the console buffer
- `use_overlay_smooth_wire` · boolean — Enable overlay smooth wires, reducing aliasing
- `use_edit_mode_smooth_wire` · boolean — Enable edit mode edge smoothing, reducing aliasing (requires restart)
- `use_region_overlap` · boolean — Display tool/property regions over the main region
- `show_panel_tabs_compact` · boolean — Display sidebar tabs in a compact size that shows icons when available
- `viewport_aa` · enum · {OFF, FXAA, 5, 8, 11, 16, 32} — Method of anti-aliasing in 3d viewport
- `solid_lights` · collection · read-only — Lights used to display objects in solid shading mode
- `light_ambient` · float · [3] — Color of the ambient light that uniformly lit the scene
- `use_studio_light_edit` · boolean — View the result of the studio light editor in the viewport
- `gl_clip_alpha` · float — Clip alpha below this threshold in the 3D textured view
- `anisotropic_filter` · enum · {FILTER_0, FILTER_2, FILTER_4, FILTER_8, FILTER_16} — Quality of anisotropic filtering
- `gl_texture_limit` · enum · {CLAMP_OFF, CLAMP_8192, CLAMP_4096, CLAMP_2048, CLAMP_1024, CLAMP_512, CLAMP_256, CLAMP_128} — Limit the texture size to save graphics memory
- `texture_time_out` · int — Time since last access of a GL texture in seconds after which it is freed (set to 0 to keep textures allocated)
- `texture_collection_rate` · int — Number of seconds between each run of the GL texture garbage collector
- `vbo_time_out` · int — Time since last access of a GL vertex buffer object in seconds after which it is freed (set to 0 to keep VBO allocated)
- `vbo_collection_rate` · int — Number of seconds between each run of the GL vertex buffer object garbage collector
- `use_gpu_subdivision` · boolean — Enable GPU acceleration for evaluating the last subdivision surface modifiers in the stack
- `gpu_backend` · enum · {OPENGL, METAL, VULKAN} — GPU backend to use (requires restarting Blender for changes to take effect)
- `gpu_preferred_device` · enum · {AUTO} — Preferred device to select during detection (requires restarting Blender for changes to take effect)
- `gpu_shader_workers` · int — Number of shader compilation threads or subprocesses, clamped at the max threads supported by the CPU (requires restarting Blender for changes to take effect). A higher number increases the RAM usage while reducing compilation time. A value of 0 will use automatic configuration. (OpenGL only)
- `shader_compilation_method` · enum · {THREAD, SUBPROCESS} — Compilation method used for compiling shaders in parallel. Subprocess requires a lot more RAM for each worker but might compile shaders faster on some systems. Requires restarting Blender for changes to take effect. (OpenGL only)
- `use_online_access` · boolean — Allow Blender to access the internet. Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule.
- `network_timeout` · int — The time in seconds to wait for online operations before a connection may fail with a time-out error. Zero uses the systems default.
- `network_connection_limit` · int — Limit the number of simultaneous internet connections online operations may make at once. Zero disables the limit.
- `audio_mixing_buffer` · enum · {SAMPLES_256, SAMPLES_512, SAMPLES_1024, SAMPLES_2048, SAMPLES_4096, SAMPLES_8192, SAMPLES_16384, SAMPLES_32768} — Number of samples used by the audio mixing buffer
- `audio_device` · enum · {None} — Audio output device
- `audio_sample_rate` · enum · {RATE_44100, RATE_48000, RATE_96000, RATE_192000} — Audio sample rate
- `audio_sample_format` · enum · {U8, S16, S24, S32, FLOAT, DOUBLE} — Audio sample format
- `audio_channels` · enum · {MONO, STEREO, SURROUND4, SURROUND51, SURROUND71} — Audio channel count
- `legacy_compute_device_type` · int · read-only — For backwards compatibility only
- `register_all_users` · boolean — Make this Blender version open blend files for all users. Requires elevated privileges.
- `is_microsoft_store_install` · boolean · read-only — Whether this blender installation is a sandboxed Microsoft Store version

## ThemePreferences

Theme settings for the Blender Preferences

- `match` · float · [3]
- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
