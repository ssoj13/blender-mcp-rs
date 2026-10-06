//! Animation tools: keyframes, fcurves, frame, clear. Registered automatically —
//! nothing outside this file names them.
//!
//! `anim_set_keyframe` / `anim_delete_keyframe` keep a JSON-`Value`-based core: params
//! are re-serialized to `Value` right after typed deserialization so the
//! single-vs-batch dispatch and the arbitrary-shaped `value` field work.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyIterator, PyTuple};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::bpy::{data, py_len};

// ---------------------------------------------------------------------------
// anim_set_keyframe
// ---------------------------------------------------------------------------

/// One entry of `anim_set_keyframe`'s batch mode.
#[derive(Deserialize, Serialize, schemars::JsonSchema)]
pub struct KeyframeSet {
    /// Object name, exactly as it appears in `bpy.data.objects` (see obj_list).
    pub name: String,
    /// Frame to key, in whole frames. Default: the scene's current frame
    /// (anim_set_frame moves it; scene_set owns the persistent range).
    pub frame: Option<i32>,
    /// Shorthand data path: `location`, `rotation_euler` (RADIANS), `scale`,
    /// `hide_viewport`, ... Ignored when `data_path` is given. Default: `location`.
    pub property: Option<String>,
    /// Full RNA data path, overriding `property` - e.g. `modifiers["Subdiv"].levels`,
    /// `active_material.diffuse_color`, `["hp"]` for a custom property (prop_set).
    pub data_path: Option<String>,
    /// Array component to key: 0=X, 1=Y, 2=Z (3=W on a quaternion or RGBA colour).
    /// Default: every component of the property.
    pub index: Option<i32>,
    /// Value written onto the property before the key is inserted, so the key
    /// captures it: a number, or `[x, y, z]` when `index` is omitted on a vector.
    /// Rotations are in RADIANS. Default: key whatever the property already holds.
    pub value: Option<Value>,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
pub struct AnimSetKeyframe {
    /// Single mode: object name, as in `bpy.data.objects` (see obj_list). Ignored
    /// when `keyframes` is given.
    pub name: Option<String>,
    /// Shorthand data path: `location`, `rotation_euler` (RADIANS), `scale`,
    /// `hide_viewport`, ... Ignored when `data_path` is given. Default: `location`.
    pub property: Option<String>,
    /// Full RNA data path, overriding `property` - e.g. `modifiers["Subdiv"].levels`,
    /// `active_material.diffuse_color`, `["hp"]` for a custom property (prop_set).
    pub data_path: Option<String>,
    /// Array component to key: 0=X, 1=Y, 2=Z (3=W on a quaternion or RGBA colour).
    /// Default: every component of the property.
    pub index: Option<i32>,
    /// Value written onto the property before the key is inserted, so the key
    /// captures it: a number, or `[x, y, z]` when `index` is omitted on a vector.
    /// Rotations are in RADIANS. Default: key whatever the property already holds.
    pub value: Option<Value>,
    /// Frame to key, in whole frames. Default: the scene's current frame
    /// (anim_set_frame moves it; scene_set owns the persistent range).
    pub frame: Option<i32>,
    /// Batch mode: `[{name, frame?, property?, data_path?, index?, value?}, ...]`.
    /// When present it replaces the single-mode fields above, and a per-item failure
    /// is reported inside `results` rather than failing the whole call.
    pub keyframes: Option<Vec<KeyframeSet>>,
}

fn value_to_py<'py>(py: Python<'py>, v: &Value) -> PyResult<Bound<'py, PyAny>> {
    let s = serde_json::to_string(v)
        .map_err(|e| PyValueError::new_err(format!("JSON encode value: {e}")))?;
    let json = py.import("json")?;
    json.call_method1("loads", (s,))
}

/// Writes `value` onto `obj` at `data_path` before the keyframe is inserted, so the
/// keyframe captures the intended value rather than whatever was there before.
fn apply_value(
    py: Python<'_>,
    obj: &Bound<'_, PyAny>,
    data_path: &str,
    index: Option<i32>,
    value: &Value,
) -> PyResult<bool> {
    if matches!(data_path, "location" | "rotation_euler" | "scale") {
        // No index: the caller means the whole vector, so `value` must be [x, y, z].
        let Some(index) = index else {
            if let Some(arr) = value.as_array() {
                let nums: Vec<f64> = arr.iter().filter_map(|x| x.as_f64()).collect();
                if nums.len() >= 3 {
                    let t = PyTuple::new(py, [nums[0], nums[1], nums[2]])?;
                    obj.setattr(data_path, t)?;
                    return Ok(true);
                }
            }
            return Ok(false);
        };
        let seq = obj.getattr(data_path)?;
        seq.set_item(index as isize, value_to_py(py, value)?)?;
        return Ok(true);
    }

    // `modifiers["Name"].prop` style paths, as produced by Blender's own "Copy Data Path".
    static MOD_RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let re = MOD_RE.get_or_init(|| {
        Regex::new(r#"modifiers\["([^"]+)"\]\.([A-Za-z_][A-Za-z0-9_]*)"#).expect("regex")
    });
    if let Some(caps) = re.captures(data_path) {
        let mname = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let prop = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let mods = obj.getattr("modifiers")?;
        let m = mods.call_method1("get", (mname,))?;
        if !m.is_none() && m.hasattr(prop)? {
            m.setattr(prop, value_to_py(py, value)?)?;
            return Ok(true);
        }
        return Ok(false);
    }

    if let Some(rest) = data_path.strip_prefix("active_material.") {
        let mat = obj.getattr("active_material")?;
        if mat.is_none() {
            return Ok(false);
        }
        if mat.hasattr(rest)? {
            mat.setattr(rest, value_to_py(py, value)?)?;
            return Ok(true);
        }
        return Ok(false);
    }

    if !data_path.contains('.') && obj.hasattr(data_path)? {
        obj.setattr(data_path, value_to_py(py, value)?)?;
        return Ok(true);
    }

    Ok(false)
}

fn set_one_keyframe(py: Python<'_>, params: &Value) -> PyResult<Value> {
    let bpy = py.import("bpy")?;
    let d = data(&bpy)?;
    let name = params
        .get("name")
        .and_then(|n| n.as_str())
        .ok_or_else(|| PyValueError::new_err("missing name"))?;
    let objects = d.getattr("objects")?;
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {name}")));
    }

    let sc = bpy.getattr("context")?.getattr("scene")?;
    let frame = if let Some(f) = params.get("frame").and_then(|x| x.as_i64()) {
        f as i32
    } else {
        sc.getattr("frame_current")?.extract()?
    };
    let data_path = params
        .get("data_path")
        .and_then(|x| x.as_str())
        .or_else(|| params.get("property").and_then(|x| x.as_str()))
        .unwrap_or("location");
    let index = params
        .get("index")
        .and_then(|x| x.as_i64())
        .map(|x| x as i32);

    let value = params.get("value");
    let mut value_applied = false;
    if let Some(v) = value {
        value_applied = apply_value(py, &obj, data_path, index, v)?;
    }

    let kwargs = PyDict::new(py);
    kwargs.set_item("data_path", data_path)?;
    kwargs.set_item("frame", frame)?;
    if let Some(ix) = index {
        kwargs.set_item("index", ix)?;
    }

    // `keyframe_insert` itself failing means Blender rejected the operation (e.g. an
    // invalid data path) — the object existed, so this is a runtime failure, not a
    // bad-input one.
    obj.call_method("keyframe_insert", (), Some(&kwargs))
        .map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "keyframe_insert failed for {name} at {data_path}: {e}"
            ))
        })?;

    let on: String = obj.getattr("name")?.extract()?;
    Ok(json!({
        "object": on,
        "frame": frame,
        "property": data_path,
        "index": index,
        "value_applied": value_applied,
    }))
}

crate::tool!(
    "anim_set_keyframe",
    "Insert a keyframe on an object property at a frame, optionally writing `value` \
     first so the key captures it. Rotations are in RADIANS. One object via `name`, \
     or many via `keyframes` (per-item errors land in `results`). Read the keys back \
     with anim_get_keyframes, delete one with anim_delete_keyframe.",
    AnimSetKeyframe,
    |py: Python<'_>, p: AnimSetKeyframe| {
        let params = serde_json::to_value(&p)
            .map_err(|e| PyValueError::new_err(format!("re-encode params: {e}")))?;
        if let Some(arr) = params.get("keyframes").and_then(|k| k.as_array()) {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for k in arr {
                match set_one_keyframe(py, k) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    // Per-item failure inside a batch is data, not a call failure.
                    Err(e) => {
                        let name = k.get("name").and_then(|n| n.as_str()).unwrap_or_default();
                        results.push(json!({ "name": name, "error": e.to_string() }));
                    }
                }
            }
            return Ok(json!({ "set": ok, "results": results }));
        }
        set_one_keyframe(py, &params)
    }
);

// ---------------------------------------------------------------------------
// anim_delete_keyframe
// ---------------------------------------------------------------------------

/// One entry of `anim_delete_keyframe`'s batch mode.
#[derive(Deserialize, Serialize, schemars::JsonSchema)]
pub struct KeyframeDel {
    /// Object name, as in `bpy.data.objects` (see obj_list).
    pub name: String,
    /// Frame whose key is removed, in whole frames. Default: the scene's current
    /// frame (anim_set_frame).
    pub frame: Option<i32>,
    /// Data path the key sits on: `location`, `rotation_euler`, `scale`, ...
    /// Default: `location`. Every array component at that frame is removed.
    pub property: Option<String>,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
pub struct AnimDeleteKeyframe {
    /// Single mode: object name, as in `bpy.data.objects` (see obj_list). Ignored
    /// when `keyframes` is given.
    pub name: Option<String>,
    /// Data path the key sits on: `location`, `rotation_euler`, `scale`, ...
    /// Default: `location`. Every array component at that frame is removed.
    pub property: Option<String>,
    /// Frame whose key is removed, in whole frames. Default: the scene's current
    /// frame (anim_set_frame).
    pub frame: Option<i32>,
    /// Batch mode: `[{name, frame?, property?}, ...]`. When present it replaces the
    /// single-mode fields above, and a per-item failure is reported inside `results`
    /// rather than failing the whole call.
    pub keyframes: Option<Vec<KeyframeDel>>,
}

fn delete_one_kf(py: Python<'_>, params: &Value) -> PyResult<Value> {
    let bpy = py.import("bpy")?;
    let d = data(&bpy)?;
    let name = params
        .get("name")
        .and_then(|n| n.as_str())
        .ok_or_else(|| PyValueError::new_err("missing name"))?;
    let objects = d.getattr("objects")?;
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {name}")));
    }
    let sc = bpy.getattr("context")?.getattr("scene")?;
    let frame = if let Some(f) = params.get("frame").and_then(|x| x.as_i64()) {
        f as i32
    } else {
        sc.getattr("frame_current")?.extract()?
    };
    let data_path = params
        .get("property")
        .and_then(|x| x.as_str())
        .unwrap_or("location");
    let kwargs = PyDict::new(py);
    kwargs.set_item("data_path", data_path)?;
    kwargs.set_item("frame", frame)?;
    // `keyframe_delete` itself failing (e.g. no keyframe at that frame/path) is a
    // runtime failure — the object existed, so the request just could not be
    // carried out as asked.
    obj.call_method("keyframe_delete", (), Some(&kwargs))
        .map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!(
                "keyframe_delete failed for {name} at {data_path} frame {frame}: {e}"
            ))
        })?;
    let on: String = obj.getattr("name")?.extract()?;
    Ok(json!({
        "object": on,
        "frame": frame,
        "property": data_path,
    }))
}

crate::tool!(
    "anim_delete_keyframe",
    "Remove the keyframe at one frame from one property; the F-Curve itself survives \
     while other keys remain. One object via `name`, or many via `keyframes`. \
     Failing when there is no key at that frame is intended. To wipe every action, \
     NLA track and driver at once use anim_clear.",
    AnimDeleteKeyframe,
    |py: Python<'_>, p: AnimDeleteKeyframe| {
        let params = serde_json::to_value(&p)
            .map_err(|e| PyValueError::new_err(format!("re-encode params: {e}")))?;
        if let Some(arr) = params.get("keyframes").and_then(|k| k.as_array()) {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for k in arr {
                match delete_one_kf(py, k) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    // Per-item failure inside a batch is data, not a call failure.
                    Err(e) => {
                        let name = k.get("name").and_then(|n| n.as_str()).unwrap_or_default();
                        results.push(json!({ "name": name, "error": e.to_string() }));
                    }
                }
            }
            return Ok(json!({ "deleted": ok, "results": results }));
        }
        delete_one_kf(py, &params)
    }
);

// ---------------------------------------------------------------------------
// anim_get_keyframes
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AnimGetKeyframes {
    /// Object name, as in `bpy.data.objects` (see obj_list).
    pub name: String,
}

crate::tool!(
    "anim_get_keyframes",
    "Every keyframe of an object's ACTIVE action, grouped by data path, each with \
     frame, value, array index and interpolation (CONSTANT, LINEAR, BEZIER, SINE, \
     QUAD, CUBIC, QUART, QUINT, EXPO, CIRC, BACK, BOUNCE, ELASTIC on Blender 5.2). \
     An un-animated object returns an empty map, not an error. NLA strips \
     (nla_list_tracks) and drivers (drv_*) are not included; for a per-curve summary \
     without the individual keys use anim_get_data.",
    AnimGetKeyframes,
    |py: Python<'_>, p: AnimGetKeyframes| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d.getattr("objects")?.call_method1("get", (&p.name,))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Object not found: {}",
                p.name
            )));
        }
        let ad = obj.getattr("animation_data")?;
        if ad.is_none() {
            let on: String = obj.getattr("name")?.extract()?;
            return Ok(json!({ "object": on, "keyframes": serde_json::Map::new() }));
        }
        let action = ad.getattr("action")?;
        if action.is_none() {
            let on: String = obj.getattr("name")?.extract()?;
            return Ok(json!({ "object": on, "keyframes": serde_json::Map::new() }));
        }
        let mut keyframes: serde_json::Map<String, Value> = serde_json::Map::new();
        for fc in super::bpy::action_fcurves(&action)? {
            let path: String = fc.getattr("data_path")?.extract()?;
            let arr_idx: i32 = fc.getattr("array_index")?.extract()?;
            let kps = fc.getattr("keyframe_points")?;
            let kit = PyIterator::from_object(&kps)?;
            let entry = keyframes.entry(path.clone()).or_insert_with(|| json!([]));
            let arr = entry
                .as_array_mut()
                .ok_or_else(|| PyValueError::new_err("keyframes entry is not an array"))?;
            for kp in kit {
                let kp = kp?;
                let co = kp.getattr("co")?;
                let frame = co.get_item(0)?.extract::<f64>()?;
                let val = co.get_item(1)?.extract::<f64>()?;
                let interp: String = kp.getattr("interpolation")?.extract()?;
                arr.push(json!({
                    "frame": frame,
                    "value": val,
                    "interpolation": interp,
                    "index": arr_idx,
                }));
            }
        }
        let on: String = obj.getattr("name")?.extract()?;
        Ok(json!({ "object": on, "keyframes": Value::Object(keyframes) }))
    }
);

// ---------------------------------------------------------------------------
// anim_set_frame
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AnimSetFrame {
    /// Frame to jump to, in whole frames. Required. May sit outside the scene's
    /// frame_start..frame_end range; negative frames are allowed.
    pub frame: i32,
}

crate::tool!(
    "anim_set_frame",
    "Move the scene playhead to `frame` (`scene.frame_set`) and re-evaluate the \
     depsgraph there. This is the CURRENT frame only - a transient cursor, not a \
     setting: the persistent frame_start / frame_end / fps that renders and bakes \
     read belong to scene_set (and are reported by scene_get_info).",
    AnimSetFrame,
    |py: Python<'_>, p: AnimSetFrame| {
        let bpy = py.import("bpy")?;
        bpy.getattr("context")?
            .getattr("scene")?
            .call_method1("frame_set", (p.frame,))?;
        Ok(json!({ "frame": p.frame }))
    }
);

// ---------------------------------------------------------------------------
// anim_get_data
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AnimGetData {
    /// Object name, as in `bpy.data.objects` (see obj_list).
    pub name: String,
}

crate::tool!(
    "anim_get_data",
    "Summary of an object's ACTIVE action: its name plus one entry per F-Curve \
     (data_path, array_index, keyframe_count, frame range). An un-animated object \
     returns has_animation:false, not an error. For the individual keys use \
     anim_get_keyframes; for NLA strips use nla_list_tracks.",
    AnimGetData,
    |py: Python<'_>, p: AnimGetData| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d.getattr("objects")?.call_method1("get", (&p.name,))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Object not found: {}",
                p.name
            )));
        }
        let ad = obj.getattr("animation_data")?;
        if ad.is_none() {
            let on: String = obj.getattr("name")?.extract()?;
            return Ok(json!({
                "object": on,
                "has_animation": false,
            }));
        }
        let action = ad.getattr("action")?;
        let action_name: Value = if action.is_none() {
            Value::Null
        } else {
            Value::String(action.getattr("name")?.extract::<String>()?)
        };
        let mut fcurves = Vec::new();
        if !action.is_none() {
            for fc in super::bpy::action_fcurves(&action)? {
                let n_kp = py_len(py, &fc.getattr("keyframe_points")?)?;
                let range_val = if n_kp > 0 {
                    let r = fc.call_method0("range")?;
                    let a = r.get_item(0)?.extract::<f64>()?;
                    let b = r.get_item(1)?.extract::<f64>()?;
                    Some(json!([a, b]))
                } else {
                    None
                };
                fcurves.push(json!({
                    "data_path": fc.getattr("data_path")?.extract::<String>()?,
                    "array_index": fc.getattr("array_index")?.extract::<i32>()?,
                    "keyframe_count": n_kp,
                    "range": range_val,
                }));
            }
        }
        let on: String = obj.getattr("name")?.extract()?;
        Ok(json!({
            "object": on,
            "has_animation": true,
            "action": action_name,
            "fcurves": fcurves,
        }))
    }
);

// ---------------------------------------------------------------------------
// anim_clear
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AnimClear {
    /// Single mode: object name, as in `bpy.data.objects` (see obj_list). Ignored
    /// when `names` is given.
    pub name: Option<String>,
    /// Batch mode: object names. A name that does not exist is reported inside
    /// `results` rather than failing the whole call.
    pub names: Option<Vec<String>>,
}

crate::tool!(
    "anim_clear",
    "Remove ALL animation from an object: `animation_data_clear()` drops the active \
     action, every NLA track (nla_push) and every driver (drv_add / drv_create) in \
     one step. One object via `name`, many via `names`. To delete a single key use \
     anim_delete_keyframe; to unlink one driver use drv_remove.",
    AnimClear,
    |py: Python<'_>, p: AnimClear| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let objects = d.getattr("objects")?;

        let names: Vec<String> = if let Some(v) = p.names {
            v
        } else if let Some(n) = p.name {
            vec![n]
        } else {
            Vec::new()
        };

        let mut results = Vec::new();
        for name in names {
            let obj = objects.call_method1("get", (&name,))?;
            if obj.is_none() {
                results.push(json!({ "name": name, "error": "not found" }));
                continue;
            }
            obj.call_method0("animation_data_clear")?;
            let on: String = obj.getattr("name")?.extract()?;
            results.push(json!({ "object": on, "cleared": true }));
        }

        // Single-object call returns the bare result, not a batch envelope — but a
        // bare result that is itself `{"error": ...}` is a call failure, not data,
        // and must still surface as `Err`.
        if results.len() == 1 {
            let one = results.remove(0);
            if one.get("error").is_some() {
                let name = one.get("name").and_then(Value::as_str).unwrap_or_default();
                return Err(PyValueError::new_err(format!("object not found: {name}")));
            }
            return Ok(one);
        }
        let cleared = results.iter().filter(|r| r.get("error").is_none()).count();
        Ok(json!({ "cleared": cleared, "results": results }))
    }
);
