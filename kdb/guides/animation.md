---
area: guide
title: Animation — keyframes, F-curves, NLA, drivers
version: 5.2
---

# Animation

Everything that makes a value change over time: keyframes on object properties,
the F-curves they live on, procedural F-curve modifiers, motion paths, actions and
NLA strips, and drivers. Rigs, bones and constraints are a different area — see
`kdb_guide {"area": "rigging"}`.

## The tools

| Tool | What it is for |
|---|---|
| `anim_set_keyframe` | Insert a keyframe; single or batch. The only tool that writes a value *and* keys it |
| `anim_delete_keyframe` | Remove the key at one frame. Takes `property`, not `data_path` |
| `anim_get_keyframes` | Every key on an object, grouped by data path, with values and interpolation |
| `anim_get_data` | Cheaper overview: action name plus one row per F-curve (`keyframe_count`, `range`) |
| `anim_clear` | `animation_data_clear()` — drops action, drivers and NLA in one go |
| `anim_set_frame` | Move the playhead (`scene.frame_set`) |
| `scene_set` | Frame range and fps. The only tool that writes `frame_start`/`frame_end`/`fps` |
| `scene_get_info` | Read the range/fps back |
| `anim_add_fcurve_noise` | Procedural wiggle. Creates the F-curve if it does not exist yet |
| `anim_add_fcurve_cycles` | Loop a curve. Requires an existing curve |
| `anim_add_fcurve_envelope` | Amplitude envelope. Requires an existing curve |
| `anim_show_motion_path` | Compute a motion path for an object or one pose bone |
| `anim_update_motion_paths` | Recompute existing paths |
| `anim_clear_motion_paths` | Remove them |
| `nla_create_action` | New action, optionally assigned to an object |
| `nla_push` | Move an action into an NLA strip on a track |
| `nla_list_tracks` | Tracks, strips, timings, blend, influence, plus the active action |
| `nla_set_strip_props` | Strip timing, `blend_type`, `extrapolation`, `influence`, `mute` |
| `nla_delete_strip` | Remove one strip |
| `drv_add` | Driver on a property, component-wise, SCRIPTED by default |
| `drv_create` | Driver on **one** component, transform-variable oriented |
| `drv_copy` | Duplicate an existing driver onto another property |
| `drv_remove` | Remove a driver |
| `prop_set` / `prop_list` / `prop_remove` | Custom properties — what a rig control actually is |

## How to do the usual things

### Key a property

`anim_set_keyframe` resolves the path as `data_path` if given, else `property`,
else `location`. `frame` defaults to the current frame. `value`, when given, is
written onto the object *before* the key is inserted, so the key captures what you
asked for:

- `location` / `rotation_euler` / `scale` with no `index` — `value` must be a
  3-element array; with `index` it sets that one component.
- `modifiers["Name"].prop` and `active_material.prop` are understood.
- Any dot-free attribute on the object is written directly.

Anything else keys the value already there and reports `value_applied: false` —
that is the honest answer, not an error. Batch mode takes `keyframes`:

```json
{"keyframes": [
  {"name": "Cube", "frame": 1,  "property": "location", "value": [0, 0, 0]},
  {"name": "Cube", "frame": 24, "property": "location", "value": [4, 0, 2]},
  {"name": "Cube", "frame": 24, "data_path": "scale", "index": 2, "value": 1.5}
]}
```

The result is `{"set": N, "results": [...]}`, and a per-entry failure appears as
`{"name": ..., "error": ...}` inside `results` rather than failing the call. A
single-entry call instead returns the bare result and raises on failure.

### Read animation back

`anim_get_data` first — one row per curve, with `data_path`, `array_index`,
`keyframe_count` and `range`. `anim_get_keyframes` when you need actual key values;
it returns `{data_path: [{frame, value, interpolation, index}, ...]}`, so a keyed
`location` yields three entries per frame, one per axis. Neither tool fails on an
un-animated object; both return an empty result.

### Where the curves actually live (Blender 5)

`Action.fcurves` was **removed in Blender 5.0**. An action is layered and slotted:

```
action.layers[i].strips[j].channelbag(slot).fcurves
```

`action.is_action_layered` is always `True` on 5.2, `action.slots` holds one slot
per animated ID, and a strip has no channelbag for a slot it does not animate.
Every tool here walks that chain; that is why `anim_get_data` works on 5.2 while a
4.x snippet doing `action.fcurves` raises `AttributeError`. Drivers are the one
exception: they live on `object.animation_data.drivers`, a separate F-curve
collection that 5.0 kept.

### Frame and timing

- `anim_set_frame` calls `scene.frame_set(n)` — it moves the playhead *and*
  re-evaluates the dependency graph in the same call. Use it before reading
  evaluated state, screenshotting or baking.
- `scene_set` assigns `frame_current` plainly, and is the only way to write
  `frame_start`, `frame_end`, `fps` and `fps_base`. It refuses `frame_end <
  frame_start` instead of silently clamping, and orders the two assignments so
  Blender's own auto-clamp never fires.

Actual fps is `fps / fps_base` (`30 / 1.001` for 29.97). Factory default on 5.2 is
`24 / 1.0`. Read it back with `scene_get_info`.

### Procedural F-curve modifiers

`anim_add_fcurve_noise` will **create** the curve if it does not exist, seeding one
keyframe at frame 1 so the curve is not empty. `anim_add_fcurve_cycles` and
`anim_add_fcurve_envelope` will not — they fail with `F-Curve not found:
<path>[<index>]`, so key the property with `anim_set_keyframe` first.

Order matters. Blender only honours a Cycles modifier when it is **first** in the
stack. On 5.2 adding one after a Noise modifier still returns a modifier, but its
`is_valid` is `False` and it does nothing — verified live on 5.2.2. Add Cycles
first, then Noise, then Envelope.

`mode_before` / `mode_after` take `NONE`, `REPEAT`, `REPEAT_OFFSET`, `MIRROR`
(default `REPEAT`). Blender 5.0/5.1 refused a *second* Cycles modifier on one curve
— the data API declined and returned `None`, which the tool had to surface as an
error. 5.2 accepts it and the curve carries two.

### Motion paths

`anim_show_motion_path` takes `name`, optional `bone` (the object must be an
armature — the bone branch switches to Pose mode and back), and
`frame_start`/`frame_end`/`frame_step`, defaulting to the scene range. The range is
written onto `animation_visualization.motion_path` and the operator is called with
`display_type=RANGE, range=MANUAL`: `paths_calculate` has no start/end arguments on
Blender 5. Blender then reports `motion_path.frame_end` as one past the requested
end frame; that is its own bookkeeping. `anim_update_motion_paths` recomputes,
`anim_clear_motion_paths` removes (single `name` or a `names` batch).

### Actions and NLA

1. `nla_create_action {"name": "Walk", "object": "Rig"}` — creates and assigns.
2. Key on it with `anim_set_keyframe`.
3. `nla_push {"object": "Rig", "track_name": "Track1", "start_frame": 1}` — pushes
   the active action (or a named `action`) into a strip. A `track_name` that does
   not exist is created; omitting it creates an unnamed track.
4. `nla_list_tracks` to learn the strip name — `nla_set_strip_props` and
   `nla_delete_strip` address a strip by `object` + `track` + `strip`, all three
   required.

`blend_type`: `REPLACE`, `COMBINE`, `ADD`, `SUBTRACT`, `MULTIPLY`.
`extrapolation`: `NOTHING`, `HOLD`, `HOLD_FORWARD`. Both checked against 5.2.2 RNA.

### Drivers: `drv_add` vs `drv_create`

They are not aliases; pick deliberately.

`drv_add` addresses the object as `name`, the property as `data_path` + `index`,
and `index` defaults to **-1 = every component**: on `location` it configures all
three F-curves identically. `driver_type` defaults to `SCRIPTED` (`AVERAGE`, `SUM`,
`MIN`, `MAX` are the others), and its variables default to `SINGLE_PROP` driven by
a `data_path` — property-to-property driving.

```json
{"name": "Cube", "data_path": "location", "index": 2,
 "driver_type": "SCRIPTED", "expression": "ctrl * 2",
 "variables": [{"name": "ctrl", "type": "SINGLE_PROP",
                "target": "Rig", "data_path": "pose.bones[\"Ctrl\"][\"speed\"]"}]}
```

`drv_create` addresses the object as `target`, the property as `target_prop`, and
one component as `prop_index`. Its variables default to `TRANSFORMS`
(`transform_type` default `LOC_X`, `space` default `WORLD_SPACE`, taken from
`source` or the driven object itself) — transform-driven rigging. An empty or
absent `expression` leaves the driver as `AVERAGE` rather than `SCRIPTED`. **Always
pass `prop_index` for an array property**: at the default `-1` Blender hands back a
list of F-curves and this tool expects a single one.

`drv_copy {"source", "source_prop", "target", "target_prop"}` finds the driver by
data path on `source.animation_data.drivers` and rebuilds it — type, expression,
every variable and target — on the same array index of the target property.
`drv_remove {"name", "data_path", "index"}` removes; `index` defaults to -1 (all).

### Custom properties as rig controls

A rig control is a custom property plus a driver that reads it. `prop_set`
addresses a datablock as `<collection>:<name>` (`objects:Cube`, `materials:Rust`,
`scenes:Scene`) or a pose bone as `bone:<armature>/<bone>`:

```json
{"target": "bone:Rig/Ctrl", "props": {"speed": 1.5},
 "min": 0.0, "max": 10.0, "description": "walk speed"}
```

`min`/`max`/`description` write the ID-property UI metadata — that is what turns a
raw number field into a slider. The range must match the stored type: store `100`
and the range is written as ints, store `1.5` and it is floats. A string or list
property has no range, and the call says so instead of leaking a `TypeError`.
`prop_list` reads them back (Blender's own `_`-prefixed and `cycles` keys are
filtered out); `prop_remove` takes a `names` array and errors on a key that is not
there. Drive them with `drv_add` using a `SINGLE_PROP` variable.

## Traps

- `anim_delete_keyframe` has **no** `data_path` parameter — only `property`, and it
  errors when there is no key at that frame/path.
- `anim_clear` removes drivers and NLA too, not only the action.
- A batch `anim_set_keyframe` still reports success when entries failed; compare
  `set` against the number of entries you sent.
- `value` on a path the writer does not understand is silently not applied —
  `value_applied: false` is the signal.
- Cycles must be the first F-curve modifier. On 5.2 a misplaced one is created but
  `is_valid == False`, and nothing raises.
- `anim_add_fcurve_cycles` / `_envelope` need an existing curve; `_noise` makes one.
- `nla_push` clears the object's active action afterwards, like the NLA push-down
  button. Create or assign a new action before keying again.
- Never write `action.fcurves` in a script handed to Blender 5.2 — it does not
  exist. Go through `layers[].strips[].channelbag(slot).fcurves`.
- `drv_create` with `prop_index` omitted on `location`/`scale`/`rotation_euler`
  fails; `drv_add` handles that shape and drives every component.

## See also

`kdb_guide {"area": "rigging"}` — armatures, pose bones, IK, constraints and shape
keys; `con_add` with `bone` is how an animated rig is actually built.
`kdb_search {"query": "NlaStrip blend_type", "area": "api-types"}`,
`kdb_search {"query": "FModifierCycles mode_before"}`,
`kdb_search {"query": "action slot channelbag", "area": "release-notes"}`.
