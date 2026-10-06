---
area: api-types
group: nla
version: 5.2.2
---

# Blender 5.2.2 RNA types: nla

## AnnotationLayer

Collection of related sketches

- `info` · string — Layer name
- `frames` · collection · [] of AnnotationFrames · read-only — Sketches for this layer on different frames
- `active_frame` · pointer · -> AnnotationFrame · read-only — Frame currently being displayed for this layer
- `annotation_opacity` · float — Annotation Layer Opacity
- `color` · float · [3] — Color for all strokes in this layer
- `thickness` · int — Thickness of annotation strokes
- `use_annotation_onion_skinning` · boolean — Display annotation onion skins before and after the current frame
- `annotation_onion_before_range` · int — Maximum number of frames to show before current frame
- `annotation_onion_after_range` · int — Maximum number of frames to show after current frame
- `annotation_onion_before_color` · float · [3] — Base color for ghosts before the active frame
- `annotation_onion_after_color` · float · [3] — Base color for ghosts after the active frame
- `annotation_onion_use_custom_color` · boolean — Use custom colors for onion skinning instead of the theme
- `annotation_hide` · boolean — Set annotation Visibility
- `lock` · boolean — Protect layer from further editing and/or frame changes
- `lock_frame` · boolean — Lock current frame displayed by layer
- `is_ruler` · boolean · read-only — This is a special ruler layer
- `select` · boolean — Layer is selected for editing in the Dope Sheet
- `show_in_front` · boolean — Make the layer display in front of objects

## AnnotationLayers

Collection of annotation layers

- `active_index` · int — Index of active annotation layer
- `active_note` · enum · {DEFAULT} — Note/Layer to add annotation strokes to

## NlaTrack

An animation layer containing Actions referenced as NLA strips

- `strips` · collection · [] of NlaStrips · read-only — NLA Strips on this NLA-track
- `is_override_data` · boolean · read-only — In a local override data, whether this NLA track comes from the linked reference data, or is local to the override
- `name` · string
- `active` · boolean · read-only — NLA Track is active
- `is_solo` · boolean — NLA Track is evaluated itself (i.e. active Action and all other NLA Tracks in the same AnimData block are disabled)
- `select` · boolean — NLA Track is selected
- `mute` · boolean — Disable NLA Track evaluation
- `lock` · boolean — NLA Track is locked

## NlaTracks

Collection of NLA Tracks

- `active` · pointer · -> NlaTrack — Active NLA Track

## SpaceNLA

NLA editor space data

- `show_region_footer` · boolean
- `show_region_channels` · boolean
- `show_region_ui` · boolean
- `show_region_hud` · boolean
- `show_seconds` · boolean — Show timing as a timecode instead of frames
- `show_strip_curves` · boolean — Show influence F-Curves on strips
- `show_local_markers` · boolean — Show action-local markers on the strips, useful when synchronizing timing across strips
- `show_markers` · boolean — If any exists, show markers in a separate row at the bottom of the editor
- `use_realtime_update` · boolean — When transforming strips, changes to the animation data are flushed to other views
- `dopesheet` · pointer · -> DopeSheet · read-only — Settings for filtering animation data

## ThemeNLAEditor

Theme settings for the NLA Editor

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `grid` · float · [3]
- `active_action` · float · [4] — Animation data-block has active action
- `active_action_unset` · float · [4] — Animation data-block does not have active action
- `strips` · float · [3] — Unselected Action-Clip Strip
- `strips_selected` · float · [3] — Selected Action-Clip Strip
- `transition_strips` · float · [3] — Unselected Transition Strip
- `transition_strips_selected` · float · [3] — Selected Transition Strip
- `meta_strips` · float · [3] — Unselected Meta Strip (for grouping related strips)
- `meta_strips_selected` · float · [3] — Selected Meta Strip (for grouping related strips)
- `sound_strips` · float · [3] — Unselected Sound Strip (for timing speaker sounds)
- `sound_strips_selected` · float · [3] — Selected Sound Strip (for timing speaker sounds)
- `tweak` · float · [3] — Color for strip/action being "tweaked" or edited
- `tweak_duplicate` · float · [3] — Warning/error indicator color for strips referencing the strip being tweaked
- `keyframe_border` · float · [4] — Color of keyframe border
- `keyframe_border_selected` · float · [4] — Color of selected keyframe border
