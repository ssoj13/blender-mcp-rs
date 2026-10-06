//! Materials via `bpy.data.materials` and Principled BSDF, shaped around typed params
//! structs so the registry can generate their JSON Schema.

use std::collections::HashSet;

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyIterator, PyList};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, py_len};

fn principled_bsdf<'a>(mat: &'a Bound<'a, PyAny>) -> PyResult<Bound<'a, PyAny>> {
    let nt = mat.getattr("node_tree")?;
    let nodes = nt.getattr("nodes")?;
    nodes.call_method1("get", ("Principled BSDF",))
}

fn vec_from_seq(seq: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    let it = PyIterator::from_object(seq)?;
    let mut v = Vec::new();
    for x in it {
        let x = x?;
        v.push(x.extract::<f64>()?);
    }
    Ok(v)
}

/// Overlay `color` (up to 4 components) onto `default`, leaving any missing
/// trailing components at their default value.
fn rgba_with_default(color: Option<&[f64]>, default: [f64; 4]) -> [f64; 4] {
    let Some(arr) = color else {
        return default;
    };
    let mut out = default;
    for (i, x) in arr.iter().take(4).enumerate() {
        out[i] = *x;
    }
    out
}

fn build_material(
    py: Python<'_>,
    d: &Bound<'_, PyAny>,
    name: &str,
    color: Option<&[f64]>,
    metallic: Option<f64>,
    roughness: Option<f64>,
) -> PyResult<Value> {
    let materials = d.getattr("materials")?;
    let mat = materials.call_method1("new", (name,))?;
    mat.setattr("use_nodes", true)?;
    let bsdf = principled_bsdf(&mat)?;
    if !bsdf.is_none() {
        let rgba = rgba_with_default(color, [0.8, 0.8, 0.8, 1.0]);
        let inp = bsdf.getattr("inputs")?;
        let bc = inp.call_method1("get", ("Base Color",))?;
        bc.setattr("default_value", PyList::new(py, rgba)?)?;
        inp.call_method1("get", ("Metallic",))?
            .setattr("default_value", metallic.unwrap_or(0.0))?;
        inp.call_method1("get", ("Roughness",))?
            .setattr("default_value", roughness.unwrap_or(0.5))?;
    }
    let n: String = mat.getattr("name")?.extract()?;
    Ok(json!({ "name": n }))
}

/// `mat_list` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatList {}

crate::tool!(
    "mat_list",
    "Every material datablock in the file, with its Principled base_color, \
     metallic and roughness where it has them. Use mat_get_object to see which \
     materials sit on one object, and mat_settings for the non-node flags.",
    MatList,
    |py: Python<'_>, _p: MatList| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let materials = d.getattr("materials")?;
        let it = PyIterator::from_object(&materials)?;
        let mut rows = Vec::new();
        for m in it {
            let m = m?;
            let name: String = m.getattr("name")?.extract()?;
            let use_nodes: bool = m.getattr("use_nodes")?.extract()?;
            let mut info = json!({
                "name": name,
                "use_nodes": use_nodes,
            });
            if use_nodes
                && let Ok(nt) = m.getattr("node_tree")
                && !nt.is_none()
            {
                let nodes = nt.getattr("nodes")?;
                let nit = PyIterator::from_object(&nodes)?;
                for n in nit {
                    let n = n?;
                    let ty: String = n.getattr("type")?.extract()?;
                    if ty == "BSDF_PRINCIPLED" {
                        let bc = n.getattr("inputs")?.call_method1("get", ("Base Color",))?;
                        let base_color = vec_from_seq(&bc.getattr("default_value")?)?;
                        let metallic = n
                            .getattr("inputs")?
                            .call_method1("get", ("Metallic",))?
                            .getattr("default_value")?
                            .extract::<f64>()?;
                        let roughness = n
                            .getattr("inputs")?
                            .call_method1("get", ("Roughness",))?
                            .getattr("default_value")?
                            .extract::<f64>()?;
                        info["base_color"] = json!(base_color);
                        info["metallic"] = json!(metallic);
                        info["roughness"] = json!(roughness);
                        break;
                    }
                }
            }
            rows.push(info);
        }
        Ok(Value::Array(rows))
    }
);

/// One entry of `mat_create`'s batch mode.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct MaterialSpec {
    /// Material name. Blender renames on collision, so the created name is echoed back.
    pub name: String,
    /// Base Color as [r, g, b] or [r, g, b, a], each 0-1. Default [0.8, 0.8, 0.8, 1.0].
    pub color: Option<Vec<f64>>,
    /// Principled Metallic, 0-1. Default 0.0.
    pub metallic: Option<f64>,
    /// Principled Roughness, 0-1. Default 0.5.
    pub roughness: Option<f64>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatCreate {
    /// Single mode: material name. Blender renames on collision, so the created
    /// name is echoed back.
    pub name: Option<String>,
    /// Single mode: Base Color as [r, g, b] or [r, g, b, a], each 0-1.
    /// Default [0.8, 0.8, 0.8, 1.0].
    pub color: Option<Vec<f64>>,
    /// Single mode: Principled Metallic, 0-1. Default 0.0.
    pub metallic: Option<f64>,
    /// Single mode: Principled Roughness, 0-1. Default 0.5.
    pub roughness: Option<f64>,
    /// Batch mode: [{name, color?, metallic?, roughness?}, ...]. Takes precedence
    /// over the single-mode fields when given.
    pub materials: Option<Vec<MaterialSpec>>,
}

crate::tool!(
    "mat_create",
    "Create a node-based Principled BSDF material (one, or a batch via `materials`). \
     The material exists but is attached to nothing — put it on an object with \
     mat_assign, or into a specific slot with mat_slot_add / mat_slot_set.",
    MatCreate,
    |py: Python<'_>, p: MatCreate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        if let Some(specs) = p.materials {
            let mut results = Vec::new();
            for spec in specs {
                results.push(build_material(
                    py,
                    &d,
                    &spec.name,
                    spec.color.as_deref(),
                    spec.metallic,
                    spec.roughness,
                )?);
            }
            return Ok(json!({
                "created": results.len(),
                "results": results,
            }));
        }
        let name = p
            .name
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("missing name"))?;
        build_material(py, &d, &name, p.color.as_deref(), p.metallic, p.roughness)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatAssign {
    /// Single object name
    pub object: Option<String>,
    /// Batch mode: list of object names. Takes precedence over `object` when given.
    pub objects: Option<Vec<String>>,
    /// Name of an existing material (mat_list / mat_create).
    pub material: String,
}

crate::tool!(
    "mat_assign",
    "Put one material on one or more objects, filling slot 0 (appending it if the \
     object had no slots). This is whole-object assignment and can never produce a \
     second material: for that use mat_slot_add, then mat_assign_faces to paint \
     faces with the new slot.",
    MatAssign,
    |py: Python<'_>, p: MatAssign| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let materials = d.getattr("materials")?;
        let mat = materials.call_method1("get", (p.material.as_str(),))?;
        if mat.is_none() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Material not found: {}",
                p.material
            )));
        }

        let obj_names: Vec<String> = if let Some(objs) = p.objects {
            objs
        } else if let Some(o) = p.object {
            vec![o]
        } else {
            Vec::new()
        };

        let objects = d.getattr("objects")?;
        let mut results = Vec::new();
        for obj_name in obj_names {
            let obj = objects.call_method1("get", (&obj_name,))?;
            if obj.is_none() {
                results.push(json!({ "object": obj_name, "error": "not found" }));
                continue;
            }
            let data_ob = obj.getattr("data")?;
            if data_ob.is_none() {
                results.push(json!({ "object": obj_name, "error": "no data" }));
                continue;
            }
            let mats = data_ob.getattr("materials")?;
            let n = py_len(py, &mats)?;
            if n == 0 {
                mats.call_method1("append", (&mat,))?;
            } else {
                mats.set_item(0, &mat)?;
            }
            let mn: String = mat.getattr("name")?.extract()?;
            results.push(json!({ "object": obj_name, "material": mn }));
        }

        if results.len() == 1 {
            let only = results.into_iter().next().unwrap_or(Value::Null);
            // A lone target that failed outright is a hard error, not data — only a
            // genuine multi-target *partial* success belongs in `results` below.
            if let Some(err) = only.get("error").and_then(Value::as_str) {
                let obj_name = only.get("object").and_then(Value::as_str).unwrap_or("?");
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "cannot assign material to '{obj_name}': {err}"
                )));
            }
            return Ok(only);
        }
        let assigned = results.iter().filter(|r| r.get("error").is_none()).count();
        Ok(json!({ "assigned": assigned, "results": results }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatDelete {
    /// Single mode: name of the material to delete.
    pub name: Option<String>,
    /// Batch mode: list of material names. Takes precedence over `name` when given.
    pub names: Option<Vec<String>>,
}

crate::tool!(
    "mat_delete",
    "Delete material datablocks by name (one, or a batch via `names`). Objects \
     using them are left with empty slots — clear or refill those with \
     mat_slot_set / mat_slot_remove.",
    MatDelete,
    |py: Python<'_>, p: MatDelete| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let materials = d.getattr("materials")?;

        let names: Vec<String> = if let Some(n) = p.names {
            n
        } else if let Some(n) = p.name {
            vec![n]
        } else {
            Vec::new()
        };

        let mut results = Vec::new();
        for name in names {
            let mat = materials.call_method1("get", (&name,))?;
            if mat.is_none() {
                results.push(json!({ "name": name, "error": "not found" }));
                continue;
            }
            materials.call_method1("remove", (&mat,))?;
            results.push(json!({ "deleted": name }));
        }

        if results.len() == 1 {
            let only = results.into_iter().next().unwrap_or(Value::Null);
            // A lone target that failed outright is a hard error, not data — only a
            // genuine multi-target *partial* success belongs in `results` below.
            if let Some(err) = only.get("error").and_then(Value::as_str) {
                let name = only.get("name").and_then(Value::as_str).unwrap_or("?");
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "cannot delete material '{name}': {err}"
                )));
            }
            return Ok(only);
        }
        let deleted = results.iter().filter(|r| r.get("error").is_none()).count();
        Ok(json!({ "deleted": deleted, "results": results }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatUpdate {
    /// Existing material name. Its node tree must still contain a node literally
    /// named `Principled BSDF`; after a node_graph_build with clear:true it does
    /// not, and node_graph_edit set_inputs is the tool to use instead.
    pub name: String,
    /// Base Color as [r, g, b] (alpha forced to 1) or [r, g, b, a], each 0-1.
    pub color: Option<Vec<f64>>,
    /// Principled Metallic, 0-1.
    pub metallic: Option<f64>,
    /// Principled Roughness, 0-1.
    pub roughness: Option<f64>,
    /// Emission Color as [r, g, b], each 0-1 (values above 1 are allowed and read
    /// as HDR). Ignored unless at least 3 components are given.
    pub emission: Option<Vec<f64>>,
    /// Emission Strength, >= 0. Applied only together with `emission`; defaults to
    /// 1.0 when `emission` is given without it.
    pub emission_strength: Option<f64>,
    /// Principled Alpha, 0-1. A value below 1 also flips the material's
    /// blend_method to BLEND (OPAQUE at 1.0), since EEVEE otherwise renders it solid.
    pub alpha: Option<f64>,
    /// Index of refraction, typically 1.0-2.5 (glass ~1.45, water ~1.33). Default 1.5.
    pub ior: Option<f64>,
    /// Transmission Weight, 0-1 (1 = fully refractive, i.e. glass). Default 0.0.
    pub transmission: Option<f64>,
}

crate::tool!(
    "mat_update",
    "Change a Principled BSDF material's basic inputs in place: color, metallic, \
     roughness, emission, alpha, ior, transmission. Works only while the node is \
     still named `Principled BSDF`; for any other node or socket use \
     node_graph_edit set_inputs, with node_schema for the real socket names.",
    MatUpdate,
    |py: Python<'_>, p: MatUpdate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let materials = d.getattr("materials")?;
        let mat = materials.call_method1("get", (p.name.as_str(),))?;
        if mat.is_none() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Material not found: {}",
                p.name
            )));
        }
        mat.setattr("use_nodes", true)?;
        let bsdf = principled_bsdf(&mat)?;
        if bsdf.is_none() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Material has no Principled BSDF node",
            ));
        }
        let inp = bsdf.getattr("inputs")?;
        if let Some(c) = p.color {
            let rgba: Vec<f64> = match c.len() {
                3 => vec![c[0], c[1], c[2], 1.0],
                n if n >= 4 => c.iter().take(4).copied().collect(),
                _ => return Err(pyo3::exceptions::PyValueError::new_err("bad color")),
            };
            inp.call_method1("get", ("Base Color",))?
                .setattr("default_value", PyList::new(py, rgba)?)?;
        }
        if let Some(m) = p.metallic {
            inp.call_method1("get", ("Metallic",))?
                .setattr("default_value", m)?;
        }
        if let Some(r) = p.roughness {
            inp.call_method1("get", ("Roughness",))?
                .setattr("default_value", r)?;
        }
        if let Some(rgb) = p.emission
            && rgb.len() >= 3
        {
            let em: Vec<f64> = vec![rgb[0], rgb[1], rgb[2], 1.0];
            inp.call_method1("get", ("Emission Color",))?
                .setattr("default_value", PyList::new(py, em)?)?;
            let strength = p.emission_strength.unwrap_or(1.0);
            inp.call_method1("get", ("Emission Strength",))?
                .setattr("default_value", strength)?;
        }
        if let Some(a) = p.alpha {
            inp.call_method1("get", ("Alpha",))?
                .setattr("default_value", a)?;
            let blend = if a < 1.0 { "BLEND" } else { "OPAQUE" };
            mat.setattr("blend_method", blend)?;
        }
        if let Some(ior) = p.ior {
            inp.call_method1("get", ("IOR",))?
                .setattr("default_value", ior)?;
        }
        if let Some(t) = p.transmission {
            inp.call_method1("get", ("Transmission Weight",))?
                .setattr("default_value", t)?;
        }
        let mn: String = mat.getattr("name")?.extract()?;
        Ok(json!({ "name": mn, "updated": true }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatGetObjectsBy {
    /// Material names to look for. At least one; unknown names are an error, not
    /// an empty result.
    pub materials: Vec<String>,
}

crate::tool!(
    "mat_get_objects_by",
    "Find every object using the given material(s), with the slot index each one \
     sits in. The reverse lookup of mat_get_object.",
    MatGetObjectsBy,
    |py: Python<'_>, p: MatGetObjectsBy| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let mat_names = p.materials;
        if mat_names.is_empty() {
            return Err(pyo3::exceptions::PyValueError::new_err("missing materials"));
        }

        let materials = d.getattr("materials")?;
        let mut search = HashSet::new();
        for name in &mat_names {
            let m = materials.call_method1("get", (name.as_str(),))?;
            if m.is_none() {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Material not found: {name}"
                )));
            }
            let real: String = m.getattr("name")?.extract()?;
            search.insert(real);
        }

        let mut result_map: serde_json::Map<String, Value> = serde_json::Map::new();
        for mn in &search {
            result_map.insert(mn.clone(), json!([]));
        }

        let objects = d.getattr("objects")?;
        let oit = PyIterator::from_object(&objects)?;
        for o in oit {
            let obj = o?;
            let data_ob = obj.getattr("data")?;
            if data_ob.is_none() {
                continue;
            }
            if !data_ob.hasattr("materials")? {
                continue;
            }
            let mats = data_ob.getattr("materials")?;
            let mit = PyIterator::from_object(&mats)?;
            for (slot_idx, mat) in mit.enumerate() {
                let mat = mat?;
                if mat.is_none() {
                    continue;
                }
                let mname: String = mat.getattr("name")?.extract()?;
                if search.contains(&mname) {
                    let oname: String = obj.getattr("name")?.extract()?;
                    let ty: String = obj.getattr("type")?.extract()?;
                    if let Some(arr) = result_map.get_mut(&mname).and_then(Value::as_array_mut) {
                        arr.push(json!({
                            "object": oname,
                            "slot_index": slot_idx,
                            "type": ty,
                        }));
                    }
                }
            }
        }

        let total: usize = result_map
            .values()
            .filter_map(|v| v.as_array())
            .map(|a| a.len())
            .sum();
        Ok(json!({
            "materials": Value::Object(result_map),
            "total_objects": total,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatGetObject {
    /// Object name. Objects with no material slots come back with
    /// has_materials:false rather than an error.
    pub name: String,
}

crate::tool!(
    "mat_get_object",
    "Every material on one object, with its slot index and Principled values. \
     mat_slots is the leaner view when only slot indices and link modes matter.",
    MatGetObject,
    |py: Python<'_>, p: MatGetObject| {
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
        let data_ob = obj.getattr("data")?;
        if data_ob.is_none() || !data_ob.hasattr("materials")? {
            let on: String = obj.getattr("name")?.extract()?;
            return Ok(json!({
                "object": on,
                "has_materials": false,
                "materials": [],
            }));
        }
        let mats = data_ob.getattr("materials")?;
        let mit = PyIterator::from_object(&mats)?;
        let mut materials = Vec::new();
        for (slot_idx, mat) in mit.enumerate() {
            let mat = mat?;
            if mat.is_none() {
                materials.push(json!({
                    "name": Value::Null,
                    "slot_index": slot_idx,
                    "empty": true,
                }));
                continue;
            }
            let mn: String = mat.getattr("name")?.extract()?;
            let use_nodes: bool = mat.getattr("use_nodes")?.extract()?;
            let mut mat_info = json!({
                "name": mn,
                "slot_index": slot_idx,
                "use_nodes": use_nodes,
            });
            if use_nodes
                && let Ok(nt) = mat.getattr("node_tree")
                && !nt.is_none()
                && let Ok(bsdf) = nt
                    .getattr("nodes")?
                    .call_method1("get", ("Principled BSDF",))
                && !bsdf.is_none()
            {
                let inp = bsdf.getattr("inputs")?;
                let bc = inp.call_method1("get", ("Base Color",))?;
                let base_color = vec_from_seq(&bc.getattr("default_value")?)?;
                let metallic = inp
                    .call_method1("get", ("Metallic",))?
                    .getattr("default_value")?
                    .extract::<f64>()?;
                let roughness = inp
                    .call_method1("get", ("Roughness",))?
                    .getattr("default_value")?
                    .extract::<f64>()?;
                mat_info["base_color"] = json!(base_color);
                mat_info["metallic"] = json!(metallic);
                mat_info["roughness"] = json!(roughness);
            }
            materials.push(mat_info);
        }
        let on: String = obj.getattr("name")?.extract()?;
        Ok(json!({
            "object": on,
            "has_materials": true,
            "materials": materials,
        }))
    }
);

// ---------------------------------------------------------------------------
// Multi-material support: an object can only ever get one material through
// `mat_assign` (it fills slot 0). Real objects need more than one — a
// multi-material cube, a face-ranged selection painted with a second
// material — which needs slot-level control below.
// ---------------------------------------------------------------------------

/// Looks up `name` and confirms its data-block supports material slots (Mesh,
/// Curve, MetaBall, GreasePencil… — not Empty/Camera/Light). Every slot tool
/// below needs this same check, centralized here rather than repeated.
fn require_object_with_slots<'py>(
    d: &Bound<'py, PyAny>,
    name: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Object not found: {name}"
        )));
    }
    let data_ob = obj.getattr("data")?;
    if data_ob.is_none() || !data_ob.hasattr("materials")? {
        let ty: String = obj.getattr("type")?.extract()?;
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "object `{name}` has no material slots (type {ty})"
        )));
    }
    Ok(obj)
}

/// Sets an RNA enum property to `value` after checking it against this
/// Blender's own `bl_rna.properties[prop].enum_items` — never a hardcoded
/// table. EEVEE Next reworked several `Material` properties across 4.2/5.0
/// (`shadow_method` no longer exists on Blender 5.2, verified live); a
/// property this Blender does not have is reported by name, not guessed at.
fn set_rna_enum(obj: &Bound<'_, PyAny>, prop: &str, value: &str) -> PyResult<()> {
    let props = obj.getattr("bl_rna")?.getattr("properties")?;
    let p = props.call_method1("get", (prop,))?;
    if p.is_none() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "this Blender's Material has no `{prop}` property (removed or renamed in this version)"
        )));
    }
    let mut items = Vec::new();
    for item in PyIterator::from_object(&p.getattr("enum_items")?)? {
        items.push(item?.getattr("identifier")?.extract::<String>()?);
    }
    let up = value.to_uppercase();
    if !items.iter().any(|i| i == &up) {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "invalid {prop} `{value}`. This Blender accepts: {}",
            items.join(", ")
        )));
    }
    obj.setattr(prop, up)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// mat_slots
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatSlots {
    /// Object name. Must be a type that has material slots (Mesh, Curve, MetaBall,
    /// Grease Pencil...) — an Empty/Camera/Light is rejected by name.
    pub object: String,
}

crate::tool!(
    "mat_slots",
    "List an object's material slots: index, material name, and link mode \
     (OBJECT or DATA). Read this first to learn the slot indices that mat_slot_set, \
     mat_slot_remove and mat_assign_faces take.",
    MatSlots,
    |py: Python<'_>, p: MatSlots| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = require_object_with_slots(&d, &p.object)?;
        let slots = obj.getattr("material_slots")?;

        let mut rows = Vec::new();
        for (i, slot) in PyIterator::from_object(&slots)?.enumerate() {
            let slot = slot?;
            let mat = slot.getattr("material")?;
            let link: String = slot.getattr("link")?.extract()?;
            rows.push(json!({
                "index": i,
                "material": if mat.is_none() {
                    Value::Null
                } else {
                    Value::String(mat.getattr("name")?.extract::<String>()?)
                },
                "link": link,
            }));
        }
        Ok(json!({ "object": p.object, "slots": rows }))
    }
);

// ---------------------------------------------------------------------------
// mat_slot_add
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatSlotAdd {
    /// Object name. Must be a type that has material slots (not Empty/Camera/Light).
    pub object: String,
    /// Existing material name to put in the new slot. Omit to add an empty slot.
    pub material: Option<String>,
}

crate::tool!(
    "mat_slot_add",
    "Append a material slot to an object and return its index — this is how an \
     object gets a SECOND material, since mat_assign only ever fills slot 0. The \
     new slot paints nothing until mat_assign_faces points some faces at its index.",
    MatSlotAdd,
    |py: Python<'_>, p: MatSlotAdd| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = require_object_with_slots(&d, &p.object)?;

        let mat_obj = match p.material.as_deref() {
            Some(name) => {
                let m = d.getattr("materials")?.call_method1("get", (name,))?;
                if m.is_none() {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "Material not found: {name}"
                    )));
                }
                m
            }
            None => py.None().into_bound(py),
        };

        let materials = obj.getattr("data")?.getattr("materials")?;
        materials.call_method1("append", (mat_obj,))?;
        let index = py_len(py, &materials)? - 1;

        Ok(json!({ "object": p.object, "index": index }))
    }
);

// ---------------------------------------------------------------------------
// mat_slot_remove
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatSlotRemove {
    /// Object name.
    pub object: String,
    /// Slot index to remove, 0-based; must be < the slot count mat_slots reports.
    pub index: i64,
}

crate::tool!(
    "mat_slot_remove",
    "Remove a material slot by index and report which material was in it. Higher \
     slot indices shift down, so re-read mat_slots afterwards. To empty a slot \
     without removing it, call mat_slot_set with no material.",
    MatSlotRemove,
    |py: Python<'_>, p: MatSlotRemove| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = require_object_with_slots(&d, &p.object)?;
        let materials = obj.getattr("data")?.getattr("materials")?;

        let count = py_len(py, &materials)? as i64;
        if p.index < 0 || p.index >= count {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "index {} out of range: object `{}` has {count} material slot(s)",
                p.index, p.object
            )));
        }

        let kwargs = PyDict::new(py);
        kwargs.set_item("index", p.index)?;
        let removed = materials.call_method("pop", (), Some(&kwargs))?;
        let removed_name = if removed.is_none() {
            Value::Null
        } else {
            Value::String(removed.getattr("name")?.extract::<String>()?)
        };

        Ok(json!({ "object": p.object, "index": p.index, "removed": removed_name }))
    }
);

// ---------------------------------------------------------------------------
// mat_slot_set
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatSlotSet {
    /// Object name.
    pub object: String,
    /// Slot index, 0-based; the slot must already exist (mat_slot_add creates one).
    pub index: i64,
    /// Existing material name. Omit to clear the slot, leaving it empty.
    pub material: Option<String>,
}

crate::tool!(
    "mat_slot_set",
    "Assign a material into a specific already-existing slot index, without \
     disturbing the object's other slots or which faces use them. mat_slot_add \
     creates the slot; mat_assign overwrites slot 0 only.",
    MatSlotSet,
    |py: Python<'_>, p: MatSlotSet| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = require_object_with_slots(&d, &p.object)?;
        let slots = obj.getattr("material_slots")?;

        let count = py_len(py, &slots)? as i64;
        if p.index < 0 || p.index >= count {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "index {} out of range: object `{}` has {count} material slot(s)",
                p.index, p.object
            )));
        }
        let slot = slots.get_item(p.index)?;

        let mat_obj = match p.material.as_deref() {
            Some(name) => {
                let m = d.getattr("materials")?.call_method1("get", (name,))?;
                if m.is_none() {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "Material not found: {name}"
                    )));
                }
                m
            }
            None => py.None().into_bound(py),
        };
        // Respects the slot's own link mode (OBJECT vs DATA): writing through
        // `material_slots[i].material` routes to the right storage, unlike
        // writing `obj.data.materials[i]` directly, which is only correct for
        // DATA-linked slots.
        slot.setattr("material", mat_obj)?;

        let mn = slot.getattr("material")?;
        Ok(json!({
            "object": p.object,
            "index": p.index,
            "material": if mn.is_none() {
                Value::Null
            } else {
                Value::String(mn.getattr("name")?.extract::<String>()?)
            },
        }))
    }
);

// ---------------------------------------------------------------------------
// mat_assign_faces
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatAssignFaces {
    /// Object name; must be a MESH (polygons are what carry a material index).
    pub object: String,
    /// Material slot index, 0-based, as listed by mat_slots. Must already exist.
    pub slot_index: i64,
    /// Range mode: first polygon index, inclusive, 0-based. Needs face_end too.
    pub face_start: Option<i64>,
    /// Range mode: last polygon index, inclusive, 0-based. Needs face_start too.
    pub face_end: Option<i64>,
    /// List mode: explicit 0-based polygon indices. Takes precedence over the
    /// range fields when given; every index is bounds-checked before anything is
    /// written, so a bad one changes nothing.
    pub face_indices: Option<Vec<i64>>,
}

crate::tool!(
    "mat_assign_faces",
    "Point a range or list of mesh faces at a material slot — this is how a \
     multi-material object is actually made, since mat_assign paints the whole \
     object and mat_slot_add only creates the empty slot. Give either face_indices \
     or both face_start and face_end.",
    MatAssignFaces,
    |py: Python<'_>, p: MatAssignFaces| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.object.as_str(),))?;
        if obj.is_none() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Object not found: {}",
                p.object
            )));
        }
        let ty: String = obj.getattr("type")?.extract()?;
        if ty != "MESH" {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "mat_assign_faces requires a MESH object, `{}` is {ty}",
                p.object
            )));
        }

        let mesh = obj.getattr("data")?;
        let polygons = mesh.getattr("polygons")?;
        let face_count = py_len(py, &polygons)? as i64;

        let slot_count = py_len(py, &obj.getattr("material_slots")?)? as i64;
        if p.slot_index < 0 || p.slot_index >= slot_count {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "slot_index {} out of range: object `{}` has {slot_count} material slot(s)",
                p.slot_index, p.object
            )));
        }

        let indices: Vec<i64> = if let Some(list) = p.face_indices {
            list
        } else if let (Some(start), Some(end)) = (p.face_start, p.face_end) {
            (start..=end).collect()
        } else {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "give either face_indices or both face_start and face_end",
            ));
        };
        if indices.is_empty() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "no face indices given",
            ));
        }
        for &i in &indices {
            if i < 0 || i >= face_count {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "face index {i} out of range: mesh `{}` has {face_count} face(s)",
                    p.object
                )));
            }
        }

        for &i in &indices {
            polygons
                .get_item(i)?
                .setattr("material_index", p.slot_index)?;
        }
        mesh.call_method0("update")?;

        Ok(json!({
            "object": p.object,
            "slot_index": p.slot_index,
            "faces_assigned": indices.len(),
        }))
    }
);

// ---------------------------------------------------------------------------
// mat_settings
// ---------------------------------------------------------------------------

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatSettings {
    /// Existing material name (mat_list).
    pub name: String,
    /// EEVEE blend mode: OPAQUE (default), CLIP, HASHED or BLEND — verified on
    /// Blender 5.2. A transparent material (alpha < 1 via mat_update) looks wrong
    /// with no diagnosable cause unless this is set too. Matched case-insensitively
    /// against this Blender's own RNA enum, so a future rename is reported with the
    /// real list rather than silently accepted.
    pub blend_method: Option<String>,
    /// Cull back-facing polygons (RNA: `use_backface_culling`). Default false.
    pub backface_culling: Option<bool>,
    /// How a displacement input is realised: BUMP (default), DISPLACEMENT or BOTH
    /// — verified on Blender 5.2, and re-checked against this Blender's RNA enum.
    pub displacement_method: Option<String>,
    /// Use the material's node tree. Default true, and effectively fixed on
    /// Blender 5: the setter is deprecated and a no-op, so the response reports
    /// what the material really ended up with, not what was asked for.
    pub use_nodes: Option<bool>,
}

crate::tool!(
    "mat_settings",
    "Set the whole-material flags that live beside the node tree: blend_method, \
     displacement_method, backface_culling, use_nodes. (Blender 5's EEVEE Next \
     removed Material.shadow_method entirely, so there is no shadow mode here.) \
     Enum values are validated against this Blender's own RNA and rejected with \
     the accepted list, never guessed. Socket values are mat_update's job.",
    MatSettings,
    |py: Python<'_>, p: MatSettings| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let mat = d
            .getattr("materials")?
            .call_method1("get", (p.name.as_str(),))?;
        if mat.is_none() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Material not found: {}",
                p.name
            )));
        }

        let mut updated = serde_json::Map::new();

        if let Some(v) = p.blend_method.as_deref() {
            set_rna_enum(&mat, "blend_method", v)?;
            updated.insert(
                "blend_method".into(),
                json!(mat.getattr("blend_method")?.extract::<String>()?),
            );
        }
        if let Some(v) = p.backface_culling {
            mat.setattr("use_backface_culling", v)?;
            updated.insert(
                "backface_culling".into(),
                json!(mat.getattr("use_backface_culling")?.extract::<bool>()?),
            );
        }
        if let Some(v) = p.displacement_method.as_deref() {
            set_rna_enum(&mat, "displacement_method", v)?;
            updated.insert(
                "displacement_method".into(),
                json!(mat.getattr("displacement_method")?.extract::<String>()?),
            );
        }
        if let Some(v) = p.use_nodes {
            mat.setattr("use_nodes", v)?;
            // Re-read rather than echo `v`: Blender 5.0 kept the setter for
            // compatibility (deprecated, removed in 6.0) but it is a no-op —
            // a material always has a node tree now. Echoing the request
            // here would silently lie about what actually happened, exactly
            // the class of bug this tool exists to make diagnosable.
            updated.insert(
                "use_nodes".into(),
                json!(mat.getattr("use_nodes")?.extract::<bool>()?),
            );
        }

        Ok(json!({ "name": p.name, "updated": Value::Object(updated) }))
    }
);
