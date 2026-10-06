//! Scene analysis, camera, light, template and preset tools. Registered
//! automatically — nothing outside this file names them.
//!
//! Ported from the Python handlers `_h_scene_analyze` / `_h_scene_detect_issues` /
//! `_h_scene_auto_fix` / `_h_scene_get_stats` / `_h_scene_list_unused` /
//! `_h_scene_purge_unused` / `_h_scene_apply_template` / `_h_scene_list_templates` /
//! `_h_cam_*` / `_h_light_*` / `_h_preset_*` in `addon/mcp_server/__init__.py`.
//!
//! ## Consolidation: `scene_analyze` vs `scene_detect_issues`
//! Both handlers used to walk every mesh's `polygons` independently to build an
//! edge -> face-count map for a non-manifold check. That walk (and the
//! "vertices referenced by a face" walk `scene_analyze` also needs for its loose
//! vertex count) is now done once per mesh by [`scan_mesh`], shared by both tools.
//!
//! ## Bug fix: boundary edges are not non-manifold
//! The original edge-count check flagged `count != 2` as non-manifold, which also
//! flags **boundary edges** (`count == 1`) — an edge on the border of an open mesh
//! (a plane, a cut, a bowl). Any non-closed mesh therefore always reported
//! "non-manifold" even when perfectly valid. [`scan_mesh`] separates the two:
//! only `count > 2` is genuinely non-manifold; `count == 1` is a boundary edge,
//! reported separately by `scene_detect_issues` as `boundary_edges`.
//!
//! ## `inverted_normals` (was a dead key)
//! `scene_detect_issues` advertised an `inverted_normals` result key that the
//! original handler never populated. It is populated here via the divergence
//! theorem: for a **closed, manifold** mesh (no boundary or non-manifold edges),
//! the signed volume `sum(dot(p0, cross(p1, p2))) / 6` over a per-polygon fan
//! triangulation is positive iff the polygon winding is outward-consistent. A
//! negative signed volume means the mesh's faces are, as a whole, wound inward —
//! i.e. its normals are flipped. This check does not apply to open meshes, where
//! "enclosed volume" is not a meaningful concept.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyIterator, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};

use super::bpy::{data, op_ok, py_len, scene};

// ============================================================================
// Shared helpers
// ============================================================================

/// Extract a Python float sequence (`location`, `scale`, ...) into a `Vec<f64>`.
fn py_vec(seq: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    let mut v = Vec::new();
    for x in PyIterator::from_object(seq)? {
        v.push(x?.extract::<f64>()?);
    }
    Ok(v)
}

/// Round-trips a `serde_json::Value` into a native Python object via `json.loads`,
/// so arbitrary caller-supplied JSON (preset `data`, ...) becomes a plain Python
/// dict/list/primitive that Blender's custom-property system can store directly.
fn value_to_py<'py>(py: Python<'py>, v: &Value) -> PyResult<Bound<'py, PyAny>> {
    let s = serde_json::to_string(v)
        .map_err(|e| PyValueError::new_err(format!("JSON encode value: {e}")))?;
    py.import("json")?.call_method1("loads", (s,))
}

/// Look up an object by name and require it to have `want_type`, mirroring the
/// `if not obj or obj.type != "X": raise ValueError(...)` guard repeated across
/// the light/camera handlers.
fn get_typed<'py>(
    d: &Bound<'py, PyAny>,
    name: &str,
    want_type: &str,
    kind_label: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let obj = d.getattr("objects")?.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!(
            "{kind_label} not found: {name}"
        )));
    }
    let ty: String = obj.getattr("type")?.extract()?;
    if ty != want_type {
        return Err(PyValueError::new_err(format!(
            "{kind_label} not found: {name}"
        )));
    }
    Ok(obj)
}

/// Result of a single walk over a mesh's `polygons`: everything `scene_analyze`
/// and `scene_detect_issues` need from the edge/vertex adjacency, computed once.
struct MeshScan {
    /// Vertex indices referenced by at least one polygon.
    verts_in_faces: usize,
    /// Edges shared by more than two faces — a genuine non-manifold edge.
    non_manifold_edges: usize,
    /// Edges shared by exactly one face — a mesh boundary, not an error.
    boundary_edges: usize,
}

fn scan_mesh(mesh: &Bound<'_, PyAny>) -> PyResult<MeshScan> {
    let polygons = mesh.getattr("polygons")?;
    let mut edge_counts: HashMap<(usize, usize), u32> = HashMap::new();
    let mut verts_in_faces: HashSet<usize> = HashSet::new();

    for poly in PyIterator::from_object(&polygons)? {
        let poly = poly?;
        let idx: Vec<usize> = PyIterator::from_object(&poly.getattr("vertices")?)?
            .map(|v| v?.extract::<usize>())
            .collect::<PyResult<_>>()?;
        let n = idx.len();
        for i in 0..n {
            let a = idx[i];
            let b = idx[(i + 1) % n];
            verts_in_faces.insert(a);
            let key = if a < b { (a, b) } else { (b, a) };
            *edge_counts.entry(key).or_insert(0) += 1;
        }
    }

    let mut non_manifold_edges = 0usize;
    let mut boundary_edges = 0usize;
    for c in edge_counts.values() {
        match c {
            1 => boundary_edges += 1,
            2 => {}
            _ => non_manifold_edges += 1,
        }
    }

    Ok(MeshScan {
        verts_in_faces: verts_in_faces.len(),
        non_manifold_edges,
        boundary_edges,
    })
}

/// Signed enclosed volume of a mesh via the divergence theorem: each polygon is
/// fan-triangulated from its own first vertex, and every triangle contributes
/// `dot(p0, cross(p1, p2)) / 6`. Only meaningful for a closed, manifold mesh —
/// callers must check [`MeshScan`] has no boundary/non-manifold edges first.
fn mesh_signed_volume(mesh: &Bound<'_, PyAny>) -> PyResult<f64> {
    let mut coords: Vec<[f64; 3]> = Vec::new();
    for v in PyIterator::from_object(&mesh.getattr("vertices")?)? {
        let co = v?.getattr("co")?;
        coords.push([
            co.get_item(0)?.extract()?,
            co.get_item(1)?.extract()?,
            co.get_item(2)?.extract()?,
        ]);
    }
    let vertex_at = |i: usize| -> PyResult<[f64; 3]> {
        coords
            .get(i)
            .copied()
            .ok_or_else(|| PyValueError::new_err("mesh polygon references an out-of-range vertex"))
    };

    let mut volume = 0.0;
    for poly in PyIterator::from_object(&mesh.getattr("polygons")?)? {
        let poly = poly?;
        let idx: Vec<usize> = PyIterator::from_object(&poly.getattr("vertices")?)?
            .map(|v| v?.extract::<usize>())
            .collect::<PyResult<_>>()?;
        if idx.len() < 3 {
            continue;
        }
        let p0 = vertex_at(idx[0])?;
        for i in 1..idx.len() - 1 {
            let p1 = vertex_at(idx[i])?;
            let p2 = vertex_at(idx[i + 1])?;
            volume += (p0[0] * (p1[1] * p2[2] - p1[2] * p2[1])
                - p0[1] * (p1[0] * p2[2] - p1[2] * p2[0])
                + p0[2] * (p1[0] * p2[1] - p1[1] * p2[0]))
                / 6.0;
        }
    }
    Ok(volume)
}

/// True when an object's scale is (1,1,1) within Blender's usual epsilon. Shared
/// by `scene_analyze` ("Unapplied scale" issue) and `scene_auto_fix` (decides
/// whether `transform_apply` needs to run).
fn scale_applied(obj: &Bound<'_, PyAny>) -> PyResult<bool> {
    let scale = py_vec(&obj.getattr("scale")?)?;
    Ok(scale.len() == 3
        && (scale[0] - 1.0).abs() <= 0.001
        && (scale[1] - 1.0).abs() <= 0.001
        && (scale[2] - 1.0).abs() <= 0.001)
}

// ============================================================================
// Scene Statistics & Utilities
// ============================================================================

/// `scene_get_stats` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneGetStats {}

crate::tool!(
    "scene_get_stats",
    "Scene-wide totals: object counts per type, vertices/edges/faces/triangles across every mesh, data-block counts, render engine and resolution, and a rough memory estimate. Takes no parameters. Use scene_analyze instead when the question is what is *wrong* with the scene rather than how big it is.",
    SceneGetStats,
    |py: Python<'_>, _p: SceneGetStats| {
        let bpy = py.import("bpy")?;
        let sc = scene(&bpy)?;
        let d = data(&bpy)?;

        let mut obj_counts: serde_json::Map<String, Value> = serde_json::Map::new();
        let mut total_verts: i64 = 0;
        let mut total_edges: i64 = 0;
        let mut total_faces: i64 = 0;
        let mut total_tris: i64 = 0;

        for obj in PyIterator::from_object(&d.getattr("objects")?)? {
            let obj = obj?;
            let ty: String = obj.getattr("type")?.extract()?;
            let count = obj_counts.get(&ty).and_then(Value::as_i64).unwrap_or(0);
            obj_counts.insert(ty.clone(), json!(count + 1));

            if ty == "MESH" {
                let mesh_data = obj.getattr("data")?;
                if !mesh_data.is_none() {
                    total_verts += py_len(py, &mesh_data.getattr("vertices")?)? as i64;
                    total_edges += py_len(py, &mesh_data.getattr("edges")?)? as i64;
                    let polys = mesh_data.getattr("polygons")?;
                    total_faces += py_len(py, &polys)? as i64;
                    // `loop_total` is the polygon's vertex count without materializing
                    // its `vertices` array — cheaper than Python's `len(poly.vertices)`.
                    for poly in PyIterator::from_object(&polys)? {
                        let loop_total: i64 = poly?.getattr("loop_total")?.extract()?;
                        total_tris += loop_total - 2;
                    }
                }
            }
        }

        let data_counts = json!({
            "objects": py_len(py, &d.getattr("objects")?)?,
            "meshes": py_len(py, &d.getattr("meshes")?)?,
            "materials": py_len(py, &d.getattr("materials")?)?,
            "textures": py_len(py, &d.getattr("textures")?)?,
            "images": py_len(py, &d.getattr("images")?)?,
            "collections": py_len(py, &d.getattr("collections")?)?,
            "actions": py_len(py, &d.getattr("actions")?)?,
            "armatures": py_len(py, &d.getattr("armatures")?)?,
            "cameras": py_len(py, &d.getattr("cameras")?)?,
            "lights": py_len(py, &d.getattr("lights")?)?,
            "curves": py_len(py, &d.getattr("curves")?)?,
            "fonts": py_len(py, &d.getattr("fonts")?)?,
            "worlds": py_len(py, &d.getattr("worlds")?)?,
            "node_groups": py_len(py, &d.getattr("node_groups")?)?,
            "particles": py_len(py, &d.getattr("particles")?)?,
        });

        let render = sc.getattr("render")?;
        let render_info = json!({
            "engine": render.getattr("engine")?.extract::<String>()?,
            "resolution": [
                render.getattr("resolution_x")?.extract::<i32>()?,
                render.getattr("resolution_y")?.extract::<i32>()?,
            ],
            "resolution_percentage": render.getattr("resolution_percentage")?.extract::<i32>()?,
            "fps": render.getattr("fps")?.extract::<i32>()?,
            "frame_range": [
                sc.getattr("frame_start")?.extract::<i32>()?,
                sc.getattr("frame_end")?.extract::<i32>()?,
            ],
            "current_frame": sc.getattr("frame_current")?.extract::<i32>()?,
        });

        let mem_estimate = json!({
            "vertices_mb": ((total_verts as f64 * 12.0 / 1024.0 / 1024.0) * 100.0).round() / 100.0,
            "faces_mb": ((total_faces as f64 * 16.0 / 1024.0 / 1024.0) * 100.0).round() / 100.0,
        });

        Ok(json!({
            "scene_name": sc.getattr("name")?.extract::<String>()?,
            "objects_by_type": obj_counts,
            "geometry": {
                "vertices": total_verts,
                "edges": total_edges,
                "faces": total_faces,
                "triangles": total_tris,
            },
            "data_blocks": data_counts,
            "render": render_info,
            "memory_estimate": mem_estimate,
        }))
    }
);

/// Names of data-blocks in `coll` with zero users.
fn unused_names(coll: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
    let mut out = Vec::new();
    for item in PyIterator::from_object(coll)? {
        let item = item?;
        let users: i64 = item.getattr("users")?.extract()?;
        if users == 0 {
            out.push(item.getattr("name")?.extract::<String>()?);
        }
    }
    Ok(out)
}

/// `scene_list_unused` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneListUnused {}

crate::tool!(
    "scene_list_unused",
    "List orphan data-blocks — meshes, materials, images, textures, actions, node groups, curves and armatures with zero users. Takes no parameters. The dry run for scene_purge_unused, which deletes exactly these.",
    SceneListUnused,
    |py: Python<'_>, _p: SceneListUnused| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        let meshes = unused_names(&d.getattr("meshes")?)?;
        let materials = unused_names(&d.getattr("materials")?)?;
        let images = unused_names(&d.getattr("images")?)?;
        let textures = unused_names(&d.getattr("textures")?)?;
        let actions = unused_names(&d.getattr("actions")?)?;
        let node_groups = unused_names(&d.getattr("node_groups")?)?;
        let curves = unused_names(&d.getattr("curves")?)?;
        let armatures = unused_names(&d.getattr("armatures")?)?;

        let total = meshes.len()
            + materials.len()
            + images.len()
            + textures.len()
            + actions.len()
            + node_groups.len()
            + curves.len()
            + armatures.len();

        Ok(json!({
            "unused": {
                "meshes": meshes,
                "materials": materials,
                "images": images,
                "textures": textures,
                "actions": actions,
                "node_groups": node_groups,
                "curves": curves,
                "armatures": armatures,
            },
            "total_unused": total,
        }))
    }
);

/// `scene_purge_unused` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct ScenePurgeUnused {}

crate::tool!(
    "scene_purge_unused",
    "Delete every orphan data-block (`outliner.orphans_purge`, recursive) and report how many went. Takes no parameters and cannot be undone — call scene_list_unused first to see what would be removed.",
    ScenePurgeUnused,
    |py: Python<'_>, _p: ScenePurgeUnused| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        const ATTRS: [&str; 8] = [
            "meshes",
            "materials",
            "images",
            "textures",
            "actions",
            "node_groups",
            "curves",
            "armatures",
        ];
        let count_all = |d: &Bound<'_, PyAny>| -> PyResult<usize> {
            let mut total = 0usize;
            for attr in ATTRS {
                total += py_len(py, &d.getattr(attr)?)?;
            }
            Ok(total)
        };

        let before = count_all(&d)?;
        let kw = PyDict::new(py);
        kw.set_item("do_recursive", true)?;
        // Not checked: verified live that `orphans_purge` returns {'CANCELLED'}
        // precisely when there was nothing orphaned to purge — that IS this
        // tool's success case (`before == after`, reported honestly below), not
        // a failure to surface as `Err`.
        bpy.getattr("ops")?
            .getattr("outliner")?
            .call_method("orphans_purge", (), Some(&kw))?;
        let after = count_all(&d)?;

        Ok(json!({ "removed": before.saturating_sub(after), "remaining_data_blocks": after }))
    }
);

// ============================================================================
// Scene Templates
// ============================================================================

/// `scene_list_templates` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneListTemplates {}

crate::tool!(
    "scene_list_templates",
    "List the scene templates scene_apply_template accepts, each with a one-line description. Takes no parameters.",
    SceneListTemplates,
    |_py: Python<'_>, _p: SceneListTemplates| {
        Ok(json!({
            "templates": {
                "studio": "Three-point lighting setup with floor and camera",
                "outdoor": "Sun light with sky background",
                "product_shot": "Clean product photography setup with turntable",
                "architectural": "Interior lighting setup with area lights",
                "portrait": "Portrait lighting with key, fill, and rim lights",
            }
        }))
    }
);

fn op_add_plane<'py>(
    py: Python<'py>,
    bpy: &Bound<'py, PyAny>,
    size: f64,
    loc: [f64; 3],
) -> PyResult<Bound<'py, PyAny>> {
    let kw = PyDict::new(py);
    kw.set_item("size", size)?;
    kw.set_item("location", PyTuple::new(py, loc)?)?;
    let result =
        bpy.getattr("ops")?
            .getattr("mesh")?
            .call_method("primitive_plane_add", (), Some(&kw))?;
    op_ok(&result, "add plane")?;
    bpy.getattr("context")?.getattr("active_object")
}

fn op_add_light<'py>(
    py: Python<'py>,
    bpy: &Bound<'py, PyAny>,
    ty: &str,
    loc: [f64; 3],
) -> PyResult<Bound<'py, PyAny>> {
    let kw = PyDict::new(py);
    kw.set_item("type", ty)?;
    kw.set_item("location", PyTuple::new(py, loc)?)?;
    let result = bpy
        .getattr("ops")?
        .getattr("object")?
        .call_method("light_add", (), Some(&kw))?;
    op_ok(&result, &format!("add {ty} light"))?;
    bpy.getattr("context")?.getattr("active_object")
}

fn op_add_camera<'py>(
    py: Python<'py>,
    bpy: &Bound<'py, PyAny>,
    loc: [f64; 3],
) -> PyResult<Bound<'py, PyAny>> {
    let kw = PyDict::new(py);
    kw.set_item("location", PyTuple::new(py, loc)?)?;
    let result = bpy
        .getattr("ops")?
        .getattr("object")?
        .call_method("camera_add", (), Some(&kw))?;
    op_ok(&result, "add camera")?;
    bpy.getattr("context")?.getattr("active_object")
}

fn rename<'py>(obj: &Bound<'py, PyAny>, name: &str) -> PyResult<String> {
    obj.setattr("name", name)?;
    obj.getattr("name")?.extract()
}

fn template_studio<'py>(py: Python<'py>, bpy: &Bound<'py, PyAny>) -> PyResult<Vec<String>> {
    let mut created = Vec::new();

    let floor = op_add_plane(py, bpy, 20.0, [0.0, 0.0, 0.0])?;
    created.push(rename(&floor, "Floor")?);

    let key = op_add_light(py, bpy, "AREA", [4.0, -3.0, 5.0])?;
    let key_data = key.getattr("data")?;
    key_data.setattr("energy", 500.0)?;
    key_data.setattr("size", 2.0)?;
    created.push(rename(&key, "Key_Light")?);

    let fill = op_add_light(py, bpy, "AREA", [-3.0, -2.0, 3.0])?;
    let fill_data = fill.getattr("data")?;
    fill_data.setattr("energy", 200.0)?;
    fill_data.setattr("size", 3.0)?;
    created.push(rename(&fill, "Fill_Light")?);

    let rim = op_add_light(py, bpy, "AREA", [0.0, 4.0, 4.0])?;
    let rim_data = rim.getattr("data")?;
    rim_data.setattr("energy", 300.0)?;
    rim_data.setattr("size", 1.5)?;
    created.push(rename(&rim, "Rim_Light")?);

    let cam = op_add_camera(py, bpy, [7.0, -7.0, 5.0])?;
    cam.setattr("rotation_euler", PyTuple::new(py, [1.1, 0.0, 0.8])?)?;
    scene(bpy)?.setattr("camera", &cam)?;
    created.push(rename(&cam, "Camera")?);

    Ok(created)
}

fn template_outdoor<'py>(py: Python<'py>, bpy: &Bound<'py, PyAny>) -> PyResult<Vec<String>> {
    let mut created = Vec::new();

    let ground = op_add_plane(py, bpy, 50.0, [0.0, 0.0, 0.0])?;
    created.push(rename(&ground, "Ground")?);

    let sun = op_add_light(py, bpy, "SUN", [0.0, 0.0, 10.0])?;
    sun.getattr("data")?.setattr("energy", 5.0)?;
    sun.setattr("rotation_euler", PyTuple::new(py, [0.8, 0.2, 0.5])?)?;
    created.push(rename(&sun, "Sun")?);

    let cam = op_add_camera(py, bpy, [10.0, -10.0, 5.0])?;
    cam.setattr("rotation_euler", PyTuple::new(py, [1.2, 0.0, 0.8])?)?;
    scene(bpy)?.setattr("camera", &cam)?;
    created.push(rename(&cam, "Camera")?);

    let d = data(bpy)?;
    let sc = scene(bpy)?;
    let mut world = sc.getattr("world")?;
    if world.is_none() {
        world = d.getattr("worlds")?.call_method1("new", ("World",))?;
        sc.setattr("world", &world)?;
    }
    world.setattr("use_nodes", true)?;
    let bg = world
        .getattr("node_tree")?
        .getattr("nodes")?
        .call_method1("get", ("Background",))?;
    if !bg.is_none() {
        bg.getattr("inputs")?
            .get_item(0)?
            .setattr("default_value", PyTuple::new(py, [0.5, 0.7, 1.0, 1.0])?)?;
    }

    Ok(created)
}

fn template_product_shot<'py>(py: Python<'py>, bpy: &Bound<'py, PyAny>) -> PyResult<Vec<String>> {
    let mut created = Vec::new();

    let floor = op_add_plane(py, bpy, 10.0, [0.0, 0.0, 0.0])?;
    created.push(rename(&floor, "Cyclorama")?);

    for (i, (x, y)) in [(-3.0, -3.0), (3.0, -3.0), (0.0, 3.0)]
        .into_iter()
        .enumerate()
    {
        let light = op_add_light(py, bpy, "AREA", [x, y, 4.0])?;
        let light_data = light.getattr("data")?;
        light_data.setattr("energy", 300.0)?;
        light_data.setattr("size", 2.0)?;
        created.push(rename(&light, &format!("Softbox_{}", i + 1))?);
    }

    let cam = op_add_camera(py, bpy, [0.0, -5.0, 3.0])?;
    cam.setattr("rotation_euler", PyTuple::new(py, [1.3, 0.0, 0.0])?)?;
    scene(bpy)?.setattr("camera", &cam)?;
    created.push(rename(&cam, "Camera")?);

    Ok(created)
}

fn template_architectural<'py>(py: Python<'py>, bpy: &Bound<'py, PyAny>) -> PyResult<Vec<String>> {
    let mut created = Vec::new();

    let floor = op_add_plane(py, bpy, 20.0, [0.0, 0.0, 0.0])?;
    created.push(rename(&floor, "Floor")?);

    let positions = [(5.0, 0.0, 3.0), (-5.0, 0.0, 3.0), (0.0, 5.0, 3.0)];
    for (i, pos) in positions.into_iter().enumerate() {
        let light = op_add_light(py, bpy, "AREA", [pos.0, pos.1, pos.2])?;
        let light_data = light.getattr("data")?;
        light_data.setattr("energy", 200.0)?;
        light_data.setattr("size", 3.0)?;
        light_data.setattr("size_y", 2.0)?;
        created.push(rename(&light, &format!("Window_Light_{}", i + 1))?);
    }

    let cam = op_add_camera(py, bpy, [8.0, -8.0, 2.0])?;
    cam.setattr("rotation_euler", PyTuple::new(py, [1.4, 0.0, 0.8])?)?;
    cam.getattr("data")?.setattr("lens", 24.0)?;
    scene(bpy)?.setattr("camera", &cam)?;
    created.push(rename(&cam, "Camera")?);

    Ok(created)
}

fn template_portrait<'py>(py: Python<'py>, bpy: &Bound<'py, PyAny>) -> PyResult<Vec<String>> {
    let mut created = Vec::new();

    let key = op_add_light(py, bpy, "AREA", [2.0, -2.0, 2.0])?;
    let key_data = key.getattr("data")?;
    key_data.setattr("energy", 400.0)?;
    key_data.setattr("size", 1.5)?;
    created.push(rename(&key, "Key_Light")?);

    let fill = op_add_light(py, bpy, "AREA", [-2.0, -1.0, 1.5])?;
    let fill_data = fill.getattr("data")?;
    fill_data.setattr("energy", 150.0)?;
    fill_data.setattr("size", 2.0)?;
    created.push(rename(&fill, "Fill_Light")?);

    // Rotations are radians. The Python template spelled these 3.14 / 1.57; they are
    // meant to be half a turn and a quarter turn, so use the exact constants.
    let rim = op_add_light(py, bpy, "SPOT", [0.0, 2.0, 3.0])?;
    rim.getattr("data")?.setattr("energy", 300.0)?;
    rim.setattr(
        "rotation_euler",
        PyTuple::new(py, [0.5, 0.0, std::f64::consts::PI])?,
    )?;
    created.push(rename(&rim, "Rim_Light")?);

    let cam = op_add_camera(py, bpy, [0.0, -3.0, 1.6])?;
    cam.setattr(
        "rotation_euler",
        PyTuple::new(py, [std::f64::consts::FRAC_PI_2, 0.0, 0.0])?,
    )?;
    cam.getattr("data")?.setattr("lens", 85.0)?;
    scene(bpy)?.setattr("camera", &cam)?;
    created.push(rename(&cam, "Camera")?);

    Ok(created)
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneApplyTemplate {
    /// Exactly one of: studio (floor, three-point area lights, camera), outdoor
    /// (ground, sun, sky-blue world, camera), product_shot (cyclorama, three
    /// softboxes, camera), architectural (floor, three window area lights, 24mm
    /// camera), portrait (key/fill/rim lights, 85mm camera). Any other value is
    /// an error. Names are case-sensitive and lowercase.
    pub template: String,
    /// Delete every object in the file first. Default false, i.e. the template's
    /// objects are added alongside what is already there. Not undoable.
    pub clear_scene: Option<bool>,
}

crate::tool!(
    "scene_apply_template",
    "Build a ready-made lighting/camera setup in one call: studio, outdoor, product_shot, architectural or portrait. Each creates its own lights and a camera and makes that camera the scene's active one. scene_list_templates describes them; light_create and cam_create are the manual alternative.",
    SceneApplyTemplate,
    |py: Python<'_>, p: SceneApplyTemplate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        if p.clear_scene.unwrap_or(false) {
            let objects = d.getattr("objects")?;
            let mut to_remove: Vec<Bound<'_, PyAny>> = Vec::new();
            for o in PyIterator::from_object(&objects)? {
                to_remove.push(o?);
            }
            for o in to_remove {
                let kw = PyDict::new(py);
                kw.set_item("do_unlink", true)?;
                objects.call_method("remove", (&o,), Some(&kw))?;
            }
        }

        let created = match p.template.as_str() {
            "studio" => template_studio(py, &bpy)?,
            "outdoor" => template_outdoor(py, &bpy)?,
            "product_shot" => template_product_shot(py, &bpy)?,
            "architectural" => template_architectural(py, &bpy)?,
            "portrait" => template_portrait(py, &bpy)?,
            other => return Err(PyValueError::new_err(format!("Unknown template: {other}"))),
        };

        Ok(json!({ "template": p.template, "created": created }))
    }
);

// ============================================================================
// Lights
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct LightCreate {
    /// Light type. Exactly one of POINT, SUN, SPOT, AREA — the complete
    /// `Light.type` enum on Blender 5.2.2. Case-insensitive. Decides which of the
    /// per-type parameters below apply; the others are ignored.
    pub r#type: String,
    /// Name for the light data-block and object. Defaults to the uppercased type
    /// ("POINT", "AREA", …). Blender appends `.001` etc. on a collision; the
    /// resulting object name is returned.
    pub name: Option<String>,
    /// World-space position as `[x, y, z]` in metres. Default `[0, 0, 5]`. A list
    /// of any other length is ignored. Move it later with obj_update.
    pub location: Option<Vec<f64>>,
    /// Euler rotation as `[x, y, z]` in radians. Default: unrotated. Matters for
    /// SUN (direction only), SPOT and AREA; a POINT light is rotation-invariant.
    /// cam_look_at also works on lights and is easier than picking angles.
    pub rotation: Option<Vec<f64>>,
    /// Light power. Watts for POINT/SPOT/AREA, irradiance in W/m2 for SUN — so
    /// the useful magnitudes differ by orders of magnitude. Defaults here: 1000
    /// for POINT, 5 for SUN/SPOT/AREA (Blender's own data-block default is 10).
    pub energy: Option<f64>,
    /// Light colour as `[r, g, b]`, linear, 0..1. Default `[1, 1, 1]`. A list of
    /// any other length is ignored.
    pub color: Option<Vec<f64>>,
    /// Whether the light casts shadows (`use_shadow`). Default true.
    pub shadow: Option<bool>,
    /// POINT only: `shadow_soft_size`, the emitter's radius in metres — larger
    /// means softer shadows. Default 0.25. Ignored for other types.
    pub radius: Option<f64>,
    /// SUN only: angular diameter of the sun disc in radians (0..pi); larger
    /// means softer shadows. Default 0.526 (about 30 degrees — far wider than the
    /// real sun's 0.0092, which is Blender's own default). Ignored otherwise.
    pub angle: Option<f64>,
    /// SPOT only: full cone angle in radians, 0.0175..pi. Default 0.785 (45
    /// degrees). Ignored for other types.
    pub spot_size: Option<f64>,
    /// SPOT only: cone edge softness, 0..1, where 0 is a hard-edged cone.
    /// Default 0.15. Ignored for other types.
    pub spot_blend: Option<f64>,
    /// AREA only: one of SQUARE, RECTANGLE, DISK, ELLIPSE — the complete
    /// `AreaLight.shape` enum on Blender 5.2.2. Default SQUARE. Ignored otherwise.
    pub shape: Option<String>,
    /// AREA only: emitter size in metres (its X extent for RECTANGLE/ELLIPSE).
    /// Default 1.0. Ignored for other types.
    pub size: Option<f64>,
    /// AREA only, and only when `shape` is RECTANGLE or ELLIPSE: the Y extent in
    /// metres. Default 1.0. Ignored for SQUARE/DISK and for other light types.
    pub size_y: Option<f64>,
}

crate::tool!(
    "light_create",
    "Create a light of type POINT, SUN, SPOT or AREA and link it into the active collection. Which optional parameters apply depends on the type: `radius` for POINT, `angle` for SUN, `spot_size`/`spot_blend` for SPOT, `shape`/`size`/`size_y` for AREA; `energy`, `color`, `shadow`, `location` and `rotation` apply to all. Change an existing light with light_update (which cannot change its type, shape or size_y), or use scene_apply_template for a whole ready-made rig.",
    LightCreate,
    |py: Python<'_>, p: LightCreate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        let ty = p.r#type.to_uppercase();
        let name = p.name.clone().unwrap_or_else(|| ty.clone());

        let light_data = d
            .getattr("lights")?
            .call_method1("new", (name.as_str(), ty.as_str()))?;

        let default_energy = if ty == "POINT" { 1000.0 } else { 5.0 };
        light_data.setattr("energy", p.energy.unwrap_or(default_energy))?;

        let color = p.color.unwrap_or_else(|| vec![1.0, 1.0, 1.0]);
        if color.len() == 3 {
            light_data.setattr("color", PyTuple::new(py, [color[0], color[1], color[2]])?)?;
        }

        match ty.as_str() {
            "POINT" => {
                light_data.setattr("shadow_soft_size", p.radius.unwrap_or(0.25))?;
            }
            "SUN" => {
                light_data.setattr("angle", p.angle.unwrap_or(0.526))?;
            }
            "SPOT" => {
                light_data.setattr("spot_size", p.spot_size.unwrap_or(0.785))?;
                light_data.setattr("spot_blend", p.spot_blend.unwrap_or(0.15))?;
            }
            "AREA" => {
                let shape = p.shape.clone().unwrap_or_else(|| "SQUARE".to_string());
                light_data.setattr("shape", shape.as_str())?;
                light_data.setattr("size", p.size.unwrap_or(1.0))?;
                if shape == "RECTANGLE" || shape == "ELLIPSE" {
                    light_data.setattr("size_y", p.size_y.unwrap_or(1.0))?;
                }
            }
            _ => {}
        }

        light_data.setattr("use_shadow", p.shadow.unwrap_or(true))?;

        let light_name: String = light_data.getattr("name")?.extract()?;
        let light_obj = d
            .getattr("objects")?
            .call_method1("new", (light_name.as_str(), &light_data))?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&light_obj,))?;

        let loc = p.location.unwrap_or_else(|| vec![0.0, 0.0, 5.0]);
        if loc.len() == 3 {
            light_obj.setattr("location", PyTuple::new(py, [loc[0], loc[1], loc[2]])?)?;
        }
        if let Some(rot) = p.rotation
            && rot.len() == 3
        {
            light_obj.setattr(
                "rotation_euler",
                PyTuple::new(py, [rot[0], rot[1], rot[2]])?,
            )?;
        }

        Ok(json!({
            "name": light_obj.getattr("name")?.extract::<String>()?,
            "type": ty,
            "location": py_vec(&light_obj.getattr("location")?)?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct LightUpdate {
    /// Name of an existing LIGHT object. An object of any other type is reported
    /// as "Light not found".
    pub name: String,
    /// New power: watts for POINT/SPOT/AREA, W/m2 for SUN. Omit to leave it.
    pub energy: Option<f64>,
    /// New colour as `[r, g, b]`, linear, 0..1. A list of any other length is
    /// ignored. Omit to leave it.
    pub color: Option<Vec<f64>>,
    /// Whether the light casts shadows (`use_shadow`). Applies to every type.
    /// Omit to leave it.
    pub shadow: Option<bool>,
    /// POINT only: emitter radius in metres (`shadow_soft_size`). Silently
    /// ignored on any other light type.
    pub radius: Option<f64>,
    /// SUN only: angular diameter of the sun disc in radians, 0..pi. Silently
    /// ignored on any other light type.
    pub angle: Option<f64>,
    /// SPOT only: full cone angle in radians, 0.0175..pi. Silently ignored on any
    /// other light type.
    pub spot_size: Option<f64>,
    /// SPOT only: cone edge softness, 0..1. Silently ignored on any other light
    /// type.
    pub spot_blend: Option<f64>,
    /// AREA only: emitter size in metres. Silently ignored on any other light
    /// type. There is no `size_y` or `shape` here — set those at light_create.
    pub size: Option<f64>,
}

crate::tool!(
    "light_update",
    "Change an existing light's properties in place. `energy`, `color` and `shadow` apply to every type; `radius` only to POINT, `angle` only to SUN, `spot_size`/`spot_blend` only to SPOT, `size` only to AREA — a parameter aimed at the wrong type is silently ignored. Cannot change the light's type, area shape or size_y: for those use light_create. Move or rotate it with obj_update.",
    LightUpdate,
    |py: Python<'_>, p: LightUpdate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed(&d, &p.name, "LIGHT", "Light")?;
        let light = obj.getattr("data")?;
        let light_type: String = light.getattr("type")?.extract()?;

        if let Some(e) = p.energy {
            light.setattr("energy", e)?;
        }
        if let Some(c) = &p.color
            && c.len() == 3
        {
            light.setattr("color", PyTuple::new(py, [c[0], c[1], c[2]])?)?;
        }
        if let Some(s) = p.shadow {
            light.setattr("use_shadow", s)?;
        }
        if let Some(r) = p.radius
            && light_type == "POINT"
        {
            light.setattr("shadow_soft_size", r)?;
        }
        if let Some(a) = p.angle
            && light_type == "SUN"
        {
            light.setattr("angle", a)?;
        }
        if let Some(s) = p.spot_size
            && light_type == "SPOT"
        {
            light.setattr("spot_size", s)?;
        }
        if let Some(b) = p.spot_blend
            && light_type == "SPOT"
        {
            light.setattr("spot_blend", b)?;
        }
        if let Some(sz) = p.size
            && light_type == "AREA"
        {
            light.setattr("size", sz)?;
        }

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "type": light_type,
        }))
    }
);

// ============================================================================
// Cameras
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct CamCreate {
    /// Projection: PERSP (default), ORTHO, PANO, or CUSTOM — the complete
    /// `Camera.type` enum on Blender 5.2.2 (CUSTOM projects through a custom
    /// shader and was not in the 4.x enum). Case-insensitive.
    pub r#type: Option<String>,
    /// Name for the camera data-block and object. Default "Camera". Blender
    /// appends `.001` etc. on a collision; the resulting object name is returned.
    pub name: Option<String>,
    /// World-space position as `[x, y, z]` in metres. Default `[0, -10, 5]`. A
    /// list of any other length is ignored.
    pub location: Option<Vec<f64>>,
    /// Euler rotation as `[x, y, z]` in radians. Default: unrotated, which points
    /// the camera straight down -Z. Use cam_look_at to aim it at something
    /// instead of computing angles.
    pub rotation: Option<Vec<f64>>,
    /// Focal length in millimetres, 1..5000. Default 50. Applies to PERSP and
    /// PANO; ORTHO uses `ortho_scale` instead.
    pub lens: Option<f64>,
    /// Near clip plane in metres — geometry closer than this is not drawn.
    /// Default 0.1.
    pub clip_start: Option<f64>,
    /// Far clip plane in metres — geometry beyond this is not drawn. Default 1000.
    pub clip_end: Option<f64>,
    /// Sensor width in millimetres, 1..100. Default 36 (full frame). Together
    /// with `lens` this fixes the field of view.
    pub sensor_width: Option<f64>,
    /// ORTHO only: the viewport's width in metres. Default 6, and written only
    /// when `type` is ORTHO.
    pub ortho_scale: Option<f64>,
    /// Turn on depth of field. Default false. `focus_distance` and `aperture` are
    /// written either way, so a camera can be set up now and its DOF enabled later
    /// with cam_update.
    pub dof_enabled: Option<bool>,
    /// Distance in metres to the focal plane. Applied whether or not `dof_enabled`
    /// is set. Default: Blender's own (10 m), left untouched when omitted.
    pub focus_distance: Option<f64>,
    /// Aperture as an f-stop, 0.1..128 — smaller means more blur. Applied whether or
    /// not `dof_enabled` is set. Default: Blender's own (2.8), left untouched when
    /// omitted.
    pub aperture: Option<f64>,
}

crate::tool!(
    "cam_create",
    "Create a camera (PERSP, ORTHO, PANO or CUSTOM) and link it into the active collection. It does NOT become the scene's render camera — call cam_set_active for that. Aim it with cam_look_at rather than guessing `rotation`, and change it later with cam_update.",
    CamCreate,
    |py: Python<'_>, p: CamCreate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        let ty = p
            .r#type
            .clone()
            .unwrap_or_else(|| "PERSP".to_string())
            .to_uppercase();
        let name = p.name.clone().unwrap_or_else(|| "Camera".to_string());

        let cam_data = d
            .getattr("cameras")?
            .call_method1("new", (name.as_str(),))?;
        cam_data.setattr("type", ty.as_str())?;
        cam_data.setattr("lens", p.lens.unwrap_or(50.0))?;
        cam_data.setattr("clip_start", p.clip_start.unwrap_or(0.1))?;
        cam_data.setattr("clip_end", p.clip_end.unwrap_or(1000.0))?;
        cam_data.setattr("sensor_width", p.sensor_width.unwrap_or(36.0))?;

        if ty == "ORTHO" {
            cam_data.setattr("ortho_scale", p.ortho_scale.unwrap_or(6.0))?;
        }

        // Focus and aperture are written whether or not depth of field is on, the way
        // cam_update already did: they are ordinary camera properties, and dropping a
        // value the caller passed would be a silent no-op reported as success. This
        // also lets a camera be set up first and have its DOF switched on later.
        let dof = cam_data.getattr("dof")?;
        dof.setattr("use_dof", p.dof_enabled.unwrap_or(false))?;
        if let Some(v) = p.focus_distance {
            dof.setattr("focus_distance", v)?;
        }
        if let Some(v) = p.aperture {
            dof.setattr("aperture_fstop", v)?;
        }

        let cam_name: String = cam_data.getattr("name")?.extract()?;
        let cam_obj = d
            .getattr("objects")?
            .call_method1("new", (cam_name.as_str(), &cam_data))?;
        bpy.getattr("context")?
            .getattr("collection")?
            .getattr("objects")?
            .call_method1("link", (&cam_obj,))?;

        let loc = p.location.unwrap_or_else(|| vec![0.0, -10.0, 5.0]);
        if loc.len() == 3 {
            cam_obj.setattr("location", PyTuple::new(py, [loc[0], loc[1], loc[2]])?)?;
        }
        if let Some(rot) = p.rotation
            && rot.len() == 3
        {
            cam_obj.setattr(
                "rotation_euler",
                PyTuple::new(py, [rot[0], rot[1], rot[2]])?,
            )?;
        }

        Ok(json!({
            "name": cam_obj.getattr("name")?.extract::<String>()?,
            "type": ty,
            "location": py_vec(&cam_obj.getattr("location")?)?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct CamUpdate {
    /// Name of an existing CAMERA object. An object of any other type is reported
    /// as "Camera not found".
    pub name: String,
    /// New projection: PERSP, ORTHO, PANO or CUSTOM (Blender 5.2.2's complete
    /// `Camera.type` enum). Case-insensitive. Omit to leave it.
    pub r#type: Option<String>,
    /// Focal length in millimetres, 1..5000. Omit to leave it.
    pub lens: Option<f64>,
    /// Near clip plane in metres. Omit to leave it.
    pub clip_start: Option<f64>,
    /// Far clip plane in metres. Omit to leave it.
    pub clip_end: Option<f64>,
    /// Orthographic viewport width in metres. Written whatever the camera's type,
    /// but only has an effect on an ORTHO camera.
    pub ortho_scale: Option<f64>,
    /// Turn depth of field on or off. Focus distance and aperture are applied
    /// even when this is omitted or false, as in cam_create.
    pub dof_enabled: Option<bool>,
    /// Distance in metres to the focal plane. Omit to leave it.
    pub focus_distance: Option<f64>,
    /// Aperture as an f-stop, 0.1..128. Omit to leave it.
    pub aperture: Option<f64>,
}

crate::tool!(
    "cam_update",
    "Change an existing camera's lens, clipping, projection and depth-of-field settings in place; every parameter is optional and an omitted one is left alone. It does not move the camera (obj_update), aim it (cam_look_at) or make it the render camera (cam_set_active). There is no `sensor_width` here — set that at cam_create.",
    CamUpdate,
    |py: Python<'_>, p: CamUpdate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed(&d, &p.name, "CAMERA", "Camera")?;
        let cam = obj.getattr("data")?;

        if let Some(t) = &p.r#type {
            cam.setattr("type", t.to_uppercase())?;
        }
        if let Some(v) = p.lens {
            cam.setattr("lens", v)?;
        }
        if let Some(v) = p.clip_start {
            cam.setattr("clip_start", v)?;
        }
        if let Some(v) = p.clip_end {
            cam.setattr("clip_end", v)?;
        }
        if let Some(v) = p.ortho_scale {
            cam.setattr("ortho_scale", v)?;
        }
        if let Some(v) = p.dof_enabled {
            cam.getattr("dof")?.setattr("use_dof", v)?;
        }
        if let Some(v) = p.focus_distance {
            cam.getattr("dof")?.setattr("focus_distance", v)?;
        }
        if let Some(v) = p.aperture {
            cam.getattr("dof")?.setattr("aperture_fstop", v)?;
        }

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "type": cam.getattr("type")?.extract::<String>()?,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct CamSetActive {
    /// Name of an existing CAMERA object to make the scene's render camera. An
    /// object of any other type is reported as "Camera not found".
    pub name: String,
}

crate::tool!(
    "cam_set_active",
    "Make a camera the active scene camera — the one rendering and the camera view use. cam_create does not do this, so a freshly created camera needs this call before it renders anything.",
    CamSetActive,
    |py: Python<'_>, p: CamSetActive| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let obj = get_typed(&d, &p.name, "CAMERA", "Camera")?;
        scene(&bpy)?.setattr("camera", &obj)?;
        Ok(json!({ "active_camera": obj.getattr("name")?.extract::<String>()? }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct CamLookAt {
    /// Object to rotate. Any object type, not only cameras — a spot light or an
    /// empty works too.
    pub name: String,
    /// What to aim at: either a JSON string naming another object (its origin is
    /// used) or a JSON array `[x, y, z]` of world-space metres. Anything else is
    /// an error.
    pub target: Value,
}

crate::tool!(
    "cam_look_at",
    "Rotate an object so its -Z axis points at a target, with +Y up — the camera convention, which is why it also aims spot and area lights. Writes `rotation_euler` only: it does not move the object (obj_update) and does not make a camera active (cam_set_active).",
    CamLookAt,
    |py: Python<'_>, p: CamLookAt| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let mathutils = py.import("mathutils")?;

        let obj = d
            .getattr("objects")?
            .call_method1("get", (p.name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "Object not found: {}",
                p.name
            )));
        }

        let target_loc: Bound<'_, PyAny> = match &p.target {
            Value::String(s) => {
                let tobj = d.getattr("objects")?.call_method1("get", (s.as_str(),))?;
                if tobj.is_none() {
                    return Err(PyValueError::new_err(format!(
                        "Target object not found: {s}"
                    )));
                }
                tobj.getattr("location")?
            }
            Value::Array(a) => {
                let nums: Vec<f64> = a.iter().filter_map(|x| x.as_f64()).collect();
                mathutils
                    .getattr("Vector")?
                    .call1((PyTuple::new(py, nums)?,))?
            }
            _ => {
                return Err(PyValueError::new_err(
                    "target must be an object name or [x, y, z]",
                ));
            }
        };

        let direction = target_loc.call_method1("__sub__", (obj.getattr("location")?,))?;
        let rot_quat = direction.call_method1("to_track_quat", ("-Z", "Y"))?;
        let euler = rot_quat.call_method0("to_euler")?;
        obj.setattr("rotation_euler", &euler)?;

        Ok(json!({
            "name": obj.getattr("name")?.extract::<String>()?,
            "target": py_vec(&target_loc)?,
            "rotation": py_vec(&obj.getattr("rotation_euler")?)?,
        }))
    }
);

// ============================================================================
// AI Scene Analysis
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneAnalyze {
    /// Report loose vertices, non-manifold edges and unapplied scale per mesh.
    /// Default true. Walks every polygon of every mesh, so this is the expensive
    /// half on a heavy scene.
    pub check_geometry: Option<bool>,
    /// Report materials with a missing or unloaded image texture, and materials
    /// with no users. Default true.
    pub check_materials: Option<bool>,
    /// Report Boolean modifiers with no target object and Array modifiers whose
    /// object offset is empty. Default true.
    pub check_modifiers: Option<bool>,
    /// Report objects whose action has an inverted frame range. Default true.
    pub check_animation: Option<bool>,
}

crate::tool!(
    "scene_analyze",
    "Survey the whole scene and report what looks wrong: geometry, materials, modifiers, animation, plus a summary and recommendations. Each category can be switched off to make it cheaper. scene_get_stats answers how big the scene is instead; scene_detect_issues is the narrower, per-object geometry check; scene_auto_fix repairs what is found here.",
    SceneAnalyze,
    |py: Python<'_>, p: SceneAnalyze| {
        let bpy = py.import("bpy")?;
        let sc = scene(&bpy)?;
        let d = data(&bpy)?;

        let check_geo = p.check_geometry.unwrap_or(true);
        let check_mat = p.check_materials.unwrap_or(true);
        let check_mod = p.check_modifiers.unwrap_or(true);
        let check_anim = p.check_animation.unwrap_or(true);

        let scene_objects = sc.getattr("objects")?;
        let total_objects = py_len(py, &scene_objects)?;

        let mut mesh_objects: Vec<Bound<'_, PyAny>> = Vec::new();
        let mut total_verts = 0usize;
        let mut total_faces = 0usize;
        for o in PyIterator::from_object(&scene_objects)? {
            let o = o?;
            let ty: String = o.getattr("type")?.extract()?;
            if ty == "MESH" {
                let md = o.getattr("data")?;
                if !md.is_none() {
                    total_verts += py_len(py, &md.getattr("vertices")?)?;
                    total_faces += py_len(py, &md.getattr("polygons")?)?;
                }
                mesh_objects.push(o);
            }
        }

        let mut geometry_issues = Vec::new();
        if check_geo {
            for obj in &mesh_objects {
                let mesh = obj.getattr("data")?;
                let mut issues: Vec<String> = Vec::new();

                let verts_len = py_len(py, &mesh.getattr("vertices")?)?;
                let scan = scan_mesh(&mesh)?;
                let loose = verts_len.saturating_sub(scan.verts_in_faces);
                if loose > 0 {
                    issues.push(format!("{loose} loose vertices"));
                }
                if scan.non_manifold_edges > 0 {
                    issues.push(format!("{} non-manifold edges", scan.non_manifold_edges));
                }
                if !scale_applied(obj)? {
                    issues.push("Unapplied scale".to_string());
                }

                if !issues.is_empty() {
                    geometry_issues.push(json!({
                        "object": obj.getattr("name")?.extract::<String>()?,
                        "issues": issues,
                    }));
                }
            }
        }

        let material_count = py_len(py, &d.getattr("materials")?)?;
        let mut material_issues = Vec::new();
        if check_mat {
            for mat in PyIterator::from_object(&d.getattr("materials")?)? {
                let mat = mat?;
                let mut issues: Vec<String> = Vec::new();
                let use_nodes: bool = mat.getattr("use_nodes")?.extract()?;
                if use_nodes {
                    for node in
                        PyIterator::from_object(&mat.getattr("node_tree")?.getattr("nodes")?)?
                    {
                        let node = node?;
                        let ntype: String = node.getattr("type")?.extract()?;
                        if ntype != "TEX_IMAGE" {
                            continue;
                        }
                        let image = node.getattr("image")?;
                        if image.is_none() {
                            issues.push("Missing image texture".to_string());
                        } else {
                            let has_data: bool = image.getattr("has_data")?.extract()?;
                            if !has_data {
                                let iname: String = image.getattr("name")?.extract()?;
                                issues.push(format!("Texture not loaded: {iname}"));
                            }
                        }
                    }
                }
                let users: i64 = mat.getattr("users")?.extract()?;
                if users == 0 {
                    issues.push("Unused material".to_string());
                }
                if !issues.is_empty() {
                    material_issues.push(json!({
                        "material": mat.getattr("name")?.extract::<String>()?,
                        "issues": issues,
                    }));
                }
            }
        }

        let mut modifier_issues = Vec::new();
        if check_mod {
            for obj in &mesh_objects {
                let mut issues: Vec<String> = Vec::new();
                for m in PyIterator::from_object(&obj.getattr("modifiers")?)? {
                    let m = m?;
                    let mtype: String = m.getattr("type")?.extract()?;
                    let mname: String = m.getattr("name")?.extract()?;
                    if mtype == "BOOLEAN" && m.getattr("object")?.is_none() {
                        issues.push(format!("Boolean '{mname}' has no target"));
                    }
                    if mtype == "ARRAY" {
                        let use_offset: bool = m.getattr("use_object_offset")?.extract()?;
                        if use_offset && m.getattr("offset_object")?.is_none() {
                            issues.push(format!("Array '{mname}' missing offset object"));
                        }
                    }
                }
                if !issues.is_empty() {
                    modifier_issues.push(json!({
                        "object": obj.getattr("name")?.extract::<String>()?,
                        "issues": issues,
                    }));
                }
            }
        }

        let mut animation_issues = Vec::new();
        if check_anim {
            for o in PyIterator::from_object(&scene_objects)? {
                let o = o?;
                let anim_data = o.getattr("animation_data")?;
                if anim_data.is_none() {
                    continue;
                }
                let action = anim_data.getattr("action")?;
                if action.is_none() {
                    continue;
                }
                let range = py_vec(&action.getattr("frame_range")?)?;
                if range.len() == 2 && range[1] < range[0] {
                    animation_issues.push(json!({
                        "object": o.getattr("name")?.extract::<String>()?,
                        "issues": ["Invalid frame range"],
                    }));
                }
            }
        }

        let mut recommendations: Vec<&str> = Vec::new();
        if total_faces > 1_000_000 {
            recommendations.push("High polygon count - consider decimation or LODs");
        }
        if material_count > 50 {
            recommendations.push("Many materials - consider consolidation");
        }
        if !geometry_issues.is_empty() {
            recommendations.push("Use scene_auto_fix to resolve geometry issues");
        }

        Ok(json!({
            "summary": {
                "total_objects": total_objects,
                "mesh_objects": mesh_objects.len(),
                "total_vertices": total_verts,
                "total_faces": total_faces,
                "materials": material_count,
                "textures": py_len(py, &d.getattr("images")?)?,
            },
            "geometry_issues": geometry_issues,
            "material_issues": material_issues,
            "modifier_issues": modifier_issues,
            "animation_issues": animation_issues,
            "recommendations": recommendations,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneDetectIssues {
    /// Names of the objects to check, as a JSON array. Omit to check every object
    /// in the scene. A name that does not exist is skipped silently; non-mesh
    /// objects are skipped too. Image and orphan-data checks always cover the
    /// whole file regardless of this list.
    pub objects: Option<Vec<String>>,
}

crate::tool!(
    "scene_detect_issues",
    "Per-mesh geometry audit plus file-wide checks: non-manifold edges (an edge shared by more than two faces), boundary edges (shared by one — normal for an open mesh, not an error), inverted normals (negative enclosed volume, only meaningful on a closed manifold mesh), missing image files and orphan data. Narrower and cheaper than scene_analyze; fix the geometry findings with scene_auto_fix and the orphans with scene_purge_unused.",
    SceneDetectIssues,
    |py: Python<'_>, p: SceneDetectIssues| {
        let bpy = py.import("bpy")?;
        let sc = scene(&bpy)?;
        let d = data(&bpy)?;

        let mut objects: Vec<Bound<'_, PyAny>> = Vec::new();
        if let Some(names) = p.objects {
            let dobjs = d.getattr("objects")?;
            for n in names {
                let o = dobjs.call_method1("get", (n.as_str(),))?;
                if !o.is_none() {
                    objects.push(o);
                }
            }
        } else {
            for o in PyIterator::from_object(&sc.getattr("objects")?)? {
                objects.push(o?);
            }
        }

        let mut non_manifold = Vec::new();
        let mut boundary_edges = Vec::new();
        let mut inverted_normals = Vec::new();

        for obj in &objects {
            let ty: String = obj.getattr("type")?.extract()?;
            if ty != "MESH" {
                continue;
            }
            let mesh = obj.getattr("data")?;
            if mesh.is_none() {
                continue;
            }

            let scan = scan_mesh(&mesh)?;
            let name: String = obj.getattr("name")?.extract()?;
            if scan.non_manifold_edges > 0 {
                non_manifold.push(json!({ "object": name, "edges": scan.non_manifold_edges }));
            }
            if scan.boundary_edges > 0 {
                boundary_edges.push(json!({ "object": name, "edges": scan.boundary_edges }));
            }
            // The enclosed-volume sign is only a valid winding check on a closed,
            // manifold mesh - anything with a boundary or a non-manifold edge is
            // skipped rather than reported as a false positive.
            if scan.non_manifold_edges == 0
                && scan.boundary_edges == 0
                && mesh_signed_volume(&mesh)? < 0.0
            {
                inverted_normals.push(json!({ "object": name }));
            }
        }

        let mut missing_textures = Vec::new();
        for img in PyIterator::from_object(&d.getattr("images")?)? {
            let img = img?;
            let source: String = img.getattr("source")?.extract()?;
            let has_data: bool = img.getattr("has_data")?.extract()?;
            if source == "FILE" && !has_data {
                missing_textures.push(json!({
                    "image": img.getattr("name")?.extract::<String>()?,
                    "path": img.getattr("filepath")?.extract::<String>()?,
                }));
            }
        }

        let mut orphan_data = Vec::new();
        for mesh in PyIterator::from_object(&d.getattr("meshes")?)? {
            let mesh = mesh?;
            let users: i64 = mesh.getattr("users")?.extract()?;
            if users == 0 {
                orphan_data.push(json!({
                    "type": "MESH",
                    "name": mesh.getattr("name")?.extract::<String>()?,
                }));
            }
        }
        for mat in PyIterator::from_object(&d.getattr("materials")?)? {
            let mat = mat?;
            let users: i64 = mat.getattr("users")?.extract()?;
            if users == 0 {
                orphan_data.push(json!({
                    "type": "MATERIAL",
                    "name": mat.getattr("name")?.extract::<String>()?,
                }));
            }
        }

        Ok(json!({
            "non_manifold": non_manifold,
            "boundary_edges": boundary_edges,
            "inverted_normals": inverted_normals,
            "missing_textures": missing_textures,
            "orphan_data": orphan_data,
        }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneAutoFix {
    /// Recalculate normals to face outward (`mesh.normals_make_consistent`,
    /// inside=false). Default false — nothing happens unless at least one fix
    /// is switched on.
    pub fix_normals: Option<bool>,
    /// Apply object scale into the mesh, leaving scale at (1,1,1). Default false.
    /// Skipped for objects already within 0.001 of unit scale. Destructive.
    pub fix_scale: Option<bool>,
    /// Merge vertices closer than `merge_distance`. Default false. Destructive —
    /// it removes geometry.
    pub remove_doubles: Option<bool>,
    /// Merge threshold in metres for `remove_doubles`. Default 0.0001. Ignored
    /// unless `remove_doubles` is true.
    pub merge_distance: Option<f64>,
    /// Names of the mesh objects to fix, as a JSON array. Omit to use the
    /// currently selected mesh objects, and if nothing is selected, every mesh in
    /// the scene. A name that does not exist is skipped silently.
    pub objects: Option<Vec<String>>,
}

crate::tool!(
    "scene_auto_fix",
    "Repair the geometry problems scene_analyze and scene_detect_issues report: recalculate normals, apply scale, merge doubles. Every fix is off by default, so a call with no flags does nothing; each one is destructive and toggles the object through Edit mode. Reports which objects each fix actually changed.",
    SceneAutoFix,
    |py: Python<'_>, p: SceneAutoFix| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        let fix_normals = p.fix_normals.unwrap_or(false);
        let fix_scale = p.fix_scale.unwrap_or(false);
        let remove_doubles = p.remove_doubles.unwrap_or(false);
        let merge_dist = p.merge_distance.unwrap_or(0.0001);

        let mut objects: Vec<Bound<'_, PyAny>> = Vec::new();
        if let Some(names) = &p.objects {
            let dobjs = d.getattr("objects")?;
            for n in names {
                let o = dobjs.call_method1("get", (n.as_str(),))?;
                if !o.is_none() {
                    objects.push(o);
                }
            }
        } else {
            for o in PyIterator::from_object(&bpy.getattr("context")?.getattr("selected_objects")?)?
            {
                let o = o?;
                let ty: String = o.getattr("type")?.extract()?;
                if ty == "MESH" {
                    objects.push(o);
                }
            }
        }
        if objects.is_empty() {
            for o in PyIterator::from_object(&scene(&bpy)?.getattr("objects")?)? {
                let o = o?;
                let ty: String = o.getattr("type")?.extract()?;
                if ty == "MESH" {
                    objects.push(o);
                }
            }
        }

        let mut fixed_normals = Vec::new();
        let mut fixed_scale = Vec::new();
        let mut fixed_doubles = Vec::new();

        let ops = bpy.getattr("ops")?;
        let view_layer = bpy.getattr("context")?.getattr("view_layer")?;

        for obj in &objects {
            let ty: String = obj.getattr("type")?.extract()?;
            if ty != "MESH" {
                continue;
            }

            // Not checked: reset step, and `select_all(DESELECT)` legitimately
            // returns {'CANCELLED'} when nothing was selected to begin with
            // (verified live) — the fixes below are what actually matter.
            let deselect = PyDict::new(py);
            deselect.set_item("action", "DESELECT")?;
            ops.getattr("object")?
                .call_method("select_all", (), Some(&deselect))?;
            obj.call_method1("select_set", (true,))?;
            view_layer.getattr("objects")?.setattr("active", obj)?;

            let name: String = obj.getattr("name")?.extract()?;

            // Each fix below is a per-object batch item: this tool's contract is
            // "fix whatever can be fixed", and the fixed_* lists already report
            // exactly what happened per object, so a decline on one object's fix
            // simply leaves it out of the corresponding list rather than
            // aborting fixes for every other object in the scene.
            if fix_scale && !scale_applied(obj)? {
                let kw = PyDict::new(py);
                kw.set_item("location", false)?;
                kw.set_item("rotation", false)?;
                kw.set_item("scale", true)?;
                let result =
                    ops.getattr("object")?
                        .call_method("transform_apply", (), Some(&kw))?;
                if op_ok(&result, &format!("apply scale on {name}")).is_ok() {
                    fixed_scale.push(name.clone());
                }
            }

            if fix_normals || remove_doubles {
                let mk = PyDict::new(py);
                mk.set_item("mode", "EDIT")?;
                ops.getattr("object")?
                    .call_method("mode_set", (), Some(&mk))?;

                // Not checked: prep step, always {'FINISHED'} for `SELECT`
                // (verified live) — unlike `DESELECT`, it does not cancel when
                // idempotent.
                let sel = PyDict::new(py);
                sel.set_item("action", "SELECT")?;
                ops.getattr("mesh")?
                    .call_method("select_all", (), Some(&sel))?;

                if fix_normals {
                    let nk = PyDict::new(py);
                    nk.set_item("inside", false)?;
                    let result = ops.getattr("mesh")?.call_method(
                        "normals_make_consistent",
                        (),
                        Some(&nk),
                    )?;
                    if op_ok(&result, &format!("recalculate normals on {name}")).is_ok() {
                        fixed_normals.push(name.clone());
                    }
                }

                if remove_doubles {
                    let rk = PyDict::new(py);
                    rk.set_item("threshold", merge_dist)?;
                    let result =
                        ops.getattr("mesh")?
                            .call_method("remove_doubles", (), Some(&rk))?;
                    if op_ok(&result, &format!("remove doubles on {name}")).is_ok() {
                        fixed_doubles.push(name.clone());
                    }
                }

                let ok = PyDict::new(py);
                ok.set_item("mode", "OBJECT")?;
                ops.getattr("object")?
                    .call_method("mode_set", (), Some(&ok))?;
            }
        }

        Ok(json!({
            "fixed": {
                "normals": fixed_normals,
                "scale": fixed_scale,
                "doubles": fixed_doubles,
            },
            "objects_processed": objects.len(),
        }))
    }
);

// ============================================================================
// Preset System
// ============================================================================
//
// Presets are stored in a scene custom property (`scene["mcp_presets"]`), a
// nested `category -> name -> data` structure - unchanged from the Python
// handler's storage format, so presets saved by the legacy addon remain
// readable. Blender's ID custom properties implement the mapping protocol
// (`__getitem__`/`__setitem__`/`__contains__`) recursively for nested groups
// exactly like a plain dict, so no `bpy`-specific conversion API is needed -
// only the same subscription protocol the original Python handler used.

fn presets_storage<'py>(sc: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
    if !sc.contains("mcp_presets")? {
        sc.set_item("mcp_presets", PyDict::new(sc.py()))?;
    }
    sc.get_item("mcp_presets")
}

fn preset_category<'py>(
    storage: &Bound<'py, PyAny>,
    category: &str,
) -> PyResult<Bound<'py, PyAny>> {
    if !storage.contains(category)? {
        storage.set_item(category, PyDict::new(storage.py()))?;
    }
    storage.get_item(category)
}

fn py_dict_keys(obj: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
    let mut out = Vec::new();
    for k in PyIterator::from_object(&obj.call_method0("keys")?)? {
        out.push(k?.extract::<String>()?);
    }
    Ok(out)
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PresetSave {
    /// Preset name, the key it is stored and retrieved under. Saving over an
    /// existing name in the same category replaces it without warning.
    pub name: String,
    /// Free-form grouping key — there is no fixed list; "render", "material",
    /// "modifier", "physics" are the conventional ones. Created on first use.
    pub category: String,
    /// Any JSON value to store: object, array, string, number or bool. Kept
    /// verbatim; nothing here is applied to the scene.
    pub data: Value,
}

crate::tool!(
    "preset_save",
    "Store an arbitrary JSON blob under category/name in the scene's `mcp_presets` custom property, so it survives in the .blend file. It only records values — it neither reads the current settings nor applies anything. List them with preset_list, remove one with preset_delete.",
    PresetSave,
    |py: Python<'_>, p: PresetSave| {
        let bpy = py.import("bpy")?;
        let sc = scene(&bpy)?;
        let storage = presets_storage(&sc)?;
        let cat = preset_category(&storage, &p.category)?;
        cat.set_item(p.name.as_str(), value_to_py(py, &p.data)?)?;
        // Reassign the top-level property so Blender registers the mutation,
        // mirroring the original handler's `scene["mcp_presets"] = dict(storage)`.
        sc.set_item("mcp_presets", &storage)?;
        Ok(json!({ "saved": p.name, "category": p.category }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PresetList {
    /// Return only this category's preset names. Omit to return every category
    /// mapped to its names. An unknown category returns an empty list, not an
    /// error.
    pub category: Option<String>,
}

crate::tool!(
    "preset_list",
    "List the preset names stored by preset_save in this scene, either across all categories or within one. Returns names only, not the stored data.",
    PresetList,
    |py: Python<'_>, p: PresetList| {
        let bpy = py.import("bpy")?;
        let sc = scene(&bpy)?;
        let storage = presets_storage(&sc)?;

        if let Some(category) = p.category {
            if !storage.contains(category.as_str())? {
                return Ok(json!({ "category": category, "presets": Vec::<String>::new() }));
            }
            let cat = storage.get_item(category.as_str())?;
            return Ok(json!({ "category": category, "presets": py_dict_keys(&cat)? }));
        }

        let mut result = serde_json::Map::new();
        for key in py_dict_keys(&storage)? {
            let cat = storage.get_item(key.as_str())?;
            result.insert(key, json!(py_dict_keys(&cat)?));
        }
        Ok(json!({ "presets": result }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct PresetDelete {
    /// Preset name to delete. A name that is not in the category is an error.
    pub name: String,
    /// Category the preset lives in — the same string given to preset_save.
    pub category: String,
}

crate::tool!(
    "preset_delete",
    "Delete one preset by category and name. Errors when it does not exist; use preset_list first if unsure.",
    PresetDelete,
    |py: Python<'_>, p: PresetDelete| {
        let bpy = py.import("bpy")?;
        let sc = scene(&bpy)?;
        let storage = presets_storage(&sc)?;
        if storage.contains(p.category.as_str())? {
            let cat = storage.get_item(p.category.as_str())?;
            if cat.contains(p.name.as_str())? {
                cat.del_item(p.name.as_str())?;
                sc.set_item("mcp_presets", &storage)?;
                return Ok(json!({ "deleted": p.name, "category": p.category }));
            }
        }
        Err(PyValueError::new_err(format!(
            "Preset not found: {}/{}",
            p.category, p.name
        )))
    }
);
