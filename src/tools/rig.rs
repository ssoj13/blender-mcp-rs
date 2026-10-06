//! Constraint, driver and armature/rigging tools. Registered automatically —
//! nothing outside this file names them.
//!
//! Ported from `addon/mcp_server/__init__.py`'s `_h_con_*`, `_h_drv_*`,
//! `_h_anim_create_driver`/`_h_anim_copy_driver` (which back the `drv_create` /
//! `drv_copy` MCP names) and `_h_rig_*` handlers.
//!
//! ## Blender 5 driver storage
//! Drivers live on `object.animation_data.drivers` (an `FCurve` collection distinct
//! from `Action.fcurves`), which Blender 5.0 kept even though it removed the flat
//! `Action.fcurves` attribute. `drv_copy` therefore reads
//! `source.animation_data.drivers` directly and must **not** route through
//! `bpy::action_fcurves` — there is no action involved at all.
//!
//! ## `rig_create_biped`
//! Builds a full 19-bone edit-mode skeleton (root/spine/chest/neck/head, L+R
//! shoulder/upper_arm/forearm/hand, L+R thigh/shin/foot) from height-derived
//! proportions — not a stub or canned response.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyTuple};
use serde::Deserialize;
use serde_json::{Map, Value, json};

use super::bpy::{data, op_ok, py_len};

// ---------------------------------------------------------------------------
// shared helpers
// ---------------------------------------------------------------------------

/// Extract a Python float sequence into a `Vec<f64>`.
fn py_vec(seq: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    let mut v = Vec::new();
    for x in seq.try_iter()? {
        v.push(x?.extract::<f64>()?);
    }
    Ok(v)
}

/// Fills the leading `1..=3` components from `v` into `default`, matching the
/// original tools' tolerant handling of a location shorter than three floats.
fn vec3_or(v: Option<&[f64]>, default: [f64; 3]) -> [f64; 3] {
    let Some(a) = v else {
        return default;
    };
    let mut out = default;
    for (i, x) in a.iter().take(3).enumerate() {
        out[i] = *x;
    }
    out
}

fn set_vec(py: Python<'_>, obj: &Bound<'_, PyAny>, attr: &str, v: &[f64]) -> PyResult<()> {
    obj.setattr(attr, PyTuple::new(py, v.to_vec())?)
}

fn set_active(bpy: &Bound<'_, PyAny>, obj: &Bound<'_, PyAny>) -> PyResult<()> {
    bpy.getattr("context")?
        .getattr("view_layer")?
        .getattr("objects")?
        .setattr("active", obj)
}

/// Not checked with `op_ok`: verified live against Blender 5.0 that setting a
/// mode already active returns `{'FINISHED'}`, never a silent `{'CANCELLED'}`;
/// an incompatible mode/object pairing raises instead, which `?` propagates.
fn set_mode(py: Python<'_>, bpy: &Bound<'_, PyAny>, mode: &str) -> PyResult<()> {
    let kwargs = PyDict::new(py);
    kwargs.set_item("mode", mode)?;
    bpy.getattr("ops")?
        .getattr("object")?
        .call_method("mode_set", (), Some(&kwargs))?;
    Ok(())
}

/// Look up an armature object by name, erroring with the message the legacy
/// handlers used (`"Armature not found: {name}"`) if missing or of the wrong type.
fn get_armature<'py>(d: &Bound<'py, PyAny>, name: &str) -> PyResult<Bound<'py, PyAny>> {
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    if obj.is_none() || obj.getattr("type")?.extract::<String>()? != "ARMATURE" {
        return Err(PyValueError::new_err(format!("Armature not found: {name}")));
    }
    Ok(obj)
}

/// Create one edit-mode bone with head/tail/parent/connect already applied.
fn add_edit_bone<'py>(
    py: Python<'py>,
    edit_bones: &Bound<'py, PyAny>,
    name: &str,
    head: [f64; 3],
    tail: [f64; 3],
    parent: Option<&Bound<'py, PyAny>>,
    connect: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let bone = edit_bones.call_method1("new", (name,))?;
    set_vec(py, &bone, "head", &head)?;
    set_vec(py, &bone, "tail", &tail)?;
    if let Some(p) = parent {
        bone.setattr("parent", p)?;
        if connect {
            bone.setattr("use_connect", true)?;
        }
    }
    Ok(bone)
}

// ---------------------------------------------------------------------------
// Constraints — on objects AND on pose bones
// ---------------------------------------------------------------------------
//
// The old tools could only constrain an OBJECT, and only through a fixed list of
// hand-picked fields (`use_x`, `track_axis`, `min_x`…). That meant the actual content
// of a rig — Copy Rotation, Damped Track, Limit Rotation, Stretch To — was unreachable,
// and so was targeting a bone at all (no `subtarget`, so not even "parent the sword to
// the hand bone").
//
// Now: a constraint goes on an object or on a pose bone (`bone`), it can target a bone
// on the target armature (`subtarget`), and every other setting goes through a generic
// `properties` map applied by RNA. Blender has 29 constraint types with wildly
// different settings; enumerating their fields by hand is how the old code got stuck.

/// The thing a constraint hangs off: an object, or one of its pose bones.
///
/// Returns the constraint collection plus a label for error messages.
fn constraint_owner<'py>(
    d: &Bound<'py, PyAny>,
    object: &str,
    bone: Option<&str>,
) -> PyResult<(Bound<'py, PyAny>, String)> {
    let obj = d.getattr("objects")?.call_method1("get", (object,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {object}")));
    }

    let Some(bone) = bone else {
        return Ok((obj.getattr("constraints")?, object.to_string()));
    };

    let pose = obj.getattr("pose")?;
    if pose.is_none() {
        return Err(PyValueError::new_err(format!(
            "{object} is not an armature, so it has no bone `{bone}`"
        )));
    }
    let bones = pose.getattr("bones")?;
    let pb = bones.call_method1("get", (bone,))?;
    if pb.is_none() {
        let mut have = Vec::new();
        for b in bones.try_iter()? {
            have.push(b?.getattr("name")?.extract::<String>()?);
        }
        return Err(PyValueError::new_err(format!(
            "{object} has no bone `{bone}`. Its bones: {}",
            have.join(", ")
        )));
    }
    Ok((pb.getattr("constraints")?, format!("{object}/{bone}")))
}

/// Apply a `properties` map onto a constraint through RNA.
///
/// A constraint's settings differ completely by type, so they cannot be a fixed struct.
/// An unknown key is an error naming the settings this constraint actually has —
/// otherwise a typo silently does nothing and the rig quietly misbehaves.
fn apply_con_props(con: &Bound<'_, PyAny>, props: Option<&Map<String, Value>>) -> PyResult<()> {
    let Some(props) = props else {
        return Ok(());
    };
    for (k, v) in props {
        if !con.hasattr(k.as_str())? {
            let ty: String = con.getattr("type")?.extract()?;
            let mut have = Vec::new();
            let base: std::collections::HashSet<String> = {
                let py = con.py();
                let b = py.import("bpy")?.getattr("types")?.getattr("Constraint")?;
                let mut s = std::collections::HashSet::new();
                for p in b.getattr("bl_rna")?.getattr("properties")?.try_iter()? {
                    s.insert(p?.getattr("identifier")?.extract::<String>()?);
                }
                s
            };
            for p in con.getattr("bl_rna")?.getattr("properties")?.try_iter()? {
                let p = p?;
                let id: String = p.getattr("identifier")?.extract()?;
                if !base.contains(&id) && !p.getattr("is_readonly")?.extract::<bool>()? {
                    have.push(id);
                }
            }
            return Err(PyValueError::new_err(format!(
                "a {ty} constraint has no setting `{k}`. Its own settings: {}",
                have.join(", ")
            )));
        }
        super::bpy::set_rna(con, k, v)?;
    }
    Ok(())
}

fn con_info(con: &Bound<'_, PyAny>) -> PyResult<Value> {
    let target = con.getattr("target").ok().filter(|t| !t.is_none());
    let target_name = match &target {
        Some(t) => Some(t.getattr("name")?.extract::<String>()?),
        None => None,
    };
    Ok(json!({
        "name": con.getattr("name")?.extract::<String>()?,
        "type": con.getattr("type")?.extract::<String>()?,
        "target": target_name,
        "subtarget": con.getattr("subtarget").ok()
            .and_then(|s| s.extract::<String>().ok())
            .filter(|s| !s.is_empty()),
        "influence": con.getattr("influence").ok().and_then(|i| i.extract::<f64>().ok()),
        "mute": con.getattr("mute")?.extract::<bool>().unwrap_or(false),
    }))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ConAdd {
    /// Object to constrain.
    pub object: String,
    /// Pose bone to constrain. Omit to constrain the object itself. This is what makes
    /// a rig: Copy Rotation, Damped Track, Limit Rotation and friends live on bones.
    pub bone: Option<String>,
    /// Constraint type. Blender 5.2 has exactly these 29 (case is normalised for
    /// you, and a wrong value is answered with the live list):
    /// CAMERA_SOLVER, FOLLOW_TRACK, OBJECT_SOLVER, COPY_LOCATION, COPY_ROTATION,
    /// COPY_SCALE, COPY_TRANSFORMS, LIMIT_DISTANCE, LIMIT_LOCATION, LIMIT_ROTATION,
    /// LIMIT_SCALE, MAINTAIN_VOLUME, TRANSFORM, TRANSFORM_CACHE, CLAMP_TO,
    /// DAMPED_TRACK, IK, LOCKED_TRACK, SPLINE_IK, STRETCH_TO, TRACK_TO, ACTION,
    /// ARMATURE, CHILD_OF, FLOOR, FOLLOW_PATH, GEOMETRY_ATTRIBUTE, PIVOT, SHRINKWRAP.
    /// IK is usually easier through rig_add_ik, which also wires the pole target.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Name for the constraint, used later by con_update / con_remove. Default:
    /// Blender's own name for that type ("Copy Rotation", "IK", ...).
    pub con_name: Option<String>,
    /// Name of the object the constraint aims at, as in `bpy.data.objects`.
    /// Default: no target (most constraint types then do nothing).
    pub target: Option<String>,
    /// Bone on the target armature. This is how you parent a sword to a hand bone.
    pub subtarget: Option<String>,
    /// Every other setting, by its Blender name — influence, use_x, track_axis,
    /// min_x, head_tail, … They differ completely per constraint type; use the error
    /// message from a wrong key to discover the real ones.
    pub properties: Option<Map<String, Value>>,
}

crate::tool!(
    "con_add",
    "Add a constraint to an object or a POSE BONE. Type-specific settings go in `properties`; a wrong key is answered with the settings that constraint really has.",
    ConAdd,
    |py: Python<'_>, p: ConAdd| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let (constraints, owner) = constraint_owner(&d, &p.object, p.bone.as_deref())?;

        let ty = p.r#type.to_uppercase();
        let con = constraints
            .call_method1("new", (ty.as_str(),))
            .map_err(|e| {
                // Blender's own error is unhelpful; the enum is the useful answer.
                let listing = constraint_types(py)
                    .map(|v| v.join(", "))
                    .unwrap_or_else(|_| format!("{e}"));
                PyValueError::new_err(format!("unknown constraint type `{ty}`. Valid: {listing}"))
            })?;

        // The constraint exists in the rig the moment `new()` returns, so anything that
        // fails after this point must take it back out again. A failed call that leaves
        // a half-configured constraint behind is worse than one that does nothing: the
        // rig now misbehaves and nothing says why.
        let configure = (|| -> PyResult<()> {
            if let Some(n) = &p.con_name {
                con.setattr("name", n.as_str())?;
            }
            if let Some(t) = &p.target {
                let target = d.getattr("objects")?.call_method1("get", (t.as_str(),))?;
                if target.is_none() {
                    return Err(PyValueError::new_err(format!(
                        "target object not found: {t}"
                    )));
                }
                con.setattr("target", target)?;
            }
            if let Some(sub) = &p.subtarget {
                if !con.hasattr("subtarget")? {
                    return Err(PyValueError::new_err(format!(
                        "a {ty} constraint has no subtarget (it cannot aim at a bone)"
                    )));
                }
                con.setattr("subtarget", sub.as_str())?;
            }
            apply_con_props(&con, p.properties.as_ref())
        })();

        if let Err(e) = configure {
            constraints.call_method1("remove", (&con,))?;
            return Err(e);
        }

        Ok(json!({ "owner": owner, "constraint": con_info(&con)? }))
    }
);

/// Every constraint type this Blender knows, from RNA.
fn constraint_types(py: Python<'_>) -> PyResult<Vec<String>> {
    let prop = py
        .import("bpy")?
        .getattr("types")?
        .getattr("Constraint")?
        .getattr("bl_rna")?
        .getattr("properties")?
        .call_method1("get", ("type",))?;
    let mut out = Vec::new();
    for item in prop.getattr("enum_items")?.try_iter()? {
        out.push(item?.getattr("identifier")?.extract::<String>()?);
    }
    Ok(out)
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ConList {
    /// Object, as in `bpy.data.objects` (see obj_list).
    pub object: String,
    /// Pose bone whose constraints to list. Omit for the object's own constraints.
    pub bone: Option<String>,
}

crate::tool!(
    "con_list",
    "The constraints on an object or a pose bone: name, type, target, subtarget, \
     influence and mute. Read-only, and an empty list is a valid answer. These names \
     are what con_update and con_remove take.",
    ConList,
    |py: Python<'_>, p: ConList| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let (constraints, owner) = constraint_owner(&d, &p.object, p.bone.as_deref())?;
        let mut out = Vec::new();
        for c in constraints.try_iter()? {
            out.push(con_info(&c?)?);
        }
        Ok(json!({ "owner": owner, "constraints": out }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ConUpdate {
    /// Object that carries the constraint, as in `bpy.data.objects` (see obj_list).
    pub object: String,
    /// Pose bone that carries the constraint. Omit for the object's own constraints.
    pub bone: Option<String>,
    /// Constraint name (see con_list).
    pub con_name: String,
    /// New target object name. Default: unchanged.
    pub target: Option<String>,
    /// New bone on the target armature. Default: unchanged.
    pub subtarget: Option<String>,
    /// Any other setting, by its Blender name (influence 0.0–1.0, use_x, track_axis,
    /// min_x, head_tail, ...). A key this constraint type does not have is answered
    /// with the settings it really has. Default: unchanged.
    pub properties: Option<Map<String, Value>>,
}

crate::tool!(
    "con_update",
    "Change an existing constraint's settings, on an object or a pose bone. Omitted \
     fields are left alone, and a `properties` key the type does not have is answered \
     with the ones it really has. Find the name with con_list; create with con_add.",
    ConUpdate,
    |py: Python<'_>, p: ConUpdate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let (constraints, owner) = constraint_owner(&d, &p.object, p.bone.as_deref())?;

        let con = constraints.call_method1("get", (p.con_name.as_str(),))?;
        if con.is_none() {
            let mut have = Vec::new();
            for c in constraints.try_iter()? {
                have.push(c?.getattr("name")?.extract::<String>()?);
            }
            return Err(PyValueError::new_err(format!(
                "{owner} has no constraint `{}`. It has: {}",
                p.con_name,
                have.join(", ")
            )));
        }

        if let Some(t) = &p.target {
            let target = d.getattr("objects")?.call_method1("get", (t.as_str(),))?;
            if target.is_none() {
                return Err(PyValueError::new_err(format!(
                    "target object not found: {t}"
                )));
            }
            con.setattr("target", target)?;
        }
        if let Some(sub) = &p.subtarget {
            con.setattr("subtarget", sub.as_str())?;
        }
        apply_con_props(&con, p.properties.as_ref())?;

        Ok(json!({ "owner": owner, "constraint": con_info(&con)? }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ConRemove {
    /// Object, as in `bpy.data.objects` (see obj_list).
    pub object: String,
    /// Pose bone to remove constraints from. Omit for the object's own constraints.
    pub bone: Option<String>,
    /// Constraint names to remove (see con_list). A name that is not there is
    /// reported inside `results` rather than failing the whole call.
    pub names: Vec<String>,
}

crate::tool!(
    "con_remove",
    "Remove named constraints from an object or a pose bone (see con_list for the \
     names). Missing names come back per-item in `results` rather than failing the \
     call. To disable a constraint instead, set `mute: true` via con_update.",
    ConRemove,
    |py: Python<'_>, p: ConRemove| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let (constraints, owner) = constraint_owner(&d, &p.object, p.bone.as_deref())?;

        let mut results = Vec::new();
        for name in &p.names {
            let con = constraints.call_method1("get", (name.as_str(),))?;
            if con.is_none() {
                // Per-item failure inside a batch is data: the other removals worked.
                results.push(json!({ "name": name, "error": "not found" }));
                continue;
            }
            constraints.call_method1("remove", (&con,))?;
            results.push(json!({ "name": name, "removed": true }));
        }
        let removed = results.iter().filter(|r| r.get("error").is_none()).count();
        Ok(json!({ "owner": owner, "removed": removed, "results": results }))
    }
);

// ---------------------------------------------------------------------------
// drv_add
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct DrvVar {
    /// Variable name as used inside the driver `expression` — a valid Python
    /// identifier.
    pub name: String,
    /// Variable kind. Blender 5.2 accepts exactly: SINGLE_PROP (read one RNA
    /// property), TRANSFORMS (one transform channel of an object/bone),
    /// ROTATION_DIFF (angle between two bones), LOC_DIFF (distance between two
    /// targets), CONTEXT_PROP (a scene/view-layer property). Default here:
    /// SINGLE_PROP, which is also Blender's default.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Object this variable reads from, as in `bpy.data.objects`. Default: no id set
    /// (the variable stays unresolved).
    pub target: Option<String>,
    /// RNA path read on `target`, e.g. `location.x`, `scale[1]`, `["hp"]` for a
    /// custom property (prop_set). Default: `location.x`.
    pub data_path: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct DrvAdd {
    /// Object to drive, as in `bpy.data.objects` (see obj_list).
    pub name: String,
    /// RNA path of the driven property: `location`, `scale`, `rotation_euler`
    /// (RADIANS), `["hp"]` for a custom property (prop_set), ...
    pub data_path: String,
    /// Array component to drive: 0=X, 1=Y, 2=Z. Default: -1, meaning EVERY component
    /// of an array property — each resulting F-Curve gets the same driver. This
    /// array handling is the difference from drv_create.
    pub index: Option<i32>,
    /// How the variables are combined. Blender 5.2 accepts exactly: AVERAGE (its own
    /// default), SUM, SCRIPTED, MIN, MAX. Default here: SCRIPTED, so `expression` is
    /// what drives the value.
    pub driver_type: Option<String>,
    /// Python expression evaluated for a SCRIPTED driver, using the variable names
    /// below (e.g. `var * 2`). Default: Blender's empty expression.
    pub expression: Option<String>,
    /// Driver variables, `[{name, type?, target?, data_path?}, ...]`. Default: none,
    /// so the expression can only use constants.
    pub variables: Option<Vec<DrvVar>>,
}

crate::tool!(
    "drv_add",
    "Add a driver to an object property. This is the ARRAY-capable one: with `index` \
     left at its -1 default it drives every component of location/scale/ \
     rotation_euler at once, and its variables are SINGLE_PROP (target object + RNA \
     data_path) with a SCRIPTED expression. Use drv_create instead when the variable \
     should read a TRANSFORM channel — but that one drives a single curve and needs \
     an explicit prop_index on an array property. Remove with drv_remove.",
    DrvAdd,
    |py: Python<'_>, p: DrvAdd| {
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

        let idx = p.index.unwrap_or(-1);
        let added = obj.call_method1("driver_add", (p.data_path.as_str(), idx))?;

        // `driver_add(path, -1)` returns a *list* of FCurves when the property is an
        // array (e.g. "location") and a single FCurve otherwise. The legacy Python
        // handler assumed a single FCurve unconditionally and would raise
        // `AttributeError: 'list' object has no attribute 'driver'` for any vector
        // property left at the default index — an invalid-for-this-shape API call.
        // Handle both return shapes and configure every resulting F-Curve the same
        // way, which matches the tool description's "-1 for all" intent.
        let fcurves: Vec<Bound<'_, PyAny>> = if added.hasattr("driver")? {
            vec![added]
        } else {
            let mut v = Vec::new();
            for fc in added.try_iter()? {
                v.push(fc?);
            }
            v
        };
        if fcurves.is_empty() {
            return Err(PyValueError::new_err(format!(
                "driver_add produced no F-Curve for {}",
                p.data_path
            )));
        }

        let mut expression = String::new();
        for fc in &fcurves {
            let driver = fc.getattr("driver")?;
            driver.setattr("type", p.driver_type.as_deref().unwrap_or("SCRIPTED"))?;
            if let Some(expr) = p.expression.as_deref() {
                driver.setattr("expression", expr)?;
            }
            for var_def in p.variables.iter().flatten() {
                let var = driver.getattr("variables")?.call_method0("new")?;
                var.setattr("name", var_def.name.as_str())?;
                var.setattr("type", var_def.r#type.as_deref().unwrap_or("SINGLE_PROP"))?;
                let target = var.getattr("targets")?.get_item(0)?;
                if let Some(t) = var_def.target.as_deref() {
                    target.setattr("id", d.getattr("objects")?.call_method1("get", (t,))?)?;
                }
                target.setattr(
                    "data_path",
                    var_def.data_path.as_deref().unwrap_or("location.x"),
                )?;
            }
            expression = driver.getattr("expression")?.extract()?;
        }

        let on: String = obj.getattr("name")?.extract()?;
        Ok(json!({ "name": on, "data_path": p.data_path, "expression": expression }))
    }
);

// ---------------------------------------------------------------------------
// drv_remove
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct DrvRemove {
    /// Object whose driver is removed, as in `bpy.data.objects` (see obj_list).
    pub name: String,
    /// RNA path of the driven property, exactly as it was driven, e.g. `location`.
    pub data_path: String,
    /// Array component: 0=X, 1=Y, 2=Z. Default: -1, i.e. every component of an array
    /// property.
    pub index: Option<i32>,
}

crate::tool!(
    "drv_remove",
    "Remove the driver from one property (all components by default), leaving the \
     rest of the object's animation alone — anim_clear is the one that wipes \
     everything. Works on drivers made by drv_add, drv_create or drv_copy alike.",
    DrvRemove,
    |py: Python<'_>, p: DrvRemove| {
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
        let idx = p.index.unwrap_or(-1);
        obj.call_method1("driver_remove", (p.data_path.as_str(), idx))?;
        let on: String = obj.getattr("name")?.extract()?;
        Ok(json!({ "name": on, "data_path": p.data_path, "removed": true }))
    }
);

// ---------------------------------------------------------------------------
// drv_create
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct DrvCreateVar {
    /// Variable name as used inside `expression` — a valid Python identifier.
    /// Default: `var`.
    pub name: Option<String>,
    /// Variable kind. Blender 5.2 accepts exactly: SINGLE_PROP, TRANSFORMS,
    /// ROTATION_DIFF, LOC_DIFF, CONTEXT_PROP. Default here: TRANSFORMS (Blender's own
    /// default is SINGLE_PROP). Only TRANSFORMS reads `transform_type`/`space`, and
    /// only SINGLE_PROP reads `source_prop`.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Object this variable reads from, as in `bpy.data.objects`. Default: the driven
    /// object itself (`target`).
    pub source: Option<String>,
    /// TRANSFORMS only: which channel to read. Blender 5.2 accepts exactly LOC_X,
    /// LOC_Y, LOC_Z, ROT_X, ROT_Y, ROT_Z, ROT_W, SCALE_X, SCALE_Y, SCALE_Z,
    /// SCALE_AVG. Rotation channels are reported in RADIANS. Default: LOC_X.
    pub transform_type: Option<String>,
    /// SINGLE_PROP only: RNA path read on `source`, e.g. `location[0]`, `["hp"]`.
    /// Default: `location[0]`.
    pub source_prop: Option<String>,
    /// TRANSFORMS only: the space the channel is measured in. Blender 5.2 accepts
    /// exactly WORLD_SPACE, TRANSFORM_SPACE, LOCAL_SPACE. Default: WORLD_SPACE.
    pub space: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct DrvCreate {
    /// Object to drive, as in `bpy.data.objects` (see obj_list).
    pub target: String,
    /// RNA path of the driven property: `location`, `scale`, `rotation_euler`
    /// (RADIANS), `["hp"]` for a custom property (prop_set), ...
    pub target_prop: String,
    /// Array component to drive: 0=X, 1=Y, 2=Z. **Required for an array property**
    /// (location, scale, rotation_euler): the default -1 makes Blender return one
    /// F-Curve per component and this tool, which configures exactly one, then fails.
    /// Use -1 only for a scalar property, or use drv_add to drive a whole array.
    pub prop_index: Option<i32>,
    /// Python expression using the variable names below. Giving one sets the driver
    /// type to SCRIPTED; leaving it empty sets AVERAGE instead.
    pub expression: Option<String>,
    /// Driver variables, `[{name?, type?, source?, transform_type?, source_prop?,
    /// space?}, ...]`. They default to TRANSFORMS/LOC_X/WORLD_SPACE on the driven
    /// object. Default: none.
    pub variables: Option<Vec<DrvCreateVar>>,
}

crate::tool!(
    "drv_create",
    "Create ONE driver whose variables default to reading an object's TRANSFORM \
     channel (transform_type + space) — the usual \"move A, rotate B\" rig wiring. \
     Because it configures a single F-Curve, an array property (location, scale, \
     rotation_euler) REQUIRES an explicit `prop_index` (0/1/2); drv_add is the one \
     that handles a whole array at index -1 and defaults its variables to \
     SINGLE_PROP. Copy an existing driver with drv_copy, remove with drv_remove.",
    DrvCreate,
    |py: Python<'_>, p: DrvCreate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.target.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Object not found: {}",
                p.target
            )));
        }

        let prop_index = p.prop_index.unwrap_or(-1);
        let fcurve = if prop_index >= 0 {
            obj.call_method1("driver_add", (p.target_prop.as_str(), prop_index))
        } else {
            obj.call_method1("driver_add", (p.target_prop.as_str(),))
        };
        // `driver_add` genuinely failing means Blender rejected the property path —
        // the operation was attempted and failed, not a bad-input mismatch.
        let fcurve = fcurve.map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to add driver: {e}"))
        })?;
        let driver = fcurve.getattr("driver")?;

        let expression = p.expression.filter(|s| !s.is_empty()).unwrap_or_default();
        if !expression.is_empty() {
            driver.setattr("type", "SCRIPTED")?;
            driver.setattr("expression", expression.as_str())?;
        } else {
            driver.setattr("type", "AVERAGE")?;
        }

        let mut var_names = Vec::new();
        for var_def in p.variables.iter().flatten() {
            let var = driver.getattr("variables")?.call_method0("new")?;
            let vname = var_def.name.as_deref().unwrap_or("var");
            var.setattr("name", vname)?;
            let vtype = var_def.r#type.as_deref().unwrap_or("TRANSFORMS");
            var.setattr("type", vtype)?;

            let target = var.getattr("targets")?.get_item(0)?;
            let source_name = var_def.source.as_deref().unwrap_or(p.target.as_str());
            let source_obj = d.getattr("objects")?.call_method1("get", (source_name,))?;
            if !source_obj.is_none() {
                target.setattr("id", &source_obj)?;
            }
            if vtype == "TRANSFORMS" {
                target.setattr(
                    "transform_type",
                    var_def.transform_type.as_deref().unwrap_or("LOC_X"),
                )?;
                target.setattr(
                    "transform_space",
                    var_def.space.as_deref().unwrap_or("WORLD_SPACE"),
                )?;
            } else if vtype == "SINGLE_PROP" {
                target.setattr(
                    "data_path",
                    var_def.source_prop.as_deref().unwrap_or("location[0]"),
                )?;
            }
            var_names.push(vname.to_string());
        }

        Ok(json!({
            "target": p.target,
            "property": p.target_prop,
            "expression": expression,
            "variables": var_names,
        }))
    }
);

// ---------------------------------------------------------------------------
// drv_copy
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct DrvCopy {
    /// Object that already carries the driver, as in `bpy.data.objects`.
    pub source: String,
    /// RNA path the source driver sits on, matched exactly against
    /// `animation_data.drivers[].data_path` — e.g. `location`, `["hp"]`.
    pub source_prop: String,
    /// Object to receive the copy, as in `bpy.data.objects`.
    pub target: String,
    /// RNA path to drive on the target. For an array property the copy lands on the
    /// same component the source driver used.
    pub target_prop: String,
}

crate::tool!(
    "drv_copy",
    "Duplicate an existing driver — type, expression and every variable with its \
     targets — onto another object's property, keeping the source's array component. \
     The source must already have a driver (drv_add / drv_create make one).",
    DrvCopy,
    |py: Python<'_>, p: DrvCopy| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let source = d
            .getattr("objects")?
            .call_method1("get", (p.source.as_str(),))?;
        let target_obj = d
            .getattr("objects")?
            .call_method1("get", (p.target.as_str(),))?;
        if source.is_none() {
            return Err(PyValueError::new_err(format!(
                "Source not found: {}",
                p.source
            )));
        }
        if target_obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Target not found: {}",
                p.target
            )));
        }

        // Drivers live on `animation_data.drivers` — a collection distinct from
        // `Action.fcurves` that Blender 5.0 kept even after removing the flat
        // action attribute. No action is involved in reading a driver.
        //
        // No animation data at all means there is definitively no driver to copy —
        // the request cannot be satisfied, so this is a real failure, not an empty
        // result (unlike a read-only query for existing keyframes/fcurves).
        let anim_data = source.getattr("animation_data")?;
        if anim_data.is_none() {
            return Err(PyValueError::new_err(format!(
                "No animation data on {}",
                p.source
            )));
        }

        let mut src_fc = None;
        for fc in anim_data.getattr("drivers")?.try_iter()? {
            let fc = fc?;
            let dp: String = fc.getattr("data_path")?.extract()?;
            if dp == p.source_prop {
                src_fc = Some(fc);
                break;
            }
        }
        let Some(src_fc) = src_fc else {
            return Err(PyValueError::new_err(format!(
                "Driver not found on {}",
                p.source_prop
            )));
        };
        let src_driver = src_fc.getattr("driver")?;
        let src_index: i64 = src_fc.getattr("array_index")?.extract()?;

        // `driver_add(path)` defaults to index -1: a single FCurve for a scalar
        // property, or a LIST of FCurves (one per component) for an array one —
        // the exact ambiguity `drv_add` already has to handle (see its comment).
        // Blindly reading `.driver` off the raw result crashes with `AttributeError:
        // 'list' object has no attribute 'driver'` for every vector property
        // (location, scale, rotation_euler, ...); verified live. Pick the same
        // component the source driver was on.
        let added = target_obj
            .call_method1("driver_add", (p.target_prop.as_str(),))
            .map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to add driver: {e}"))
            })?;
        let new_fc = if added.hasattr("driver")? {
            added
        } else {
            added.get_item(src_index)?
        };
        let new_driver = new_fc.getattr("driver")?;
        new_driver.setattr("type", src_driver.getattr("type")?)?;
        new_driver.setattr("expression", src_driver.getattr("expression")?)?;

        for src_var in src_driver.getattr("variables")?.try_iter()? {
            let src_var = src_var?;
            let new_var = new_driver.getattr("variables")?.call_method0("new")?;
            new_var.setattr("name", src_var.getattr("name")?)?;
            new_var.setattr("type", src_var.getattr("type")?)?;

            let src_targets = src_var.getattr("targets")?;
            let new_targets = new_var.getattr("targets")?;
            let n = py_len(py, &src_targets)?.min(py_len(py, &new_targets)?);
            for i in 0..n {
                let idx = i as i32;
                let st = src_targets.get_item(idx)?;
                let nt = new_targets.get_item(idx)?;
                nt.setattr("id", st.getattr("id")?)?;
                nt.setattr("data_path", st.getattr("data_path")?)?;
                if st.hasattr("transform_type")? {
                    nt.setattr("transform_type", st.getattr("transform_type")?)?;
                    nt.setattr("transform_space", st.getattr("transform_space")?)?;
                }
            }
        }

        Ok(json!({
            "source": format!("{}.{}", p.source, p.source_prop),
            "target": format!("{}.{}", p.target, p.target_prop),
            "copied": true,
        }))
    }
);

// ---------------------------------------------------------------------------
// rig_create_armature
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RigCreateArmature {
    /// Name for the armature object and its data. Default: `Armature`.
    pub name: Option<String>,
    /// `[x, y, z]` world location of the armature object, in metres. Default:
    /// `[0, 0, 0]`.
    pub location: Option<Vec<f64>>,
    /// Create a single bone named `Root` inside the new armature. Default: true.
    pub add_root_bone: Option<bool>,
    /// `[x, y, z]` head of that root bone, in armature-local metres. Default:
    /// `[0, 0, 0]`. Ignored when `add_root_bone` is false.
    pub root_head: Option<Vec<f64>>,
    /// `[x, y, z]` tail of that root bone, in armature-local metres. Default:
    /// `[0, 0, 1]`. Head and tail must differ or Blender discards the bone.
    pub root_tail: Option<Vec<f64>>,
}

crate::tool!(
    "rig_create_armature",
    "Create an empty armature object, optionally with one `Root` bone. Add more bones \
     with rig_add_bone, or skip straight to a ready 19-bone skeleton with \
     rig_create_biped. Bind a mesh to it with rig_auto_weights.",
    RigCreateArmature,
    |py: Python<'_>, p: RigCreateArmature| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let name = p.name.as_deref().unwrap_or("Armature");
        let loc = vec3_or(p.location.as_deref(), [0.0, 0.0, 0.0]);

        let arm_data = d.getattr("armatures")?.call_method1("new", (name,))?;
        let arm_data_name: String = arm_data.getattr("name")?.extract()?;
        let arm_obj = d
            .getattr("objects")?
            .call_method1("new", (arm_data_name.as_str(), &arm_data))?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&arm_obj,))?;
        set_vec(py, &arm_obj, "location", &loc)?;

        if p.add_root_bone.unwrap_or(true) {
            set_active(&bpy, &arm_obj)?;
            set_mode(py, &bpy, "EDIT")?;
            let edit_bones = arm_data.getattr("edit_bones")?;
            let head = vec3_or(p.root_head.as_deref(), [0.0, 0.0, 0.0]);
            let tail = vec3_or(p.root_tail.as_deref(), [0.0, 0.0, 1.0]);
            add_edit_bone(py, &edit_bones, "Root", head, tail, None, false)?;
            set_mode(py, &bpy, "OBJECT")?;
        }

        let bones = py_len(py, &arm_data.getattr("bones")?)?;
        let on: String = arm_obj.getattr("name")?.extract()?;
        Ok(json!({ "name": on, "bones": bones }))
    }
);

// ---------------------------------------------------------------------------
// rig_add_bone
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RigAddBone {
    /// Armature OBJECT name (not a bone name), as in `bpy.data.objects`.
    pub name: String,
    /// Name for the new bone. Blender appends `.001` on a collision.
    pub bone_name: String,
    /// `[x, y, z]` start of the bone, in armature-local metres. Default: `[0, 0, 0]`.
    pub head: Option<Vec<f64>>,
    /// `[x, y, z]` end of the bone, in armature-local metres. Default: `[0, 0, 1]`.
    /// A zero-length bone (head == tail) is discarded by Blender.
    pub tail: Option<Vec<f64>>,
    /// Existing bone in the same armature to parent to. A name that is not there is
    /// treated as no parent. Default: no parent.
    pub parent: Option<String>,
    /// Snap this bone's head to the parent's tail and lock them together
    /// (`use_connect`). Ignored without `parent`. Default: false.
    pub connected: Option<bool>,
    /// Roll of the bone about its own axis, in RADIANS (Blender's range is
    /// -2pi..2pi). Default: 0.0.
    pub roll: Option<f64>,
}

crate::tool!(
    "rig_add_bone",
    "Add one bone to an existing armature; the tool enters Edit mode and returns to \
     Object mode itself. Coordinates are armature-local metres and `roll` is in \
     radians. Reshape it later with rig_update_bone, pose it with rig_pose_bone, \
     delete it with rig_remove_bone.",
    RigAddBone,
    |py: Python<'_>, p: RigAddBone| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_armature(&d, &p.name)?;

        set_active(&bpy, &obj)?;
        set_mode(py, &bpy, "EDIT")?;

        let edit_bones = obj.getattr("data")?.getattr("edit_bones")?;
        let head = vec3_or(p.head.as_deref(), [0.0, 0.0, 0.0]);
        let tail = vec3_or(p.tail.as_deref(), [0.0, 0.0, 1.0]);

        let parent_bone = match p.parent.as_deref() {
            Some(pn) => {
                let b = edit_bones.call_method1("get", (pn,))?;
                if b.is_none() { None } else { Some(b) }
            }
            None => None,
        };
        let bone = add_edit_bone(
            py,
            &edit_bones,
            &p.bone_name,
            head,
            tail,
            parent_bone.as_ref(),
            p.connected.unwrap_or(false),
        )?;
        if let Some(r) = p.roll {
            bone.setattr("roll", r)?;
        }

        // The EditBone's name (and everything else about it) must be read BEFORE
        // switching back to Object mode: leaving Edit mode frees Blender's edit-bone
        // data, and this Python wrapper goes stale. Verified live on Blender 5.0 —
        // reading `.name` afterwards silently returns `""`, and in a busier scene
        // can return corrupted non-UTF8 bytes that blow up `.extract::<String>()`.
        let bn: String = bone.getattr("name")?.extract()?;
        set_mode(py, &bpy, "OBJECT")?;
        Ok(json!({ "armature": p.name, "bone": bn }))
    }
);

// ---------------------------------------------------------------------------
// rig_remove_bone
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RigRemoveBone {
    /// Armature OBJECT name (not a bone name), as in `bpy.data.objects`.
    pub name: String,
    /// Bone to delete. Its children lose their parent but survive.
    pub bone_name: String,
}

crate::tool!(
    "rig_remove_bone",
    "Delete one bone from an armature (via Edit mode, restored afterwards). A bone \
     that is not there is an error, not a silent no-op. Children are unparented, not \
     deleted. See rig_get_pose for the bone names.",
    RigRemoveBone,
    |py: Python<'_>, p: RigRemoveBone| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_armature(&d, &p.name)?;

        set_active(&bpy, &obj)?;
        set_mode(py, &bpy, "EDIT")?;

        let edit_bones = obj.getattr("data")?.getattr("edit_bones")?;
        let bone = edit_bones.call_method1("get", (p.bone_name.as_str(),))?;
        if bone.is_none() {
            set_mode(py, &bpy, "OBJECT")?;
            return Err(PyValueError::new_err(format!(
                "Bone not found: {}",
                p.bone_name
            )));
        }
        edit_bones.call_method1("remove", (&bone,))?;
        set_mode(py, &bpy, "OBJECT")?;

        Ok(json!({ "armature": p.name, "removed": p.bone_name }))
    }
);

// ---------------------------------------------------------------------------
// rig_update_bone
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RigUpdateBone {
    /// Armature OBJECT name (not a bone name), as in `bpy.data.objects`.
    pub name: String,
    /// Bone to reshape.
    pub bone_name: String,
    /// New `[x, y, z]` head, in armature-local metres. Default: unchanged.
    pub head: Option<Vec<f64>>,
    /// New `[x, y, z]` tail, in armature-local metres. Default: unchanged.
    pub tail: Option<Vec<f64>>,
    /// New roll about the bone's own axis, in RADIANS. Default: unchanged.
    pub roll: Option<f64>,
}

crate::tool!(
    "rig_update_bone",
    "Move a bone's REST shape — head, tail, roll — in Edit mode. This changes the \
     skeleton itself, not the pose: to animate a bone use rig_pose_bone (plus \
     anim_set_keyframe). Omitted fields are left alone; `roll` is in radians.",
    RigUpdateBone,
    |py: Python<'_>, p: RigUpdateBone| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_armature(&d, &p.name)?;

        set_active(&bpy, &obj)?;
        set_mode(py, &bpy, "EDIT")?;

        let edit_bones = obj.getattr("data")?.getattr("edit_bones")?;
        let bone = edit_bones.call_method1("get", (p.bone_name.as_str(),))?;
        if bone.is_none() {
            set_mode(py, &bpy, "OBJECT")?;
            return Err(PyValueError::new_err(format!(
                "Bone not found: {}",
                p.bone_name
            )));
        }

        if let Some(h) = p.head.as_deref() {
            set_vec(py, &bone, "head", h)?;
        }
        if let Some(t) = p.tail.as_deref() {
            set_vec(py, &bone, "tail", t)?;
        }
        if let Some(r) = p.roll {
            bone.setattr("roll", r)?;
        }

        // Same rule as `rig_add_bone`: an EditBone must be read before the object
        // leaves Edit mode, or the wrapper is reading freed memory (see that
        // function's comment — verified live, not a hypothetical).
        let bn: String = bone.getattr("name")?.extract()?;
        set_mode(py, &bpy, "OBJECT")?;
        Ok(json!({ "armature": p.name, "bone": bn }))
    }
);

// ---------------------------------------------------------------------------
// rig_pose_bone
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RigPoseBone {
    /// Armature OBJECT name (not a bone name), as in `bpy.data.objects`.
    pub name: String,
    /// Pose bone to move (see rig_get_pose for the names).
    pub bone_name: String,
    /// `[x, y, z]` pose offset from the bone's rest position, in bone-local metres.
    /// Default: unchanged.
    pub location: Option<Vec<f64>>,
    /// `[x, y, z]` Euler rotation in RADIANS. Setting it also switches the bone's
    /// `rotation_mode` to XYZ, because a pose bone in the default QUATERNION mode
    /// ignores `rotation_euler` entirely. Default: unchanged.
    pub rotation: Option<Vec<f64>>,
    /// `[w, x, y, z]` quaternion. Setting it switches `rotation_mode` to QUATERNION.
    /// Applied after `rotation`, so do not send both. Default: unchanged.
    pub rotation_quaternion: Option<Vec<f64>>,
    /// `[x, y, z]` scale factors; 1.0 is the rest size. Default: unchanged.
    pub scale: Option<Vec<f64>>,
}

crate::tool!(
    "rig_pose_bone",
    "Pose one bone: location, rotation (EULER RADIANS via `rotation`, or \
     `rotation_quaternion`), scale. Blender 5.2 pose bones support the rotation_mode \
     values QUATERNION (the default), XYZ, XZY, YXZ, YZX, ZXY, ZYX and AXIS_ANGLE; \
     this tool switches the mode to XYZ or QUATERNION to match the field you send, \
     since the other field would otherwise be silently ignored. Read the pose back \
     with rig_get_pose, key it with anim_set_keyframe.",
    RigPoseBone,
    |py: Python<'_>, p: RigPoseBone| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_armature(&d, &p.name)?;

        let pose_bone = obj
            .getattr("pose")?
            .getattr("bones")?
            .call_method1("get", (p.bone_name.as_str(),))?;
        if pose_bone.is_none() {
            return Err(PyValueError::new_err(format!(
                "Pose bone not found: {}",
                p.bone_name
            )));
        }

        if let Some(v) = p.location.as_deref() {
            set_vec(py, &pose_bone, "location", v)?;
        }
        // A pose bone reads whichever rotation field its `rotation_mode` selects, and
        // that mode defaults to QUATERNION. Writing `rotation_euler` without switching
        // the mode is silently ignored: the pose does not move, but the call would
        // still report success — a lie. So set the matching mode first.
        if let Some(v) = p.rotation.as_deref() {
            pose_bone.setattr("rotation_mode", "XYZ")?;
            set_vec(py, &pose_bone, "rotation_euler", v)?;
        }
        if let Some(v) = p.rotation_quaternion.as_deref() {
            pose_bone.setattr("rotation_mode", "QUATERNION")?;
            set_vec(py, &pose_bone, "rotation_quaternion", v)?;
        }
        if let Some(v) = p.scale.as_deref() {
            set_vec(py, &pose_bone, "scale", v)?;
        }

        let bn: String = pose_bone.getattr("name")?.extract()?;
        Ok(json!({ "armature": p.name, "bone": bn }))
    }
);

// ---------------------------------------------------------------------------
// rig_get_pose
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RigGetPose {
    /// Armature OBJECT name, as in `bpy.data.objects` (see obj_list).
    pub name: String,
}

crate::tool!(
    "rig_get_pose",
    "Every pose bone of an armature with its current location, rotation_euler \
     (RADIANS), rotation_quaternion and scale. Read-only, and it reports both \
     rotation forms regardless of the bone's rotation_mode. Write it back with \
     rig_pose_bone.",
    RigGetPose,
    |py: Python<'_>, p: RigGetPose| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_armature(&d, &p.name)?;

        let mut bones = Vec::new();
        for pb in obj.getattr("pose")?.getattr("bones")?.try_iter()? {
            let pb = pb?;
            bones.push(json!({
                "name": pb.getattr("name")?.extract::<String>()?,
                "location": py_vec(&pb.getattr("location")?)?,
                "rotation_euler": py_vec(&pb.getattr("rotation_euler")?)?,
                "rotation_quaternion": py_vec(&pb.getattr("rotation_quaternion")?)?,
                "scale": py_vec(&pb.getattr("scale")?)?,
            }));
        }

        Ok(json!({ "armature": p.name, "bones": bones }))
    }
);

// ---------------------------------------------------------------------------
// rig_add_ik
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RigAddIk {
    /// Armature OBJECT name, as in `bpy.data.objects`.
    pub name: String,
    /// Pose bone the IK constraint goes on — the LAST bone of the chain (the hand or
    /// foot), not its root. The solver then works back up the parents.
    pub bone_name: String,
    /// Name for the constraint, used later by con_update / con_remove. Default: `IK`.
    pub con_name: Option<String>,
    /// How many bones up the parent chain the solver may move; 0 means the whole
    /// chain to the root. Blender's range is 0–255 and its own default is 0. Default
    /// here: 2 (a typical arm or leg).
    pub chain_length: Option<i32>,
    /// Object the chain reaches for, as in `bpy.data.objects`. Default: none — an IK
    /// constraint without a target does nothing.
    pub target: Option<String>,
    /// Bone on `target` to reach for, when the target is an armature. Ignored
    /// without `target`.
    pub subtarget: Option<String>,
    /// Object used as the pole (knee/elbow direction hint). Default: none.
    pub pole_target: Option<String>,
    /// Bone on `pole_target`, when that is an armature. Ignored without
    /// `pole_target`.
    pub pole_subtarget: Option<String>,
    /// Rotation of the chain about the pole axis, in RADIANS (Blender's range is
    /// -pi..pi). Applied only when `pole_target` is set. Default: 0.0.
    pub pole_angle: Option<f64>,
}

crate::tool!(
    "rig_add_ik",
    "Add an IK constraint to a pose bone so its parent chain follows a target, with \
     an optional pole for the knee/elbow direction (`pole_angle` in RADIANS). IK is \
     only valid as a POSE BONE constraint, which is why this is separate from \
     con_add; tune it afterwards with con_update, drop it with con_remove.",
    RigAddIk,
    |py: Python<'_>, p: RigAddIk| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_armature(&d, &p.name)?;

        // IK (like SPLINE_IK etc.) is only valid as a *pose bone* constraint, not an
        // object constraint — the pose bone's own `.constraints` collection is used
        // below, never `obj.constraints`.
        let pose_bone = obj
            .getattr("pose")?
            .getattr("bones")?
            .call_method1("get", (p.bone_name.as_str(),))?;
        if pose_bone.is_none() {
            return Err(PyValueError::new_err(format!(
                "Pose bone not found: {}",
                p.bone_name
            )));
        }

        let con = pose_bone
            .getattr("constraints")?
            .call_method1("new", ("IK",))?;
        con.setattr("name", p.con_name.as_deref().unwrap_or("IK"))?;
        con.setattr("chain_count", p.chain_length.unwrap_or(2))?;

        if let Some(target) = p.target.as_deref() {
            con.setattr(
                "target",
                d.getattr("objects")?.call_method1("get", (target,))?,
            )?;
            if let Some(sub) = p.subtarget.as_deref() {
                con.setattr("subtarget", sub)?;
            }
        }
        if let Some(pole_target) = p.pole_target.as_deref() {
            con.setattr(
                "pole_target",
                d.getattr("objects")?.call_method1("get", (pole_target,))?,
            )?;
            if let Some(sub) = p.pole_subtarget.as_deref() {
                con.setattr("pole_subtarget", sub)?;
            }
            con.setattr("pole_angle", p.pole_angle.unwrap_or(0.0))?;
        }

        let bn: String = pose_bone.getattr("name")?.extract()?;
        let cn: String = con.getattr("name")?.extract()?;
        Ok(json!({ "armature": p.name, "bone": bn, "constraint": cn }))
    }
);

// ---------------------------------------------------------------------------
// rig_auto_weights
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RigAutoWeights {
    /// Mesh object to bind, as in `bpy.data.objects`. Must be of type MESH.
    pub mesh: String,
    /// Armature object to bind it to, as in `bpy.data.objects`.
    pub armature: String,
}

crate::tool!(
    "rig_auto_weights",
    "Parent a mesh to an armature with automatic weights \
     (`object.parent_set(type='ARMATURE_AUTO')`): it adds the Armature modifier and \
     generates one vertex group per deforming bone. Build the skeleton first \
     (rig_create_armature / rig_create_biped); tidy up symmetry with \
     rig_mirror_weights.",
    RigAutoWeights,
    |py: Python<'_>, p: RigAutoWeights| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let mesh_obj = d
            .getattr("objects")?
            .call_method1("get", (p.mesh.as_str(),))?;
        if mesh_obj.is_none() || mesh_obj.getattr("type")?.extract::<String>()? != "MESH" {
            return Err(PyValueError::new_err(format!("Mesh not found: {}", p.mesh)));
        }
        let arm_obj = get_armature(&d, &p.armature)?;

        let object_ops = bpy.getattr("ops")?.getattr("object")?;
        // Not checked: reset step, and `select_all(DESELECT)` legitimately
        // returns {'CANCELLED'} when nothing was selected to begin with
        // (verified live) — that is the goal already met, not a failure.
        let deselect = PyDict::new(py);
        deselect.set_item("action", "DESELECT")?;
        object_ops.call_method("select_all", (), Some(&deselect))?;

        mesh_obj.call_method1("select_set", (true,))?;
        arm_obj.call_method1("select_set", (true,))?;
        set_active(&bpy, &arm_obj)?;

        let parent_kwargs = PyDict::new(py);
        parent_kwargs.set_item("type", "ARMATURE_AUTO")?;
        let result = object_ops.call_method("parent_set", (), Some(&parent_kwargs))?;
        op_ok(
            &result,
            &format!("parent {} to {} with automatic weights", p.mesh, p.armature),
        )?;

        let mn: String = mesh_obj.getattr("name")?.extract()?;
        let an: String = arm_obj.getattr("name")?.extract()?;
        Ok(json!({ "mesh": mn, "armature": an, "parented": true }))
    }
);

// ---------------------------------------------------------------------------
// rig_create_biped
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RigCreateBiped {
    /// Name for the armature object and its data. Default: `Biped`.
    pub name: Option<String>,
    /// `[x, y, z]` world location of the armature object, in metres. Default:
    /// `[0, 0, 0]`.
    pub location: Option<Vec<f64>>,
    /// Total character height in metres; every bone proportion is derived from it
    /// (spine at 0.45h, chest 0.8h, neck 0.95h, arms 0.35h, legs 0.45h). Default: 2.0.
    pub height: Option<f64>,
}

crate::tool!(
    "rig_create_biped",
    "Create a 19-bone biped skeleton — root/spine/chest/neck/head plus mirrored \
     shoulder, upper_arm, forearm, hand, thigh, shin and foot — scaled from `height` \
     in metres. Add IK with rig_add_ik, bind a mesh with rig_auto_weights, adjust \
     individual bones with rig_update_bone.",
    RigCreateBiped,
    |py: Python<'_>, p: RigCreateBiped| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let name = p.name.as_deref().unwrap_or("Biped");
        let loc = vec3_or(p.location.as_deref(), [0.0, 0.0, 0.0]);
        let height = p.height.unwrap_or(2.0);

        let arm_data = d.getattr("armatures")?.call_method1("new", (name,))?;
        let arm_obj = d
            .getattr("objects")?
            .call_method1("new", (name, &arm_data))?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&arm_obj,))?;
        set_vec(py, &arm_obj, "location", &loc)?;
        set_active(&bpy, &arm_obj)?;
        set_mode(py, &bpy, "EDIT")?;

        let edit_bones = arm_data.getattr("edit_bones")?;

        // Proportions derived from total height, matching the legacy handler's ratios.
        let spine_base = height * 0.45;
        let spine_top = height * 0.8;
        let neck_top = height * 0.95;
        let shoulder_width = height * 0.2;
        let arm_length = height * 0.35;
        let leg_length = height * 0.45;
        let foot_length = height * 0.1;

        let mut bones_created: Vec<String> = Vec::new();

        // Spine
        let root_t = [0.0, 0.0, spine_base + 0.1];
        let root = add_edit_bone(
            py,
            &edit_bones,
            "root",
            [0.0, 0.0, spine_base],
            root_t,
            None,
            false,
        )?;
        bones_created.push("root".into());

        let spine_t = [0.0, 0.0, (spine_base + spine_top) / 2.0];
        let spine = add_edit_bone(
            py,
            &edit_bones,
            "spine",
            [0.0, 0.0, spine_base],
            spine_t,
            Some(&root),
            false,
        )?;
        bones_created.push("spine".into());

        let chest_t = [0.0, 0.0, spine_top];
        let chest = add_edit_bone(
            py,
            &edit_bones,
            "chest",
            spine_t,
            chest_t,
            Some(&spine),
            true,
        )?;
        bones_created.push("chest".into());

        let neck_t = [0.0, 0.0, neck_top];
        let neck = add_edit_bone(py, &edit_bones, "neck", chest_t, neck_t, Some(&chest), true)?;
        bones_created.push("neck".into());

        let head_t = [0.0, 0.0, height];
        let _head = add_edit_bone(py, &edit_bones, "head", neck_t, head_t, Some(&neck), true)?;
        bones_created.push("head".into());

        // Arms
        for (side, sign) in [("L", 1.0), ("R", -1.0)] {
            let shoulder_t = [sign * shoulder_width, 0.0, spine_top];
            let shoulder = add_edit_bone(
                py,
                &edit_bones,
                &format!("shoulder.{side}"),
                [0.0, 0.0, spine_top],
                shoulder_t,
                Some(&chest),
                false,
            )?;
            bones_created.push(format!("shoulder.{side}"));

            let upper_arm_t = [
                sign * (shoulder_width + arm_length * 0.5),
                0.0,
                spine_top - arm_length * 0.3,
            ];
            let upper_arm = add_edit_bone(
                py,
                &edit_bones,
                &format!("upper_arm.{side}"),
                shoulder_t,
                upper_arm_t,
                Some(&shoulder),
                true,
            )?;
            bones_created.push(format!("upper_arm.{side}"));

            let forearm_t = [
                sign * (shoulder_width + arm_length),
                0.0,
                spine_top - arm_length * 0.6,
            ];
            let forearm = add_edit_bone(
                py,
                &edit_bones,
                &format!("forearm.{side}"),
                upper_arm_t,
                forearm_t,
                Some(&upper_arm),
                true,
            )?;
            bones_created.push(format!("forearm.{side}"));

            let hand_t = [
                sign * (shoulder_width + arm_length + 0.1),
                0.0,
                spine_top - arm_length * 0.6,
            ];
            let _hand = add_edit_bone(
                py,
                &edit_bones,
                &format!("hand.{side}"),
                forearm_t,
                hand_t,
                Some(&forearm),
                true,
            )?;
            bones_created.push(format!("hand.{side}"));
        }

        // Legs
        for (side, sign) in [("L", 1.0), ("R", -1.0)] {
            let hip_x = sign * shoulder_width * 0.5;

            let thigh_t = [hip_x, 0.0, leg_length * 0.5];
            let thigh = add_edit_bone(
                py,
                &edit_bones,
                &format!("thigh.{side}"),
                [hip_x, 0.0, spine_base],
                thigh_t,
                Some(&root),
                false,
            )?;
            bones_created.push(format!("thigh.{side}"));

            let shin_t = [hip_x, 0.0, 0.0];
            let shin = add_edit_bone(
                py,
                &edit_bones,
                &format!("shin.{side}"),
                thigh_t,
                shin_t,
                Some(&thigh),
                true,
            )?;
            bones_created.push(format!("shin.{side}"));

            let foot_t = [hip_x, -foot_length, 0.0];
            let _foot = add_edit_bone(
                py,
                &edit_bones,
                &format!("foot.{side}"),
                shin_t,
                foot_t,
                Some(&shin),
                true,
            )?;
            bones_created.push(format!("foot.{side}"));
        }

        set_mode(py, &bpy, "OBJECT")?;

        let an: String = arm_obj.getattr("name")?.extract()?;
        let bone_count = bones_created.len();
        Ok(json!({ "armature": an, "bones": bones_created, "bone_count": bone_count }))
    }
);

// ---------------------------------------------------------------------------
// rig_mirror_weights
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RigMirrorWeights {
    /// Mesh object whose vertex groups are mirrored, as in `bpy.data.objects`. Must
    /// be of type MESH.
    pub mesh: String,
    /// LEFT_TO_RIGHT or RIGHT_TO_LEFT. Accepted and echoed back for compatibility
    /// only: `object.vertex_group_mirror` has no directional argument and mirrors
    /// matching `.L`/`.R` group pairs both ways in one pass. Default: LEFT_TO_RIGHT.
    pub direction: Option<String>,
}

crate::tool!(
    "rig_mirror_weights",
    "Mirror a mesh's vertex weights across X between matching `.L`/`.R` vertex \
     groups, selecting all vertices first and restoring the original mode. `direction` \
     is cosmetic — Blender's operator mirrors both ways at once. Generate the groups \
     first with rig_auto_weights.",
    RigMirrorWeights,
    |py: Python<'_>, p: RigMirrorWeights| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.mesh.as_str(),))?;
        if obj.is_none() || obj.getattr("type")?.extract::<String>()? != "MESH" {
            return Err(PyValueError::new_err(format!("Mesh not found: {}", p.mesh)));
        }

        // `bpy.ops.object.vertex_group_mirror` has no directional argument of its
        // own — it mirrors symmetric ".L"/".R" vertex group pairs both ways in one
        // call. The legacy handler's LEFT_TO_RIGHT and RIGHT_TO_LEFT branches already
        // invoked the exact same operator; `direction` is accepted and echoed back
        // for API compatibility but does not change the operator's behaviour.
        let direction = p.direction.unwrap_or_else(|| "LEFT_TO_RIGHT".to_string());

        let orig_mode: String = obj.getattr("mode")?.extract()?;
        set_active(&bpy, &obj)?;
        set_mode(py, &bpy, "WEIGHT_PAINT")?;

        set_mode(py, &bpy, "EDIT")?;
        // Not checked: prep step, always {'FINISHED'} for `SELECT` (verified
        // live) — unlike `DESELECT`, it does not cancel when idempotent.
        let select_all = PyDict::new(py);
        select_all.set_item("action", "SELECT")?;
        bpy.getattr("ops")?
            .getattr("mesh")?
            .call_method("select_all", (), Some(&select_all))?;
        set_mode(py, &bpy, "WEIGHT_PAINT")?;

        let mirror_kwargs = PyDict::new(py);
        mirror_kwargs.set_item("mirror_weights", true)?;
        mirror_kwargs.set_item("flip_group_names", true)?;
        mirror_kwargs.set_item("use_topology", false)?;
        let result = bpy.getattr("ops")?.getattr("object")?.call_method(
            "vertex_group_mirror",
            (),
            Some(&mirror_kwargs),
        )?;
        op_ok(&result, &format!("mirror vertex weights on {}", p.mesh))?;

        set_mode(py, &bpy, orig_mode.as_str())?;

        let mn: String = obj.getattr("name")?.extract()?;
        Ok(json!({ "mesh": mn, "direction": direction, "mirrored": true }))
    }
);
