---
area: api-ops
group: armature
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.armature

- `bpy.ops.armature.align()` — Align selected bones to the active bone (or to their parent)
- `bpy.ops.armature.assign_to_collection(collection_index: int, new_collection_name: string)` — Assign all selected bones to a collection, or unassign them, depending on whether the active bone is already assigned or not
- `bpy.ops.armature.autoside_names(type: enum{XAXIS, YAXIS, ZAXIS})` — Automatically renames the selected bones according to which side of the target axis they fall on
- `bpy.ops.armature.bone_primitive_add(name: string, space: enum{OBJECT, WORLD}, align: enum{UP, AXES, 3D_CURSOR, 3D_VIEW}, length: float, use_deform: boolean)` — Add a new bone located at the 3D cursor
- `bpy.ops.armature.calculate_roll(type: enum{POS_X, POS_Z, GLOBAL_POS_X, GLOBAL_POS_Y, GLOBAL_POS_Z, NEG_X, NEG_Z, GLOBAL_NEG_X, GLOBAL_NEG_Y, GLOBAL_NEG_Z, ACTIVE, VIEW, CURSOR}, axis_flip: boolean, axis_only: boolean)` — Automatically fix alignment of select bones' axes
- `bpy.ops.armature.click_extrude()` — Create a new bone going from the last selected joint to the mouse position
- `bpy.ops.armature.collection_add()` — Add a new bone collection
- `bpy.ops.armature.collection_assign(name: string)` — Add selected bones to the chosen bone collection
- `bpy.ops.armature.collection_create_and_assign(name: string)` — Create a new bone collection and assign all selected bones
- `bpy.ops.armature.collection_deselect()` — Deselect bones of active Bone Collection
- `bpy.ops.armature.collection_move(direction: enum{UP, DOWN})` — Change position of active Bone Collection in list of Bone collections
- `bpy.ops.armature.collection_remove()` — Remove the active bone collection
- `bpy.ops.armature.collection_remove_unused()` — Remove all bone collections that have neither bones nor children. This is done recursively, so bone collections that only have unused children are also removed
- `bpy.ops.armature.collection_select()` — Select bones in active Bone Collection
- `bpy.ops.armature.collection_show_all()` — Show all bone collections
- `bpy.ops.armature.collection_unassign(name: string)` — Remove selected bones from the active bone collection
- `bpy.ops.armature.collection_unassign_named(name: string, bone_name: string)` — Unassign the named bone from this bone collection
- `bpy.ops.armature.collection_unsolo_all()` — Clear the 'solo' setting on all bone collections
- `bpy.ops.armature.copy_bone_color_to_selected(bone_type: enum{EDIT, POSE})` — Copy the bone color of the active bone to all selected bones
- `bpy.ops.armature.delete(confirm: boolean)` — Remove selected bones from the armature
- `bpy.ops.armature.dissolve()` — Dissolve selected bones from the armature
- `bpy.ops.armature.duplicate(do_flip_names: boolean)` — Make copies of the selected bones within the same armature
- `bpy.ops.armature.duplicate_move(ARMATURE_OT_duplicate: pointer, TRANSFORM_OT_translate: pointer)` — Make copies of the selected bones within the same armature and move them
- `bpy.ops.armature.duplicate_rename(do_flip_names: boolean, search: string, replace: string)` — Make copies of the selected bones within the same armature and replace a part of their name
- `bpy.ops.armature.extrude(forked: boolean)` — Create new bones from the selected joints
- `bpy.ops.armature.extrude_forked(ARMATURE_OT_extrude: pointer, TRANSFORM_OT_translate: pointer)` — Create new bones from the selected joints and move them
- `bpy.ops.armature.extrude_move(ARMATURE_OT_extrude: pointer, TRANSFORM_OT_translate: pointer)` — Create new bones from the selected joints and move them
- `bpy.ops.armature.fill()` — Add bone between selected joint(s) and/or 3D cursor
- `bpy.ops.armature.flip_names(do_strip_numbers: boolean)` — Flips (and corrects) the axis suffixes of the names of selected bones
- `bpy.ops.armature.hide(unselected: boolean)` — Tag selected bones to not be visible in Edit Mode
- `bpy.ops.armature.move_to_collection(collection_index: int, new_collection_name: string)` — Move bones to a collection
- `bpy.ops.armature.parent_clear(type: enum{CLEAR, DISCONNECT})` — Remove the parent-child relationship between selected bones and their parents
- `bpy.ops.armature.parent_set(type: enum{CONNECTED, OFFSET})` — Set the active bone as the parent of the selected bones
- `bpy.ops.armature.reveal(select: boolean)` — Reveal all bones hidden in Edit Mode
- `bpy.ops.armature.roll_clear(roll: float)` — Clear roll for selected bones
- `bpy.ops.armature.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Toggle selection status of all bones
- `bpy.ops.armature.select_hierarchy(direction: enum{PARENT, CHILD}, extend: boolean)` — Select immediate parent/children of selected bones
- `bpy.ops.armature.select_less()` — Deselect those bones at the boundary of each selection region
- `bpy.ops.armature.select_linked(all_forks: boolean)` — Select all bones linked by parent/child connections to the current selection
- `bpy.ops.armature.select_linked_pick(deselect: boolean, all_forks: boolean)` — (De)select bones linked by parent/child connections under the mouse cursor
- `bpy.ops.armature.select_mirror(only_active: boolean, extend: boolean)` — Mirror the bone selection
- `bpy.ops.armature.select_more()` — Select those bones connected to the initial selection
- `bpy.ops.armature.select_similar(type: enum{CHILDREN, CHILDREN_IMMEDIATE, SIBLINGS, LENGTH, DIRECTION, PREFIX, SUFFIX, BONE_COLLECTION, COLOR, SHAPE}, threshold: float)` — Select similar bones by property types
- `bpy.ops.armature.separate()` — Isolate selected bones into a separate armature
- `bpy.ops.armature.shortest_path_pick()` — Select shortest path between two bones
- `bpy.ops.armature.split()` — Split off selected bones from connected unselected bones
- `bpy.ops.armature.subdivide(number_cuts: int)` — Break selected bones into chains of smaller bones
- `bpy.ops.armature.switch_direction()` — Change the direction that a chain of bones points in (head and tail swap)
- `bpy.ops.armature.symmetrize(direction: enum{NEGATIVE_X, POSITIVE_X}, copy_bone_colors: boolean)` — Enforce symmetry, make copies of the selection or use existing