//! Whole-mesh sculpt/remesh operators that a model can actually drive headless.
//!
//! ## Why not brush strokes
//! A brush stroke is a sequence of tiny incremental events driven by a mouse and a
//! real event loop -- there is no sane way for an LLM to "paint" and no way to run
//! one in `--background` at all. This file only exposes operators that act on the
//! *whole* mesh in one shot: remesh, symmetrize, mask flood fill. `bpy.ops.object
//! .mode_set(mode='SCULPT')` itself was verified live to work fine in
//! `--background` -- it is specific sculpt operators that do not.
//!
//! ## Two operators that CRASH Blender headless (verified live, not inferred)
//! `bpy.ops.paint.mask_flood_fill` and `bpy.ops.sculpt.symmetrize` both raise
//! `EXCEPTION_ACCESS_VIOLATION` the moment they run in `--background` --
//! `blender.exe --background --factory-startup --python <script>` with either call
//! present terminates the process outright, no Python exception, no traceback,
//! `blender.crash.txt` written to temp. The backtrace is identical for both:
//!
//! ```text
//! ED_undosys_stack_memfile_id_changed_tag
//! blender::ed::sculpt_paint::undo::push_begin_ex
//! blender::ed::sculpt_paint::mask::mask_flood_fill_exec   (or ..symmetrize_exec)
//! ```
//!
//! Both operators begin a **sculpt undo step**, and that path dereferences
//! something that only exists with a window/screen -- absent in `--background`.
//! `bpy.ops.sculpt.dynamic_topology_toggle` was also tested and does NOT crash
//! (it does not touch the sculpt undo stack, being a flag flip, not a geometry
//! edit), and `bpy.ops.sculpt.detail_flood_fill` fails its `poll()` cleanly
//! ("context is incorrect") rather than crashing -- neither of those facts helps,
//! see the dyntopo section below.
//!
//! Since **shipping a tool that crashes the host process is strictly worse than
//! shipping nothing**, both are worked around instead of exposed as-is:
//! - `sculpt_mask` writes the `.sculpt_mask` mesh attribute directly (FLOAT,
//!   POINT domain -- exactly what `mask_flood_fill` itself edits under the hood;
//!   verified live that creating/writing it needs no sculpt mode at all).
//! - `sculpt_symmetrize` calls `bmesh.ops.symmetrize` -- the identical bmesh-level
//!   function `sculpt.symmetrize`'s C implementation calls internally, minus the
//!   sculpt-undo wrapper that crashes. Verified live: `bmesh.ops.symmetrize`'s
//!   `direction` enum is exactly `-X`/`X`/`-Y`/`Y`/`-Z`/`Z` (read from the
//!   `ValueError` a bad value provokes), distinct from `bpy.ops.mesh.symmetrize`'s
//!   `NEGATIVE_X`/`POSITIVE_X`/... -- this file accepts the latter (matches this
//!   codebase's convention elsewhere) and translates.
//!
//! ## Dynamic topology: not shipped, and here is the live proof why
//! Toggling dyntopo on/off with `dynamic_topology_toggle` genuinely runs headless,
//! but toggling the flag alone changes zero geometry -- dyntopo only re-tessellates
//! *during a brush stroke*, which this file cannot do. The one operator that would
//! apply it without a stroke, `sculpt.detail_flood_fill`, fails
//! `poll()` in `--background` ("context is incorrect", verified live) -- so there
//! is no headless path that ever changes a single vertex via dyntopo. A tool that
//! can only toggle a flag with no observable effect is not worth shipping; this is
//! the "leave it out and report it" case the task brief asked for.
//!
//! ## What does run headless, verified live
//! - `bpy.ops.object.voxel_remesh` -- takes no operator arguments; reads
//!   `mesh.remesh_voxel_size` / `mesh.remesh_voxel_adaptivity` off the mesh
//!   itself. Runs entirely in Object Mode (it is an `object.*` operator, not a
//!   `sculpt.*` one) -- no mode switch needed at all. Returns `{'CANCELLED'}`
//!   (not a raise) on a mesh with zero polygons, so it goes through `op_ok`.
//! - `bpy.ops.object.quadriflow_remesh` -- also an Object Mode operator, also
//!   runs fine headless. Its RNA (`mode` ENUM `RATIO`/`EDGE`/`FACES`,
//!   `target_ratio`/`target_edge_length`/`target_faces`, `use_mesh_symmetry`, ...)
//!   was read live via `get_rna_type()`, not guessed. Also returns
//!   `{'CANCELLED'}` on a mesh with zero polygons.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use serde::Deserialize;
use serde_json::json;

use super::bpy::{data, op_ok, py_len};

/// Look up `bpy.data.objects[name]`, requiring it to be a `MESH` object. A
/// private copy of the equivalent helper in `mesh_edit.rs` -- this file must not
/// touch that one.
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

/// `bpy.context.view_layer.objects.active = obj` -- both remesh operators poll on
/// having an active object.
fn set_active(bpy: &Bound<'_, PyAny>, obj: &Bound<'_, PyAny>) -> PyResult<()> {
    bpy.getattr("context")?
        .getattr("view_layer")?
        .getattr("objects")?
        .setattr("active", obj)?;
    Ok(())
}

// ============================================================================
// sculpt_remesh
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SculptRemesh {
    /// Mesh object to remesh, in place, destructively. Existing UVs, vertex
    /// groups and shape keys do not survive a remesh -- neither does in
    /// Blender's own operator.
    pub name: String,
    /// "voxel" (default): robust uniform/adaptive remesh, works on almost any
    /// mesh. "quadriflow": quad-dominant retopology -- declines (reported as an
    /// error, not a silent no-op) on a non-manifold mesh or inconsistent face
    /// winding.
    pub method: Option<String>,

    /// method="voxel": edge length of the remesh voxel grid. Smaller values
    /// produce more polygons. Must be > 0. If omitted, the mesh's current
    /// `remesh_voxel_size` (Blender default 0.1) is used unchanged.
    pub voxel_size: Option<f64>,
    /// method="voxel": 0..1, redistributes voxel density toward areas of high
    /// curvature instead of a uniform grid. Clamped. Default 0 (uniform).
    pub voxel_adaptivity: Option<f64>,

    /// method="quadriflow": target polygon count. Mutually exclusive with
    /// `target_ratio` and `target_edge_length` -- give at most one.
    pub target_faces: Option<i32>,
    /// method="quadriflow": target face count as a multiple of the current one
    /// (0.5 halves it, 2.0 doubles it). Mutually exclusive with `target_faces`
    /// and `target_edge_length`.
    pub target_ratio: Option<f64>,
    /// method="quadriflow": target edge length. Mutually exclusive with
    /// `target_faces` and `target_ratio`.
    pub target_edge_length: Option<f64>,
    /// method="quadriflow": preserve the mesh's own mirror symmetry in the
    /// result. Default true, matching Blender's own operator default.
    pub quad_preserve_symmetry: Option<bool>,
}

crate::tool!(
    "sculpt_remesh",
    "Destructively rebuild a mesh's topology: voxel remesh (uniform/adaptive, \
     robust -- the workhorse) or quadriflow remesh (quad-dominant retopology). \
     Runs entirely in Object Mode; no sculpt-mode brush strokes involved. \
     Topology only -- sculpt_symmetrize mirrors a mesh, sculpt_mask flood-fills \
     its sculpt mask.",
    SculptRemesh,
    |py: Python<'_>, p: SculptRemesh| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.name)?;
        let mesh = obj.getattr("data")?;

        let method = p
            .method
            .unwrap_or_else(|| "voxel".to_string())
            .to_lowercase();
        if !matches!(method.as_str(), "voxel" | "quadriflow") {
            return Err(PyValueError::new_err(format!(
                "sculpt_remesh: method must be voxel or quadriflow, got `{method}`"
            )));
        }

        let quad_params_given = p.target_faces.is_some()
            || p.target_ratio.is_some()
            || p.target_edge_length.is_some()
            || p.quad_preserve_symmetry.is_some();
        if method == "voxel" && quad_params_given {
            return Err(PyValueError::new_err(
                "sculpt_remesh: target_faces/target_ratio/target_edge_length/\
                 quad_preserve_symmetry only apply to method=quadriflow",
            ));
        }
        let voxel_params_given = p.voxel_size.is_some() || p.voxel_adaptivity.is_some();
        if method == "quadriflow" && voxel_params_given {
            return Err(PyValueError::new_err(
                "sculpt_remesh: voxel_size/voxel_adaptivity only apply to method=voxel",
            ));
        }
        let targets_given = [
            p.target_faces.is_some(),
            p.target_ratio.is_some(),
            p.target_edge_length.is_some(),
        ]
        .iter()
        .filter(|g| **g)
        .count();
        if targets_given > 1 {
            return Err(PyValueError::new_err(
                "sculpt_remesh: give at most one of target_faces, target_ratio, \
                 target_edge_length",
            ));
        }

        set_active(&bpy, &obj)?;
        let verts_before = py_len(py, &mesh.getattr("vertices")?)?;
        let polys_before = py_len(py, &mesh.getattr("polygons")?)?;

        match method.as_str() {
            "voxel" => {
                if let Some(size) = p.voxel_size {
                    if size <= 0.0 {
                        return Err(PyValueError::new_err(
                            "sculpt_remesh: voxel_size must be > 0",
                        ));
                    }
                    mesh.setattr("remesh_voxel_size", size)?;
                }
                if let Some(a) = p.voxel_adaptivity {
                    mesh.setattr("remesh_voxel_adaptivity", a.clamp(0.0, 1.0))?;
                }
                let result = bpy
                    .getattr("ops")?
                    .getattr("object")?
                    .call_method0("voxel_remesh")?;
                op_ok(&result, &format!("voxel remesh {}", p.name))?;
            }
            _ => {
                let kwargs = PyDict::new(py);
                if let Some(f) = p.target_faces {
                    if f <= 0 {
                        return Err(PyValueError::new_err(
                            "sculpt_remesh: target_faces must be > 0",
                        ));
                    }
                    kwargs.set_item("mode", "FACES")?;
                    kwargs.set_item("target_faces", f)?;
                } else if let Some(r) = p.target_ratio {
                    if r <= 0.0 {
                        return Err(PyValueError::new_err(
                            "sculpt_remesh: target_ratio must be > 0",
                        ));
                    }
                    kwargs.set_item("mode", "RATIO")?;
                    kwargs.set_item("target_ratio", r)?;
                } else if let Some(e) = p.target_edge_length {
                    if e <= 0.0 {
                        return Err(PyValueError::new_err(
                            "sculpt_remesh: target_edge_length must be > 0",
                        ));
                    }
                    kwargs.set_item("mode", "EDGE")?;
                    kwargs.set_item("target_edge_length", e)?;
                }
                if let Some(sym) = p.quad_preserve_symmetry {
                    kwargs.set_item("use_mesh_symmetry", sym)?;
                }
                let result = bpy.getattr("ops")?.getattr("object")?.call_method(
                    "quadriflow_remesh",
                    (),
                    Some(&kwargs),
                )?;
                op_ok(&result, &format!("quadriflow remesh {}", p.name))?;
            }
        }

        let mesh_after = obj.getattr("data")?;
        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "method": method,
            "vertices_before": verts_before,
            "polygons_before": polys_before,
            "vertices": py_len(py, &mesh_after.getattr("vertices")?)?,
            "polygons": py_len(py, &mesh_after.getattr("polygons")?)?,
        }))
    }
);

// ============================================================================
// sculpt_symmetrize
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SculptSymmetrize {
    /// Mesh object to symmetrize, in place.
    pub name: String,
    /// Mirror plane and which half is kept: NEGATIVE_X (default) keeps the -X
    /// half and mirrors it onto +X, POSITIVE_X keeps +X and mirrors onto -X, and
    /// likewise for Y/Z. Same enum `bpy.ops.mesh.symmetrize` uses.
    pub direction: Option<String>,
    /// Merge threshold, in local mesh units, for vertices that land on the
    /// symmetry plane. Default 0.0001 (Blender's own `mesh.symmetrize` operator
    /// default).
    pub merge_distance: Option<f64>,
}

crate::tool!(
    "sculpt_symmetrize",
    "Mirror one half of a mesh onto the other across a chosen axis plane, \
     replacing whichever half is discarded. Runs in Object Mode via bmesh -- \
     bpy.ops.sculpt.symmetrize needs Sculpt Mode and crashes Blender outright \
     when run headless (verified live: a null-pointer access violation inside \
     its undo push, with no window to push into), so this calls the identical \
     bmesh.ops.symmetrize the sculpt operator's C implementation uses \
     internally, minus the crashing sculpt-undo wrapper.",
    SculptSymmetrize,
    |py: Python<'_>, p: SculptSymmetrize| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.name)?;
        let mesh = obj.getattr("data")?;

        let dir_in = p
            .direction
            .as_deref()
            .unwrap_or("NEGATIVE_X")
            .to_uppercase();
        let dir_lit = match dir_in.as_str() {
            "NEGATIVE_X" => "-X",
            "POSITIVE_X" => "X",
            "NEGATIVE_Y" => "-Y",
            "POSITIVE_Y" => "Y",
            "NEGATIVE_Z" => "-Z",
            "POSITIVE_Z" => "Z",
            other => {
                return Err(PyValueError::new_err(format!(
                    "sculpt_symmetrize: direction must be one of NEGATIVE_X, \
                     POSITIVE_X, NEGATIVE_Y, POSITIVE_Y, NEGATIVE_Z, POSITIVE_Z, \
                     got `{other}`"
                )));
            }
        };
        let dist = p.merge_distance.unwrap_or(0.0001);
        if dist < 0.0 {
            return Err(PyValueError::new_err(
                "sculpt_symmetrize: merge_distance must be >= 0",
            ));
        }

        let bmesh = py.import("bmesh")?;
        let bm = bmesh.call_method0("new")?;
        bm.call_method1("from_mesh", (&mesh,))?;

        let vert_count = py_len(py, &bm.getattr("verts")?)?;
        if vert_count == 0 {
            bm.call_method0("free")?;
            return Err(PyValueError::new_err(
                "sculpt_symmetrize: mesh has no geometry",
            ));
        }

        let mut input = Vec::new();
        for x in bm.getattr("verts")?.try_iter()? {
            input.push(x?);
        }
        for x in bm.getattr("edges")?.try_iter()? {
            input.push(x?);
        }
        for x in bm.getattr("faces")?.try_iter()? {
            input.push(x?);
        }
        let input_py = PyList::new(py, input)?;

        let kwargs = PyDict::new(py);
        kwargs.set_item("input", input_py)?;
        kwargs.set_item("direction", dir_lit)?;
        kwargs.set_item("dist", dist)?;
        bmesh
            .getattr("ops")?
            .call_method("symmetrize", (&bm,), Some(&kwargs))?;

        bm.call_method1("to_mesh", (&mesh,))?;
        mesh.call_method0("update")?;
        bm.call_method0("free")?;

        let mesh_after = obj.getattr("data")?;
        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "direction": dir_in,
            "vertices": py_len(py, &mesh_after.getattr("vertices")?)?,
            "polygons": py_len(py, &mesh_after.getattr("polygons")?)?,
        }))
    }
);

// ============================================================================
// sculpt_mask
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SculptMask {
    /// Mesh object to mask.
    pub name: String,
    /// Mask value to flood-fill onto every vertex, 0..1 (0 = unmasked / fully
    /// paintable, 1 = fully masked / protected from sculpting). Clamped.
    pub value: f64,
}

crate::tool!(
    "sculpt_mask",
    "Flood-fill a mesh's sculpt mask to one uniform value across every vertex -- \
     the non-brush way to fully mask or fully clear a mesh (1.0 protects \
     everything, 0.0 clears it). bpy.ops.paint.mask_flood_fill crashes Blender \
     outright when run headless (verified live: a null-pointer access violation \
     inside its undo push, identical crash site to sculpt.symmetrize), so this \
     writes the `.sculpt_mask` mesh attribute directly -- exactly what that \
     operator itself edits, and no sculpt mode is needed to touch it.",
    SculptMask,
    |py: Python<'_>, p: SculptMask| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_mesh_obj(&d, &p.name)?;
        let mesh = obj.getattr("data")?;

        let value = p.value.clamp(0.0, 1.0);

        let attrs = mesh.getattr("attributes")?;
        let existing = attrs.call_method1("get", (".sculpt_mask",))?;
        let created = existing.is_none();
        let attr = if created {
            attrs.call_method1("new", (".sculpt_mask", "FLOAT", "POINT"))?
        } else {
            existing
        };

        let attr_data = attr.getattr("data")?;
        let n = py_len(py, &attr_data)?;
        for i in 0..n {
            attr_data.get_item(i as i64)?.setattr("value", value)?;
        }

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "value": value,
            "vertices": n,
            "created_attribute": created,
        }))
    }
);
