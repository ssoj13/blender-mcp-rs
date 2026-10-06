---
area: guide
title: Rigging — armatures, constraints, skinning, shape keys
version: 5.2
---

# Rigging

Armatures and their bones, posing, IK, constraints on objects and pose bones,
vertex-group skinning, and shape keys. Keying the result and driving it belongs to
the animation area — see `kdb_guide {"area": "animation"}`.

## The tools

| Tool | What it is for |
|---|---|
| `rig_create_armature` | New armature object, optionally with a `Root` bone |
| `rig_create_biped` | A 19-bone humanoid skeleton derived from `height` |
| `rig_add_bone` | Add an edit bone: head/tail/parent/connected/roll |
| `rig_update_bone` | Move head/tail or change roll on an existing bone |
| `rig_remove_bone` | Delete a bone |
| `rig_pose_bone` | Pose one bone: location, rotation (euler or quaternion), scale |
| `rig_get_pose` | Read every pose bone's transform back |
| `rig_add_ik` | IK constraint on a pose bone, with chain length, target and pole |
| `con_add` | Any of the 29 constraint types, on an object **or** a pose bone |
| `con_update` | Change a constraint's target/subtarget/settings |
| `con_list` | What constraints an object or bone carries |
| `con_remove` | Remove constraints by name |
| `rig_auto_weights` | Parent a mesh to an armature with automatic weights |
| `rig_mirror_weights` | Mirror `.L`/`.R` vertex-group pairs |
| `vgroup_add` / `vgroup_list` / `vgroup_rename` / `vgroup_remove` | Vertex groups |
| `vgroup_set` / `vgroup_get` | Write and read weights |
| `shapekey_add` / `shapekey_update` / `shapekey_remove` / `shapekey_list` | Shape keys |
| `prop_set` / `prop_list` / `prop_remove` | Custom properties on a bone — rig controls |

## The usual pipeline

Armature → bones → skinning → constraints → pose → keyframes. Do it in that order:
weights are painted against a rest pose, and constraints evaluate on top of a pose
that already exists.

## How to do the usual things

### Build an armature

`rig_create_armature {"name": "Rig", "location": [0,0,0], "add_root_bone": true}`
— `add_root_bone` defaults to `true` and places a bone named `Root` from
`root_head` (default `[0,0,0]`) to `root_tail` (default `[0,0,1]`).

Then `rig_add_bone`, one bone per call:

```json
{"name": "Rig", "bone_name": "Spine", "parent": "Root",
 "head": [0, 0, 1], "tail": [0, 0, 2], "connected": true, "roll": 0.0}
```

- `name` is the **armature object**, `bone_name` the new bone. Defaults are head
  `[0,0,0]`, tail `[0,0,1]`.
- `connected: true` only has an effect when `parent` is given — it sets
  `use_connect`, which snaps this bone's head to the parent's tail and keeps it
  there.
- `roll` is radians around the bone axis and decides where the bone's local X/Z
  point. It matters the moment a `COPY_ROTATION` or `LIMIT_ROTATION` constraint is
  attached, so set it while building rather than later.
- A `parent` name that does not exist is simply not applied and the bone is created
  unparented, with no warning. No tool reports bone hierarchy, so spell parents
  correctly; `rig_get_pose` at least lists every bone name that exists.

`rig_update_bone` takes the same `head`/`tail`/`roll` and leaves out what you omit.
`rig_remove_bone` deletes and errors on an unknown bone. All three enter Edit mode
and return to Object mode inside the call — the armature is left in Object mode.

`rig_create_biped {"name": "Biped", "height": 2.0}` builds, in one call, a 19-bone
skeleton: `root`, `spine`, `chest`, `neck`, `head`, then per side `shoulder.L/R`,
`upper_arm.L/R`, `forearm.L/R`, `hand.L/R`, `thigh.L/R`, `shin.L/R`, `foot.L/R`.
Proportions are derived from `height` (default 2.0). Limb chains are connected
(`upper_arm` → `forearm` → `hand`, `thigh` → `shin` → `foot`); shoulders and thighs
are parented but not connected. The `.L`/`.R` suffixes are exactly what
`rig_mirror_weights` and Blender's own symmetry tools look for.

### Pose

`rig_pose_bone {"name": "Rig", "bone_name": "Spine", "location": [...],
"rotation": [...], "scale": [...]}`.

A pose bone's `rotation_mode` is `QUATERNION` by default on 5.2, and Blender only
reads the field that matches the mode. So:

- `rotation` (3 euler radians) sets `rotation_mode` to `XYZ` first, then writes
  `rotation_euler`.
- `rotation_quaternion` (4 floats, w x y z) sets the mode back to `QUATERNION`.

Do not send both in one call — the last one wins and silently changes the mode.
Pick one convention per rig; quaternions for anything that will be interpolated
through large rotations, euler for FK controls a human will scrub.

`rig_get_pose {"name": "Rig"}` returns every bone with `location`,
`rotation_euler`, `rotation_quaternion` and `scale`. It reports both rotation
fields regardless of mode, so read `rotation_mode`'s effect from the pose, not from
which field is non-zero.

### IK

`rig_add_ik` puts the constraint on the pose bone named by `bone_name` — IK is only
valid as a bone constraint, never on an object.

```json
{"name": "Rig", "bone_name": "shin.L", "chain_length": 2,
 "target": "Rig", "subtarget": "foot_ik.L",
 "pole_target": "Rig", "pole_subtarget": "knee_pole.L", "pole_angle": -1.5708}
```

- `chain_length` maps to Blender's `chain_count` (default 2) — how many bones up
  from this one the solver owns. `0` means the whole chain to the root.
- Put IK on the **last bone of the chain** (`shin`, `forearm`), not on the tip.
- `subtarget` is a bone on `target`; `target` may be the same armature.
- `pole_angle` is radians and is only applied when `pole_target` is given. It is
  almost always wrong by ±90° on the first try; adjust it rather than the pole
  bone's position.
- `con_name` names the constraint (default `IK`), which is what `con_update` and
  `con_remove` will need.

For anything other than IK use `con_add` — it is the general form.

### Constraints

`con_add` works on an object (omit `bone`) or on a pose bone (give `bone`). `type`
is case-insensitive. Type-specific settings go in `properties` and are applied
through RNA:

```json
{"object": "Rig", "bone": "chest", "type": "COPY_ROTATION",
 "con_name": "FollowHips", "target": "Rig", "subtarget": "spine",
 "properties": {"influence": 0.5, "use_x": false, "owner_space": "LOCAL",
                "target_space": "LOCAL", "mix_mode": "ADD"}}
```

Blender 5.2 has 29 constraint types: `COPY_LOCATION`, `COPY_ROTATION`,
`COPY_SCALE`, `COPY_TRANSFORMS`, `LIMIT_DISTANCE`, `LIMIT_LOCATION`,
`LIMIT_ROTATION`, `LIMIT_SCALE`, `MAINTAIN_VOLUME`, `TRANSFORM`,
`TRANSFORM_CACHE`, `CLAMP_TO`, `DAMPED_TRACK`, `IK`, `LOCKED_TRACK`, `SPLINE_IK`,
`STRETCH_TO`, `TRACK_TO`, `ACTION`, `ARMATURE`, `CHILD_OF`, `FLOOR`, `FOLLOW_PATH`,
`GEOMETRY_ATTRIBUTE`, `PIVOT`, `SHRINKWRAP`, `CAMERA_SOLVER`, `FOLLOW_TRACK`,
`OBJECT_SOLVER`. A wrong `type` is answered with that full list, and a wrong
`properties` key is answered with the settings *that* constraint really has — use
the error as the reference instead of guessing. A failed `con_add` removes the
half-made constraint, so a rig never ends up quietly misbehaving.

`subtarget` is how a prop is parented to a hand: `con_add {"object": "Sword",
"type": "CHILD_OF", "target": "Rig", "subtarget": "hand.R"}`. Constraints without a
`subtarget` (e.g. `LIMIT_ROTATION`) reject it explicitly.

`con_list {"object": "Rig", "bone": "chest"}` gives name, type, target, subtarget,
influence and mute — the names `con_update` and `con_remove` address. `con_remove`
takes a `names` array and reports per-name results; a missing name is data, not a
failure.

### Skinning

Fast path: `rig_auto_weights {"mesh": "Body", "armature": "Rig"}` — it parents with
`ARMATURE_AUTO`, which adds the Armature modifier and heat-map weights in one step.
It works headless.

Hand control: `vgroup_add {"object": "Body", "name": "hand.R"}` creates the group
(MESH and LATTICE only), then

```json
{"object": "Body", "name": "hand.R", "indices": [0, 1, 2],
 "weights": [0.1, 0.5, 1.0], "mode": "REPLACE"}
```

`vgroup_set` takes either one `weight` for every index or a `weights` array of the
same length — giving both is rejected. `mode` is `REPLACE` (default), `ADD` or
`SUBTRACT`, matching `VertexGroup.add()`. A vertex group only deforms when a bone
of the same name exists and the mesh carries the Armature modifier.

`vgroup_get` reads weights back; with `indices` omitted it returns every member
vertex, and an index that is not a member is simply absent rather than an error.
`vgroup_list`, `vgroup_rename` (Blender may append `.001`; the result reports the
real name) and `vgroup_remove` complete the set.

`rig_mirror_weights {"mesh": "Body"}` mirrors symmetric `.L`/`.R` group pairs. The
`direction` parameter is accepted and echoed for compatibility but changes nothing:
`bpy.ops.object.vertex_group_mirror` mirrors both ways in one pass. The tool
handles the mode switching itself and restores the mesh's original mode.

### Shape keys

`shapekey_add {"name": "Head", "key_name": "smile", "value": 0.0}` — on a mesh with
no shape keys it creates the `Basis` first, then the requested key, so you never
accidentally turn your target into the basis. `from_mix: true` captures the current
evaluated mix as the new key's shape, which is how a corrective shape is made after
posing.

`shapekey_update` sets `value`, `slider_min`, `slider_max`, `mute`.
`shapekey_list {"name": "Head"}` enumerates them; `shapekey_remove` takes
`key_name`. To make a shape key a rig control, drive it: put a custom property on a
bone with `prop_set {"target": "bone:Rig/face_ctrl", ...}` and point a driver at
the key's value with `drv_add` (see the animation guide).

## Traps

- Bone-edit tools take the **armature object** in `name` and the bone in
  `bone_name`; the constraint tools take `object` + `bone`. They are different
  parameter names for the same two things.
- `connected` without `parent` does nothing.
- Writing `rotation` on a pose bone whose mode is QUATERNION would be ignored by
  Blender; `rig_pose_bone` switches the mode for you, which also means it *changes*
  the bone's rotation mode. Mixing both rotation parameters across calls on one
  bone gives an unstable rig.
- `chain_length` in `rig_add_ik` is Blender's `chain_count`; there is no
  `chain_length` on the RNA.
- IK, SPLINE_IK and friends are pose-bone-only. `con_add` with `bone` on a
  non-armature object errors explicitly.
- `properties` keys are Blender's own RNA names (`use_x`, `min_x`, `head_tail`,
  `owner_space`, `track_axis`). They differ per constraint type; there is no shared
  set beyond `influence`, `mute`, `target`, `subtarget`.
- Auto weights need the mesh to be a `MESH` object and the armature a real
  `ARMATURE`; both are checked, and neither is repaired silently.
- Deleting a bone leaves any vertex group of that name behind, weighted and inert.
  Clean up with `vgroup_remove`.

## See also

`kdb_guide {"area": "animation"}` — keyframing a pose, NLA actions, drivers and
custom-property rig controls.
`kdb_search {"query": "KinematicConstraint chain_count pole_angle", "area": "api-types"}`,
`kdb_search {"query": "CopyRotationConstraint mix_mode owner_space"}`,
`kdb_search {"query": "armature edit_bones use_connect roll"}`.
