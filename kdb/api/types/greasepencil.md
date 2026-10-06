---
area: api-types
group: greasepencil
version: 5.2.2
---

# Blender 5.2.2 RNA types: greasepencil

## AttributeGroupGreasePencil

Group of geometry attributes

- `active` · pointer · -> Attribute — Active attribute
- `active_index` · int — Active attribute index or -1 when none are active

## AttributeGroupGreasePencilDrawing

Group of geometry attributes

- `active` · pointer · -> Attribute — Active attribute
- `active_index` · int — Active attribute index or -1 when none are active

## GreasePencil

Grease Pencil data-block

- `attributes` · collection · [] of AttributeGroupGreasePencil · read-only — Geometry attributes
- `color_attributes` · collection · [] of AttributeGroupGreasePencil · read-only — Geometry color attributes
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `materials` · collection · [] of IDMaterials · read-only
- `layers` · collection · [] of GreasePencilv3Layers · read-only — Grease Pencil layers
- `layer_groups` · collection · [] of GreasePencilv3LayerGroup · read-only — Grease Pencil layer groups
- `root_nodes` · collection · read-only — The root nodes of the layer tree. Ordered by stack order, meaning the first node is the bottom most node in the layer tree.
- `use_autolock_layers` · boolean — Automatically lock all layers except the active one to avoid accidental changes
- `stroke_depth_order` · enum · {2D, 3D} — Defines how the strokes are ordered in 3D space (for objects not displayed 'In Front')
- `ghost_before_range` · int — Maximum number of frames to show before current frame (0 = don't show any frames before current)
- `ghost_after_range` · int — Maximum number of frames to show after current frame (0 = don't show any frames after current)
- `use_ghost_custom_colors` · boolean — Use custom colors for ghost frames
- `before_color` · float · [3] — Base color for ghosts before the active frame
- `after_color` · float · [3] — Base color for ghosts after the active frame
- `onion_mode` · enum · {ABSOLUTE, RELATIVE, SELECTED} — Mode to display frames
- `onion_keyframe_type` · enum · {ALL, KEYFRAME, BREAKDOWN, MOVING_HOLD, EXTREME, JITTER, GENERATED} — Type of keyframe (for filtering)
- `use_onion_fade` · boolean — Display onion keyframes with a fade in color transparency
- `use_onion_loop` · boolean — Display onion keyframes for looping animations
- `onion_factor` · float — Change fade opacity of displayed onion frames

## GreasePencilDrawing

A Grease Pencil drawing

- `type` · enum · {DRAWING, REFERENCE} · read-only — Drawing type
- `user_count` · int · read-only — The number of keyframes this drawing is used by
- `curve_offsets` · collection · read-only — Offset indices of the first point of each curve
- `attributes` · collection · [] of AttributeGroupGreasePencilDrawing · read-only — Geometry attributes
- `color_attributes` · collection · [] of AttributeGroupGreasePencilDrawing · read-only — Geometry color attributes

## GreasePencilFrame

A Grease Pencil keyframe

- `drawing` · pointer · -> GreasePencilDrawing — A Grease Pencil drawing
- `frame_number` · int · read-only — The frame number in the scene
- `select` · boolean — Frame Selection in the Dope Sheet
- `keyframe_type` · enum · {KEYFRAME, BREAKDOWN, MOVING_HOLD, EXTREME, JITTER, GENERATED} — Type of keyframe

## GreasePencilLayer

Collection of related drawings

- `frames` · collection · [] of GreasePencilFrames · read-only — Grease Pencil frames
- `mask_layers` · collection · [] of GreasePencilLayerMasks · read-only — List of Masking Layers
- `lock_frame` · boolean — Lock current frame displayed by layer
- `opacity` · float — Layer Opacity
- `tint_color` · float · [3] — Color for tinting stroke colors
- `tint_factor` · float — Factor of tinting color
- `radius_offset` · float — Radius change to apply to current strokes
- `use_lights` · boolean — Enable the use of lights on stroke and fill materials
- `pass_index` · int — Index number for the "Layer Index" pass
- `parent` · pointer · -> Object — Parent object
- `parent_bone` · string — Name of parent bone. Only used when the parent object is an armature.
- `translation` · float · [3] — Translation of the layer
- `rotation` · float · [3] — Euler rotation of the layer
- `scale` · float · [3] — Scale of the layer
- `viewlayer_render` · string — Only include Layer in this View Layer render output (leave blank to include always)
- `use_viewlayer_masks` · boolean — Include the mask layers when rendering the view-layer
- `blend_mode` · enum · {REGULAR, HARDLIGHT, ADD, SUBTRACT, MULTIPLY, DIVIDE} — Blend mode
- `ignore_locked_materials` · boolean — Allow editing strokes even if they use locked materials
- `matrix_local` · float · [16] · read-only — Local transformation matrix of the layer
- `matrix_parent_inverse` · float · [16] · read-only — Inverse of layer's parent transformation matrix

## GreasePencilLayerGroup

Group of Grease Pencil layers

- `is_expanded` · boolean — The layer group is expanded in the UI
- `color_tag` · enum · {NONE, COLOR1, COLOR2, COLOR3, COLOR4, COLOR5, COLOR6, COLOR7, COLOR8}
- `children` · collection · read-only — The direct children of this layer group. Ordered by stack order, meaning the first child is the bottom most child in the layer tree.

## GreasePencilLayerMask

List of Mask Layers

- `name` · string — Mask layer name
- `hide` · boolean — Set mask Visibility
- `invert` · boolean — Invert mask

## GreasePencilLayerMasks

Collection of Grease Pencil masking layers

- `active_mask_index` · int — Active index in layer mask array

## GreasePencilv3LayerGroup

Collection of Grease Pencil layers

- `active` · pointer · -> GreasePencilLayerGroup — Active Grease Pencil layer group

## GreasePencilv3Layers

Collection of Grease Pencil layers

- `active` · pointer · -> GreasePencilLayer — Active Grease Pencil layer
