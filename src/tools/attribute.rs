//! Vertex groups, colour attributes, and generic named mesh attributes.
//!
//! ## Why this file exists
//! `mod_add` (`object.rs`) already advertises a `vertex_group` parameter on
//! HOOK/MASK/ARMATURE modifiers, but until this file nothing could *create* a
//! vertex group -- weight-painted falloff, the standard way to art-direct a
//! modifier, was impossible. Geometry nodes lean entirely on named attributes
//! (Store Named Attribute / Named Attribute), and a shader's Color Attribute node
//! reads a colour attribute -- but nothing outside the node editor could create or
//! write either, so those inputs were dead ends. This closes all three gaps.
//!
//! ## Facts verified against a live Blender 5.2.2 LTS (`--background --factory-startup`)
//! - `VertexGroup.add(index_or_indices, weight, type)` -- `type` is an enum with
//!   exactly `REPLACE`/`ADD`/`SUBTRACT` (read from
//!   `bpy.types.VertexGroup.bl_rna.functions['add']`, not hardcoded).
//! - `VertexGroup.weight(index)` **raises** `RuntimeError` for a vertex that
//!   legitimately exists (is within the mesh's vertex count) but is not a member
//!   of the group. For an index past the vertex count entirely it does NOT raise --
//!   it returns `0.0` (verified live: a fresh group with members [0,1] on an 8-vert
//!   mesh raises on `weight(5)` but returns `0.0` from `weight(99)`). `add()` itself
//!   does not validate the index against vertex count at all (Blender tolerates
//!   dangling weight entries). See [`vg_weight`].
//! - `vertex_groups.remove()` takes the `VertexGroup` object, not a name or index.
//! - `bpy.types.Attribute.bl_rna` has two enum properties, `data_type` and
//!   `domain`. On 5.2.2: `data_type` is FLOAT, INT, BOOLEAN, FLOAT_VECTOR,
//!   FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2,
//!   **FLOAT4**, BYTE_COLOR -- all 14 accepted by a `Mesh`; `domain` is POINT,
//!   EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER -- a `Mesh` accepts only
//!   POINT/EDGE/FACE/CORNER and raises `RuntimeError: Attribute domain not
//!   supported by this geometry type` on CURVE/INSTANCE/LAYER. Both lists are read
//!   live, not copied here: 5.0 added `INT16_2D`/`INT32_2D`/`FLOAT4X4` and 5.2
//!   added `FLOAT4`, so any hardcoded table goes stale.
//! - `mesh.color_attributes.new(name, type, domain)` and `mesh.attributes.new(...)`
//!   are, at the RNA level, **the exact same function** -- Blender does not
//!   restrict `color_attributes.new()` to FLOAT_COLOR/BYTE_COLOR, nor its domain
//!   to POINT/CORNER: on 5.2.2 a plain `type='FLOAT'` call through it succeeds, as
//!   do `domain='EDGE'` and `'FACE'`. Only the UI (and what the Color Attribute
//!   node / Vertex Paint actually read) treats them as separate concepts.
//!   `color_attr_add` still defaults to FLOAT_COLOR/POINT, because
//!   defaulting to the RNA function's own default (FLOAT) would silently produce
//!   a non-colour attribute from a tool named "add a colour attribute".
//! - The `.data[i]` item's writable field differs by `data_type` and is not a
//!   single name: `value` for FLOAT/INT/BOOLEAN/QUATERNION/FLOAT4X4/STRING/INT8/
//!   INT16_2D/INT32_2D, `vector` for FLOAT_VECTOR/FLOAT2/FLOAT4, `color` for
//!   FLOAT_COLOR/BYTE_COLOR (which also exposes a `color_srgb` view of the same
//!   four floats). [`value_field`] asks the item's own RNA instead of encoding
//!   this table, so a future data type cannot silently fall through it.
//! - `StringAttributeValue.value` takes Python `bytes`, not `str` -- assigning a
//!   `str` raises `TypeError`.

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyBytes, PyDict, PyList, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, py_len};
use super::nodes::py_to_json;

// ============================================================================
// Shared helpers
// ============================================================================

/// Look up `bpy.data.objects[name]`, any type. Vertex groups are not exclusive to
/// meshes (Lattice has them too), and Blender's own error already names the
/// offending type clearly when an operation does not support it -- see the
/// `vgroup_add` doc comment.
fn get_obj<'py>(d: &Bound<'py, PyAny>, name: &str) -> PyResult<Bound<'py, PyAny>> {
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {name}")));
    }
    Ok(obj)
}

/// Look up a MESH object by name. Colour and generic named attributes live on
/// `Mesh` data only -- `obj.data.attributes` does not exist on a Camera or
/// Armature -- so every attribute (as opposed to vertex-group) tool needs this.
fn get_mesh_obj<'py>(d: &Bound<'py, PyAny>, name: &str) -> PyResult<Bound<'py, PyAny>> {
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    let ok = if obj.is_none() {
        false
    } else {
        obj.getattr("type")?.extract::<String>()? == "MESH"
    };
    if !ok {
        return Err(PyValueError::new_err(format!(
            "Mesh object not found: {name}"
        )));
    }
    Ok(obj)
}

/// `obj.vertex_groups[name]`, erroring with the object's real group list on a
/// miss instead of leaving `None` to blow up on the next attribute access.
fn find_vgroup<'py>(obj: &Bound<'py, PyAny>, name: &str) -> PyResult<Bound<'py, PyAny>> {
    let groups = obj.getattr("vertex_groups")?;
    let vg = groups.call_method1("get", (name,))?;
    if vg.is_none() {
        let mut have = Vec::new();
        for g in groups.try_iter()? {
            have.push(g?.getattr("name")?.extract::<String>()?);
        }
        return Err(PyValueError::new_err(format!(
            "no vertex group `{name}`. This object has: {}",
            if have.is_empty() {
                "none".to_string()
            } else {
                have.join(", ")
            }
        )));
    }
    Ok(vg)
}

/// `VertexGroup.weight(index)`, with Blender's "not a member" signal -- a
/// `RuntimeError`, not a return value -- turned into `None`. A vertex absent from
/// a group is the ordinary case for `vgroup_get`, not a failure to propagate.
fn vg_weight(vg: &Bound<'_, PyAny>, idx: i64) -> PyResult<Option<f64>> {
    match vg.call_method1("weight", (idx,)) {
        Ok(w) => Ok(Some(w.extract::<f64>()?)),
        Err(e) if e.is_instance_of::<PyRuntimeError>(vg.py()) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Validate a `vgroup_set` weight mode against the real enum
/// `VertexGroup.add()` accepts, read from RNA so a Blender version that changes
/// the set (unlikely, but so was `Action.fcurves` disappearing) cannot go stale
/// here silently.
fn vgroup_mode(py: Python<'_>, value: &str) -> PyResult<String> {
    let cls = py.import("bpy")?.getattr("types")?.getattr("VertexGroup")?;
    let func = cls
        .getattr("bl_rna")?
        .getattr("functions")?
        .call_method1("get", ("add",))?;
    let param = func.getattr("parameters")?.call_method1("get", ("type",))?;
    let mut avail = Vec::new();
    for item in param.getattr("enum_items")?.try_iter()? {
        avail.push(item?.getattr("identifier")?.extract::<String>()?);
    }
    let up = value.to_uppercase();
    avail.iter().find(|a| **a == up).cloned().ok_or_else(|| {
        PyValueError::new_err(format!(
            "unknown vertex group mode `{value}`. Valid: {}",
            avail.join(", ")
        ))
    })
}

/// Validate a `data_type`/`domain` string against Blender's live RNA enum for
/// `bpy.types.Attribute`, so a typo gets today's real option list rather than a
/// bare `TypeError`. See the module doc for why this is read every call.
fn attr_enum(py: Python<'_>, prop_name: &str, value: &str) -> PyResult<String> {
    let attr_rna = py
        .import("bpy")?
        .getattr("types")?
        .getattr("Attribute")?
        .getattr("bl_rna")?;
    let prop = attr_rna
        .getattr("properties")?
        .call_method1("get", (prop_name,))?;
    let mut avail = Vec::new();
    for item in prop.getattr("enum_items")?.try_iter()? {
        avail.push(item?.getattr("identifier")?.extract::<String>()?);
    }
    let up = value.to_uppercase();
    avail.iter().find(|a| **a == up).cloned().ok_or_else(|| {
        PyValueError::new_err(format!(
            "unknown attribute {prop_name} `{value}`. This Blender has: {}",
            avail.join(", ")
        ))
    })
}

/// Which field on an attribute data item actually holds the value. Blender does
/// not use one name across types (see the module doc). Rather than hardcode a
/// data_type -> field table, this asks a real item's own RNA which of its
/// properties are writable, so an unfamiliar data type fails loudly instead of
/// silently writing nothing.
fn value_field(item: &Bound<'_, PyAny>) -> PyResult<String> {
    let mut candidates = Vec::new();
    for prop in item.getattr("bl_rna")?.getattr("properties")?.try_iter()? {
        let prop = prop?;
        let ident: String = prop.getattr("identifier")?.extract()?;
        if ident == "rna_type" {
            continue;
        }
        if prop.getattr("is_readonly")?.extract::<bool>()? {
            continue;
        }
        candidates.push(ident);
    }
    // Colour items expose both `color` (linear) and `color_srgb` (a gamma-corrected
    // view of the same four floats) -- write through the primary one.
    if candidates.iter().any(|c| c == "color") {
        return Ok("color".to_string());
    }
    match candidates.as_slice() {
        [one] => Ok(one.clone()),
        _ => Err(PyValueError::new_err(format!(
            "cannot tell which field to write on this attribute item; RNA offers: {}",
            candidates.join(", ")
        ))),
    }
}

/// An empty row, so a malformed nested value degrades to "no cells" instead of
/// needing an unwrap on `as_array`.
static EMPTY_ROW: Vec<Value> = Vec::new();

/// Write one JSON value into an attribute data item's writable field. The legal
/// shapes mirror what Blender itself accepts for that RNA property: a plain
/// number/bool for a scalar field, a JSON array of numbers for a vector/colour/
/// quaternion/matrix field, and a JSON string -- encoded as UTF-8 bytes, since
/// `StringAttributeValue.value` refuses a Python `str` -- for STRING.
fn set_item_field(py: Python<'_>, item: &Bound<'_, PyAny>, field: &str, v: &Value) -> PyResult<()> {
    match v {
        Value::Bool(b) => item.setattr(field, *b),
        Value::String(s) => item.setattr(field, PyBytes::new(py, s.as_bytes())),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                item.setattr(field, i)
            } else if let Some(f) = n.as_f64() {
                item.setattr(field, f)
            } else {
                Err(PyValueError::new_err(format!(
                    "attribute value {n} is out of range"
                )))
            }
        }
        // A matrix field (FLOAT4X4) wants a nested sequence, not 16 loose floats:
        // Blender answers a flat one with "expected a sequence of float, not
        // float". Accept the caller's nested form directly. (Blender stores the
        // groups column-major -- verified live by writing a transform and reading
        // its translation back out of the last column.)
        Value::Array(arr) if arr.iter().all(Value::is_array) => {
            let mut rows = Vec::with_capacity(arr.len());
            for row in arr {
                let cells = row.as_array().unwrap_or(&EMPTY_ROW);
                let mut floats = Vec::with_capacity(cells.len());
                for x in cells {
                    floats.push(x.as_f64().ok_or_else(|| {
                        PyValueError::new_err(format!("attribute value {x} is not numeric"))
                    })?);
                }
                rows.push(PyTuple::new(py, floats)?);
            }
            item.setattr(field, PyTuple::new(py, rows)?)
        }
        Value::Array(arr) => {
            // Whole numbers go through as Python ints (required by the INT*
            // fields, and coerced fine where a float is expected); anything with
            // a fractional part forces the float path.
            let ints: Vec<i64> = arr.iter().filter_map(Value::as_i64).collect();
            let flat = if ints.len() == arr.len() {
                item.setattr(field, PyTuple::new(py, ints)?)
            } else {
                let mut floats = Vec::with_capacity(arr.len());
                for x in arr {
                    floats.push(x.as_f64().ok_or_else(|| {
                        PyValueError::new_err(format!("attribute value {x} is not numeric"))
                    })?);
                }
                item.setattr(field, PyTuple::new(py, floats)?)
            };
            // 16 numbers are what a caller naturally writes for a 4x4; fold them
            // into rows and retry rather than handing back Blender's opaque
            // "expected a sequence of float, not float".
            match flat {
                Err(_) if arr.len() == 16 => {
                    let mut floats = Vec::with_capacity(16);
                    for x in arr {
                        floats.push(x.as_f64().ok_or_else(|| {
                            PyValueError::new_err(format!("attribute value {x} is not numeric"))
                        })?);
                    }
                    let rows: Vec<_> = floats
                        .chunks(4)
                        .map(|row| PyTuple::new(py, row.to_vec()))
                        .collect::<PyResult<_>>()?;
                    item.setattr(field, PyTuple::new(py, rows)?)
                }
                other => other,
            }
        }
        _ => Err(PyValueError::new_err(format!(
            "attribute value must be a number, bool, string, or array of numbers, got {v}"
        ))),
    }
}

/// Read an attribute data item's field back to JSON. Used only for `attr_list`'s
/// preview sample, so lossy display (e.g. UTF-8 replacement on non-text bytes) is
/// acceptable -- this never feeds back into a write.
fn item_value_json(item: &Bound<'_, PyAny>, field: &str) -> PyResult<Value> {
    let v = item.getattr(field)?;
    if let Ok(bytes) = v.extract::<Vec<u8>>() {
        return Ok(json!(String::from_utf8_lossy(&bytes).into_owned()));
    }
    Ok(py_to_json(&v).unwrap_or(Value::Null))
}

/// One `mesh.attributes` / `mesh.color_attributes` entry, described for the model.
fn describe_attr(a: &Bound<'_, PyAny>, size: usize) -> PyResult<Value> {
    Ok(json!({
        "name": a.getattr("name")?.extract::<String>()?,
        "data_type": a.getattr("data_type")?.extract::<String>()?,
        "domain": a.getattr("domain")?.extract::<String>()?,
        "is_internal": a.getattr("is_internal")?.extract::<bool>()?,
        "size": size,
    }))
}

/// List every entry of `mesh.attributes` or `mesh.color_attributes` (`which`
/// names the collection), each with a short sample of its actual values so the
/// model can see what is stored without a second round trip.
fn list_attrs(mesh: &Bound<'_, PyAny>, which: &str) -> PyResult<Value> {
    let py = mesh.py();
    let coll = mesh.getattr(which)?;
    let mut out = Vec::new();
    for a in coll.try_iter()? {
        let a = a?;
        let data = a.getattr("data")?;
        let len = py_len(py, &data)?;
        let mut entry = describe_attr(&a, len)?;
        if len > 0 {
            let field = value_field(&data.get_item(0i64)?)?;
            let n = len.min(5);
            let mut sample = Vec::with_capacity(n);
            for i in 0..n {
                sample.push(item_value_json(&data.get_item(i as i64)?, &field)?);
            }
            entry["sample"] = json!(sample);
        }
        out.push(entry);
    }
    Ok(json!(out))
}

/// `mesh.attributes.new(name, type, domain)` / `mesh.color_attributes.new(...)`,
/// with `type`/`domain` validated against live RNA first.
fn add_attr(
    mesh: &Bound<'_, PyAny>,
    which: &str,
    py: Python<'_>,
    attr_name: &str,
    data_type: &str,
    domain: &str,
) -> PyResult<Value> {
    let data_type = attr_enum(py, "data_type", data_type)?;
    let domain = attr_enum(py, "domain", domain)?;
    let coll = mesh.getattr(which)?;
    let a = coll.call_method1("new", (attr_name, data_type.as_str(), domain.as_str()))?;
    let size = py_len(py, &a.getattr("data")?)?;
    describe_attr(&a, size)
}

/// `mesh.attributes[name]` / `mesh.color_attributes[name]`, erroring with the
/// mesh's real attribute list on a miss.
fn find_attr<'py>(
    mesh: &Bound<'py, PyAny>,
    which: &str,
    name: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let coll = mesh.getattr(which)?;
    let a = coll.call_method1("get", (name,))?;
    if a.is_none() {
        let mut have = Vec::new();
        for x in coll.try_iter()? {
            have.push(x?.getattr("name")?.extract::<String>()?);
        }
        return Err(PyValueError::new_err(format!(
            "no attribute `{name}` in {which}. This mesh has: {}",
            if have.is_empty() {
                "none".to_string()
            } else {
                have.join(", ")
            }
        )));
    }
    Ok(a)
}

fn remove_attr(mesh: &Bound<'_, PyAny>, which: &str, name: &str) -> PyResult<()> {
    let a = find_attr(mesh, which, name)?;
    mesh.getattr(which)?.call_method1("remove", (&a,))?;
    Ok(())
}

/// Write `values` into an attribute's `.data`, at `indices` if given or
/// sequentially over every item otherwise. Every element addresses the same
/// attribute, so the writable field is resolved once (from the first target)
/// rather than re-inspected per item.
fn set_attr_values(
    py: Python<'_>,
    mesh: &Bound<'_, PyAny>,
    which: &str,
    name: &str,
    indices: Option<Vec<i64>>,
    values: &[Value],
) -> PyResult<(String, usize)> {
    let a = find_attr(mesh, which, name)?;
    let data = a.getattr("data")?;
    let total = py_len(py, &data)?;

    let targets: Vec<i64> = match indices {
        Some(ix) => ix,
        None => (0..total as i64).collect(),
    };
    if targets.len() != values.len() {
        return Err(PyValueError::new_err(format!(
            "{}: {} indices but {} values",
            name,
            targets.len(),
            values.len()
        )));
    }
    if targets.is_empty() {
        return Ok(("".to_string(), 0));
    }

    let first = data.get_item(targets[0])?;
    let field = value_field(&first)?;

    for (idx, val) in targets.iter().zip(values.iter()) {
        let item = data.get_item(*idx)?;
        set_item_field(py, &item, &field, val).map_err(|e| {
            PyValueError::new_err(format!("writing item {idx} field `{field}`: {e}"))
        })?;
    }

    Ok((field, targets.len()))
}

// ============================================================================
// Vertex groups
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VgroupList {
    /// Object name.
    pub object: String,
}

crate::tool!(
    "vgroup_list",
    "List an object's vertex groups (name and index). A vertex group is what a HOOK/MASK/ARMATURE modifier's `vertex_group` parameter refers to, and what weight-painted falloff is stored in. Vertex groups are a separate system from mesh attributes: use vgroup_* for per-vertex weights bound to modifiers and armatures, attr_* for arbitrary named data geometry nodes read, color_attr_* for vertex colours.",
    VgroupList,
    |py: Python<'_>, p: VgroupList| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_obj(&d, &p.object)?;

        let mut groups = Vec::new();
        for g in obj.getattr("vertex_groups")?.try_iter()? {
            let g = g?;
            groups.push(json!({
                "name": g.getattr("name")?.extract::<String>()?,
                "index": g.getattr("index")?.extract::<i64>()?,
            }));
        }
        Ok(json!({ "name": p.object, "groups": groups }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VgroupAdd {
    /// Object name (MESH or LATTICE).
    pub object: String,
    /// New vertex group's name. Blender appends `.001` etc. on a collision; the
    /// actual resulting name is returned. The group starts empty -- fill it with
    /// vgroup_set.
    pub name: String,
}

crate::tool!(
    "vgroup_add",
    "Create a vertex group on an object -- the missing half of mod_add's `vertex_group` parameter: without a group to point at, HOOK/MASK/ARMATURE falloff cannot be art-directed. Only MESH and LATTICE objects support this; Blender raises its own clear error on other types.",
    VgroupAdd,
    |py: Python<'_>, p: VgroupAdd| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_obj(&d, &p.object)?;

        // `VertexGroups.new()` insists `name` be passed as a keyword -- unlike
        // every other `.new()` in this file, a positional call raises `TypeError`
        // (verified live; not documented anywhere obvious).
        let kwargs = PyDict::new(py);
        kwargs.set_item("name", p.name.as_str())?;
        let vg = obj
            .getattr("vertex_groups")?
            .call_method("new", (), Some(&kwargs))?;

        Ok(json!({
            "name": p.object,
            "group_name": vg.getattr("name")?.extract::<String>()?,
            "index": vg.getattr("index")?.extract::<i64>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VgroupRemove {
    /// Object name.
    pub object: String,
    /// Vertex group to remove. Missing name is an error, which lists the groups
    /// the object does have.
    pub name: String,
}

crate::tool!(
    "vgroup_remove",
    "Remove a vertex group from an object. The weights stored in it are lost; any modifier still naming it silently falls back to affecting nothing.",
    VgroupRemove,
    |py: Python<'_>, p: VgroupRemove| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_obj(&d, &p.object)?;
        let vg = find_vgroup(&obj, &p.name)?;
        obj.getattr("vertex_groups")?
            .call_method1("remove", (&vg,))?;
        Ok(json!({ "name": p.object, "removed": p.name }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VgroupRename {
    /// Object name.
    pub object: String,
    /// Vertex group to rename.
    pub name: String,
    /// Requested new name. Blender appends `.001` etc. on a collision; the actual
    /// resulting name is returned.
    pub new_name: String,
}

crate::tool!(
    "vgroup_rename",
    "Rename a vertex group. Modifiers that name the old group are not updated -- re-point them with mod_update.",
    VgroupRename,
    |py: Python<'_>, p: VgroupRename| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_obj(&d, &p.object)?;
        let vg = find_vgroup(&obj, &p.name)?;
        vg.setattr("name", p.new_name.as_str())?;
        Ok(json!({
            "name": p.object,
            "old_name": p.name,
            "new_name": vg.getattr("name")?.extract::<String>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VgroupSet {
    /// Object name.
    pub object: String,
    /// Vertex group to write into.
    pub name: String,
    /// Vertex indices to assign a weight to, as a JSON array of ints. Not
    /// validated against the mesh's vertex count -- Blender tolerates a dangling
    /// weight entry rather than raising.
    pub indices: Vec<i64>,
    /// One weight (0..1) applied to every index in `indices`. Mutually exclusive
    /// with `weights` -- give exactly one of the two.
    pub weight: Option<f64>,
    /// A weight per index, same length and order as `indices`. Use this for a
    /// hand-painted falloff instead of one uniform value.
    pub weights: Option<Vec<f64>>,
    /// One of REPLACE, ADD, SUBTRACT -- the full enum of
    /// `VertexGroup.add(indices, weight, type)` on Blender 5.2.2, read back from
    /// live RNA on every call. REPLACE (default) overwrites each vertex's weight
    /// in this group, ADD adds to the existing weight, SUBTRACT removes the given
    /// indices from the group.
    pub mode: Option<String>,
}

crate::tool!(
    "vgroup_set",
    "Assign vertex weights in a group: one weight for a batch of indices (`weight`), or a weight per index for a hand-painted falloff (`weights`). The group must already exist -- create it with vgroup_add. Read back with vgroup_get.",
    VgroupSet,
    |py: Python<'_>, p: VgroupSet| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_obj(&d, &p.object)?;
        let vg = find_vgroup(&obj, &p.name)?;
        let mode = vgroup_mode(py, &p.mode.unwrap_or_else(|| "REPLACE".to_string()))?;

        match (p.weight, p.weights) {
            (Some(_), Some(_)) => {
                return Err(PyValueError::new_err(
                    "vgroup_set: give either `weight` or `weights`, not both",
                ));
            }
            (None, None) => {
                return Err(PyValueError::new_err(
                    "vgroup_set: give one of `weight` (uniform) or `weights` (per index)",
                ));
            }
            (Some(w), None) => {
                let idx_list = PyList::new(py, p.indices.iter().copied())?;
                vg.call_method1("add", (idx_list, w, mode.as_str()))?;
            }
            (None, Some(ws)) => {
                if ws.len() != p.indices.len() {
                    return Err(PyValueError::new_err(format!(
                        "vgroup_set: {} indices but {} weights",
                        p.indices.len(),
                        ws.len()
                    )));
                }
                for (idx, w) in p.indices.iter().zip(ws.iter()) {
                    let idx_list = PyList::new(py, [*idx])?;
                    vg.call_method1("add", (idx_list, *w, mode.as_str()))?;
                }
            }
        }

        Ok(json!({
            "name": p.object,
            "group_name": p.name,
            "mode": mode,
            "written": p.indices.len(),
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VgroupGet {
    /// Object name.
    pub object: String,
    /// Vertex group to read.
    pub name: String,
    /// Which vertices to read, as a JSON array of vertex indices. Omit to return every vertex actually in the group
    /// (requires a MESH object, read from its geometry directly); when given, a
    /// requested index that is not a member of the group is simply absent from
    /// the result, not an error -- Blender's own `VertexGroup.weight()` raises for
    /// that case, and this tool absorbs it rather than propagating it.
    pub indices: Option<Vec<i64>>,
}

crate::tool!(
    "vgroup_get",
    "Read weights back from a vertex group: which vertices are in it, and at what weight. The counterpart to vgroup_set; a vertex absent from the group is simply omitted, not an error.",
    VgroupGet,
    |py: Python<'_>, p: VgroupGet| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_obj(&d, &p.object)?;
        let vg = find_vgroup(&obj, &p.name)?;

        let mut weights = Vec::new();

        if let Some(indices) = &p.indices {
            for &idx in indices {
                if let Some(w) = vg_weight(&vg, idx)? {
                    weights.push(json!({ "index": idx, "weight": w }));
                }
            }
        } else {
            let mesh_data = obj.getattr("data")?;
            if !mesh_data.hasattr("vertices")? {
                return Err(PyValueError::new_err(
                    "vgroup_get: object has no `data.vertices` to enumerate -- pass `indices` explicitly",
                ));
            }
            let vg_index = vg.getattr("index")?.extract::<i64>()?;
            for v in mesh_data.getattr("vertices")?.try_iter()? {
                let v = v?;
                let vidx = v.getattr("index")?.extract::<i64>()?;
                for ve in v.getattr("groups")?.try_iter()? {
                    let ve = ve?;
                    if ve.getattr("group")?.extract::<i64>()? == vg_index {
                        weights.push(json!({
                            "index": vidx,
                            "weight": ve.getattr("weight")?.extract::<f64>()?,
                        }));
                    }
                }
            }
        }

        Ok(json!({ "name": p.object, "group_name": p.name, "weights": weights }))
    }
);

// ============================================================================
// Colour attributes
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ColorAttrList {
    /// Mesh object name. Only MESH objects carry colour attributes.
    pub object: String,
}

crate::tool!(
    "color_attr_list",
    "List a mesh's colour attributes -- Blender 3.2+'s replacement for vertex colours -- with name, data_type (FLOAT_COLOR/BYTE_COLOR), domain (POINT/CORNER), which one is active, and a sample of the actual colours stored. This is the colour-only view of attr_list, which shows every named attribute on the same mesh.",
    ColorAttrList,
    |py: Python<'_>, p: ColorAttrList| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.object)?;
        let mesh = obj.getattr("data")?;

        let active = mesh.getattr("color_attributes")?.getattr("active_color")?;
        let active_name = if active.is_none() {
            None
        } else {
            Some(active.getattr("name")?.extract::<String>()?)
        };

        Ok(json!({
            "name": p.object,
            "active": active_name,
            "attributes": list_attrs(&mesh, "color_attributes")?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ColorAttrAdd {
    /// Mesh object name.
    pub object: String,
    /// New colour attribute's name. Blender appends `.001` etc. on a collision;
    /// the resulting name is returned.
    pub name: String,
    /// FLOAT_COLOR (32-bit float per channel, linear) or BYTE_COLOR (8-bit per
    /// channel, sRGB storage). Defaults to FLOAT_COLOR. The underlying RNA call is
    /// the same as attr_add's and will accept any Attribute data_type (verified on
    /// 5.2.2), but only these two are colours to the rest of Blender.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// POINT (per vertex) or CORNER (per face corner / loop -- lets a colour
    /// differ on either side of a UV seam). Defaults to POINT. EDGE and FACE are
    /// also accepted by the RNA call on 5.2.2, but Vertex Paint and the Color
    /// Attribute node only handle POINT/CORNER.
    pub domain: Option<String>,
}

crate::tool!(
    "color_attr_add",
    "Add a colour attribute to a mesh -- what a shader's Color Attribute node and the Vertex Paint tools read and write. Use attr_add instead for non-colour named data, and color_attr_set to fill this one in.",
    ColorAttrAdd,
    |py: Python<'_>, p: ColorAttrAdd| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.object)?;
        let mesh = obj.getattr("data")?;

        let entry = add_attr(
            &mesh,
            "color_attributes",
            py,
            &p.name,
            &p.r#type.unwrap_or_else(|| "FLOAT_COLOR".to_string()),
            &p.domain.unwrap_or_else(|| "POINT".to_string()),
        )?;
        Ok(json!({ "name": p.object, "attribute": entry }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ColorAttrRemove {
    /// Mesh object name.
    pub object: String,
    /// Colour attribute to remove. A miss errors with the mesh's real list.
    pub name: String,
}

crate::tool!(
    "color_attr_remove",
    "Remove a colour attribute from a mesh. Removes it from `mesh.attributes` too -- the two collections are the same storage.",
    ColorAttrRemove,
    |py: Python<'_>, p: ColorAttrRemove| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.object)?;
        let mesh = obj.getattr("data")?;
        remove_attr(&mesh, "color_attributes", &p.name)?;
        Ok(json!({ "name": p.object, "removed": p.name }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ColorAttrSet {
    /// Mesh object name.
    pub object: String,
    /// Colour attribute to write into. It must already exist -- create it with
    /// color_attr_add.
    pub name: String,
    /// Item indices to write (point index for a POINT-domain attribute, loop
    /// index for CORNER). Omit to write every item in order; then `colors` must
    /// have exactly one entry per item.
    pub indices: Option<Vec<i64>>,
    /// One `[r, g, b, a]` per target item, same length and order as `indices` (or
    /// every item, if `indices` is omitted). Channels are 0..1; FLOAT_COLOR stores
    /// them linear, BYTE_COLOR quantises to 8 bits.
    pub colors: Vec<Vec<f64>>,
}

crate::tool!(
    "color_attr_set",
    "Write colours into a mesh colour attribute. The typed convenience form of attr_set (which takes the same targets as free-form `values`); read the result back with color_attr_list.",
    ColorAttrSet,
    |py: Python<'_>, p: ColorAttrSet| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.object)?;
        let mesh = obj.getattr("data")?;

        let values: Vec<Value> = p.colors.into_iter().map(|c| json!(c)).collect();
        let (field, written) =
            set_attr_values(py, &mesh, "color_attributes", &p.name, p.indices, &values)?;
        Ok(json!({ "name": p.object, "attr_name": p.name, "field": field, "written": written }))
    }
);

// ============================================================================
// Generic named attributes -- what geometry nodes actually read and write
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AttrList {
    /// Mesh object name. Only MESH objects expose `data.attributes`.
    pub object: String,
}

crate::tool!(
    "attr_list",
    "List every named attribute on a mesh: name, data_type, domain, and a sample of values. Includes Blender's own built-ins (position, .corner_vert, sharp_face, UVMap, ...) as well as user-created ones -- this is the full set geometry nodes' Named Attribute / Store Named Attribute can address. A superset of color_attr_list, which shows only the colour ones; vertex groups are not attributes and are listed by vgroup_list.",
    AttrList,
    |py: Python<'_>, p: AttrList| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.object)?;
        let mesh = obj.getattr("data")?;
        Ok(json!({ "name": p.object, "attributes": list_attrs(&mesh, "attributes")? }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AttrAdd {
    /// Mesh object name.
    pub object: String,
    /// New attribute's name. A leading `.` marks a Blender-internal attribute --
    /// do not use one. Blender appends `.001` etc. on a collision.
    pub name: String,
    /// One of the 14 data types a Mesh accepts on Blender 5.2.2: FLOAT, INT,
    /// BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8,
    /// INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR. Defaults to FLOAT. A wrong
    /// value errors with this Blender's real list, since the set moves across
    /// versions (5.0 added INT16_2D/INT32_2D/FLOAT4X4, 5.2 added FLOAT4).
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// POINT (per vertex), EDGE, FACE, or CORNER (per face corner / loop).
    /// Defaults to POINT. CURVE/INSTANCE/LAYER also exist in Blender's domain
    /// enum but a Mesh rejects them with `RuntimeError: Attribute domain not
    /// supported by this geometry type` (verified live on 5.2.2).
    pub domain: Option<String>,
}

crate::tool!(
    "attr_add",
    "Add a named attribute to a mesh, for geometry nodes to read via Named Attribute or write into via Store Named Attribute. Fill it with attr_set. For vertex colours prefer color_attr_add (same RNA call, colour-appropriate defaults); for modifier/armature weights use vgroup_add instead.",
    AttrAdd,
    |py: Python<'_>, p: AttrAdd| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.object)?;
        let mesh = obj.getattr("data")?;

        let entry = add_attr(
            &mesh,
            "attributes",
            py,
            &p.name,
            &p.r#type.unwrap_or_else(|| "FLOAT".to_string()),
            &p.domain.unwrap_or_else(|| "POINT".to_string()),
        )?;
        Ok(json!({ "name": p.object, "attribute": entry }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AttrRemove {
    /// Mesh object name.
    pub object: String,
    /// Attribute to remove. A miss errors with the mesh's real attribute list.
    pub name: String,
}

crate::tool!(
    "attr_remove",
    "Remove a named attribute from a mesh. Works on colour attributes too -- `mesh.attributes` and `mesh.color_attributes` are the same storage.",
    AttrRemove,
    |py: Python<'_>, p: AttrRemove| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.object)?;
        let mesh = obj.getattr("data")?;
        remove_attr(&mesh, "attributes", &p.name)?;
        Ok(json!({ "name": p.object, "removed": p.name }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct AttrSet {
    /// Mesh object name.
    pub object: String,
    /// Attribute to write into. It must already exist -- create it with attr_add.
    pub name: String,
    /// Item indices to write. What an index addresses depends on the attribute's
    /// domain: vertex for POINT, edge for EDGE, face for FACE, loop for CORNER.
    /// Omit to write every item in order.
    pub indices: Option<Vec<i64>>,
    /// One value per target item, same length and order as `indices` (or every
    /// item, if `indices` is omitted). The shape follows the attribute's
    /// data_type: FLOAT/INT/INT8 take a plain JSON number, BOOLEAN a JSON bool,
    /// STRING a JSON string; FLOAT_VECTOR takes `[x,y,z]`, FLOAT2/INT16_2D/INT32_2D
    /// a 2-element array, and FLOAT4/FLOAT_COLOR/BYTE_COLOR/QUATERNION a
    /// 4-element array. FLOAT4X4 takes either a nested `[[..4],[..4],[..4],[..4]]`
    /// or 16 loose numbers, which are folded into that shape for you. Blender
    /// stores a matrix column-major, so each group of four is a COLUMN: the
    /// translation of a transform is the last group, not the last element of each.
    pub values: Vec<Value>,
}

crate::tool!(
    "attr_set",
    "Write values into a named mesh attribute created by attr_add. Handles every data_type; for FLOAT_COLOR/BYTE_COLOR attributes color_attr_set is the typed equivalent. Inspect the result with attr_list, which samples the stored values.",
    AttrSet,
    |py: Python<'_>, p: AttrSet| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.object)?;
        let mesh = obj.getattr("data")?;

        let (field, written) =
            set_attr_values(py, &mesh, "attributes", &p.name, p.indices, &p.values)?;
        Ok(json!({ "name": p.object, "attr_name": p.name, "field": field, "written": written }))
    }
);
