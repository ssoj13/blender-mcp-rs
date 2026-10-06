//! Real mesh-topology editing: build a mesh from raw data, select vertices/edges/
//! faces by predicate, run edit operations against that selection, transform it, and
//! mark seams/sharp/crease.
//!
//! ## Why `bmesh`, not `bpy.ops.mesh.*`
//!
//! `bpy.ops.mesh.*` needs Edit Mode and a window/screen context to run at all, is
//! fragile headless, and can only ever act on "whatever happens to be selected in
//! the UI" — it has no way to address a specific vertex by index. `bmesh` works
//! directly on the mesh data (`bmesh.new()` + `bm.from_mesh(mesh)` / `bm.to_mesh
//! (mesh)`), addresses elements by index, and does not care whether there is a
//! screen at all. **Every tool in this file stays in Object Mode from start to
//! finish** — Edit Mode is never entered, so there is nothing to restore.
//!
//! ## Selection persistence (verified, not assumed)
//!
//! `bmesh` select flags live on the mesh data itself and round-trip through
//! `to_mesh`/`from_mesh`: `mesh.vertices/edges/polygons[i].select` still reads back
//! correctly after `bm.to_mesh(mesh)` and `bm.free()`, in a *separate* later call
//! that opens a brand new `bmesh.new()` on the same mesh. Verified live against
//! Blender 5: select a face's 4 verts, flush, write back, free; a fresh `bmesh.new()`
//! reading that same mesh in an unrelated script run still reports the face
//! selected. So `mesh_select` now and `mesh_edit`/`mesh_mark` later, in separate MCP
//! calls, just works — nothing is cached on the Rust side.
//!
//! `bm.select_mode` (a `{'VERT'}`/`{'EDGE'}`/`{'FACE'}` set) plus
//! `bm.select_flush_mode()` reproduce the exact flush behaviour of interactive Edit
//! Mode (verified: selecting all 4 verts of a face in VERT mode flushes up through
//! its edges to the face; selecting a face in FACE mode flushes down to its verts/
//! edges; partial edge selection does not prematurely select a face). Every tool
//! that changes a selection calls `select_flush_mode()` before writing it back, so a
//! later `mesh_edit` op always sees a consistent selection across all three domains.
//!
//! Every `bmesh.ops.*` signature and enum value used below was read from the live
//! Blender 5.0 `bmesh.ops` docstrings / from the error message a deliberately-bad
//! enum value provokes (e.g. `bmesh.ops.delete`'s `context` values), not guessed.

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList, PySet, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, py_len};

// ============================================================================
// Shared helpers
// ============================================================================

/// Look up `bpy.data.objects[name]`, requiring it to be a `MESH` object. A private
/// copy of the equivalent helper in `mesh.rs`: that one is not `pub`, and this file
/// must not touch `mesh.rs`.
fn get_mesh_obj<'py>(d: &Bound<'py, PyAny>, name: &str) -> PyResult<Bound<'py, PyAny>> {
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    let ok = if obj.is_none() {
        false
    } else {
        obj.getattr("type")?.extract::<String>()? == "MESH"
    };
    if !ok {
        return Err(PyValueError::new_err(format!("Mesh not found: {name}")));
    }
    Ok(obj)
}

/// Open a fresh `bmesh` on an object's mesh data, entirely in Object Mode.
fn open_bmesh<'py>(py: Python<'py>, mesh: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    let bm = py.import("bmesh")?.call_method0("new")?;
    bm.call_method1("from_mesh", (mesh,))?;
    Ok(bm)
}

/// Write a `bmesh` back to the object's mesh data and free it. Every handler below
/// calls this exactly once before returning `Ok`, or its edits are silently lost.
fn commit_bmesh(bm: &Bound<'_, PyAny>, mesh: &Bound<'_, PyAny>) -> PyResult<()> {
    bm.call_method1("to_mesh", (mesh,))?;
    mesh.call_method0("update")?;
    bm.call_method0("free")?;
    Ok(())
}

/// Extract exactly 3 floats from a `[x, y, z]` param. A caller-supplied array of the
/// wrong length becomes a clean error here instead of an out-of-bounds panic.
fn vec3(v: &[f64]) -> PyResult<[f64; 3]> {
    match v {
        [x, y, z] => Ok([*x, *y, *z]),
        other => Err(PyValueError::new_err(format!(
            "expected [x, y, z], got {} component(s)",
            other.len()
        ))),
    }
}

/// Extract a Python float sequence (`Vector`, a vert's `co`, a face's `normal`) as a
/// fixed 3-array.
fn py_vec3(seq: &Bound<'_, PyAny>) -> PyResult<[f64; 3]> {
    let mut v = Vec::with_capacity(3);
    for x in seq.try_iter()? {
        v.push(x?.extract::<f64>()?);
    }
    vec3(&v)
}

fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalize3(v: [f64; 3]) -> Option<[f64; 3]> {
    let len = dot3(v, v).sqrt();
    if len < 1e-12 {
        None
    } else {
        Some([v[0] / len, v[1] / len, v[2] / len])
    }
}

fn matvec3(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

/// Cofactor-expansion inverse of a 3x3 matrix. `None` if singular (e.g. an object
/// squashed to zero scale on some axis) — checked against a live Blender comparison
/// (rotate+scale+translate in "global" space, on a rotated *and* non-uniformly
/// scaled object, matched `bmesh.ops` to 6 significant figures).
fn invert3(m: &[[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
    if det.abs() < 1e-12 {
        return None;
    }
    let inv_det = 1.0 / det;
    Some([
        [
            (m[1][1] * m[2][2] - m[1][2] * m[2][1]) * inv_det,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * inv_det,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * inv_det,
        ],
        [
            (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * inv_det,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * inv_det,
            (m[0][2] * m[1][0] - m[0][0] * m[1][2]) * inv_det,
        ],
        [
            (m[1][0] * m[2][1] - m[1][1] * m[2][0]) * inv_det,
            (m[0][1] * m[2][0] - m[0][0] * m[2][1]) * inv_det,
            (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * inv_det,
        ],
    ])
}

/// Rodrigues' rotation formula: the rotation matrix for `angle_rad` around a unit
/// `axis`. Cross-checked live against `mathutils.Matrix.Rotation` combined with
/// `bmesh.ops.rotate` on a rotated+scaled object — identical to float precision.
fn rotation_matrix3(axis: [f64; 3], angle_rad: f64) -> [[f64; 3]; 3] {
    let (s, c) = angle_rad.sin_cos();
    let t = 1.0 - c;
    let [x, y, z] = axis;
    [
        [t * x * x + c, t * x * y - s * z, t * x * z + s * y],
        [t * x * y + s * z, t * y * y + c, t * y * z - s * x],
        [t * x * z - s * y, t * y * z + s * x, t * z * z + c],
    ]
}

/// Move a bmesh element's `co` by a local-space delta.
fn translate_co(py: Python<'_>, v: &Bound<'_, PyAny>, d: [f64; 3]) -> PyResult<()> {
    let co = py_vec3(&v.getattr("co")?)?;
    v.setattr(
        "co",
        PyTuple::new(py, [co[0] + d[0], co[1] + d[1], co[2] + d[2]])?,
    )?;
    Ok(())
}

/// The verts/edges/faces currently selected in a `bmesh`, gathered fresh. Called at
/// the start of every op in a `mesh_edit` batch, since an earlier op in the same
/// batch may have re-pointed the selection at newly created geometry (see
/// `select_only`) — stale references from a previous iteration are never reused.
struct Selected<'py> {
    verts: Vec<Bound<'py, PyAny>>,
    edges: Vec<Bound<'py, PyAny>>,
    faces: Vec<Bound<'py, PyAny>>,
}

fn gather_selected<'py>(bm: &Bound<'py, PyAny>) -> PyResult<Selected<'py>> {
    let mut verts = Vec::new();
    for v in bm.getattr("verts")?.try_iter()? {
        let v = v?;
        if v.getattr("select")?.extract::<bool>()? {
            verts.push(v);
        }
    }
    let mut edges = Vec::new();
    for e in bm.getattr("edges")?.try_iter()? {
        let e = e?;
        if e.getattr("select")?.extract::<bool>()? {
            edges.push(e);
        }
    }
    let mut faces = Vec::new();
    for f in bm.getattr("faces")?.try_iter()? {
        let f = f?;
        if f.getattr("select")?.extract::<bool>()? {
            faces.push(f);
        }
    }
    Ok(Selected {
        verts,
        edges,
        faces,
    })
}

fn deselect_all(bm: &Bound<'_, PyAny>) -> PyResult<()> {
    for domain in ["verts", "edges", "faces"] {
        for el in bm.getattr(domain)?.try_iter()? {
            el?.setattr("select", false)?;
        }
    }
    Ok(())
}

/// Clear the whole selection, then select exactly the elements in `seqs` (each a
/// Python iterable, typically a key straight out of a `bmesh.ops` result dict). This
/// is how every geometry-creating op below hands its result to the *next* op in the
/// same batch, mirroring what an interactive extrude/inset/bevel leaves selected.
fn select_only(bm: &Bound<'_, PyAny>, seqs: &[&Bound<'_, PyAny>]) -> PyResult<()> {
    deselect_all(bm)?;
    for seq in seqs {
        for item in seq.try_iter()? {
            item?.setattr("select", true)?;
        }
    }
    Ok(())
}

fn count_selected(seq: &Bound<'_, PyAny>) -> PyResult<usize> {
    let mut n = 0;
    for el in seq.try_iter()? {
        if el?.getattr("select")?.extract::<bool>()? {
            n += 1;
        }
    }
    Ok(n)
}

fn median_of(verts: &[Bound<'_, PyAny>]) -> PyResult<[f64; 3]> {
    if verts.is_empty() {
        return Err(PyValueError::new_err(
            "no vertices selected; call mesh_select first",
        ));
    }
    let mut sum = [0.0; 3];
    for v in verts {
        let co = py_vec3(&v.getattr("co")?)?;
        sum[0] += co[0];
        sum[1] += co[1];
        sum[2] += co[2];
    }
    let n = verts.len() as f64;
    Ok([sum[0] / n, sum[1] / n, sum[2] / n])
}

fn average_normal(faces: &[Bound<'_, PyAny>]) -> PyResult<[f64; 3]> {
    let mut sum = [0.0; 3];
    for f in faces {
        let n = py_vec3(&f.getattr("normal")?)?;
        sum[0] += n[0];
        sum[1] += n[1];
        sum[2] += n[2];
    }
    normalize3(sum)
        .ok_or_else(|| PyValueError::new_err("selected faces have a zero average normal"))
}

// ============================================================================
// mesh_create
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshCreate {
    /// Name for the new mesh object. Defaults to "Mesh".
    pub name: Option<String>,
    /// Object location [x, y, z]. Defaults to the origin.
    pub location: Option<Vec<f64>>,
    /// Vertex positions, one [x, y, z] per vertex. Vertices are implicitly indexed
    /// 0..N in this order; `edges` and `faces` reference them by that index.
    pub verts: Vec<Vec<f64>>,
    /// Loose edges as [vertex_index_a, vertex_index_b] pairs. Edges implied by a
    /// face's boundary do not need to be listed here too.
    pub edges: Option<Vec<Vec<u32>>>,
    /// Faces as lists of vertex indices, e.g. [0,1,2] for a triangle, [0,1,2,3] for a
    /// quad; ngons (5+) are fine. Winding order determines the face normal.
    pub faces: Option<Vec<Vec<u32>>>,
}

crate::tool!(
    "mesh_create",
    "Build a new mesh object from raw vertex/edge/face lists (mesh.from_pydata). For a \
     cube/sphere/cylinder and the other stock shapes use obj_create_primitive instead; \
     to reshape the result use mesh_select + mesh_edit",
    MeshCreate,
    |py: Python<'_>, p: MeshCreate| {
        if p.verts.is_empty() {
            return Err(PyValueError::new_err(
                "mesh_create: `verts` must not be empty",
            ));
        }
        let nv = p.verts.len();
        for (i, v) in p.verts.iter().enumerate() {
            if v.len() != 3 {
                return Err(PyValueError::new_err(format!(
                    "mesh_create: vertex {i} must be [x, y, z], got {} component(s)",
                    v.len()
                )));
            }
        }
        let edges = p.edges.unwrap_or_default();
        for (i, e) in edges.iter().enumerate() {
            if e.len() != 2 {
                return Err(PyValueError::new_err(format!(
                    "mesh_create: edge {i} must have exactly 2 indices, got {}",
                    e.len()
                )));
            }
            for &idx in e {
                if idx as usize >= nv {
                    return Err(PyValueError::new_err(format!(
                        "mesh_create: edge {i} references vertex index {idx}, but only {nv} \
                         vertices were given"
                    )));
                }
            }
        }
        let faces = p.faces.unwrap_or_default();
        for (i, f) in faces.iter().enumerate() {
            if f.len() < 3 {
                return Err(PyValueError::new_err(format!(
                    "mesh_create: face {i} has {} vertices, needs at least 3",
                    f.len()
                )));
            }
            for &idx in f {
                if idx as usize >= nv {
                    return Err(PyValueError::new_err(format!(
                        "mesh_create: face {i} references vertex index {idx}, but only {nv} \
                         vertices were given"
                    )));
                }
            }
        }

        let bpy = py.import("bpy")?;
        let name = p.name.unwrap_or_else(|| "Mesh".to_string());
        let loc = p.location.unwrap_or_else(|| vec![0.0, 0.0, 0.0]);

        let mut vert_items = Vec::with_capacity(nv);
        for v in &p.verts {
            vert_items.push(PyTuple::new(py, [v[0], v[1], v[2]])?);
        }
        let verts_py = PyList::new(py, vert_items)?;

        let mut edge_items = Vec::with_capacity(edges.len());
        for e in &edges {
            edge_items.push(PyTuple::new(py, [e[0], e[1]])?);
        }
        let edges_py = PyList::new(py, edge_items)?;

        let mut face_items = Vec::with_capacity(faces.len());
        for f in &faces {
            face_items.push(PyTuple::new(py, f.iter().copied())?);
        }
        let faces_py = PyList::new(py, face_items)?;

        let mesh_data = data(&bpy)?
            .getattr("meshes")?
            .call_method1("new", (name.as_str(),))?;
        mesh_data.call_method1("from_pydata", (verts_py, edges_py, faces_py))?;

        // `from_pydata` itself does not raise on bad topology it can't fix (index
        // bounds are already guarded above); `validate()` catches everything else
        // (degenerate/duplicate geometry) and reports whether it had to remove any.
        let vkwargs = PyDict::new(py);
        vkwargs.set_item("verbose", false)?;
        let had_invalid_geometry: bool = mesh_data
            .call_method("validate", (), Some(&vkwargs))?
            .extract()?;
        mesh_data.call_method0("update")?;

        let obj = data(&bpy)?
            .getattr("objects")?
            .call_method1("new", (name.as_str(), &mesh_data))?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&obj,))?;
        obj.setattr("location", PyTuple::new(py, loc)?)?;

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "vertices": py_len(py, &mesh_data.getattr("vertices")?)?,
            "edges": py_len(py, &mesh_data.getattr("edges")?)?,
            "faces": py_len(py, &mesh_data.getattr("polygons")?)?,
            "had_invalid_geometry": had_invalid_geometry,
        }))
    }
);

// ============================================================================
// mesh_select
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshSelect {
    /// Mesh object to select on.
    pub name: String,
    /// Selection domain the indices/predicate below address. Exactly one of VERT,
    /// EDGE or FACE (case-insensitive). The selection is flushed across all three
    /// domains afterwards, the same way interactive Edit Mode does it.
    pub mode: String,
    /// Add to the current selection instead of replacing it. Ignored for
    /// `by: "invert"`, which always operates on the current selection regardless.
    /// Default: false (replace -- clears every domain first).
    pub extend: Option<bool>,
    /// Explicit element indices to select, in `mode`'s domain. Takes priority over
    /// `by` when given.
    pub indices: Option<Vec<u32>>,
    /// Predicate used when `indices` is not given. One of: "all", "none", "invert",
    /// "material_index" (FACE only), "normal" (FACE only), "bbox" (any domain),
    /// "non_manifold" (VERT/EDGE only), "boundary" (VERT/EDGE only), "loose"
    /// (VERT/EDGE only -- elements touching no face at all).
    pub by: Option<String>,
    /// by="material_index": select faces whose `material_index` equals this slot.
    pub material_index: Option<i32>,
    /// by="normal": axis to compare face normals against, e.g. [0,0,1] for "up".
    pub normal_axis: Option<Vec<f64>>,
    /// by="normal": max angle in degrees between a face's normal and `normal_axis`.
    /// Defaults to 45.
    pub normal_angle: Option<f64>,
    /// by="bbox": inclusive min corner [x,y,z], local mesh space. An element is
    /// selected only if ALL of its vertices fall inside [bbox_min, bbox_max].
    pub bbox_min: Option<Vec<f64>>,
    /// by="bbox": inclusive max corner [x,y,z], local mesh space.
    pub bbox_max: Option<Vec<f64>>,
}

crate::tool!(
    "mesh_select",
    "Select vertices/edges/faces on a mesh, by explicit index or by predicate (all, \
     none, invert, material index, normal direction, bounding box, non-manifold, \
     boundary, loose). The selection is stored on the mesh data and survives across \
     calls: select now, edit with mesh_edit, mesh_transform_selection or mesh_mark in a \
     later call. Note mesh_subdivide and mesh_unwrap select everything and destroy it.",
    MeshSelect,
    |py: Python<'_>, p: MeshSelect| {
        let mode = p.mode.to_uppercase();
        if !matches!(mode.as_str(), "VERT" | "EDGE" | "FACE") {
            return Err(PyValueError::new_err(format!(
                "mesh_select: mode must be VERT, EDGE or FACE, got `{mode}`"
            )));
        }

        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.name)?;
        let mesh = obj.getattr("data")?;
        let bm = open_bmesh(py, &mesh)?;
        bm.setattr("select_mode", PySet::new(py, [mode.as_str()])?)?;

        let verts_seq = bm.getattr("verts")?;
        let edges_seq = bm.getattr("edges")?;
        let faces_seq = bm.getattr("faces")?;
        verts_seq.call_method0("ensure_lookup_table")?;
        edges_seq.call_method0("ensure_lookup_table")?;
        faces_seq.call_method0("ensure_lookup_table")?;

        let extend = p.extend.unwrap_or(false);
        let by = p.by.as_deref().unwrap_or("").to_lowercase();

        if !extend && by != "invert" {
            deselect_all(&bm)?;
        }

        let domain: &Bound<'_, PyAny> = match mode.as_str() {
            "VERT" => &verts_seq,
            "EDGE" => &edges_seq,
            _ => &faces_seq,
        };

        if let Some(indices) = &p.indices {
            let domain_len = py_len(py, domain)?;
            for &idx in indices {
                if idx as usize >= domain_len {
                    return Err(PyValueError::new_err(format!(
                        "mesh_select: {mode} index {idx} out of range (mesh has {domain_len})"
                    )));
                }
                domain.get_item(idx)?.setattr("select", true)?;
            }
        } else {
            match by.as_str() {
                "all" => {
                    for el in domain.try_iter()? {
                        el?.setattr("select", true)?;
                    }
                }
                "none" => {
                    for el in domain.try_iter()? {
                        el?.setattr("select", false)?;
                    }
                }
                "invert" => {
                    for el in domain.try_iter()? {
                        let el = el?;
                        let cur: bool = el.getattr("select")?.extract()?;
                        el.setattr("select", !cur)?;
                    }
                }
                "material_index" => {
                    if mode != "FACE" {
                        return Err(PyValueError::new_err(
                            "mesh_select: by=material_index requires mode=FACE",
                        ));
                    }
                    let want = p.material_index.ok_or_else(|| {
                        PyValueError::new_err(
                            "mesh_select: by=material_index needs `material_index`",
                        )
                    })?;
                    for f in faces_seq.try_iter()? {
                        let f = f?;
                        let mi: i32 = f.getattr("material_index")?.extract()?;
                        if mi == want {
                            f.setattr("select", true)?;
                        }
                    }
                }
                "normal" => {
                    if mode != "FACE" {
                        return Err(PyValueError::new_err(
                            "mesh_select: by=normal requires mode=FACE",
                        ));
                    }
                    let axis = p.normal_axis.as_deref().ok_or_else(|| {
                        PyValueError::new_err("mesh_select: by=normal needs `normal_axis`")
                    })?;
                    let axis_n = normalize3(vec3(axis)?).ok_or_else(|| {
                        PyValueError::new_err("mesh_select: normal_axis must not be zero-length")
                    })?;
                    let max_angle = p.normal_angle.unwrap_or(45.0).to_radians();
                    for f in faces_seq.try_iter()? {
                        let f = f?;
                        let n = py_vec3(&f.getattr("normal")?)?;
                        if let Some(n) = normalize3(n) {
                            let cos = dot3(n, axis_n).clamp(-1.0, 1.0);
                            if cos.acos() <= max_angle {
                                f.setattr("select", true)?;
                            }
                        }
                    }
                }
                "bbox" => {
                    let bmin = vec3(p.bbox_min.as_deref().ok_or_else(|| {
                        PyValueError::new_err("mesh_select: by=bbox needs bbox_min and bbox_max")
                    })?)?;
                    let bmax = vec3(p.bbox_max.as_deref().ok_or_else(|| {
                        PyValueError::new_err("mesh_select: by=bbox needs bbox_min and bbox_max")
                    })?)?;
                    let in_box =
                        |co: [f64; 3]| (0..3).all(|i| co[i] >= bmin[i] && co[i] <= bmax[i]);
                    for el in domain.try_iter()? {
                        let el = el?;
                        let ok = if mode == "VERT" {
                            in_box(py_vec3(&el.getattr("co")?)?)
                        } else {
                            let mut all_in = true;
                            for v in el.getattr("verts")?.try_iter()? {
                                if !in_box(py_vec3(&v?.getattr("co")?)?) {
                                    all_in = false;
                                    break;
                                }
                            }
                            all_in
                        };
                        if ok {
                            el.setattr("select", true)?;
                        }
                    }
                }
                "non_manifold" => {
                    if mode == "FACE" {
                        return Err(PyValueError::new_err(
                            "mesh_select: by=non_manifold requires mode=VERT or EDGE",
                        ));
                    }
                    for el in domain.try_iter()? {
                        let el = el?;
                        if !el.getattr("is_manifold")?.extract::<bool>()? {
                            el.setattr("select", true)?;
                        }
                    }
                }
                "boundary" => {
                    if mode == "FACE" {
                        return Err(PyValueError::new_err(
                            "mesh_select: by=boundary requires mode=VERT or EDGE",
                        ));
                    }
                    for el in domain.try_iter()? {
                        let el = el?;
                        if el.getattr("is_boundary")?.extract::<bool>()? {
                            el.setattr("select", true)?;
                        }
                    }
                }
                "loose" => {
                    if mode == "FACE" {
                        return Err(PyValueError::new_err(
                            "mesh_select: by=loose requires mode=VERT or EDGE",
                        ));
                    }
                    for el in domain.try_iter()? {
                        let el = el?;
                        let is_loose = if mode == "VERT" {
                            py_len(py, &el.getattr("link_faces")?)? == 0
                        } else {
                            el.getattr("is_wire")?.extract::<bool>()?
                        };
                        if is_loose {
                            el.setattr("select", true)?;
                        }
                    }
                }
                "" => {
                    return Err(PyValueError::new_err(
                        "mesh_select: give either `indices` or `by`",
                    ));
                }
                other => {
                    return Err(PyValueError::new_err(format!(
                        "mesh_select: unknown `by`: {other}"
                    )));
                }
            }
        }

        bm.call_method0("select_flush_mode")?;

        let selected_verts = count_selected(&verts_seq)?;
        let selected_edges = count_selected(&edges_seq)?;
        let selected_faces = count_selected(&faces_seq)?;

        commit_bmesh(&bm, &mesh)?;

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "selected_verts": selected_verts,
            "selected_edges": selected_edges,
            "selected_faces": selected_faces,
        }))
    }
);

// ============================================================================
// mesh_edit
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshEditOp {
    /// One of: extrude, inset, bevel, subdivide, merge, delete, duplicate, spin,
    /// bridge, triangulate, recalc_normals, flip_normals, dissolve.
    pub op: String,

    /// extrude: exactly one of "region" (default — extrudes the selection as one
    /// connected patch) or "individual" (extrudes each selected face along its own
    /// normal separately).
    pub extrude_mode: Option<String>,
    /// extrude: a literal displacement vector [x,y,z] (local space), applied as-is.
    /// spin: the rotation axis [x,y,z] (default [0,0,1]).
    pub vector: Option<Vec<f64>>,
    /// extrude: distance to move along the average normal of the selected faces
    /// (extrude_mode="region") or each face's own normal (extrude_mode="individual").
    /// Used only when `vector` is not given.
    pub distance: Option<f64>,

    /// inset/bevel: offset width in local mesh units. Default 0 (a no-op, so this
    /// is effectively required for both).
    pub thickness: Option<f64>,
    /// inset: push the new faces in (negative) or out (positive) along the normal.
    /// Default 0.
    pub depth: Option<f64>,
    /// inset: true = inset each selected face on its own instead of as one region.
    /// Default false.
    pub individual: Option<bool>,
    /// bevel: which kind of selected element is beveled. "edges" (default, also
    /// accepts "edge") or "verts" (also "vert"/"vertices").
    pub affect: Option<String>,
    /// bevel: number of segments across the bevel. Default 1.
    pub segments: Option<i32>,

    /// subdivide: number of cuts per edge. Default 1.
    pub cuts: Option<i32>,
    /// subdivide: 0..1 smoothing factor pulling new verts toward a sphere. Default 0
    /// (straight cuts).
    pub smoothness: Option<f64>,

    /// merge: weld selected vertices closer than this local-space distance. Default
    /// 0.0001 (Blender's own "Merge by Distance" default).
    pub merge_distance: Option<f64>,

    /// delete: which domain to remove, and how much goes with it. Exactly one of
    /// "verts" (and every edge/face using them), "edges" (and every face using
    /// them), "faces" (and any edge/vert left orphaned), or "only_faces" (remove
    /// the faces but keep their edges and verts). Required for op="delete".
    pub delete_mode: Option<String>,

    /// spin: pivot point [x,y,z], local space. Defaults to the selection's median.
    pub center: Option<Vec<f64>>,
    /// spin: total sweep angle in degrees. Default 360.
    pub angle: Option<f64>,
    /// spin: number of steps around the sweep. Default 12.
    pub steps: Option<i32>,
    /// spin: keep the original geometry in place and sweep a copy outward, instead
    /// of moving the selection itself. Default false.
    pub duplicate: Option<bool>,

    /// dissolve: exactly one of "verts", "edges", "faces" (remove the element and
    /// merge the geometry around it, unlike delete which leaves a hole), or
    /// "limited" (angle-based cleanup of near-coplanar geometry using
    /// `angle_limit`). Required for op="dissolve".
    pub dissolve_mode: Option<String>,
    /// dissolve="limited": max angle in degrees between adjacent faces to still
    /// dissolve the edge/vertex between them. Default 5.
    pub angle_limit: Option<f64>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshEdit {
    /// Mesh object to edit.
    pub name: String,
    /// Operations to run in order. Each op acts on the selection left by the
    /// previous one -- an op that creates geometry (extrude, inset, bevel,
    /// subdivide, duplicate, spin, bridge, triangulate, dissolve) re-points the
    /// selection at what it created, mirroring interactive Blender.
    pub ops: Vec<MeshEditOp>,
}

crate::tool!(
    "mesh_edit",
    "Run a batch of bmesh edit operations (extrude, inset, bevel, subdivide, merge, \
     delete, duplicate, spin, bridge, triangulate, recalc_normals, flip_normals, \
     dissolve) against a mesh object's current selection, in order, each op acting on \
     what the previous one created. Build the selection first with mesh_select; \
     mesh_subdivide and mesh_extrude are whole-mesh shortcuts for the two simplest cases",
    MeshEdit,
    |py: Python<'_>, p: MeshEdit| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.name)?;
        let mesh = obj.getattr("data")?;
        let bm = open_bmesh(py, &mesh)?;

        let mut results = Vec::with_capacity(p.ops.len());
        for (i, op) in p.ops.iter().enumerate() {
            let r = run_edit_op(py, &bm, op).map_err(|e| {
                PyValueError::new_err(format!("mesh_edit: op {i} (`{}`) failed: {e}", op.op))
            })?;
            results.push(r);
        }

        bm.call_method0("select_flush_mode")?;
        commit_bmesh(&bm, &mesh)?;

        Ok(json!({ "name": obj.getattr("name")?.extract::<String>()?, "ops": results }))
    }
);

/// Run one `MeshEditOp` against `bm`'s current selection. Every `bmesh.ops` call
/// here, its keyword names and its enum literals were read from the live Blender 5
/// `bmesh.ops` docstrings (or the error a bad enum value provokes), not guessed.
fn run_edit_op(py: Python<'_>, bm: &Bound<'_, PyAny>, op: &MeshEditOp) -> PyResult<Value> {
    let bmesh_ops = py.import("bmesh")?.getattr("ops")?;
    let sel = gather_selected(bm)?;

    let result = match op.op.to_lowercase().as_str() {
        "extrude" => {
            if sel.faces.is_empty() {
                return Err(PyValueError::new_err("extrude: no faces selected"));
            }
            let mode = op
                .extrude_mode
                .as_deref()
                .unwrap_or("region")
                .to_lowercase();

            if mode == "individual" {
                let faces_py = PyList::new(py, sel.faces.iter().cloned())?;
                let kwargs = PyDict::new(py);
                kwargs.set_item("faces", faces_py)?;
                let out = bmesh_ops.call_method("extrude_discrete_faces", (bm,), Some(&kwargs))?;
                let new_faces = out.get_item("faces")?;

                let uniform = match &op.vector {
                    Some(v) => Some(vec3(v)?),
                    None => None,
                };
                if uniform.is_none() && op.distance.is_none() {
                    return Err(PyValueError::new_err(
                        "extrude: give `vector` or `distance`",
                    ));
                }

                let mut moved = 0usize;
                for f in new_faces.try_iter()? {
                    let f = f?;
                    let d = if let Some(u) = uniform {
                        u
                    } else {
                        let n = normalize3(py_vec3(&f.getattr("normal")?)?).ok_or_else(|| {
                            PyValueError::new_err("extrude: a face has a zero-length normal")
                        })?;
                        // No `expect` here: a panic inside Blender takes Blender down,
                        // not just the tool.
                        let Some(dist) = op.distance else {
                            return Err(PyValueError::new_err(
                                "extrude along the normal needs `distance`",
                            ));
                        };
                        [n[0] * dist, n[1] * dist, n[2] * dist]
                    };
                    for v in f.getattr("verts")?.try_iter()? {
                        translate_co(py, &v?, d)?;
                        moved += 1;
                    }
                }
                select_only(bm, &[&new_faces])?;
                json!({ "op": "extrude", "mode": "individual", "new_faces": py_len(py, &new_faces)?, "verts_moved": moved })
            } else {
                // The average normal must be read *before* running the op:
                // `extrude_face_region` deletes the original selected faces (verified
                // live -- the input face's `is_valid` goes False), so touching
                // `sel.faces` afterward raises `ReferenceError: BMesh data ... has
                // been removed` instead of a clean Rust error.
                let dir = match &op.vector {
                    Some(v) => Some(vec3(v)?),
                    None => None,
                };
                let dir = match dir {
                    Some(v) => v,
                    None => {
                        let dist = op.distance.ok_or_else(|| {
                            PyValueError::new_err("extrude: give `vector` or `distance`")
                        })?;
                        let n = average_normal(&sel.faces)?;
                        [n[0] * dist, n[1] * dist, n[2] * dist]
                    }
                };

                let geom: Vec<Bound<'_, PyAny>> = sel
                    .faces
                    .iter()
                    .cloned()
                    .chain(sel.edges.iter().cloned())
                    .chain(sel.verts.iter().cloned())
                    .collect();
                let geom_py = PyList::new(py, geom)?;
                let kwargs = PyDict::new(py);
                kwargs.set_item("geom", geom_py)?;
                let out = bmesh_ops.call_method("extrude_face_region", (bm,), Some(&kwargs))?;
                let new_geom = out.get_item("geom")?;

                let bmvert_ty = py.import("bmesh")?.getattr("types")?.getattr("BMVert")?;
                let mut moved = 0usize;
                for item in new_geom.try_iter()? {
                    let item = item?;
                    if item.is_instance(&bmvert_ty)? {
                        translate_co(py, &item, dir)?;
                        moved += 1;
                    }
                }
                select_only(bm, &[&new_geom])?;
                json!({ "op": "extrude", "mode": "region", "verts_moved": moved })
            }
        }

        "inset" => {
            if sel.faces.is_empty() {
                return Err(PyValueError::new_err("inset: no faces selected"));
            }
            let individual = op.individual.unwrap_or(false);
            let faces_py = PyList::new(py, sel.faces.iter().cloned())?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("faces", faces_py)?;
            kwargs.set_item("thickness", op.thickness.unwrap_or(0.0))?;
            kwargs.set_item("depth", op.depth.unwrap_or(0.0))?;
            let fname = if individual {
                "inset_individual"
            } else {
                "inset_region"
            };
            let out = bmesh_ops.call_method(fname, (bm,), Some(&kwargs))?;
            let new_faces = out.get_item("faces")?;
            select_only(bm, &[&new_faces])?;
            json!({ "op": "inset", "individual": individual, "new_faces": py_len(py, &new_faces)? })
        }

        "bevel" => {
            let affect_in = op.affect.as_deref().unwrap_or("edges").to_lowercase();
            let (affect_lit, geom): (&str, &[Bound<'_, PyAny>]) = match affect_in.as_str() {
                "verts" | "vert" | "vertices" => ("VERTICES", &sel.verts),
                "edges" | "edge" => ("EDGES", &sel.edges),
                other => {
                    return Err(PyValueError::new_err(format!(
                        "bevel: affect must be verts or edges, got `{other}`"
                    )));
                }
            };
            if geom.is_empty() {
                return Err(PyValueError::new_err("bevel: nothing selected to bevel"));
            }
            let geom_py = PyList::new(py, geom.iter().cloned())?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("geom", geom_py)?;
            kwargs.set_item("offset", op.thickness.unwrap_or(0.0))?;
            kwargs.set_item("segments", op.segments.unwrap_or(1))?;
            kwargs.set_item("affect", affect_lit)?;
            let out = bmesh_ops.call_method("bevel", (bm,), Some(&kwargs))?;
            let new_faces = out.get_item("faces")?;
            let new_edges = out.get_item("edges")?;
            let new_verts = out.get_item("verts")?;
            select_only(bm, &[&new_faces, &new_edges, &new_verts])?;
            json!({
                "op": "bevel",
                "affect": affect_lit,
                "new_faces": py_len(py, &new_faces)?,
                "new_edges": py_len(py, &new_edges)?,
            })
        }

        "subdivide" => {
            if sel.edges.is_empty() {
                return Err(PyValueError::new_err(
                    "subdivide: no edges selected (select edges, or verts/faces whose \
                     edges are all selected)",
                ));
            }
            let cuts = op.cuts.unwrap_or(1).max(1);
            let edges_py = PyList::new(py, sel.edges.iter().cloned())?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("edges", edges_py)?;
            kwargs.set_item("cuts", cuts)?;
            kwargs.set_item("smooth", op.smoothness.unwrap_or(0.0))?;
            kwargs.set_item("use_grid_fill", true)?;
            let out = bmesh_ops.call_method("subdivide_edges", (bm,), Some(&kwargs))?;
            let geom = out.get_item("geom")?;
            select_only(bm, &[&geom])?;
            json!({ "op": "subdivide", "cuts": cuts })
        }

        "merge" => {
            if sel.verts.is_empty() {
                return Err(PyValueError::new_err("merge: no vertices selected"));
            }
            let before = py_len(py, &bm.getattr("verts")?)?;
            let verts_py = PyList::new(py, sel.verts.iter().cloned())?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("verts", verts_py)?;
            kwargs.set_item("dist", op.merge_distance.unwrap_or(0.0001))?;
            bmesh_ops.call_method("remove_doubles", (bm,), Some(&kwargs))?;
            let after = py_len(py, &bm.getattr("verts")?)?;
            json!({ "op": "merge", "vertices_removed": before.saturating_sub(after) })
        }

        "delete" => {
            let mode_in = op
                .delete_mode
                .as_deref()
                .ok_or_else(|| PyValueError::new_err("delete: `delete_mode` is required"))?
                .to_lowercase();
            let (context, geom): (&str, &[Bound<'_, PyAny>]) = match mode_in.as_str() {
                "verts" => ("VERTS", &sel.verts),
                "edges" => ("EDGES", &sel.edges),
                "faces" => ("FACES", &sel.faces),
                "only_faces" => ("FACES_ONLY", &sel.faces),
                other => {
                    return Err(PyValueError::new_err(format!(
                        "delete: delete_mode must be verts, edges, faces or only_faces, got `{other}`"
                    )));
                }
            };
            if geom.is_empty() {
                return Err(PyValueError::new_err(format!(
                    "delete: nothing selected in the `{mode_in}` domain"
                )));
            }
            let n = geom.len();
            let geom_py = PyList::new(py, geom.iter().cloned())?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("geom", geom_py)?;
            kwargs.set_item("context", context)?;
            bmesh_ops.call_method("delete", (bm,), Some(&kwargs))?;
            json!({ "op": "delete", "delete_mode": mode_in, "deleted": n })
        }

        "duplicate" => {
            let all: Vec<Bound<'_, PyAny>> = sel
                .verts
                .iter()
                .cloned()
                .chain(sel.edges.iter().cloned())
                .chain(sel.faces.iter().cloned())
                .collect();
            if all.is_empty() {
                return Err(PyValueError::new_err("duplicate: nothing selected"));
            }
            let geom_py = PyList::new(py, all)?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("geom", geom_py)?;
            let out = bmesh_ops.call_method("duplicate", (bm,), Some(&kwargs))?;
            let new_geom = out.get_item("geom")?;
            select_only(bm, &[&new_geom])?;
            json!({ "op": "duplicate", "new_elements": py_len(py, &new_geom)? })
        }

        "spin" => {
            let all: Vec<Bound<'_, PyAny>> = sel
                .verts
                .iter()
                .cloned()
                .chain(sel.edges.iter().cloned())
                .chain(sel.faces.iter().cloned())
                .collect();
            if all.is_empty() {
                return Err(PyValueError::new_err("spin: nothing selected"));
            }
            let center = match &op.center {
                Some(c) => vec3(c)?,
                None => median_of(&sel.verts)?,
            };
            let axis = match &op.vector {
                Some(v) => normalize3(vec3(v)?).ok_or_else(|| {
                    PyValueError::new_err("spin: `vector` (axis) must not be zero")
                })?,
                None => [0.0, 0.0, 1.0],
            };
            let angle = op.angle.unwrap_or(360.0).to_radians();
            let steps = op.steps.unwrap_or(12).max(1);
            let duplicate = op.duplicate.unwrap_or(false);

            let mathutils = py.import("mathutils")?;
            let vector_ty = mathutils.getattr("Vector")?;
            let matrix_ty = mathutils.getattr("Matrix")?;
            let cent_py = vector_ty.call1((PyTuple::new(py, center)?,))?;
            let axis_py = vector_ty.call1((PyTuple::new(py, axis)?,))?;
            let dvec_py = vector_ty.call1((PyTuple::new(py, [0.0, 0.0, 0.0])?,))?;
            let space_py = matrix_ty.call_method1("Identity", (4,))?;

            let geom_py = PyList::new(py, all)?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("geom", geom_py)?;
            kwargs.set_item("cent", cent_py)?;
            kwargs.set_item("axis", axis_py)?;
            kwargs.set_item("dvec", dvec_py)?;
            kwargs.set_item("angle", angle)?;
            kwargs.set_item("space", space_py)?;
            kwargs.set_item("steps", steps)?;
            kwargs.set_item("use_duplicate", duplicate)?;
            let out = bmesh_ops.call_method("spin", (bm,), Some(&kwargs))?;
            let geom_last = out.get_item("geom_last")?;
            select_only(bm, &[&geom_last])?;
            json!({ "op": "spin", "steps": steps, "angle_degrees": op.angle.unwrap_or(360.0) })
        }

        "bridge" => {
            if sel.edges.is_empty() {
                return Err(PyValueError::new_err(
                    "bridge: no edges selected (select two boundary edge loops)",
                ));
            }
            let edges_py = PyList::new(py, sel.edges.iter().cloned())?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("edges", edges_py)?;
            let out = bmesh_ops.call_method("bridge_loops", (bm,), Some(&kwargs))?;
            let new_faces = out.get_item("faces")?;
            let new_edges = out.get_item("edges")?;
            select_only(bm, &[&new_faces, &new_edges])?;
            json!({ "op": "bridge", "new_faces": py_len(py, &new_faces)? })
        }

        "triangulate" => {
            if sel.faces.is_empty() {
                return Err(PyValueError::new_err("triangulate: no faces selected"));
            }
            let faces_py = PyList::new(py, sel.faces.iter().cloned())?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("faces", faces_py)?;
            let out = bmesh_ops.call_method("triangulate", (bm,), Some(&kwargs))?;
            let new_faces = out.get_item("faces")?;
            select_only(bm, &[&new_faces])?;
            json!({ "op": "triangulate", "faces": py_len(py, &new_faces)? })
        }

        "recalc_normals" => {
            if sel.faces.is_empty() {
                return Err(PyValueError::new_err("recalc_normals: no faces selected"));
            }
            let faces_py = PyList::new(py, sel.faces.iter().cloned())?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("faces", faces_py)?;
            bmesh_ops.call_method("recalc_face_normals", (bm,), Some(&kwargs))?;
            json!({ "op": "recalc_normals", "faces": sel.faces.len() })
        }

        "flip_normals" => {
            if sel.faces.is_empty() {
                return Err(PyValueError::new_err("flip_normals: no faces selected"));
            }
            let faces_py = PyList::new(py, sel.faces.iter().cloned())?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("faces", faces_py)?;
            bmesh_ops.call_method("reverse_faces", (bm,), Some(&kwargs))?;
            json!({ "op": "flip_normals", "faces": sel.faces.len() })
        }

        "dissolve" => {
            let mode_in = op
                .dissolve_mode
                .as_deref()
                .ok_or_else(|| PyValueError::new_err("dissolve: `dissolve_mode` is required"))?
                .to_lowercase();
            match mode_in.as_str() {
                "verts" => {
                    if sel.verts.is_empty() {
                        return Err(PyValueError::new_err("dissolve: no vertices selected"));
                    }
                    let verts_py = PyList::new(py, sel.verts.iter().cloned())?;
                    let kwargs = PyDict::new(py);
                    kwargs.set_item("verts", verts_py)?;
                    bmesh_ops.call_method("dissolve_verts", (bm,), Some(&kwargs))?;
                }
                "edges" => {
                    if sel.edges.is_empty() {
                        return Err(PyValueError::new_err("dissolve: no edges selected"));
                    }
                    let edges_py = PyList::new(py, sel.edges.iter().cloned())?;
                    let kwargs = PyDict::new(py);
                    kwargs.set_item("edges", edges_py)?;
                    let out = bmesh_ops.call_method("dissolve_edges", (bm,), Some(&kwargs))?;
                    select_only(bm, &[&out.get_item("region")?])?;
                }
                "faces" => {
                    if sel.faces.is_empty() {
                        return Err(PyValueError::new_err("dissolve: no faces selected"));
                    }
                    let faces_py = PyList::new(py, sel.faces.iter().cloned())?;
                    let kwargs = PyDict::new(py);
                    kwargs.set_item("faces", faces_py)?;
                    let out = bmesh_ops.call_method("dissolve_faces", (bm,), Some(&kwargs))?;
                    select_only(bm, &[&out.get_item("region")?])?;
                }
                "limited" => {
                    if sel.verts.is_empty() && sel.edges.is_empty() {
                        return Err(PyValueError::new_err(
                            "dissolve: limited needs selected verts and/or edges",
                        ));
                    }
                    let verts_py = PyList::new(py, sel.verts.iter().cloned())?;
                    let edges_py = PyList::new(py, sel.edges.iter().cloned())?;
                    let kwargs = PyDict::new(py);
                    kwargs.set_item("verts", verts_py)?;
                    kwargs.set_item("edges", edges_py)?;
                    kwargs.set_item("angle_limit", op.angle_limit.unwrap_or(5.0).to_radians())?;
                    let out = bmesh_ops.call_method("dissolve_limit", (bm,), Some(&kwargs))?;
                    select_only(bm, &[&out.get_item("region")?])?;
                }
                other => {
                    return Err(PyValueError::new_err(format!(
                        "dissolve: dissolve_mode must be verts, edges, faces or limited, got `{other}`"
                    )));
                }
            }
            json!({ "op": "dissolve", "dissolve_mode": mode_in })
        }

        other => return Err(PyValueError::new_err(format!("unknown op `{other}`"))),
    };

    Ok(result)
}

// ============================================================================
// mesh_transform_selection
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshTransformSelection {
    /// Mesh object to transform.
    pub name: String,
    /// Either "local" (default) or "global", case-insensitive.
    /// "local": `translate`/`rotate_axis`/`scale` reference the object's
    /// own mesh-local axes. "global": they are given in world space and converted
    /// into local space using the object's `matrix_world`, so e.g. `rotate_axis:
    /// [0,0,1]` always means "world up" regardless of the object's own rotation.
    pub space: Option<String>,
    /// Move the selection by this [x,y,z] vector (in `space`). Pivot-independent.
    pub translate: Option<Vec<f64>>,
    /// Rotate the selection around `rotate_axis` by this many DEGREES (positive =
    /// counter-clockwise looking down the axis). Requires `rotate_axis`.
    pub rotate_angle: Option<f64>,
    /// Rotation axis [x,y,z] (in `space`). Required together with `rotate_angle`.
    pub rotate_axis: Option<Vec<f64>>,
    /// Scale the selection by [x,y,z] factors (in `space`), relative to `pivot`.
    pub scale: Option<Vec<f64>>,
    /// Pivot point [x,y,z] in local mesh coordinates, used by rotate and scale.
    /// Defaults to the median of the selected vertices.
    pub pivot: Option<Vec<f64>>,
}

crate::tool!(
    "mesh_transform_selection",
    "Translate/rotate/scale a mesh object's current selection (from mesh_select), in \
     local or global space, optionally around a pivot. This moves existing vertices \
     only; to add geometry use mesh_edit, to move the whole object use obj_update",
    MeshTransformSelection,
    |py: Python<'_>, p: MeshTransformSelection| {
        if p.translate.is_none() && p.rotate_angle.is_none() && p.scale.is_none() {
            return Err(PyValueError::new_err(
                "mesh_transform_selection: give at least one of translate, \
                 rotate_angle (+rotate_axis), scale",
            ));
        }
        let global = matches!(p.space.as_deref(), Some(s) if s.eq_ignore_ascii_case("global"));

        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.name)?;
        let mesh = obj.getattr("data")?;
        let bm = open_bmesh(py, &mesh)?;

        let verts_seq = bm.getattr("verts")?;
        let mut selected = Vec::new();
        for v in verts_seq.try_iter()? {
            let v = v?;
            if v.getattr("select")?.extract::<bool>()? {
                selected.push(v);
            }
        }
        if selected.is_empty() {
            return Err(PyValueError::new_err(
                "mesh_transform_selection: no vertices selected; call mesh_select first",
            ));
        }

        let mut positions: Vec<[f64; 3]> = Vec::with_capacity(selected.len());
        for v in &selected {
            positions.push(py_vec3(&v.getattr("co")?)?);
        }

        let pivot = match &p.pivot {
            Some(pv) => vec3(pv)?,
            None => median_of(&selected)?,
        };

        // Linear (rotation+scale, no translation) part of the object's world matrix,
        // needed only in "global" mode to reinterpret world-space direction vectors
        // as local ones. Verified against bmesh.ops on a rotated *and* non-uniformly
        // scaled object -- see the doc comments on invert3/rotation_matrix3.
        let mw_lin = if global {
            let mw = obj.getattr("matrix_world")?;
            let mut m = [[0.0f64; 3]; 3];
            for (i, row_out) in m.iter_mut().enumerate() {
                let row = mw.get_item(i)?;
                for (j, cell) in row_out.iter_mut().enumerate() {
                    *cell = row.get_item(j)?.extract::<f64>()?;
                }
            }
            Some(m)
        } else {
            None
        };
        let mw_lin_inv = match &mw_lin {
            Some(m) => Some(invert3(m).ok_or_else(|| {
                PyValueError::new_err(
                    "mesh_transform_selection: object's world matrix is not invertible \
                     (zero scale on some axis?)",
                )
            })?),
            None => None,
        };

        if let Some(vec) = &p.translate {
            let v = vec3(vec)?;
            let dir = match &mw_lin_inv {
                Some(inv) => matvec3(inv, v),
                None => v,
            };
            for co in positions.iter_mut() {
                co[0] += dir[0];
                co[1] += dir[1];
                co[2] += dir[2];
            }
        }

        if let Some(angle_deg) = p.rotate_angle {
            let axis_in = p.rotate_axis.as_deref().ok_or_else(|| {
                PyValueError::new_err("mesh_transform_selection: rotate_angle needs rotate_axis")
            })?;
            let axis_raw = vec3(axis_in)?;
            let axis_dir = match &mw_lin_inv {
                Some(inv) => matvec3(inv, axis_raw),
                None => axis_raw,
            };
            let axis = normalize3(axis_dir).ok_or_else(|| {
                PyValueError::new_err(
                    "mesh_transform_selection: rotate_axis must not be zero-length",
                )
            })?;
            let r = rotation_matrix3(axis, angle_deg.to_radians());
            for co in positions.iter_mut() {
                let d = [co[0] - pivot[0], co[1] - pivot[1], co[2] - pivot[2]];
                let d2 = matvec3(&r, d);
                *co = [pivot[0] + d2[0], pivot[1] + d2[1], pivot[2] + d2[2]];
            }
        }

        if let Some(sc) = &p.scale {
            let sc = vec3(sc)?;
            for co in positions.iter_mut() {
                let d = [co[0] - pivot[0], co[1] - pivot[1], co[2] - pivot[2]];
                let d2 = match (&mw_lin, &mw_lin_inv) {
                    (Some(lin), Some(inv)) => {
                        let dw = matvec3(lin, d);
                        let dw2 = [dw[0] * sc[0], dw[1] * sc[1], dw[2] * sc[2]];
                        matvec3(inv, dw2)
                    }
                    _ => [d[0] * sc[0], d[1] * sc[1], d[2] * sc[2]],
                };
                *co = [pivot[0] + d2[0], pivot[1] + d2[1], pivot[2] + d2[2]];
            }
        }

        for (v, co) in selected.iter().zip(positions.iter()) {
            v.setattr("co", PyTuple::new(py, *co)?)?;
        }
        commit_bmesh(&bm, &mesh)?;

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "vertices_moved": selected.len(),
            "pivot": pivot,
            "space": if global { "global" } else { "local" },
        }))
    }
);

// ============================================================================
// mesh_mark
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MeshMark {
    /// Mesh object to mark.
    pub name: String,
    /// Set (true) or clear (false) the UV seam flag on the selected edges. Seams
    /// matter: `mesh_unwrap` without them produces one giant stretched island.
    pub seam: Option<bool>,
    /// Set (true) or clear (false) the sharp-shading flag on the selected edges.
    /// Per-edge counterpart to obj_shade's whole-object SMOOTH/FLAT.
    pub sharp: Option<bool>,
    /// Subdivision-surface edge crease for the selected edges, 0 (soft) to 1
    /// (fully sharp under a Subdivision modifier). Values outside are clamped.
    pub crease: Option<f64>,
}

crate::tool!(
    "mesh_mark",
    "Mark seams / sharp / crease on the edges selected with mesh_select. Seams steer \
     mesh_unwrap, sharp is the per-edge counterpart to obj_shade, crease is read by a \
     Subdivision modifier",
    MeshMark,
    |py: Python<'_>, p: MeshMark| {
        if p.seam.is_none() && p.sharp.is_none() && p.crease.is_none() {
            return Err(PyValueError::new_err(
                "mesh_mark: give at least one of seam, sharp, crease",
            ));
        }

        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.name)?;
        let mesh = obj.getattr("data")?;
        let bm = open_bmesh(py, &mesh)?;

        let edges_seq = bm.getattr("edges")?;

        // Resolve (and, if needed, create) the crease custom-data layer *before*
        // gathering element references below. Verified live: `layers.float.new(...)`
        // invalidates every `BMEdge` Python wrapper obtained beforehand (their
        // `.is_valid` flips to False and touching them raises `ReferenceError: BMesh
        // data ... has been removed`), even though the underlying edges are
        // untouched -- adding a layer reallocates bmesh's internal custom-data
        // block. Fetching/creating the layer first, then gathering `edges`
        // afterward, sidesteps it entirely.
        let crease_layer = if p.crease.is_some() {
            let layers = edges_seq.getattr("layers")?.getattr("float")?;
            let existing = layers.call_method1("get", ("crease_edge",))?;
            Some(if existing.is_none() {
                layers.call_method1("new", ("crease_edge",))?
            } else {
                existing
            })
        } else {
            None
        };

        let mut edges = Vec::new();
        for e in edges_seq.try_iter()? {
            let e = e?;
            if e.getattr("select")?.extract::<bool>()? {
                edges.push(e);
            }
        }
        if edges.is_empty() {
            return Err(PyValueError::new_err(
                "mesh_mark: no edges selected; call mesh_select first",
            ));
        }

        if let Some(seam) = p.seam {
            for e in &edges {
                e.setattr("seam", seam)?;
            }
        }
        if let Some(sharp) = p.sharp {
            // bmesh has no separate "sharp" flag: `smooth == false` *is* sharp.
            for e in &edges {
                e.setattr("smooth", !sharp)?;
            }
        }
        if let Some(crease) = p.crease {
            let crease = crease.clamp(0.0, 1.0);
            // No `expect`: a panic inside Blender takes Blender down with it.
            let Some(layer) = crease_layer else {
                return Err(PyRuntimeError::new_err(
                    "crease: could not resolve the crease_edge layer",
                ));
            };
            for e in &edges {
                e.set_item(layer.clone(), crease)?;
            }
        }

        commit_bmesh(&bm, &mesh)?;

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "edges_marked": edges.len(),
        }))
    }
);
