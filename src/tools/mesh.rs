//! Mesh, curve, text, shape-key and instancing tools. Registered automatically —
//! nothing outside this file names them.
//!
//! Ported verbatim from the Python handlers in `addon/mcp_server/__init__.py`
//! (`_h_mesh_*`, `_h_curve_*`, `_h_text_*`, `_h_shapekey_*`, `_h_obj_instance_*`,
//! `_h_obj_convert_to_mesh` -> `mesh_convert_to`). Single/batch dualities and
//! "truthy" defaults from the Python source are preserved where they affect
//! behaviour.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyIterator, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, op_ok, py_len};

// ============================================================================
// Shared helpers
// ============================================================================

/// Look up `bpy.data.objects[name]`, requiring it to exist and be of type
/// `want` (e.g. `"MESH"`, `"CURVE"`, `"FONT"`). Mirrors the `if not obj or
/// obj.type != X: raise ValueError(...)` guard repeated at the top of nearly
/// every handler in the Python source.
fn get_typed_obj<'py>(
    d: &Bound<'py, PyAny>,
    name: &str,
    want: &str,
    label: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    let ok = if obj.is_none() {
        false
    } else {
        obj.getattr("type")?.extract::<String>()? == want
    };
    if !ok {
        return Err(PyValueError::new_err(format!("{label} not found: {name}")));
    }
    Ok(obj)
}

/// Extract a Python float sequence (e.g. vertex `co`, `normal`) into `Vec<f64>`.
fn py_vec_f64(seq: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    let it = PyIterator::from_object(seq)?;
    let mut v = Vec::new();
    for x in it {
        v.push(x?.extract::<f64>()?);
    }
    Ok(v)
}

/// Extract a Python int sequence (e.g. `polygon.vertices`) into `Vec<i64>`.
fn py_vec_i64(seq: &Bound<'_, PyAny>) -> PyResult<Vec<i64>> {
    let it = PyIterator::from_object(seq)?;
    let mut v = Vec::new();
    for x in it {
        v.push(x?.extract::<i64>()?);
    }
    Ok(v)
}

// ============================================================================
// Mesh Editing
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshGetData {
    /// Mesh object name
    pub name: String,
    /// Also return every vertex position as [x, y, z] in local mesh space.
    /// Default false — the payload is one array entry per vertex
    pub include_verts: Option<bool>,
    /// Also return each polygon's vertex indices. Default false
    pub include_faces: Option<bool>,
    /// Also return every vertex normal as a unit [x, y, z]. Default false
    pub include_normals: Option<bool>,
}

crate::tool!(
    "mesh_get_data",
    "Read a mesh's vertex/edge/polygon counts and material slots, optionally with the \
     raw vertex positions, face indices and normals. To write positions back use \
     mesh_set_data; for UVs use mesh_get_uvs",
    MeshGetData,
    |py: Python<'_>, p: MeshGetData| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed_obj(&d, &p.name, "MESH", "Mesh")?;
        let mesh = obj.getattr("data")?;

        let verts = mesh.getattr("vertices")?;
        let edges = mesh.getattr("edges")?;
        let polys = mesh.getattr("polygons")?;

        let materials = mesh.getattr("materials")?;
        let mut mat_names = Vec::new();
        for m in PyIterator::from_object(&materials)? {
            let m = m?;
            if m.is_none() {
                mat_names.push(Value::Null);
            } else {
                mat_names.push(Value::String(m.getattr("name")?.extract::<String>()?));
            }
        }

        let mut r = json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "vertices": py_len(py, &verts)?,
            "edges": py_len(py, &edges)?,
            "polygons": py_len(py, &polys)?,
            "materials": mat_names,
        });

        if p.include_verts.unwrap_or(false) {
            let mut positions = Vec::new();
            for v in PyIterator::from_object(&verts)? {
                positions.push(py_vec_f64(&v?.getattr("co")?)?);
            }
            r["vertex_positions"] = json!(positions);
        }
        if p.include_faces.unwrap_or(false) {
            let mut faces = Vec::new();
            for f in PyIterator::from_object(&polys)? {
                faces.push(py_vec_i64(&f?.getattr("vertices")?)?);
            }
            r["face_indices"] = json!(faces);
        }
        if p.include_normals.unwrap_or(false) {
            let mut normals = Vec::new();
            for v in PyIterator::from_object(&verts)? {
                normals.push(py_vec_f64(&v?.getattr("normal")?)?);
            }
            r["vertex_normals"] = json!(normals);
        }

        Ok(r)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshSetData {
    /// Mesh object name
    pub name: String,
    /// One [x, y, z] per vertex, in local mesh space, in vertex-index order. The
    /// list length must equal the mesh's current vertex count or the call fails —
    /// this moves vertices, it never adds or removes them
    pub vertex_positions: Option<Vec<Vec<f64>>>,
}

crate::tool!(
    "mesh_set_data",
    "Overwrite every vertex position of a mesh from an explicit list (topology \
     unchanged). Read the current ones with mesh_get_data; to move only part of a mesh \
     use mesh_select + mesh_transform_selection",
    MeshSetData,
    |py: Python<'_>, p: MeshSetData| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed_obj(&d, &p.name, "MESH", "Mesh")?;
        let mesh = obj.getattr("data")?;
        let verts = mesh.getattr("vertices")?;
        let vert_count = py_len(py, &verts)?;

        if let Some(positions) = p.vertex_positions {
            if positions.len() != vert_count {
                return Err(PyValueError::new_err(format!(
                    "Vertex count mismatch: {} vs {}",
                    positions.len(),
                    vert_count
                )));
            }
            for (i, pos) in positions.iter().enumerate() {
                let v = verts.get_item(i as i64)?;
                v.setattr("co", PyTuple::new(py, pos.iter().copied())?)?;
            }
            mesh.call_method0("update")?;
        }

        Ok(
            json!({ "name": obj.getattr("name")?.extract::<String>()?, "vertices_updated": vert_count }),
        )
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshExtrude {
    /// Mesh object name
    pub name: String,
    /// Displacement [x, y, z] applied to the extruded geometry, in local mesh
    /// space. Default: [0, 0, 1]
    pub direction: Option<Vec<f64>>,
}

crate::tool!(
    "mesh_extrude",
    "Convenience wrapper: extrude the mesh's currently selected faces by a fixed \
     [x, y, z] vector. For extruding along the normal, a chosen distance, or several \
     edit steps in one call, use mesh_edit with op `extrude`",
    MeshExtrude,
    |py: Python<'_>, p: MeshExtrude| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed_obj(&d, &p.name, "MESH", "Mesh")?;
        let direction = p.direction.unwrap_or_else(|| vec![0.0, 0.0, 1.0]);

        bpy.getattr("context")?
            .getattr("view_layer")?
            .getattr("objects")?
            .setattr("active", &obj)?;
        set_mode(py, &bpy, "EDIT")?;

        let inner = PyDict::new(py);
        inner.set_item("value", PyTuple::new(py, direction.iter().copied())?)?;
        let kwargs = PyDict::new(py);
        kwargs.set_item("TRANSFORM_OT_translate", inner)?;
        let result = bpy.getattr("ops")?.getattr("mesh")?.call_method(
            "extrude_region_move",
            (),
            Some(&kwargs),
        )?;
        op_ok(&result, &format!("extrude {}", p.name))?;

        set_mode(py, &bpy, "OBJECT")?;

        Ok(json!({ "name": obj.getattr("name")?.extract::<String>()?, "direction": direction }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshSubdivide {
    /// Mesh object name
    pub name: String,
    /// Number of cuts per edge (1 = each edge split once). Default 1, Blender's
    /// own default
    pub cuts: Option<i32>,
}

crate::tool!(
    "mesh_subdivide",
    "Convenience wrapper: subdivide the WHOLE mesh — it selects everything first, so \
     any selection built with mesh_select is lost. To subdivide only part of a mesh, \
     use mesh_edit with op `subdivide`",
    MeshSubdivide,
    |py: Python<'_>, p: MeshSubdivide| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed_obj(&d, &p.name, "MESH", "Mesh")?;
        let cuts = p.cuts.unwrap_or(1);

        bpy.getattr("context")?
            .getattr("view_layer")?
            .getattr("objects")?
            .setattr("active", &obj)?;
        set_mode(py, &bpy, "EDIT")?;
        select_all(py, &bpy, "SELECT")?;

        let kwargs = PyDict::new(py);
        kwargs.set_item("number_cuts", cuts)?;
        let result =
            bpy.getattr("ops")?
                .getattr("mesh")?
                .call_method("subdivide", (), Some(&kwargs))?;
        op_ok(&result, &format!("subdivide {}", p.name))?;

        set_mode(py, &bpy, "OBJECT")?;

        let verts = obj.getattr("data")?.getattr("vertices")?;
        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "vertices": py_len(py, &verts)?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshGetUvs {
    /// Mesh object name
    pub name: String,
}

crate::tool!(
    "mesh_get_uvs",
    "Report whether a mesh has UVs, which UV layer is active and how many UV loops it \
     holds. To generate UVs use mesh_unwrap",
    MeshGetUvs,
    |py: Python<'_>, p: MeshGetUvs| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed_obj(&d, &p.name, "MESH", "Mesh")?;
        let mesh = obj.getattr("data")?;
        let name: String = obj.getattr("name")?.extract()?;

        let uv_layers = mesh.getattr("uv_layers")?;
        if py_len(py, &uv_layers)? == 0 {
            return Ok(json!({ "name": name, "has_uvs": false }));
        }

        let uv_layer = uv_layers.getattr("active")?;
        let uv_layer_name: String = uv_layer.getattr("name")?.extract()?;
        let loops = mesh.getattr("loops")?;
        Ok(json!({
            "name": name,
            "has_uvs": true,
            "uv_layer": uv_layer_name,
            "uv_count": py_len(py, &loops)?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshUnwrap {
    /// Mesh object name
    pub name: String,
    /// "smart" runs Smart UV Project (needs no seams); anything else is passed to
    /// `bpy.ops.uv.unwrap` uppercased, where Blender 5.2 accepts ANGLE_BASED,
    /// CONFORMAL and MINIMUM_STRETCH (the last one is new in 4.x — verified live).
    /// Default: angle_based
    pub method: Option<String>,
    /// method="smart" only: the angle limit in DEGREES, 0-90. Default 66, which is
    /// also Blender's own default
    pub angle: Option<f64>,
}

crate::tool!(
    "mesh_unwrap",
    "Unwrap a mesh's UVs with Angle Based / Conformal / Minimum Stretch, or with Smart \
     UV Project. It selects the whole mesh, so run it BEFORE building a selection. \
     Mark seams first with mesh_select + mesh_mark, or the non-smart methods give one \
     stretched island",
    MeshUnwrap,
    |py: Python<'_>, p: MeshUnwrap| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed_obj(&d, &p.name, "MESH", "Mesh")?;
        let method = p
            .method
            .unwrap_or_else(|| "angle_based".to_string())
            .to_lowercase();

        bpy.getattr("context")?
            .getattr("view_layer")?
            .getattr("objects")?
            .setattr("active", &obj)?;
        set_mode(py, &bpy, "EDIT")?;
        select_all(py, &bpy, "SELECT")?;

        let uv = bpy.getattr("ops")?.getattr("uv")?;
        if method == "smart" {
            let kwargs = PyDict::new(py);
            // `smart_project`'s `angle_limit` is an RNA ANGLE property, i.e.
            // radians with a hard max of pi/2 — verified live on 5.2: passing 66
            // raw is silently clamped and does NOT mean the documented 66 degrees.
            kwargs.set_item("angle_limit", p.angle.unwrap_or(66.0).to_radians())?;
            let result = uv.call_method("smart_project", (), Some(&kwargs))?;
            op_ok(&result, &format!("smart UV project {}", p.name))?;
        } else {
            let kwargs = PyDict::new(py);
            kwargs.set_item("method", method.to_uppercase())?;
            // Verified live: `uv.unwrap` returns {'CANCELLED'} (not a raise) when
            // there is nothing selected to unwrap — the one case in this file
            // where the "empty selection" failure mode from the task brief is real.
            let result = uv.call_method("unwrap", (), Some(&kwargs))?;
            op_ok(&result, &format!("unwrap {}", p.name))?;
        }

        set_mode(py, &bpy, "OBJECT")?;

        Ok(json!({ "name": obj.getattr("name")?.extract::<String>()?, "method": method }))
    }
);

/// `bpy.ops.object.mode_set(mode=...)`, used by every edit-mode tool above.
///
/// Not checked with `op_ok`: verified live against Blender 5.0 that setting a
/// mode already active (the "OBJECT" restore calls above) returns
/// `{'FINISHED'}`, never a silent `{'CANCELLED'}`; a genuinely incompatible
/// mode/object pairing raises instead, which `?` already propagates.
fn set_mode(py: Python<'_>, bpy: &Bound<'_, PyAny>, mode: &str) -> PyResult<()> {
    let kwargs = PyDict::new(py);
    kwargs.set_item("mode", mode)?;
    bpy.getattr("ops")?
        .getattr("object")?
        .call_method("mode_set", (), Some(&kwargs))?;
    Ok(())
}

/// `bpy.ops.mesh.select_all(action=...)`. Every caller in this file passes
/// `"SELECT"` as prep before the real edit operator, never `"DESELECT"`.
///
/// Not checked with `op_ok`: verified live that `SELECT` always returns
/// `{'FINISHED'}` (unlike `DESELECT`, which legitimately cancels when nothing
/// was selected to begin with). If the mesh truly has nothing to select, the
/// real operator that follows will report that itself.
fn select_all(py: Python<'_>, bpy: &Bound<'_, PyAny>, action: &str) -> PyResult<()> {
    let kwargs = PyDict::new(py);
    kwargs.set_item("action", action)?;
    bpy.getattr("ops")?
        .getattr("mesh")?
        .call_method("select_all", (), Some(&kwargs))?;
    Ok(())
}

// ============================================================================
// Curves & Text
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct CurveCreate {
    /// Spline type, uppercased before use. Blender 5.2 accepts BEZIER, NURBS and
    /// POLY. Default: BEZIER (handles are set to AUTO)
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Name for the curve datablock and object. Default: "Curve"
    pub name: Option<String>,
    /// Object location [x, y, z]. Default: [0, 0, 0]
    pub location: Option<Vec<f64>>,
    /// Control points, one [x, y, z] each, in curve-local space. Default: a
    /// 3-point demo curve [[0,0,0], [1,0,0], [2,1,0]]
    pub points: Option<Vec<Vec<f64>>>,
    /// "3D" (points may leave the XY plane) or "2D" (flat, fillable). Default: 3D
    pub dimensions: Option<String>,
    /// Round profile radius swept along the curve, giving it thickness. 0 = a
    /// zero-width curve. Default: 0 (a value of exactly 0.0 is skipped)
    pub bevel_depth: Option<f64>,
    /// Straight extrusion distance along the local Z, each side. Default: 0
    pub extrude: Option<f64>,
}

crate::tool!(
    "curve_create",
    "Create a Bezier/NURBS/Poly curve object from a list of control points, optionally \
     given thickness by bevel_depth/extrude. Edit points afterwards with \
     curve_add_point / curve_update_point; turn it into geometry with mesh_convert_to",
    CurveCreate,
    |py: Python<'_>, p: CurveCreate| {
        let bpy = py.import("bpy")?;
        let curve_type = p
            .r#type
            .unwrap_or_else(|| "BEZIER".to_string())
            .to_uppercase();
        let loc = p.location.unwrap_or_else(|| vec![0.0, 0.0, 0.0]);
        let name = p.name.unwrap_or_else(|| "Curve".to_string());
        let points = p.points.unwrap_or_else(|| {
            vec![
                vec![0.0, 0.0, 0.0],
                vec![1.0, 0.0, 0.0],
                vec![2.0, 1.0, 0.0],
            ]
        });

        let curves = data(&bpy)?.getattr("curves")?;
        let curve_data = curves.call_method1("new", (name.as_str(), "CURVE"))?;
        curve_data.setattr(
            "dimensions",
            p.dimensions.unwrap_or_else(|| "3D".to_string()),
        )?;

        let spline = curve_data
            .getattr("splines")?
            .call_method1("new", (curve_type.as_str(),))?;

        // `.add(n)` grows the collection by `n` beyond the single point every new
        // spline starts with; `points.len() - 1` is computed in i64 so an (unlikely)
        // empty `points` list cannot underflow a usize and panic.
        let n_extra = points.len() as i64 - 1;
        if curve_type == "BEZIER" {
            let bezier_points = spline.getattr("bezier_points")?;
            bezier_points.call_method1("add", (n_extra,))?;
            for (i, pt) in points.iter().enumerate() {
                let bp = bezier_points.get_item(i as i64)?;
                bp.setattr("co", PyTuple::new(py, pt.iter().copied())?)?;
                bp.setattr("handle_left_type", "AUTO")?;
                bp.setattr("handle_right_type", "AUTO")?;
            }
        } else {
            let pts = spline.getattr("points")?;
            pts.call_method1("add", (n_extra,))?;
            for (i, pt) in points.iter().enumerate() {
                let mut co = pt.clone();
                co.push(1.0); // NURBS points are 4D (homogeneous weight).
                pts.get_item(i as i64)?
                    .setattr("co", PyTuple::new(py, co)?)?;
            }
        }

        // Python's `if bevel := p.get("bevel_depth")` treats 0.0 as falsy and skips
        // the assignment; replicated here since curve defaults are already 0.0.
        if let Some(b) = p.bevel_depth
            && b != 0.0
        {
            curve_data.setattr("bevel_depth", b)?;
        }
        if let Some(e) = p.extrude
            && e != 0.0
        {
            curve_data.setattr("extrude", e)?;
        }

        let curve_data_name: String = curve_data.getattr("name")?.extract()?;
        let curve_obj = data(&bpy)?
            .getattr("objects")?
            .call_method1("new", (curve_data_name.as_str(), &curve_data))?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&curve_obj,))?;
        curve_obj.setattr("location", PyTuple::new(py, loc)?)?;

        Ok(json!({
            "name": curve_obj.getattr("name")?.extract::<String>()?,
            "type": curve_type,
            "points": points.len(),
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct CurveAddPoint {
    /// Curve object name
    pub name: String,
    /// Position [x, y, z] of the new control point, in curve-local space. It is
    /// appended at the end of the spline
    pub position: Vec<f64>,
    /// Which spline of the curve to append to. Default: 0 (the first)
    pub spline_index: Option<i32>,
}

crate::tool!(
    "curve_add_point",
    "Append one control point to the end of a curve's spline (Bezier handles are set \
     to AUTO). To move an existing point instead use curve_update_point",
    CurveAddPoint,
    |py: Python<'_>, p: CurveAddPoint| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed_obj(&d, &p.name, "CURVE", "Curve")?;
        let spline_index = p.spline_index.unwrap_or(0);
        let spline = obj
            .getattr("data")?
            .getattr("splines")?
            .get_item(spline_index)?;
        let spline_type: String = spline.getattr("type")?.extract()?;

        let point_count = if spline_type == "BEZIER" {
            let bezier_points = spline.getattr("bezier_points")?;
            bezier_points.call_method1("add", (1,))?;
            let bp = bezier_points.get_item(-1i32)?;
            bp.setattr("co", PyTuple::new(py, p.position.iter().copied())?)?;
            bp.setattr("handle_left_type", "AUTO")?;
            bp.setattr("handle_right_type", "AUTO")?;
            py_len(py, &bezier_points)?
        } else {
            let pts = spline.getattr("points")?;
            pts.call_method1("add", (1,))?;
            let mut co = p.position.clone();
            co.push(1.0);
            pts.get_item(-1i32)?.setattr("co", PyTuple::new(py, co)?)?;
            py_len(py, &pts)?
        };

        Ok(json!({ "name": obj.getattr("name")?.extract::<String>()?, "points": point_count }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct CurveUpdatePoint {
    /// Curve object name
    pub name: String,
    /// Index of the control point within the spline. Negative indices count from
    /// the end, as in Python
    pub point_index: i32,
    /// New position [x, y, z], curve-local. Optional on a Bezier spline;
    /// REQUIRED on a NURBS/Poly spline, which has nothing else to set
    pub position: Option<Vec<f64>>,
    /// Bezier only: absolute position [x, y, z] of the left handle
    pub handle_left: Option<Vec<f64>>,
    /// Bezier only: absolute position [x, y, z] of the right handle
    pub handle_right: Option<Vec<f64>>,
    /// Bezier only: handle type applied to BOTH handles. Blender 5.2 accepts
    /// FREE, VECTOR, ALIGNED, AUTO. Unchanged if omitted
    pub handle_type: Option<String>,
    /// Which spline of the curve the point belongs to. Default: 0
    pub spline_index: Option<i32>,
}

crate::tool!(
    "curve_update_point",
    "Move one existing control point of a curve and, on a Bezier spline, set its \
     handles and handle type. To add a point instead use curve_add_point",
    CurveUpdatePoint,
    |py: Python<'_>, p: CurveUpdatePoint| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed_obj(&d, &p.name, "CURVE", "Curve")?;
        let spline_index = p.spline_index.unwrap_or(0);
        let spline = obj
            .getattr("data")?
            .getattr("splines")?
            .get_item(spline_index)?;
        let spline_type: String = spline.getattr("type")?.extract()?;
        let idx = p.point_index;

        if spline_type == "BEZIER" {
            let bp = spline.getattr("bezier_points")?.get_item(idx)?;
            if let Some(pos) = &p.position {
                bp.setattr("co", PyTuple::new(py, pos.iter().copied())?)?;
            }
            if let Some(hl) = &p.handle_left {
                bp.setattr("handle_left", PyTuple::new(py, hl.iter().copied())?)?;
            }
            if let Some(hr) = &p.handle_right {
                bp.setattr("handle_right", PyTuple::new(py, hr.iter().copied())?)?;
            }
            if let Some(ht) = &p.handle_type {
                bp.setattr("handle_left_type", ht.as_str())?;
                bp.setattr("handle_right_type", ht.as_str())?;
            }
        } else {
            // The Python source reads `p["position"]` unconditionally on this
            // branch (no `"position" in p` guard), so a missing position is a
            // hard error here too rather than a silent no-op.
            let pos = p
                .position
                .ok_or_else(|| PyValueError::new_err("missing position"))?;
            let mut co = pos;
            co.push(1.0);
            spline
                .getattr("points")?
                .get_item(idx)?
                .setattr("co", PyTuple::new(py, co)?)?;
        }

        Ok(json!({ "name": obj.getattr("name")?.extract::<String>()?, "point_index": idx }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct TextCreate {
    /// The text to display. Newlines make multiple lines. Default: "Text"
    pub body: Option<String>,
    /// Name for the text datablock and object. Default: "Text"
    pub name: Option<String>,
    /// Object location [x, y, z]. Default: [0, 0, 0]
    pub location: Option<Vec<f64>>,
    /// Font size in Blender units (cap height scale). Default: 1.0
    pub size: Option<f64>,
    /// Extrusion depth along local Z, each side — this is what makes the text
    /// solid instead of a flat sheet. Default: 0.0
    pub extrude: Option<f64>,
    /// Rounding radius on the extruded edge. Default: 0.0
    pub bevel_depth: Option<f64>,
    /// Filesystem path to a font file (.ttf/.otf) to load. Default: Blender's
    /// built-in Bfont
    pub font: Option<String>,
    /// Horizontal alignment. Blender 5.2 accepts LEFT, CENTER, RIGHT, JUSTIFY,
    /// FLUSH. Default: LEFT
    pub align_x: Option<String>,
    /// Vertical alignment. Blender 5.2 accepts TOP, TOP_BASELINE, CENTER,
    /// BOTTOM_BASELINE, BOTTOM. Default: TOP_BASELINE (Blender's own default)
    pub align_y: Option<String>,
}

crate::tool!(
    "text_create",
    "Create a 3D text (FONT) object with body, size, alignment and optional extrusion \
     or custom font file. Change it later with text_update; to turn it into editable \
     geometry use mesh_convert_to",
    TextCreate,
    |py: Python<'_>, p: TextCreate| {
        let bpy = py.import("bpy")?;
        let loc = p.location.unwrap_or_else(|| vec![0.0, 0.0, 0.0]);
        let name = p.name.unwrap_or_else(|| "Text".to_string());

        let curves = data(&bpy)?.getattr("curves")?;
        let text_data = curves.call_method1("new", (name.as_str(), "FONT"))?;
        text_data.setattr("body", p.body.unwrap_or_else(|| "Text".to_string()))?;
        text_data.setattr("size", p.size.unwrap_or(1.0))?;
        text_data.setattr("extrude", p.extrude.unwrap_or(0.0))?;
        text_data.setattr("bevel_depth", p.bevel_depth.unwrap_or(0.0))?;
        if let Some(font) = p.font {
            let loaded = data(&bpy)?
                .getattr("fonts")?
                .call_method1("load", (font,))?;
            text_data.setattr("font", loaded)?;
        }
        text_data.setattr("align_x", p.align_x.unwrap_or_else(|| "LEFT".to_string()))?;
        text_data.setattr(
            "align_y",
            p.align_y.unwrap_or_else(|| "TOP_BASELINE".to_string()),
        )?;

        let text_data_name: String = text_data.getattr("name")?.extract()?;
        let text_obj = data(&bpy)?
            .getattr("objects")?
            .call_method1("new", (text_data_name.as_str(), &text_data))?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&text_obj,))?;
        text_obj.setattr("location", PyTuple::new(py, loc)?)?;

        Ok(json!({
            "name": text_obj.getattr("name")?.extract::<String>()?,
            "body": text_data.getattr("body")?.extract::<String>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct TextUpdate {
    /// Text (FONT) object name
    pub name: String,
    /// New text content. Unchanged if omitted
    pub body: Option<String>,
    /// Font size in Blender units. Unchanged if omitted
    pub size: Option<f64>,
    /// Extrusion depth along local Z, each side. Unchanged if omitted
    pub extrude: Option<f64>,
    /// Rounding radius on the extruded edge. Unchanged if omitted
    pub bevel_depth: Option<f64>,
    /// LEFT, CENTER, RIGHT, JUSTIFY or FLUSH. Unchanged if omitted
    pub align_x: Option<String>,
    /// TOP, TOP_BASELINE, CENTER, BOTTOM_BASELINE or BOTTOM. Unchanged if omitted
    pub align_y: Option<String>,
}

crate::tool!(
    "text_update",
    "Change an existing text object's body, size, extrusion, bevel and alignment. \
     To create one use text_create",
    TextUpdate,
    |py: Python<'_>, p: TextUpdate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed_obj(&d, &p.name, "FONT", "Text object")?;
        let text = obj.getattr("data")?;

        if let Some(body) = p.body {
            text.setattr("body", body)?;
        }
        if let Some(size) = p.size {
            text.setattr("size", size)?;
        }
        if let Some(extrude) = p.extrude {
            text.setattr("extrude", extrude)?;
        }
        if let Some(bevel) = p.bevel_depth {
            text.setattr("bevel_depth", bevel)?;
        }
        if let Some(ax) = p.align_x {
            text.setattr("align_x", ax)?;
        }
        if let Some(ay) = p.align_y {
            text.setattr("align_y", ay)?;
        }

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "body": text.getattr("body")?.extract::<String>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshConvertTo {
    /// Single mode: object to convert. Must be a CURVE, FONT or SURFACE object.
    /// Used only when `names` is absent
    pub name: Option<String>,
    /// Batch mode: object names to convert. Takes priority over `name`; per-item
    /// failures are reported in `results`
    pub names: Option<Vec<String>>,
}

crate::tool!(
    "mesh_convert_to",
    "Convert CURVE / FONT / SURFACE objects into real meshes in place, so mesh_select \
     and mesh_edit can work on them. This is destructive — the curve/text data is gone \
     afterwards",
    MeshConvertTo,
    |py: Python<'_>, p: MeshConvertTo| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        let names: Vec<String> = p
            .names
            .or_else(|| p.name.map(|n| vec![n]))
            .unwrap_or_default();

        let mut results = Vec::new();
        for name in &names {
            results.push(convert_one_to_mesh(py, &bpy, &d, name)?);
        }

        if results.len() == 1 {
            let only = results.remove(0);
            // A lone target that failed outright is a hard error, not data — only a
            // genuine multi-target *partial* success belongs in `results` below.
            if let Some(err) = only.get("error").and_then(Value::as_str) {
                return Err(PyValueError::new_err(format!(
                    "cannot convert '{}': {err}",
                    names[0]
                )));
            }
            return Ok(only);
        }
        let converted = results.iter().filter(|r| r.get("error").is_none()).count();
        Ok(json!({ "converted": converted, "results": results }))
    }
);

fn convert_one_to_mesh(
    py: Python<'_>,
    bpy: &Bound<'_, PyAny>,
    d: &Bound<'_, PyAny>,
    name: &str,
) -> PyResult<Value> {
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    if obj.is_none() {
        return Ok(json!({ "name": name, "error": "not found" }));
    }
    let ty: String = obj.getattr("type")?.extract()?;
    if !matches!(ty.as_str(), "CURVE" | "FONT" | "SURFACE") {
        return Ok(json!({ "name": name, "error": format!("type {ty} cannot be converted") }));
    }

    bpy.getattr("context")?
        .getattr("view_layer")?
        .getattr("objects")?
        .setattr("active", &obj)?;
    obj.call_method1("select_set", (true,))?;

    let kwargs = PyDict::new(py);
    kwargs.set_item("target", "MESH")?;
    let result =
        bpy.getattr("ops")?
            .getattr("object")?
            .call_method("convert", (), Some(&kwargs))?;
    // A decline here is a per-item batch failure like "not found" above, not a
    // call failure: report it as `error` data instead of aborting the rest of
    // `mesh_convert_to`'s batch (the caller propagates this fn's `Err` with `?`).
    if let Err(e) = op_ok(&result, &format!("convert {name} to mesh")) {
        return Ok(json!({ "name": name, "error": e.to_string() }));
    }

    // Not checked: `object.select_all(DESELECT)` legitimately returns
    // {'CANCELLED'} when there is nothing left to deselect (verified live) —
    // that is not a failure of this restore step, it is the goal already met.
    let deselect = PyDict::new(py);
    deselect.set_item("action", "DESELECT")?;
    bpy.getattr("ops")?
        .getattr("object")?
        .call_method("select_all", (), Some(&deselect))?;

    let verts = obj.getattr("data")?.getattr("vertices")?;
    Ok(json!({
        "name": obj.getattr("name")?.extract::<String>()?,
        "type": ty,
        "vertices": py_len(py, &verts)?,
    }))
}

// ============================================================================
// Shape Keys
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ShapekeyAdd {
    /// Mesh object name
    pub name: String,
    /// Name for the new shape key. Default: Blender's own ("Key 1", "Key 2", ...)
    pub key_name: Option<String>,
    /// true captures the current blended result of the existing keys as the new
    /// key's shape; false starts from the Basis. Default: false
    pub from_mix: Option<bool>,
    /// Initial blend weight of the new key, normally 0-1. Default: 0.0
    pub value: Option<f64>,
}

crate::tool!(
    "shapekey_add",
    "Add a shape key to a mesh, creating the Basis key first if the mesh has none. \
     Change its weight or range later with shapekey_update; list them with \
     shapekey_list",
    ShapekeyAdd,
    |py: Python<'_>, p: ShapekeyAdd| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed_obj(&d, &p.name, "MESH", "Mesh")?;

        bpy.getattr("context")?
            .getattr("view_layer")?
            .getattr("objects")?
            .setattr("active", &obj)?;

        let shape_key_add = bpy
            .getattr("ops")?
            .getattr("object")?
            .getattr("shape_key_add")?;

        // A mesh with no shape keys yet needs an explicit Basis first, or the
        // key added below silently becomes the Basis instead of a real target.
        if obj.getattr("data")?.getattr("shape_keys")?.is_none() {
            let kwargs = PyDict::new(py);
            kwargs.set_item("from_mix", false)?;
            let result = shape_key_add.call((), Some(&kwargs))?;
            op_ok(&result, &format!("add Basis shape key to {}", p.name))?;
        }

        let kwargs = PyDict::new(py);
        kwargs.set_item("from_mix", p.from_mix.unwrap_or(false))?;
        let result = shape_key_add.call((), Some(&kwargs))?;
        op_ok(&result, &format!("add shape key to {}", p.name))?;

        let key_blocks = obj
            .getattr("data")?
            .getattr("shape_keys")?
            .getattr("key_blocks")?;
        let sk = key_blocks.get_item(-1i32)?;
        if let Some(kn) = p.key_name {
            sk.setattr("name", kn)?;
        }
        sk.setattr("value", p.value.unwrap_or(0.0))?;

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "shape_key": sk.getattr("name")?.extract::<String>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ShapekeyRemove {
    /// Mesh object name
    pub name: String,
    /// Name of the shape key to remove, as reported by shapekey_list. Removing
    /// the Basis key discards every other key's offsets with it
    pub key_name: String,
}

crate::tool!(
    "shapekey_remove",
    "Remove one shape key from a mesh by name. To only switch it off without losing \
     it, use shapekey_update with mute",
    ShapekeyRemove,
    |py: Python<'_>, p: ShapekeyRemove| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        let shape_keys = if obj.is_none() {
            None
        } else {
            let sk = obj.getattr("data")?.getattr("shape_keys")?;
            if sk.is_none() { None } else { Some(sk) }
        };
        let Some(shape_keys) = shape_keys else {
            return Err(PyValueError::new_err(format!(
                "Object has no shape keys: {}",
                p.name
            )));
        };

        let sk = shape_keys
            .getattr("key_blocks")?
            .call_method1("get", (p.key_name.as_str(),))?;
        if sk.is_none() {
            return Err(PyValueError::new_err(format!(
                "Shape key not found: {}",
                p.key_name
            )));
        }
        obj.call_method1("shape_key_remove", (&sk,))?;

        Ok(json!({ "name": obj.getattr("name")?.extract::<String>()?, "removed": p.key_name }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ShapekeyUpdate {
    /// Mesh object name
    pub name: String,
    /// Name of the shape key to change, as reported by shapekey_list
    pub key_name: String,
    /// Blend weight, clamped by Blender to [slider_min, slider_max]. Unchanged if
    /// omitted; Blender's own default range is 0-1
    pub value: Option<f64>,
    /// Lower end of the allowed `value` range. Blender's default is 0.0 and it
    /// will not go below -10. Unchanged if omitted
    pub slider_min: Option<f64>,
    /// Upper end of the allowed `value` range. Blender's default is 1.0.
    /// Unchanged if omitted
    pub slider_max: Option<f64>,
    /// true disables the key without removing it. Unchanged if omitted
    pub mute: Option<bool>,
}

crate::tool!(
    "shapekey_update",
    "Set a shape key's blend value, slider range or mute flag. Read the current ones \
     with shapekey_list; to delete a key use shapekey_remove",
    ShapekeyUpdate,
    |py: Python<'_>, p: ShapekeyUpdate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        let shape_keys = if obj.is_none() {
            None
        } else {
            let sk = obj.getattr("data")?.getattr("shape_keys")?;
            if sk.is_none() { None } else { Some(sk) }
        };
        let Some(shape_keys) = shape_keys else {
            return Err(PyValueError::new_err(format!(
                "Object has no shape keys: {}",
                p.name
            )));
        };

        let sk = shape_keys
            .getattr("key_blocks")?
            .call_method1("get", (p.key_name.as_str(),))?;
        if sk.is_none() {
            return Err(PyValueError::new_err(format!(
                "Shape key not found: {}",
                p.key_name
            )));
        }

        if let Some(v) = p.value {
            sk.setattr("value", v)?;
        }
        if let Some(v) = p.slider_min {
            sk.setattr("slider_min", v)?;
        }
        if let Some(v) = p.slider_max {
            sk.setattr("slider_max", v)?;
        }
        if let Some(v) = p.mute {
            sk.setattr("mute", v)?;
        }

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "shape_key": sk.getattr("name")?.extract::<String>()?,
            "value": sk.getattr("value")?.extract::<f64>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ShapekeyList {
    /// Object name. An object with no shape keys is not an error — it reports
    /// has_shape_keys: false and an empty list
    pub name: String,
}

crate::tool!(
    "shapekey_list",
    "List an object's shape keys with their value, slider range and mute flag. \
     Change any of those with shapekey_update",
    ShapekeyList,
    |py: Python<'_>, p: ShapekeyList| {
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
        let name: String = obj.getattr("name")?.extract()?;

        let shape_keys = obj.getattr("data")?.getattr("shape_keys")?;
        if shape_keys.is_none() {
            return Ok(json!({ "name": name, "has_shape_keys": false, "keys": [] }));
        }

        let mut keys = Vec::new();
        for sk in PyIterator::from_object(&shape_keys.getattr("key_blocks")?)? {
            let sk = sk?;
            keys.push(json!({
                "name": sk.getattr("name")?.extract::<String>()?,
                "value": sk.getattr("value")?.extract::<f64>()?,
                "slider_min": sk.getattr("slider_min")?.extract::<f64>()?,
                "slider_max": sk.getattr("slider_max")?.extract::<f64>()?,
                "mute": sk.getattr("mute")?.extract::<bool>()?,
            }));
        }

        Ok(json!({ "name": name, "has_shape_keys": true, "keys": keys }))
    }
);

// ============================================================================
// Instancing
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjInstanceToPoints {
    /// Base object whose geometry supplies the instancing points. Usually a mesh
    pub name: String,
    /// Object to duplicate at each point. It is parented to the base object and
    /// hidden in viewport and render, so only the instances are visible
    pub instance: String,
    /// Where instances are placed. Blender 5.2's `Object.instance_type` accepts
    /// NONE (off), VERTS (one per vertex), FACES (one per face) and COLLECTION
    /// (which needs an instance_collection this tool does not set). Default: VERTS
    pub instance_type: Option<String>,
}

crate::tool!(
    "obj_instance_to_points",
    "Instance one object at every vertex or face of another (Blender's vertex/face \
     duplication). The instances are virtual until obj_make_instances_real turns them \
     into objects",
    ObjInstanceToPoints,
    |py: Python<'_>, p: ObjInstanceToPoints| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let objects = d.getattr("objects")?;

        let obj = objects.call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Object not found: {}",
                p.name
            )));
        }
        let instance_obj = objects.call_method1("get", (p.instance.as_str(),))?;
        if instance_obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Instance object not found: {}",
                p.instance
            )));
        }

        let instance_type = p.instance_type.unwrap_or_else(|| "VERTS".to_string());
        obj.setattr("instance_type", instance_type.as_str())?;
        instance_obj.setattr("parent", &obj)?;
        // NOT `hide_viewport = true`: that property ("Disable in Viewports") excludes the
        // object from the depsgraph entirely, so Blender never generates dupli-instances
        // for it in the first place -- the base object silently shows nothing, and a later
        // `obj_make_instances_real` reports success while creating zero objects. Verified
        // live against Blender 5.0: with `hide_viewport = true` on the child,
        // `duplicates_make_real` returns `{'FINISHED'}` but creates nothing; swapping to
        // `hide_set(true)` (the per-view-layer "eye" hide, independent of dupli generation)
        // keeps the child's own copy hidden while instancing still works.
        instance_obj.call_method1("hide_set", (true,))?;
        instance_obj.setattr("hide_render", true)?;

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "instance": instance_obj.getattr("name")?.extract::<String>()?,
            "type": obj.getattr("instance_type")?.extract::<String>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ObjMakeInstancesReal {
    /// Object carrying the instances, i.e. the base object passed to
    /// obj_instance_to_points
    pub name: String,
}

crate::tool!(
    "obj_make_instances_real",
    "Turn an object's virtual instances into independent scene objects, so each can be \
     edited on its own. Create the instances first with obj_instance_to_points",
    ObjMakeInstancesReal,
    |py: Python<'_>, p: ObjMakeInstancesReal| {
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

        bpy.getattr("context")?
            .getattr("view_layer")?
            .getattr("objects")?
            .setattr("active", &obj)?;
        obj.call_method1("select_set", (true,))?;
        let result = bpy
            .getattr("ops")?
            .getattr("object")?
            .call_method0("duplicates_make_real")?;
        op_ok(&result, &format!("make instances of {} real", p.name))?;

        Ok(json!({ "name": obj.getattr("name")?.extract::<String>()?, "made_real": true }))
    }
);
