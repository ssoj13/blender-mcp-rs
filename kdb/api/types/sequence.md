---
area: api-types
group: sequence
version: 5.2.2
---

# Blender 5.2.2 RNA types: sequence

## ColorManagedSequencerColorspaceSettings

Input color space settings

- `name` · enum · {ACES 1.3 sRGB, ACES 2.0 sRGB, ACES2065-1, ACEScc, ACEScct, ACEScg, ADX10, ADX16, ARRI LogC3 (EI800), ARRI LogC4, AgX Base sRGB, AgX Log, Apple Log, Apple Log 2, BMDFilm WideGamut Gen5, CanonLog2 CinemaGamut D55, CanonLog3 CinemaGamut D55, D-Log D-Gamut, DaVinci Intermediate WideGamut, Display P3, Filmic Log, Filmic sRGB, Gamma 1.8 Encoded Rec.709, Gamma 2.2 Encoded AP1, Gamma 2.2 Encoded AdobeRGB, Gamma 2.2 Encoded Rec.709, Gamma 2.4 Encoded Rec.709, Khronos PBR Neutral sRGB, Linear ARRI Wide Gamut 3, Linear ARRI Wide Gamut 4, Linear Adobe RGB, Linear Apple Wide Gamut, Linear BMD WideGamut Gen5, Linear CIE-XYZ D65, Linear CIE-XYZ E, Linear CinemaGamut D55, Linear D-Gamut, Linear DCI-P3 D65, Linear DaVinci WideGamut, Linear FilmLight E-Gamut, ... (63 total)} — Color space that the sequencer operates in

## SequenceEditor

Sequence editing data for a Scene data-block

- `strips` · collection · [] of StripsTopLevel · read-only — Top-level strips only
- `strips_all` · collection · read-only — All strips, recursively including those inside metastrips
- `meta_stack` · collection · read-only — Meta strip stack, last is currently edited meta strip
- `channels` · collection · read-only
- `active_strip` · pointer · -> Strip — Sequencer's active strip
- `selected_retiming_keys` · boolean · read-only
- `show_overlay_frame` · boolean — Partial overlay on top of the sequencer with a frame offset
- `use_overlay_frame_lock` · boolean
- `show_missing_media` · boolean — Render missing images/movies with a solid magenta color
- `overlay_frame` · int — Number of frames to offset
- `proxy_storage` · enum · {PER_STRIP, PROJECT} — How to store proxies for this project
- `proxy_dir` · string
- `use_cache_raw` · boolean — Cache raw images read from disk, for faster tweaking of strip parameters at the cost of memory usage
- `use_cache_final` · boolean — Cache final image for each frame
- `use_prefetch` · boolean — Render frames ahead of current frame in the background for faster playback
- `cache_raw_size` · int · read-only — Size of raw source images cache in megabytes
- `cache_final_size` · int · read-only — Size of final rendered images cache in megabytes

## SequenceTimelineChannel

- `name` · string
- `number` · int · read-only — Channel number
- `lock` · boolean
- `mute` · boolean

## SequencerCacheOverlay

- `show_cache` · boolean — Visualize cached images on the timeline
- `show_cache_final_out` · boolean — Visualize cached complete frames
- `show_cache_raw` · boolean — Visualize cached raw images

## SequencerPreviewOverlay

- `show_safe_areas` · boolean — Show TV title safe and action safe areas in preview
- `show_safe_center` · boolean — Show safe areas to fit content in a different aspect ratio
- `show_composition_center` · boolean — Display center composition guide
- `show_composition_center_diagonal` · boolean — Display diagonal center composition guide
- `show_composition_thirds` · boolean — Display rule of thirds composition guide
- `show_composition_golden` · boolean — Display golden ratio composition guide
- `show_composition_golden_tria_a` · boolean — Display golden triangle A composition guide
- `show_composition_golden_tria_b` · boolean — Display golden triangle B composition guide
- `show_composition_harmony_tri_a` · boolean — Display harmony A composition guide
- `show_composition_harmony_tri_b` · boolean — Display harmony B composition guide
- `composition_guide_color` · float · [4] — Color and alpha for compositional guide overlays
- `show_metadata` · boolean — Show metadata of first visible strip
- `show_annotation` · boolean — Show annotations for this view
- `show_image_outline` · boolean
- `show_cursor` · boolean
- `show_composition_guides` · boolean — Display composition guides over the preview

## SequencerTimelineOverlay

- `waveform_display_type` · enum · {ALL_WAVEFORMS, DEFAULT_WAVEFORMS, NO_WAVEFORMS} — How Waveforms are displayed
- `waveform_display_style` · enum · {FULL_WAVEFORMS, HALF_WAVEFORMS} — How Waveforms are displayed
- `thumbnail_display_style` · enum · {NO_THUMBNAILS, STRIP_ENDS, CONTINUOUS} — How thumbnails are displayed
- `show_fcurves` · boolean — Display strip opacity/volume curve
- `show_strip_name` · boolean
- `show_strip_source` · boolean — Display path to source file, or name of source data-block
- `show_strip_duration` · boolean
- `show_grid` · boolean — Show vertical grid lines
- `show_strip_offset` · boolean — Display strip in/out offsets
- `show_strip_tag_color` · boolean — Display the strip color tags in the sequencer
- `show_strip_retiming` · boolean — Display retiming keys on top of strips

## SequencerToolSettings

- `fit_method` · enum · {FIT, FILL, STRETCH, ORIGINAL} — Scale fit method
- `snap_to_current_frame` · boolean — Snap to current frame
- `snap_to_hold_offset` · boolean — Snap to underlying strip content start and end in cases where the strip length extends beyond this range, producing holds
- `snap_to_markers` · boolean — Snap to markers
- `snap_to_retiming_keys` · boolean — Snap to retiming keys
- `snap_to_frame_range` · boolean — Snap to preview or scene start and end frame
- `snap_to_borders` · boolean — Snap to preview borders
- `snap_to_center` · boolean — Snap to preview center
- `snap_to_strips_preview` · boolean — Snap to borders and origins of deselected, visible strips
- `snap_ignore_muted` · boolean — Don't snap to hidden strips
- `snap_ignore_sound` · boolean — Don't snap to sound strips
- `use_snap_current_frame_to_strips` · boolean — Snap current frame to strip start or end
- `snap_to_all_channels` · boolean — Allow snapping to any channel. If disabled, only snap to strips currently on the same channel as transformed strips
- `snap_distance` · int — Maximum distance for snapping in pixels
- `overlap_mode` · enum · {EXPAND, OVERWRITE, SHUFFLE} — How to resolve overlap after transformation
- `pivot_point` · enum · {CENTER, MEDIAN, CURSOR, INDIVIDUAL_ORIGINS} — Rotation or scaling pivot point

## SpaceSequenceEditor

Sequence editor space data

- `show_region_tool_header` · boolean
- `show_region_footer` · boolean
- `show_region_toolbar` · boolean
- `show_region_channels` · boolean
- `show_region_ui` · boolean
- `show_region_hud` · boolean
- `view_type` · enum · {SEQUENCER, PREVIEW, SEQUENCER_PREVIEW} — Type of the Sequencer view (sequencer, preview or both)
- `display_mode` · enum · {IMAGE, WAVEFORM, RGB_PARADE, VECTOR_SCOPE, HISTOGRAM} — View mode to use for displaying sequencer output
- `show_frames` · boolean — Display frames rather than seconds
- `use_marker_sync` · boolean — Transform markers as well as strips
- `show_seconds` · boolean — Show timing as a timecode instead of frames
- `show_markers` · boolean — If any exists, show markers in a separate row at the bottom of the editor
- `display_channel` · int — Preview all channels less than or equal to this value. 0 shows every channel, and negative values climb that many meta-strip levels if applicable, showing every channel there.
- `preview_channels` · enum · {COLOR_ALPHA, COLOR} — Channels of the preview to display
- `use_zoom_to_fit` · boolean — Automatically zoom preview image to make it fully fit the region
- `show_overexposed` · int — Show overexposed areas with zebra stripes
- `proxy_render_size` · enum · {NONE, SCENE, PROXY_25, PROXY_50, PROXY_75, PROXY_100} — Display preview using full resolution or different proxy resolutions
- `use_proxies` · boolean — Use optimized files for faster scrubbing when available
- `use_clamp_view` · boolean — Limit timeline height to maximum used channel slot
- `show_scrubbing_region` · boolean — Region with full playback range for scrubbing in the sequencer
- `annotation` · pointer · -> Annotation — Annotation data for this Preview region
- `overlay_frame_type` · enum · {RECTANGLE, REFERENCE, CURRENT} — Overlay display method
- `show_transform_preview` · boolean — Show a preview of the start or end frame of a strip while transforming its respective handle
- `show_gizmo` · boolean — Show gizmos of all types
- `show_gizmo_navigate` · boolean — Viewport navigation gizmo
- `show_gizmo_context` · boolean — Context sensitive gizmos for the active item
- `show_gizmo_tool` · boolean — Active tool gizmo
- `show_overlays` · boolean
- `preview_overlay` · pointer · -> SequencerPreviewOverlay · read-only — Settings for display of overlays
- `timeline_overlay` · pointer · -> SequencerTimelineOverlay · read-only — Settings for display of overlays
- `cache_overlay` · pointer · -> SequencerCacheOverlay · read-only — Settings for display of overlays
- `cursor_location` · float · [2] — 2D cursor location for this view
- `zoom_percentage` · float — Zoom percentage

## ThemeSequenceEditor

Theme settings for the Sequence Editor

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `grid` · float · [3]
- `movie_strip` · float · [3]
- `movieclip_strip` · float · [3]
- `image_strip` · float · [3]
- `scene_strip` · float · [3]
- `audio_strip` · float · [3]
- `effect_strip` · float · [3]
- `transition_strip` · float · [3]
- `color_strip` · float · [3]
- `meta_strip` · float · [3]
- `mask_strip` · float · [3]
- `text_strip` · float · [3]
- `active_strip` · float · [3]
- `selected_strip` · float · [3]
- `keyframe_border` · float · [4] — Color of keyframe border
- `keyframe_border_selected` · float · [4] — Color of selected keyframe border
- `preview_back` · float · [3]
- `metadatabg` · float · [3]
- `metadatatext` · float · [3]
- `row_alternate` · float · [4] — Overlay color on every other row
- `text_strip_cursor` · float · [4] — Text strip editing cursor
- `selected_text` · float · [4] — Text strip editing selection
