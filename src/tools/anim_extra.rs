//! ANIMATION-EXTRA / NLA / Grease Pencil tools. Registered automatically —
//! nothing outside this file names them.
//!
//! ## Blender 5 traps fixed in this file
//!
//! **F-Curves.** `anim_add_fcurve_noise/_cycles/_envelope` all touch F-Curves on an
//! object's action. The legacy Python handlers read/write through
//! `obj.animation_data.action.fcurves`, which Blender 5.0 removed outright (curves
//! moved into `action.layers[].strips[].channelbag(slot).fcurves` with 4.4's
//! slotted actions). Reads go through [`super::bpy::action_fcurves`]; the one case
//! that also needs to *create* a curve (`anim_add_fcurve_noise`, which auto-creates
//! the action+curve if missing) goes through [`ensure_fcurve`], which builds the
//! slot/layer/strip/channelbag chain by hand when `action.layers` exists and falls
//! back to the flat `action.fcurves.new(...)` on Blender <= 4.3.
//!
//! **Grease Pencil.** Blender 4.3 replaced the old "GPencil" data-block with Grease
//! Pencil v3; Blender 5.0 finished the rename (`bpy.data.grease_pencils_v3` ->
//! `bpy.data.grease_pencils`, `GreasePencilv3` -> `GreasePencil`). Concretely, versus
//! the legacy API the Python handlers were written against:
//! - object type string is `GREASEPENCIL`, not `GPENCIL`.
//! - a frame no longer owns strokes directly; `frame.drawing.strokes` is the
//!   documented compatibility shim for the old `frame.strokes` API for *reading*.
//! - **writing a stroke is a different API altogether**, not just a renamed one:
//!   `drawing.strokes` is a read-only slice (no `.new()`); a stroke is allocated
//!   with `drawing.add_strokes([point_count])`. `GreasePencilStroke` has no
//!   `line_width`; each point (`GreasePencilStrokePoint`) has `.position` (was
//!   `.co`), `.radius` (replaces the old `line_width` * `.pressure` combination)
//!   and `.opacity` (was `.strength`) — verified live against Blender 5.0, see
//!   `gp_draw_stroke`.
//! - Grease Pencil modifiers were merged into the regular modifier stack
//!   (`obj.modifiers`, not `obj.grease_pencil_modifiers`), with `type` enum values
//!   renamed to a `GREASE_PENCIL_*` scheme (`GP_NOISE` -> `GREASE_PENCIL_NOISE`, …).
//! - `GreasePencilLayer` (a `GreasePencilTreeNode`, the same tree-node base every
//!   other outliner-tree type uses) exposes `.name` like everything else; the old
//!   `.info` display-name alias was a legacy-GPencil-only quirk and is gone.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{action_fcurves, data, op_ok, py_len, scene};

/// Read every element of a Python float sequence (e.g. a `Vector`) into a `Vec`.
fn py_vec(seq: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    let mut v = Vec::new();
    for x in seq.try_iter()? {
        v.push(x?.extract::<f64>()?);
    }
    Ok(v)
}

// ---------------------------------------------------------------------------
// Motion Paths
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AnimShowMotionPath {
    /// Object name, as in `bpy.data.objects` (see obj_list).
    pub name: String,
    /// Pose bone to trace instead of the object itself. Honoured only when the
    /// object's type is ARMATURE; the tool then enters Pose mode, selects that bone
    /// alone and restores Object mode. Default: trace the object.
    pub bone: Option<String>,
    /// First frame of the path, in whole frames. Default: the scene's `frame_start`
    /// (scene_set).
    pub frame_start: Option<i32>,
    /// Last frame of the path, in whole frames. Default: the scene's `frame_end`
    /// (scene_set). Blender stores the computed `frame_end` one past this value.
    pub frame_end: Option<i32>,
    /// Sample the path every N frames; 1 samples every frame. Default: 1.
    pub frame_step: Option<i32>,
}

crate::tool!(
    "anim_show_motion_path",
    "Calculate and display the motion path of an object, or of one pose bone when \
     `bone` is given on an ARMATURE. The frame range defaults to the scene range \
     owned by scene_set. Recompute after keys move with anim_update_motion_paths; \
     remove with anim_clear_motion_paths.",
    AnimShowMotionPath,
    |py: Python<'_>, p: AnimShowMotionPath| {
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

        let sc = scene(&bpy)?;
        let frame_start = match p.frame_start {
            Some(f) => f,
            None => sc.getattr("frame_start")?.extract()?,
        };
        let frame_end = match p.frame_end {
            Some(f) => f,
            None => sc.getattr("frame_end")?.extract()?,
        };
        let frame_step = p.frame_step.unwrap_or(1);

        bpy.getattr("context")?
            .getattr("view_layer")?
            .getattr("objects")?
            .setattr("active", &obj)?;
        let ops = bpy.getattr("ops")?;

        let obj_type: String = obj.getattr("type")?.extract()?;
        if let Some(bone_name) = p.bone.as_deref().filter(|_| obj_type == "ARMATURE") {
            // mode_set is not checked: verified live against Blender 5.0 that
            // switching to a mode already active (or into POSE with a valid
            // armature active, as here) always returns {'FINISHED'} — an
            // incompatible mode/object pairing raises a TypeError instead of
            // silently cancelling, which `?` below already propagates.
            let mk = PyDict::new(py);
            mk.set_item("mode", "POSE")?;
            ops.getattr("object")?
                .call_method("mode_set", (), Some(&mk))?;

            for pb in obj.getattr("pose")?.getattr("bones")?.try_iter()? {
                let pb = pb?;
                let n: String = pb.getattr("name")?.extract()?;
                pb.getattr("bone")?.setattr("select", n == bone_name)?;
            }

            let mp = obj
                .getattr("pose")?
                .getattr("animation_visualization")?
                .getattr("motion_path")?;
            mp.setattr("frame_start", frame_start)?;
            mp.setattr("frame_end", frame_end)?;
            mp.setattr("frame_step", frame_step)?;

            // `paths_calculate` takes no start/end frame: its RNA is
            // `display_type` + `range` (verified live on Blender 5). The explicit
            // range lives on the motion_path settings above, and `range=MANUAL` is
            // what tells the operator to use it. Passing start_frame/end_frame
            // raised a TypeError, so this tool never worked at all.
            let ck = PyDict::new(py);
            ck.set_item("display_type", "RANGE")?;
            ck.set_item("range", "MANUAL")?;
            let result = ops
                .getattr("pose")?
                .call_method("paths_calculate", (), Some(&ck))?;
            op_ok(
                &result,
                &format!("calculate motion path for bone `{bone_name}`"),
            )?;

            let mk2 = PyDict::new(py);
            mk2.set_item("mode", "OBJECT")?;
            ops.getattr("object")?
                .call_method("mode_set", (), Some(&mk2))?;

            let on: String = obj.getattr("name")?.extract()?;
            return Ok(
                json!({ "object": on, "bone": bone_name, "frames": [frame_start, frame_end] }),
            );
        }

        obj.call_method1("select_set", (true,))?;
        let mp = obj
            .getattr("animation_visualization")?
            .getattr("motion_path")?;
        mp.setattr("frame_start", frame_start)?;
        mp.setattr("frame_end", frame_end)?;
        mp.setattr("frame_step", frame_step)?;

        // Same as the bone branch: the range is on the motion_path settings, and
        // `range=MANUAL` is what makes the operator honour it.
        let ck = PyDict::new(py);
        ck.set_item("display_type", "RANGE")?;
        ck.set_item("range", "MANUAL")?;
        let result = ops
            .getattr("object")?
            .call_method("paths_calculate", (), Some(&ck))?;
        op_ok(&result, "calculate motion path")?;

        let on: String = obj.getattr("name")?.extract()?;
        Ok(json!({ "object": on, "frames": [frame_start, frame_end] }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AnimUpdateMotionPaths {
    /// Object names to select before recalculating. A name that does not exist is
    /// skipped. Default: leave the selection alone and update whatever is already
    /// selected.
    pub names: Option<Vec<String>>,
}

crate::tool!(
    "anim_update_motion_paths",
    "Recalculate motion paths that already exist, after keyframes have moved. It \
     does not create one - anim_show_motion_path does that - and it keeps each \
     path's own frame range.",
    AnimUpdateMotionPaths,
    |py: Python<'_>, p: AnimUpdateMotionPaths| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        if let Some(names) = p.names {
            let objects = d.getattr("objects")?;
            for name in names {
                let obj = objects.call_method1("get", (name.as_str(),))?;
                if !obj.is_none() {
                    obj.call_method1("select_set", (true,))?;
                }
            }
        }
        let result = bpy
            .getattr("ops")?
            .getattr("object")?
            .call_method0("paths_update")?;
        op_ok(&result, "update motion paths")?;
        Ok(json!({ "updated": true }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AnimClearMotionPaths {
    /// Single mode: object name. Used when `names` is absent or empty.
    pub name: Option<String>,
    /// Batch mode: object names. A name that does not exist is skipped silently;
    /// the names actually cleared come back in `cleared`.
    pub names: Option<Vec<String>>,
}

crate::tool!(
    "anim_clear_motion_paths",
    "Delete the displayed motion paths of one or more objects (created by \
     anim_show_motion_path). It removes only the path visualisation - the keyframes \
     themselves are untouched (that is anim_delete_keyframe / anim_clear).",
    AnimClearMotionPaths,
    |py: Python<'_>, p: AnimClearMotionPaths| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let objects = d.getattr("objects")?;
        let view_layer = bpy.getattr("context")?.getattr("view_layer")?;
        let ops = bpy.getattr("ops")?;

        // Mirrors the Python `p.get("names") or [p.get("name")]`: an explicit but
        // empty `names` array falls back to the single-name form too.
        let names: Vec<String> = match p.names {
            Some(v) if !v.is_empty() => v,
            _ => p.name.into_iter().collect(),
        };

        let mut cleared = Vec::new();
        for name in names {
            let obj = objects.call_method1("get", (name.as_str(),))?;
            if obj.is_none() {
                continue;
            }
            obj.call_method1("select_set", (true,))?;
            view_layer.getattr("objects")?.setattr("active", &obj)?;
            let result = ops.getattr("object")?.call_method0("paths_clear")?;
            // Per-item failure inside a batch is data, not a call failure: skip
            // this name rather than aborting the rest of the batch, mirroring
            // the "object not found" skip just above.
            if op_ok(&result, "clear motion path").is_ok() {
                cleared.push(name);
            }
        }

        Ok(json!({ "cleared": cleared }))
    }
);

// ---------------------------------------------------------------------------
// Procedural F-Curve modifiers
// ---------------------------------------------------------------------------

/// Locate an existing F-Curve at `data_path[index]` on `action`. Read-only: routes
/// through [`action_fcurves`] so it works on both the flat (<= 4.3) and the slotted
/// (>= 4.4) action layout.
fn find_fcurve<'py>(
    action: &Bound<'py, PyAny>,
    data_path: &str,
    index: i32,
) -> PyResult<Option<Bound<'py, PyAny>>> {
    for fc in action_fcurves(action)? {
        let path: String = fc.getattr("data_path")?.extract()?;
        let idx: i32 = fc.getattr("array_index")?.extract()?;
        if path == data_path && idx == index {
            return Ok(Some(fc));
        }
    }
    Ok(None)
}

/// Add a new F-Curve modifier of `ty` to `fcurve`, checked **twice**.
///
/// `FCurveModifiers.new()` is a data-API call, not a `bpy.ops` operator — it does not
/// raise when Blender declines to add the modifier. There are two distinct ways it
/// reports the refusal, and both look like success to careless calling code:
///
/// 1. It returns `None` after printing an `RPT_ERROR` to the console. Calling code
///    that assumed success would then crash on the very next `setattr` with an opaque
///    `'NoneType' object has no attribute ...` instead of a clear reason.
/// 2. **Blender 5.2**: it *creates* the modifier and flags it `is_valid == False`.
///    The modifier is in the stack, every `setattr` on it works, the tool's JSON looks
///    perfect — and the modifier does nothing at all. Verified live on 5.2.2: adding
///    CYCLES after a NOISE modifier yields `[('NOISE', True), ('CYCLES', False)]`,
///    where 5.0/5.1 returned `None` for the same call. Reporting that as success is
///    exactly the failure-dressed-as-success this project forbids, so the invalid
///    modifier is removed again and the call fails.
///
/// Both cases come from Blender's ordering rule that a "Cycles" modifier is only ever
/// valid as the first modifier on a curve.
fn new_fmodifier<'py>(fcurve: &Bound<'py, PyAny>, ty: &str) -> PyResult<Bound<'py, PyAny>> {
    let py = fcurve.py();
    let modifiers = fcurve.getattr("modifiers")?;
    let mkw = PyDict::new(py);
    mkw.set_item("type", ty)?;
    let m = modifiers.call_method("new", (), Some(&mkw))?;
    if m.is_none() {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "Blender declined to add a '{ty}' F-Curve modifier (a CYCLES modifier, \
             for example, may only ever be first in the stack)"
        )));
    }
    // Blender 5.2's silent variant of the same refusal, see the doc comment above.
    if m.hasattr("is_valid")? && !m.getattr("is_valid")?.extract::<bool>()? {
        modifiers.call_method1("remove", (&m,))?;
        return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "Blender created a '{ty}' F-Curve modifier but marked it invalid \
             (is_valid == False), so it would have silently done nothing; it has been \
             removed again. A CYCLES modifier is only valid as the FIRST modifier on a \
             curve — add it before any NOISE/ENVELOPE modifier, or clear the existing \
             modifiers first."
        )));
    }
    Ok(m)
}

/// Get-or-create the F-Curve at `data_path[index]` on `obj`, creating animation
/// data / the action itself, and (on Blender >= 4.4) the slot/layer/strip/channelbag
/// chain, as needed.
///
/// Blender 5.0 removed `Action.fcurves` outright — a new curve on a slotted action
/// must be created through a channelbag reached via
/// `action.layers[].strips[].channelbag(slot, ensure=True)`. Blender <= 4.3 still
/// exposes the flat `action.fcurves.new(...)`, used as a fallback when the action
/// has no `layers` collection at all.
fn ensure_fcurve<'py>(
    py: Python<'py>,
    obj: &Bound<'py, PyAny>,
    obj_name: &str,
    data_path: &str,
    index: i32,
) -> PyResult<Bound<'py, PyAny>> {
    let ad = obj.getattr("animation_data")?;
    let ad = if ad.is_none() {
        obj.call_method0("animation_data_create")?
    } else {
        ad
    };

    let action = ad.getattr("action")?;
    let action = if action.is_none() {
        let new_action = py
            .import("bpy")?
            .getattr("data")?
            .getattr("actions")?
            .call_method1("new", (format!("{obj_name}_Action"),))?;
        ad.setattr("action", &new_action)?;
        new_action
    } else {
        action
    };

    if let Some(fc) = find_fcurve(&action, data_path, index)? {
        return Ok(fc);
    }

    let fcurve = if action.hasattr("layers")? {
        // Slotted action (Blender >= 4.4): ensure slot -> layer -> strip -> channelbag.
        let slots = action.getattr("slots")?;
        if py_len(py, &slots)? == 0 {
            let id_type: String = obj.getattr("id_type")?.extract()?;
            let skw = PyDict::new(py);
            skw.set_item("id_type", id_type)?;
            skw.set_item("name", obj_name)?;
            let slot = slots.call_method("new", (), Some(&skw))?;
            ad.setattr("action_slot", &slot)?;
        } else if ad.getattr("action_slot")?.is_none() {
            let first = slots.try_iter()?.next().ok_or_else(|| {
                PyValueError::new_err("action has slots but iteration yielded none")
            })??;
            ad.setattr("action_slot", &first)?;
        }
        let slot = ad.getattr("action_slot")?;

        let layers = action.getattr("layers")?;
        let layer = if py_len(py, &layers)? == 0 {
            layers.call_method1("new", ("Layer",))?
        } else {
            layers.try_iter()?.next().ok_or_else(|| {
                PyValueError::new_err("action has layers but iteration yielded none")
            })??
        };

        let strips = layer.getattr("strips")?;
        let strip = if py_len(py, &strips)? == 0 {
            let stkw = PyDict::new(py);
            stkw.set_item("type", "KEYFRAME")?;
            strips.call_method("new", (), Some(&stkw))?
        } else {
            strips.try_iter()?.next().ok_or_else(|| {
                PyValueError::new_err("layer has strips but iteration yielded none")
            })??
        };

        let ckw = PyDict::new(py);
        ckw.set_item("ensure", true)?;
        let channelbag = strip.call_method("channelbag", (&slot,), Some(&ckw))?;

        let fkw = PyDict::new(py);
        fkw.set_item("index", index)?;
        channelbag
            .getattr("fcurves")?
            .call_method("new", (data_path,), Some(&fkw))?
    } else {
        // Blender <= 4.3: flat list on the action itself.
        let fkw = PyDict::new(py);
        fkw.set_item("index", index)?;
        action
            .getattr("fcurves")?
            .call_method("new", (data_path,), Some(&fkw))?
    };

    // At least one keyframe, matching the legacy handler, so the curve is not empty.
    fcurve
        .getattr("keyframe_points")?
        .call_method1("insert", (1.0_f64, 0.0_f64))?;
    Ok(fcurve)
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AnimAddFcurveNoise {
    /// Object name, as in `bpy.data.objects` (see obj_list).
    pub name: String,
    /// RNA path of the curve to wiggle: `location`, `rotation_euler` (RADIANS),
    /// `scale`, ... Default: `location`. Unlike anim_add_fcurve_cycles and
    /// anim_add_fcurve_envelope, this tool CREATES the animation data, the action
    /// and the F-Curve (with one key at frame 1) when they do not exist yet.
    pub data_path: Option<String>,
    /// Array component of the curve: 0=X, 1=Y, 2=Z. Default: 0.
    pub index: Option<i32>,
    /// Noise amplitude, in the property's own units (RADIANS for a rotation curve,
    /// metres for location). Default: 1.0.
    pub strength: Option<f64>,
    /// Noise period in frames — larger is slower and smoother. Default: 1.0.
    pub scale: Option<f64>,
    /// Random seed of the noise function; give two objects different phases so they
    /// do not wiggle in lockstep. Default here: 0.0 (Blender's RNA default is 1.0).
    pub phase: Option<f64>,
    /// Time offset of the noise, in frames. Default: 0.0.
    pub offset: Option<f64>,
}

crate::tool!(
    "anim_add_fcurve_noise",
    "Add a NOISE F-modifier to an F-Curve for procedural wiggle. This is the only \
     fcurve tool that CREATES the action and the curve (with one key at frame 1) when \
     the object has none. If the loop from anim_add_fcurve_cycles is wanted too, add \
     that one FIRST — Blender only honours a CYCLES modifier at the head of the stack.",
    AnimAddFcurveNoise,
    |py: Python<'_>, p: AnimAddFcurveNoise| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {}",
                p.name
            )));
        }

        let data_path = p.data_path.unwrap_or_else(|| "location".to_string());
        let index = p.index.unwrap_or(0);
        let strength = p.strength.unwrap_or(1.0);
        let scale = p.scale.unwrap_or(1.0);
        let phase = p.phase.unwrap_or(0.0);
        let offset = p.offset.unwrap_or(0.0);

        let fcurve = ensure_fcurve(py, &obj, &p.name, &data_path, index)?;

        let m = new_fmodifier(&fcurve, "NOISE")?;
        m.setattr("strength", strength)?;
        m.setattr("scale", scale)?;
        m.setattr("phase", phase)?;
        m.setattr("offset", offset)?;

        Ok(json!({
            "object": p.name,
            "data_path": data_path,
            "index": index,
            "modifier": "NOISE",
            "strength": strength,
            "scale": scale,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AnimAddFcurveCycles {
    /// Object name, as in `bpy.data.objects` (see obj_list).
    pub name: String,
    /// RNA path of the curve to loop: `location`, `rotation_euler` (RADIANS),
    /// `scale`, ... Default: `location`. The curve must ALREADY exist — key it with
    /// anim_set_keyframe (or create it via anim_add_fcurve_noise) first; this tool
    /// fails rather than creating one.
    pub data_path: Option<String>,
    /// Array component of the curve: 0=X, 1=Y, 2=Z. Default: 0.
    pub index: Option<i32>,
    /// Cycling mode BEFORE the curve's first keyframe. Blender 5.2 accepts exactly:
    /// NONE (no cycling), REPEAT, REPEAT_OFFSET (repeat and keep accumulating the
    /// value), MIRROR (alternate forwards/backwards). Default here: REPEAT
    /// (Blender's own RNA default is NONE).
    pub mode_before: Option<String>,
    /// Cycling mode AFTER the curve's last keyframe. Same four values: NONE, REPEAT,
    /// REPEAT_OFFSET, MIRROR. Default here: REPEAT (Blender's RNA default is NONE).
    pub mode_after: Option<String>,
}

crate::tool!(
    "anim_add_fcurve_cycles",
    "Add a CYCLES F-modifier so an EXISTING F-Curve keeps looping outside its keyed \
     range (keyframe it first with anim_set_keyframe). Blender honours a CYCLES \
     modifier only as the FIRST entry in the curve's modifier stack, so call this \
     before anim_add_fcurve_noise / anim_add_fcurve_envelope — added later it comes \
     back is_valid == False and this tool fails instead of pretending it worked.",
    AnimAddFcurveCycles,
    |py: Python<'_>, p: AnimAddFcurveCycles| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {}",
                p.name
            )));
        }

        let data_path = p.data_path.unwrap_or_else(|| "location".to_string());
        let index = p.index.unwrap_or(0);
        let mode_before = p.mode_before.unwrap_or_else(|| "REPEAT".to_string());
        let mode_after = p.mode_after.unwrap_or_else(|| "REPEAT".to_string());

        let ad = obj.getattr("animation_data")?;
        if ad.is_none() {
            return Err(PyValueError::new_err(format!(
                "no animation data on {}",
                p.name
            )));
        }
        let action = ad.getattr("action")?;
        if action.is_none() {
            return Err(PyValueError::new_err(format!(
                "no animation data on {}",
                p.name
            )));
        }

        let Some(fcurve) = find_fcurve(&action, &data_path, index)? else {
            return Err(PyValueError::new_err(format!(
                "F-Curve not found: {data_path}[{index}]"
            )));
        };

        let m = new_fmodifier(&fcurve, "CYCLES")?;
        m.setattr("mode_before", mode_before.as_str())?;
        m.setattr("mode_after", mode_after.as_str())?;

        Ok(json!({
            "object": p.name,
            "data_path": data_path,
            "index": index,
            "modifier": "CYCLES",
            "mode_before": mode_before,
            "mode_after": mode_after,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct EnvelopePoint {
    /// Frame this control point sits on, in whole frames. Default: 1.
    pub frame: Option<i32>,
    /// Lower reference value at this frame, in the curve's own units. Default: the
    /// modifier's `default_min`.
    pub min: Option<f64>,
    /// Upper reference value at this frame, in the curve's own units. Default: the
    /// modifier's `default_max`.
    pub max: Option<f64>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AnimAddFcurveEnvelope {
    /// Object name, as in `bpy.data.objects` (see obj_list).
    pub name: String,
    /// RNA path of the curve to reshape: `location`, `rotation_euler` (RADIANS),
    /// `scale`, ... Default: `location`. The curve must ALREADY exist — this tool
    /// fails rather than creating one (anim_add_fcurve_noise creates).
    pub data_path: Option<String>,
    /// Array component of the curve: 0=X, 1=Y, 2=Z. Default: 0.
    pub index: Option<i32>,
    /// Lower reference value used where no control point applies, in the curve's own
    /// units. Default here: 0.0 (Blender's RNA default is -1.0).
    pub default_min: Option<f64>,
    /// Upper reference value used where no control point applies, in the curve's own
    /// units. Default: 1.0.
    pub default_max: Option<f64>,
    /// Control points `[{frame, min, max}, ...]`, each remapping the curve at that
    /// frame. Default: none, i.e. only default_min/default_max apply.
    pub points: Option<Vec<EnvelopePoint>>,
}

crate::tool!(
    "anim_add_fcurve_envelope",
    "Add an ENVELOPE F-modifier that rescales an EXISTING F-Curve between per-frame \
     min/max reference values (keyframe the curve first with anim_set_keyframe). If \
     the curve should also loop, add anim_add_fcurve_cycles BEFORE this one: a CYCLES \
     modifier is only valid at the head of the stack.",
    AnimAddFcurveEnvelope,
    |py: Python<'_>, p: AnimAddFcurveEnvelope| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {}",
                p.name
            )));
        }

        let data_path = p.data_path.unwrap_or_else(|| "location".to_string());
        let index = p.index.unwrap_or(0);
        let default_min = p.default_min.unwrap_or(0.0);
        let default_max = p.default_max.unwrap_or(1.0);
        let points = p.points.unwrap_or_default();

        let ad = obj.getattr("animation_data")?;
        if ad.is_none() {
            return Err(PyValueError::new_err(format!(
                "no animation data on {}",
                p.name
            )));
        }
        let action = ad.getattr("action")?;
        if action.is_none() {
            return Err(PyValueError::new_err(format!(
                "no animation data on {}",
                p.name
            )));
        }

        let Some(fcurve) = find_fcurve(&action, &data_path, index)? else {
            return Err(PyValueError::new_err(format!(
                "F-Curve not found: {data_path}[{index}]"
            )));
        };

        let m = new_fmodifier(&fcurve, "ENVELOPE")?;
        m.setattr("default_min", default_min)?;
        m.setattr("default_max", default_max)?;

        let control_points = m.getattr("control_points")?;
        for pt in &points {
            let frame = pt.frame.unwrap_or(1);
            let cp = control_points.call_method1("add", (frame,))?;
            cp.setattr("min", pt.min.unwrap_or(default_min))?;
            cp.setattr("max", pt.max.unwrap_or(default_max))?;
        }

        Ok(json!({
            "object": p.name,
            "data_path": data_path,
            "index": index,
            "modifier": "ENVELOPE",
            "control_points": points.len(),
        }))
    }
);

// ---------------------------------------------------------------------------
// NLA Editor
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NlaCreateAction {
    /// Name for the new action datablock. Blender appends `.001` on a collision.
    pub name: String,
    /// Object to make this the ACTIVE action of, creating its animation data if
    /// needed. An object that does not exist is ignored. Default: leave the action
    /// unassigned (an orphan datablock).
    pub object: Option<String>,
}

crate::tool!(
    "nla_create_action",
    "Create an empty action datablock and optionally make it an object's active \
     action. Fill it with anim_set_keyframe, then bank it as an NLA strip with \
     nla_push.",
    NlaCreateAction,
    |py: Python<'_>, p: NlaCreateAction| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let action = d
            .getattr("actions")?
            .call_method1("new", (p.name.as_str(),))?;

        if let Some(obj_name) = p.object.as_deref() {
            let obj = d.getattr("objects")?.call_method1("get", (obj_name,))?;
            if !obj.is_none() {
                let ad = obj.getattr("animation_data")?;
                let ad = if ad.is_none() {
                    obj.call_method0("animation_data_create")?
                } else {
                    ad
                };
                ad.setattr("action", &action)?;
            }
        }

        Ok(json!({
            "action": action.getattr("name")?.extract::<String>()?,
            "assigned_to": p.object,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NlaPush {
    /// Object name, as in `bpy.data.objects` (see obj_list). Its animation data is
    /// created if missing.
    pub object: String,
    /// Action in `bpy.data.actions` to push down (see nla_create_action). Default:
    /// the object's current active action, which is then cleared.
    pub action: Option<String>,
    /// NLA track to hold the strip; a track of that name is created if there is
    /// none. Default: a brand-new track with Blender's automatic name.
    pub track_name: Option<String>,
    /// Frame the strip starts at, in whole frames. Default: 1. The end frame follows
    /// from the action's own length.
    pub start_frame: Option<i32>,
}

crate::tool!(
    "nla_push",
    "Push an action down into an NLA strip on a track, freeing the object's active \
     action slot for the next take. Inspect the result with nla_list_tracks, retime \
     or blend it with nla_set_strip_props, remove it with nla_delete_strip.",
    NlaPush,
    |py: Python<'_>, p: NlaPush| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {}",
                p.object
            )));
        }

        let ad = obj.getattr("animation_data")?;
        let ad = if ad.is_none() {
            obj.call_method0("animation_data_create")?
        } else {
            ad
        };

        let action = if let Some(action_name) = p.action.as_deref() {
            let a = d.getattr("actions")?.call_method1("get", (action_name,))?;
            if a.is_none() {
                return Err(PyValueError::new_err(format!(
                    "action not found: {action_name}"
                )));
            }
            a
        } else {
            let a = ad.getattr("action")?;
            if a.is_none() {
                return Err(PyValueError::new_err(format!(
                    "no active action on {} and none was specified",
                    p.object
                )));
            }
            a
        };

        let start_frame = p.start_frame.unwrap_or(1);
        let nla_tracks = ad.getattr("nla_tracks")?;
        let track = if let Some(track_name) = p.track_name.as_deref() {
            let t = nla_tracks.call_method1("get", (track_name,))?;
            if t.is_none() {
                let new_t = nla_tracks.call_method0("new")?;
                new_t.setattr("name", track_name)?;
                new_t
            } else {
                t
            }
        } else {
            nla_tracks.call_method0("new")?
        };

        let action_name: String = action.getattr("name")?.extract()?;
        let strip = track
            .getattr("strips")?
            .call_method1("new", (action_name.as_str(), start_frame, &action))?;

        // Clear the active action if it is the one just pushed to the strip.
        let cur_action = ad.getattr("action")?;
        if !cur_action.is_none() {
            let cur_name: String = cur_action.getattr("name")?.extract()?;
            if cur_name == action_name {
                ad.setattr("action", py.None())?;
            }
        }

        Ok(json!({
            "object": p.object,
            "track": track.getattr("name")?.extract::<String>()?,
            "strip": strip.getattr("name")?.extract::<String>()?,
            "action": action_name,
            "frame_start": strip.getattr("frame_start")?.extract::<f64>()?,
            "frame_end": strip.getattr("frame_end")?.extract::<f64>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NlaListTracks {
    /// Object name, as in `bpy.data.objects` (see obj_list).
    pub object: String,
}

crate::tool!(
    "nla_list_tracks",
    "The object's NLA tracks with their strips (name, action, frame range, \
     blend_type, influence, mute) plus its active action. An object with no animation \
     data returns an empty list, not an error. These names are what \
     nla_set_strip_props and nla_delete_strip take.",
    NlaListTracks,
    |py: Python<'_>, p: NlaListTracks| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {}",
                p.object
            )));
        }

        // No animation data is a truthful empty result, not a failure: the object
        // exists, it just has no NLA tracks yet.
        let ad = obj.getattr("animation_data")?;
        if ad.is_none() {
            return Ok(json!({ "object": p.object, "tracks": [] }));
        }

        let mut tracks_info = Vec::new();
        for track in ad.getattr("nla_tracks")?.try_iter()? {
            let track = track?;
            let mut strips_info = Vec::new();
            for strip in track.getattr("strips")?.try_iter()? {
                let strip = strip?;
                let action = strip.getattr("action")?;
                let action_name: Value = if action.is_none() {
                    Value::Null
                } else {
                    Value::String(action.getattr("name")?.extract::<String>()?)
                };
                strips_info.push(json!({
                    "name": strip.getattr("name")?.extract::<String>()?,
                    "action": action_name,
                    "frame_start": strip.getattr("frame_start")?.extract::<f64>()?,
                    "frame_end": strip.getattr("frame_end")?.extract::<f64>()?,
                    "blend_type": strip.getattr("blend_type")?.extract::<String>()?,
                    "influence": strip.getattr("influence")?.extract::<f64>()?,
                    "mute": strip.getattr("mute")?.extract::<bool>()?,
                }));
            }
            tracks_info.push(json!({
                "name": track.getattr("name")?.extract::<String>()?,
                "mute": track.getattr("mute")?.extract::<bool>()?,
                "strips": strips_info,
            }));
        }

        let active_action = ad.getattr("action")?;
        let active_action_name: Value = if active_action.is_none() {
            Value::Null
        } else {
            Value::String(active_action.getattr("name")?.extract::<String>()?)
        };

        Ok(json!({
            "object": p.object,
            "active_action": active_action_name,
            "tracks": tracks_info,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NlaSetStripProps {
    /// Object that owns the track, as in `bpy.data.objects` (see obj_list).
    pub object: String,
    /// NLA track name (see nla_list_tracks).
    pub track: String,
    /// Strip name on that track (see nla_list_tracks).
    pub strip: String,
    /// New start frame of the strip (fractional frames allowed). Default: unchanged.
    pub frame_start: Option<f64>,
    /// New end frame of the strip (fractional frames allowed). Default: unchanged.
    pub frame_end: Option<f64>,
    /// Playback time scale: 1.0 is the action's own speed, 2.0 stretches it to twice
    /// the length. Blender's range is 0.0001–1000, default 1.0. Default: unchanged.
    pub scale: Option<f64>,
    /// How many times the action repeats inside the strip. Blender's range is
    /// 0.1–1000, default 1.0. Default: unchanged.
    pub repeat: Option<f64>,
    /// How the strip combines with the tracks beneath it. Blender 5.2 accepts
    /// exactly: REPLACE (Blender's default), COMBINE, ADD, SUBTRACT, MULTIPLY.
    /// Default: unchanged.
    pub blend_type: Option<String>,
    /// Blend weight of the strip, 0.0–1.0 (a factor, not a percentage). Default:
    /// unchanged.
    pub influence: Option<f64>,
    /// true disables the strip without deleting it (nla_delete_strip deletes).
    /// Default: unchanged.
    pub mute: Option<bool>,
    /// How the strip's value behaves outside its own frame range. Blender 5.2
    /// accepts exactly: NOTHING, HOLD (Blender's default — hold both ends),
    /// HOLD_FORWARD (hold only after the strip). Default: unchanged.
    pub extrapolation: Option<String>,
}

crate::tool!(
    "nla_set_strip_props",
    "Retime or reblend one existing NLA strip: frames, scale, repeat, blend_type, \
     influence, mute, extrapolation. Every setting is optional and an omitted one is \
     left alone. Get the track/strip names from nla_list_tracks.",
    NlaSetStripProps,
    |py: Python<'_>, p: NlaSetStripProps| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found or has no animation data: {}",
                p.object
            )));
        }
        let ad = obj.getattr("animation_data")?;
        if ad.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found or has no animation data: {}",
                p.object
            )));
        }

        let track = ad
            .getattr("nla_tracks")?
            .call_method1("get", (p.track.as_str(),))?;
        if track.is_none() {
            return Err(PyValueError::new_err(format!(
                "track not found: {}",
                p.track
            )));
        }
        let strip = track
            .getattr("strips")?
            .call_method1("get", (p.strip.as_str(),))?;
        if strip.is_none() {
            return Err(PyValueError::new_err(format!(
                "strip not found: {}",
                p.strip
            )));
        }

        if let Some(v) = p.frame_start {
            strip.setattr("frame_start", v)?;
        }
        if let Some(v) = p.frame_end {
            strip.setattr("frame_end", v)?;
        }
        if let Some(v) = p.scale {
            strip.setattr("scale", v)?;
        }
        if let Some(v) = p.repeat {
            strip.setattr("repeat", v)?;
        }
        if let Some(v) = p.blend_type.as_deref() {
            strip.setattr("blend_type", v)?;
        }
        if let Some(v) = p.influence {
            strip.setattr("influence", v)?;
        }
        if let Some(v) = p.mute {
            strip.setattr("mute", v)?;
        }
        if let Some(v) = p.extrapolation.as_deref() {
            strip.setattr("extrapolation", v)?;
        }

        Ok(json!({
            "object": p.object,
            "track": p.track,
            "strip": p.strip,
            "updated": true,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NlaDeleteStrip {
    /// Object that owns the track, as in `bpy.data.objects` (see obj_list).
    pub object: String,
    /// NLA track name (see nla_list_tracks).
    pub track: String,
    /// Strip name on that track (see nla_list_tracks).
    pub strip: String,
}

crate::tool!(
    "nla_delete_strip",
    "Remove one NLA strip from a track. The strip's action datablock survives in \
     bpy.data and can be pushed again with nla_push; to silence a strip without \
     losing it set mute via nla_set_strip_props.",
    NlaDeleteStrip,
    |py: Python<'_>, p: NlaDeleteStrip| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found or has no animation data: {}",
                p.object
            )));
        }
        let ad = obj.getattr("animation_data")?;
        if ad.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found or has no animation data: {}",
                p.object
            )));
        }

        let track = ad
            .getattr("nla_tracks")?
            .call_method1("get", (p.track.as_str(),))?;
        if track.is_none() {
            return Err(PyValueError::new_err(format!(
                "track not found: {}",
                p.track
            )));
        }
        let strip = track
            .getattr("strips")?
            .call_method1("get", (p.strip.as_str(),))?;
        if strip.is_none() {
            return Err(PyValueError::new_err(format!(
                "strip not found: {}",
                p.strip
            )));
        }

        track.getattr("strips")?.call_method1("remove", (&strip,))?;
        Ok(json!({ "deleted": true, "strip": p.strip }))
    }
);

// ---------------------------------------------------------------------------
// Grease Pencil
// ---------------------------------------------------------------------------

/// The Grease Pencil object type string as of Blender 4.3+ (`GREASEPENCIL`); the
/// legacy `GPENCIL` type these handlers were originally written against is gone.
const GP_OBJECT_TYPE: &str = "GREASEPENCIL";

#[derive(Deserialize, schemars::JsonSchema)]
pub struct GpCreate {
    /// Name for the new Grease Pencil object and its data. Default: `GPencil`.
    pub name: Option<String>,
    /// `[x, y, z]` world location in metres. Default: `[0, 0, 0]`.
    pub location: Option<Vec<f64>>,
}

crate::tool!(
    "gp_create",
    "Create a Grease Pencil object (Blender 5 type `GREASEPENCIL`) with one layer \
     named `Layer` and one frame at the current frame. Add more layers with \
     gp_add_layer, frames with gp_add_frame, geometry with gp_draw_stroke.",
    GpCreate,
    |py: Python<'_>, p: GpCreate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let name = p.name.unwrap_or_else(|| "GPencil".to_string());
        let location = p.location.unwrap_or_else(|| vec![0.0, 0.0, 0.0]);

        // Blender 5.0: `grease_pencils` is the (formerly "_v3") new-style data-block;
        // the legacy GPencil type it replaced was removed in 4.3.
        let gp_data = d
            .getattr("grease_pencils")?
            .call_method1("new", (name.as_str(),))?;
        let gp_obj = d
            .getattr("objects")?
            .call_method1("new", (name.as_str(), &gp_data))?;
        if location.len() == 3 {
            gp_obj.setattr(
                "location",
                PyTuple::new(py, [location[0], location[1], location[2]])?,
            )?;
        }

        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&gp_obj,))?;

        let lkw = PyDict::new(py);
        lkw.set_item("set_active", true)?;
        let layer = gp_data
            .getattr("layers")?
            .call_method("new", ("Layer",), Some(&lkw))?;

        let cur_frame: i32 = scene(&bpy)?.getattr("frame_current")?.extract()?;
        layer.getattr("frames")?.call_method1("new", (cur_frame,))?;

        Ok(json!({
            "object": gp_obj.getattr("name")?.extract::<String>()?,
            "data": gp_data.getattr("name")?.extract::<String>()?,
            "layer": layer.getattr("name")?.extract::<String>()?,
            "location": py_vec(&gp_obj.getattr("location")?)?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct GpAddLayer {
    /// Grease Pencil object name (see gp_create / gp_list_layers).
    pub object: String,
    /// Name for the new layer, which is made active. Default: `Layer`.
    pub name: Option<String>,
    /// Layer colour `[r, g, b]`, each component 0.0–1.0. Blender 5.2 made the
    /// dope-sheet swatch `channel_color` read-only, so the colour falls back to
    /// `tint_color` there; the reply's `color_applied_as` says which was used.
    /// Default: leave the colour alone.
    pub color: Option<Vec<f64>>,
}

crate::tool!(
    "gp_add_layer",
    "Add a layer to a Grease Pencil object and make it active. The new layer has no \
     frames — add one with gp_add_frame, or let gp_draw_stroke create it. List what \
     exists with gp_list_layers.",
    GpAddLayer,
    |py: Python<'_>, p: GpAddLayer| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() || obj.getattr("type")?.extract::<String>()? != GP_OBJECT_TYPE {
            return Err(PyValueError::new_err(format!(
                "Grease Pencil object not found: {}",
                p.object
            )));
        }

        let layer_name = p.name.unwrap_or_else(|| "Layer".to_string());
        let lkw = PyDict::new(py);
        lkw.set_item("set_active", true)?;
        let layer = obj.getattr("data")?.getattr("layers")?.call_method(
            "new",
            (layer_name.as_str(),),
            Some(&lkw),
        )?;

        // Blender 5.2 made `channel_color` (the dope-sheet channel swatch) read-only;
        // the writable per-layer colour there is the tint. Try the swatch first so
        // 5.0/5.1 keep their old behaviour, and fall back to the tint on 5.2+,
        // reporting which one actually took the colour.
        let mut applied_as = Value::Null;
        if let Some(color) = p.color {
            let rgb = if color.len() >= 3 {
                PyTuple::new(py, [color[0], color[1], color[2]])?
            } else {
                PyTuple::new(py, color)?
            };
            match layer.setattr("channel_color", &rgb) {
                Ok(()) => applied_as = json!("channel_color"),
                Err(_) => {
                    layer.setattr("tint_color", &rgb)?;
                    layer.setattr("tint_factor", 1.0)?;
                    applied_as = json!("tint_color");
                }
            }
        }

        Ok(json!({
            "object": p.object,
            "layer": layer.getattr("name")?.extract::<String>()?,
            "color_applied_as": applied_as,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct GpAddFrame {
    /// Grease Pencil object name (see gp_create / gp_list_layers).
    pub object: String,
    /// Layer to add the frame to. Default: the object's active layer.
    pub layer: Option<String>,
    /// Frame number, in whole frames. Default: the scene's current frame
    /// (anim_set_frame).
    pub frame: Option<i32>,
}

crate::tool!(
    "gp_add_frame",
    "Add a drawing frame to a Grease Pencil layer. A frame that already exists is \
     reported with `existed: true` rather than duplicated. Draw into it with \
     gp_draw_stroke.",
    GpAddFrame,
    |py: Python<'_>, p: GpAddFrame| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() || obj.getattr("type")?.extract::<String>()? != GP_OBJECT_TYPE {
            return Err(PyValueError::new_err(format!(
                "Grease Pencil object not found: {}",
                p.object
            )));
        }

        let gp_data = obj.getattr("data")?;
        let layer = if let Some(layer_name) = p.layer.as_deref() {
            gp_data
                .getattr("layers")?
                .call_method1("get", (layer_name,))?
        } else {
            gp_data.getattr("layers")?.getattr("active")?
        };
        if layer.is_none() {
            return Err(PyValueError::new_err(format!(
                "no layer found on {} (specify `layer` or add one first)",
                p.object
            )));
        }

        let frame_num = match p.frame {
            Some(f) => f,
            None => scene(&bpy)?.getattr("frame_current")?.extract()?,
        };

        // `GreasePencilFrames` has no documented keyed `.get()`; scan explicitly,
        // the same way `gp_draw_stroke` already has to.
        let frames = layer.getattr("frames")?;
        let mut existing = false;
        for f in frames.try_iter()? {
            let fnum: i32 = f?.getattr("frame_number")?.extract()?;
            if fnum == frame_num {
                existing = true;
                break;
            }
        }

        let layer_name_out: String = layer.getattr("name")?.extract()?;
        if existing {
            return Ok(json!({
                "object": p.object,
                "layer": layer_name_out,
                "frame": frame_num,
                "existed": true,
            }));
        }

        let new_frame = frames.call_method1("new", (frame_num,))?;
        Ok(json!({
            "object": p.object,
            "layer": layer_name_out,
            "frame": new_frame.getattr("frame_number")?.extract::<i32>()?,
            "existed": false,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct GpDrawStroke {
    /// Grease Pencil object name (see gp_create / gp_list_layers).
    pub object: String,
    /// The stroke, as `[[x, y, z, pressure?], ...]` in object-local metres.
    /// `pressure` (default 1.0) multiplies the per-point radius. At least one point
    /// is required.
    pub points: Vec<Vec<f64>>,
    /// Layer to draw on; it is created if the object has no layer at all. Default:
    /// the active layer.
    pub layer: Option<String>,
    /// Frame to draw on, in whole frames; it is created if missing. Default: the
    /// scene's current frame (anim_set_frame).
    pub frame: Option<i32>,
    /// Material slot to use, by material name on the object. Default: slot 0.
    pub material: Option<String>,
    /// Legacy stroke width in pixels. Blender 5's Grease Pencil v3 has no
    /// stroke-level width, so it is folded into each point's radius as
    /// `line_width / 2000 * pressure` scene units. Default: 10.
    pub line_width: Option<i32>,
}

crate::tool!(
    "gp_draw_stroke",
    "Draw one stroke of 3D points onto a Grease Pencil layer/frame, creating the \
     layer or frame if needed (gp_add_layer / gp_add_frame do that explicitly). \
     Coordinates are object-local metres; there is no per-stroke width on Blender 5, \
     so `line_width` becomes a per-point radius.",
    GpDrawStroke,
    |py: Python<'_>, p: GpDrawStroke| {
        if p.points.is_empty() {
            return Err(PyValueError::new_err("no points provided"));
        }

        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() || obj.getattr("type")?.extract::<String>()? != GP_OBJECT_TYPE {
            return Err(PyValueError::new_err(format!(
                "Grease Pencil object not found: {}",
                p.object
            )));
        }

        let gp_data = obj.getattr("data")?;
        let layers = gp_data.getattr("layers")?;
        let mut layer = if let Some(layer_name) = p.layer.as_deref() {
            layers.call_method1("get", (layer_name,))?
        } else {
            layers.getattr("active")?
        };
        if layer.is_none() {
            let lkw = PyDict::new(py);
            lkw.set_item("set_active", true)?;
            layer = layers.call_method("new", ("Layer",), Some(&lkw))?;
        }

        let frame_num = match p.frame {
            Some(f) => f,
            None => scene(&bpy)?.getattr("frame_current")?.extract()?,
        };

        let frames = layer.getattr("frames")?;
        let mut frame_obj = None;
        for f in frames.try_iter()? {
            let f = f?;
            let fnum: i32 = f.getattr("frame_number")?.extract()?;
            if fnum == frame_num {
                frame_obj = Some(f);
                break;
            }
        }
        let frame_obj = match frame_obj {
            Some(f) => f,
            None => frames.call_method1("new", (frame_num,))?,
        };

        // Blender 4.3+: a frame no longer owns strokes directly; `drawing.strokes`
        // is the read compatibility shim for the old `frame.strokes`. But the write
        // side changed completely in Blender 5's Grease Pencil v3: strokes moved to
        // a curves-style flat point layout with no `strokes.new()` / `points.add()`
        // / `point.co` / `.pressure` / `.strength`, and no stroke-level `line_width`
        // at all. Verified live against Blender 5.0: a stroke is allocated with
        // `drawing.add_strokes([point_count])`, and each point carries `.position`
        // (was `.co`) and `.radius` (replaces `line_width` + `.pressure` combined)
        // and `.opacity` (was `.strength`).
        let drawing = frame_obj.getattr("drawing")?;
        let stroke_index = py_len(py, &drawing.getattr("strokes")?)?;

        let n_points = p.points.len();
        drawing.call_method1("add_strokes", (PyList::new(py, [n_points as i32])?,))?;
        // `drawing.strokes` is a length-bound view: the handle fetched *before*
        // `add_strokes` does not grow with it (indexing into it raised `IndexError:
        // Key 0 is out of range` even though the new stroke existed) — verified
        // live. Re-fetch the collection after mutating so the index is valid.
        let stroke = drawing.getattr("strokes")?.get_item(stroke_index)?;

        if let Some(material_name) = p.material.as_deref() {
            for (i, slot) in obj.getattr("material_slots")?.try_iter()?.enumerate() {
                let slot = slot?;
                let mat = slot.getattr("material")?;
                if mat.is_none() {
                    continue;
                }
                let mn: String = mat.getattr("name")?.extract()?;
                if mn == material_name {
                    stroke.setattr("material_index", i as i32)?;
                    break;
                }
            }
        }

        // The old stroke-level `line_width` (pixels) no longer exists; fold it into
        // the new per-point `radius` (scene units) alongside the old per-point
        // pressure multiplier, so the parameter keeps roughly its old meaning.
        let line_width = p.line_width.unwrap_or(10) as f64;
        let stroke_points = stroke.getattr("points")?;
        for (i, pt) in p.points.iter().enumerate() {
            let sp = stroke_points.get_item(i)?;
            let mut xyz: Vec<f64> = pt.iter().take(3).copied().collect();
            xyz.resize(3, 0.0);
            sp.setattr("position", PyTuple::new(py, xyz)?)?;
            let pressure = pt.get(3).copied().unwrap_or(1.0);
            sp.setattr("radius", (line_width / 2000.0) * pressure)?;
            sp.setattr("opacity", 1.0)?;
        }

        Ok(json!({
            "object": p.object,
            "layer": layer.getattr("name")?.extract::<String>()?,
            "frame": frame_num,
            "points": n_points,
        }))
    }
);

/// Best-effort `setattr(m, key, value)` for every key in `properties` the modifier
/// actually has, mirroring the legacy handler's permissive `{key: val}` property bag.
fn apply_gp_properties(m: &Bound<'_, PyAny>, properties: Option<&Value>) -> PyResult<()> {
    let Some(Value::Object(pmap)) = properties else {
        return Ok(());
    };
    for (k, v) in pmap {
        if !m.hasattr(k.as_str())? {
            continue;
        }
        match v {
            Value::Bool(b) => m.setattr(k.as_str(), *b)?,
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    m.setattr(k.as_str(), i)?;
                } else if let Some(f) = n.as_f64() {
                    m.setattr(k.as_str(), f)?;
                }
            }
            Value::String(s) => m.setattr(k.as_str(), s.as_str())?,
            Value::Array(a) => {
                let nums: Vec<f64> = a.iter().filter_map(Value::as_f64).collect();
                if nums.len() == a.len() && !nums.is_empty() {
                    m.setattr(k.as_str(), PyTuple::new(m.py(), nums)?)?;
                }
            }
            Value::Null | Value::Object(_) => {}
        }
    }
    Ok(())
}

/// Map the tool's short modifier names to Blender's `type` enum. Blender 4.3 merged
/// Grease Pencil modifiers into the regular modifier stack and renamed every
/// `GP_*` identifier to `GREASE_PENCIL_*`; unknown names pass through unchanged so
/// a caller can still supply a raw Blender identifier directly.
fn gp_modifier_type(ty: &str) -> String {
    let mapped = match ty.to_lowercase().as_str() {
        "noise" => "GREASE_PENCIL_NOISE",
        "smooth" => "GREASE_PENCIL_SMOOTH",
        "thickness" => "GREASE_PENCIL_THICKNESS",
        "tint" => "GREASE_PENCIL_TINT",
        "offset" => "GREASE_PENCIL_OFFSET",
        "opacity" => "GREASE_PENCIL_OPACITY",
        "color" => "GREASE_PENCIL_COLOR",
        "array" => "GREASE_PENCIL_ARRAY",
        "build" => "GREASE_PENCIL_BUILD",
        "mirror" => "GREASE_PENCIL_MIRROR",
        "simplify" => "GREASE_PENCIL_SIMPLIFY",
        "subdiv" => "GREASE_PENCIL_SUBDIV",
        "multiply" => "GREASE_PENCIL_MULTIPLY",
        "armature" => "GREASE_PENCIL_ARMATURE",
        "hook" => "GREASE_PENCIL_HOOK",
        "lattice" => "GREASE_PENCIL_LATTICE",
        "time" => "GREASE_PENCIL_TIME",
        "length" => "GREASE_PENCIL_LENGTH",
        "dash" => "GREASE_PENCIL_DASH",
        "envelope" => "GREASE_PENCIL_ENVELOPE",
        "outline" => "GREASE_PENCIL_OUTLINE",
        "shrinkwrap" => "GREASE_PENCIL_SHRINKWRAP",
        "texture" => "GREASE_PENCIL_TEXTURE",
        "weight_proximity" => "GREASE_PENCIL_VERTEX_WEIGHT_PROXIMITY",
        "weight_angle" => "GREASE_PENCIL_VERTEX_WEIGHT_ANGLE",
        // Line Art is not a Grease-Pencil-prefixed modifier: it is the generic
        // LINEART modifier, which a Grease Pencil object carries like any other.
        "lineart" => "LINEART",
        _ => return ty.to_string(),
    };
    mapped.to_string()
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct GpAddModifier {
    /// Grease Pencil object name (see gp_create / gp_list_layers).
    pub object: String,
    /// Short modifier name, mapped to Blender 5's `GREASE_PENCIL_*` enum: noise,
    /// smooth, thickness, tint, offset, opacity, color, array, build, mirror,
    /// simplify, subdiv, multiply, armature, hook, lattice, time, length, dash,
    /// envelope, outline, shrinkwrap, texture, weight_proximity, weight_angle,
    /// lineart. An unrecognised value is passed to Blender verbatim, so a raw
    /// identifier such as `GREASE_PENCIL_NOISE` also works.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Name for the modifier. Default: the `type` string as given.
    pub name: Option<String>,
    /// Type-specific settings as `{blender_property_name: value}` — numbers,
    /// booleans, strings or number lists. A key the modifier does not have is
    /// skipped. Default: Blender's own defaults.
    pub properties: Option<Value>,
}

crate::tool!(
    "gp_add_modifier",
    "Add a modifier to a Grease Pencil object. Blender 4.3 merged GP modifiers into \
     the regular stack, so the short names here map to the `GREASE_PENCIL_*` enum \
     (noise, smooth, thickness, tint, build, array, ...); type-specific settings go \
     in `properties`. This is a GP-only wrapper — a mesh takes geo_modifier_* / \
     obj_add_modifier instead.",
    GpAddModifier,
    |py: Python<'_>, p: GpAddModifier| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() || obj.getattr("type")?.extract::<String>()? != GP_OBJECT_TYPE {
            return Err(PyValueError::new_err(format!(
                "Grease Pencil object not found: {}",
                p.object
            )));
        }

        let actual_type = gp_modifier_type(&p.r#type);
        let mod_name = p.name.unwrap_or_else(|| p.r#type.clone());

        // Blender 4.3 unified Grease Pencil modifiers into the regular modifier
        // stack (`obj.modifiers`); the old `obj.grease_pencil_modifiers` collection
        // these handlers were written against is gone.
        let mkw = PyDict::new(py);
        mkw.set_item("name", mod_name.as_str())?;
        mkw.set_item("type", actual_type.as_str())?;
        let m = match obj.getattr("modifiers")?.call_method("new", (), Some(&mkw)) {
            Ok(m) => m,
            Err(e) => {
                // The object and type name are valid; the failure happened inside
                // Blender's modifier-creation call itself.
                return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                    "failed to add modifier '{actual_type}' on {}: {e}",
                    p.object
                )));
            }
        };

        apply_gp_properties(&m, p.properties.as_ref())?;

        Ok(json!({
            "object": p.object,
            "modifier": m.getattr("name")?.extract::<String>()?,
            "type": actual_type,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct GpListLayers {
    /// Grease Pencil object name (see gp_create).
    pub object: String,
}

crate::tool!(
    "gp_list_layers",
    "Every layer of a Grease Pencil object with its opacity, hide flag, whether it is \
     active, and each of its frames with a stroke count. Read-only; the layer names \
     here are what gp_add_frame and gp_draw_stroke take.",
    GpListLayers,
    |py: Python<'_>, p: GpListLayers| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() || obj.getattr("type")?.extract::<String>()? != GP_OBJECT_TYPE {
            return Err(PyValueError::new_err(format!(
                "Grease Pencil object not found: {}",
                p.object
            )));
        }

        let layers = obj.getattr("data")?.getattr("layers")?;
        let active = layers.getattr("active")?;

        let mut layers_info = Vec::new();
        for layer in layers.try_iter()? {
            let layer = layer?;
            let mut frames_info = Vec::new();
            for frame in layer.getattr("frames")?.try_iter()? {
                let frame = frame?;
                let strokes = py_len(py, &frame.getattr("drawing")?.getattr("strokes")?)?;
                frames_info.push(json!({
                    "frame": frame.getattr("frame_number")?.extract::<i32>()?,
                    "strokes": strokes,
                }));
            }
            let is_active = !active.is_none() && layer.eq(&active)?;
            layers_info.push(json!({
                "name": layer.getattr("name")?.extract::<String>()?,
                "active": is_active,
                "frames": frames_info,
                "opacity": layer.getattr("opacity")?.extract::<f64>()?,
                "hide": layer.getattr("hide")?.extract::<bool>()?,
            }));
        }

        Ok(json!({
            "object": p.object,
            "layers": layers_info,
        }))
    }
);
