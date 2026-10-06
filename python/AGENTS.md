# python — test scripts

This is **not** part of the product. There is no tool logic here — all of it is in `src/tools/`.
Only what we use to verify the build lives here.

| File | What it does | Needs Blender? |
|---|---|---|
| `run_tests.py` | Runner: runs `test_mcp_server.py` | no |
| `test_mcp_server.py` | Brings up the server and talks over stdio (`rmcp` format): `initialize`, `tools/list`, and the **server-side control tools** (`blender_status` answers without Blender) | no |
| `blender_smoke.py` | Quick slice: runs tools against a real scene | **yes** |
| `tests/` | Per-domain suite: `test_<domain>.py` + `harness.py` + `run.py`. Each domain on a fresh scene; what is checked is the **effect in `bpy`**, not just "the call returned". Run via `run.py` through `bootstrap.py t` | **yes** |

Run via `bootstrap.py t` — it builds the `.pyd`, puts it into staging and
hands it to Blender. Manually:

```bash
python python/run_tests.py                       # server only, no Blender
blender --background --factory-startup \
    --python python/blender_smoke.py -- <directory with blender_mcp.pyd>
```

## What `blender_smoke.py` covers and why exactly this

The three shapes that break most often:

1. **Single mode** — simple calls.
2. **Batch mode** — nested typed structures (`primitives`, `updates`,
   `operations`). This is where the port from Python broke most readily.
3. **Error contract** — a failed single call must be an **error**, while a batch
   with one broken item must remain a **success**, carrying the failure as data. This
   is not cosmetic: previously a failure was returned as a successful result with an `error` key,
   and the model read right past it.

## What is not here and will not be

**`scene_screenshot` is not tested here.** It needs an OpenGL context; in
`--background` Blender does not provide one, and the call fails. That is exactly why its breakage
(the `full` argument, removed in Blender 5) survived dozens of headless runs and
was found only by hand in the GUI. Verify only in the GUI.
