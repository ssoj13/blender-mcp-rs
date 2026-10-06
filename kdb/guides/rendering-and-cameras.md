---
area: guide
title: Rendering and cameras — how to work in it
version: 5.2
---

# Rendering and cameras

Everything between "the scene exists" and "there is an image". Cameras, lights,
world/HDRI, the render settings that persist in the file, the one-shot renders
that do not, colour management, render passes for compositing, and the job
queue. An agent lands here when it has to *look* at what it built, or hand the
user a frame.

## The tools

| Tool | What it is for |
|---|---|
| `cam_create` | New camera datablock + object. `type`: PERSP, ORTHO, PANO (5.2 also has CUSTOM, which this tool does not set up) |
| `cam_update` | Change an existing camera: lens, clipping, ortho_scale, DOF |
| `cam_set_active` | Point `scene.camera` at a camera. Renders use this one |
| `cam_look_at` | Rotate any object to face an object name or `[x, y, z]`. Not camera-only |
| `light_create` | New light. `type`: POINT, SUN, SPOT, AREA |
| `light_update` | Change one light. Type-specific params are silently ignored on the wrong type — see Traps |
| `world_set_hdri` | Replace the world node tree with TexCoord/Mapping/Environment/Background/Output and load an HDRI |
| `world_set_background` | Just colour + strength on the existing Background node. Use instead of `world_set_hdri` when there is no image |
| `world_get` | Read the current world settings back |
| `render_set` | Render settings that **persist**: engine, resolution, samples, film transparency, motion blur, output path/format/depth |
| `scene_render` | Render one frame to a file with **temporary** overrides. Settings are restored afterwards |
| `scene_screenshot` | Capture viewport/window/render and return the image to the model. GUI only — see Traps |
| `mat_preview` | Render one material on a sphere/cube/plane/cloth/monkey in a throwaway scene. Nothing in the user's file changes |
| `color_set` | View transform, look, exposure, gamma |
| `view_layer_passes` | Enable/disable render passes on the active view layer |
| `render_queue_add`, `render_queue_list`, `render_queue_start`, `render_queue_clear` | Queue animation jobs, then run them all in one blocking call |
| `scene_apply_template`, `scene_list_templates` | Prebuilt light+camera rigs: studio, outdoor, product_shot, architectural, portrait |
| `scene_set` | Frame range, current frame, fps — what `render_queue_start` and `vse_render` read |
| `scene_get_info` | Read back engine, resolution, view transform, film transparency, enabled passes |

## How to do the usual things

### Frame a subject and shoot it

1. `cam_create` with a `location`; skip `rotation`.
2. `cam_look_at` with `{"name": "Camera", "target": "Suzanne"}` — targeting by
   object name avoids computing an euler by hand. `target` also takes
   `[x, y, z]`.
3. `cam_set_active` — `cam_create` does **not** make the new camera active.
4. `scene_render` with `output`.

### Depth of field

DOF lives on the camera data, not on the render settings. `aperture` is the
f-stop (`dof.aperture_fstop`), `focus_distance` is in metres. Smaller f-stop =
shallower.

```json
{"name": "Camera", "dof_enabled": true, "focus_distance": 4.2, "aperture": 1.8}
```

`cam_update` writes `focus_distance`/`aperture` whether or not DOF is on, so
setting them without `dof_enabled: true` changes nothing visible.

### Lens vs ortho_scale

They are not interchangeable. `lens` (mm, default 50) only affects a PERSP (or
PANO) camera; `ortho_scale` (world units across the frame, default 6.0) only
affects an ORTHO one. Switching `type` to ORTHO with `cam_update` does **not**
set `ortho_scale` — `cam_create` only defaults it when the camera is created as
ORTHO. Pass it explicitly in the same call.

### Lights: which parameter applies to which type

Verified against 5.2 RNA. `light_update` checks the light's actual type and
drops a parameter that does not belong to it, without error — a `spot_size` sent
to an AREA light is a silent no-op, not a failure.

| Param | POINT | SUN | SPOT | AREA |
|---|---|---|---|---|
| `energy`, `color`, `shadow` | yes | yes | yes | yes |
| `radius` (shadow soft size) | yes | — | — | — |
| `angle` (angular diameter) | — | yes | — | — |
| `spot_size`, `spot_blend` | — | — | yes | — |
| `shape`, `size`, `size_y` | — | — | — | yes (`shape`/`size_y` are `light_create` only) |

`energy` is watts for POINT/SPOT/AREA and irradiance for SUN — `light_create`
defaults to 1000.0 for POINT and 5.0 for everything else, so a SUN created with
`energy: 1000` blows the frame out. AREA `shape`: SQUARE, RECTANGLE, DISK,
ELLIPSE; `size_y` is only read for RECTANGLE and ELLIPSE. Angles
(`spot_size`, `spot_blend` edge softness 0–1, `angle`) are radians.

### World and HDRI

`world_set_hdri` **clears the world node tree** and rebuilds it — any custom
world shading is lost. `rotation` is `[x, y, z]` radians on the Mapping node.
`world_set_background` is the non-destructive one: it finds the existing
Background node and writes Color/Strength, creating a world only if the scene
has none.

### Persisting settings vs one-shot renders

Use `render_set` when the choice should survive: it writes `scene.render`,
`scene.eevee` or `scene.cycles` and leaves them. Use `scene_render` when only
this frame should be that way: engine, resolution and filepath are restored
afterwards, so a `scene_render` with `engine: "CYCLES"` does *not* leave the
scene on Cycles.

```json
{"engine": "CYCLES", "resolution_x": 1920, "resolution_y": 1080,
 "samples": 128, "film_transparent": true,
 "filepath": "//render/shot_", "file_format": "OPEN_EXR", "color_depth": "32"}
```

`samples` goes to a different RNA path per engine: EEVEE's
`taa_render_samples`, Cycles' `samples`. Workbench has no integer sample count
at all and `render_set` errors rather than dropping the value. `scene_render`'s
own `samples` is **Cycles-only** and is ignored on EEVEE.

5.2 `file_format` values: PNG, JPEG, JPEG2000, OPEN_EXR, OPEN_EXR_MULTILAYER,
TIFF, TARGA, TARGA_RAW, BMP, IRIS, CINEON, DPX, HDR, WEBP, AVIF, FFMPEG.
`color_depth` is a string: "8", "10", "12", "16", "32", and which ones are legal
depends on the format.

### Colour management

`color_set` writes `scene.view_settings`. On a stock 5.2 OCIO config the view
transforms are AgX (the default), Standard, Filmic, Filmic Log, Raw, False Color
and Khronos PBR Neutral; looks include "AgX - Punchy", "AgX - Base Contrast" and
"None". Valid looks depend on the active view transform, so pass both in one
call — `view_transform` is applied first, then `look` is checked against it.
`exposure` is in stops; `gamma` applies after the transform (1.0 = off). A bad
value errors with this build's real option list, so guessing is cheap to correct
but still slower than `scene_get_info`.

### Render passes for compositing

`view_layer_passes` toggles what the compositor's Render Layers node can output.
Cryptomatte is the reliable way to isolate objects in comp;
`object_index`/`material_index` need a per-object/per-material Pass Index set
first, or the pass comes out uniformly zero. A pass this build does not have is
reported in `unavailable` rather than silently skipped.

```json
{"z": true, "normal": true, "cryptomatte_object": true, "mist": false}
```

### The render queue

`render_queue_add` stores a job as JSON in a scene custom property; nothing
renders until `render_queue_start`, which renders **animations**, blocks Blender
until every pending job finishes, and marks each job `completed` or
`failed: …`. One job's failure does not abort the rest. Job ids come from a
monotonic counter and are never reused. Set the range per job with
`start_frame`/`end_frame`; omitted, they come from the scene's current range
(`scene_set`). `render_queue_clear` with `completed_only: true` drops exactly
the completed jobs and keeps pending and failed ones.

Job settings are applied to the scene and **not restored** — after a queue run
the scene carries the last job's resolution, camera, engine and frame range.

For a single still, `scene_render` is the right tool; the queue is for batches
of animation.

### Looking at a material without touching the scene

`mat_preview` renders into a disposable scene that is deleted afterwards. Cheap
by design: 256 px, 24 EEVEE samples (32 with Cycles + denoising), capped at
1024. Pass `hdri` for image-based lighting instead of the built-in three-point
rig.

```json
{"material": "Copper", "shape": "cloth", "resolution": 512, "engine": "CYCLES"}
```

### Scene templates

`scene_apply_template` builds a light rig, a camera and (for some templates) a
floor, and sets the scene camera. `clear_scene: true` deletes **every object in
the file**, not just the scene's — check with `scene_get_info` first on a file
the user cares about. `scene_list_templates` names the five and what each is.

## Traps

- **`scene_screenshot` does not work in `--background`.** Verified on 5.2:
  `render.opengl` raises "Cannot use OpenGL render in background mode (no opengl
  context)" and `screen.screenshot`'s poll is False. Only `mode: "render"` works
  headless, and that is just `scene_render` returning the image inline. Launch
  Blender with a GUI (`blender_launch` without `background`) for viewport
  captures.
- **The engine enum under-reports.** On 5.2 `scene.render`'s RNA lists only
  `BLENDER_EEVEE` even though CYCLES and BLENDER_WORKBENCH are both settable.
  `render_set` falls back to resolving the alias against Blender's live engine
  list, so `"cycles"`, `"CYCLES"` and `"workbench"` all work.
- **EEVEE's identifier moved across versions** (BLENDER_EEVEE, then
  BLENDER_EEVEE_NEXT in 4.2, then BLENDER_EEVEE again in 5.0). Pass the friendly
  name and let the tool resolve it; never hardcode.
- **`cam_create` does not set the active camera.** `scene_render` renders from
  `scene.camera`; without `cam_set_active` you render from whatever was there.
- **`film_transparent` is a render setting, not a world setting.** Set it with
  `render_set` and write OPEN_EXR or PNG — JPEG has no alpha to keep.
- **`world_set_hdri` wipes the world node tree.** Read `world_get` first if the
  world might be hand-built. (5.2 also deprecates `World.use_nodes`, which both
  world tools still set; it is harmless until Blender 6.0.)
- **A queue run leaves the scene changed.** Re-apply `render_set`/`scene_set`
  afterwards if the interactive state matters.
- **`resolution_percentage` scales the output**: 1920x1080 at 50% writes
  960x540. `scene_get_info` reports the base resolution, not the scaled one.

## See also

`kdb_guide` areas: `physics-and-simulation` (bake before rendering an
animation), `pipeline-io` (`vse_render` for sequencer output, `img_save`).
`kdb_search` queries worth running: "view_transform AgX", "use_pass_cryptomatte",
"taa_render_samples", "aperture_fstop", "render engine identifier".
