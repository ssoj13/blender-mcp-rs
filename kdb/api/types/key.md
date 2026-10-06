---
area: api-types
group: key
version: 5.2.2
---

# Blender 5.2.2 RNA types: key

## Key

Shape keys data-block containing different shapes of geometric data-blocks

- `reference_key` · pointer · -> ShapeKey · read-only
- `key_blocks` · collection · read-only — Shape keys
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `user` · pointer · -> ID · read-only — Data-block using these shape keys
- `use_relative` · boolean — Make shape keys relative, otherwise play through shapes as a sequence using the evaluation time
- `eval_time` · float — Evaluation time for absolute shape keys

## KeyConfig

Input configuration, including keymaps

- `name` · string — Name of the key configuration
- `keymaps` · collection · [] of KeyMaps · read-only — Key maps configured as part of this configuration
- `is_user_defined` · boolean · read-only — Indicates that a keyconfig was defined by the user
- `preferences` · pointer · -> KeyConfigPreferences · read-only

## KeyConfigPreferences

- `bl_idname` · string

## KeyConfigurations

Collection of KeyConfigs

- `active` · pointer · -> KeyConfig — Active key configuration (preset)
- `default` · pointer · -> KeyConfig · read-only — Default builtin key configuration
- `addon` · pointer · -> KeyConfig · read-only — Key configuration that can be extended by add-ons, and is added to the active configuration when handling events
- `user` · pointer · -> KeyConfig · read-only — Final key configuration that combines keymaps from the active and add-on configurations, and can be edited by the user

## KeyMap

Input configuration, including keymaps

- `name` · string · read-only — Name of the key map
- `bl_owner_id` · string — Internal owner
- `space_type` · enum · {EMPTY, VIEW_3D, IMAGE_EDITOR, NODE_EDITOR, SEQUENCE_EDITOR, CLIP_EDITOR, DOPESHEET_EDITOR, GRAPH_EDITOR, NLA_EDITOR, TEXT_EDITOR, CONSOLE, INFO, TOPBAR, STATUSBAR, OUTLINER, PROPERTIES, FILE_BROWSER, SPREADSHEET, PREFERENCES} · read-only — Optional space type keymap is associated with
- `region_type` · enum · {WINDOW, HEADER, CHANNELS, TEMPORARY, UI, TOOLS, TOOL_PROPS, ASSET_SHELF, ASSET_SHELF_HEADER, PREVIEW, HUD, NAVIGATION_BAR, EXECUTE, FOOTER, TOOL_HEADER, XR, SCRUBBING} · read-only — Optional region type keymap is associated with
- `keymap_items` · collection · [] of KeyMapItems · read-only — Items in the keymap, linking an operator to an input event
- `is_user_modified` · boolean — Keymap is defined by the user
- `is_modal` · boolean · read-only — Indicates that a keymap is used for translate modal events for an operator
- `show_expanded_items` · boolean — Expanded in the user interface
- `show_expanded_children` · boolean — Children expanded in the user interface
- `modal_event_values` · collection · read-only — Give access to the possible event values of this modal keymap's items (#KeyMapItem.propvalue), for API introspection

## KeyMapItem

Item in a Key Map

- `idname` · string — Identifier of operator to call on input event
- `name` · string · read-only — Name of operator (translated) to call on input event
- `properties` · pointer · -> OperatorProperties · read-only — Properties to set when the operator is called
- `map_type` · enum · {KEYBOARD, MOUSE, NDOF, TEXTINPUT, TIMER} — Type of event mapping
- `type` · enum · {NONE, LEFTMOUSE, MIDDLEMOUSE, RIGHTMOUSE, BUTTON4MOUSE, BUTTON5MOUSE, BUTTON6MOUSE, BUTTON7MOUSE, PEN, ERASER, MOUSEMOVE, INBETWEEN_MOUSEMOVE, TRACKPADPAN, TRACKPADZOOM, MOUSEROTATE, MOUSESMARTZOOM, WHEELUPMOUSE, WHEELDOWNMOUSE, WHEELINMOUSE, WHEELOUTMOUSE, WHEELLEFTMOUSE, WHEELRIGHTMOUSE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, ... (196 total)} — Type of event
- `value` · enum · {ANY, PRESS, RELEASE, CLICK, DOUBLE_CLICK, CLICK_DRAG, NOTHING}
- `direction` · enum · {ANY, NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST} — The direction (only applies to drag events)
- `id` · int · read-only — ID of the item
- `any` · boolean — Any modifier keys pressed
- `shift` · int — Shift key pressed, -1 for any state
- `ctrl` · int — Control key pressed, -1 for any state
- `alt` · int — Alt key pressed, -1 for any state
- `oskey` · int — Operating system key pressed, -1 for any state
- `hyper` · int — Hyper key pressed, -1 for any state
- `shift_ui` · boolean — Shift key pressed
- `ctrl_ui` · boolean — Control key pressed
- `alt_ui` · boolean — Alt key pressed
- `oskey_ui` · boolean — Operating system key pressed
- `hyper_ui` · boolean — Hyper key pressed. An additional modifier which can be configured on Linux, typically replacing CapsLock
- `key_modifier` · enum · {NONE, LEFTMOUSE, MIDDLEMOUSE, RIGHTMOUSE, BUTTON4MOUSE, BUTTON5MOUSE, BUTTON6MOUSE, BUTTON7MOUSE, PEN, ERASER, MOUSEMOVE, INBETWEEN_MOUSEMOVE, TRACKPADPAN, TRACKPADZOOM, MOUSEROTATE, MOUSESMARTZOOM, WHEELUPMOUSE, WHEELDOWNMOUSE, WHEELINMOUSE, WHEELOUTMOUSE, WHEELLEFTMOUSE, WHEELRIGHTMOUSE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, ... (196 total)} — Regular key pressed as a modifier
- `repeat` · boolean — Active on key-repeat events (when a key is held)
- `show_expanded` · boolean — Show key map event and property details in the user interface
- `propvalue` · enum · {NONE} — The value this event translates to in a modal keymap
- `active` · boolean — Activate or deactivate item
- `is_user_modified` · boolean · read-only — Is this keymap item modified by the user
- `is_user_defined` · boolean · read-only — Is this keymap item user defined (does not just replace a builtin item)

## Keyframe

Bézier curve point with two handles defining a Keyframe on an F-Curve

- `select_left_handle` · boolean — Left handle selection status
- `select_right_handle` · boolean — Right handle selection status
- `select_control_point` · boolean — Control point selection status
- `handle_left_type` · enum · {FREE, ALIGNED, VECTOR, AUTO, AUTO_CLAMPED} — Handle types
- `handle_right_type` · enum · {FREE, ALIGNED, VECTOR, AUTO, AUTO_CLAMPED} — Handle types
- `interpolation` · enum · {CONSTANT, LINEAR, BEZIER, SINE, QUAD, CUBIC, QUART, QUINT, EXPO, CIRC, BACK, BOUNCE, ELASTIC} — Interpolation method to use for segment of the F-Curve from this Keyframe until the next Keyframe
- `type` · enum · {KEYFRAME, BREAKDOWN, MOVING_HOLD, EXTREME, JITTER, GENERATED} — Type of keyframe (for visual purposes only)
- `easing` · enum · {AUTO, EASE_IN, EASE_OUT, EASE_IN_OUT} — Which ends of the segment between this and the next keyframe easing interpolation is applied to
- `back` · float — Amount of overshoot for 'back' easing
- `amplitude` · float — Amount to boost elastic bounces for 'elastic' easing
- `period` · float — Time between bounces for elastic easing
- `handle_left` · float · [2] — Coordinates of the left handle (before the control point)
- `co` · float · [2] — Coordinates of the control point
- `co_ui` · float · [2] — Coordinates of the control point. Note: Changing this value also updates the handles similar to using the graph editor transform operator
- `handle_right` · float · [2] — Coordinates of the right handle (after the control point)

## KeyingSet

Settings that should be keyframed together

- `bl_idname` · string — If this is set, the Keying Set gets a custom ID, otherwise it takes the name of the class used to define the Keying Set (for example, if the class name is "BUILTIN_KSI_location", and bl_idname is not set by the script, then bl_idname = "BUILTIN_KSI_location")
- `bl_label` · string
- `bl_description` · string — A short description of the keying set
- `type_info` · pointer · -> KeyingSetInfo · read-only — Callback function defines for built-in Keying Sets
- `paths` · collection · [] of KeyingSetPaths · read-only — Keying Set Paths to define settings that get keyframed together
- `is_path_absolute` · boolean · read-only — Keying Set defines specific paths/settings to be keyframed (i.e. is not reliant on context info)
- `use_insertkey_override_needed` · boolean — Override default setting to only insert keyframes where they're needed in the relevant F-Curves
- `use_insertkey_override_visual` · boolean — Override default setting to insert keyframes based on 'visual transforms'
- `use_insertkey_needed` · boolean — Only insert keyframes where they're needed in the relevant F-Curves
- `use_insertkey_visual` · boolean — Insert keyframes based on 'visual transforms'

## KeyingSetInfo

Callback function defines for builtin Keying Sets

- `bl_idname` · string — If this is set, the Keying Set gets a custom ID, otherwise it takes the name of the class used to define the Keying Set (for example, if the class name is "BUILTIN_KSI_location", and bl_idname is not set by the script, then bl_idname = "BUILTIN_KSI_location")
- `bl_label` · string
- `bl_description` · string — A short description of the keying set
- `bl_options` · enum · {INSERTKEY_NEEDED, INSERTKEY_VISUAL} — Keying Set options to use when inserting keyframes

## KeyingSetPath

Path to a setting for use in a Keying Set

- `id` · pointer · -> ID — ID-Block that keyframes for Keying Set should be added to (for Absolute Keying Sets only)
- `id_type` · enum · {ACTION, ARMATURE, BRUSH, CACHEFILE, CAMERA, COLLECTION, CURVE, CURVES, FONT, GREASEPENCIL, GREASEPENCIL_V3, IMAGE, KEY, LATTICE, LIBRARY, LIGHT, LIGHT_PROBE, LINESTYLE, MASK, MATERIAL, MESH, META, MOVIECLIP, NODETREE, OBJECT, PAINTCURVE, PALETTE, PARTICLE, POINTCLOUD, SCENE, SCREEN, SOUND, SPEAKER, TEXT, TEXTURE, VOLUME, WINDOWMANAGER, WORKSPACE, WORLD} — Type of ID-block that can be used
- `group` · string — Name of Action Group to assign setting(s) for this path to
- `group_method` · enum · {NAMED, NONE, KEYINGSET} — Method used to define which Group-name to use
- `data_path` · string — Path to property setting
- `array_index` · int — Index to the specific setting if applicable
- `use_entire_array` · boolean — When an 'array/vector' type is chosen (Location, Rotation, Color, etc.), entire array is to be used
- `use_insertkey_override_needed` · boolean — Override default setting to only insert keyframes where they're needed in the relevant F-Curves
- `use_insertkey_override_visual` · boolean — Override default setting to insert keyframes based on 'visual transforms'
- `use_insertkey_needed` · boolean — Only insert keyframes where they're needed in the relevant F-Curves
- `use_insertkey_visual` · boolean — Insert keyframes based on 'visual transforms'

## KeyingSetPaths

Collection of keying set paths

- `active` · pointer · -> KeyingSetPath — Active Keying Set used to insert/delete keyframes
- `active_index` · int — Current Keying Set index

## KeyingSets

Scene keying sets

- `active` · pointer · -> KeyingSet — Active Keying Set used to insert/delete keyframes
- `active_index` · int — Current Keying Set index (negative for 'builtin' and positive for 'absolute')

## KeyingSetsAll

All available keying sets

- `active` · pointer · -> KeyingSet — Active Keying Set used to insert/delete keyframes
- `active_index` · int — Current Keying Set index (negative for 'builtin' and positive for 'absolute')

## PreferencesKeymap

Shortcut setup for keyboards and other input devices

- `show_ui_keyconfig` · boolean
- `active_keyconfig` · string — The name of the active key configuration

## RetimingKey

Key mapped to particular frame that can be moved to change playback speed

- `timeline_frame` · int — Position of retiming key in timeline

## ShapeKey

Shape key in a shape keys data-block

- `name` · string — Name of Shape Key
- `select` · boolean — Shape key selection state
- `frame` · float · read-only — Frame for absolute keys
- `value` · float — Value of shape key at the current frame
- `interpolation` · enum · {KEY_LINEAR, KEY_CARDINAL, KEY_CATMULL_ROM, KEY_BSPLINE} — Interpolation type for absolute shape keys
- `vertex_group` · string — Vertex weight group, to blend with basis shape
- `relative_key` · pointer · -> ShapeKey — Shape used as a relative key
- `mute` · boolean — Toggle this shape key
- `lock_shape` · boolean — Protect the shape key from accidental sculpting and editing
- `slider_min` · float — Minimum for slider
- `slider_max` · float — Maximum for slider
- `data` · collection · read-only
- `points` · collection · read-only — Optimized access to shape keys point data, when using foreach_get/foreach_set accessors. Warning: Does not support legacy Curve shape keys.

## ShapeKeyBezierPoint

Point in a shape key for Bézier curves

- `co` · float · [3]
- `handle_left` · float · [3]
- `handle_right` · float · [3]
- `tilt` · float — Tilt in 3D View
- `radius` · float — Radius for beveling

## ShapeKeyPoint

Point in a shape key

- `co` · float · [3]
