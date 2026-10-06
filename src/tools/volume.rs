//! Volume and hair-curve objects.
//!
//! Object-level tools, not node tools: they create the datablocks and the modifiers.
//! The geometry-node trees those modifiers carry are edited through the unified node
//! layer (`object:<obj>/<modifier>` as the tree address).
//!
//! Extracted from the old `geo.rs` when the `geo_*` node tools were deleted — those
//! hand-maintained a 75-entry table of geometry node types, out of the 237 Blender
//! actually has. `node_types` now reads all of them from RNA.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyIterator, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, py_len, py_vec};

/// Fills the leading `1..=3` components from `v` into `default`, matching the
/// tolerant handling used across the ported tools for a location/color shorter
/// than three floats.
fn vec3_or_default(v: Option<&[f64]>, default: [f64; 3]) -> [f64; 3] {
    let Some(a) = v else {
        return default;
    };
    let mut out = default;
    for (i, x) in a.iter().take(3).enumerate() {
        out[i] = *x;
    }
    out
}

/// Apply a `properties` map onto an RNA object (e.g. a modifier), skipping keys
/// it doesn't actually have. Local twin of misc.rs's `apply_properties` â€” not
/// shared because tool modules are self-contained (see module-level docs).
fn apply_properties(
    py: Python<'_>,
    obj: &Bound<'_, PyAny>,
    properties: Option<&Value>,
) -> PyResult<()> {
    let Some(Value::Object(pmap)) = properties else {
        return Ok(());
    };
    for (k, v) in pmap.iter() {
        if obj.hasattr(k.as_str())? {
            match v {
                Value::Bool(b) => obj.setattr(k.as_str(), *b)?,
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        obj.setattr(k.as_str(), i as i32)?;
                    } else if let Some(f) = n.as_f64() {
                        obj.setattr(k.as_str(), f)?;
                    }
                }
                Value::String(s) => obj.setattr(k.as_str(), s.as_str())?,
                Value::Array(a) => {
                    let nums: Vec<f64> = a.iter().filter_map(Value::as_f64).collect();
                    if nums.len() == a.len() && !nums.is_empty() {
                        obj.setattr(k.as_str(), PyTuple::new(py, nums)?)?;
                    }
                }
                Value::Null | Value::Object(_) => {}
            }
        }
    }
    Ok(())
}

// ============================================================================
// Volume
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VolCreate {
    /// Name for the new volume object and its datablock. Default "Volume";
    /// Blender appends .001 etc. on a collision.
    pub name: Option<String>,
    /// World-space location as [x, y, z] in metres. Shorter arrays fill the
    /// leading components, longer ones are truncated. Default [0, 0, 0].
    pub location: Option<Vec<f64>>,
}

crate::tool!(
    "vol_create",
    "Create an empty VOLUME object, linked into the active collection. It holds \
     no OpenVDB grids until one is loaded — use vol_import_vdb to create a volume \
     from a .vdb file instead. Shade it with vol_set_shader.",
    VolCreate,
    |py: Python<'_>, p: VolCreate| {
        let name = p.name.unwrap_or_else(|| "Volume".to_string());
        let loc = vec3_or_default(p.location.as_deref(), [0.0, 0.0, 0.0]);

        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let vol_data = d
            .getattr("volumes")?
            .call_method1("new", (name.as_str(),))?;
        let vol_obj = d
            .getattr("objects")?
            .call_method1("new", (name.as_str(), &vol_data))?;
        vol_obj.setattr("location", PyTuple::new(py, loc)?)?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&vol_obj,))?;

        Ok(json!({
            "name": vol_obj.getattr("name")?.extract::<String>()?,
            "type": "VOLUME",
            "location": py_vec(&vol_obj.getattr("location")?)?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VolImportVdb {
    /// Absolute path to an existing .vdb (OpenVDB) file. The call errors if the
    /// file does not exist.
    pub path: String,
    /// Name for the new volume object and datablock. Default: the file's stem
    /// (filename without extension).
    pub name: Option<String>,
}

crate::tool!(
    "vol_import_vdb",
    "Create a VOLUME object from an OpenVDB (.vdb) file on disk and load its \
     grids, returning each grid's name and data_type (BOOLEAN, FLOAT, DOUBLE, \
     INT, INT64, MASK, VECTOR_FLOAT, VECTOR_DOUBLE, VECTOR_INT, POINTS, \
     UNKNOWN). For an empty volume with no file behind it use vol_create.",
    VolImportVdb,
    |py: Python<'_>, p: VolImportVdb| {
        let path = std::path::Path::new(&p.path);
        if !path.exists() {
            return Err(PyValueError::new_err(format!("file not found: {}", p.path)));
        }

        let vol_name = p.name.unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Volume")
                .to_string()
        });

        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let vol_data = d
            .getattr("volumes")?
            .call_method1("new", (vol_name.as_str(),))?;
        vol_data.setattr("filepath", p.path.as_str())?;
        let vol_obj = d
            .getattr("objects")?
            .call_method1("new", (vol_name.as_str(), &vol_data))?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&vol_obj,))?;

        vol_data.getattr("grids")?.call_method0("load")?;

        let mut grids = Vec::new();
        for g in PyIterator::from_object(&vol_data.getattr("grids")?)? {
            let g = g?;
            grids.push(json!({
                "name": g.getattr("name")?.extract::<String>()?,
                "data_type": g.getattr("data_type")?.extract::<String>()?,
            }));
        }

        Ok(json!({
            "name": vol_obj.getattr("name")?.extract::<String>()?,
            "path": p.path,
            "grids": grids,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct VolSetShader {
    /// Name of an existing object of type VOLUME.
    pub object: String,
    /// Principled Volume Density, >= 0. Higher is thicker/more opaque. Default
    /// 1.0.
    pub density: Option<f64>,
    /// Scattering colour as linear RGB [r, g, b], each 0..1. Alpha is forced to
    /// 1. Default [0.8, 0.8, 0.8].
    pub color: Option<Vec<f64>>,
    /// Emission Strength, >= 0. 0 = non-emissive. Default 0.0.
    pub emission_strength: Option<f64>,
    /// Emission colour as linear RGB [r, g, b], each 0..1. Default [1, 1, 1].
    pub emission_color: Option<Vec<f64>>,
    /// Absorption colour as linear RGB [r, g, b], each 0..1. Omit to leave the
    /// node's own default untouched (unlike the other fields, this one is only
    /// written when given).
    pub absorption_color: Option<Vec<f64>>,
    /// Scattering Anisotropy, -1..1: negative = backward scattering, 0 =
    /// isotropic, positive = forward. Default 0.0.
    pub anisotropy: Option<f64>,
}

crate::tool!(
    "vol_set_shader",
    "Build (or rebuild) a `<object>_Volume` material wired as Principled Volume \
     -> Material Output and assign it to a VOLUME object's first material slot. \
     Destructive: it clears the material's existing node tree. Use it on volumes \
     made by vol_create or vol_import_vdb.",
    VolSetShader,
    |py: Python<'_>, p: VolSetShader| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        let is_volume = !obj.is_none() && obj.getattr("type")?.extract::<String>()? == "VOLUME";
        if !is_volume {
            return Err(PyValueError::new_err(format!(
                "volume object not found: {}",
                p.object
            )));
        }

        let density = p.density.unwrap_or(1.0);
        let color = vec3_or_default(p.color.as_deref(), [0.8, 0.8, 0.8]);
        let emission_strength = p.emission_strength.unwrap_or(0.0);
        let emission_color = vec3_or_default(p.emission_color.as_deref(), [1.0, 1.0, 1.0]);
        let anisotropy = p.anisotropy.unwrap_or(0.0);
        let absorption_color = p
            .absorption_color
            .as_deref()
            .map(|a| vec3_or_default(Some(a), [0.0, 0.0, 0.0]));

        let mat_name = format!("{}_Volume", p.object);
        let materials = d.getattr("materials")?;
        let mut mat = materials.call_method1("get", (mat_name.as_str(),))?;
        if mat.is_none() {
            mat = materials.call_method1("new", (mat_name.as_str(),))?;
        }
        mat.setattr("use_nodes", true)?;
        let node_tree = mat.getattr("node_tree")?;
        let nodes = node_tree.getattr("nodes")?;
        let links = node_tree.getattr("links")?;
        nodes.call_method0("clear")?;

        let vol_node = nodes.call_method1("new", ("ShaderNodeVolumePrincipled",))?;
        vol_node.setattr("location", PyTuple::new(py, [0.0, 0.0])?)?;
        let vin = vol_node.getattr("inputs")?;
        vin.get_item("Density")?.setattr("default_value", density)?;
        vin.get_item("Color")?.setattr(
            "default_value",
            PyTuple::new(py, [color[0], color[1], color[2], 1.0])?,
        )?;
        vin.get_item("Emission Strength")?
            .setattr("default_value", emission_strength)?;
        vin.get_item("Emission Color")?.setattr(
            "default_value",
            PyTuple::new(
                py,
                [emission_color[0], emission_color[1], emission_color[2], 1.0],
            )?,
        )?;
        vin.get_item("Anisotropy")?
            .setattr("default_value", anisotropy)?;
        if let Some(ac) = absorption_color {
            vin.get_item("Absorption Color")?.setattr(
                "default_value",
                PyTuple::new(py, [ac[0], ac[1], ac[2], 1.0])?,
            )?;
        }

        let output = nodes.call_method1("new", ("ShaderNodeOutputMaterial",))?;
        output.setattr("location", PyTuple::new(py, [300.0, 0.0])?)?;
        links.call_method1(
            "new",
            (
                vol_node.getattr("outputs")?.get_item("Volume")?,
                output.getattr("inputs")?.get_item("Volume")?,
            ),
        )?;

        let obj_materials = obj.getattr("data")?.getattr("materials")?;
        if py_len(py, &obj_materials)? == 0 {
            obj_materials.call_method1("append", (&mat,))?;
        } else {
            obj_materials.set_item(0, &mat)?;
        }

        Ok(json!({
            "object": p.object,
            "material": mat_name,
            "density": density,
        }))
    }
);

// ============================================================================
// Hair Curves (Blender 3.3+)
// ============================================================================

/// The Python original advertised a `count` ("initial hair count") that it never
/// used: the hair object is created with an empty "Generate" geometry-nodes
/// modifier, and nothing feeds a density into it. Rather than let the model set a
/// hair count that silently does nothing, the parameter is not advertised. Wiring a
/// real density input into the Generate node group would be a feature, not a port.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct HairCreate {
    /// Name of an existing MESH object to use as the hair surface. Anything
    /// else is an error.
    pub surface: String,
    /// Name for the new hair-curves object and datablock. Default "HairCurves".
    pub name: Option<String>,
}

crate::tool!(
    "hair_create",
    "Create an empty CURVES (hair) object bound to a mesh surface, with an empty \
     geometry-nodes modifier named \"Generate\" ready to be filled in (Blender \
     3.3+). It grows no hair on its own — build the node tree through the node \
     tools, then set the surface UV map with hair_set_params or add further \
     modifiers with hair_add_modifier.",
    HairCreate,
    |py: Python<'_>, p: HairCreate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let surface = d
            .getattr("objects")?
            .call_method1("get", (p.surface.as_str(),))?;
        let is_mesh = !surface.is_none() && surface.getattr("type")?.extract::<String>()? == "MESH";
        if !is_mesh {
            return Err(PyValueError::new_err(format!(
                "mesh object not found: {}",
                p.surface
            )));
        }

        let name = p.name.unwrap_or_else(|| "HairCurves".to_string());
        let curves_data = d
            .getattr("hair_curves")?
            .call_method1("new", (name.as_str(),))?;
        let curves_obj = d
            .getattr("objects")?
            .call_method1("new", (name.as_str(), &curves_data))?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&curves_obj,))?;

        curves_data.setattr("surface", &surface)?;

        let kw = PyDict::new(py);
        kw.set_item("name", "Generate")?;
        kw.set_item("type", "NODES")?;
        curves_obj
            .getattr("modifiers")?
            .call_method("new", (), Some(&kw))?;

        Ok(json!({
            "name": curves_obj.getattr("name")?.extract::<String>()?,
            "type": "CURVES",
            "surface": p.surface,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct HairSetParams {
    /// Name of an existing object of type CURVES (hair curves).
    pub object: String,
    /// Name of a UV map on the surface mesh that hair roots are attached
    /// through. Must exist on the surface mesh. Omit to leave it unchanged.
    pub surface_uv_map: Option<String>,
}

crate::tool!(
    "hair_set_params",
    "Set the surface UV map a hair-curves object attaches its roots through, and \
     read back its current point and curve counts. Called with only `object` it \
     is a pure read. Create the object with hair_create.",
    HairSetParams,
    |py: Python<'_>, p: HairSetParams| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        let is_curves = !obj.is_none() && obj.getattr("type")?.extract::<String>()? == "CURVES";
        if !is_curves {
            return Err(PyValueError::new_err(format!(
                "hair curves object not found: {}",
                p.object
            )));
        }

        let curves = obj.getattr("data")?;
        if let Some(uv) = p.surface_uv_map.as_deref() {
            curves.setattr("surface_uv_map", uv)?;
        }

        let point_count = if curves.hasattr("points")? {
            py_len(py, &curves.getattr("points")?)?
        } else {
            0
        };
        let curve_count = if curves.hasattr("curves")? {
            py_len(py, &curves.getattr("curves")?)?
        } else {
            0
        };

        Ok(json!({
            "object": p.object,
            "point_count": point_count,
            "curve_count": curve_count,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct HairAddModifier {
    /// Name of an existing object of type CURVES (hair curves).
    pub object: String,
    /// Modifier type id. Verified live on Blender 5.2: `modifiers.new` on a
    /// CURVES object accepts the whole `Modifier.type` enum, but only a few are
    /// useful on hair — NODES (geometry nodes, how hair is actually built),
    /// ARMATURE, LATTICE, HOOK, SHRINKWRAP, SMOOTH, SURFACE_DEFORM. The new
    /// modifier is also named after this id.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Optional JSON object of RNA property name -> value applied to the new
    /// modifier, e.g. {"show_viewport": false}. Keys the modifier does not have
    /// are skipped silently; null and nested-object values are ignored.
    pub properties: Option<Value>,
}

crate::tool!(
    "hair_add_modifier",
    "Add a modifier to a hair-curves object and optionally set properties on it \
     in the same call. The modifier is named after its type. Use it after \
     hair_create; the geometry-node tree inside a NODES modifier is edited \
     through the node tools at address `object:<obj>/<modifier>`.",
    HairAddModifier,
    |py: Python<'_>, p: HairAddModifier| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        let is_curves = !obj.is_none() && obj.getattr("type")?.extract::<String>()? == "CURVES";
        if !is_curves {
            return Err(PyValueError::new_err(format!(
                "hair curves object not found: {}",
                p.object
            )));
        }

        let kw = PyDict::new(py);
        kw.set_item("name", p.r#type.as_str())?;
        kw.set_item("type", p.r#type.as_str())?;
        let m = obj
            .getattr("modifiers")?
            .call_method("new", (), Some(&kw))?;
        apply_properties(py, &m, p.properties.as_ref())?;

        Ok(json!({
            "object": p.object,
            "modifier": m.getattr("name")?.extract::<String>()?,
            "type": m.getattr("type")?.extract::<String>()?,
        }))
    }
);
