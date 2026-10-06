//! Unified node-graph layer: one family of tools for **every** node tree Blender has.
//!
//! ## Why this replaced shader_* / comp_* / geo_*
//! Those were three near-identical families backed by three hand-written tables of
//! node types (205 lines). Blender 5 has **506 node types** (99 shader, 237 geometry,
//! 86 compositor, 36 texture, 48 function) — the tables covered a minority and rotted
//! with every Blender release. `shader_list_node_types` still advertised `musgrave`,
//! removed in Blender 4.1, and the mapping silently redirected it to Noise.
//!
//! Everything the tables tried to hold is in Blender's RNA: the node types, each
//! node's sockets (with `enabled` telling you which are live for the current
//! `data_type`), and its properties with their enum options. So we ask Blender
//! instead of remembering. The lie about `musgrave` cannot come back: the type list
//! *is* the RNA.
//!
//! ## Tree addressing
//! Node trees live in five different places. One string addresses all of them:
//!
//! | Address | Tree |
//! |---|---|
//! | `material:Rust` | that material's shader tree |
//! | `world` | the world's shader tree |
//! | `compositor` | the scene's compositing tree |
//! | `group:MyGroup` | a node group in `bpy.data.node_groups` |
//! | `object:Cube/GeoNodes` | the tree of a geometry-nodes modifier |
//!
//! **Blender 5 moved the compositor.** `scene.node_tree` is gone; the tree now lives
//! in `scene.compositing_node_group`, a real node group. The old `comp_*` tools read
//! `scene.node_tree` and were therefore broken on Blender 5 — nobody noticed because
//! they were never exercised.

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use serde_json::{Value, json};

pub mod graph;
pub mod group;
pub mod introspect;
pub mod preview;

/// Which RNA base class a tree's nodes derive from — this is what makes
/// `node_types` work without a table.
pub fn tree_kind(tree: &Bound<'_, PyAny>) -> PyResult<String> {
    tree.getattr("bl_idname")?.extract()
}

/// Resolve an address like `material:Rust` to the live node tree.
///
/// A missing target is an error naming what was looked for — never a silently
/// created tree, which would leave the caller editing something the user cannot see.
pub fn resolve_tree<'py>(py: Python<'py>, addr: &str) -> PyResult<Bound<'py, PyAny>> {
    let bpy = py.import("bpy")?;
    let data = bpy.getattr("data")?;

    let (kind, rest) = match addr.split_once(':') {
        Some((k, r)) => (k, r),
        None => (addr, ""),
    };

    let tree = match kind {
        "material" => {
            let mat = data.getattr("materials")?.call_method1("get", (rest,))?;
            if mat.is_none() {
                return Err(PyValueError::new_err(format!("material not found: {rest}")));
            }
            // A material without nodes has no tree at all; turning them on here is
            // what every caller wants and is not destructive.
            mat.setattr("use_nodes", true)?;
            mat.getattr("node_tree")?
        }
        "world" => {
            let world = bpy.getattr("context")?.getattr("scene")?.getattr("world")?;
            if world.is_none() {
                return Err(PyValueError::new_err("the scene has no world"));
            }
            world.setattr("use_nodes", true)?;
            world.getattr("node_tree")?
        }
        "compositor" => {
            // Blender 5: the compositor tree is a node group hanging off the scene,
            // NOT scene.node_tree (which no longer exists).
            let scene = bpy.getattr("context")?.getattr("scene")?;
            scene.setattr("use_nodes", true)?;
            let existing = scene.getattr("compositing_node_group")?;
            if existing.is_none() {
                let created = data
                    .getattr("node_groups")?
                    .call_method1("new", ("Compositing", "CompositorNodeTree"))?;
                scene.setattr("compositing_node_group", &created)?;
                created
            } else {
                existing
            }
        }
        "group" => {
            let grp = data.getattr("node_groups")?.call_method1("get", (rest,))?;
            if grp.is_none() {
                return Err(PyValueError::new_err(format!(
                    "node group not found: {rest}"
                )));
            }
            grp
        }
        "object" => {
            let (obj_name, mod_name) = rest.split_once('/').ok_or_else(|| {
                PyValueError::new_err(
                    "object address must be `object:<object>/<modifier>`, e.g. object:Cube/GeoNodes",
                )
            })?;
            let obj = data.getattr("objects")?.call_method1("get", (obj_name,))?;
            if obj.is_none() {
                return Err(PyValueError::new_err(format!(
                    "object not found: {obj_name}"
                )));
            }
            let m = obj.getattr("modifiers")?.call_method1("get", (mod_name,))?;
            if m.is_none() {
                return Err(PyValueError::new_err(format!(
                    "modifier not found on {obj_name}: {mod_name}"
                )));
            }
            let group = m.getattr("node_group")?;
            if group.is_none() {
                return Err(PyValueError::new_err(format!(
                    "modifier {mod_name} has no node group — create one with node_tree_create"
                )));
            }
            group
        }
        other => {
            return Err(PyValueError::new_err(format!(
                "unknown tree address `{addr}`: expected material:<name>, world, compositor, \
                 group:<name> or object:<object>/<modifier>, got kind `{other}`"
            )));
        }
    };

    Ok(tree)
}

/// A node by name, with the tree's actual node names in the error — the caller is
/// usually a model working from a stale memory of what it created.
pub fn node_by_name<'py>(tree: &Bound<'py, PyAny>, name: &str) -> PyResult<Bound<'py, PyAny>> {
    let nodes = tree.getattr("nodes")?;
    let node = nodes.call_method1("get", (name,))?;
    if node.is_none() {
        let mut have = Vec::new();
        for n in nodes.try_iter()? {
            have.push(n?.getattr("name")?.extract::<String>()?);
        }
        return Err(PyValueError::new_err(format!(
            "node not found: {name}. Nodes in this tree: {}",
            have.join(", ")
        )));
    }
    Ok(node)
}

/// Sockets that are actually live for the node's current configuration.
///
/// A node carries a full set of sockets for *every* `data_type` it supports and hides
/// the ones that do not apply: `ShaderNodeMix` has four sockets called `A`, one per
/// data type. Only the enabled ones exist as far as the user is concerned, so those
/// are the only ones we offer, and the only ones we name in an error.
pub fn enabled_sockets<'py>(
    node: &Bound<'py, PyAny>,
    input: bool,
) -> PyResult<Vec<Bound<'py, PyAny>>> {
    let collection = node.getattr(if input { "inputs" } else { "outputs" })?;
    let mut out = Vec::new();
    for s in collection.try_iter()? {
        let s = s?;
        if s.getattr("enabled")?.extract::<bool>()? {
            out.push(s);
        }
    }
    Ok(out)
}

pub fn socket_names(node: &Bound<'_, PyAny>, input: bool) -> PyResult<Vec<String>> {
    enabled_sockets(node, input)?
        .iter()
        .map(|s| s.getattr("name")?.extract::<String>())
        .collect()
}

/// Find a socket by name (or by index, when the caller gives a number).
///
/// **This is the guard that matters most.** A model works from memory, and Blender
/// renamed half of the Principled BSDF's inputs in 4.x: `Specular` became
/// `Specular IOR Level`, `Emission` split into `Emission Color` and `Emission
/// Strength`, `Transmission` became `Transmission Weight`. Guessing fails, so the
/// error hands back the real list rather than just saying "not found".
pub fn find_socket<'py>(
    node: &Bound<'py, PyAny>,
    spec: &Value,
    input: bool,
) -> PyResult<Bound<'py, PyAny>> {
    let sockets = enabled_sockets(node, input)?;
    let side = if input { "input" } else { "output" };

    if let Some(idx) = spec.as_u64() {
        return sockets.into_iter().nth(idx as usize).ok_or_else(|| {
            PyValueError::new_err(format!("{side} socket index {idx} is out of range"))
        });
    }

    let want = spec.as_str().ok_or_else(|| {
        PyValueError::new_err(format!(
            "{side} socket must be a name or an index, got {spec}"
        ))
    })?;

    for s in &sockets {
        if s.getattr("name")?.extract::<String>()? == want {
            return Ok(s.clone());
        }
    }

    let node_name: String = node.getattr("name")?.extract()?;
    let node_type: String = node.getattr("bl_idname")?.extract()?;
    let have = socket_names(node, input)?;
    Err(PyValueError::new_err(format!(
        "{node_name} ({node_type}) has no {side} socket `{want}`. Its {side}s are: {}",
        have.join(", ")
    )))
}

/// Which `bpy.data` collection holds a given ID type — derived from RNA, not a table.
///
/// `bpy.data` declares one collection per ID type, and each collection's RNA knows the
/// type it holds (`fixed_type`). So `Image` -> `bpy.data.images` and `Object` ->
/// `bpy.data.objects` fall out of Blender's own metadata, for all 39 ID types.
fn data_collection<'py>(py: Python<'py>, id_type: &str) -> PyResult<Bound<'py, PyAny>> {
    let data = py.import("bpy")?.getattr("data")?;
    let props = data.getattr("bl_rna")?.getattr("properties")?;

    for prop in props.try_iter()? {
        let prop = prop?;
        if prop.getattr("type")?.extract::<String>()? != "COLLECTION" {
            continue;
        }
        let Ok(fixed) = prop.getattr("fixed_type") else {
            continue;
        };
        if fixed.is_none() {
            continue;
        }
        if fixed.getattr("identifier")?.extract::<String>()? == id_type {
            let ident: String = prop.getattr("identifier")?.extract()?;
            return data.getattr(ident.as_str());
        }
    }
    Err(PyRuntimeError::new_err(format!(
        "no bpy.data collection holds `{id_type}`"
    )))
}

/// Turn a datablock *name* into the datablock itself.
///
/// An Image Texture's `image`, an Object Info's `Object` socket, a Collection Info's
/// `Collection` — these are pointers, not values, and `setattr(node, "image", "brick")`
/// raises. A model can only sensibly refer to them by name, so names are what we take,
/// and this is where a name becomes the thing.
///
/// Not found is an error listing what *is* in the file: a model has no other way to
/// learn what the user has loaded.
fn resolve_datablock<'py>(
    py: Python<'py>,
    id_type: &str,
    name: &str,
    what: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let collection = data_collection(py, id_type)?;
    let found = collection.call_method1("get", (name,))?;
    if !found.is_none() {
        return Ok(found);
    }

    let mut have = Vec::new();
    for item in collection.try_iter()? {
        have.push(item?.getattr("name")?.extract::<String>()?);
    }
    let listing = if have.is_empty() {
        "there are none in this file".to_string()
    } else {
        format!("this file has: {}", have.join(", "))
    };
    Err(PyValueError::new_err(format!(
        "{what} expects the name of an existing {id_type}; `{name}` was not found — {listing}"
    )))
}

/// The ID type a pointer-valued RNA property or socket points at, or `None` when it is
/// not a pointer at all.
fn pointer_target(prop: &Bound<'_, PyAny>) -> PyResult<Option<String>> {
    if prop.getattr("type")?.extract::<String>()? != "POINTER" {
        return Ok(None);
    }
    let fixed = prop.getattr("fixed_type")?;
    if fixed.is_none() {
        return Ok(None);
    }
    Ok(Some(fixed.getattr("identifier")?.extract()?))
}

/// Write a JSON value into a socket's `default_value`, matching the socket's arity.
pub fn set_socket_value(socket: &Bound<'_, PyAny>, value: &Value) -> PyResult<()> {
    let py = socket.py();
    if !socket.hasattr("default_value")? {
        let name: String = socket.getattr("name")?.extract()?;
        return Err(PyValueError::new_err(format!(
            "socket `{name}` holds no value (it can only be linked)"
        )));
    }

    // An Object/Collection/Material/Image socket holds a datablock, not a number. The
    // caller names it; we look it up.
    let dv_prop = socket
        .getattr("bl_rna")?
        .getattr("properties")?
        .call_method1("get", ("default_value",))?;
    if !dv_prop.is_none()
        && let Some(id_type) = pointer_target(&dv_prop)?
    {
        let socket_name: String = socket.getattr("name")?.extract()?;
        if value.is_null() {
            socket.setattr("default_value", py.None())?;
            return Ok(());
        }
        let name = value.as_str().ok_or_else(|| {
            PyValueError::new_err(format!(
                "socket `{socket_name}` takes the name of a {id_type}, got {value}"
            ))
        })?;
        let target = resolve_datablock(py, &id_type, name, &format!("socket `{socket_name}`"))?;
        socket.setattr("default_value", target)?;
        return Ok(());
    }

    match value {
        Value::Bool(b) => socket.setattr("default_value", *b)?,
        Value::Number(n) => {
            let f = n
                .as_f64()
                .ok_or_else(|| PyValueError::new_err("socket value is not a number"))?;
            socket.setattr("default_value", f)?;
        }
        Value::String(s) => socket.setattr("default_value", s.as_str())?,
        Value::Array(a) => {
            let nums: Vec<f64> = a.iter().filter_map(Value::as_f64).collect();
            if nums.len() != a.len() {
                return Err(PyValueError::new_err(
                    "a vector/colour socket takes numbers only",
                ));
            }
            let current = socket.getattr("default_value")?;
            // A colour socket is RGBA: accept RGB and keep alpha opaque, which is what
            // a caller passing three numbers means.
            let want = crate::tools::bpy::py_len(py, &current).unwrap_or(nums.len());
            let mut nums = nums;
            if want == 4 && nums.len() == 3 {
                nums.push(1.0);
            }
            if nums.len() != want {
                let name: String = socket.getattr("name")?.extract()?;
                return Err(PyValueError::new_err(format!(
                    "socket `{name}` takes {want} numbers, got {}",
                    nums.len()
                )));
            }
            socket.setattr("default_value", pyo3::types::PyTuple::new(py, nums)?)?;
        }
        Value::Null | Value::Object(_) => {
            let name: String = socket.getattr("name")?.extract()?;
            return Err(PyValueError::new_err(format!(
                "socket `{name}` cannot take {value}"
            )));
        }
    }
    Ok(())
}

/// Set a node property (`data_type`, `blend_type`, `operation`, …).
///
/// An invalid enum value is answered with the values Blender actually accepts: this
/// is the second place a model's memory goes stale, right after socket names.
pub fn set_node_prop(node: &Bound<'_, PyAny>, key: &str, value: &Value) -> PyResult<()> {
    if !node.hasattr(key)? {
        let ty: String = node.getattr("bl_idname")?.extract()?;
        return Err(PyValueError::new_err(format!(
            "{ty} has no property `{key}` (use node_schema to see its properties)"
        )));
    }

    // The single most-used shader node after Principled is Image Texture, and its
    // `image` is a POINTER, not a string: `node.image = "brick.png"` raises. The caller
    // names the datablock; we look it up. Same for a group node's `node_tree`, a
    // texture's `texture`, and so on.
    let prop = node
        .getattr("bl_rna")?
        .getattr("properties")?
        .call_method1("get", (key,))?;
    if !prop.is_none()
        && let Some(id_type) = pointer_target(&prop)?
    {
        if value.is_null() {
            node.setattr(key, node.py().None())?;
            return Ok(());
        }
        let name = value.as_str().ok_or_else(|| {
            PyValueError::new_err(format!(
                "property `{key}` takes the name of a {id_type}, got {value}"
            ))
        })?;
        let target = resolve_datablock(node.py(), &id_type, name, &format!("property `{key}`"))?;
        node.setattr(key, target)?;
        return Ok(());
    }

    let apply = || -> PyResult<()> {
        match value {
            Value::Bool(b) => node.setattr(key, *b),
            Value::String(s) => node.setattr(key, s.as_str()),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    node.setattr(key, i)
                } else {
                    node.setattr(
                        key,
                        n.as_f64()
                            .ok_or_else(|| PyValueError::new_err("not a number"))?,
                    )
                }
            }
            Value::Array(a) => {
                let nums: Vec<f64> = a.iter().filter_map(Value::as_f64).collect();
                node.setattr(key, pyo3::types::PyTuple::new(node.py(), nums)?)
            }
            _ => Err(PyValueError::new_err(format!(
                "property `{key}` cannot take {value}"
            ))),
        }
    };

    apply().map_err(|e| {
        // Blender rejects a bad enum with a TypeError listing nothing useful. Say what
        // it will accept instead.
        match enum_values(node, key) {
            Ok(Some(vals)) => PyValueError::new_err(format!(
                "property `{key}` does not accept {value}. Allowed: {}",
                vals.join(", ")
            )),
            _ => PyRuntimeError::new_err(format!("setting `{key}` failed: {e}")),
        }
    })
}

/// The values an enum property accepts, straight from RNA. `None` when it is not an enum.
pub fn enum_values(node: &Bound<'_, PyAny>, key: &str) -> PyResult<Option<Vec<String>>> {
    let props = node.getattr("bl_rna")?.getattr("properties")?;
    let prop = props.call_method1("get", (key,))?;
    if prop.is_none() {
        return Ok(None);
    }
    if prop.getattr("type")?.extract::<String>()? != "ENUM" {
        return Ok(None);
    }
    let mut out = Vec::new();
    for item in prop.getattr("enum_items")?.try_iter()? {
        out.push(item?.getattr("identifier")?.extract::<String>()?);
    }
    Ok(Some(out))
}

/// One socket, described for the model: what it is called, what it carries, what it
/// currently holds, and whether something is already plugged into it.
pub fn socket_info(socket: &Bound<'_, PyAny>) -> PyResult<Value> {
    let name: String = socket.getattr("name")?.extract()?;
    let ty: String = socket.getattr("type")?.extract()?;
    let linked: bool = socket.getattr("is_linked")?.extract()?;

    let mut info = json!({ "name": name, "type": ty });
    if linked {
        info["linked"] = json!(true);
    }
    if let Ok(dv) = socket.getattr("default_value")
        && let Some(v) = py_to_json(&dv)
    {
        info["value"] = v;
    }
    Ok(info)
}

/// Best-effort conversion of a socket's default value into JSON. `None` for the
/// things that have no JSON shape (an image pointer, an object pointer…).
pub fn py_to_json(v: &Bound<'_, PyAny>) -> Option<Value> {
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
