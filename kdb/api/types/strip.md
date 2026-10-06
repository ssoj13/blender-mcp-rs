---
area: api-types
group: strip
version: 5.2.2
---

# Blender 5.2.2 RNA types: strip

## ActionKeyframeStrip

Strip with a set of F-Curves for each action slot

- `channelbags` · collection · [] of ActionChannelbags · read-only

## ActionStrip

- `type` · enum · {KEYFRAME} · read-only

## AddStrip

Add Strip

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `input_2` · pointer · -> Strip — Second input for the effect strip

## AdjustmentStrip

Sequence strip to perform filter adjustments to layers below

- `input_count` · int · read-only
- `animation_offset_start` · int — Animation start offset (trim start)
- `content_trim_start` · int — Number of frames to ignore from the start of the underlying source. The source content is trimmed, and previous frames are turned into holds
- `animation_offset_end` · int — Animation end offset (trim end)
- `content_trim_end` · int — Number of frames to ignore from the end of the underlying source. The source content is trimmed, and future frames are turned into holds

## AlphaOverStrip

Alpha Over Strip

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `input_2` · pointer · -> Strip — Second input for the effect strip

## AlphaUnderStrip

Alpha Under Strip

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `input_2` · pointer · -> Strip — Second input for the effect strip

## ColorMixStrip

Color Mix Strip

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `input_2` · pointer · -> Strip — Second input for the effect strip
- `blend_effect` · enum · {DARKEN, MULTIPLY, BURN, LINEAR_BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, HARD_LIGHT, VIVID_LIGHT, LINEAR_LIGHT, PIN_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, HUE, SATURATION, COLOR, VALUE} — Method for controlling how the strip combines with other strips
- `factor` · float — Percentage of how much the strip's colors affect other strips

## ColorStrip

Sequence strip creating an image filled with a single color

- `input_count` · int · read-only
- `color` · float · [3] — Effect Strip color
- `width` · int — Width of the color strip in pixels
- `height` · int — Height of the color strip in pixels

## CompositorStrip

Compositor Strip

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `input_2` · pointer · -> Strip — Second input for the effect strip
- `node_group` · pointer · -> NodeTree — Node group that controls what this effect does

## CrossStrip

Crossfade Strip

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `input_2` · pointer · -> Strip — Second input for the effect strip

## EffectStrip

Sequence strip applying an effect on the images created by other strips

- `use_deinterlace` · boolean — Remove fields from video movies
- `alpha_mode` · enum · {STRAIGHT, PREMUL} — Representation of alpha information in the RGBA pixels
- `use_flip_x` · boolean — Flip on the X axis
- `use_flip_y` · boolean — Flip on the Y axis
- `use_float` · boolean — Convert input to float data
- `use_reverse_frames` · boolean — Reverse frame order
- `color_multiply` · float
- `multiply_alpha` · boolean — Multiply alpha along with color channels
- `color_saturation` · float — Adjust the intensity of the input's color
- `strobe` · float — Only display every nth frame
- `transform` · pointer · -> StripTransform · read-only
- `crop` · pointer · -> StripCrop · read-only
- `use_proxy` · boolean — Use a preview proxy for this strip
- `proxy` · pointer · -> StripProxy · read-only

## GammaCrossStrip

Gamma Crossfade Strip

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `input_2` · pointer · -> Strip — Second input for the effect strip

## GaussianBlurStrip

Sequence strip creating a gaussian blur

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `size_x` · float — Size of the blur along X axis
- `size_y` · float — Size of the blur along Y axis

## GlowStrip

Sequence strip creating a glow effect

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `threshold` · float — Minimum intensity to trigger a glow
- `clamp` · float — Brightness limit of intensity
- `boost_factor` · float — Brightness multiplier
- `blur_radius` · float — Radius of glow effect
- `quality` · int — Accuracy of the blur effect
- `use_only_boost` · boolean — Show the glow buffer only

## ImageStrip

Sequence strip to load one or more images

- `directory` · string
- `elements` · collection · [] of StripElements · read-only
- `retiming_keys` · collection · [] of RetimingKeys · read-only
- `use_multiview` · boolean — Use Multiple Views (when available)
- `views_format` · enum · {INDIVIDUAL, STEREO_3D} — Mode to load image views
- `stereo_3d_format` · pointer · -> Stereo3dFormat · read-only — Settings for stereo 3D
- `use_deinterlace` · boolean — Remove fields from video movies
- `alpha_mode` · enum · {STRAIGHT, PREMUL} — Representation of alpha information in the RGBA pixels
- `use_flip_x` · boolean — Flip on the X axis
- `use_flip_y` · boolean — Flip on the Y axis
- `use_float` · boolean — Convert input to float data
- `use_reverse_frames` · boolean — Reverse frame order
- `color_multiply` · float
- `multiply_alpha` · boolean — Multiply alpha along with color channels
- `color_saturation` · float — Adjust the intensity of the input's color
- `strobe` · float — Only display every nth frame
- `transform` · pointer · -> StripTransform · read-only
- `crop` · pointer · -> StripCrop · read-only
- `use_proxy` · boolean — Use a preview proxy for this strip
- `proxy` · pointer · -> StripProxy · read-only
- `animation_offset_start` · int — Animation start offset (trim start)
- `content_trim_start` · int — Number of frames to ignore from the start of the underlying source. The source content is trimmed, and previous frames are turned into holds
- `animation_offset_end` · int — Animation end offset (trim end)
- `content_trim_end` · int — Number of frames to ignore from the end of the underlying source. The source content is trimmed, and future frames are turned into holds
- `colorspace_settings` · pointer · -> ColorManagedInputColorspaceSettings · read-only — Input color space settings

## MaskStrip

Sequence strip to load a video from a mask

- `mask` · pointer · -> Mask — Mask that this strip uses
- `use_deinterlace` · boolean — Remove fields from video movies
- `alpha_mode` · enum · {STRAIGHT, PREMUL} — Representation of alpha information in the RGBA pixels
- `use_flip_x` · boolean — Flip on the X axis
- `use_flip_y` · boolean — Flip on the Y axis
- `use_float` · boolean — Convert input to float data
- `use_reverse_frames` · boolean — Reverse frame order
- `color_multiply` · float
- `multiply_alpha` · boolean — Multiply alpha along with color channels
- `color_saturation` · float — Adjust the intensity of the input's color
- `strobe` · float — Only display every nth frame
- `transform` · pointer · -> StripTransform · read-only
- `crop` · pointer · -> StripCrop · read-only
- `animation_offset_start` · int — Animation start offset (trim start)
- `content_trim_start` · int — Number of frames to ignore from the start of the underlying source. The source content is trimmed, and previous frames are turned into holds
- `animation_offset_end` · int — Animation end offset (trim end)
- `content_trim_end` · int — Number of frames to ignore from the end of the underlying source. The source content is trimmed, and future frames are turned into holds

## MetaStrip

Sequence strip to group other strips as a single sequence strip

- `strips` · collection · [] of StripsMeta · read-only — Strips nested in meta strip
- `channels` · collection · read-only
- `volume` · float — Playback volume of the sound
- `use_deinterlace` · boolean — Remove fields from video movies
- `alpha_mode` · enum · {STRAIGHT, PREMUL} — Representation of alpha information in the RGBA pixels
- `use_flip_x` · boolean — Flip on the X axis
- `use_flip_y` · boolean — Flip on the Y axis
- `use_float` · boolean — Convert input to float data
- `use_reverse_frames` · boolean — Reverse frame order
- `color_multiply` · float
- `multiply_alpha` · boolean — Multiply alpha along with color channels
- `color_saturation` · float — Adjust the intensity of the input's color
- `strobe` · float — Only display every nth frame
- `transform` · pointer · -> StripTransform · read-only
- `crop` · pointer · -> StripCrop · read-only
- `use_proxy` · boolean — Use a preview proxy for this strip
- `proxy` · pointer · -> StripProxy · read-only
- `animation_offset_start` · int — Animation start offset (trim start)
- `content_trim_start` · int — Number of frames to ignore from the start of the underlying source. The source content is trimmed, and previous frames are turned into holds
- `animation_offset_end` · int — Animation end offset (trim end)
- `content_trim_end` · int — Number of frames to ignore from the end of the underlying source. The source content is trimmed, and future frames are turned into holds

## MovieClipStrip

Sequence strip to load a video from the clip editor

- `clip` · pointer · -> MovieClip — Movie clip that this strip uses
- `undistort` · boolean — Use the undistorted version of the clip
- `stabilize2d` · boolean — Use the 2D stabilized version of the clip
- `use_deinterlace` · boolean — Remove fields from video movies
- `alpha_mode` · enum · {STRAIGHT, PREMUL} — Representation of alpha information in the RGBA pixels
- `use_flip_x` · boolean — Flip on the X axis
- `use_flip_y` · boolean — Flip on the Y axis
- `use_float` · boolean — Convert input to float data
- `use_reverse_frames` · boolean — Reverse frame order
- `color_multiply` · float
- `multiply_alpha` · boolean — Multiply alpha along with color channels
- `color_saturation` · float — Adjust the intensity of the input's color
- `strobe` · float — Only display every nth frame
- `transform` · pointer · -> StripTransform · read-only
- `crop` · pointer · -> StripCrop · read-only
- `animation_offset_start` · int — Animation start offset (trim start)
- `content_trim_start` · int — Number of frames to ignore from the start of the underlying source. The source content is trimmed, and previous frames are turned into holds
- `animation_offset_end` · int — Animation end offset (trim end)
- `content_trim_end` · int — Number of frames to ignore from the end of the underlying source. The source content is trimmed, and future frames are turned into holds
- `fps` · float · read-only — Frames per second

## MovieStrip

Sequence strip to load a video

- `stream_index` · int — For files with several movie streams, use the stream with the given index
- `elements` · collection · read-only
- `retiming_keys` · collection · [] of RetimingKeys · read-only
- `filepath` · string
- `use_multiview` · boolean — Use Multiple Views (when available)
- `views_format` · enum · {INDIVIDUAL, STEREO_3D} — Mode to load movie views
- `stereo_3d_format` · pointer · -> Stereo3dFormat · read-only — Settings for stereo 3D
- `use_deinterlace` · boolean — Remove fields from video movies
- `alpha_mode` · enum · {STRAIGHT, PREMUL} — Representation of alpha information in the RGBA pixels
- `use_flip_x` · boolean — Flip on the X axis
- `use_flip_y` · boolean — Flip on the Y axis
- `use_float` · boolean — Convert input to float data
- `use_reverse_frames` · boolean — Reverse frame order
- `color_multiply` · float
- `multiply_alpha` · boolean — Multiply alpha along with color channels
- `color_saturation` · float — Adjust the intensity of the input's color
- `strobe` · float — Only display every nth frame
- `transform` · pointer · -> StripTransform · read-only
- `crop` · pointer · -> StripCrop · read-only
- `use_proxy` · boolean — Use a preview proxy for this strip
- `proxy` · pointer · -> StripProxy · read-only
- `animation_offset_start` · int — Animation start offset (trim start)
- `content_trim_start` · int — Number of frames to ignore from the start of the underlying source. The source content is trimmed, and previous frames are turned into holds
- `animation_offset_end` · int — Animation end offset (trim end)
- `content_trim_end` · int — Number of frames to ignore from the end of the underlying source. The source content is trimmed, and future frames are turned into holds
- `colorspace_settings` · pointer · -> ColorManagedInputColorspaceSettings · read-only — Input color space settings
- `fps` · float · read-only — Frames per second

## MulticamStrip

Sequence strip to perform multicam editing

- `input_count` · int · read-only
- `multicam_source` · int
- `animation_offset_start` · int — Animation start offset (trim start)
- `content_trim_start` · int — Number of frames to ignore from the start of the underlying source. The source content is trimmed, and previous frames are turned into holds
- `animation_offset_end` · int — Animation end offset (trim end)
- `content_trim_end` · int — Number of frames to ignore from the end of the underlying source. The source content is trimmed, and future frames are turned into holds

## MultiplyStrip

Multiply Strip

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `input_2` · pointer · -> Strip — Second input for the effect strip

## NlaStrip

A container referencing an existing Action

- `name` · string
- `type` · enum · {CLIP, TRANSITION, META, SOUND} · read-only — Type of NLA Strip
- `extrapolation` · enum · {NOTHING, HOLD, HOLD_FORWARD} — Action to take for gaps past the strip extents
- `blend_type` · enum · {REPLACE, COMBINE, ADD, SUBTRACT, MULTIPLY} — Method used for combining strip's result with accumulated result
- `frame_start` · float
- `frame_end` · float
- `frame_start_raw` · float — Same as frame_start, except that any value can be set, including ones that create an invalid state
- `frame_end_raw` · float — Same as frame_end, except that any value can be set, including ones that create an invalid state
- `frame_start_ui` · float — Start frame of the NLA strip. Note: changing this value also updates the value of the strip's end frame. If only the start frame should be changed, see the "frame_start" property instead.
- `frame_end_ui` · float — End frame of the NLA strip. Note: changing this value also updates the value of the strip's repeats or its action's end frame. If only the end frame should be changed, see the "frame_end" property instead.
- `blend_in` · float — Number of frames at start of strip to fade in influence
- `blend_out` · float
- `use_auto_blend` · boolean — Number of frames for Blending In/Out is automatically determined from overlapping strips
- `action` · pointer · -> Action — Action referenced by this strip
- `action_slot_handle` · int — A number that identifies which sub-set of the Action is considered to be for this NLA strip
- `last_slot_identifier` · string — The identifier of the most recently assigned action slot. The slot identifies which sub-set of the Action is considered to be for this strip, and its identifier is used to find the right slot when assigning an Action.
- `action_slot` · pointer · -> ActionSlot — The slot identifies which sub-set of the Action is considered to be for this strip, and its name is used to find the right slot when assigning another Action
- `action_suitable_slots` · collection · read-only — The list of action slots suitable for this NLA strip
- `action_frame_start` · float — First frame from action to use
- `action_frame_end` · float — Last frame from action to use
- `repeat` · float — Number of times to repeat the action range
- `scale` · float — Scaling factor for action
- `fcurves` · collection · [] of NlaStripFCurves · read-only — F-Curves for controlling the strip's influence and timing
- `modifiers` · collection · read-only — Modifiers affecting all the F-Curves in the referenced Action
- `strips` · collection · read-only — NLA Strips that this strip acts as a container for (if it is of type Meta)
- `influence` · float — Amount the strip contributes to the current result
- `strip_time` · float — Frame of referenced Action to evaluate
- `use_animated_influence` · boolean — Influence setting is controlled by an F-Curve rather than automatically determined
- `use_animated_time` · boolean — Strip time is controlled by an F-Curve rather than automatically determined
- `use_animated_time_cyclic` · boolean — Cycle the animated time within the action start and end
- `active` · boolean · read-only — NLA Strip is active
- `select` · boolean — NLA Strip is selected
- `mute` · boolean — Disable NLA Strip evaluation
- `use_reverse` · boolean — NLA Strip is played back in reverse order (only when timing is automatically determined)
- `use_sync_length` · boolean — Update range of frames referenced from action after tweaking strip and its keyframes

## SceneStrip

Sequence strip using the rendered image of a scene

- `scene` · pointer · -> Scene — Scene that this strip uses
- `scene_camera` · pointer · -> Object — Override the scene's active camera
- `view_layer` · pointer · -> ViewLayer — View Layer of the scene to render (uses the default if unset)
- `scene_input` · enum · {CAMERA, SEQUENCER} — Input type to use for the Scene strip
- `use_annotations` · boolean — Show Annotations in OpenGL previews
- `retiming_keys` · collection · [] of RetimingKeys · read-only
- `volume` · float — Playback volume of the sound
- `use_deinterlace` · boolean — Remove fields from video movies
- `alpha_mode` · enum · {STRAIGHT, PREMUL} — Representation of alpha information in the RGBA pixels
- `use_flip_x` · boolean — Flip on the X axis
- `use_flip_y` · boolean — Flip on the Y axis
- `use_float` · boolean — Convert input to float data
- `use_reverse_frames` · boolean — Reverse frame order
- `color_multiply` · float
- `multiply_alpha` · boolean — Multiply alpha along with color channels
- `color_saturation` · float — Adjust the intensity of the input's color
- `strobe` · float — Only display every nth frame
- `transform` · pointer · -> StripTransform · read-only
- `crop` · pointer · -> StripCrop · read-only
- `use_proxy` · boolean — Use a preview proxy for this strip
- `proxy` · pointer · -> StripProxy · read-only
- `animation_offset_start` · int — Animation start offset (trim start)
- `content_trim_start` · int — Number of frames to ignore from the start of the underlying source. The source content is trimmed, and previous frames are turned into holds
- `animation_offset_end` · int — Animation end offset (trim end)
- `content_trim_end` · int — Number of frames to ignore from the end of the underlying source. The source content is trimmed, and future frames are turned into holds
- `fps` · float · read-only — Frames per second

## SoundStrip

Sequence strip defining a sound to be played over a period of time

- `sound` · pointer · -> Sound — Sound data-block used by this strip
- `volume` · float — Playback volume of the sound
- `pan` · float — Playback panning of the sound (only for Mono sources)
- `sound_offset` · float — Subframe offset of the sound source start expressed in seconds
- `show_waveform` · boolean — Display the audio waveform inside the strip
- `pitch_correction` · boolean — Maintain the original pitch of the audio when changing playback speed
- `retiming_keys` · collection · [] of RetimingKeys · read-only
- `animation_offset_start` · int — Animation start offset (trim start)
- `content_trim_start` · int — Number of frames to ignore from the start of the underlying source. The source content is trimmed, and previous frames are turned into holds
- `animation_offset_end` · int — Animation end offset (trim end)
- `content_trim_end` · int — Number of frames to ignore from the end of the underlying source. The source content is trimmed, and future frames are turned into holds

## SpeedControlStrip

Sequence strip to control the speed of other strips

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `speed_control` · enum · {STRETCH, MULTIPLY, FRAME_NUMBER, LENGTH} — Speed control method
- `speed_factor` · float — Multiply the current speed of the strip with this number or remap current frame to this frame
- `speed_frame_number` · float — Frame number of input strip
- `speed_length` · float — Percentage of input strip length
- `use_frame_interpolate` · boolean — Do crossfade blending between current and next frame

## Strip

A single container for content in the Video Sequence Editor

- `name` · string
- `type` · enum · {IMAGE, META, SCENE, MOVIE, MOVIECLIP, MASK, SOUND, CROSS, ADD, SUBTRACT, ALPHA_OVER, ALPHA_UNDER, GAMMA_CROSS, COMPOSITOR, MULTIPLY, WIPE, GLOW, COLOR, SPEED, MULTICAM, ADJUSTMENT, GAUSSIAN_BLUR, TEXT, COLORMIX} · read-only
- `select` · boolean — Whether the strip is selected
- `select_left_handle` · boolean — Whether the left handle is selected
- `select_right_handle` · boolean — Whether the right handle is selected
- `mute` · boolean — Disable strip so that it does not contribute any output
- `lock` · boolean — Lock strip so that it cannot be transformed
- `frame_final_duration` · int — The length of the contents of this strip after the handles are applied
- `duration` · int — Length of the strip in frames from left handle to right handle
- `frame_duration` · int · read-only — The length of the contents of this strip before the handles are applied
- `content_duration` · int · read-only — Length of the underlying strip source in frames, excluding handles
- `frame_start` · float — X position where the strip begins
- `content_start` · float — Timeline frame where underlying strip source begins
- `content_end` · int · read-only — Timeline frame where underlying strip source ends
- `frame_final_start` · int — Start frame displayed in the sequence editor after offsets are applied, setting this is equivalent to moving the handle, not the actual start frame
- `left_handle` · int — Timeline frame of the left handle and the start frame of the strip
- `frame_final_end` · int — End frame displayed in the sequence editor after offsets are applied
- `right_handle` · int — Timeline frame of the right handle, which is the first frame where the strip no longer contributes to the output
- `frame_offset_start` · float — Offset from the start of the strip in frames
- `left_handle_offset` · float — Rightward frame offset of the left handle from the start of the strip content
- `frame_offset_end` · float — Offset from the end of the strip in frames
- `right_handle_offset` · float — Leftward frame offset of the right handle from the end of the strip content
- `channel` · int — Vertical position of the strip
- `blend_type` · enum · {REPLACE, CROSS, DARKEN, MULTIPLY, BURN, LINEAR_BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, HARD_LIGHT, VIVID_LIGHT, LINEAR_LIGHT, PIN_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, HUE, SATURATION, COLOR, VALUE, ALPHA_OVER, ALPHA_UNDER, GAMMA_CROSS} — Method for controlling how the strip combines with other strips
- `blend_alpha` · float — Percentage of how much the strip's colors affect other strips
- `effect_fader` · float — Custom fade value
- `use_default_fade` · boolean — Fade effect using the built-in default (usually makes the transition as long as the effect strip)
- `color_tag` · enum · {NONE, COLOR_01, COLOR_02, COLOR_03, COLOR_04, COLOR_05, COLOR_06, COLOR_07, COLOR_08, COLOR_09} — Color tag for a strip
- `modifiers` · collection · [] of StripModifiers · read-only — Modifiers affecting this strip
- `show_retiming_keys` · boolean — Show retiming keys, so they can be moved
- `connections` · collection · read-only — Other strips currently connected to this strip

## StripColorBalanceData

Color balance parameters for a sequence strip and its modifiers

- `correction_method` · enum · {LIFT_GAMMA_GAIN, OFFSET_POWER_SLOPE}
- `lift` · float · [3] — Color balance lift (shadows)
- `gamma` · float · [3] — Color balance gamma (midtones)
- `gain` · float · [3] — Color balance gain (highlights)
- `slope` · float · [3] — Correction for highlights
- `offset` · float · [3] — Correction for entire tonal range
- `power` · float · [3] — Correction for midtones
- `invert_lift` · boolean — Invert the lift color
- `invert_gamma` · boolean — Invert the gamma color
- `invert_gain` · boolean — Invert the gain color
- `invert_slope` · boolean — Invert the slope color
- `invert_offset` · boolean — Invert the offset color
- `invert_power` · boolean — Invert the power color

## StripCrop

Cropping parameters for a sequence strip

- `max_y` · int — Number of pixels to crop from the top
- `min_y` · int — Number of pixels to crop from the bottom
- `min_x` · int — Number of pixels to crop from the left side
- `max_x` · int — Number of pixels to crop from the right side

## StripElement

Sequence strip data for a single frame

- `filename` · string — Name of the source file
- `orig_width` · int · read-only — Original image width
- `orig_height` · int · read-only — Original image height
- `orig_fps` · float · read-only — Original frames per second

## StripProxy

Proxy parameters for a sequence strip

- `directory` · string — Location to store the proxy files
- `filepath` · string — Location of custom proxy file
- `use_overwrite` · boolean — Overwrite existing proxy files when building
- `build_25` · boolean — Build 25% proxy resolution
- `build_50` · boolean — Build 50% proxy resolution
- `build_75` · boolean — Build 75% proxy resolution
- `build_100` · boolean — Build 100% proxy resolution
- `quality` · int — Quality of proxies to build
- `use_proxy_custom_directory` · boolean — Use a custom directory to store data
- `use_proxy_custom_file` · boolean — Use a custom file to read proxy data from

## StripTransform

Transform parameters for a sequence strip

- `scale_x` · float — Scale along X axis
- `scale_y` · float — Scale along Y axis
- `offset_x` · float — Move along X axis
- `offset_y` · float — Move along Y axis
- `rotation` · float — Rotate around image center
- `origin` · float · [2] — Origin of image for transformation
- `filter` · enum · {AUTO, NEAREST, BILINEAR, CUBIC_MITCHELL, CUBIC_BSPLINE, BOX} — Type of filter to use for image transformation

## SubtractStrip

Subtract Strip

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `input_2` · pointer · -> Strip — Second input for the effect strip

## TextStrip

Sequence strip creating text

- `input_count` · int · read-only
- `font` · pointer · -> VectorFont — Font of the text. Falls back to the UI font by default.
- `font_size` · float — Size of the text
- `space_line` · float — Distance between lines of text in proportion to text size
- `abs_space_line` · float — Distance between lines of text in pixels
- `use_absolute_line_spacing` · boolean — Define spacing using pixel values instead of relative scaling based on font size
- `color` · float · [4] — Text color
- `shadow_color` · float · [4]
- `shadow_angle` · float
- `shadow_offset` · float
- `shadow_blur` · float
- `outline_color` · float · [4]
- `outline_width` · float
- `box_color` · float · [4]
- `location` · float · [2] — Location of the text
- `wrap_width` · float — Word wrap width as factor, zero disables
- `box_margin` · float — Box margin as factor of image width
- `box_roundness` · float — Box corner radius as a factor of box height
- `alignment_x` · enum · {LEFT, CENTER, RIGHT} — Horizontal text alignment
- `anchor_x` · enum · {LEFT, CENTER, RIGHT} — Horizontal position of the text box relative to Location
- `anchor_y` · enum · {TOP, CENTER, BOTTOM} — Vertical position of the text box relative to Location
- `text` · string — Text that will be displayed
- `use_shadow` · boolean — Display shadow behind text
- `use_outline` · boolean — Display outline around text
- `use_box` · boolean — Display colored box behind text
- `use_bold` · boolean — Display text as bold
- `use_italic` · boolean — Display text as italic
- `textbox_state` · pointer · -> TextboxState · read-only — Textbox state in the UI

## ThemeStripColor

Theme settings for strip colors

- `color` · float · [3] — Strip Color

## WipeStrip

Sequence strip creating a wipe transition

- `input_count` · int · read-only
- `input_1` · pointer · -> Strip — First input for the effect strip
- `input_2` · pointer · -> Strip — Second input for the effect strip
- `blur_width` · float — Width of the blur for the transition, in percentage relative to the image size
- `angle` · float — Angle of the transition
- `direction` · enum · {OUT, IN} — Whether to fade in or out
- `transition_type` · enum · {SINGLE, DOUBLE, IRIS, CLOCK}
