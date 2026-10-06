---
area: api-types
group: action
version: 5.2.2
---

# Blender 5.2.2 RNA types: action

## Action

A collection of F-Curves for animation

- `is_empty` · boolean · read-only — False when there is any Layer, Slot, or legacy F-Curve
- `is_action_legacy` · boolean · read-only — Return whether this is a legacy Action. Legacy Actions have no layers or slots. Since Blender 4.4 actions are automatically updated to layered actions. This will only return true on empty actions
- `is_action_layered` · boolean · read-only — Return whether this is a layered Action. At this point all actions are layered through versioning and this function will always return true
- `slots` · collection · [] of ActionSlots · read-only — The list of slots in this Action
- `layers` · collection · [] of ActionLayers · read-only — The list of layers that make up this Action
- `pose_markers` · collection · [] of ActionPoseMarkers · read-only — Markers specific to this action, for labeling poses
- `use_frame_range` · boolean — Manually specify the intended playback frame range for the action (this range is used by some tools, but does not affect animation evaluation)
- `use_cyclic` · boolean — The action is intended to be used as a cycle looping over its manually set playback frame range (enabling this does not automatically make it loop)
- `frame_start` · float — The start frame of the manually set intended playback range
- `frame_end` · float — The end frame of the manually set intended playback range
- `frame_range` · float · [2] — The intended playback frame range of this action, using the manually set range if available, or the combined frame range of all F-Curves within this action if not (assigning sets the manual frame range)
- `curve_frame_range` · float · [2] · read-only — The combined frame range of all F-Curves within this action

## ActionChannelbag

Collection of animation channels, typically associated with an action slot

- `slot_handle` · int · read-only
- `slot` · pointer · -> ActionSlot · read-only — The Slot that the Channelbag's animation data is for
- `fcurves` · collection · [] of ActionChannelbagFCurves · read-only — The individual F-Curves that animate the slot
- `groups` · collection · [] of ActionChannelbagGroups · read-only — Groupings of F-Curves for display purposes, in e.g. the dopesheet and graph editor

## ActionGroup

Groups of F-Curves

- `name` · string
- `channels` · collection · read-only — F-Curves in this group
- `select` · boolean — Action group is selected
- `lock` · boolean — Action group is locked
- `mute` · boolean — Action group is muted
- `show_expanded` · boolean — Action group is expanded except in graph editor
- `show_expanded_graph` · boolean — Action group is expanded in graph editor
- `use_pin` · boolean
- `color_set` · enum · {DEFAULT, THEME01, THEME02, THEME03, THEME04, THEME05, THEME06, THEME07, THEME08, THEME09, THEME10, THEME11, THEME12, THEME13, THEME14, THEME15, THEME16, THEME17, THEME18, THEME19, THEME20, CUSTOM} — Custom color set to use
- `is_custom_color_set` · boolean · read-only — Color set is user-defined instead of a fixed theme color set
- `colors` · pointer · -> ThemeBoneColorSet · read-only — Copy of the colors associated with the group's color set

## ActionLayer

- `name` · string
- `strips` · collection · [] of ActionStrips · read-only — The list of strips that are on this animation layer

## ActionPoseMarkers

Collection of timeline markers

- `active` · pointer · -> TimelineMarker — Active pose marker for this action
- `active_index` · int — Index of active pose marker

## ActionSlot

Identifier for a set of channels in this Action, that can be used by a data-block to specify what it gets animated by

- `identifier` · string — Used when connecting an Action to a data-block, to find the correct slot handle. This is the display name, prefixed by two characters determined by the slot's ID type
- `target_id_type` · enum · {ACTION, ARMATURE, BRUSH, CACHEFILE, CAMERA, COLLECTION, CURVE, CURVES, FONT, GREASEPENCIL, GREASEPENCIL_V3, IMAGE, KEY, LATTICE, LIBRARY, LIGHT, LIGHT_PROBE, LINESTYLE, MASK, MATERIAL, MESH, META, MOVIECLIP, NODETREE, OBJECT, PAINTCURVE, PALETTE, PARTICLE, POINTCLOUD, SCENE, SCREEN, SOUND, SPEAKER, TEXT, TEXTURE, VOLUME, WINDOWMANAGER, WORKSPACE, WORLD, UNSPECIFIED} — Type of data-block that this slot is intended to animate; can be set when 'UNSPECIFIED' but is otherwise read-only
- `target_id_type_icon` · int · read-only
- `name_display` · string — Name of the slot, for display in the user interface. This name combined with the slot's data-block type is unique within its Action
- `handle` · int · read-only — Number specific to this Slot, unique within the Action.
This is used, for example, on a ActionKeyframeStrip to look up the ActionChannelbag for this Slot
- `active` · boolean · read-only — Whether this is the active slot, can be set by assigning to action.slots.active
- `select` · boolean — Selection state of the slot
- `show_expanded` · boolean — Expanded state of the slot

## ActionSlots

Collection of action slots

- `active` · pointer · -> ActionSlot — Active slot for this action

## XrActionMap

- `name` · string — Name of the action map
- `actionmap_items` · collection · [] of XrActionMapItems · read-only — Items in the action map, mapping an XR event to an operator, pose, or haptic output
- `selected_item` · int

## XrActionMapBinding

Binding in an XR action map item

- `name` · string — Name of the action map binding
- `profile` · string — OpenXR interaction profile path
- `component_paths` · collection · [] of XrComponentPaths · read-only — OpenXR component paths
- `threshold` · float — Input threshold for button/axis actions
- `axis0_region` · enum · {ANY, POSITIVE, NEGATIVE} — Action execution region for the first input axis
- `axis1_region` · enum · {ANY, POSITIVE, NEGATIVE} — Action execution region for the second input axis
- `pose_location` · float · [3]
- `pose_rotation` · float · [3]

## XrActionMapItem

- `name` · string — Name of the action map item
- `type` · enum · {FLOAT, VECTOR2D, POSE, VIBRATION} — Action type
- `user_paths` · collection · [] of XrUserPaths · read-only — OpenXR user paths
- `op` · string — Identifier of operator to call on action event
- `op_name` · string · read-only — Name of operator (translated) to call on action event
- `op_properties` · pointer · -> OperatorProperties · read-only — Properties to set when the operator is called
- `op_mode` · enum · {PRESS, RELEASE, MODAL} — Operator execution mode
- `bimanual` · boolean — The action depends on the states/poses of both user paths
- `pose_is_controller_grip` · boolean — The action poses will be used for the VR controller grips
- `pose_is_controller_aim` · boolean — The action poses will be used for the VR controller aims
- `haptic_name` · string — Name of the haptic action to apply when executing this action
- `haptic_match_user_paths` · boolean — Apply haptics to the same user paths for the haptic action and this action
- `haptic_duration` · float — Haptic duration in seconds. 0.0 is the minimum supported duration.
- `haptic_frequency` · float — Frequency of the haptic vibration in hertz. 0.0 specifies the OpenXR runtime's default frequency.
- `haptic_amplitude` · float — Intensity of the haptic vibration, ranging from 0.0 to 1.0
- `haptic_mode` · enum · {PRESS, RELEASE, PRESS_RELEASE, REPEAT} — Haptic application mode
- `bindings` · collection · [] of XrActionMapBindings · read-only — Bindings for the action map item, mapping the action to an XR input
- `selected_binding` · int — Currently selected binding
