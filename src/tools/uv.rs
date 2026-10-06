//! UV maps.
//!
//! `mesh_unwrap` existed, but a mesh could only ever have the one UV layer Blender made
//! for it, and there was no way to add a second — so lightmaps, and any bake that needs
//! its own layout, were impossible. Seams are marked through `mesh_edit` (`mesh_mark`);
//! without them an angle-based unwrap of anything non-trivial is garbage, which is why
//! that pair matters together.
//!
//! Division of labour: `uv_add`/`uv_remove`/`uv_set_active` manage which UV *layers*
//! exist and which one is current; `mesh_unwrap` computes the coordinates *inside* the
//! current layer. A second layer therefore takes two calls — `uv_add` then `mesh_unwrap`.
//!
//! Verified live on Blender 5.2.2 (`--background --factory-startup`): a mesh holds at
//! most 8 UV layers, and `uv_layers.new()` returns `None` at the cap instead of raising.

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::data;

/// The mesh of an object, or a clear error. Every tool here needs exactly this.
fn mesh_of<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
    let bpy = py.import("bpy")?;
    let obj = data(&bpy)?
        .getattr("objects")?
        .call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {name}")));
    }
    let ty: String = obj.getattr("type")?.extract()?;
    if ty != "MESH" {
        return Err(PyValueError::new_err(format!(
            "{name} is a {ty}, not a mesh — it has no UV maps"
        )));
    }
    obj.getattr("data")
}

fn layers_info(mesh: &Bound<'_, PyAny>) -> PyResult<Value> {
    let layers = mesh.getattr("uv_layers")?;
    let active = layers.getattr("active")?;
    let active_name: Option<String> = if active.is_none() {
        None
    } else {
        Some(active.getattr("name")?.extract()?)
    };

    let mut out = Vec::new();
    for l in layers.try_iter()? {
        let l = l?;
        out.push(json!({
            "name": l.getattr("name")?.extract::<String>()?,
            // The render layer is the one Blender actually samples at render time, and
            // it is NOT necessarily the one selected in the UI. Getting these two out of
            // step is a classic "why does my bake look wrong".
            "active_render": l.getattr("active_render")?.extract::<bool>().unwrap_or(false),
        }));
    }
    Ok(json!({ "active": active_name, "layers": out }))
}

/// The named layer, or an error that lists the ones the mesh does have.
fn layer_or_err<'py>(
    mesh: &Bound<'py, PyAny>,
    object: &str,
    name: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let layers = mesh.getattr("uv_layers")?;
    let layer = layers.call_method1("get", (name,))?;
    if layer.is_none() {
        let have = layers_info(mesh)?;
        return Err(PyValueError::new_err(format!(
            "{object} has no UV map `{name}`. It has: {}",
            have["layers"]
        )));
    }
    Ok(layer)
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UvList {
    /// Mesh object name. A non-MESH object is an error — only meshes have UV maps.
    pub object: String,
}

crate::tool!(
    "uv_list",
    "The UV maps of a mesh: their names, which one is active (what mesh_unwrap and baking act on), and which one renders. Blender keeps `active` and `active_render` as two separate flags, and a bake reading the wrong one is the classic \"why does my bake look wrong\".",
    UvList,
    |py: Python<'_>, p: UvList| {
        let mesh = mesh_of(py, &p.object)?;
        layers_info(&mesh)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UvAdd {
    /// Mesh object name.
    pub object: String,
    /// Name for the new UV map, e.g. "Lightmap". Omit to let Blender name it
    /// ("UVMap", then "UVMap.001", …); a collision is resolved the same way.
    pub name: Option<String>,
    /// Make it the active layer — what mesh_unwrap and baking act on. Default
    /// true. Does not touch the render flag; set that with uv_set_active.
    pub active: Option<bool>,
}

crate::tool!(
    "uv_add",
    "Add a UV map to a mesh — a second UV set is what a lightmap, or a bake with its own layout, needs. This only creates the empty layer; mesh_unwrap fills the active layer with actual coordinates. A mesh holds at most 8 layers (Blender 5.2.2); asking for a ninth is an error.",
    UvAdd,
    |py: Python<'_>, p: UvAdd| {
        let mesh = mesh_of(py, &p.object)?;
        let layers = mesh.getattr("uv_layers")?;

        let kw = PyDict::new(py);
        if let Some(n) = &p.name {
            kw.set_item("name", n.as_str())?;
        }
        let layer = layers
            .call_method("new", (), Some(&kw))
            .map_err(|e| PyRuntimeError::new_err(format!("could not add a UV map: {e}")))?;
        // Blender caps a mesh at 8 UV layers and `new()` returns None at the limit
        // rather than raising — which would otherwise surface much later as a confusing
        // crash on a None.
        if layer.is_none() {
            return Err(PyRuntimeError::new_err(
                "could not add a UV map — a mesh can hold at most 8",
            ));
        }

        if p.active.unwrap_or(true) {
            layers.setattr("active", &layer)?;
        }
        layers_info(&mesh)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UvRemove {
    /// Mesh object name.
    pub object: String,
    /// UV map to remove, by name. A miss errors and lists the maps the mesh does
    /// have (see uv_list).
    pub name: String,
}

crate::tool!(
    "uv_remove",
    "Remove a UV map from a mesh, freeing one of its 8 slots. The coordinates stored in it are lost; a material's UV Map node still naming it falls back to the active layer.",
    UvRemove,
    |py: Python<'_>, p: UvRemove| {
        let mesh = mesh_of(py, &p.object)?;
        let layer = layer_or_err(&mesh, &p.object, &p.name)?;
        mesh.getattr("uv_layers")?
            .call_method1("remove", (&layer,))?;
        layers_info(&mesh)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct UvSetActive {
    /// Mesh object name.
    pub object: String,
    /// UV map to make active, by name. A miss errors and lists the maps the mesh
    /// does have.
    pub name: String,
    /// Also make it the layer used at render time. Default false — active and
    /// active_render are two separate flags in Blender, and a bake reads the
    /// render one. Only ever sets the flag; there is no way to clear it here,
    /// since exactly one layer must carry it.
    pub render: Option<bool>,
}

crate::tool!(
    "uv_set_active",
    "Choose which UV map is active — what mesh_unwrap writes into and what baking acts on — and optionally which one renders. Call this before mesh_unwrap when a mesh has more than one layer from uv_add.",
    UvSetActive,
    |py: Python<'_>, p: UvSetActive| {
        let mesh = mesh_of(py, &p.object)?;
        let layer = layer_or_err(&mesh, &p.object, &p.name)?;
        mesh.getattr("uv_layers")?.setattr("active", &layer)?;
        if p.render.unwrap_or(false) {
            layer.setattr("active_render", true)?;
        }
        layers_info(&mesh)
    }
);
