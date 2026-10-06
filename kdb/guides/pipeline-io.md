---
area: guide
title: Pipeline and I/O — how to work in it
version: 5.2
---

# Pipeline and I/O

Getting data in and out of the .blend: saving, import/export, linked libraries
and overrides, the asset browser, loose datablocks (text, image, sound, speaker,
movie clip), the video sequencer, grease pencil, and the housekeeping that keeps
a file from rotting. An agent lands here at the start of a job (bring the asset
in) and at the end of one (write something out).

## The tools

| Tool | What it is for |
|---|---|
| `file_save` | Save the .blend. `path` = save-as; omitted = save in place |
| `file_import` | Import fbx / obj / gltf / glb / usd / usda / usdc / abc. Format auto-detected from the extension |
| `file_export` | Export fbx (default) / obj / gltf / usd / abc, optionally `selected_only` |
| `library_list` | Every linked library and **every library override in the file** — check this before reload/relocate |
| `library_override_create` | Make a linked object locally editable. Overrides the whole hierarchy |
| `library_override_reset` | Revert an override's local edits to the linked values |
| `library_override_clear` | Drop one object's override. Does **not** cascade |
| `library_reload` | Re-read a linked library from disk. Refuses while any override exists |
| `library_relocate` | Repoint a library at a new .blend. Refuses while any override exists |
| `asset_mark`, `asset_unmark`, `asset_list`, `asset_import` | Asset Browser marking (OBJECT, MATERIAL, NODE_GROUP, WORLD, ACTION) and appending/linking one datablock out of another .blend |
| `text_datablock_create/list/get/set/remove` | `bpy.data.texts` buffers: driver expressions, OSL, generated scripts |
| `img_load`, `img_new`, `img_list`, `img_save` | Image datablocks — what the shader node tools refer to by name |
| `sound_load`, `sound_list`, `speaker_add`, `speaker_set`, `speaker_list` | Audio datablocks and the SPEAKER objects that play them |
| `movieclip_load`, `movieclip_set`, `movieclip_list` | Footage for tracking |
| `vse_setup`, `vse_add_strip`, `vse_add_effect`, `vse_set_strip_props`, `vse_list_strips`, `vse_delete_strip`, `vse_render` | The Video Sequence Editor |
| `gp_create`, `gp_add_layer`, `gp_add_frame`, `gp_draw_stroke`, `gp_add_modifier`, `gp_list_layers` | Grease Pencil v3 |
| `scene_list_unused`, `scene_purge_unused` | Orphan datablocks: look, then sweep |
| `preset_save`, `preset_list`, `preset_delete`, `util_undo`, `util_redo` | Named JSON blobs in the scene, and Blender's undo stack |

## How to do the usual things

### Import and export

`file_import` picks the operator from `format` or, failing that, the file
extension:

| format | operator |
|---|---|
| `fbx` | `import_scene.fbx` / `export_scene.fbx` |
| `obj` | `wm.obj_import` / `wm.obj_export` |
| `gltf`, `glb` | `import_scene.gltf` / `export_scene.gltf` (export is `gltf` only) |
| `usd`, `usda`, `usdc` | `wm.usd_import` / `wm.usd_export` (export is `usd` only) |
| `abc` | `wm.alembic_import` / `wm.alembic_export` |

All ten operators are present in 5.2. `selected_only` maps to the per-format
flag (`use_selection`, `export_selected_objects`, `selected_objects_only`,
`selected`), so the same argument works everywhere. STL and PLY have operators
in 5.2 but are **not** exposed here; an unknown format is an error, not a
fallback. `file_save` with no `path` fails on a file never saved — save before a
long bake, a big import, or any `library_*` call.

### Linked libraries and overrides

A linked datablock is read-only until overridden. The refusal rules these tools
implement are not caution, they are crash avoidance — all verified live:

1. `library_override_create` uses `id.override_hierarchy_create`: no selection
   needed, and it **overrides the whole linked hierarchy** — overriding a linked
   armature also overrides its child mesh and re-parents it. `fully_editable`
   defaults to true here (Blender's own default is off, giving a "system
   override" the resync machinery may regenerate).
2. `library_override_clear` clears **only the object named**. Clearing does not
   cascade the way creating does, so a hierarchy needs one call per member. The
   tool reports the leftovers it can see.
3. `library_reload` and `library_relocate` **refuse to run while any library
   override exists anywhere in the file** — not just on the library being
   touched. Reloading with a partially-cleared hierarchy override present is a
   native access violation that takes the whole Blender process down; both tools
   share that code path. Clear everything first and confirm `library_list`'s
   `overrides` array is empty.
4. `library_override_reset` calls `operations_update()` before `reset()` — a
   property set from a script is not tracked as an override operation until
   Blender diffs it, so a bare `reset()` would be a silent no-op.

### Assets

`asset_mark` works on OBJECT, MATERIAL, NODE_GROUP, WORLD and ACTION, writing
`description`, `author` and `tags`; `asset_list` with `type: "ALL"` (the
default) sweeps all five. `asset_import` appends one named datablock out of
another .blend (or links it with `link: true`), and refuses to import a .blend
into itself — that is a native crash, not a Python error.

```json
{"filepath": "D:/lib/props.blend", "asset_name": "Chair_A", "type": "OBJECT", "link": true}
```

A linked import is read-only; follow it with `library_override_create` to pose
or animate it.

### Loose datablocks

- `img_load` returns the datablock name the shader node tools take as
  `{"image": "<name>"}`. Set `colorspace: "Non-Color"` for roughness, normal and
  metallic maps — an sRGB curve on data is the classic subtly-wrong material.
  `pack: true` embeds the pixels. `img_list` reports `has_data`, false for the
  missing-file pink texture. `img_new` makes a bake target or paint canvas;
  `img_save` writes one back out.
- `sound_load` returns a name that `speaker_add`/`speaker_set` take as `sound`.
  `speaker_set` covers `volume`, `pitch`, `attenuation`, `distance_reference` /
  `distance_max`, the three cone properties and `muted`.
- `movieclip_load` takes a movie file or an image sequence. `movieclip_set`
  writes `frame_start`, `frame_offset` and `colorspace`; size and fps are
  read-only, defined by the footage.
- `text_datablock_set` **overwrites** the body. These are script/driver buffers,
  not 3D text objects.

### The VSE

`vse_setup` creates the sequence editor (`clear: true` removes every existing
strip). `vse_add_strip` types: IMAGE, MOVIE, SOUND (each needs `path`), COLOR
and TEXT (each needs `length`, default 50), SCENE (`scene_name`, defaults to the
current scene; `length` is accepted but never applied — a scene strip takes its
duration from the target scene).

`vse_add_effect` puts an effect above its inputs: `seq1` is required, `seq2`
only matters for the two-input types (CROSS, ADD, SUBTRACT, ALPHA_OVER,
ALPHA_UNDER, GAMMA_CROSS, MULTIPLY, WIPE, COLORMIX), which get a frame range
spanning both. Single-input effects (GLOW, GAUSSIAN_BLUR, SPEED, ADJUSTMENT,
MULTICAM) copy `seq1`'s range. The channel defaults to one above the highest
input. 5.2's full effect enum is CROSS, ADD, SUBTRACT, ALPHA_OVER, ALPHA_UNDER,
GAMMA_CROSS, MULTIPLY, WIPE, GLOW, COLOR, SPEED, MULTICAM, ADJUSTMENT,
GAUSSIAN_BLUR, TEXT, COLORMIX, COMPOSITOR — **`TRANSFORM` and `OVER_DROP` are
gone** (verified live; transform is strip properties now, set with
`vse_set_strip_props`'s `scale`/`position`/`rotation`).

`vse_render` renders the **animation** to `output` with `format` (PNG, JPEG,
FFMPEG, …), temporarily overriding filepath, format and frame range — unlike
`render_set`, nothing sticks.

```json
{"type": "TEXT", "text": "Shot 010", "channel": 3, "frame_start": 1, "length": 48}
```

### VSE strip timing on 5.2

Blender 5.1 renamed the strip timing properties. `vse_set_strip_props` still
writes the old names, and on 5.2 they still work — as **deprecated aliases that
emit a DeprecationWarning and are slated for removal in Blender 6.0**:

| old (what the tool takes) | new (5.1+) |
|---|---|
| `frame_offset_start` | `left_handle_offset` |
| `frame_offset_end` | `right_handle_offset` |

`frame_start`, `frame_final_end` and `frame_final_duration` are deprecated the
same way. Verified live: writing `left_handle_offset` and reading
`frame_offset_start` gives the same value, and both are floats, not ints —
`vse_list_strips` therefore reports `frame_start` rounded.

Two more 5.2 facts: `sequence_editor.sequences` is **gone** (only `.strips` and
`.strips_all` exist; the tools try `.strips` first and fall back), and
`speed_factor` does not exist on every strip type, so `vse_set_strip_props`
skips it rather than failing.

### Grease Pencil

`gp_create` makes a `bpy.data.grease_pencils` datablock (the legacy GPencil type
went in 4.3; the object type string is `GREASEPENCIL`) with one layer and a
frame at the current frame. Then:

- `gp_add_layer` — on **5.2 `channel_color` is read-only**, so the tool tries it
  first (keeping 5.0/5.1 behaviour) and falls back to `tint_color` +
  `tint_factor: 1.0`, reporting which one took the colour in
  `color_applied_as`.
- `gp_add_frame` — idempotent; an existing frame returns `"existed": true`.
- `gp_draw_stroke` — `points` is `[[x, y, z, pressure?], ...]`. Under v3 there
  is no `strokes.new()`, no `point.co`, no per-stroke `line_width`: strokes come
  from `drawing.add_strokes([n])` and each point carries `.position`, `.radius`
  and `.opacity`, so `line_width` is folded into the per-point radius.
- `gp_add_modifier` (noise, smooth, thickness, tint, build, array, …) and
  `gp_list_layers` round the set out.

### Housekeeping

`scene_list_unused` reports zero-user meshes, materials, images, textures,
actions, node groups, curves and armatures; `scene_purge_unused` runs a
recursive `orphans_purge`. Look before you sweep — a material with no users
right now may be one you are about to assign.

`preset_save`/`preset_list`/`preset_delete` store arbitrary JSON under
`scene["mcp_presets"][category][name]` — a scratchpad in the .blend, not
Blender's own preset system. `util_undo`/`util_redo` can walk back past tool
calls the model has forgotten about; prefer an explicit fix.

## Traps

- **`library_reload`/`library_relocate` refuse while *any* override exists**,
  including one belonging to an unrelated library. The alternative is a process
  crash, so clear them all.
- **Clearing an override hierarchy takes one call per member.** `library_list`
  shows what is left; a half-cleared hierarchy is exactly what crashes a reload.
- **`asset_import` into the same file crashes Blender natively**; the tool
  refuses. **Old VSE strip-timing names still work on 5.2 but warn and go in
  6.0** — do not build anything long-lived on `frame_offset_start`.
- **`vse_add_effect`'s description still names TRANSFORM**, which 5.2 rejects
  with `enum "TRANSFORM" not found`; so does `OVER_DROP`. Use
  `vse_set_strip_props` for transforms.
- **`vse_setup` with `clear: true` deletes every strip**, and **`vse_render`
  renders the animation, not a still** (restoring the frame range afterwards).
- **`text_datablock_set` replaces the body.** Read with `text_datablock_get`
  first if you meant to append. **`img_save` without `path`** errors on a
  generated image that has no filepath of its own.
- **`file_import` has no STL or PLY path** even though 5.2 has the operators,
  and **`scene_purge_unused` is recursive and unconditional** — `file_save`
  first if the file matters.

## See also

`kdb_guide` areas: `rendering-and-cameras` (`render_set` vs `vse_render`,
`img_save` for bake results), `physics-and-simulation` (save before a bake).
`kdb_search`: "library override hierarchy", "wm.obj_export", "strip
left_handle_offset", "grease pencil drawing add_strokes", "orphans_purge".
