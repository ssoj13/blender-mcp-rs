//! Utility (snap/align/distribute/randomize/boolean), Asset Browser, and Video
//! Sequence Editor (VSE) tools.
//!
//! Ported from `addon/mcp_server/__init__.py`'s `_h_util_*` / `_h_asset_*` /
//! `_h_vse_*` handlers. The Python originals returned a `{"error": ...}`
//! payload instead of raising for every failure path. That is **not**
//! preserved here: per the `tool!` macro's error contract (`mod.rs`), a call
//! that could not do what it was asked returns a hard `Err`, so MCP marks the
//! result `isError` and the model cannot read past it as success. The one
//! exception is a per-item entry inside a batch
//! (`results.push(json!({"error": ...}))`), which stays data because the
//! call as a whole did partially succeed.
//!
//! ## VSE and Blender 5
//! Blender 4.4 renamed `SequenceEditor.sequences`/`.sequences_all` to
//! `.strips`/`.strips_all` (the `new_image`/`new_movie`/... constructors moved
//! with them, and `Sequence` became `Strip`); 5.0 dropped the pre-4.4 names
//! entirely. The legacy Python handlers were written against the old names and
//! raise `AttributeError` on Blender 5. [`vse_strips`] / [`vse_strips_all`]
//! below prefer the new attribute and fall back to the old one, so this module
//! works unchanged on Blender 5.0 and on <= 4.3.

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyIterator, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, op_ok, py_len, scene};

// ============================================================================
// Shared helpers
// ============================================================================

/// Extract a Python float sequence (`location`, `scale`, a bound-box corner,
/// ...) into a `Vec<f64>`.
fn py_vec(seq: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    let mut v = Vec::new();
    for x in PyIterator::from_object(seq)? {
        v.push(x?.extract::<f64>()?);
    }
    Ok(v)
}

/// Maps `X`/`Y`/`Z` to the matching `Vector` attribute name, defaulting to `x`
/// for anything else — mirrors the original Python's
/// `{"X": 0, "Y": 1, "Z": 2}.get(axis, 0)` (unknown axis silently becomes X).
fn axis_component(axis: &str) -> &'static str {
    match axis {
        "Y" => "y",
        "Z" => "z",
        _ => "x",
    }
}

/// Promotes a shared per-item helper's `{"error": ...}` result to a hard
/// `Err` when that result becomes the *entire* tool result (single-target
/// mode). Helpers like `snap_one`/`randomize_one`/`apply_booleans_one` double
/// as batch-item builders, where the same JSON shape is legitimate data — so
/// this conversion is only applied at single-target call sites, never when
/// the value is being pushed into a `results` array.
fn require_ok(v: Value) -> PyResult<Value> {
    match v.get("error").and_then(Value::as_str) {
        Some(msg) => Err(PyValueError::new_err(msg.to_string())),
        None => Ok(v),
    }
}

// ============================================================================
// Snap & Align
// ============================================================================

/// The lowest world-space Z of `obj`'s bounding box, computed by hand (matrix
/// row 2 dotted with each local corner) rather than via `mathutils.Vector`, so
/// this needs nothing beyond plain attribute reads.
///
/// `matrix_world` is not recomputed synchronously when `location` is set
/// directly — a standing Blender gotcha that bites hardest in `--background`
/// mode, where nothing else (no viewport redraw, no frame change) ever forces
/// the depsgraph to resolve it. Verified live: setting `obj.location.z` and
/// immediately reading `obj.matrix_world.translation` back returns the OLD
/// value until `view_layer.update()` runs. A caller that just repositioned
/// this object (its own earlier tool call, or another tool entirely, in the
/// same headless session) would otherwise have this function silently snap
/// against a stale transform.
fn min_bbox_world_z(obj: &Bound<'_, PyAny>) -> PyResult<f64> {
    obj.py()
        .import("bpy")?
        .getattr("context")?
        .getattr("view_layer")?
        .call_method0("update")?;

    let mut rows = Vec::with_capacity(4);
    for row in PyIterator::from_object(&obj.getattr("matrix_world")?)? {
        rows.push(py_vec(&row?)?);
    }
    let row2 = rows
        .get(2)
        .ok_or_else(|| PyValueError::new_err("matrix_world has fewer than 3 rows"))?;
    let at = |v: &[f64], i: usize| v.get(i).copied().unwrap_or(0.0);

    let mut min_z = f64::INFINITY;
    for corner in PyIterator::from_object(&obj.getattr("bound_box")?)? {
        let c = py_vec(&corner?)?;
        let wz = at(row2, 0) * at(&c, 0)
            + at(row2, 1) * at(&c, 1)
            + at(row2, 2) * at(&c, 2)
            + at(row2, 3);
        min_z = min_z.min(wz);
    }
    Ok(min_z)
}

fn snap_one(
    objects: &Bound<'_, PyAny>,
    name: &str,
    target_z: f64,
    align_bottom: bool,
) -> PyResult<Value> {
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Ok(json!({ "name": name, "error": format!("Object not found: {name}") }));
    }
    let otype: String = obj.getattr("type")?.extract()?;
    let loc = obj.getattr("location")?;
    if align_bottom && otype == "MESH" {
        let offset = target_z - min_bbox_world_z(&obj)?;
        let cur_z: f64 = loc.getattr("z")?.extract()?;
        loc.setattr("z", cur_z + offset)?;
    } else {
        loc.setattr("z", target_z)?;
    }
    let new_z: f64 = obj.getattr("location")?.getattr("z")?.extract()?;
    Ok(json!({ "name": name, "new_z": new_z }))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UtilSnapToGround {
    /// Single mode: name of one object in `bpy.data.objects`. Ignored when `names`
    /// is given; exactly one of `name`/`names` is required.
    pub name: Option<String>,
    /// Batch mode: object names to snap in one call. Takes precedence over `name`.
    /// A name that does not exist becomes a per-item `{"error": ...}` entry in
    /// `results` rather than failing the whole call.
    pub names: Option<Vec<String>>,
    /// World-space Z, in Blender units, that the object lands on. Default: 0.0.
    pub target_z: Option<f64>,
    /// true: put the lowest corner of the world-space bounding box at `target_z`
    /// (MESH objects only; anything else falls back to setting the origin).
    /// false: set the object origin's Z to `target_z` directly. Default: true.
    pub align_bottom: Option<bool>,
}

crate::tool!(
    "util_snap_to_ground",
    "Drop object(s) onto a horizontal plane by moving them along Z only. Single: \
     {name}. Batch: {names: [...]}. X/Y are untouched — use util_align to line \
     objects up on an axis and util_distribute to space them out.",
    UtilSnapToGround,
    |py: Python<'_>, p: UtilSnapToGround| {
        let bpy = py.import("bpy")?;
        let objects = data(&bpy)?.getattr("objects")?;
        let target_z = p.target_z.unwrap_or(0.0);
        let align_bottom = p.align_bottom.unwrap_or(true);

        if let Some(names) = p.names {
            let results: Vec<Value> = names
                .iter()
                .map(|n| snap_one(&objects, n, target_z, align_bottom))
                .collect::<PyResult<_>>()?;
            let ok = results.iter().filter(|r| r.get("error").is_none()).count();
            return Ok(json!({ "snapped": ok, "results": results }));
        }
        let Some(name) = p.name else {
            return Err(PyValueError::new_err("provide 'name' or 'names'"));
        };
        require_ok(snap_one(&objects, &name, target_z, align_bottom)?)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UtilAlign {
    /// Object names to align. Names that do not exist are skipped; at least one
    /// must resolve.
    pub names: Vec<String>,
    /// Axis whose object-origin component is overwritten. One of: X, Y, Z
    /// (case-insensitive). Anything else is treated as X.
    pub axis: String,
    /// Where to align to, either a keyword string or a number:
    /// "MIN" (lowest current position), "MAX" (highest), "CENTER" (mean of the
    /// current positions), "CURSOR" (the 3D cursor's component on `axis`), or a
    /// numeric world-space coordinate in Blender units. Default: "CENTER".
    pub align_to: Option<Value>,
}

crate::tool!(
    "util_align",
    "Set one axis component of several objects' origins to a common value \
     (min/max/center/cursor/number), leaving the other two axes alone. Use \
     util_distribute to spread them out along an axis instead, and \
     util_snap_to_ground to sit them on a Z plane by bounding box.",
    UtilAlign,
    |py: Python<'_>, p: UtilAlign| {
        if p.names.is_empty() {
            return Err(PyValueError::new_err("provide 'names' array"));
        }
        let axis = p.axis.to_uppercase();
        let comp = axis_component(&axis);

        let bpy = py.import("bpy")?;
        let objects = data(&bpy)?.getattr("objects")?;
        let mut found: Vec<Bound<'_, PyAny>> = Vec::new();
        for name in &p.names {
            let obj = objects.call_method1("get", (name.as_str(),))?;
            if !obj.is_none() {
                found.push(obj);
            }
        }
        if found.is_empty() {
            return Err(PyValueError::new_err("no valid objects found"));
        }

        let positions: Vec<f64> = found
            .iter()
            .map(|o| -> PyResult<f64> { o.getattr("location")?.getattr(comp)?.extract() })
            .collect::<PyResult<_>>()?;

        let target = match &p.align_to {
            None => positions.iter().sum::<f64>() / positions.len() as f64,
            Some(Value::String(s)) => match s.as_str() {
                "MIN" => positions.iter().cloned().fold(f64::INFINITY, f64::min),
                "MAX" => positions.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                "CENTER" => positions.iter().sum::<f64>() / positions.len() as f64,
                "CURSOR" => scene(&bpy)?
                    .getattr("cursor")?
                    .getattr("location")?
                    .getattr(comp)?
                    .extract()?,
                other => return Err(PyValueError::new_err(format!("invalid align_to: {other}"))),
            },
            Some(Value::Number(n)) => match n.as_f64() {
                Some(f) => f,
                None => return Err(PyValueError::new_err(format!("invalid align_to: {n}"))),
            },
            Some(other) => return Err(PyValueError::new_err(format!("invalid align_to: {other}"))),
        };

        for obj in &found {
            obj.getattr("location")?.setattr(comp, target)?;
        }
        Ok(json!({ "aligned": found.len(), "axis": axis, "value": target }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UtilDistribute {
    /// Object names to distribute; at least 2 must resolve. They are sorted by
    /// their current position on `axis` first, so call order does not matter.
    pub names: Vec<String>,
    /// Axis to spread along. One of: X, Y, Z (case-insensitive). Anything else
    /// is treated as X.
    pub axis: String,
    /// Step between neighbours, in Blender units, measured from the lowest
    /// object. Omit to spread evenly between the current lowest and highest
    /// object instead (they stay put; everything between them is respaced).
    pub spacing: Option<f64>,
}

crate::tool!(
    "util_distribute",
    "Space objects evenly along one axis, either at a fixed `spacing` or spread \
     between the two outermost objects. Only that axis moves — util_align puts \
     them all at the same value instead.",
    UtilDistribute,
    |py: Python<'_>, p: UtilDistribute| {
        if p.names.len() < 2 {
            return Err(PyValueError::new_err("need at least 2 objects"));
        }
        let axis = p.axis.to_uppercase();
        let comp = axis_component(&axis);

        let bpy = py.import("bpy")?;
        let objects = data(&bpy)?.getattr("objects")?;
        let mut found: Vec<(f64, Bound<'_, PyAny>)> = Vec::new();
        for name in &p.names {
            let obj = objects.call_method1("get", (name.as_str(),))?;
            if !obj.is_none() {
                let pos: f64 = obj.getattr("location")?.getattr(comp)?.extract()?;
                found.push((pos, obj));
            }
        }
        if found.len() < 2 {
            return Err(PyValueError::new_err("need at least 2 valid objects"));
        }
        found.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let n = found.len();
        let start = found[0].0;
        let step = match p.spacing {
            Some(sp) => sp,
            None => (found[n - 1].0 - start) / (n - 1) as f64,
        };
        for (i, (_, obj)) in found.iter().enumerate() {
            obj.getattr("location")?
                .setattr(comp, start + i as f64 * step)?;
        }
        Ok(json!({ "distributed": n, "axis": axis }))
    }
);

// ============================================================================
// Randomize
// ============================================================================

fn range_at(v: &Option<Vec<f64>>, i: usize) -> f64 {
    v.as_ref().and_then(|a| a.get(i)).copied().unwrap_or(0.0)
}

fn jitter(random: &Bound<'_, PyAny>, r: f64) -> PyResult<f64> {
    random.call_method1("uniform", (-r, r))?.extract()
}

fn randomize_one(
    objects: &Bound<'_, PyAny>,
    random: &Bound<'_, PyAny>,
    name: &str,
    loc_range: &Option<Vec<f64>>,
    rot_range: &Option<Vec<f64>>,
    scale_range: &Option<Vec<f64>>,
) -> PyResult<Value> {
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Ok(json!({ "name": name, "error": format!("Object not found: {name}") }));
    }

    let lr = [
        range_at(loc_range, 0),
        range_at(loc_range, 1),
        range_at(loc_range, 2),
    ];
    if lr.iter().any(|x| *x != 0.0) {
        let loc = obj.getattr("location")?;
        for (comp, r) in ["x", "y", "z"].into_iter().zip(lr) {
            let cur: f64 = loc.getattr(comp)?.extract()?;
            loc.setattr(comp, cur + jitter(random, r)?)?;
        }
    }

    let rr = [
        range_at(rot_range, 0),
        range_at(rot_range, 1),
        range_at(rot_range, 2),
    ];
    if rr.iter().any(|x| *x != 0.0) {
        let rot = obj.getattr("rotation_euler")?;
        for (comp, r) in ["x", "y", "z"].into_iter().zip(rr) {
            let cur: f64 = rot.getattr(comp)?.extract()?;
            rot.setattr(comp, cur + jitter(random, r)?)?;
        }
    }

    let sr = [
        range_at(scale_range, 0),
        range_at(scale_range, 1),
        range_at(scale_range, 2),
    ];
    if sr.iter().any(|x| *x != 0.0) {
        let sc = obj.getattr("scale")?;
        for (comp, r) in ["x", "y", "z"].into_iter().zip(sr) {
            let cur: f64 = sc.getattr(comp)?.extract()?;
            sc.setattr(comp, cur * (1.0 + jitter(random, r)?))?;
        }
    }

    Ok(json!({
        "name": name,
        "location": py_vec(&obj.getattr("location")?)?,
        "rotation": py_vec(&obj.getattr("rotation_euler")?)?,
        "scale": py_vec(&obj.getattr("scale")?)?,
    }))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UtilRandomizeTransform {
    /// Single mode: name of one object. Ignored when `names` is given; exactly
    /// one of `name`/`names` is required.
    pub name: Option<String>,
    /// Batch mode: object names to randomize in one call. Takes precedence over
    /// `name`. A missing name becomes a per-item `{"error": ...}` in `results`.
    pub names: Option<Vec<String>>,
    /// [x, y, z] half-widths, in Blender units. Each axis gets a uniform offset
    /// in [-r, +r] ADDED to the current location. Missing entries and 0 mean no
    /// change on that axis. Default: no location change.
    pub loc_range: Option<Vec<f64>>,
    /// [x, y, z] half-widths in RADIANS, added to the current `rotation_euler`
    /// the same way as `loc_range`. Default: no rotation change.
    pub rot_range: Option<Vec<f64>>,
    /// [x, y, z] relative variation: scale is MULTIPLIED by `1 + uniform(-r, r)`,
    /// so 0.1 means +/-10%. Default: no scale change.
    pub scale_range: Option<Vec<f64>>,
    /// Seed for Python's `random` module. The same seed and the same object
    /// order reproduce the same jitter exactly. Omit for an unseeded sequence.
    pub seed: Option<i64>,
}

crate::tool!(
    "util_randomize_transform",
    "Add uniform random jitter to object location/rotation/scale — for scattering \
     duplicates so they do not look cloned. Single: {name}. Batch: {names: [...]}. \
     Offsets are relative to the current transform, so repeated calls compound.",
    UtilRandomizeTransform,
    |py: Python<'_>, p: UtilRandomizeTransform| {
        let bpy = py.import("bpy")?;
        let objects = data(&bpy)?.getattr("objects")?;
        // Delegates to Python's own `random` module (seed + uniform) instead of
        // pulling in a `rand` crate dependency: a given `seed` then reproduces
        // exactly the sequence the legacy handler produced.
        let random = py.import("random")?;
        if let Some(seed) = p.seed {
            random.call_method1("seed", (seed,))?;
        }

        if let Some(names) = &p.names {
            let results: Vec<Value> = names
                .iter()
                .map(|n| {
                    randomize_one(
                        &objects,
                        &random,
                        n,
                        &p.loc_range,
                        &p.rot_range,
                        &p.scale_range,
                    )
                })
                .collect::<PyResult<_>>()?;
            let ok = results.iter().filter(|r| r.get("error").is_none()).count();
            return Ok(json!({ "randomized": ok, "results": results }));
        }
        let Some(name) = &p.name else {
            return Err(PyValueError::new_err("provide 'name' or 'names'"));
        };
        require_ok(randomize_one(
            &objects,
            &random,
            name,
            &p.loc_range,
            &p.rot_range,
            &p.scale_range,
        )?)
    }
);

// ============================================================================
// Boolean Operations
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UtilBoolean {
    /// Name of the MESH object that receives the Boolean modifier and is
    /// reshaped by it.
    pub object: String,
    /// Name of the MESH object used as the cutter/operand. It is referenced by
    /// the modifier, not consumed: it stays in the scene (hide it yourself).
    pub target: String,
    /// One of: DIFFERENCE (subtract `target` from `object`), UNION, INTERSECT.
    /// Case-insensitive. Default: DIFFERENCE.
    pub operation: Option<String>,
    /// true: run `object.modifier_apply` right away, baking the result into the
    /// mesh. false: leave the modifier live so mod_update can still tune it, and
    /// util_apply_booleans can bake it later. Default: false.
    pub apply: Option<bool>,
    /// Blender 5.2 solver enum, verified live: FLOAT (fast, approximate),
    /// EXACT (robust, the default here), MANIFOLD (fastest, requires
    /// watertight/manifold inputs). The older value FAST is gone in 5.2 and
    /// raises a TypeError. Default: EXACT.
    pub solver: Option<String>,
}

crate::tool!(
    "util_boolean",
    "Add a Boolean modifier cutting one mesh with another (DIFFERENCE/UNION/\
     INTERSECT), optionally applying it at once. Solver values on Blender 5.2 are \
     FLOAT, EXACT (default) and MANIFOLD — FAST was removed. To bake Boolean \
     modifiers that are already on an object, use util_apply_booleans instead; to \
     retune one before baking, use mod_update.",
    UtilBoolean,
    |py: Python<'_>, p: UtilBoolean| {
        let bpy = py.import("bpy")?;
        let objects = data(&bpy)?.getattr("objects")?;
        let obj = objects.call_method1("get", (p.object.as_str(),))?;
        let target = objects.call_method1("get", (p.target.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {}",
                p.object
            )));
        }
        if target.is_none() {
            return Err(PyValueError::new_err(format!(
                "target not found: {}",
                p.target
            )));
        }
        let obj_type: String = obj.getattr("type")?.extract()?;
        let target_type: String = target.getattr("type")?.extract()?;
        if obj_type != "MESH" || target_type != "MESH" {
            return Err(PyValueError::new_err("both objects must be meshes"));
        }

        let operation = p
            .operation
            .unwrap_or_else(|| "DIFFERENCE".to_string())
            .to_uppercase();
        let solver = p.solver.unwrap_or_else(|| "EXACT".to_string());

        let mods = obj.getattr("modifiers")?;
        let kw = PyDict::new(py);
        kw.set_item("name", format!("Boolean_{}", p.target))?;
        kw.set_item("type", "BOOLEAN")?;
        let m = mods.call_method("new", (), Some(&kw))?;
        m.setattr("operation", operation.as_str())?;
        m.setattr("object", &target)?;
        m.setattr("solver", solver.as_str())?;
        let mod_name: String = m.getattr("name")?.extract()?;

        let mut result = json!({
            "object": p.object,
            "target": p.target,
            "operation": operation,
            "modifier": mod_name,
        });

        if p.apply.unwrap_or(false) {
            bpy.getattr("context")?
                .getattr("view_layer")?
                .getattr("objects")?
                .setattr("active", &obj)?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("modifier", mod_name.as_str())?;
            let apply_result = bpy.getattr("ops")?.getattr("object")?.call_method(
                "modifier_apply",
                (),
                Some(&kwargs),
            )?;
            // Verified live: applying a stale/gone modifier name does not raise —
            // it returns {'CANCELLED'} silently.
            let obj_name: String = obj.getattr("name")?.extract()?;
            op_ok(
                &apply_result,
                &format!("apply modifier {mod_name} on {obj_name}"),
            )?;
            result["applied"] = json!(true);
        }
        Ok(result)
    }
);

fn apply_booleans_one(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    objects: &Bound<'_, PyAny>,
    name: &str,
) -> PyResult<Value> {
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Ok(json!({ "name": name, "error": format!("Object not found: {name}") }));
    }
    bpy.getattr("context")?
        .getattr("view_layer")?
        .getattr("objects")?
        .setattr("active", &obj)?;

    // Snapshot the modifier stack before applying: `modifier_apply` removes the
    // applied modifier from the live collection, so iterating it directly would
    // skip entries — mirrors the original's `for mod in list(obj.modifiers)`.
    let mut snapshot: Vec<(String, String)> = Vec::new();
    for m in PyIterator::from_object(&obj.getattr("modifiers")?)? {
        let m = m?;
        let mtype: String = m.getattr("type")?.extract()?;
        let mname: String = m.getattr("name")?.extract()?;
        snapshot.push((mname, mtype));
    }

    let mut applied = Vec::new();
    for (mname, mtype) in snapshot {
        if mtype != "BOOLEAN" {
            continue;
        }
        let kwargs = PyDict::new(py);
        kwargs.set_item("modifier", mname.as_str())?;
        // A failed apply is skipped and the batch continues, matching the
        // original's bare `except Exception: pass`. Checked with op_ok too:
        // verified live that a stale/gone modifier name does not raise, it
        // returns {'CANCELLED'} silently, which a bare `.is_ok()` would miss.
        if bpy
            .getattr("ops")?
            .getattr("object")?
            .call_method("modifier_apply", (), Some(&kwargs))
            .and_then(|r| op_ok(&r, "apply modifier"))
            .is_ok()
        {
            applied.push(mname);
        }
    }
    Ok(json!({ "name": name, "applied": applied }))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UtilApplyBooleans {
    /// Single mode: name of one object. Ignored when `names` is given; exactly
    /// one of `name`/`names` is required.
    pub name: Option<String>,
    /// Batch mode: object names to process in one call. Takes precedence over
    /// `name`. A missing name becomes a per-item `{"error": ...}` in `results`.
    pub names: Option<Vec<String>>,
}

crate::tool!(
    "util_apply_booleans",
    "Bake every BOOLEAN modifier already on the object(s) into the mesh, in stack \
     order, leaving other modifier types untouched. Single: {name}. Batch: \
     {names: [...]}. The applied names come back in `applied`; a modifier Blender \
     declines to apply is skipped silently. Use util_boolean to create one (it can \
     apply immediately), or mod_apply to apply a single named modifier of any type.",
    UtilApplyBooleans,
    |py: Python<'_>, p: UtilApplyBooleans| {
        let bpy = py.import("bpy")?;
        let objects = data(&bpy)?.getattr("objects")?;

        if let Some(names) = p.names {
            let results: Vec<Value> = names
                .iter()
                .map(|n| apply_booleans_one(py, &bpy, &objects, n))
                .collect::<PyResult<_>>()?;
            return Ok(json!({ "processed": results.len(), "results": results }));
        }
        let Some(name) = p.name else {
            return Err(PyValueError::new_err("provide 'name' or 'names'"));
        };
        require_ok(apply_booleans_one(py, &bpy, &objects, &name)?)
    }
);

// ============================================================================
// Asset Browser
// ============================================================================

/// Data-block type name -> matching `bpy.data` collection attribute. Order
/// matches the legacy handler's dict literal, which Python's insertion-ordered
/// dicts preserved for the `type == "ALL"` fan-out in `asset_list`.
const ASSET_TYPES: [(&str, &str); 5] = [
    ("OBJECT", "objects"),
    ("MATERIAL", "materials"),
    ("NODE_GROUP", "node_groups"),
    ("WORLD", "worlds"),
    ("ACTION", "actions"),
];

fn asset_collection<'py>(
    data: &Bound<'py, PyAny>,
    dtype: &str,
) -> PyResult<Option<Bound<'py, PyAny>>> {
    match ASSET_TYPES.iter().find(|(t, _)| *t == dtype) {
        Some((_, attr)) => Ok(Some(data.getattr(*attr)?)),
        None => Ok(None),
    }
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AssetMark {
    /// Name of the data-block to mark, inside the collection chosen by `type`.
    pub name: String,
    /// Which `bpy.data` collection `name` lives in. One of: OBJECT, MATERIAL,
    /// NODE_GROUP, WORLD, ACTION (case-insensitive). These five are the only
    /// types this tool set handles.
    pub r#type: String,
    /// Free text shown in the Asset Browser sidebar. Empty string is ignored.
    pub description: Option<String>,
    /// Author credit shown in the Asset Browser. Empty string is ignored.
    pub author: Option<String>,
    /// Tags to add for Asset Browser filtering. Added to whatever tags the
    /// data-block already has; this never removes tags.
    pub tags: Option<Vec<String>>,
}

crate::tool!(
    "asset_mark",
    "Mark a data-block (OBJECT, MATERIAL, NODE_GROUP, WORLD or ACTION) as an asset \
     so the Asset Browser lists it, with optional description/author/tags. Reverse \
     it with asset_unmark; see what is marked with asset_list.",
    AssetMark,
    |py: Python<'_>, p: AssetMark| {
        let bpy = py.import("bpy")?;
        let dtype = p.r#type.to_uppercase();
        let Some(collection) = asset_collection(&data(&bpy)?, &dtype)? else {
            return Err(PyValueError::new_err(format!("invalid type: {dtype}")));
        };
        let item = collection.call_method1("get", (p.name.as_str(),))?;
        if item.is_none() {
            return Err(PyValueError::new_err(format!(
                "{dtype} not found: {}",
                p.name
            )));
        }
        item.call_method0("asset_mark")?;
        let asset_data = item.getattr("asset_data")?;
        if let Some(desc) = p.description.filter(|s| !s.is_empty()) {
            asset_data.setattr("description", desc)?;
        }
        if let Some(author) = p.author.filter(|s| !s.is_empty()) {
            asset_data.setattr("author", author)?;
        }
        if let Some(tags) = p.tags {
            let tags_coll = asset_data.getattr("tags")?;
            for tag in tags {
                tags_coll.call_method1("new", (tag,))?;
            }
        }
        Ok(json!({ "name": p.name, "type": dtype, "marked": true }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AssetUnmark {
    /// Name of the data-block to unmark, inside the collection chosen by `type`.
    pub name: String,
    /// Which `bpy.data` collection `name` lives in. One of: OBJECT, MATERIAL,
    /// NODE_GROUP, WORLD, ACTION (case-insensitive).
    pub r#type: String,
}

crate::tool!(
    "asset_unmark",
    "Clear a data-block's asset marking (and its description/author/tags) so the \
     Asset Browser stops listing it. The data-block itself is kept. A data-block \
     that was not an asset is reported as `was_not_asset`, not an error. Inverse \
     of asset_mark.",
    AssetUnmark,
    |py: Python<'_>, p: AssetUnmark| {
        let bpy = py.import("bpy")?;
        let dtype = p.r#type.to_uppercase();
        let Some(collection) = asset_collection(&data(&bpy)?, &dtype)? else {
            return Err(PyValueError::new_err(format!("invalid type: {dtype}")));
        };
        let item = collection.call_method1("get", (p.name.as_str(),))?;
        if item.is_none() {
            return Err(PyValueError::new_err(format!(
                "{dtype} not found: {}",
                p.name
            )));
        }
        if !item.getattr("asset_data")?.is_none() {
            item.call_method0("asset_clear")?;
            return Ok(json!({ "name": p.name, "type": dtype, "unmarked": true }));
        }
        Ok(json!({ "name": p.name, "type": dtype, "was_not_asset": true }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AssetList {
    /// Restrict the scan to one data-block family. One of: OBJECT, MATERIAL,
    /// NODE_GROUP, WORLD, ACTION, or ALL to scan all five (case-insensitive).
    /// Default: ALL.
    pub r#type: Option<String>,
    // A `library` param used to be advertised and then ignored — it only ever scanned
    // the current file. A parameter the model can set and we drop is a lie the schema
    // tells; scanning another .blend is a real feature (a library load), not a filter,
    // so it is not advertised here. Use asset_import to pull from another file.
}

crate::tool!(
    "asset_list",
    "List the asset-marked data-blocks IN THE CURRENT FILE (name, type, \
     description, author, tags). It does not open other .blend files — to pull a \
     data-block out of another file use asset_import.",
    AssetList,
    |py: Python<'_>, p: AssetList| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let dtype = p.r#type.unwrap_or_else(|| "ALL".to_string()).to_uppercase();

        let wanted: Vec<(&str, &str)> = if dtype == "ALL" {
            ASSET_TYPES.to_vec()
        } else {
            match ASSET_TYPES.iter().find(|(t, _)| *t == dtype.as_str()) {
                Some(pair) => vec![*pair],
                None => return Err(PyValueError::new_err(format!("invalid type: {dtype}"))),
            }
        };

        let mut assets = Vec::new();
        for (type_name, attr) in wanted {
            for item in PyIterator::from_object(&d.getattr(attr)?)? {
                let item = item?;
                let asset_data = item.getattr("asset_data")?;
                if asset_data.is_none() {
                    continue;
                }
                let tags: Vec<String> = PyIterator::from_object(&asset_data.getattr("tags")?)?
                    .map(|t| t?.getattr("name")?.extract::<String>())
                    .collect::<PyResult<_>>()?;
                assets.push(json!({
                    "name": item.getattr("name")?.extract::<String>()?,
                    "type": type_name,
                    "description": asset_data.getattr("description")?.extract::<String>()?,
                    "author": asset_data.getattr("author")?.extract::<String>()?,
                    "tags": tags,
                }));
            }
        }
        let count = assets.len();
        Ok(json!({ "assets": assets, "count": count }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AssetImport {
    /// Absolute path to the source .blend file. Importing the currently open
    /// file into itself is refused (it crashes Blender natively).
    pub filepath: String,
    /// Exact data-block name inside that .blend, as it appears there.
    pub asset_name: String,
    /// Data-block family to look the name up in. One of: OBJECT, MATERIAL,
    /// NODE_GROUP, WORLD, ACTION (case-insensitive) — mapped to the RNA path
    /// segment Object/Material/NodeTree/World/Action.
    pub r#type: String,
    /// true: link (`wm.link`) — the data-block stays owned by the source file and
    /// is read-only here until library_override_create makes it editable.
    /// false: append (`wm.append`) — a full local copy, editable, with no link
    /// back. Default: false.
    pub link: Option<bool>,
}

crate::tool!(
    "asset_import",
    "Pull ONE named data-block out of another .blend file, either appended (a local \
     copy) or linked (read-only, still owned by that file). Use file_import for \
     interchange formats (fbx/obj/gltf/usd/abc) instead, and \
     library_override_create afterwards to make a linked object editable.",
    AssetImport,
    |py: Python<'_>, p: AssetImport| {
        let dtype = p.r#type.to_uppercase();
        // `wm.link`/`wm.append` address a data-block by filepath/directory/
        // filename, where `directory` is the .blend path plus the RNA type
        // name as a path segment (`Object`, `Material`, ...) — not a real
        // filesystem path, hence the literal backslash the original handler
        // builds it with (kept as-is; this tool is Windows-oriented already).
        let data_path = match dtype.as_str() {
            "OBJECT" => "Object",
            "MATERIAL" => "Material",
            "NODE_GROUP" => "NodeTree",
            "WORLD" => "World",
            "ACTION" => "Action",
            _ => return Err(PyValueError::new_err(format!("invalid type: {dtype}"))),
        };
        let directory = format!("{}\\{data_path}\\", p.filepath);
        let link = p.link.unwrap_or(false);

        let bpy = py.import("bpy")?;

        // Linking/appending a file into itself crashes Blender natively (an access
        // violation, not a Python exception) — a known Blender self-reference hazard.
        // Refuse it here rather than let a tool call take the whole process down.
        let current: String = bpy.getattr("data")?.getattr("filepath")?.extract()?;
        if !current.is_empty()
            && std::path::Path::new(&current) == std::path::Path::new(&p.filepath)
        {
            return Err(PyValueError::new_err(
                "cannot import a .blend into itself — save under a different path or open a \
                 different file first",
            ));
        }
        let kw = PyDict::new(py);
        kw.set_item("filepath", p.filepath.as_str())?;
        kw.set_item("directory", directory.as_str())?;
        kw.set_item("filename", p.asset_name.as_str())?;
        let wm = bpy.getattr("ops")?.getattr("wm")?;
        let call = if link {
            wm.call_method("link", (), Some(&kw))
        } else {
            wm.call_method("append", (), Some(&kw))
        };
        match call.and_then(|r| op_ok(&r, &format!("import asset '{}'", p.asset_name))) {
            Ok(()) => Ok(json!({ "imported": p.asset_name, "type": dtype, "linked": link })),
            // The `wm.link`/`wm.append` operator itself refused — a runtime
            // failure inside Blender, not a bad-input problem.
            Err(e) => Err(PyRuntimeError::new_err(format!(
                "failed to import asset '{}': {e}",
                p.asset_name
            ))),
        }
    }
);

// ============================================================================
// Video Sequence Editor (VSE)
// ============================================================================

/// The strip container: `.strips` on Blender >= 4.4, `.sequences` on <= 4.3.
fn vse_strips<'py>(se: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    se.getattr("strips").or_else(|_| se.getattr("sequences"))
}

/// The flat all-strips view: `.strips_all` on Blender >= 4.4, `.sequences_all`
/// on <= 4.3.
fn vse_strips_all<'py>(se: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    se.getattr("strips_all")
        .or_else(|_| se.getattr("sequences_all"))
}

/// `Strip.frame_start`, as a whole frame number.
///
/// Verified live on Blender 5.0: unlike every other frame-number RNA property
/// touched in this file (`Scene.frame_start`/`frame_end`, `frame_final_end`,
/// `frame_offset_start/end`), `Strip.frame_start` is a **float** property, not
/// an int — `type(strip.frame_start)` is `float`. A plain `.extract::<i32>()`
/// on it raises `TypeError: 'float' object cannot be interpreted as an
/// integer` (PyO3's message for a failed `__index__` call, which `float`
/// doesn't implement), so every read of it has to go through here instead.
fn strip_frame_start(strip: &Bound<'_, PyAny>) -> PyResult<i32> {
    Ok(strip.getattr("frame_start")?.extract::<f64>()? as i32)
}

fn strip_info(strip: &Bound<'_, PyAny>) -> PyResult<Value> {
    Ok(json!({
        "name": strip.getattr("name")?.extract::<String>()?,
        "type": strip.getattr("type")?.extract::<String>()?,
        "channel": strip.getattr("channel")?.extract::<i32>()?,
        "frame_start": strip_frame_start(strip)?,
        "frame_end": strip.getattr("frame_final_end")?.extract::<i32>()?,
    }))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VseSetup {
    /// true: remove every existing strip in the scene's sequence editor,
    /// including strips nested inside meta strips. Destructive and not undoable
    /// through this tool. Default: false.
    pub clear: Option<bool>,
}

crate::tool!(
    "vse_setup",
    "Create the current scene's sequence editor if it does not exist yet, and \
     optionally wipe every strip in it. vse_add_strip creates the editor on demand \
     too, so this is only needed to clear the timeline or to check the strip count \
     before building one.",
    VseSetup,
    |py: Python<'_>, p: VseSetup| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        if s.getattr("sequence_editor")?.is_none() {
            s.call_method0("sequence_editor_create")?;
        }
        let se = s.getattr("sequence_editor")?;

        if p.clear.unwrap_or(false) {
            let mut snapshot = Vec::new();
            for strip in PyIterator::from_object(&vse_strips_all(&se)?)? {
                snapshot.push(strip?);
            }
            let strips = vse_strips(&se)?;
            for strip in snapshot {
                strips.call_method1("remove", (strip,))?;
            }
        }

        let scene_name: String = s.getattr("name")?.extract()?;
        let count = py_len(py, &vse_strips_all(&se)?)?;
        Ok(json!({ "scene": scene_name, "strips": count }))
    }
);

/// `path`'s file stem (e.g. `/a/b/clip.mp4` -> `clip`), falling back to the
/// full path if it has no stem — matches the original's `Path(path).stem`.
fn stem_of(path: &str) -> String {
    std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
        .to_string()
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VseAddStrip {
    /// Kind of source strip to create. One of: IMAGE, MOVIE, SOUND (all three
    /// need `path`), COLOR, TEXT, SCENE (case-insensitive). These are the six
    /// this tool builds; effect/transition strips go through vse_add_effect.
    pub r#type: String,
    /// Absolute path to the image, video or audio file. Required for IMAGE,
    /// MOVIE and SOUND; ignored for the rest.
    pub path: Option<String>,
    /// Sequencer channel (row), 1-based. Higher channels composite over lower
    /// ones. Default: 1.
    pub channel: Option<i32>,
    /// Scene frame the strip starts on. Default: 1.
    pub frame_start: Option<i32>,
    /// Strip name. Defaults to the file stem for IMAGE/MOVIE/SOUND, "Color" /
    /// "Text" for those types, and the scene name for SCENE. Blender appends
    /// .001 etc. on a clash, so read the returned `name` back.
    pub name: Option<String>,
    /// COLOR strips only: [r, g, b] in linear scene-referred floats, 0.0-1.0.
    /// Missing components default to 0.5. Default: [0.5, 0.5, 0.5].
    pub color: Option<Vec<f64>>,
    /// TEXT strips only: the string to display. Default: "Text".
    pub text: Option<String>,
    /// Duration in frames, for the generated COLOR and TEXT strips (they have no
    /// source to derive a length from and Blender rejects 0). Default: 50.
    /// Ignored for IMAGE/MOVIE/SOUND and for SCENE (whose range comes from the
    /// target scene).
    pub length: Option<i32>,
    /// SCENE strips only: name of the scene to embed. Defaults to the current
    /// scene — note Blender cannot render a scene strip of the scene it is
    /// rendering.
    pub scene_name: Option<String>,
}

crate::tool!(
    "vse_add_strip",
    "Add one source strip to the sequencer — footage (IMAGE/MOVIE/SOUND), a \
     generated COLOR or TEXT card, or another SCENE — creating the sequence editor \
     if needed. Transitions and filters are vse_add_effect; tweak an existing strip \
     with vse_set_strip_props and render the timeline with vse_render.",
    VseAddStrip,
    |py: Python<'_>, p: VseAddStrip| {
        let strip_type = p.r#type.to_uppercase();
        let channel = p.channel.unwrap_or(1);
        let frame_start = p.frame_start.unwrap_or(1);

        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        if s.getattr("sequence_editor")?.is_none() {
            s.call_method0("sequence_editor_create")?;
        }
        let se = s.getattr("sequence_editor")?;
        let strips = vse_strips(&se)?;

        let strip = match strip_type.as_str() {
            "IMAGE" => {
                let Some(path) = &p.path else {
                    return Err(PyValueError::new_err("path required for IMAGE strip"));
                };
                let kw = PyDict::new(py);
                kw.set_item("name", p.name.clone().unwrap_or_else(|| stem_of(path)))?;
                kw.set_item("filepath", path.as_str())?;
                kw.set_item("channel", channel)?;
                kw.set_item("frame_start", frame_start)?;
                strips.call_method("new_image", (), Some(&kw))?
            }
            "MOVIE" => {
                let Some(path) = &p.path else {
                    return Err(PyValueError::new_err("path required for MOVIE strip"));
                };
                let kw = PyDict::new(py);
                kw.set_item("name", p.name.clone().unwrap_or_else(|| stem_of(path)))?;
                kw.set_item("filepath", path.as_str())?;
                kw.set_item("channel", channel)?;
                kw.set_item("frame_start", frame_start)?;
                strips.call_method("new_movie", (), Some(&kw))?
            }
            "SOUND" => {
                let Some(path) = &p.path else {
                    return Err(PyValueError::new_err("path required for SOUND strip"));
                };
                let kw = PyDict::new(py);
                kw.set_item("name", p.name.clone().unwrap_or_else(|| stem_of(path)))?;
                kw.set_item("filepath", path.as_str())?;
                kw.set_item("channel", channel)?;
                kw.set_item("frame_start", frame_start)?;
                strips.call_method("new_sound", (), Some(&kw))?
            }
            "COLOR" => {
                let color = p.color.clone().unwrap_or_else(|| vec![0.5, 0.5, 0.5]);
                let length = p.length.unwrap_or(50);
                let kw = PyDict::new(py);
                kw.set_item(
                    "name",
                    p.name.clone().unwrap_or_else(|| "Color".to_string()),
                )?;
                kw.set_item("type", "COLOR")?;
                kw.set_item("channel", channel)?;
                kw.set_item("frame_start", frame_start)?;
                // `StripsTopLevel.new_effect`'s real signature (verified live on
                // Blender 5.0) is `(name, type, channel, frame_start, length=0,
                // input1=None, input2=None)` — there is no `frame_end` keyword at
                // all, and a zero-input effect like COLOR needs a nonzero `length`
                // or Blender raises "invalid length" (it has no input strip to
                // derive a duration from).
                kw.set_item("length", length)?;
                let strip = strips.call_method("new_effect", (), Some(&kw))?;
                let c = [
                    color.first().copied().unwrap_or(0.5),
                    color.get(1).copied().unwrap_or(0.5),
                    color.get(2).copied().unwrap_or(0.5),
                ];
                strip.setattr("color", PyTuple::new(py, c)?)?;
                strip
            }
            "TEXT" => {
                let text = p.text.clone().unwrap_or_else(|| "Text".to_string());
                let length = p.length.unwrap_or(50);
                let kw = PyDict::new(py);
                kw.set_item("name", p.name.clone().unwrap_or_else(|| "Text".to_string()))?;
                kw.set_item("type", "TEXT")?;
                kw.set_item("channel", channel)?;
                kw.set_item("frame_start", frame_start)?;
                // Same `length`-not-`frame_end` signature as COLOR above.
                kw.set_item("length", length)?;
                let strip = strips.call_method("new_effect", (), Some(&kw))?;
                strip.setattr("text", text.as_str())?;
                strip
            }
            "SCENE" => {
                let current_name: String = s.getattr("name")?.extract()?;
                let scene_name = p.scene_name.clone().unwrap_or(current_name);
                let target = data(&bpy)?
                    .getattr("scenes")?
                    .call_method1("get", (scene_name.as_str(),))?;
                if target.is_none() {
                    return Err(PyValueError::new_err(format!(
                        "scene not found: {scene_name}"
                    )));
                }
                // `length` is accepted (matching the advertised schema) but,
                // like the original handler, never applied to a SCENE strip:
                // `new_scene` derives its frame range from the target scene
                // and the original computed `length` here without using it.
                let kw = PyDict::new(py);
                kw.set_item("name", p.name.clone().unwrap_or_else(|| scene_name.clone()))?;
                kw.set_item("scene", &target)?;
                kw.set_item("channel", channel)?;
                kw.set_item("frame_start", frame_start)?;
                strips.call_method("new_scene", (), Some(&kw))?
            }
            other => {
                return Err(PyValueError::new_err(format!(
                    "unknown strip type: {other}"
                )));
            }
        };

        strip_info(&strip)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VseAddEffect {
    /// Effect type. The full Blender 5.2 `new_effect` enum, verified live:
    /// two-input (both `seq1` and `seq2` required) — CROSS, ADD, SUBTRACT,
    /// ALPHA_OVER, ALPHA_UNDER, GAMMA_CROSS, MULTIPLY, WIPE, COLORMIX;
    /// one-input — GLOW, GAUSSIAN_BLUR, SPEED, COMPOSITOR;
    /// input-less — MULTICAM, ADJUSTMENT (and COLOR/TEXT, which vse_add_strip
    /// builds instead). Case-insensitive. TRANSFORM and OVER_DROP existed in
    /// Blender 4.x and are GONE in 5.2 — a transform is now done with
    /// vse_set_strip_props's `scale`/`position`/`rotation` on the strip itself.
    pub r#type: String,
    /// Name of the input strip (`input1`). Required, and must already exist.
    pub seq1: String,
    /// Name of the second input strip (`input2`), required by the two-input
    /// types listed above and ignored by the rest. When both inputs are given,
    /// the effect spans their combined frame range; otherwise it spans `seq1`.
    /// A name that does not exist is treated as if it were omitted.
    pub seq2: Option<String>,
    /// Sequencer channel for the new effect strip. Default: one above the
    /// highest channel of its inputs.
    pub channel: Option<i32>,
    /// Strip name. Defaults to the uppercased effect type. Blender appends .001
    /// etc. on a clash, so read the returned `name` back.
    pub name: Option<String>,
}

crate::tool!(
    "vse_add_effect",
    "Add an effect/transition strip on top of one or two existing strips (CROSS, \
     WIPE, ALPHA_OVER, GLOW, GAUSSIAN_BLUR, SPEED, ADJUSTMENT, ...). Source strips \
     come from vse_add_strip. On Blender 5.2 there is no TRANSFORM effect — scale, \
     position and rotation live on the strip itself, via vse_set_strip_props.",
    VseAddEffect,
    |py: Python<'_>, p: VseAddEffect| {
        // The legacy `type_map` here was a lowercase-key -> uppercase-value
        // table whose values were always just the uppercased key — i.e. a
        // no-op once `type` is uppercased (which it already is). Ported as a
        // plain `to_uppercase()`.
        let actual_type = p.r#type.to_uppercase();

        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        if s.getattr("sequence_editor")?.is_none() {
            return Err(PyValueError::new_err("no sequence editor"));
        }
        let se = s.getattr("sequence_editor")?;
        let strips = vse_strips(&se)?;

        let seq1 = strips.call_method1("get", (p.seq1.as_str(),))?;
        if seq1.is_none() {
            return Err(PyValueError::new_err(format!(
                "strip not found: {}",
                p.seq1
            )));
        }
        let seq2 = match &p.seq2 {
            Some(n) => {
                let s2 = strips.call_method1("get", (n.as_str(),))?;
                if s2.is_none() { None } else { Some(s2) }
            }
            None => None,
        };

        let seq1_channel: i32 = seq1.getattr("channel")?.extract()?;
        let seq2_channel = match &seq2 {
            Some(s2) => s2.getattr("channel")?.extract::<i32>()?,
            None => 0,
        };
        let channel = p.channel.unwrap_or(seq1_channel.max(seq2_channel) + 1);

        let seq1_start = strip_frame_start(&seq1)?;
        let seq1_end: i32 = seq1.getattr("frame_final_end")?.extract()?;

        // Verified live on Blender 5.2.2: these are exactly the `new_effect`
        // types that raise "effect takes 2 input strips" when handed only
        // `input1`. `OVER_DROP` used to be listed here but no longer exists in
        // the 5.2 enum (`('CROSS', 'ADD', 'SUBTRACT', 'ALPHA_OVER',
        // 'ALPHA_UNDER', 'GAMMA_CROSS', 'COMPOSITOR', 'MULTIPLY', 'WIPE',
        // 'GLOW', 'COLOR', 'SPEED', 'MULTICAM', 'ADJUSTMENT', 'GAUSSIAN_BLUR',
        // 'TEXT', 'COLORMIX')`), and neither does `TRANSFORM`.
        const TWO_INPUT: [&str; 9] = [
            "CROSS",
            "ADD",
            "SUBTRACT",
            "ALPHA_OVER",
            "ALPHA_UNDER",
            "GAMMA_CROSS",
            "MULTIPLY",
            "WIPE",
            "COLORMIX",
        ];

        let kw = PyDict::new(py);
        kw.set_item(
            "name",
            p.name.clone().unwrap_or_else(|| actual_type.clone()),
        )?;
        kw.set_item("type", actual_type.as_str())?;
        kw.set_item("channel", channel)?;
        // `StripsTopLevel.new_effect`'s real signature (verified live on Blender
        // 5.0) is `(name, type, channel, frame_start, length=0, input1=None,
        // input2=None)`: the input strips are `input1`/`input2`, not the pre-4.4
        // `seq1`/`seq2` this handler inherited, and there is no `frame_end`
        // keyword — only `length`, which must be positive.
        kw.set_item("input1", &seq1)?;

        if TWO_INPUT.contains(&actual_type.as_str())
            && let Some(s2) = &seq2
        {
            let seq2_start = strip_frame_start(s2)?;
            let seq2_end: i32 = s2.getattr("frame_final_end")?.extract()?;
            let start = seq1_start.min(seq2_start);
            let end = seq1_end.max(seq2_end);
            kw.set_item("frame_start", start)?;
            kw.set_item("length", (end - start).max(1))?;
            kw.set_item("input2", s2)?;
            let strip = strips.call_method("new_effect", (), Some(&kw))?;
            return strip_info(&strip);
        }
        kw.set_item("frame_start", seq1_start)?;
        kw.set_item("length", (seq1_end - seq1_start).max(1))?;
        let strip = strips.call_method("new_effect", (), Some(&kw))?;
        strip_info(&strip)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VseSetStripProps {
    /// Name of an existing top-level strip in the current scene's sequence
    /// editor (see vse_list_strips). Strips nested inside a meta are not found.
    pub name: String,
    /// How this strip composites over lower channels. Blender 5.2 enum, verified
    /// live: REPLACE, CROSS, DARKEN, MULTIPLY, BURN, LINEAR_BURN, LIGHTEN,
    /// SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, HARD_LIGHT, VIVID_LIGHT,
    /// LINEAR_LIGHT, PIN_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, HUE,
    /// SATURATION, COLOR, VALUE, ALPHA_OVER, ALPHA_UNDER, GAMMA_CROSS.
    /// Uppercased before use. Unchanged if omitted.
    pub blend_type: Option<String>,
    /// Opacity of the blend, 0.0 (invisible) to 1.0 (opaque).
    pub blend_alpha: Option<f64>,
    /// true disables the strip (it neither renders nor plays). Unchanged if
    /// omitted.
    pub mute: Option<bool>,
    /// Scene frame the strip's content begins at, as a whole frame number.
    /// Moves the strip along the timeline. NOTE: on Blender 5.2 this writes
    /// `Strip.frame_start`, which works but emits a DeprecationWarning — it is
    /// slated for removal in Blender 6.0, where the property is `content_start`.
    pub frame_start: Option<i32>,
    /// Frames trimmed off the START of the strip's source (the left handle), as
    /// a whole frame count; positive trims in. NOTE: writes the deprecated
    /// `Strip.frame_offset_start` on 5.2 (removed in 6.0, renamed
    /// `left_handle_offset`).
    pub frame_offset_start: Option<i32>,
    /// Frames trimmed off the END of the strip's source (the right handle), as a
    /// whole frame count; negative trims in. NOTE: writes the deprecated
    /// `Strip.frame_offset_end` on 5.2 (removed in 6.0, renamed
    /// `right_handle_offset`).
    pub frame_offset_end: Option<i32>,
    /// Sequencer channel (row), 1-based. Higher channels composite over lower.
    pub channel: Option<i32>,
    /// Playback rate multiplier: 1.0 is normal, 2.0 twice as fast. Only applied
    /// when the strip type actually has `speed_factor` (movie/sound); silently
    /// skipped otherwise.
    pub speed_factor: Option<f64>,
    /// Image transform scale, [x, y] as multipliers (1.0 = original size). A
    /// single element is reused for Y. Only applied to strips with an image
    /// transform (IMAGE/MOVIE/SCENE/...); silently skipped otherwise.
    pub scale: Option<Vec<f64>>,
    /// Image transform offset, [x, y] in PIXELS from centre. A missing Y
    /// defaults to 0. Only applied to strips with an image transform.
    pub position: Option<Vec<f64>>,
    /// Image transform rotation in RADIANS, counter-clockwise. Only applied to
    /// strips with an image transform.
    pub rotation: Option<f64>,
}

crate::tool!(
    "vse_set_strip_props",
    "Update an existing sequencer strip in place: blend mode/opacity, mute, \
     channel, timeline position and handle trims, playback speed, and the 2D \
     image transform (scale/position/rotation — Blender 5.2 has no TRANSFORM \
     effect strip, so this is where a transform lives). Only the fields you pass \
     change. Create strips with vse_add_strip / vse_add_effect, list them with \
     vse_list_strips.",
    VseSetStripProps,
    |py: Python<'_>, p: VseSetStripProps| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        if s.getattr("sequence_editor")?.is_none() {
            return Err(PyValueError::new_err("no sequence editor"));
        }
        let se = s.getattr("sequence_editor")?;
        let strip = vse_strips(&se)?.call_method1("get", (p.name.as_str(),))?;
        if strip.is_none() {
            return Err(PyValueError::new_err(format!(
                "strip not found: {}",
                p.name
            )));
        }

        if let Some(bt) = &p.blend_type {
            strip.setattr("blend_type", bt.to_uppercase())?;
        }
        if let Some(v) = p.blend_alpha {
            strip.setattr("blend_alpha", v)?;
        }
        if let Some(v) = p.mute {
            strip.setattr("mute", v)?;
        }
        if let Some(v) = p.frame_start {
            strip.setattr("frame_start", v)?;
        }
        if let Some(v) = p.frame_offset_start {
            strip.setattr("frame_offset_start", v)?;
        }
        if let Some(v) = p.frame_offset_end {
            strip.setattr("frame_offset_end", v)?;
        }
        if let Some(v) = p.channel {
            strip.setattr("channel", v)?;
        }
        if let Some(v) = p.speed_factor
            && strip.hasattr("speed_factor")?
        {
            strip.setattr("speed_factor", v)?;
        }
        if let Some(scale) = &p.scale
            && strip.hasattr("scale_start_x")?
            && !scale.is_empty()
        {
            let x = scale[0];
            // A single-element `scale` reuses X for Y. The legacy handler
            // instead assigned the *whole list* to `scale_start_y` in that
            // case, which Blender rejects (a float property, not a list) —
            // that is a latent bug there, not intended behaviour, so it is
            // not reproduced here.
            let y = scale.get(1).copied().unwrap_or(x);
            strip.setattr("scale_start_x", x)?;
            strip.setattr("scale_start_y", y)?;
        }
        if let Some(pos) = &p.position
            && strip.hasattr("translate_start_x")?
            && !pos.is_empty()
        {
            strip.setattr("translate_start_x", pos[0])?;
            strip.setattr("translate_start_y", pos.get(1).copied().unwrap_or(0.0))?;
        }
        if let Some(v) = p.rotation
            && strip.hasattr("rotation_start")?
        {
            strip.setattr("rotation_start", v)?;
        }

        Ok(json!({
            "name": strip.getattr("name")?.extract::<String>()?,
            "type": strip.getattr("type")?.extract::<String>()?,
            "blend_type": strip.getattr("blend_type")?.extract::<String>()?,
            "mute": strip.getattr("mute")?.extract::<bool>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VseDeleteStrip {
    /// Single mode: name of one top-level strip. Ignored when `names` is given.
    pub name: Option<String>,
    /// Batch mode: strip names to delete in one call. Takes precedence over
    /// `name`.
    pub names: Option<Vec<String>>,
}

crate::tool!(
    "vse_delete_strip",
    "Remove strip(s) from the current scene's sequencer by name. Single: {name}. \
     Batch: {names: [...]}. Names that do not exist are skipped, and the names \
     actually removed come back in `deleted` — to wipe the whole timeline use \
     vse_setup with clear:true instead.",
    VseDeleteStrip,
    |py: Python<'_>, p: VseDeleteStrip| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        // No sequence editor means no strips exist at all, so there is
        // nothing to delete — an empty result, not a failure: the rest of
        // this handler already treats an individually-missing strip the
        // same tolerant way (skipped, not reported as an error).
        if s.getattr("sequence_editor")?.is_none() {
            return Ok(json!({ "deleted": Vec::<String>::new() }));
        }
        let se = s.getattr("sequence_editor")?;
        let strips = vse_strips(&se)?;

        // Batch `names` wins over single `name`; if neither is given the
        // original built a `[None]` list and could error looking it up — here
        // that case is simply zero strips to delete.
        let names: Vec<String> = p.names.unwrap_or_else(|| p.name.into_iter().collect());
        let mut deleted = Vec::new();
        for name in names {
            let strip = strips.call_method1("get", (name.as_str(),))?;
            if !strip.is_none() {
                strips.call_method1("remove", (strip,))?;
                deleted.push(name);
            }
        }
        Ok(json!({ "deleted": deleted }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VseListStrips {}

crate::tool!(
    "vse_list_strips",
    "List every strip in the current scene's sequencer, including strips nested \
     inside meta strips, with name, type, channel, frame range, mute and blend \
     type. The source of the `name` the other vse_* tools take. Returns an empty \
     list when the scene has no sequence editor.",
    VseListStrips,
    |py: Python<'_>, _p: VseListStrips| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        if s.getattr("sequence_editor")?.is_none() {
            return Ok(json!({ "strips": [] }));
        }
        let se = s.getattr("sequence_editor")?;
        let mut strips = Vec::new();
        for strip in PyIterator::from_object(&vse_strips_all(&se)?)? {
            let strip = strip?;
            strips.push(json!({
                "name": strip.getattr("name")?.extract::<String>()?,
                "type": strip.getattr("type")?.extract::<String>()?,
                "channel": strip.getattr("channel")?.extract::<i32>()?,
                "frame_start": strip_frame_start(&strip)?,
                "frame_end": strip.getattr("frame_final_end")?.extract::<i32>()?,
                "mute": strip.getattr("mute")?.extract::<bool>()?,
                "blend_type": strip.getattr("blend_type")?.extract::<String>()?,
            }));
        }
        Ok(json!({ "strips": strips }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VseRender {
    /// Output path. For an image format this is a prefix — Blender appends the
    /// frame number and extension per frame; for FFMPEG it is the movie file.
    pub output: String,
    /// `ImageFormatSettings.file_format`. Blender 5.2 enum, verified live: PNG,
    /// JPEG, JPEG2000, AVIF, WEBP, BMP, IRIS, TARGA, TARGA_RAW, TIFF, CINEON,
    /// DPX, HDR, OPEN_EXR, OPEN_EXR_MULTILAYER, FFMPEG (the only one that
    /// writes a video file). Uppercased before use. Default: PNG.
    pub format: Option<String>,
    /// First scene frame to render. Default: the scene's current frame_start.
    /// Restored afterwards along with every other setting this tool touches.
    pub frame_start: Option<i32>,
    /// Last scene frame to render, inclusive. Default: the scene's current
    /// frame_end.
    pub frame_end: Option<i32>,
}

crate::tool!(
    "vse_render",
    "Render the sequencer timeline as an animation (image sequence or, with \
     format FFMPEG, a movie), then restore the scene's output path, format and \
     frame range. For a single 3D-viewport render of the scene itself use \
     scene_render instead.",
    VseRender,
    |py: Python<'_>, p: VseRender| {
        let fmt = p.format.unwrap_or_else(|| "PNG".to_string()).to_uppercase();
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        let render = s.getattr("render")?;

        let orig_output: String = render.getattr("filepath")?.extract()?;
        let orig_format: String = render
            .getattr("image_settings")?
            .getattr("file_format")?
            .extract()?;
        let orig_start: i32 = s.getattr("frame_start")?.extract()?;
        let orig_end: i32 = s.getattr("frame_end")?.extract()?;

        // Render settings are mutated in place for the duration of the render
        // and restored afterwards regardless of outcome (mirrors `scene_render`
        // in `misc.rs` and the original's `try/finally`).
        let out = (|| -> PyResult<Value> {
            render.setattr("filepath", p.output.as_str())?;
            render
                .getattr("image_settings")?
                .setattr("file_format", fmt.as_str())?;
            if let Some(fs) = p.frame_start {
                s.setattr("frame_start", fs)?;
            }
            if let Some(fe) = p.frame_end {
                s.setattr("frame_end", fe)?;
            }
            let kw = PyDict::new(py);
            kw.set_item("animation", true)?;
            let result =
                bpy.getattr("ops")?
                    .getattr("render")?
                    .call_method("render", (), Some(&kw))?;
            op_ok(&result, &format!("render VSE output to {}", p.output))?;
            Ok(json!({
                "output": p.output,
                "format": fmt,
                "frame_start": s.getattr("frame_start")?.extract::<i32>()?,
                "frame_end": s.getattr("frame_end")?.extract::<i32>()?,
            }))
        })();

        render.setattr("filepath", orig_output)?;
        render
            .getattr("image_settings")?
            .setattr("file_format", orig_format)?;
        s.setattr("frame_start", orig_start)?;
        s.setattr("frame_end", orig_end)?;

        out
    }
);
