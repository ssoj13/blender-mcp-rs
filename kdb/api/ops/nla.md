---
area: api-ops
group: nla
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.nla

- `bpy.ops.nla.action_pushdown(track_index: int)` — Push action down onto the top of the NLA stack as a new strip
- `bpy.ops.nla.action_sync_length(active: boolean)` — Synchronize the length of the referenced Action with the length used in the strip
- `bpy.ops.nla.action_unlink(force_delete: boolean)` — Unlink this action from the active action slot (and/or exit Tweak Mode)
- `bpy.ops.nla.actionclip_add(action: enum{})` — Add an Action-Clip strip (i.e. an NLA Strip referencing an Action) to the active track
- `bpy.ops.nla.apply_scale()` — Apply scaling of selected strips to their referenced Actions
- `bpy.ops.nla.bake(frame_start: int, frame_end: int, step: int, only_selected: boolean, visual_keying: boolean, clear_constraints: boolean, clear_parents: boolean, use_current_action: boolean, clean_curves: boolean, bake_types: enum{POSE, OBJECT}, channel_types: enum{LOCATION, ROTATION, SCALE, BBONE, PROPS})` — Bake all selected objects location/scale/rotation animation to an action
- `bpy.ops.nla.channels_click(extend: boolean)` — Handle clicks to select NLA tracks
- `bpy.ops.nla.clear_scale()` — Reset scaling of selected strips
- `bpy.ops.nla.click_select(wait_to_deselect_others: boolean, use_select_on_click: boolean, mouse_x: int, mouse_y: int, extend: boolean, deselect_all: boolean)` — Handle clicks to select NLA Strips
- `bpy.ops.nla.delete()` — Delete selected strips
- `bpy.ops.nla.duplicate(linked: boolean)` — Duplicate selected NLA-Strips, adding the new strips to new track(s)
- `bpy.ops.nla.duplicate_linked_move(NLA_OT_duplicate: pointer, TRANSFORM_OT_translate: pointer)` — Duplicate Linked selected NLA-Strips, adding the new strips to new track(s)
- `bpy.ops.nla.duplicate_move(NLA_OT_duplicate: pointer, TRANSFORM_OT_translate: pointer)` — Duplicate selected NLA-Strips, adding the new strips to new track(s)
- `bpy.ops.nla.fmodifier_add(type: enum{NULL, GENERATOR, FNGENERATOR, ENVELOPE, CYCLES, NOISE, LIMITS, STEPPED, SMOOTH}, only_active: boolean)` — Add F-Modifier to the active/selected NLA-Strips
- `bpy.ops.nla.fmodifier_copy()` — Copy the F-Modifier(s) of the active NLA-Strip
- `bpy.ops.nla.fmodifier_paste(only_active: boolean, replace: boolean)` — Add copied F-Modifiers to the selected NLA-Strips
- `bpy.ops.nla.make_single_user(confirm: boolean)` — Make linked action local to each strip
- `bpy.ops.nla.meta_add()` — Add new meta-strips incorporating the selected strips
- `bpy.ops.nla.meta_remove()` — Separate out the strips held by the selected meta-strips
- `bpy.ops.nla.move_down()` — Move selected strips down a track if there's room
- `bpy.ops.nla.move_up()` — Move selected strips up a track if there's room
- `bpy.ops.nla.mute_toggle()` — Mute or un-mute selected strips
- `bpy.ops.nla.previewrange_set()` — Set Preview Range based on extents of selected strips
- `bpy.ops.nla.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Select or deselect all NLA-Strips
- `bpy.ops.nla.select_box(axis_range: boolean, tweak: boolean, xmin: int, xmax: int, ymin: int, ymax: int, wait_for_input: boolean, mode: enum{SET, ADD, SUB})` — Use box selection to grab NLA-Strips
- `bpy.ops.nla.select_leftright(mode: enum{CHECK, LEFT, RIGHT}, extend: boolean)` — Select strips to the left or the right of the current frame
- `bpy.ops.nla.selected_objects_add()` — Make selected objects appear in NLA Editor by adding Animation Data
- `bpy.ops.nla.snap(type: enum{CFRA, NEAREST_FRAME, NEAREST_SECOND, NEAREST_MARKER})` — Move start of strips to specified time
- `bpy.ops.nla.soundclip_add()` — Add a strip for controlling when speaker plays its sound clip
- `bpy.ops.nla.split()` — Split selected strips at their midpoints
- `bpy.ops.nla.swap()` — Swap order of selected strips within tracks
- `bpy.ops.nla.tracks_add(above_selected: boolean)` — Add NLA-Tracks above/after the selected tracks
- `bpy.ops.nla.tracks_delete()` — Delete selected NLA-Tracks and the strips they contain
- `bpy.ops.nla.transition_add()` — Add a transition strip between two adjacent selected strips
- `bpy.ops.nla.tweakmode_enter(isolate_action: boolean, use_upper_stack_evaluation: boolean)` — Enter tweaking mode for the action referenced by the active strip to edit its keyframes
- `bpy.ops.nla.tweakmode_exit(isolate_action: boolean)` — Exit tweaking mode for the action referenced by the active strip
- `bpy.ops.nla.view_all()` — Reset viewable area to show full strips range
- `bpy.ops.nla.view_frame()` — Move the view to the current frame
- `bpy.ops.nla.view_selected()` — Reset viewable area to show selected strips range