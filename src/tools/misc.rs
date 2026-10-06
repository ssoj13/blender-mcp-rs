//! Misc tools: rendering, screenshots, file I/O, undo/redo, Python exec,
//! modifiers, collections. Registered automatically — nothing outside this
//! file names them.
//!
//! Several tools support a "batch mode" (a list of entries in one call) alongside a
//! "single mode" (flat fields) — both exist because MCP round-trips are expensive and
//! callers rely on batching to avoid one call per object.

use base64::{Engine, engine::general_purpose::STANDARD as B64};
use pyo3::Py;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyIterator, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use super::bpy::{data, op_ok, scene};

// ============================================================================
// Shared helpers
// ============================================================================

/// Apply a JSON value to a Blender RNA property by name. Mirrors the subset
/// of Python types Blender's `setattr` accepts: bool/int/float/str verbatim,
/// arrays of numbers as a tuple (covers Blender enums-as-vectors like
/// `offset`, `location`). Objects and null are silently skipped — they don't
/// map onto a single RNA attribute.
fn setattr_from_json(obj: &Bound<'_, PyAny>, k: &str, v: &Value) -> PyResult<()> {
    match v {
        Value::Null => {}
        Value::Bool(b) => {
            obj.setattr(k, *b)?;
        }
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                obj.setattr(k, i as i32)?;
            } else if let Some(f) = n.as_f64() {
                obj.setattr(k, f)?;
            }
        }
        Value::String(s) => {
            obj.setattr(k, s.as_str())?;
        }
        Value::Array(a) => {
            let nums: Vec<f64> = a.iter().filter_map(|x| x.as_f64()).collect();
            if nums.len() == a.len() && !nums.is_empty() {
                let py = obj.py();
                obj.setattr(k, PyTuple::new(py, nums)?)?;
            }
        }
        Value::Object(_) => {}
    }
    Ok(())
}

/// Apply a `properties` map onto a modifier, skipping keys the modifier type
/// doesn't actually have (different modifier types expose different RNA).
fn apply_properties(m: &Bound<'_, PyAny>, properties: Option<&Value>) -> PyResult<()> {
    if let Some(Value::Object(pmap)) = properties {
        for (k, v) in pmap.iter() {
            if m.hasattr(k.as_str())? {
                setattr_from_json(m, k, v)?;
            }
        }
    }
    Ok(())
}

fn coll_info(c: &Bound<'_, PyAny>) -> PyResult<Value> {
    let name: String = c.getattr("name")?.extract()?;
    let objects = c.getattr("objects")?;
    let mut obj_names = Vec::new();
    for o in PyIterator::from_object(&objects)? {
        obj_names.push(o?.getattr("name")?.extract::<String>()?);
    }
    let children = c.getattr("children")?;
    let mut ch = Vec::new();
    for child in PyIterator::from_object(&children)? {
        ch.push(coll_info(&child?)?);
    }
    Ok(json!({
        "name": name,
        "objects": obj_names,
        "children": ch,
    }))
}

// ============================================================================
// Rendering & screenshots
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneRender {
    /// Absolute output path for the still image. The file format follows the
    /// scene's current image settings (PNG by default), and Blender appends the
    /// extension if the path has none.
    pub output: String,
    /// Render engine. Accepts EEVEE, CYCLES or WORKBENCH in any case — the exact
    /// id for this Blender (BLENDER_EEVEE / CYCLES / BLENDER_WORKBENCH on 5.2)
    /// is resolved live, so short names are safe. Default: the scene's engine,
    /// which is restored afterwards either way.
    pub engine: Option<String>,
    /// Horizontal resolution in pixels. Default: the scene's `resolution_x`.
    /// Restored after the render.
    pub width: Option<i32>,
    /// Vertical resolution in pixels. Default: the scene's `resolution_y`.
    /// Restored after the render.
    pub height: Option<i32>,
    /// Cycles sample count. Applied only when the effective engine is CYCLES;
    /// silently ignored on EEVEE/Workbench. Default: the scene's own setting.
    pub samples: Option<i32>,
}

crate::tool!(
    "scene_render",
    "Render the current scene's camera view to an image file and restore every \
     setting it touched (engine, resolution, output path). Renders one still — \
     for the sequencer timeline use vse_render, and for a quick look at the \
     viewport without writing a file use scene_screenshot.",
    SceneRender,
    |py: Python<'_>, p: SceneRender| {
        let output = p.output;
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        let render = s.getattr("render")?;
        let old_engine: String = render.getattr("engine")?.extract()?;
        let old_fp: String = render.getattr("filepath")?.extract()?;
        let old_rx: i32 = render.getattr("resolution_x")?.extract()?;
        let old_ry: i32 = render.getattr("resolution_y")?.extract()?;

        // Render settings are mutated in place for the duration of the render
        // and restored afterwards regardless of outcome, so the scene is left
        // exactly as the caller found it.
        let out = (|| -> PyResult<Value> {
            if let Some(e) = p.engine.as_deref() {
                render.setattr("engine", super::bpy::render_engine(&s, e)?)?;
            }
            if let Some(w) = p.width {
                render.setattr("resolution_x", w)?;
            }
            if let Some(h) = p.height {
                render.setattr("resolution_y", h)?;
            }
            if let Some(samples) = p.samples {
                let eng: String = render.getattr("engine")?.extract()?;
                if eng == "CYCLES"
                    && let Ok(cycles) = s.getattr("cycles")
                {
                    cycles.setattr("samples", samples)?;
                }
            }
            render.setattr("filepath", output.as_str())?;
            let ops = bpy.getattr("ops")?;
            let rk = PyDict::new(py);
            rk.set_item("write_still", true)?;
            let result = ops
                .getattr("render")?
                .call_method("render", (), Some(&rk))?;
            op_ok(&result, "render")?;
            let eng: String = render.getattr("engine")?.extract()?;
            let rx: i32 = render.getattr("resolution_x")?.extract()?;
            let ry: i32 = render.getattr("resolution_y")?.extract()?;
            Ok(json!({
                "output": output,
                "engine": eng,
                "resolution": [rx, ry],
            }))
        })();

        render.setattr("engine", old_engine)?;
        render.setattr("filepath", old_fp)?;
        render.setattr("resolution_x", old_rx)?;
        render.setattr("resolution_y", old_ry)?;

        out
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneScreenshot {
    /// What to capture. One of: "viewport" (OpenGL render of the 3D view),
    /// "window" (the whole Blender window), "render" (a real camera render).
    /// Lowercase, matched exactly; anything unrecognised behaves as "viewport".
    /// Default: "viewport". NOTE: "viewport" and "window" need a GUI Blender —
    /// there is no OpenGL context under `--background`. "render" does work headless
    /// (verified on 5.2), so it is the only way to see a scene in that mode.
    pub mode: Option<String>,
    /// Viewport shading for mode "viewport". One of: wireframe, solid, material,
    /// rendered (case-insensitive; passed through uppercased otherwise).
    /// Default: whatever the 3D view is currently set to. Restored afterwards.
    pub shading: Option<String>,
    /// Render engine for mode "render" and for shading "rendered". Accepts
    /// EEVEE, CYCLES or WORKBENCH in any case; the exact id is resolved live.
    /// Default: the scene's engine, restored afterwards either way.
    pub engine: Option<String>,
    /// Image width in pixels. Ignored in mode "window", which captures at the
    /// window's own size. Default: 512.
    pub width: Option<i32>,
    /// Image height in pixels. Ignored in mode "window". Default: 512.
    pub height: Option<i32>,
}

crate::tool!(
    "scene_screenshot",
    "Capture the viewport, the whole window, or a camera render and return the PNG \
     inline so the model can look at it — the way to actually SEE the scene. \
     `viewport` and `window` need a GUI Blender (they read the screen, so they need an \
     OpenGL context); `mode: \"render\"` renders through the camera instead and works \
     in a headless --background Blender too. To write a render to a file rather than \
     receive it inline, use scene_render.",
    SceneScreenshot,
    |py: Python<'_>, p: SceneScreenshot| {
        let mode = p.mode.unwrap_or_else(|| "viewport".to_string());
        let shading = p.shading;
        let engine = p.engine;
        let width = p.width.unwrap_or(512);
        let height = p.height.unwrap_or(512);

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let tmp_path = std::env::temp_dir().join(format!("mcp_cap_{nanos}.png"));
        let tmp_str = tmp_path.to_string_lossy().to_string();

        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        let render = s.getattr("render")?;
        let old_engine: String = render.getattr("engine")?.extract()?;

        if let Some(e) = engine.as_deref() {
            render.setattr("engine", super::bpy::render_engine(&s, e)?)?;
        }

        // Headless has no OpenGL context, so the two screen-reading modes cannot work
        // there; "render" can. Saying so beats the raw Blender error a caller would
        // otherwise get ("Cannot use OpenGL render in background mode").
        if mode != "render"
            && bpy
                .getattr("app")?
                .getattr("background")?
                .extract::<bool>()?
        {
            return Err(PyValueError::new_err(format!(
                "scene_screenshot mode `{mode}` needs a GUI Blender: there is no OpenGL                  context under --background. Use mode \"render\" to capture through the                  camera instead."
            )));
        }

        let capture = (|| -> PyResult<()> {
            if mode == "render" {
                let old_rx: i32 = render.getattr("resolution_x")?.extract()?;
                let old_ry: i32 = render.getattr("resolution_y")?.extract()?;
                let old_fp: String = render.getattr("filepath")?.extract()?;
                let old_fmt: String = render
                    .getattr("image_settings")?
                    .getattr("file_format")?
                    .extract()?;
                render.setattr("resolution_x", width)?;
                render.setattr("resolution_y", height)?;
                render.setattr("filepath", &tmp_str)?;
                render
                    .getattr("image_settings")?
                    .setattr("file_format", "PNG")?;
                let rk = PyDict::new(py);
                rk.set_item("write_still", true)?;
                let result =
                    bpy.getattr("ops")?
                        .getattr("render")?
                        .call_method("render", (), Some(&rk))?;
                op_ok(&result, "render screenshot")?;
                render.setattr("resolution_x", old_rx)?;
                render.setattr("resolution_y", old_ry)?;
                render.setattr("filepath", old_fp)?;
                render
                    .getattr("image_settings")?
                    .setattr("file_format", old_fmt)?;
            } else if mode == "window" {
                // `screen.screenshot` already captures the whole window. The Python
                // original passed `full=True`, an argument Blender removed — on
                // Blender 5 that raises "keyword 'full' unrecognized", so window
                // mode was simply broken. (A single area is `screenshot_area`.)
                let kw = PyDict::new(py);
                kw.set_item("filepath", &tmp_str)?;
                let result = bpy.getattr("ops")?.getattr("screen")?.call_method(
                    "screenshot",
                    (),
                    Some(&kw),
                )?;
                op_ok(&result, "capture window screenshot")?;
            } else {
                let old_rx: i32 = render.getattr("resolution_x")?.extract()?;
                let old_ry: i32 = render.getattr("resolution_y")?.extract()?;
                let old_fp: String = render.getattr("filepath")?.extract()?;
                let old_fmt: String = render
                    .getattr("image_settings")?
                    .getattr("file_format")?
                    .extract()?;

                let mut old_shading: Option<String> = None;
                let mut space_opt: Option<Py<PyAny>> = None;
                let screen = bpy.getattr("context")?.getattr("screen")?;
                let areas = screen.getattr("areas")?;
                for area in PyIterator::from_object(&areas)? {
                    let area = area?;
                    let ty: String = area.getattr("type")?.extract()?;
                    if ty == "VIEW_3D" {
                        let space = area.getattr("spaces")?.getattr("active")?;
                        old_shading = Some(space.getattr("shading")?.getattr("type")?.extract()?);
                        if let Some(sh) = shading.as_deref() {
                            let sm = match sh.to_lowercase().as_str() {
                                "wireframe" => "WIREFRAME".to_string(),
                                "solid" => "SOLID".to_string(),
                                "material" => "MATERIAL".to_string(),
                                "rendered" => "RENDERED".to_string(),
                                _ => sh.to_uppercase(),
                            };
                            space.getattr("shading")?.setattr("type", sm.as_str())?;
                        }
                        space_opt = Some(space.unbind());
                        break;
                    }
                }

                render.setattr("resolution_x", width)?;
                render.setattr("resolution_y", height)?;
                render.setattr("filepath", &tmp_str)?;
                render
                    .getattr("image_settings")?
                    .setattr("file_format", "PNG")?;
                let rk = PyDict::new(py);
                rk.set_item("write_still", true)?;
                let result =
                    bpy.getattr("ops")?
                        .getattr("render")?
                        .call_method("opengl", (), Some(&rk))?;
                op_ok(&result, "capture viewport screenshot")?;

                render.setattr("resolution_x", old_rx)?;
                render.setattr("resolution_y", old_ry)?;
                render.setattr("filepath", old_fp)?;
                render
                    .getattr("image_settings")?
                    .setattr("file_format", old_fmt)?;

                if let (Some(os), Some(sp)) = (old_shading, space_opt) {
                    let sp = sp.bind(py);
                    sp.getattr("shading")?.setattr("type", os.as_str())?;
                }
            }
            Ok(())
        })();

        render.setattr("engine", &old_engine)?;
        capture?;

        let bytes = fs::read(&tmp_path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("read screenshot: {e}")))?;
        // The screenshot is already in memory; a stale temp file must not fail the
        // call, but it must not vanish silently either.
        if let Err(e) = fs::remove_file(&tmp_path) {
            eprintln!("[blender-mcp] could not remove temp screenshot {tmp_path:?}: {e}");
        }

        let img_data = B64.encode(&bytes);
        Ok(json!({
            "__image__": true,
            "data": img_data,
            "mimeType": "image/png",
            "width": width,
            "height": height,
            "mode": mode,
            "shading": shading,
            "engine": engine,
        }))
    }
);

// ============================================================================
// Utility: Python exec, mode, undo/redo
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UtilExecutePython {
    /// Python source, executed inside Blender on the main thread with `bpy`
    /// already bound in the namespace. Multi-line is fine. Assign to the
    /// variable `__result__` to send data back — it is JSON-serialised, and
    /// anything not JSON-serialisable comes back as null.
    pub code: String,
}

crate::tool!(
    "util_execute_python",
    "Run arbitrary Python inside Blender when no dedicated tool covers what is \
     needed; set `__result__` to return data. The escape hatch, not the first \
     choice — it has no schema, no validation and no undo, so prefer the typed \
     tool for anything that has one.",
    UtilExecutePython,
    |py: Python<'_>, p: UtilExecutePython| {
        let bpy = py.import("bpy")?;
        let builtins = py.import("builtins")?;
        let exec = builtins.getattr("exec")?;
        let ns = PyDict::new(py);
        ns.set_item("bpy", &bpy)?;
        ns.set_item("__result__", py.None())?;
        exec.call1((p.code.as_str(), ns.clone()))?;
        let res = ns.get_item("__result__")?;
        let pyjson = py.import("json")?;
        let dumped: String = pyjson.call_method1("dumps", (res,))?.extract()?;
        let parsed: Value = serde_json::from_str(&dumped).unwrap_or(Value::Null);
        Ok(json!({ "executed": true, "result": parsed }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UtilSetMode {
    /// Interaction mode for the active object. Blender 5.2 `object.mode_set`
    /// enum, verified live: OBJECT, EDIT, POSE, SCULPT, VERTEX_PAINT,
    /// WEIGHT_PAINT, TEXTURE_PAINT, PARTICLE_EDIT, SCULPT_CURVES,
    /// EDIT_GPENCIL, SCULPT_GREASE_PENCIL, PAINT_GREASE_PENCIL,
    /// WEIGHT_GREASE_PENCIL, VERTEX_GREASE_PENCIL. Uppercased before use. Which
    /// ones are valid depends on the active object's type (POSE needs an
    /// ARMATURE, the GREASE_PENCIL ones need a GREASEPENCIL object, ...).
    pub mode: String,
}

crate::tool!(
    "util_set_mode",
    "Switch the ACTIVE object's interaction mode (OBJECT, EDIT, POSE, SCULPT, the \
     paint modes, ...). A no-op when nothing is active. Most tools in this server \
     already handle mode themselves, so this is mainly for leaving an object in a \
     particular mode for the user.",
    UtilSetMode,
    |py: Python<'_>, p: UtilSetMode| {
        let mode = p.mode.to_uppercase();
        let bpy = py.import("bpy")?;
        if let Ok(active) = bpy.getattr("context")?.getattr("active_object")
            && !active.is_none()
        {
            let ops = bpy.getattr("ops")?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("mode", mode.as_str())?;
            let result = ops
                .getattr("object")?
                .call_method("mode_set", (), Some(&kwargs))?;
            // Unlike the internal set_mode()-style restore helpers elsewhere in
            // this crate, setting the mode IS what this tool was asked to do, so
            // its result is checked here rather than left to a bare `?`.
            op_ok(&result, &format!("set mode to {mode}"))?;
        }
        Ok(json!({ "mode": mode }))
    }
);

/// `util_undo` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct UtilUndo {}

crate::tool!(
    "util_undo",
    "Step back one entry in Blender's undo stack (`ed.undo`). Takes no arguments. \
     Reverses with util_redo. The stack is Blender's own, so it also covers \
     actions the user took in the UI, not just tool calls.",
    UtilUndo,
    |py: Python<'_>, _p: UtilUndo| {
        let bpy = py.import("bpy")?;
        let result = bpy.getattr("ops")?.getattr("ed")?.call_method0("undo")?;
        op_ok(&result, "undo")?;
        Ok(json!({ "action": "util_undo" }))
    }
);

/// `util_redo` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct UtilRedo {}

crate::tool!(
    "util_redo",
    "Step forward one entry in Blender's undo stack (`ed.redo`), re-applying what \
     util_undo took back. Takes no arguments.",
    UtilRedo,
    |py: Python<'_>, _p: UtilRedo| {
        let bpy = py.import("bpy")?;
        let result = bpy.getattr("ops")?.getattr("ed")?.call_method0("redo")?;
        op_ok(&result, "redo")?;
        Ok(json!({ "action": "util_redo" }))
    }
);

// ============================================================================
// File I/O
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct FileExport {
    /// Absolute output path, extension included. It is passed to the operator
    /// verbatim and is NOT used to pick the format — `format` does that.
    pub path: String,
    /// Output format, lowercase. Exactly these seven are supported: "fbx"
    /// (`export_scene.fbx`), "obj" (`wm.obj_export`), "gltf"
    /// (`export_scene.gltf` — also writes .glb depending on the operator's own
    /// setting), "usd" (`wm.usd_export`), "abc" (`wm.alembic_export`), "stl"
    /// (`wm.stl_export`), "ply" (`wm.ply_export`). Anything else is an error,
    /// never a fallback. Default: "fbx".
    pub format: Option<String>,
    /// true: export only the selected objects, mapped to whichever flag the
    /// per-format operator uses (`use_selection`, `export_selected_objects`,
    /// `selected_objects_only`, `selected`). false: export the whole scene.
    /// Default: false.
    pub selected_only: Option<bool>,
}

crate::tool!(
    "file_export",
    "Write the scene (or just the selection) out to an interchange file — fbx \
     (default), obj, gltf, usd, abc, stl or ply. Unlike file_import the format is NOT \
     inferred from the path, so pass `format`. Saving the .blend itself is \
     file_save; exporting a single data-block for another .blend is not this tool \
     (see asset_mark/asset_import).",
    FileExport,
    |py: Python<'_>, p: FileExport| {
        let path = p.path;
        let fmt = p.format.unwrap_or_else(|| "fbx".to_string()).to_lowercase();
        let selected_only = p.selected_only.unwrap_or(false);
        let bpy = py.import("bpy")?;
        let ops = bpy.getattr("ops")?;
        let kwargs = PyDict::new(py);
        kwargs.set_item("filepath", path.as_str())?;
        kwargs.set_item("use_selection", selected_only)?;
        let result = match fmt.as_str() {
            "fbx" => ops
                .getattr("export_scene")?
                .call_method("fbx", (), Some(&kwargs))?,
            "obj" => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                kw.set_item("export_selected_objects", selected_only)?;
                ops.getattr("wm")?
                    .call_method("obj_export", (), Some(&kw))?
            }
            "gltf" => ops
                .getattr("export_scene")?
                .call_method("gltf", (), Some(&kwargs))?,
            "usd" => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                kw.set_item("selected_objects_only", selected_only)?;
                ops.getattr("wm")?
                    .call_method("usd_export", (), Some(&kw))?
            }
            "abc" => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                kw.set_item("selected", selected_only)?;
                ops.getattr("wm")?
                    .call_method("alembic_export", (), Some(&kw))?
            }
            // `wm.stl_export` / `wm.ply_export` take the same
            // `export_selected_objects` argument as `wm.obj_export` (verified on 5.2).
            "stl" | "ply" => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                kw.set_item("export_selected_objects", selected_only)?;
                ops.getattr("wm")?
                    .call_method(format!("{fmt}_export"), (), Some(&kw))?
            }
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Unknown format: {fmt}. Use: fbx, obj, gltf, usd, abc, stl, ply"
                )));
            }
        };
        op_ok(&result, &format!("export {path} as {fmt}"))?;
        Ok(json!({ "exported": path, "format": fmt }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct FileImport {
    /// Absolute path to the file to import.
    pub path: String,
    /// Input format, lowercase. Exactly these are supported: "fbx"
    /// (`import_scene.fbx`), "obj" (`wm.obj_import`), "gltf"/"glb"
    /// (`import_scene.gltf`), "usd"/"usda"/"usdc" (`wm.usd_import`), "abc"
    /// (`wm.alembic_import`), "stl" (`wm.stl_import`), "ply" (`wm.ply_import`).
    /// Omit to take it from `path`'s extension; an extension outside that list is
    /// an error, never a fallback.
    pub format: Option<String>,
}

crate::tool!(
    "file_import",
    "Bring an interchange file into the current scene — fbx, obj, gltf/glb, \
     usd/usda/usdc, abc, stl or ply — with the format taken from the extension unless \
     `format` says otherwise. For a data-block out of another .blend use \
     asset_import (append or link) instead, and library_override_create to make a \
     linked one editable. Write files back out with file_export.",
    FileImport,
    |py: Python<'_>, p: FileImport| {
        let path = p.path;
        let fmt = if let Some(f) = p.format {
            f.to_lowercase()
        } else {
            std::path::Path::new(&path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase()
        };
        let bpy = py.import("bpy")?;
        let ops = bpy.getattr("ops")?;
        let result = match fmt.as_str() {
            "fbx" => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                ops.getattr("import_scene")?
                    .call_method("fbx", (), Some(&kw))?
            }
            "obj" => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                ops.getattr("wm")?
                    .call_method("obj_import", (), Some(&kw))?
            }
            "gltf" | "glb" => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                ops.getattr("import_scene")?
                    .call_method("gltf", (), Some(&kw))?
            }
            "usd" | "usda" | "usdc" => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                ops.getattr("wm")?
                    .call_method("usd_import", (), Some(&kw))?
            }
            "abc" => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                ops.getattr("wm")?
                    .call_method("alembic_import", (), Some(&kw))?
            }
            "stl" | "ply" => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                ops.getattr("wm")?
                    .call_method(format!("{fmt}_import"), (), Some(&kw))?
            }
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Unknown format: {fmt}. Use: fbx, obj, gltf/glb, usd/usda/usdc, abc, stl, ply"
                )));
            }
        };
        op_ok(&result, &format!("import {path} as {fmt}"))?;
        Ok(json!({ "imported": path, "format": fmt }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct FileSave {
    /// Absolute .blend path to save as, which also becomes the current file.
    /// Omit to save in place — which fails on a file that has never been saved,
    /// since there is no path to save to.
    pub path: Option<String>,
}

crate::tool!(
    "file_save",
    "Save the current .blend, either in place (no `path`) or as a new file \
     (`path`). Worth doing before a big import, a bake, or any library_* call. \
     Exporting to fbx/obj/gltf/usd/abc is file_export, not this.",
    FileSave,
    |py: Python<'_>, p: FileSave| {
        let bpy = py.import("bpy")?;
        let ops = bpy.getattr("ops")?;
        let wm = ops.getattr("wm")?;
        if let Some(path) = p.path {
            let kw = PyDict::new(py);
            kw.set_item("filepath", path.as_str())?;
            let result = wm.call_method("save_as_mainfile", (), Some(&kw))?;
            op_ok(&result, &format!("save file to {path}"))?;
            return Ok(json!({ "saved": path }));
        }
        let result = wm.call_method0("save_mainfile")?;
        op_ok(&result, "save file")?;
        let fp: String = data(&bpy)?.getattr("filepath")?.extract()?;
        Ok(json!({ "saved": fp }))
    }
);

// ============================================================================
// Modifiers
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ModList {
    /// Object name in `bpy.data.objects`.
    pub name: String,
}

crate::tool!(
    "mod_list",
    "List an object's modifier stack in evaluation order: name, type, and the \
     viewport/render visibility flags. The source of the `modifier` name that \
     mod_update, mod_apply and mod_delete take.",
    ModList,
    |py: Python<'_>, p: ModList| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Object not found: {}",
                p.name
            )));
        }
        let mods = obj.getattr("modifiers")?;
        let mut rows = Vec::new();
        for m in PyIterator::from_object(&mods)? {
            let m = m?;
            rows.push(json!({
                "name": m.getattr("name")?.extract::<String>()?,
                "type": m.getattr("type")?.extract::<String>()?,
                "show_viewport": m.getattr("show_viewport")?.extract::<bool>()?,
                "show_render": m.getattr("show_render")?.extract::<bool>()?,
            }));
        }
        Ok(Value::Array(rows))
    }
);

/// One entry of `mod_add`'s batch `modifiers` list.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct ModAddEntry {
    /// Object to add the modifier to.
    pub name: String,
    /// Modifier type id (see `mod_add`'s `type`).
    pub r#type: String,
    /// Name for the new modifier. Default: the type id.
    pub mod_name: Option<String>,
    /// Flat map of RNA property -> value (see `mod_add`'s `properties`).
    pub properties: Option<Value>,
}

fn mod_add_one(
    py: Python<'_>,
    name: &str,
    ty: &str,
    mod_name: Option<&str>,
    properties: Option<&Value>,
) -> PyResult<Value> {
    let bpy = py.import("bpy")?;
    let d = data(&bpy)?;
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {name}")));
    }
    let mod_name = mod_name.unwrap_or(ty);
    let mods = obj.getattr("modifiers")?;
    let kw = PyDict::new(py);
    kw.set_item("name", mod_name)?;
    kw.set_item("type", ty)?;
    let m = mods.call_method("new", (), Some(&kw))?;
    apply_properties(&m, properties)?;
    Ok(json!({
        "object": obj.getattr("name")?.extract::<String>()?,
        "modifier": m.getattr("name")?.extract::<String>()?,
        "type": m.getattr("type")?.extract::<String>()?,
    }))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ModAdd {
    /// Single mode: object name. Required unless `modifiers` is given.
    pub name: Option<String>,
    /// Modifier type id, UPPERCASE, exactly as Blender's `Modifier.type` enum
    /// spells it. Generate/deform/physics ids on 5.2 include SUBSURF, MIRROR,
    /// ARRAY, BEVEL, BOOLEAN, SOLIDIFY, DECIMATE, REMESH, WELD, WIREFRAME,
    /// SCREW, SKIN, TRIANGULATE, MASK, BUILD, EDGE_SPLIT, MULTIRES, NODES
    /// (geometry nodes), ARMATURE, CAST, CURVE, DISPLACE, HOOK, LATTICE,
    /// SHRINKWRAP, SIMPLE_DEFORM, SMOOTH, CORRECTIVE_SMOOTH, LAPLACIANSMOOTH,
    /// LAPLACIANDEFORM, SURFACE_DEFORM, WARP, WAVE, MESH_DEFORM, CLOTH,
    /// COLLISION, SOFT_BODY, FLUID, OCEAN, EXPLODE, PARTICLE_SYSTEM,
    /// PARTICLE_INSTANCE, DYNAMIC_PAINT, plus the GREASE_PENCIL_* family.
    /// Required unless `modifiers` is given.
    pub r#type: Option<String>,
    /// Name for the new modifier. Blender appends .001 on a clash, so read the
    /// returned `modifier` back. Default: the type id.
    pub mod_name: Option<String>,
    /// Flat JSON map of RNA property name -> value, applied right after
    /// creation (e.g. {"levels": 2} on SUBSURF, {"segments": 3, "width": 0.02}
    /// on BEVEL). Keys the modifier type does not have are skipped silently;
    /// numbers, booleans, strings and numeric arrays are supported. Use
    /// mod_list / the Blender API for the exact names.
    pub properties: Option<Value>,
    /// Batch mode: [{name, type, mod_name?, properties?}, ...] — one modifier
    /// per entry, possibly on different objects. Takes precedence over the flat
    /// fields. A failed entry becomes an `{"error": ...}` item in `results`
    /// instead of failing the call.
    pub modifiers: Option<Vec<ModAddEntry>>,
}

crate::tool!(
    "mod_add",
    "Add a modifier to an object (SUBSURF, MIRROR, ARRAY, BEVEL, BOOLEAN, NODES, \
     ...) and optionally set its properties in the same call. Single: {name, \
     type}. Batch: {modifiers: [...]}. To change an existing modifier use \
     mod_update, to bake it into the mesh mod_apply, to drop it mod_delete; \
     util_boolean is a shortcut for the BOOLEAN case.",
    ModAdd,
    |py: Python<'_>, p: ModAdd| {
        if let Some(entries) = p.modifiers {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for e in entries {
                match mod_add_one(
                    py,
                    e.name.as_str(),
                    e.r#type.as_str(),
                    e.mod_name.as_deref(),
                    e.properties.as_ref(),
                ) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    // Per-item failure inside a batch is data, not a call failure.
                    Err(err) => results.push(json!({ "name": e.name, "error": err.to_string() })),
                }
            }
            return Ok(json!({ "added": ok, "results": results }));
        }
        let name = p
            .name
            .ok_or_else(|| PyValueError::new_err("missing name"))?;
        let ty = p
            .r#type
            .ok_or_else(|| PyValueError::new_err("missing type"))?;
        mod_add_one(py, &name, &ty, p.mod_name.as_deref(), p.properties.as_ref())
    }
);

/// One entry of `mod_apply`'s batch `applications` list.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct ModApplyEntry {
    /// Object that owns the modifier.
    pub name: String,
    /// Modifier name to apply, as `mod_list` reports it.
    pub modifier: String,
}

fn mod_apply_one(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    data: &Bound<'_, PyAny>,
    name: &str,
    mod_name: &str,
) -> PyResult<Value> {
    let obj = data.getattr("objects")?.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {name}")));
    }
    let vl = bpy.getattr("context")?.getattr("view_layer")?;
    vl.getattr("objects")?.setattr("active", &obj)?;
    let ops = bpy.getattr("ops")?;
    let kwargs = PyDict::new(py);
    kwargs.set_item("modifier", mod_name)?;
    // `modifier_apply` itself failing means Blender rejected the operation (e.g.
    // wrong mode, modifier order) — the object and modifier both existed, so this
    // is a runtime failure, not a bad-input one.
    let result = ops
        .getattr("object")?
        .call_method("modifier_apply", (), Some(&kwargs))
        .map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "failed to apply modifier {mod_name} on {name}: {e}"
            ))
        })?;
    // Verified live: applying a modifier that no longer exists (e.g. already
    // applied/removed) does NOT raise — it returns {'CANCELLED'} silently. The
    // `map_err` above only catches a raised exception, not this case.
    op_ok(&result, &format!("apply modifier {mod_name} on {name}"))?;
    let on: String = obj.getattr("name")?.extract()?;
    Ok(json!({ "object": on, "applied": mod_name }))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ModApply {
    /// Single mode: object name. Required unless `applications` is given.
    pub name: Option<String>,
    /// Name of the modifier to apply, as `mod_list` reports it (the NAME, not
    /// the type). Required in single mode.
    pub modifier: Option<String>,
    /// Batch mode: [{name, modifier}, ...], possibly across several objects.
    /// Takes precedence over the flat fields. A failed entry becomes an
    /// `{"error": ...}` item in `results` instead of failing the call.
    pub applications: Option<Vec<ModApplyEntry>>,
}

crate::tool!(
    "mod_apply",
    "Bake one named modifier into the object's mesh and remove it from the stack — \
     irreversible except through util_undo. Single: {name, modifier}. Batch: \
     {applications: [...]}. mod_delete drops a modifier WITHOUT baking it, \
     mod_update only changes its settings, and util_apply_booleans bakes every \
     BOOLEAN on an object at once.",
    ModApply,
    |py: Python<'_>, p: ModApply| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        if let Some(apps) = p.applications {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for a in apps {
                match mod_apply_one(py, &bpy, &d, &a.name, &a.modifier) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    // Per-item failure inside a batch is data, not a call failure.
                    // Reconstruct the shape mod_apply_one used to embed itself:
                    // object-not-found (PyValueError) vs. a failed apply (PyRuntimeError).
                    Err(e) if e.is_instance_of::<PyValueError>(py) => {
                        results.push(json!({ "name": a.name, "error": "not found" }));
                    }
                    Err(e) => results.push(json!({
                        "object": a.name,
                        "modifier": a.modifier,
                        "error": e.to_string(),
                    })),
                }
            }
            return Ok(json!({ "applied": ok, "results": results }));
        }
        // Single mode: a missing `name` is now a hard error like everything else —
        // the call could not do what it was asked. A missing `modifier` remains a
        // real error too, since there is nothing sensible to apply.
        let name = p.name.as_deref().unwrap_or("");
        let modifier = p
            .modifier
            .ok_or_else(|| PyValueError::new_err("missing modifier"))?;
        mod_apply_one(py, &bpy, &d, name, &modifier)
    }
);

/// One entry of `mod_delete`'s batch `deletions` list.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct ModDeleteEntry {
    /// Object that owns the modifier.
    pub name: String,
    /// Modifier name to remove, as `mod_list` reports it.
    pub modifier: String,
}

fn mod_delete_one(py: Python<'_>, name: &str, mod_name: &str) -> PyResult<Value> {
    let bpy = py.import("bpy")?;
    let d = data(&bpy)?;
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {name}")));
    }
    let mods = obj.getattr("modifiers")?;
    let m = mods.call_method1("get", (mod_name,))?;
    if m.is_none() {
        return Err(PyValueError::new_err(format!(
            "modifier not found: {mod_name} on {name}"
        )));
    }
    mods.call_method1("remove", (m,))?;
    let on: String = obj.getattr("name")?.extract()?;
    Ok(json!({ "object": on, "deleted": mod_name }))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ModDelete {
    /// Single mode: object name. Required unless `deletions` is given.
    pub name: Option<String>,
    /// Name of the modifier to remove, as `mod_list` reports it. Required in
    /// single mode.
    pub modifier: Option<String>,
    /// Batch mode: [{name, modifier}, ...], possibly across several objects.
    /// Takes precedence over the flat fields. A failed entry becomes an
    /// `{"error": ...}` item in `results` instead of failing the call.
    pub deletions: Option<Vec<ModDeleteEntry>>,
}

crate::tool!(
    "mod_delete",
    "Remove a modifier from an object's stack WITHOUT baking it — the object goes \
     back to how it looked before the modifier. Single: {name, modifier}. Batch: \
     {deletions: [...]}. Use mod_apply to bake it into the mesh instead, or \
     mod_update to just change its settings.",
    ModDelete,
    |py: Python<'_>, p: ModDelete| {
        if let Some(dels) = p.deletions {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for d in dels {
                match mod_delete_one(py, &d.name, &d.modifier) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    // Per-item failure inside a batch is data, not a call failure.
                    Err(e) => results.push(json!({
                        "name": d.name,
                        "modifier": d.modifier,
                        "error": e.to_string(),
                    })),
                }
            }
            return Ok(json!({ "deleted": ok, "results": results }));
        }
        mod_delete_one(
            py,
            p.name.as_deref().unwrap_or(""),
            p.modifier.as_deref().unwrap_or(""),
        )
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ModUpdate {
    /// Object that owns the modifier.
    pub name: String,
    /// Name of the modifier to change, as `mod_list` reports it (the NAME, not
    /// the type).
    pub modifier: String,
    /// Flat JSON map of RNA property name -> value (e.g. {"levels": 3} on
    /// SUBSURF, {"thickness": 0.05} on SOLIDIFY). Keys the modifier type does
    /// not have are skipped silently; numbers, booleans, strings and numeric
    /// arrays are supported. Omitted properties keep their current values.
    pub properties: Option<Value>,
}

crate::tool!(
    "mod_update",
    "Change the settings of a modifier that already exists, leaving it live in the \
     stack. mod_add creates one (and can set the same properties at creation), \
     mod_apply bakes it in, mod_delete removes it. Geometry-node inputs are not \
     set here — use geo_modifier_set.",
    ModUpdate,
    |py: Python<'_>, p: ModUpdate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Object not found: {}",
                p.name
            )));
        }
        let mods = obj.getattr("modifiers")?;
        let m = mods.call_method1("get", (p.modifier.as_str(),))?;
        if m.is_none() {
            return Err(PyValueError::new_err(format!(
                "Modifier not found: {}",
                p.modifier
            )));
        }
        apply_properties(&m, p.properties.as_ref())?;
        Ok(json!({
            "object": obj.getattr("name")?.extract::<String>()?,
            "modifier": m.getattr("name")?.extract::<String>()?,
            "type": m.getattr("type")?.extract::<String>()?,
        }))
    }
);

// ============================================================================
// Collections
// ============================================================================

/// `col_list` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct ColList {}

crate::tool!(
    "col_list",
    "Return the scene's collection tree as nested {name, objects, children},      starting at the scene root collection. Takes no arguments. The source of the      collection names col_move_to, col_rename and col_delete take.",
    ColList,
    |py: Python<'_>, _p: ColList| {
        let bpy = py.import("bpy")?;
        let root = scene(&bpy)?.getattr("collection")?;
        Ok(Value::Array(vec![coll_info(&root)?]))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ColCreate {
    /// Name for the new collection. Must not already exist — a clash is an
    /// error, not a rename.
    pub name: String,
    /// Name of an existing collection to nest the new one under. Default: the
    /// scene's root collection.
    pub parent: Option<String>,
}

crate::tool!(
    "col_create",
    "Create an empty collection and link it into the scene, optionally under a      `parent` collection. col_move_to also creates a collection on demand, so use      this only when the collection must exist before anything goes into it.",
    ColCreate,
    |py: Python<'_>, p: ColCreate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let collections = d.getattr("collections")?;
        if !collections
            .call_method1("get", (p.name.as_str(),))?
            .is_none()
        {
            return Err(PyValueError::new_err(format!(
                "Collection already exists: {}",
                p.name
            )));
        }
        let coll = collections.call_method1("new", (p.name.as_str(),))?;
        if let Some(parent) = p.parent {
            let pc = collections.call_method1("get", (parent.as_str(),))?;
            if pc.is_none() {
                return Err(PyValueError::new_err(format!(
                    "Parent collection not found: {parent}"
                )));
            }
            pc.getattr("children")?.call_method1("link", (&coll,))?;
        } else {
            scene(&bpy)?
                .getattr("collection")?
                .getattr("children")?
                .call_method1("link", (&coll,))?;
        }
        let cn: String = coll.getattr("name")?.extract()?;
        Ok(json!({ "name": cn }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ColDelete {
    /// Name of the collection to remove.
    pub name: String,
    /// true: also delete the objects directly inside it from the file. false:
    /// keep the objects (they survive in whatever other collections hold them,
    /// or become unlinked). Default: false.
    pub delete_objects: Option<bool>,
}

crate::tool!(
    "col_delete",
    "Remove a collection, optionally deleting the objects inside it as well.      Child collections are not deleted, only unparented. To keep the collection      and just move objects out, use col_move_to.",
    ColDelete,
    |py: Python<'_>, p: ColDelete| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let collections = d.getattr("collections")?;
        let coll = collections.call_method1("get", (p.name.as_str(),))?;
        if coll.is_none() {
            return Err(PyValueError::new_err(format!(
                "Collection not found: {}",
                p.name
            )));
        }
        let delete_objects = p.delete_objects.unwrap_or(false);
        if delete_objects {
            let objs = coll.getattr("objects")?;
            let objects = d.getattr("objects")?;
            let mut to_remove: Vec<Bound<'_, PyAny>> = Vec::new();
            for o in PyIterator::from_object(&objs)? {
                to_remove.push(o?);
            }
            for o in to_remove {
                let kwargs = PyDict::new(py);
                kwargs.set_item("do_unlink", true)?;
                objects.call_method("remove", (&o,), Some(&kwargs))?;
            }
        }
        collections.call_method1("remove", (&coll,))?;
        Ok(json!({
            "deleted": p.name,
            "objects_deleted": delete_objects,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ColRename {
    /// Current name of the collection.
    pub name: String,
    /// New name. Blender appends .001 etc. on a clash, so read the returned
    /// `new_name` back.
    pub new_name: String,
}

crate::tool!(
    "col_rename",
    "Rename a collection in place, keeping its contents and its position in the      tree. Returns the old and the actually-assigned new name.",
    ColRename,
    |py: Python<'_>, p: ColRename| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let coll = d
            .getattr("collections")?
            .call_method1("get", (p.name.as_str(),))?;
        if coll.is_none() {
            return Err(PyValueError::new_err(format!(
                "Collection not found: {}",
                p.name
            )));
        }
        let old_name: String = coll.getattr("name")?.extract()?;
        coll.setattr("name", p.new_name.as_str())?;
        let nn: String = coll.getattr("name")?.extract()?;
        Ok(json!({ "old_name": old_name, "new_name": nn }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ColMoveTo {
    /// Single mode: name of one object. Ignored when `objects` is given.
    pub object: Option<String>,
    /// Batch mode: object names to move in one call. Takes precedence over
    /// `object`. A missing name becomes an `{"error": "not found"}` item in
    /// `results`.
    pub objects: Option<Vec<String>>,
    /// Destination collection name. Created and linked to the scene root if it
    /// does not exist yet.
    pub collection: String,
}

crate::tool!(
    "col_move_to",
    "MOVE object(s) into a collection: they are unlinked from every collection      they were in first, so this is a move, not a copy. The destination is created      if missing. Single: {object}. Batch: {objects: [...]}. See col_list for the      current tree and col_create to make an empty collection up front.",
    ColMoveTo,
    |py: Python<'_>, p: ColMoveTo| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        let collections = d.getattr("collections")?;
        let mut coll = collections.call_method1("get", (p.collection.as_str(),))?;
        if coll.is_none() {
            coll = collections.call_method1("new", (p.collection.as_str(),))?;
            scene(&bpy)?
                .getattr("collection")?
                .getattr("children")?
                .call_method1("link", (&coll,))?;
        }

        // Batch `objects` wins over single `object`, matching the original
        // single-mode/batch-mode precedence used across every other tool here.
        let obj_names: Vec<String> = p
            .objects
            .or_else(|| p.object.map(|o| vec![o]))
            .unwrap_or_default();

        let objects = d.getattr("objects")?;
        let mut results = Vec::new();
        for obj_name in obj_names {
            let obj = objects.call_method1("get", (obj_name.as_str(),))?;
            if obj.is_none() {
                results.push(json!({ "object": obj_name, "error": "not found" }));
                continue;
            }
            let uc = obj.getattr("users_collection")?;
            for c in PyIterator::from_object(&uc)? {
                let c = c?;
                c.getattr("objects")?.call_method1("unlink", (&obj,))?;
            }
            coll.getattr("objects")?.call_method1("link", (&obj,))?;
            let cn: String = coll.getattr("name")?.extract()?;
            results.push(json!({ "object": obj_name, "collection": cn }));
        }

        // Single-object call returns the bare result, not a batch envelope — but a
        // bare result that is itself `{"error": ...}` is a call failure, not data,
        // and must still surface as `Err`.
        if results.len() == 1 {
            let one = results.remove(0);
            if let Some(err) = one.get("error").and_then(Value::as_str) {
                let obj_name = one
                    .get("object")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                return Err(PyValueError::new_err(format!(
                    "object not found: {obj_name} ({err})"
                )));
            }
            return Ok(one);
        }
        let moved = results.iter().filter(|r| r.get("error").is_none()).count();
        let cn: String = coll.getattr("name")?.extract()?;
        Ok(json!({ "moved": moved, "collection": cn, "results": results }))
    }
);
