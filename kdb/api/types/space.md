---
area: api-types
group: space
version: 5.2.2
---

# Blender 5.2.2 RNA types: space

## AreaSpaces

Collection of spaces

- `active` · pointer · -> Space · read-only — Space currently being displayed in this area

## BlendFileColorspace

Information about the color space used for data-blocks in a blend file

- `working_space` · enum · {Linear Rec.709, Linear Rec.2020, ACEScg} · read-only — Color space used for all scene linear colors in this file, and for compositing, shader and geometry nodes processing
- `working_space_interop_id` · string · read-only — Unique identifier for common color spaces, as defined by the Color Interop Forum. May be empty if there is no interop ID for the working space. Common values are lin_rec709_scene, lin_rec2020_scene and lin_ap1_scene (for ACEScg)
- `is_missing_opencolorio_config` · boolean · read-only — A color space, view or display was not found, which likely means the OpenColorIO config used to create this blend file is missing

## ColorManagedInputColorspaceSettings

Input color space settings

- `name` · enum · {ACES 1.3 sRGB, ACES 2.0 sRGB, ACES2065-1, ACEScc, ACEScct, ACEScg, ADX10, ADX16, ARRI LogC3 (EI800), ARRI LogC4, AgX Base sRGB, AgX Log, Apple Log, Apple Log 2, BMDFilm WideGamut Gen5, CanonLog2 CinemaGamut D55, CanonLog3 CinemaGamut D55, D-Log D-Gamut, DaVinci Intermediate WideGamut, Display P3, Filmic Log, Filmic sRGB, Gamma 1.8 Encoded Rec.709, Gamma 2.2 Encoded AP1, Gamma 2.2 Encoded AdobeRGB, Gamma 2.2 Encoded Rec.709, Gamma 2.4 Encoded Rec.709, Khronos PBR Neutral sRGB, Linear ARRI Wide Gamut 3, Linear ARRI Wide Gamut 4, Linear Adobe RGB, Linear Apple Wide Gamut, Linear BMD WideGamut Gen5, Linear CIE-XYZ D65, Linear CIE-XYZ E, Linear CinemaGamut D55, Linear D-Gamut, Linear DCI-P3 D65, Linear DaVinci WideGamut, Linear FilmLight E-Gamut, ... (63 total)} — Color space in the image file, to convert to and from when saving and loading the image
- `is_data` · boolean — Treat image as non-color data without color management, like normal or displacement maps

## Space

Space data for a screen area

- `type` · enum · {EMPTY, VIEW_3D, IMAGE_EDITOR, NODE_EDITOR, SEQUENCE_EDITOR, CLIP_EDITOR, DOPESHEET_EDITOR, GRAPH_EDITOR, NLA_EDITOR, TEXT_EDITOR, CONSOLE, INFO, TOPBAR, STATUSBAR, OUTLINER, PROPERTIES, FILE_BROWSER, SPREADSHEET, PREFERENCES} · read-only — Space data type
- `show_locked_time` · boolean — Synchronize the visible timeline range with other time-based editors
- `show_region_header` · boolean

## SpaceClipEditor

Clip editor space data

- `show_region_toolbar` · boolean
- `show_region_channels` · boolean
- `show_region_ui` · boolean
- `show_region_hud` · boolean
- `clip` · pointer · -> MovieClip — Movie clip displayed and edited in this space
- `clip_user` · pointer · -> MovieClipUser · read-only — Parameters defining which frame of the movie clip is displayed
- `mask` · pointer · -> Mask — Mask displayed and edited in this space
- `mask_display_type` · enum · {OUTLINE, DASH, BLACK, WHITE} — Display type for mask splines
- `show_mask_spline` · boolean
- `show_mask_overlay` · boolean
- `mask_overlay_mode` · enum · {ALPHACHANNEL, COMBINED} — Overlay mode of rasterized mask
- `blend_factor` · float — Overlay blending factor of rasterized mask
- `mode` · enum · {TRACKING, MASK} — Editing context being displayed
- `view` · enum · {CLIP, GRAPH, DOPESHEET} — Type of the clip editor view
- `show_marker_pattern` · boolean — Show pattern boundbox for markers
- `show_marker_search` · boolean — Show search boundbox for markers
- `lock_selection` · boolean — Lock viewport to selected markers during playback
- `lock_time_cursor` · boolean — Lock curves view to time cursor during playback and tracking
- `show_track_path` · boolean — Show path of how track moves
- `path_length` · int — Length of displaying path, in frames
- `show_tiny_markers` · boolean — Show markers in a more compact manner
- `show_bundles` · boolean — Show projection of 3D markers into footage
- `use_mute_footage` · boolean — Mute footage and show black background instead
- `show_disabled` · boolean — Show disabled tracks from the footage
- `show_metadata` · boolean — Show metadata of clip
- `scopes` · pointer · -> MovieClipScopes · read-only — Scopes to visualize movie clip statistics
- `show_names` · boolean — Show track names and status
- `show_grid` · boolean — Show grid showing lens distortion
- `show_stable` · boolean — Show stable footage in editor (if stabilization is enabled)
- `use_manual_calibration` · boolean — Use manual calibration helpers
- `show_annotation` · boolean — Show annotations for this view
- `show_filters` · boolean — Show filters for graph editor
- `show_graph_frames` · boolean — Show curve for per-frame average error (camera motion should be solved first)
- `show_graph_tracks_motion` · boolean — Display speed curves for the selected tracks
- `show_graph_tracks_error` · boolean — Display the reprojection error curve for selected tracks
- `show_graph_only_selected` · boolean — Only include channels relating to selected objects and data
- `show_graph_hidden` · boolean — Include channels from objects/bone that are not visible
- `show_red_channel` · boolean — Show red channel in the frame
- `show_green_channel` · boolean — Show green channel in the frame
- `show_blue_channel` · boolean — Show blue channel in the frame
- `use_grayscale_preview` · boolean — Display frame in grayscale mode
- `show_seconds` · boolean — Show timing as a timecode instead of frames
- `annotation_source` · enum · {CLIP, TRACK} — Where the annotation comes from
- `cursor_location` · float · [2] — 2D cursor location for this view
- `pivot_point` · enum · {BOUNDING_BOX_CENTER, CURSOR, INDIVIDUAL_ORIGINS, MEDIAN_POINT} — Pivot center for rotation/scaling
- `show_gizmo` · boolean — Show gizmos of all types
- `show_gizmo_navigate` · boolean — Viewport navigation gizmo
- `zoom_percentage` · float — Zoom percentage
- `overlay` · pointer · -> SpaceClipOverlay · read-only — Settings for display of overlays in the Movie Clip editor

## SpaceClipOverlay

Settings for display of overlays in the Movie Clip editor

- `show_overlays` · boolean — Display overlays like cursor and annotations
- `show_cursor` · boolean — Display 2D cursor

## SpaceConsole

Interactive Python console

- `font_size` · int — Font size to use for displaying the text
- `select_start` · int
- `select_end` · int
- `prompt` · string — Command line prompt
- `language` · string — Command line prompt language
- `history` · collection · read-only — Command history
- `scrollback` · collection · read-only — Command output

## SpaceDopeSheetEditor

Dope Sheet space data

- `show_region_footer` · boolean
- `show_region_channels` · boolean
- `show_region_ui` · boolean
- `show_region_hud` · boolean
- `mode` · enum · {DOPESHEET, ACTION, SHAPEKEY, GPENCIL, MASK, CACHEFILE, TIMELINE} — Editing context being displayed
- `ui_mode` · enum · {DOPESHEET, ACTION, SHAPEKEY, GPENCIL, MASK, CACHEFILE} — Editing context being displayed
- `show_seconds` · boolean — Show timing as a timecode instead of frames
- `show_sliders` · boolean — Show sliders beside F-Curve channels
- `show_pose_markers` · boolean — Show markers belonging to the active action instead of Scene markers (Action and Shape Key Editors only)
- `show_interpolation` · boolean — Display keyframe handle types and non-Bézier interpolation modes
- `show_extremes` · boolean — Mark keyframes where the key value flow changes direction, based on comparison with adjacent keys
- `show_markers` · boolean — If any exists, show markers in a separate row at the bottom of the editor
- `use_auto_merge_keyframes` · boolean — Automatically merge nearby keyframes
- `use_realtime_update` · boolean — When transforming keyframes, changes to the animation data are flushed to other views
- `use_marker_sync` · boolean — Sync Markers with keyframe edits
- `dopesheet` · pointer · -> DopeSheet · read-only — Settings for filtering animation data
- `show_cache` · boolean — Show the status of cached frames in the timeline
- `cache_softbody` · boolean — Show the active object's softbody point cache
- `cache_particles` · boolean — Show the active object's particle point cache
- `cache_cloth` · boolean — Show the active object's cloth point cache
- `cache_smoke` · boolean — Show the active object's smoke cache
- `cache_simulation_nodes` · boolean — Show the active object's simulation nodes cache and bake data
- `cache_dynamicpaint` · boolean — Show the active object's Dynamic Paint cache
- `cache_rigidbody` · boolean — Show the active object's Rigid Body cache
- `overlays` · pointer · -> SpaceDopeSheetOverlay · read-only — Settings for display of overlays

## SpaceDopeSheetOverlay

- `show_overlays` · boolean — Display overlays
- `show_scene_strip_range` · boolean — When using scene time synchronization in the sequence editor, display the range of the current scene strip

## SpaceFileBrowser

File browser space data

- `show_region_toolbar` · boolean
- `show_region_tool_props` · boolean
- `show_region_ui` · boolean
- `browse_mode` · enum · {FILES, ASSETS} — Type of the File Editor view (regular file browsing or asset browsing)
- `params` · pointer · -> FileSelectParams · read-only — Parameters and Settings for the Filebrowser
- `active_operator` · pointer · -> Operator · read-only
- `operator` · pointer · -> Operator · read-only
- `system_folders` · collection · read-only — System's folders (usually root, available hard drives, etc)
- `system_folders_active` · int — Index of active system folder (-1 if none)
- `system_bookmarks` · collection · read-only — System's bookmarks
- `system_bookmarks_active` · int — Index of active system bookmark (-1 if none)
- `bookmarks` · collection — User's bookmarks
- `bookmarks_active` · int — Index of active bookmark (-1 if none)
- `recent_folders` · collection
- `recent_folders_active` · int — Index of active recent folder (-1 if none)

## SpaceGraphEditor

Graph Editor space data

- `show_region_footer` · boolean
- `show_region_channels` · boolean
- `show_region_ui` · boolean
- `show_region_hud` · boolean
- `mode` · enum · {FCURVES, DRIVERS} — Editing context being displayed
- `show_seconds` · boolean — Show timing as a timecode instead of frames
- `show_sliders` · boolean — Show sliders beside F-Curve channels
- `show_handles` · boolean — Show handles of Bézier control points
- `use_auto_lock_translation_axis` · boolean — Automatically locks the movement of keyframes to the dominant axis
- `use_only_selected_keyframe_handles` · boolean — Only show and edit handles of selected keyframes
- `show_markers` · boolean — If any exists, show markers in a separate row at the bottom of the editor
- `show_extrapolation` · boolean
- `use_auto_merge_keyframes` · boolean — Automatically merge nearby keyframes
- `use_realtime_update` · boolean — When transforming keyframes, changes to the animation data are flushed to other views
- `show_cursor` · boolean — Show 2D cursor
- `cursor_position_x` · float — Graph Editor 2D-Value cursor - X-Value component
- `cursor_position_y` · float — Graph Editor 2D-Value cursor - Y-Value component
- `pivot_point` · enum · {BOUNDING_BOX_CENTER, CURSOR, INDIVIDUAL_ORIGINS} — Pivot center for rotation/scaling
- `dopesheet` · pointer · -> DopeSheet · read-only — Settings for filtering animation data
- `has_ghost_curves` · boolean · read-only — Graph Editor instance has some ghost curves stored
- `use_normalization` · boolean — Display curves in normalized range from -1 to 1, for easier editing of multiple curves with different ranges
- `use_auto_normalization` · boolean — Automatically recalculate curve normalization on every curve edit

## SpaceInfo

Info space data

- `show_report_debug` · boolean — Display debug reporting info
- `show_report_info` · boolean — Display general information
- `show_report_operator` · boolean — Display the operator log
- `show_report_warning` · boolean — Display warnings
- `show_report_error` · boolean — Display error text

## SpaceOutliner

Outliner space data

- `display_mode` · enum · {SCENES, VIEW_LAYER, SEQUENCE, LIBRARIES, DATA_API, LIBRARY_OVERRIDES, ORPHAN_DATA} — Type of information to display
- `lib_override_view_mode` · enum · {PROPERTIES, HIERARCHIES} — Choose different visualizations of library override data
- `filter_text` · string — Live search filtering string
- `use_filter_case_sensitive` · boolean — Only use case sensitive matches of search string
- `use_filter_complete` · boolean — Only use complete matches of search string
- `use_sort_alpha` · boolean
- `use_sync_select` · boolean — Sync outliner selection with other editors
- `show_mode_column` · boolean — Show the mode column for mode toggle and activation
- `scroll_to_active` · boolean — Scroll the active item into view when it changes outside of the Outliner
- `show_restrict_column_enable` · boolean — Exclude from view layer
- `show_restrict_column_select` · boolean — Selectable
- `show_restrict_column_hide` · boolean — Temporarily hide in viewport
- `show_restrict_column_viewport` · boolean — Globally disable in viewports
- `show_restrict_column_render` · boolean — Globally disable in renders
- `show_restrict_column_holdout` · boolean — Holdout
- `show_restrict_column_indirect_only` · boolean — Indirect only
- `use_filter_object` · boolean — Show objects
- `use_filter_object_content` · boolean — Show what is inside the objects elements
- `use_filter_children` · boolean — Show children
- `use_filter_collection` · boolean — Show collections
- `use_filter_view_layers` · boolean — Show all the view layers
- `filter_state` · enum · {ALL, VISIBLE, SELECTED, ACTIVE, SELECTABLE}
- `filter_invert` · boolean — Invert the object state filter
- `use_filter_object_mesh` · boolean — Show mesh objects
- `use_filter_object_armature` · boolean — Show armature objects
- `use_filter_object_empty` · boolean — Show empty objects
- `use_filter_object_light` · boolean — Show light objects
- `use_filter_object_camera` · boolean — Show camera objects
- `use_filter_object_grease_pencil` · boolean — Show Grease Pencil objects
- `use_filter_object_others` · boolean — Show curves, lattices, light probes, fonts, ...
- `use_filter_id_type` · boolean — Show only data-blocks of one type
- `filter_id_type` · enum · {ACTION, ARMATURE, BRUSH, CACHEFILE, CAMERA, COLLECTION, CURVE, CURVES, FONT, GREASEPENCIL, GREASEPENCIL_V3, IMAGE, KEY, LATTICE, LIBRARY, LIGHT, LIGHT_PROBE, LINESTYLE, MASK, MATERIAL, MESH, META, MOVIECLIP, NODETREE, OBJECT, PAINTCURVE, PALETTE, PARTICLE, POINTCLOUD, SCENE, SCREEN, SOUND, SPEAKER, TEXT, TEXTURE, VOLUME, WINDOWMANAGER, WORKSPACE, WORLD} — Data-block type to show
- `use_filter_lib_override_system` · boolean — For libraries with overrides created, show the overridden values that are defined/controlled automatically (e.g. to make users of an overridden data-block point to the override data, not the original linked data)

## SpacePreferences

Blender preferences space data

- `show_region_ui` · boolean
- `filter_type` · enum · {NAME, KEY} — Filter method
- `filter_text` · string — Search term for filtering in the UI
- `tab_search_results` · boolean · read-only — Whether or not each visible tab has a search result
- `search_filter` · string — Live search filtering string

## SpaceProperties

Properties space data

- `context` · enum · {TOOL, SCENE, RENDER, OUTPUT, VIEW_LAYER, WORLD, COLLECTION, OBJECT, CONSTRAINT, MODIFIER, DATA, BONE, BONE_CONSTRAINT, MATERIAL, TEXTURE, PARTICLES, PHYSICS, SHADERFX, STRIP, STRIP_MODIFIER}
- `show_properties_tool` · boolean
- `show_properties_scene` · boolean
- `show_properties_render` · boolean
- `show_properties_output` · boolean
- `show_properties_view_layer` · boolean
- `show_properties_world` · boolean
- `show_properties_collection` · boolean
- `show_properties_object` · boolean
- `show_properties_constraints` · boolean
- `show_properties_modifiers` · boolean
- `show_properties_data` · boolean
- `show_properties_bone` · boolean
- `show_properties_bone_constraints` · boolean
- `show_properties_material` · boolean
- `show_properties_texture` · boolean
- `show_properties_particles` · boolean
- `show_properties_physics` · boolean
- `show_properties_effects` · boolean
- `show_properties_strip` · boolean
- `show_properties_strip_modifier` · boolean
- `pin_id` · pointer · -> ID
- `use_pin_id` · boolean — Use the pinned context
- `tab_search_results` · boolean · read-only — Whether or not each visible tab has a search result
- `search_filter` · string — Live search filtering string
- `outliner_sync` · enum · {ALWAYS, NEVER, AUTO} — Change to the corresponding tab when outliner data icons are clicked

## SpaceSpreadsheet

Spreadsheet space data

- `show_region_footer` · boolean
- `show_region_toolbar` · boolean
- `show_region_channels` · boolean
- `show_region_ui` · boolean
- `is_pinned` · boolean — Context path is pinned
- `show_internal_attributes` · boolean — Display attributes with names starting with a period that are meant for internal use
- `use_filter` · boolean
- `viewer_path` · pointer · -> ViewerPath · read-only — Path to the data that is displayed in the spreadsheet
- `show_only_selected` · boolean — Only include rows that correspond to selected elements
- `geometry_item_type` · enum · {DOMAIN, BUNDLE} — Item Type
- `geometry_component_type` · enum · {MESH, POINTCLOUD, CURVE, INSTANCES, GREASEPENCIL, VOLUME} — Part of the geometry to display data from
- `attribute_domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Attribute domain to display
- `object_eval_state` · enum · {EVALUATED, ORIGINAL, VIEWER_NODE}
- `tables` · collection · [] of SpreadsheetTables · read-only — Persistent data for the tables shown in this spreadsheet editor
- `row_filters` · collection · read-only — Filters to remove rows from the displayed data

## SpaceTextEditor

Text editor space data

- `show_region_footer` · boolean
- `show_region_ui` · boolean
- `text` · pointer · -> Text — Text displayed and edited in this space
- `show_word_wrap` · boolean — Wrap words if there is not enough horizontal space
- `show_line_numbers` · boolean — Show line numbers next to the text
- `show_syntax_highlight` · boolean — Syntax highlight for scripting
- `show_line_highlight` · boolean — Highlight the current line
- `tab_width` · int — Number of spaces to display tabs with
- `font_size` · int — Font size to use for displaying the text
- `show_margin` · boolean — Show right margin
- `margin_column` · int — Column number to show right margin at
- `top` · int — Top line visible
- `visible_lines` · int · read-only — Amount of lines that can be visible in current editor
- `use_overwrite` · boolean — Overwrite characters when typing rather than inserting them
- `use_live_edit` · boolean — Run Python while editing
- `use_find_all` · boolean — Search in all text data-blocks, instead of only the active one
- `use_find_wrap` · boolean — Search again from the start of the file when reaching the end
- `use_match_case` · boolean — Search string is sensitive to uppercase and lowercase letters
- `find_text` · string — Text to search for with the find tool
- `replace_text` · string — Text to replace selected text with using the replace tool

## SpaceUVEditor

UV editor data for the image editor space

- `edge_display_type` · enum · {OUTLINE, DASH, BLACK, WHITE} — Display style for UV edges
- `show_stretch` · boolean — Display faces colored according to the difference in shape between UVs and their 3D coordinates (blue for low distortion, red for high distortion)
- `display_stretch_type` · enum · {ANGLE, AREA} — Type of stretch to display
- `show_modified_edges` · boolean — Display edges after modifiers are applied
- `show_metadata` · boolean — Display metadata properties of the image
- `show_uv` · boolean — Display overlay of UV layer
- `show_pixel_coords` · boolean — Display UV coordinates in pixels rather than from 0.0 to 1.0
- `show_faces` · boolean — Display faces over the image
- `tile_grid_shape` · int · [2] — How many tiles will be shown in the background
- `show_grid_over_image` · boolean — Show the grid over the image
- `grid_shape_source` · enum · {DYNAMIC, FIXED, PIXEL} — Specify source for the grid shape
- `custom_grid_subdivisions` · int · [2] — Number of grid units in UV space that make one UV Unit
- `uv_opacity` · float — Opacity of UV overlays
- `uv_face_opacity` · float — Opacity of faces in UV overlays
- `uv_edge_opacity` · float — Opacity of edges in UV overlays
- `stretch_opacity` · float — Opacity of the UV Stretch overlay
- `pixel_round_mode` · enum · {DISABLED, CORNER, CENTER} — Round UVs to pixels while editing
- `lock_bounds` · boolean — Constraint to stay within the image bounds while editing
- `use_live_unwrap` · boolean — Continuously unwrap the selected UV island while transforming pinned vertices

## ThemeSpaceGeneric

- `back` · float · [3]
- `title` · float · [3]
- `text` · float · [3]
- `text_hi` · float · [3]
- `header` · float · [4]
- `header_text` · float · [3]
- `header_text_hi` · float · [3]

## ThemeSpaceGradient

- `gradients` · pointer · -> ThemeGradientColors · read-only
- `title` · float · [3]
- `text` · float · [3]
- `text_hi` · float · [3]
- `header` · float · [4]
- `header_text` · float · [3]
- `header_text_hi` · float · [3]

## WorkSpace

Workspace data-block, defining the working environment for the user

- `screens` · collection · read-only — Screen layouts of a workspace
- `owner_ids` · collection · [] of wmOwnerIDs · read-only
- `tools` · collection · [] of wmTools · read-only
- `object_mode` · enum · {OBJECT, EDIT, POSE, SCULPT, VERTEX_PAINT, WEIGHT_PAINT, TEXTURE_PAINT, PARTICLE_EDIT, EDIT_GPENCIL, SCULPT_GREASE_PENCIL, PAINT_GREASE_PENCIL, VERTEX_GREASE_PENCIL, WEIGHT_GREASE_PENCIL} — Switch to this object mode when activating the workspace
- `use_pin_scene` · boolean — Remember the last used scene for the workspace and switch to it whenever this workspace is activated again
- `use_filter_by_owner` · boolean — Filter the UI by tags
- `asset_library_reference` · enum · {ALL, LOCAL, ESSENTIALS, ONLINE_ESSENTIALS, CUSTOM} — Active asset library to show in the UI, not used by the Asset Browser (which has its own active asset library)
- `sequencer_scene` · pointer · -> Scene
- `use_scene_time_sync` · boolean — Set the active scene and time based on the current scene strip
- `active_addon` · int — Active Add-on in the Workspace Add-ons filter

## WorkSpaceTool

- `idname` · string
- `idname_fallback` · string
- `index` · int · read-only
- `space_type` · enum · {EMPTY, VIEW_3D, IMAGE_EDITOR, NODE_EDITOR, SEQUENCE_EDITOR, CLIP_EDITOR, DOPESHEET_EDITOR, GRAPH_EDITOR, NLA_EDITOR, TEXT_EDITOR, CONSOLE, INFO, TOPBAR, STATUSBAR, OUTLINER, PROPERTIES, FILE_BROWSER, SPREADSHEET, PREFERENCES} · read-only
- `mode` · enum · {DEFAULT} · read-only
- `use_paint_canvas` · boolean · read-only — Does this tool use a painting canvas
- `has_datablock` · boolean · read-only
- `use_brushes` · boolean · read-only
- `brush_type` · enum · {DEFAULT} · read-only — If the tool uses brushes and is limited to a specific brush type, the identifier of the brush type
- `widget` · string · read-only
