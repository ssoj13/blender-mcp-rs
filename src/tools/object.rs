//! Object tools. Registered automatically — nothing outside this file names them.
//!
//! Each tool has a single-item / batch duality (e.g. `obj_update` takes either `name` +
//! fields or an `updates` array) — one round-trip can touch many objects, which matters
//! because MCP calls are expensive.

use std::collections::HashSet;

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyIterator, PyList, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, py_len, scene};

// ---------------------------------------------------------------------------
// obj_list
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjList {
    /// Only list objects of this type. Blender 5.2 accepts: MESH, CURVE, SURFACE,
    /// META, FONT, CURVES, POINTCLOUD, VOLUME, GREASEPENCIL, ARMATURE, LATTICE,
    /// EMPTY, LIGHT, LIGHT_PROBE, CAMERA, SPEAKER. Default: no filter
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Only list objects linked to this collection. Default: the whole file
    pub collection: Option<String>,
}

crate::tool!(
    "obj_list",
    "List objects with type, location and visibility, optionally filtered by object type \
     or collection. For one object's full detail (parent, children, mesh counts) use obj_get",
    ObjList,
    |py: Python<'_>, p: ObjList| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        let mut objs = d.getattr("objects")?;
        if let Some(coll_name) = p.collection.as_deref() {
            let colls = d.getattr("collections")?;
            if let Ok(coll) = colls.call_method1("get", (coll_name,))
                && !coll.is_none()
            {
                objs = coll.getattr("objects")?;
            }
        }

        let it = PyIterator::from_object(&objs)?;
        let mut rows = Vec::new();
        for item in it {
            let o = item?;
            let ty: String = o.getattr("type")?.extract()?;
            if let Some(tf) = p.r#type.as_deref()
                && ty != tf
            {
                continue;
            }
            let name: String = o.getattr("name")?.extract()?;
            let loc = py_vec(&o.getattr("location")?)?;
            let visible: bool = o.call_method0("visible_get")?.extract()?;
            rows.push(json!({
                "name": name,
                "type": ty,
                "location": loc,
                "visible": visible,
            }));
        }

        Ok(Value::Array(rows))
    }
);

// ---------------------------------------------------------------------------
// obj_get
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjGet {
    /// Object name, exactly as it appears in bpy.data.objects
    pub name: String,
}

crate::tool!(
    "obj_get",
    "Get one object's transform, visibility, parent and children, plus vertex/edge/polygon \
     counts and material slots for a mesh. To scan the whole scene instead use obj_list",
    ObjGet,
    |py: Python<'_>, p: ObjGet| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let objects = d.getattr("objects")?;
        let obj = objects.call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Object not found: {}",
                p.name
            )));
        }

        let ty: String = obj.getattr("type")?.extract()?;
        let mut r = json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "type": ty,
            "location": py_vec(&obj.getattr("location")?)?,
            "rotation": py_vec(&obj.getattr("rotation_euler")?)?,
            "scale": py_vec(&obj.getattr("scale")?)?,
            "visible": obj.call_method0("visible_get")?.extract::<bool>()?,
            "parent": parent_name(&obj)?,
            "children": children_names(&obj)?,
        });

        // Mesh sub-object: vertex/edge/poly counts and the material slot names.
        if ty == "MESH"
            && let Ok(data_ob) = obj.getattr("data")
            && !data_ob.is_none()
        {
            let m = data_ob;
            let verts = py_len(py, &m.getattr("vertices")?)?;
            let edges = py_len(py, &m.getattr("edges")?)?;
            let polys = py_len(py, &m.getattr("polygons")?)?;
            let materials = m.getattr("materials")?;
            let mat_it = PyIterator::from_object(&materials)?;
            let mut mat_names = Vec::new();
            for slot in mat_it {
                let slot = slot?;
                if slot.is_none() {
                    mat_names.push(Value::Null);
                } else {
                    let mn: Option<String> = slot.getattr("name")?.extract().ok();
                    mat_names.push(mn.map(Value::String).unwrap_or(Value::Null));
                }
            }
            r["mesh"] = json!({
                "verts": verts,
                "edges": edges,
                "polys": polys,
                "materials": mat_names,
            });
        }

        Ok(r)
    }
);

fn parent_name(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    let p = obj.getattr("parent")?;
    if p.is_none() {
        return Ok(Value::Null);
    }
    Ok(Value::String(p.getattr("name")?.extract::<String>()?))
}

fn children_names(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    let ch = obj.getattr("children")?;
    let it = PyIterator::from_object(&ch)?;
    let mut names = Vec::new();
    for c in it {
        names.push(Value::String(c?.getattr("name")?.extract::<String>()?));
    }
    Ok(Value::Array(names))
}

// ---------------------------------------------------------------------------
// obj_create_primitive
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjCreatePrimitive {
    /// Single mode: one of cube, sphere, cylinder, plane, cone, torus, monkey,
    /// circle, grid, icosphere (case-insensitive). Required unless `primitives` is given
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Name for the new object. Default: Blender's own ("Cube", "Sphere", ...)
    pub name: Option<String>,
    /// World position [x, y, z]. Default: [0, 0, 0]
    pub location: Option<Vec<f64>>,
    /// Object scale factors [x, y, z], applied after creation. Default: [1, 1, 1].
    /// Scale set here is not baked into the mesh — see obj_apply_transform
    pub scale: Option<Vec<f64>>,
    /// Batch mode: [{type, name?, location?, scale?}, ...]. Takes priority over the
    /// single-object fields above; a per-item failure is reported in `results`
    pub primitives: Option<Vec<PrimitiveSpec>>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PrimitiveSpec {
    /// cube, sphere, cylinder, plane, cone, torus, monkey, circle, grid, icosphere
    #[serde(rename = "type")]
    pub r#type: String,
    /// Name for the new object. Default: Blender's own
    pub name: Option<String>,
    /// World position [x, y, z]. Default: [0, 0, 0]
    pub location: Option<Vec<f64>>,
    /// Object scale factors [x, y, z]. Default: [1, 1, 1]
    pub scale: Option<Vec<f64>>,
}

crate::tool!(
    "obj_create_primitive",
    "Create primitive mesh objects — cube, sphere, cylinder, plane, cone, torus, monkey, \
     circle, grid, icosphere — one or a batch. To build a mesh from explicit vertex/face \
     lists instead use mesh_create; every primitive is flat-shaded until obj_shade",
    ObjCreatePrimitive,
    |py: Python<'_>, p: ObjCreatePrimitive| {
        let bpy = py.import("bpy")?;

        if let Some(prims) = p.primitives {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for prim in prims {
                match create_one_primitive(
                    py,
                    &bpy,
                    Some(prim.r#type.as_str()),
                    prim.name.as_deref(),
                    prim.location.as_deref(),
                    prim.scale.as_deref(),
                ) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    // Per-item failure inside a batch is data, not a call failure.
                    Err(e) => results.push(json!({ "type": prim.r#type, "error": e.to_string() })),
                }
            }
            return Ok(json!({ "created": ok, "results": results }));
        }

        create_one_primitive(
            py,
            &bpy,
            p.r#type.as_deref(),
            p.name.as_deref(),
            p.location.as_deref(),
            p.scale.as_deref(),
        )
    }
);

fn primitive_op_name(ty: &str) -> Option<&'static str> {
    match ty {
        "cube" => Some("primitive_cube_add"),
        "sphere" => Some("primitive_uv_sphere_add"),
        "cylinder" => Some("primitive_cylinder_add"),
        "plane" => Some("primitive_plane_add"),
        "cone" => Some("primitive_cone_add"),
        "torus" => Some("primitive_torus_add"),
        "monkey" => Some("primitive_monkey_add"),
        "circle" => Some("primitive_circle_add"),
        "grid" => Some("primitive_grid_add"),
        "icosphere" => Some("primitive_ico_sphere_add"),
        _ => None,
    }
}

fn create_one_primitive(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    ty_raw: Option<&str>,
    name: Option<&str>,
    location: Option<&[f64]>,
    scale: Option<&[f64]>,
) -> PyResult<Value> {
    let ty_raw = ty_raw.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing type"))?;
    let ty = ty_raw.to_lowercase();
    let Some(op_name) = primitive_op_name(&ty) else {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "unknown type: {ty_raw}"
        )));
    };

    let loc = loc3_or_default(location, [0.0, 0.0, 0.0]);

    let mesh = bpy.getattr("ops")?.getattr("mesh")?;
    let op = mesh.getattr(op_name)?;
    let kwargs = PyDict::new(py);
    kwargs.set_item("location", PyList::new(py, loc)?)?;
    op.call((), Some(&kwargs))?;

    let active = bpy.getattr("context")?.getattr("active_object")?;
    if active.is_none() {
        // Blender ran the add operator but left no active object behind — the
        // operation was attempted and failed, not a bad-input mismatch.
        return Err(pyo3::exceptions::PyRuntimeError::new_err(
            "No active object after primitive add",
        ));
    }

    if let Some(n) = name {
        active.setattr("name", n)?;
    }
    if let Some(sc) = scale
        && sc.len() == 3
    {
        active.setattr("scale", PyTuple::new(py, [sc[0], sc[1], sc[2]])?)?;
    }

    let name: String = active.getattr("name")?.extract()?;
    let otype: String = active.getattr("type")?.extract()?;
    let loc_py = active.getattr("location")?;
    Ok(json!({
        "name": name,
        "type": otype,
        "location": py_vec(&loc_py)?,
    }))
}

/// Fills the leading `1..=3` components from `v` into `default`, matching the
/// original tool's tolerant handling of a location shorter than three floats.
fn loc3_or_default(v: Option<&[f64]>, default: [f64; 3]) -> [f64; 3] {
    let Some(a) = v else {
        return default;
    };
    let mut out = default;
    for (i, x) in a.iter().take(3).enumerate() {
        out[i] = *x;
    }
    out
}

// ---------------------------------------------------------------------------
// obj_update
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjUpdate {
    /// Single mode: object name. Required unless `updates` is given
    pub name: Option<String>,
    /// Batch mode: [{name, location?, rotation?, scale?, visible?}, ...]. Takes
    /// priority over the single-object fields below
    pub updates: Option<Vec<ObjUpdateItem>>,
    /// World position [x, y, z], absolute (not a delta). Unchanged if omitted
    pub location: Option<Vec<f64>>,
    /// Euler rotation [x, y, z] in RADIANS, absolute. Unchanged if omitted
    pub rotation: Option<Vec<f64>>,
    /// Scale factors [x, y, z], absolute. Unchanged if omitted
    pub scale: Option<Vec<f64>>,
    /// false hides the object in both viewport and render, true shows it.
    /// Unchanged if omitted
    pub visible: Option<bool>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjUpdateItem {
    /// Object name
    pub name: String,
    /// World position [x, y, z], absolute
    pub location: Option<Vec<f64>>,
    /// Euler rotation [x, y, z] in RADIANS, absolute
    pub rotation: Option<Vec<f64>>,
    /// Scale factors [x, y, z], absolute
    pub scale: Option<Vec<f64>>,
    /// false hides in viewport and render, true shows
    pub visible: Option<bool>,
}

crate::tool!(
    "obj_update",
    "Set an object's location, Euler rotation (radians), scale and visibility — one \
     object or a batch. This moves the object and leaves its mesh alone; to bake the \
     transform into the mesh data use obj_apply_transform",
    ObjUpdate,
    |py: Python<'_>, p: ObjUpdate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let objects = d.getattr("objects")?;

        if let Some(updates) = p.updates {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for upd in updates {
                let obj = objects.call_method1("get", (upd.name.as_str(),))?;
                if obj.is_none() {
                    results.push(json!({ "name": upd.name, "error": "not found" }));
                    continue;
                }
                apply_object_update(
                    py,
                    &obj,
                    upd.location.as_deref(),
                    upd.rotation.as_deref(),
                    upd.scale.as_deref(),
                    upd.visible,
                )?;
                let loc = obj.getattr("location")?;
                results.push(json!({
                    "name": obj.getattr("name")?.extract::<String>()?,
                    "location": py_vec(&loc)?,
                }));
                ok += 1;
            }
            return Ok(json!({
                "updated": ok,
                "results": results,
            }));
        }

        let name = p
            .name
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing name"))?;
        let obj = objects.call_method1("get", (name.as_str(),))?;
        if obj.is_none() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Object not found: {name}"
            )));
        }
        apply_object_update(
            py,
            &obj,
            p.location.as_deref(),
            p.rotation.as_deref(),
            p.scale.as_deref(),
            p.visible,
        )?;
        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "location": py_vec(&obj.getattr("location")?)?,
            "rotation": py_vec(&obj.getattr("rotation_euler")?)?,
            "scale": py_vec(&obj.getattr("scale")?)?,
        }))
    }
);

fn apply_object_update(
    py: Python<'_>,
    obj: &Bound<'_, PyAny>,
    location: Option<&[f64]>,
    rotation: Option<&[f64]>,
    scale: Option<&[f64]>,
    visible: Option<bool>,
) -> PyResult<()> {
    if let Some(v) = location
        && v.len() == 3
    {
        obj.setattr("location", PyTuple::new(py, [v[0], v[1], v[2]])?)?;
    }
    if let Some(v) = rotation
        && v.len() == 3
    {
        obj.setattr("rotation_euler", PyTuple::new(py, [v[0], v[1], v[2]])?)?;
    }
    if let Some(v) = scale
        && v.len() == 3
    {
        obj.setattr("scale", PyTuple::new(py, [v[0], v[1], v[2]])?)?;
    }
    if let Some(vis) = visible {
        obj.setattr("hide_viewport", !vis)?;
        obj.setattr("hide_render", !vis)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// obj_delete
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjDelete {
    /// Single mode: object name. Used only when `names` is absent or empty
    pub name: Option<String>,
    /// Batch mode: object names to delete. Takes priority over `name`
    pub names: Option<Vec<String>>,
}

crate::tool!(
    "obj_delete",
    "Delete whole objects from the scene by name, one or a batch. To delete part of a \
     mesh instead, select it with mesh_select and run mesh_edit with op `delete`",
    ObjDelete,
    |py: Python<'_>, p: ObjDelete| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let objects = d.getattr("objects")?;

        let batch = p.names.unwrap_or_default();
        let names: Vec<String> = if !batch.is_empty() {
            batch
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
            let kwargs = PyDict::new(py);
            kwargs.set_item("do_unlink", true)?;
            objects.call_method("remove", (&obj,), Some(&kwargs))?;
            results.push(json!({ "deleted": name }));
        }

        // A bare result that is itself `{"error": ...}` is a call failure, not
        // data, and must still surface as `Err` even for a single-object call.
        if results.len() == 1 {
            let one = results.remove(0);
            if one.get("error").is_some() {
                let name = one.get("name").and_then(Value::as_str).unwrap_or_default();
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "object not found: {name}"
                )));
            }
            return Ok(one);
        }
        let deleted = results.iter().filter(|r| r.get("error").is_none()).count();
        Ok(json!({ "deleted": deleted, "results": results }))
    }
);

// ---------------------------------------------------------------------------
// obj_get_selection
// ---------------------------------------------------------------------------

/// `obj_get_selection` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjGetSelection {}

crate::tool!(
    "obj_get_selection",
    "Get the names of the currently selected objects plus the active object. \
     To change the selection use obj_set_selection",
    ObjGetSelection,
    |py: Python<'_>, _p: ObjGetSelection| {
        let bpy = py.import("bpy")?;
        let ctx = bpy.getattr("context")?;
        let selected = ctx.getattr("selected_objects")?;
        let it = PyIterator::from_object(&selected)?;
        let mut names = Vec::new();
        for o in it {
            names.push(o?.getattr("name")?.extract::<String>()?);
        }
        let count = names.len();
        let active = ctx.getattr("active_object")?;
        let active_val = if active.is_none() {
            Value::Null
        } else {
            Value::String(active.getattr("name")?.extract::<String>()?)
        };
        Ok(json!({
            "selected": names,
            "active": active_val,
            "count": count,
        }))
    }
);

// ---------------------------------------------------------------------------
// obj_set_selection
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjSetSelection {
    /// Objects to select. Everything is deselected first, so this replaces the
    /// selection rather than adding to it. Names that do not exist are skipped.
    /// Default: empty (deselect everything)
    pub names: Option<Vec<String>>,
    /// Object to make active (the one operators act on). Default: unchanged
    pub active: Option<String>,
}

crate::tool!(
    "obj_set_selection",
    "Replace the object selection by name and optionally set the active object. Most \
     tools here take an explicit `name` and need no selection; read the current one \
     with obj_get_selection",
    ObjSetSelection,
    |py: Python<'_>, p: ObjSetSelection| {
        let bpy = py.import("bpy")?;
        let object = bpy.getattr("ops")?.getattr("object")?;
        let kwargs = PyDict::new(py);
        kwargs.set_item("action", "DESELECT")?;
        object.call_method("select_all", (), Some(&kwargs))?;

        let d = data(&bpy)?;
        let objects = d.getattr("objects")?;

        let names = p.names.unwrap_or_default();
        for name in &names {
            let obj = objects.call_method1("get", (name.as_str(),))?;
            if !obj.is_none() {
                obj.call_method1("select_set", (true,))?;
            }
        }

        if let Some(active) = p.active.as_deref() {
            let obj = objects.call_method1("get", (active,))?;
            if !obj.is_none() {
                let vl = bpy.getattr("context")?.getattr("view_layer")?;
                vl.getattr("objects")?.setattr("active", &obj)?;
            }
        }

        Ok(json!({ "selected": names }))
    }
);

// ---------------------------------------------------------------------------
// obj_duplicate
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjDuplicate {
    /// Single mode: object to copy. Required unless `duplicates` is given
    pub name: Option<String>,
    /// Name for the copy. Default: Blender's own ("Cube.001", ...)
    pub new_name: Option<String>,
    /// Offset [x, y, z] added to the source's location. Default: [0, 0, 0] —
    /// the copy lands exactly on top of the original
    pub offset: Option<Vec<f64>>,
    /// true = the copy shares the source's mesh data, so editing one edits both.
    /// false = independent copy of the data. Default: false
    pub linked: Option<bool>,
    /// Batch mode: [{name, new_name?, offset?, linked?}, ...]. Takes priority over
    /// the single-object fields above
    pub duplicates: Option<Vec<ObjDuplicateItem>>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjDuplicateItem {
    /// Object to copy
    pub name: String,
    /// Name for the copy. Default: Blender's own
    pub new_name: Option<String>,
    /// Offset [x, y, z] from the source's location. Default: [0, 0, 0]
    pub offset: Option<Vec<f64>>,
    /// true = share the source's mesh data. Default: false
    pub linked: Option<bool>,
}

crate::tool!(
    "obj_duplicate",
    "Duplicate objects with an optional offset, either as independent copies or sharing \
     (linked) mesh data. To merge objects back into one use obj_join",
    ObjDuplicate,
    |py: Python<'_>, p: ObjDuplicate| {
        let bpy = py.import("bpy")?;

        if let Some(dups) = p.duplicates {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for d in dups {
                match duplicate_one(
                    py,
                    &bpy,
                    &d.name,
                    d.new_name.as_deref(),
                    d.offset.as_deref(),
                    d.linked.unwrap_or(false),
                ) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    // Per-item failure inside a batch is data, not a call failure.
                    Err(e) => results.push(json!({ "name": d.name, "error": e.to_string() })),
                }
            }
            return Ok(json!({ "created": ok, "results": results }));
        }

        let name = p
            .name
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing name"))?;
        duplicate_one(
            py,
            &bpy,
            &name,
            p.new_name.as_deref(),
            p.offset.as_deref(),
            p.linked.unwrap_or(false),
        )
    }
);

fn duplicate_one(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    name: &str,
    new_name: Option<&str>,
    offset: Option<&[f64]>,
    linked: bool,
) -> PyResult<Value> {
    let d = data(bpy)?;
    let objects = d.getattr("objects")?;
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "object not found: {name}"
        )));
    }
    let new_obj = obj.call_method0("copy")?;
    let data_ob = obj.getattr("data")?;
    if !data_ob.is_none() {
        if linked {
            new_obj.setattr("data", &data_ob)?;
        } else {
            let copy = data_ob.call_method0("copy")?;
            new_obj.setattr("data", copy)?;
        }
    }
    let coll = bpy.getattr("context")?.getattr("collection")?;
    coll.getattr("objects")?.call_method1("link", (&new_obj,))?;

    if let Some(nn) = new_name {
        new_obj.setattr("name", nn)?;
    }
    if let Some(off) = offset
        && off.len() == 3
    {
        let loc = py_vec(&obj.getattr("location")?)?;
        let nl = [loc[0] + off[0], loc[1] + off[1], loc[2] + off[2]];
        new_obj.setattr("location", PyTuple::new(py, nl)?)?;
    }

    let orig: String = obj.getattr("name")?.extract()?;
    let dup: String = new_obj.getattr("name")?.extract()?;
    let loc = new_obj.getattr("location")?;
    Ok(json!({
        "original": orig,
        "duplicate": dup,
        "location": py_vec(&loc)?,
    }))
}

// ---------------------------------------------------------------------------
// obj_parent
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjParent {
    /// Single mode: the object that becomes the parent. Required unless
    /// `operations` is given
    pub parent: Option<String>,
    /// Single mode: objects that become children of `parent`. Names that do not
    /// exist are skipped. Default: empty
    pub children: Option<Vec<String>>,
    /// true keeps each child where it is in world space (sets
    /// `matrix_parent_inverse`); false lets the child inherit the parent's
    /// transform and jump. Default: true
    pub keep_transform: Option<bool>,
    /// Batch mode: [{parent, children, keep_transform?}, ...]. Takes priority over
    /// the single-operation fields above
    pub operations: Option<Vec<ObjParentOp>>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjParentOp {
    /// Object that becomes the parent
    pub parent: String,
    /// Objects that become its children
    pub children: Vec<String>,
    /// true keeps each child's world transform. Default: true
    pub keep_transform: Option<bool>,
}

crate::tool!(
    "obj_parent",
    "Parent objects to another object, by default keeping each child's world transform. \
     Read the resulting hierarchy back with obj_get",
    ObjParent,
    |py: Python<'_>, p: ObjParent| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let objects = d.getattr("objects")?;

        if let Some(ops) = p.operations {
            let mut results = Vec::new();
            for op in ops {
                let parent = objects.call_method1("get", (op.parent.as_str(),))?;
                if parent.is_none() {
                    results.push(json!({ "parent": op.parent, "error": "parent not found" }));
                    continue;
                }
                let keep = op.keep_transform.unwrap_or(true);
                let mut parented = Vec::new();
                for cn in &op.children {
                    let child = objects.call_method1("get", (cn.as_str(),))?;
                    if child.is_none() {
                        continue;
                    }
                    child.setattr("parent", &parent)?;
                    if keep {
                        let mw = parent.getattr("matrix_world")?;
                        let inv = mw.call_method0("inverted")?;
                        child.setattr("matrix_parent_inverse", inv)?;
                    }
                    parented.push(cn.clone());
                }
                let pn: String = parent.getattr("name")?.extract()?;
                results.push(json!({ "parent": pn, "children": parented }));
            }
            return Ok(json!({
                "operations": results.len(),
                "results": results,
            }));
        }

        let parent_name = p
            .parent
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing parent"))?;
        let parent = objects.call_method1("get", (parent_name.as_str(),))?;
        if parent.is_none() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Parent not found: {parent_name}"
            )));
        }
        let children = p.children.unwrap_or_default();
        let keep = p.keep_transform.unwrap_or(true);
        for cn in &children {
            let child = objects.call_method1("get", (cn.as_str(),))?;
            if child.is_none() {
                continue;
            }
            child.setattr("parent", &parent)?;
            if keep {
                let mw = parent.getattr("matrix_world")?;
                let inv = mw.call_method0("inverted")?;
                child.setattr("matrix_parent_inverse", inv)?;
            }
        }
        let pn: String = parent.getattr("name")?.extract()?;
        Ok(json!({ "parent": pn, "children": children }))
    }
);

// ---------------------------------------------------------------------------
// shared helper
// ---------------------------------------------------------------------------

/// Extract a Python float sequence (e.g. `location`, `scale`) into a `Vec<f64>`.
fn py_vec(seq: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    let it = PyIterator::from_object(seq)?;
    let mut v = Vec::new();
    for x in it {
        v.push(x?.extract::<f64>()?);
    }
    Ok(v)
}

// ---------------------------------------------------------------------------
// Shared context guard for bpy.ops.object.* — they work off "currently
// selected", not explicit object arguments, so every tool below that drives
// one must stage selection and put it back afterward. Verified live against
// Blender 5.0 (blender.exe --background --python probe): shade_smooth,
// shade_flat, shade_auto_smooth, transform_apply, join, mesh.separate and
// origin_set all read `context.selected_objects` / `context.active_object`.
// ---------------------------------------------------------------------------

/// Snapshot of object selection, the active object, and the active object's
/// mode — exactly what the operators above mutate as a side effect of doing
/// their job. `select_only` stages what a call actually needs; `restore` puts
/// everything back so the call is invisible to whatever state (a user's
/// viewport, a previous tool call) existed before it ran.
struct ObjCtx {
    selected_names: Vec<String>,
    active_name: Option<String>,
    active_mode: String,
}

impl ObjCtx {
    fn capture(bpy: &Bound<'_, PyAny>) -> PyResult<Self> {
        let ctx = bpy.getattr("context")?;
        let mut selected_names = Vec::new();
        for o in PyIterator::from_object(&ctx.getattr("selected_objects")?)? {
            selected_names.push(o?.getattr("name")?.extract::<String>()?);
        }
        let active = ctx.getattr("active_object")?;
        let (active_name, active_mode) = if active.is_none() {
            (None, "OBJECT".to_string())
        } else {
            (
                Some(active.getattr("name")?.extract::<String>()?),
                active.getattr("mode")?.extract::<String>()?,
            )
        };
        Ok(Self {
            selected_names,
            active_name,
            active_mode,
        })
    }

    /// Deselects everything, selects exactly `names` (silently skipping any
    /// that do not exist), and makes `active` the active object. Always
    /// leaves OBJECT mode, which is what every caller here needs before it
    /// runs its operator.
    fn select_only(
        py: Python<'_>,
        bpy: &Bound<'_, PyAny>,
        names: &[&str],
        active: &str,
    ) -> PyResult<()> {
        ensure_object_mode(py, bpy)?;
        deselect_all(py, bpy)?;

        let d = data(bpy)?;
        let objects = d.getattr("objects")?;
        for name in names {
            let o = objects.call_method1("get", (*name,))?;
            if !o.is_none() {
                o.call_method1("select_set", (true,))?;
            }
        }
        let active_obj = objects.call_method1("get", (active,))?;
        if !active_obj.is_none() {
            bpy.getattr("context")?
                .getattr("view_layer")?
                .getattr("objects")?
                .setattr("active", &active_obj)?;
        }
        Ok(())
    }

    /// Puts selection, active object and mode back the way `capture` found
    /// them. Objects that no longer exist (e.g. deleted by `obj_join`) are
    /// skipped rather than erroring — that is a legitimate outcome of the
    /// operation, not a restore failure.
    fn restore(&self, py: Python<'_>, bpy: &Bound<'_, PyAny>) -> PyResult<()> {
        ensure_object_mode(py, bpy)?;
        deselect_all(py, bpy)?;

        let d = data(bpy)?;
        let objects = d.getattr("objects")?;
        for name in &self.selected_names {
            let o = objects.call_method1("get", (name.as_str(),))?;
            if !o.is_none() {
                o.call_method1("select_set", (true,))?;
            }
        }
        if let Some(name) = &self.active_name {
            let o = objects.call_method1("get", (name.as_str(),))?;
            if !o.is_none() {
                bpy.getattr("context")?
                    .getattr("view_layer")?
                    .getattr("objects")?
                    .setattr("active", &o)?;
                if self.active_mode != "OBJECT" {
                    set_mode(py, bpy, &self.active_mode)?;
                }
            }
        }
        Ok(())
    }
}

/// `bpy.ops.object.mode_set(mode='OBJECT')`, but only when needed — calling it
/// with no active object raises, and a caller here may run with none set.
fn ensure_object_mode(py: Python<'_>, bpy: &Bound<'_, PyAny>) -> PyResult<()> {
    let active = bpy.getattr("context")?.getattr("active_object")?;
    if active.is_none() {
        return Ok(());
    }
    let mode: String = active.getattr("mode")?.extract()?;
    if mode != "OBJECT" {
        set_mode(py, bpy, "OBJECT")?;
    }
    Ok(())
}

/// `bpy.ops.object.mode_set(mode=...)`.
fn set_mode(py: Python<'_>, bpy: &Bound<'_, PyAny>, mode: &str) -> PyResult<()> {
    let kwargs = PyDict::new(py);
    kwargs.set_item("mode", mode)?;
    bpy.getattr("ops")?
        .getattr("object")?
        .call_method("mode_set", (), Some(&kwargs))?;
    Ok(())
}

/// `bpy.ops.object.select_all(action='DESELECT')`.
fn deselect_all(py: Python<'_>, bpy: &Bound<'_, PyAny>) -> PyResult<()> {
    let kwargs = PyDict::new(py);
    kwargs.set_item("action", "DESELECT")?;
    bpy.getattr("ops")?
        .getattr("object")?
        .call_method("select_all", (), Some(&kwargs))?;
    Ok(())
}

/// Runs `f`, always restoring the [`ObjCtx`] snapshot taken just before it —
/// regardless of whether `f` itself succeeded. If both `f` and the restore
/// fail, the error from `f` is what the caller sees: that is the failure they
/// can act on, a best-effort context restore failing afterward is secondary.
fn with_restored_context<T>(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    f: impl FnOnce() -> PyResult<T>,
) -> PyResult<T> {
    let ctx = ObjCtx::capture(bpy)?;
    let outcome = f();
    let restored = ctx.restore(py, bpy);
    match (outcome, restored) {
        (Ok(v), Ok(())) => Ok(v),
        (Ok(_), Err(e)) => Err(e),
        (Err(e), _) => Err(e),
    }
}

/// `bpy.ops` calls return a Python `set` of result tags instead of raising
/// when their poll passed but nothing actually happened — verified live:
/// `transform_apply` with nothing selected and `join` with fewer than two
/// compatible objects both return `{'CANCELLED'}` silently. Accepting that as
/// success would violate the tool error contract, so every operator call in
/// this module is checked against it.
fn require_finished(result: &Bound<'_, PyAny>, what: &str) -> PyResult<()> {
    let mut tags = Vec::new();
    for t in PyIterator::from_object(result)? {
        tags.push(t?.extract::<String>()?);
    }
    if tags.iter().any(|t| t == "FINISHED") {
        return Ok(());
    }
    Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
        "{what}: operator did not finish ({})",
        tags.join(", ")
    )))
}

/// Reads the identifiers of an operator's RNA enum property, e.g. `type` on
/// `bpy.ops.mesh.separate` or `center` on `bpy.ops.object.origin_set`, and
/// checks `value` against them. Operator enums are not a stable, hardcodable
/// table across Blender versions (`mesh.separate`'s `type` and
/// `origin_set`'s `center`/`type` were read live from Blender 5.0, not
/// assumed) — this is what "ask Blender" means for every enum value this
/// module passes into an operator call.
fn validate_op_enum(op: &Bound<'_, PyAny>, prop: &str, value: &str) -> PyResult<String> {
    let rna = op.call_method0("get_rna_type")?;
    let props = rna.getattr("properties")?;
    let p = props.call_method1("get", (prop,))?;
    if p.is_none() {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "operator has no `{prop}` property"
        )));
    }
    let mut items = Vec::new();
    for item in PyIterator::from_object(&p.getattr("enum_items")?)? {
        items.push(item?.getattr("identifier")?.extract::<String>()?);
    }
    let up = value.to_uppercase();
    if items.iter().any(|i| i == &up) {
        Ok(up)
    } else {
        Err(pyo3::exceptions::PyValueError::new_err(format!(
            "invalid {prop} `{value}`. This Blender accepts: {}",
            items.join(", ")
        )))
    }
}

// ---------------------------------------------------------------------------
// obj_shade
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjShade {
    /// Single mode: mesh object name. Required unless `objects` is given
    pub name: Option<String>,
    /// SMOOTH, FLAT, or AUTO_SMOOTH. AUTO_SMOOTH adds/updates a "Smooth by
    /// Angle" geometry-nodes modifier — this is what replaced the
    /// `mesh.use_auto_smooth` property Blender 4.1 removed
    pub mode: Option<String>,
    /// Degrees, for AUTO_SMOOTH. Blender's own operator default is 30
    pub angle: Option<f64>,
    /// For SMOOTH/FLAT only: keep edges marked Sharp from being smoothed
    /// over. Blender's own default is true
    pub keep_sharp_edges: Option<bool>,
    /// Batch mode: [{name, mode, angle?, keep_sharp_edges?}, ...]. Takes priority
    /// over the single-object fields above
    pub objects: Option<Vec<ObjShadeItem>>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjShadeItem {
    /// Mesh object name
    pub name: String,
    /// SMOOTH, FLAT, or AUTO_SMOOTH
    pub mode: String,
    /// Degrees, for AUTO_SMOOTH. Default 30
    pub angle: Option<f64>,
    /// SMOOTH/FLAT only: keep edges marked Sharp. Default true
    pub keep_sharp_edges: Option<bool>,
}

crate::tool!(
    "obj_shade",
    "Shade a mesh smooth, flat, or auto-smooth-by-angle. Every primitive Blender \
     creates is faceted by default; this is the only way to fix that. For per-edge \
     control instead, mark individual edges sharp with mesh_mark",
    ObjShade,
    |py: Python<'_>, p: ObjShade| {
        let bpy = py.import("bpy")?;

        if let Some(items) = p.objects {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for it in items {
                match shade_one(py, &bpy, &it.name, &it.mode, it.angle, it.keep_sharp_edges) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    Err(e) => results.push(json!({ "name": it.name, "error": e.to_string() })),
                }
            }
            return Ok(json!({ "shaded": ok, "results": results }));
        }

        let name = p
            .name
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing name"))?;
        let mode = p
            .mode
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing mode"))?;
        shade_one(py, &bpy, &name, &mode, p.angle, p.keep_sharp_edges)
    }
);

fn shade_one(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    name: &str,
    mode: &str,
    angle_deg: Option<f64>,
    keep_sharp_edges: Option<bool>,
) -> PyResult<Value> {
    let d = data(bpy)?;
    let objects = d.getattr("objects")?;
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "object not found: {name}"
        )));
    }
    let ty: String = obj.getattr("type")?.extract()?;
    if ty != "MESH" {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "obj_shade requires a MESH object, `{name}` is {ty}"
        )));
    }

    let up = mode.to_uppercase();
    let object_ops = bpy.getattr("ops")?.getattr("object")?;

    with_restored_context(py, bpy, || {
        ObjCtx::select_only(py, bpy, &[name], name)?;

        let result = match up.as_str() {
            "SMOOTH" => {
                let kwargs = PyDict::new(py);
                kwargs.set_item("keep_sharp_edges", keep_sharp_edges.unwrap_or(true))?;
                object_ops.call_method("shade_smooth", (), Some(&kwargs))?
            }
            "FLAT" => {
                let kwargs = PyDict::new(py);
                kwargs.set_item("keep_sharp_edges", keep_sharp_edges.unwrap_or(true))?;
                object_ops.call_method("shade_flat", (), Some(&kwargs))?
            }
            "AUTO_SMOOTH" => {
                let kwargs = PyDict::new(py);
                kwargs.set_item("angle", angle_deg.unwrap_or(30.0).to_radians())?;
                kwargs.set_item("use_auto_smooth", true)?;
                object_ops.call_method("shade_auto_smooth", (), Some(&kwargs))?
            }
            other => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "unknown shade mode `{other}`: expected SMOOTH, FLAT, or AUTO_SMOOTH"
                )));
            }
        };
        require_finished(&result, "shade")
    })?;

    let mut out = json!({ "name": name, "mode": up });
    if up == "AUTO_SMOOTH" {
        let modifiers = obj.getattr("modifiers")?;
        let m = modifiers.call_method1("get", ("Smooth by Angle",))?;
        if !m.is_none() {
            // The live per-modifier value, not the shared node-group default:
            // reading `node_group.interface` here would report the *first*
            // object ever shaded this way, not this one's current angle.
            let angle_rad: f64 =
                crate::tools::bpy::read_modifier_input(&m, "Input_1")?.extract()?;
            out["angle_deg"] = json!(angle_rad.to_degrees());
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// obj_apply_transform
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjApplyTransform {
    /// Single mode: object name. Required unless `objects` is given
    pub name: Option<String>,
    /// Bake the location in and move the origin to the world origin. Default true
    /// (Blender's own operator default) — pass false to keep the origin put
    pub location: Option<bool>,
    /// Bake the rotation in and reset rotation_euler to [0, 0, 0]. Default true
    pub rotation: Option<bool>,
    /// Bake the scale in and reset scale to [1, 1, 1]. Default true. Un-applied
    /// scale silently breaks Bevel/Solidify/Array widths and thickness — this is
    /// the fix
    pub scale: Option<bool>,
    /// Batch mode: [{name, location?, rotation?, scale?}, ...]. Takes priority over
    /// the single-object fields above
    pub objects: Option<Vec<ObjApplyTransformItem>>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjApplyTransformItem {
    /// Object name
    pub name: String,
    /// Bake the location in. Default true
    pub location: Option<bool>,
    /// Bake the rotation in. Default true
    pub rotation: Option<bool>,
    /// Bake the scale in. Default true
    pub scale: Option<bool>,
}

crate::tool!(
    "obj_apply_transform",
    "Bake location/rotation/scale into the mesh data and reset the object's transform \
     to identity. obj_update moves an object; this is what freezes that move into the \
     mesh. Un-applied scale is invisible in the viewport but silently breaks \
     Bevel/Solidify/Array widths",
    ObjApplyTransform,
    |py: Python<'_>, p: ObjApplyTransform| {
        let bpy = py.import("bpy")?;

        if let Some(items) = p.objects {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for it in items {
                match apply_transform_one(py, &bpy, &it.name, it.location, it.rotation, it.scale) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    Err(e) => results.push(json!({ "name": it.name, "error": e.to_string() })),
                }
            }
            return Ok(json!({ "applied": ok, "results": results }));
        }

        let name = p
            .name
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing name"))?;
        apply_transform_one(py, &bpy, &name, p.location, p.rotation, p.scale)
    }
);

fn apply_transform_one(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    name: &str,
    location: Option<bool>,
    rotation: Option<bool>,
    scale: Option<bool>,
) -> PyResult<Value> {
    let d = data(bpy)?;
    let objects = d.getattr("objects")?;
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "object not found: {name}"
        )));
    }

    let loc = location.unwrap_or(true);
    let rot = rotation.unwrap_or(true);
    let scl = scale.unwrap_or(true);

    with_restored_context(py, bpy, || {
        ObjCtx::select_only(py, bpy, &[name], name)?;
        let kwargs = PyDict::new(py);
        kwargs.set_item("location", loc)?;
        kwargs.set_item("rotation", rot)?;
        kwargs.set_item("scale", scl)?;
        let result = bpy.getattr("ops")?.getattr("object")?.call_method(
            "transform_apply",
            (),
            Some(&kwargs),
        )?;
        require_finished(&result, "transform_apply")
    })?;

    Ok(json!({
        "name": name,
        "location": py_vec(&obj.getattr("location")?)?,
        "rotation": py_vec(&obj.getattr("rotation_euler")?)?,
        "scale": py_vec(&obj.getattr("scale")?)?,
    }))
}

// ---------------------------------------------------------------------------
// obj_join
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjJoin {
    /// Single mode: the object that survives, keeping its name, transform and
    /// material slots. Required unless `operations` is given
    pub target: Option<String>,
    /// Single mode: objects merged into `target` and removed from the scene. Must
    /// all exist and be the same object type as `target`; must not be empty
    pub sources: Option<Vec<String>>,
    /// Batch mode: [{target, sources}, ...]. Takes priority over the single-operation
    /// fields above
    pub operations: Option<Vec<ObjJoinOp>>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjJoinOp {
    /// The object that survives
    pub target: String,
    /// Objects merged into `target` and deleted
    pub sources: Vec<String>,
}

crate::tool!(
    "obj_join",
    "Join objects into one. `target` survives; every object in `sources` is \
     merged into it and removed from the scene. The inverse is obj_separate",
    ObjJoin,
    |py: Python<'_>, p: ObjJoin| {
        let bpy = py.import("bpy")?;

        if let Some(ops) = p.operations {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for op in ops {
                match join_one(py, &bpy, &op.target, &op.sources) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    Err(e) => results.push(json!({ "target": op.target, "error": e.to_string() })),
                }
            }
            return Ok(json!({ "joined": ok, "results": results }));
        }

        let target = p
            .target
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing target"))?;
        let sources = p
            .sources
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing sources"))?;
        join_one(py, &bpy, &target, &sources)
    }
);

fn join_one(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    target: &str,
    sources: &[String],
) -> PyResult<Value> {
    if sources.is_empty() {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "sources is empty: nothing to join",
        ));
    }

    let d = data(bpy)?;
    let objects = d.getattr("objects")?;
    let target_obj = objects.call_method1("get", (target,))?;
    if target_obj.is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "target not found: {target}"
        )));
    }
    // Validate every source up front: a join that silently skipped a typo'd
    // name would change the final geometry without saying so.
    for s in sources {
        if objects.call_method1("get", (s.as_str(),))?.is_none() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "source not found: {s}"
            )));
        }
    }

    let mut names: Vec<&str> = vec![target];
    names.extend(sources.iter().map(String::as_str));

    with_restored_context(py, bpy, || {
        ObjCtx::select_only(py, bpy, &names, target)?;
        let result = bpy
            .getattr("ops")?
            .getattr("object")?
            .call_method0("join")?;
        require_finished(&result, "join")
    })?;

    let verts = if target_obj.getattr("type")?.extract::<String>()? == "MESH" {
        Some(py_len(
            py,
            &target_obj.getattr("data")?.getattr("vertices")?,
        )?)
    } else {
        None
    };

    Ok(json!({
        "target": target,
        "sources_joined": sources,
        "vertices": verts,
    }))
}

// ---------------------------------------------------------------------------
// obj_separate
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjSeparate {
    /// Single mode: object name (must be a mesh). Required unless `objects` is given
    pub name: Option<String>,
    /// LOOSE (one object per disconnected part), MATERIAL (one per material slot),
    /// or SELECTED (split off the mesh's stored selection — build it with
    /// mesh_select first). Validated live against this Blender's own
    /// `mesh.separate` RNA enum; on 5.2 those three are the whole list
    pub mode: Option<String>,
    /// Batch mode: [{name, mode}, ...]. Takes priority over the single-object
    /// fields above
    pub objects: Option<Vec<ObjSeparateItem>>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjSeparateItem {
    /// Mesh object to split
    pub name: String,
    /// LOOSE, MATERIAL, or SELECTED
    pub mode: String,
}

crate::tool!(
    "obj_separate",
    "Split a mesh into several objects: by loose (disconnected) parts, by material \
     slot, or by the selection stored with mesh_select. The inverse is obj_join",
    ObjSeparate,
    |py: Python<'_>, p: ObjSeparate| {
        let bpy = py.import("bpy")?;

        if let Some(items) = p.objects {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for it in items {
                match separate_one(py, &bpy, &it.name, &it.mode) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    Err(e) => results.push(json!({ "name": it.name, "error": e.to_string() })),
                }
            }
            return Ok(json!({ "separated": ok, "results": results }));
        }

        let name = p
            .name
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing name"))?;
        let mode = p
            .mode
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing mode"))?;
        separate_one(py, &bpy, &name, &mode)
    }
);

fn separate_one(py: Python<'_>, bpy: &Bound<'_, PyAny>, name: &str, mode: &str) -> PyResult<Value> {
    let d = data(bpy)?;
    let objects = d.getattr("objects")?;
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "object not found: {name}"
        )));
    }
    let ty: String = obj.getattr("type")?.extract()?;
    if ty != "MESH" {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "obj_separate requires a MESH object, `{name}` is {ty}"
        )));
    }

    let separate_op = bpy.getattr("ops")?.getattr("mesh")?.getattr("separate")?;
    let sep_type = validate_op_enum(&separate_op, "type", mode)?;

    let mut before = HashSet::new();
    for o in PyIterator::from_object(&objects)? {
        before.insert(o?.getattr("name")?.extract::<String>()?);
    }

    with_restored_context(py, bpy, || {
        ObjCtx::select_only(py, bpy, &[name], name)?;
        set_mode(py, bpy, "EDIT")?;
        let kwargs = PyDict::new(py);
        kwargs.set_item("type", sep_type.as_str())?;
        let result = separate_op.call((), Some(&kwargs))?;
        require_finished(&result, "separate")?;
        set_mode(py, bpy, "OBJECT")
    })?;

    let mut created = Vec::new();
    for o in PyIterator::from_object(&objects)? {
        let n = o?.getattr("name")?.extract::<String>()?;
        if !before.contains(&n) {
            created.push(n);
        }
    }

    Ok(json!({ "name": name, "mode": sep_type, "created": created }))
}

// ---------------------------------------------------------------------------
// obj_origin
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjOrigin {
    /// Single mode: object name. Required unless `objects` is given
    pub name: Option<String>,
    /// GEOMETRY (move origin to the object's own geometry) or CURSOR (move
    /// origin to the 3D cursor — combine with `cursor_position` to target an
    /// arbitrary world-space point, since there is no viewport to click in).
    /// These two are the only values this tool accepts
    pub mode: Option<String>,
    /// For GEOMETRY only: MEDIAN (average of the vertices) or BOUNDS (centre of
    /// the bounding box), validated live against this Blender's own RNA — on 5.2
    /// those two are the whole list. Default: MEDIAN
    pub center: Option<String>,
    /// [x, y, z] world position. Moves the 3D cursor there before resolving
    /// CURSOR mode; the cursor is left there afterward, same as a manual
    /// Shift+Right-click placement would
    pub cursor_position: Option<Vec<f64>>,
    /// Batch mode: [{name, mode, center?, cursor_position?}, ...]. Takes priority
    /// over the single-object fields above
    pub objects: Option<Vec<ObjOriginItem>>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjOriginItem {
    /// Object name
    pub name: String,
    /// GEOMETRY or CURSOR
    pub mode: String,
    /// GEOMETRY only: MEDIAN or BOUNDS. Default MEDIAN
    pub center: Option<String>,
    /// [x, y, z] world position to move the 3D cursor to first
    pub cursor_position: Option<Vec<f64>>,
}

crate::tool!(
    "obj_origin",
    "Move an object's origin to its own geometry (median or bounds center) or to the \
     3D cursor, leaving the geometry in place. Pass cursor_position to target an \
     arbitrary world-space point. To move the geometry under a fixed origin instead, \
     use obj_apply_transform",
    ObjOrigin,
    |py: Python<'_>, p: ObjOrigin| {
        let bpy = py.import("bpy")?;

        if let Some(items) = p.objects {
            let mut results = Vec::new();
            let mut ok = 0usize;
            for it in items {
                match origin_one(
                    py,
                    &bpy,
                    &it.name,
                    &it.mode,
                    it.center.as_deref(),
                    it.cursor_position.as_deref(),
                ) {
                    Ok(r) => {
                        ok += 1;
                        results.push(r);
                    }
                    Err(e) => results.push(json!({ "name": it.name, "error": e.to_string() })),
                }
            }
            return Ok(json!({ "updated": ok, "results": results }));
        }

        let name = p
            .name
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing name"))?;
        let mode = p
            .mode
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing mode"))?;
        origin_one(
            py,
            &bpy,
            &name,
            &mode,
            p.center.as_deref(),
            p.cursor_position.as_deref(),
        )
    }
);

fn origin_one(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    name: &str,
    mode: &str,
    center: Option<&str>,
    cursor_position: Option<&[f64]>,
) -> PyResult<Value> {
    let d = data(bpy)?;
    let objects = d.getattr("objects")?;
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "object not found: {name}"
        )));
    }

    if let Some(pos) = cursor_position {
        if pos.len() != 3 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "cursor_position needs exactly 3 components",
            ));
        }
        let cursor = scene(bpy)?.getattr("cursor")?;
        cursor.setattr("location", PyTuple::new(py, [pos[0], pos[1], pos[2]])?)?;
    }

    let origin_set = bpy
        .getattr("ops")?
        .getattr("object")?
        .getattr("origin_set")?;
    let kwargs = PyDict::new(py);
    match mode.to_uppercase().as_str() {
        "GEOMETRY" => {
            let c = validate_op_enum(&origin_set, "center", center.unwrap_or("MEDIAN"))?;
            kwargs.set_item("type", "ORIGIN_GEOMETRY")?;
            kwargs.set_item("center", c)?;
        }
        "CURSOR" => {
            kwargs.set_item("type", "ORIGIN_CURSOR")?;
        }
        other => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unknown origin mode `{other}`: expected GEOMETRY or CURSOR"
            )));
        }
    }

    with_restored_context(py, bpy, || {
        ObjCtx::select_only(py, bpy, &[name], name)?;
        let result = origin_set.call((), Some(&kwargs))?;
        require_finished(&result, "origin_set")
    })?;

    Ok(json!({ "name": name, "location": py_vec(&obj.getattr("location")?)? }))
}
