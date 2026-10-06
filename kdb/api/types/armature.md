---
area: api-types
group: armature
version: 5.2.2
---

# Blender 5.2.2 RNA types: armature

## Armature

Armature data-block containing a hierarchy of bones, usually used for rigging characters

- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `bones` · collection · [] of ArmatureBones · read-only
- `edit_bones` · collection · [] of ArmatureEditBones · read-only
- `collections` · collection · [] of BoneCollections
- `collections_all` · collection · read-only — List of all bone collections of the armature
- `pose_position` · enum · {POSE, REST} — Show armature in binding pose or final posed state
- `display_type` · enum · {OCTAHEDRAL, STICK, BBONE, ENVELOPE, WIRE}
- `show_axes` · boolean — Display bone axes
- `axes_position` · float — The position for the axes on the bone. Increasing the value moves it closer to the tip; decreasing moves it closer to the root.
- `relation_line_position` · enum · {TAIL, HEAD} — The start position of the relation lines from parent to child bones
- `show_names` · boolean — Display bone names
- `use_mirror_x` · boolean — Apply changes to matching bone on opposite side of X-Axis
- `show_bone_custom_shapes` · boolean — Display bones with their custom shapes
- `show_bone_colors` · boolean — Display bone colors
- `is_editmode` · boolean · read-only — True when used in editmode
