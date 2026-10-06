//! Custom (ID) properties — `obj["hp"] = 100`.
//!
//! Rig control UIs, driver hubs and per-instance data all run on these. Drivers could
//! already *read* a custom property (`drv_add` takes a data path), but nothing could
//! create one, so a rig had no controls to drive.
//!
//! ## Addressing
//! `target` is `<collection>:<name>`, where the collection is a real `bpy.data`
//! collection: `objects:Cube`, `materials:Rust`, `meshes:Cube`, `scenes:Scene`,
//! `collections:Props`, `node_groups:MyGroup`, `actions:Walk`, `images:Brick`.
//! A pose bone — which is not a datablock — is `bone:<armature>/<bone>`, because a rig's
//! controls live on its bones and that is the main reason this module exists.
//!
//! The valid collection names are read from `bpy.data`'s own RNA, so a wrong one is
//! answered with the real list rather than a table that would go stale.

use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use serde::Deserialize;
use serde_json::{Map, Value, json};

/// Every `bpy.data` collection Blender has, from RNA. No table.
fn data_collections(py: Python<'_>) -> PyResult<Vec<String>> {
    let data = py.import("bpy")?.getattr("data")?;
    let mut out = Vec::new();
    for prop in data.getattr("bl_rna")?.getattr("properties")?.try_iter()? {
        let prop = prop?;
        if prop.getattr("type")?.extract::<String>()? == "COLLECTION" {
            out.push(prop.getattr("identifier")?.extract::<String>()?);
        }
    }
    Ok(out)
}

/// Resolve `objects:Cube` / `bone:Rig/Hand.L` to the thing that carries the property.
fn resolve_target<'py>(py: Python<'py>, target: &str) -> PyResult<Bound<'py, PyAny>> {
    let bpy = py.import("bpy")?;
    let (kind, rest) = target.split_once(':').ok_or_else(|| {
        PyValueError::new_err(format!(
            "target must be `<collection>:<name>` (e.g. objects:Cube, materials:Rust) \
             or `bone:<armature>/<bone>`, got `{target}`"
        ))
    })?;

    if kind == "bone" {
        let (arm_name, bone_name) = rest
            .split_once('/')
            .ok_or_else(|| PyValueError::new_err("a bone target is `bone:<armature>/<bone>`"))?;
        let arm = bpy
            .getattr("data")?
            .getattr("objects")?
            .call_method1("get", (arm_name,))?;
        if arm.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {arm_name}"
            )));
        }
        let pose = arm.getattr("pose")?;
        if pose.is_none() {
            return Err(PyValueError::new_err(format!(
                "{arm_name} is not an armature — it has no pose bones"
            )));
        }
        let bone = pose.getattr("bones")?.call_method1("get", (bone_name,))?;
        if bone.is_none() {
            let mut have = Vec::new();
            for b in pose.getattr("bones")?.try_iter()? {
                have.push(b?.getattr("name")?.extract::<String>()?);
            }
            return Err(PyValueError::new_err(format!(
                "{arm_name} has no bone `{bone_name}`. Its bones: {}",
                have.join(", ")
            )));
        }
        return Ok(bone);
    }

    let collections = data_collections(py)?;
    if !collections.iter().any(|c| c == kind) {
        return Err(PyValueError::new_err(format!(
            "unknown collection `{kind}`. bpy.data has: {}",
            collections.join(", ")
        )));
    }

    let coll = bpy.getattr("data")?.getattr(kind)?;
    let item = coll.call_method1("get", (rest,))?;
    if item.is_none() {
        let mut have = Vec::new();
        for i in coll.try_iter()? {
            have.push(i?.getattr("name")?.extract::<String>()?);
        }
        let listing = if have.is_empty() {
            "there are none".to_string()
        } else {
            have.join(", ")
        };
        return Err(PyValueError::new_err(format!(
            "no `{rest}` in bpy.data.{kind} — this file has: {listing}"
        )));
    }
    Ok(item)
}

/// A JSON value as a Python object Blender will store. ID properties take numbers,
/// strings, booleans and sequences of numbers; a nested object is not storable.
fn to_py<'py>(py: Python<'py>, v: &Value) -> PyResult<Bound<'py, PyAny>> {
    Ok(match v {
        Value::Bool(b) => b.into_pyobject(py)?.to_owned().into_any(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into_pyobject(py)?.into_any()
            } else {
                n.as_f64()
                    .ok_or_else(|| PyValueError::new_err("not a number"))?
                    .into_pyobject(py)?
                    .into_any()
            }
        }
        Value::String(s) => s.as_str().into_pyobject(py)?.into_any(),
        Value::Array(a) => {
            let nums: Vec<f64> = a.iter().filter_map(Value::as_f64).collect();
            if nums.len() != a.len() {
                return Err(PyValueError::new_err(
                    "a list-valued custom property takes numbers only",
                ));
            }
            pyo3::types::PyList::new(py, nums)?.into_any()
        }
        Value::Null | Value::Object(_) => {
            return Err(PyValueError::new_err(format!(
                "a custom property cannot hold {v} — use a number, string, bool or list of numbers"
            )));
        }
    })
}

fn from_py(v: &Bound<'_, PyAny>) -> Option<Value> {
    if let Ok(b) = v.extract::<bool>() {
        return Some(json!(b));
    }
    if let Ok(i) = v.extract::<i64>() {
        return Some(json!(i));
    }
    if let Ok(f) = v.extract::<f64>() {
        return Some(json!(f));
    }
    if let Ok(s) = v.extract::<String>() {
        return Some(json!(s));
    }
    if let Ok(seq) = v.extract::<Vec<f64>>() {
        return Some(json!(seq));
    }
    None
}

fn all_props(item: &Bound<'_, PyAny>) -> PyResult<Value> {
    let mut out = Map::new();
    for key in item.call_method0("keys")?.try_iter()? {
        let key: String = key?.extract()?;
        // Blender keeps its own bookkeeping in here too (`_RNA_UI`, cycles settings…).
        // Those are not the user's properties and only clutter the answer.
        if key.starts_with('_') || key == "cycles" {
            continue;
        }
        if let Ok(v) = item.get_item(key.as_str())
            && let Some(j) = from_py(&v)
        {
            out.insert(key, j);
        }
    }
    Ok(Value::Object(out))
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PropList {
    /// `<collection>:<name>` (objects:Cube, materials:Rust, scenes:Scene…) or
    /// `bone:<armature>/<bone>`.
    pub target: String,
}

crate::tool!(
    "prop_list",
    "Read every custom (ID) property on a datablock or pose bone, skipping Blender's \
     own bookkeeping keys (`_RNA_UI`, `cycles`). Read-only; an object with none \
     returns an empty map. Write them with prop_set, delete with prop_remove, drive \
     one with drv_add using the data path `[\"name\"]`.",
    PropList,
    |py: Python<'_>, p: PropList| {
        let item = resolve_target(py, &p.target)?;
        Ok(json!({ "target": p.target, "props": all_props(&item)? }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PropSet {
    /// `<collection>:<name>` — a real `bpy.data` collection and a datablock in it
    /// (objects:Cube, materials:Rust, meshes:Cube, scenes:Scene, collections:Props,
    /// node_groups:MyGroup, actions:Walk, images:Brick) — or `bone:<armature>/<bone>`
    /// for a pose bone. A wrong collection is answered with the live list.
    pub target: String,
    /// The properties to set, as `{name: value}`. A value is a number, string, bool
    /// or list of numbers; a nested object is rejected. Existing keys are overwritten.
    pub props: Map<String, Value>,
    /// Lower end of the UI range (both hard `min` and `soft_min`), which is what
    /// turns the property into a slider. Numeric properties only — a string or list
    /// property errors. Applied to every key in `props`, and cast to an integer when
    /// the stored value is one. Default: Blender's own range.
    pub min: Option<f64>,
    /// Upper end of the UI range (both hard `max` and `soft_max`). Same rules as
    /// `min`. Default: Blender's own range.
    pub max: Option<f64>,
    /// Tooltip shown in Blender's UI for every key in `props`. Default: none.
    pub description: Option<String>,
}

crate::tool!(
    "prop_set",
    "Set custom (ID) properties on a datablock or pose bone (`obj[\"hp\"] = 100`), \
     optionally with a slider range and tooltip. This is exactly what a rig control \
     is: a custom property on a bone, read by a driver — point drv_add or drv_create \
     at the data path `[\"hp\"]`. Inspect with prop_list, delete with prop_remove.",
    PropSet,
    |py: Python<'_>, p: PropSet| {
        let item = resolve_target(py, &p.target)?;

        for (k, v) in &p.props {
            item.set_item(k.as_str(), to_py(py, v)?)?;
        }

        // The UI range is what makes a property a usable slider rather than a raw number
        // field. It lives in a separate metadata store, not on the value.
        if p.min.is_some() || p.max.is_some() || p.description.is_some() {
            for (k, v) in &p.props {
                // Blender demands the range match the property's own type: handing an
                // int property a float min raises "'float' object cannot be interpreted
                // as an integer". The value we just stored is the authority on which.
                let is_int = v.is_i64() || v.is_u64();

                let ui = item.call_method1("id_properties_ui", (k.as_str(),))?;
                let kw = pyo3::types::PyDict::new(py);
                for (key, soft, bound) in [("min", "soft_min", p.min), ("max", "soft_max", p.max)] {
                    let Some(b) = bound else { continue };
                    if is_int {
                        kw.set_item(key, b as i64)?;
                        kw.set_item(soft, b as i64)?;
                    } else {
                        kw.set_item(key, b)?;
                        kw.set_item(soft, b)?;
                    }
                }
                if let Some(d) = &p.description {
                    kw.set_item("description", d.as_str())?;
                }
                // A string or list property has no range at all; say so rather than
                // letting Blender's TypeError through.
                ui.call_method("update", (), Some(&kw)).map_err(|e| {
                    PyValueError::new_err(format!(
                        "cannot set a UI range on `{k}` — is it a number? ({e})"
                    ))
                })?;
            }
        }

        Ok(json!({ "target": p.target, "props": all_props(&item)? }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PropRemove {
    /// `<collection>:<name>` (objects:Cube, materials:Rust…) or
    /// `bone:<armature>/<bone>`, the same addressing prop_list and prop_set use.
    pub target: String,
    /// Property names to remove (see prop_list). A name that is not there fails the
    /// call — this is a single target, not a batch.
    pub names: Vec<String>,
}

crate::tool!(
    "prop_remove",
    "Delete named custom (ID) properties from a datablock or pose bone; a name that \
     is not there is an error, not a silent no-op. Any driver still reading the \
     property (drv_remove) is left dangling, so remove that first.",
    PropRemove,
    |py: Python<'_>, p: PropRemove| {
        let item = resolve_target(py, &p.target)?;
        for name in &p.names {
            item.del_item(name.as_str()).map_err(|_| {
                PyKeyError::new_err(format!("`{}` has no custom property `{name}`", p.target))
            })?;
        }
        Ok(json!({ "target": p.target, "props": all_props(&item)? }))
    }
);
