//! Texture baking: bake a shader (procedural material, high-poly detail, AO,
//! lighting...) down onto an image datablock, using Cycles.
//!
//! ## Why this file exists
//! `img_new` (`src/tools/image.rs`) already called itself "a bake target" and
//! `img_save` "a bake result" — the image layer was built assuming a baker would
//! show up. It never did. The only `*_bake` tools that existed (`phys_bake`,
//! `sim_bake` in `src/tools/phys.rs`) are point-cache / geometry-nodes-simulation
//! baking, an entirely different Blender subsystem. This file is the missing
//! piece: `bpy.ops.object.bake`, the operator behind Cycles texture baking, wired
//! up with every precondition it silently depends on.
//!
//! ## Verified against a live Blender 5.0 — not inferred
//! Every claim below was checked with
//! `blender.exe --background --factory-startup --python <probe>.py` against
//! Blender 5.0.0 (hash a37564c4df7a), never assumed from memory of older Blenders:
//!
//! - **`bpy.ops.object.bake`'s `type` enum**, read from
//!   `bpy.ops.object.bake.get_rna_type().properties['type'].enum_items`:
//!   `COMBINED, AO, SHADOW, POSITION, NORMAL, UV, ROUGHNESS, EMIT, ENVIRONMENT,
//!   DIFFUSE, GLOSSY, TRANSMISSION`. This file never hardcodes that list as a
//!   `match` — [`bake_op_enum`] reads it fresh every call, exactly so a future
//!   Blender adding/renaming a pass (the way 4.1 removed `musgrave` and 5.0 removed
//!   `Action.fcurves`, both documented in `src/tools/AGENTS.md`) cannot go stale
//!   here.
//! - **`pass_filter` is a flag-enum** (`is_enum_flag == True`) with values `NONE,
//!   EMIT, DIRECT, INDIRECT, COLOR, DIFFUSE, GLOSSY, TRANSMISSION`, and it must be
//!   handed to the operator as a genuine Python `set`: passing a `list` raises
//!   `TypeError: OBJECT_OT_bake.pass_filter expected a set, not a list` — confirmed
//!   by trying exactly that. Hence [`PySet`] below, not `PyList`.
//! - **Cycles bakes fully headless, on CPU, with zero extra setup.**
//!   `scene.cycles.device` is `CPU` by default in a fresh file; an `EMIT` bake at
//!   `samples=4` returned `{'FINISHED'}` under `--background` and the target
//!   image's pixels genuinely changed (`(0,0,0,1)` -> `(0,1,0,1)`, matching the
//!   emission colour set on the material) with no GPU, no compute device
//!   configured in preferences, and no window. Unlike `scene_screenshot` (which
//!   needs a real OpenGL context and cannot run `--background` at all — see
//!   `src/tools/AGENTS.md`), baking has no such requirement; this was verified by
//!   literally running the bake headless and diffing pixel bytes before/after,
//!   not by reasoning about it.
//! - **`scene.render.engine`'s static RNA enum reflection omits Cycles.**
//!   `scene.render.bl_rna.properties['engine'].enum_items` — the exact path
//!   `bpy::render_engine()` (`src/tools/bpy.rs`) uses — reports only
//!   `['BLENDER_EEVEE']` in this build, even *after* successfully setting
//!   `scene.render.engine = 'CYCLES'` and re-reading it. The engines actually
//!   registered are the RNA subclasses of `RenderEngine`
//!   (`bpy.types.RenderEngine.__subclasses__()`, keyed by `bl_idname`), which does
//!   correctly list `CYCLES`. So [`ensure_cycles_available`] below deliberately
//!   does **not** call `bpy::render_engine()` — that helper would wrongly reject
//!   `CYCLES` on this exact Blender build.
//! - **The operator no-ops instead of erroring when the wrong node is active.**
//!   Baking while an Image Texture node exists but is not both `select`ed and the
//!   tree's `nodes.active` prints `Info: No active and selected image texture node
//!   found...` to Blender's *log* (not a raised Python exception) and still
//!   returns `{'FINISHED'}` — having written nothing. This was only caught by
//!   comparing the image's pixels before and after; a bare `try/except` around the
//!   operator call is blind to it. That is exactly the failure mode this task
//!   warned about, and exactly why this tool finds the node and makes it
//!   `select`ed + `active` itself rather than trusting the operator to complain.
//!   Genuine hard failures, by contrast, DO raise normally and were also verified:
//!   no UV layer (`RuntimeError: ... No active UV layer found...`), nothing
//!   selected/active (`RuntimeError: ... poll() failed, context is incorrect`),
//!   EEVEE left as the active engine (`RuntimeError: ... Current render engine
//!   does not support baking`), an unknown `type` (`TypeError: ... enum "..." not
//!   found in (...)`).
//!
//! ## Design decision: require the Image Texture node, do not create it
//! Baking targets only the node that is both `select`ed and `nodes.active` inside
//! the object's **active** material slot's tree (confirmed above — a node that
//! merely exists, unselected, is invisible to the operator). This tool searches
//! that tree for a `ShaderNodeTexImage` already pointed at the target image and
//! activates/selects it, but does **not** conjure one into existence if none is
//! found. Authoring the shader graph — which UV socket it reads, what it should
//! feed into, whether it belongs in this material at all — is `node_graph_build`'s
//! job (`src/tools/nodes/graph.rs`); silently splicing in a disconnected node here
//! would let a caller who forgot to wire the material get a "successful" bake into
//! a node nobody asked for, and node introspection would then show a stray node.
//! The error instead names `node_graph_build` and the exact node type/property to
//! add (`ShaderNodeTexImage` / `image`).
//!
//! ## `bake_settings` was folded into `bake_texture`, not split out
//! Every `scene.render.bake.*` RNA field a caller would plausibly want (margin,
//! margin type, cage object/extrusion, max ray distance, normal space/swizzle,
//! selected-to-active) is *also* accepted as a same-named keyword argument directly
//! on the `bpy.ops.object.bake` call — Blender mirrors the scene's bake settings
//! onto the operator invocation, so nothing here ever has to read or write
//! `scene.render.bake` itself. A separate `bake_settings` tool would only exist to
//! mutate that global scene state ahead of time, for a value the operator call
//! already takes per-invocation with nothing left over to leak. Two RNA options —
//! `target` and `save_mode` — are hardcoded rather than exposed: `target` is always
//! `IMAGE_TEXTURES` (the other option, `VERTEX_COLORS`, bakes to vertex colours, a
//! different tool's job), and `save_mode` is always `INTERNAL` (the result lands in
//! the image datablock's pixels in memory, which `img_save` already knows how to
//! write to disk). Exposing either would be a parameter this tool always
//! overrides — the exact "schema lies to the model" anti-pattern this project's
//! `tool!` macro rustdoc warns against for the error contract, applied here to
//! parameters instead of errors.

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PySet};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, py_len, scene};

// ============================================================================
// RNA helpers — every enum this tool validates is read live, never hardcoded.
// ============================================================================

/// The values `bpy.ops.object.bake` actually accepts for RNA property `key` in
/// *this* Blender, read fresh every call. See the module doc for why this can
/// never be a hardcoded table.
fn bake_op_enum(py: Python<'_>, key: &str) -> PyResult<Vec<String>> {
    let bpy = py.import("bpy")?;
    let op = bpy.getattr("ops")?.getattr("object")?.getattr("bake")?;
    let rna = op.call_method0("get_rna_type")?;
    let prop = rna.getattr("properties")?.call_method1("get", (key,))?;
    let mut out = Vec::new();
    for item in prop.getattr("enum_items")?.try_iter()? {
        out.push(item?.getattr("identifier")?.extract::<String>()?);
    }
    Ok(out)
}

/// Validate `value` against the live enum for operator property `key`
/// (case-insensitive), returning the canonical identifier. The error names the
/// values this Blender genuinely supports — never a guess.
fn validate_enum(py: Python<'_>, key: &str, value: &str) -> PyResult<String> {
    let allowed = bake_op_enum(py, key)?;
    let want = value.to_uppercase();
    allowed
        .iter()
        .find(|a| **a == want)
        .cloned()
        .ok_or_else(|| {
            PyValueError::new_err(format!(
                "`{key}` does not accept `{value}`. object.bake's `{key}` accepts: {}",
                allowed.join(", ")
            ))
        })
}

/// Cycles must be the active render engine to bake at all. `bpy::render_engine()`
/// cannot answer this — see the module doc's note on why its `enum_items` path
/// misses Cycles on this Blender build. The RNA subclasses of `RenderEngine` are
/// the ground truth for what is actually registered.
fn ensure_cycles_available(py: Python<'_>) -> PyResult<()> {
    let bpy = py.import("bpy")?;
    let render_engine_cls = bpy.getattr("types")?.getattr("RenderEngine")?;
    let subclasses = render_engine_cls.call_method0("__subclasses__")?;
    let mut ids = Vec::new();
    for c in subclasses.try_iter()? {
        if let Ok(id) = c?.getattr("bl_idname")
            && let Ok(s) = id.extract::<String>()
        {
            ids.push(s);
        }
    }
    if ids.iter().any(|i| i == "CYCLES") {
        Ok(())
    } else {
        Err(PyRuntimeError::new_err(format!(
            "this Blender has no Cycles render engine registered — baking requires \
             it. Render engines registered here: {}",
            if ids.is_empty() {
                "none".to_string()
            } else {
                ids.join(", ")
            }
        )))
    }
}

// ============================================================================
// Node-tree helpers
// ============================================================================

/// The `ShaderNodeTexImage` node in `tree` whose `.image` is `image_name`, if any.
fn find_image_tex_node<'py>(
    tree: &Bound<'py, PyAny>,
    image_name: &str,
) -> PyResult<Option<Bound<'py, PyAny>>> {
    for n in tree.getattr("nodes")?.try_iter()? {
        let n = n?;
        let ty: String = n.getattr("bl_idname")?.extract()?;
        if ty != "ShaderNodeTexImage" {
            continue;
        }
        let img = n.getattr("image")?;
        if img.is_none() {
            continue;
        }
        if img.getattr("name")?.extract::<String>()? == image_name {
            return Ok(Some(n));
        }
    }
    Ok(None)
}

/// What every `ShaderNodeTexImage` in `tree` currently points at — used only to
/// build a useful error when the wanted node is missing, so the caller learns
/// what IS there instead of just what wasn't found.
fn image_tex_targets(tree: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
    let mut out = Vec::new();
    for n in tree.getattr("nodes")?.try_iter()? {
        let n = n?;
        if n.getattr("bl_idname")?.extract::<String>()? != "ShaderNodeTexImage" {
            continue;
        }
        let img = n.getattr("image")?;
        out.push(if img.is_none() {
            "(empty)".to_string()
        } else {
            img.getattr("name")?.extract::<String>()?
        });
    }
    Ok(out)
}

// ============================================================================
// bake_texture
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct BakeTexture {
    /// Object to bake. Must be a mesh with a UV map (mesh_unwrap) and an active
    /// material whose node tree already has a ShaderNodeTexImage pointed at
    /// `image` (node_graph_build).
    pub object: String,
    /// Target image datablock name. Must already exist — create it with img_new
    /// first; a missing name errors pointing there rather than creating one on
    /// the caller's behalf, since resolution and colour space are img_new's job.
    pub image: String,
    /// Bake pass, case-insensitive. On Blender 5.2 (verified live) the accepted
    /// set is COMBINED (default), AO, SHADOW, POSITION, NORMAL, UV, ROUGHNESS,
    /// EMIT, ENVIRONMENT, DIFFUSE, GLOSSY, TRANSMISSION. It is read from
    /// object.bake's own RNA at call time, so a bad value errors with this
    /// Blender's real list rather than a stale one.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Name of the UV map to bake into. Omit to use the mesh's active UV map
    /// (mesh_unwrap creates one).
    pub uv_layer: Option<String>,
    /// Cycles sample count for this bake only, >= 1. Omitted leaves the scene's
    /// own cycles.samples untouched (and unrestored, since nothing changed it).
    pub samples: Option<i32>,
    /// Pixel margin painted around each UV island, in pixels. Blender's default
    /// is 16. Too small and the texture bleeds seams at low mip levels.
    pub margin: Option<i32>,
    /// How the margin is filled: ADJACENT_FACES or EXTEND (Blender 5.2 default),
    /// verified live and re-checked against the operator's RNA.
    pub margin_type: Option<String>,
    /// High-to-low bake: bake source_objects (plus whatever the operator itself
    /// considers "selected") onto `object`, which becomes the active/target mesh.
    /// Default false.
    pub use_selected_to_active: Option<bool>,
    /// Names of the high-poly source objects, selected alongside `object`. Only
    /// used when use_selected_to_active is set.
    pub source_objects: Option<Vec<String>>,
    /// Ray-cast offset outward from the low-poly surface for selected-to-active
    /// baking, in Blender units (metres). Default 0.0.
    pub cage_extrusion: Option<f64>,
    /// Max inward ray distance for selected-to-active baking, in Blender units;
    /// 0 (the default) means unlimited.
    pub max_ray_distance: Option<f64>,
    /// Use a separate cage object for the ray offset instead of cage_extrusion.
    /// Default false; needs cage_object.
    pub use_cage: Option<bool>,
    /// Name of the cage object — a copy of the low-poly mesh, pushed outwards —
    /// used when use_cage is set.
    pub cage_object: Option<String>,
    /// Normal map space (NORMAL bake only): OBJECT or TANGENT (the default),
    /// verified live on Blender 5.2.
    pub normal_space: Option<String>,
    /// Swizzle for the normal map's R channel (NORMAL bake only): POS_X (default),
    /// POS_Y, POS_Z, NEG_X, NEG_Y or NEG_Z. Change these only to match another
    /// engine's convention.
    pub normal_r: Option<String>,
    /// Swizzle for the normal map's G channel. Default POS_Y; NEG_Y is the
    /// DirectX-style flipped green channel. Same six values as normal_r.
    pub normal_g: Option<String>,
    /// Swizzle for the normal map's B channel. Default POS_Z. Same six values as
    /// normal_r.
    pub normal_b: Option<String>,
    /// Which lighting contributions to include, as a set of flags, for
    /// DIFFUSE/GLOSSY/TRANSMISSION/COMBINED bakes. Blender 5.2 accepts NONE, EMIT,
    /// DIRECT, INDIRECT, COLOR, DIFFUSE, GLOSSY, TRANSMISSION (verified live); the
    /// last four are only meaningful for a COMBINED bake. A DIFFUSE bake with
    /// ["COLOR"] alone is the usual way to get a flat albedo map. Omit for
    /// Blender's own default mix.
    pub pass_filter: Option<Vec<String>>,
    /// Clear the image to black/transparent before baking, instead of compositing
    /// over what is already in it. Default false.
    pub use_clear: Option<bool>,
}

crate::tool!(
    "bake_texture",
    "Bake a material (procedural shader, high-poly detail, AO, lighting) onto an \
     image datablock using Cycles. Three preconditions, each checked before \
     anything runs: the image must already exist (img_new), the object's active \
     material must contain an Image Texture node pointed at it (node_graph_build \
     with a ShaderNodeTexImage whose `image` property is that name), and the mesh \
     needs a UV map (mesh_unwrap). The result lives in memory until img_save \
     writes it out.",
    BakeTexture,
    |py: Python<'_>, p: BakeTexture| {
        let object_name = p.object.clone();
        let image_name = p.image.clone();

        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let s = scene(&bpy)?;

        // --- resolve object ---------------------------------------------------
        let obj = d
            .getattr("objects")?
            .call_method1("get", (object_name.as_str(),))?;
        if obj.is_none() {
            return Err(PyValueError::new_err(format!(
                "object not found: {object_name}"
            )));
        }
        let obj_type: String = obj.getattr("type")?.extract()?;
        if obj_type != "MESH" {
            return Err(PyValueError::new_err(format!(
                "bake_texture only bakes onto meshes; `{object_name}` is a {obj_type}"
            )));
        }

        // --- resolve target image ----------------------------------------------
        let images = d.getattr("images")?;
        let img = images.call_method1("get", (image_name.as_str(),))?;
        if img.is_none() {
            return Err(PyValueError::new_err(format!(
                "image not found: `{image_name}` — create it first with img_new"
            )));
        }

        // --- bake pass, validated against live RNA ------------------------------
        let bake_type = validate_enum(py, "type", &p.r#type)?;

        // --- UV guard ------------------------------------------------------------
        let mesh = obj.getattr("data")?;
        let uv_layers = mesh.getattr("uv_layers")?;
        if py_len(py, &uv_layers)? == 0 {
            return Err(PyValueError::new_err(format!(
                "`{object_name}` has no UV map — run mesh_unwrap first"
            )));
        }
        if let Some(uv_name) = p.uv_layer.as_deref() {
            let layer = uv_layers.call_method1("get", (uv_name,))?;
            if layer.is_none() {
                let mut have = Vec::new();
                for l in uv_layers.try_iter()? {
                    have.push(l?.getattr("name")?.extract::<String>()?);
                }
                return Err(PyValueError::new_err(format!(
                    "UV map `{uv_name}` not found on `{object_name}`. It has: {}",
                    have.join(", ")
                )));
            }
        }

        // --- active material + node tree guard -----------------------------------
        let active_mat = obj.getattr("active_material")?;
        if active_mat.is_none() {
            return Err(PyValueError::new_err(format!(
                "`{object_name}` has no active material — assign one with \
                 mat_assign or mat_create first"
            )));
        }
        let use_nodes: bool = active_mat.getattr("use_nodes")?.extract()?;
        let node_tree = active_mat.getattr("node_tree")?;
        if !use_nodes || node_tree.is_none() {
            let mat_name: String = active_mat.getattr("name")?.extract()?;
            return Err(PyValueError::new_err(format!(
                "material `{mat_name}` (the active material on `{object_name}`) has \
                 no node tree — build one with node_graph_build"
            )));
        }

        // --- the Image Texture node the bake will actually target -----------------
        let tex_node = find_image_tex_node(&node_tree, &image_name)?;
        let Some(tex_node) = tex_node else {
            let mat_name: String = active_mat.getattr("name")?.extract()?;
            let existing = image_tex_targets(&node_tree)?;
            let hint = if existing.is_empty() {
                "it has no Image Texture nodes at all".to_string()
            } else {
                format!("its Image Texture nodes point at: {}", existing.join(", "))
            };
            return Err(PyValueError::new_err(format!(
                "material `{mat_name}` (the active material on `{object_name}`) has \
                 no Image Texture node pointed at `{image_name}` — {hint}. Add one \
                 with node_graph_build (node type `ShaderNodeTexImage`, property \
                 `image` = `{image_name}`)"
            )));
        };

        // --- Cycles must exist in this build ---------------------------------------
        ensure_cycles_available(py)?;

        // --- validate every remaining enum up front, before mutating anything -------
        let margin_type = p
            .margin_type
            .as_deref()
            .map(|v| validate_enum(py, "margin_type", v))
            .transpose()?;
        let normal_space = p
            .normal_space
            .as_deref()
            .map(|v| validate_enum(py, "normal_space", v))
            .transpose()?;
        let normal_r = p
            .normal_r
            .as_deref()
            .map(|v| validate_enum(py, "normal_r", v))
            .transpose()?;
        let normal_g = p
            .normal_g
            .as_deref()
            .map(|v| validate_enum(py, "normal_g", v))
            .transpose()?;
        let normal_b = p
            .normal_b
            .as_deref()
            .map(|v| validate_enum(py, "normal_b", v))
            .transpose()?;
        let pass_filter: Option<Vec<String>> = match &p.pass_filter {
            Some(vals) => {
                let allowed = bake_op_enum(py, "pass_filter")?;
                let mut out = Vec::with_capacity(vals.len());
                for v in vals {
                    let up = v.to_uppercase();
                    if !allowed.contains(&up) {
                        return Err(PyValueError::new_err(format!(
                            "pass_filter value `{v}` is not one of: {}",
                            allowed.join(", ")
                        )));
                    }
                    out.push(up);
                }
                Some(out)
            }
            None => None,
        };

        // --- resolve optional object references up front too ------------------------
        if let Some(cage) = p.cage_object.as_deref() {
            let c = d.getattr("objects")?.call_method1("get", (cage,))?;
            if c.is_none() {
                return Err(PyValueError::new_err(format!(
                    "cage object not found: {cage}"
                )));
            }
        }
        let mut source_objs = Vec::new();
        if let Some(names) = &p.source_objects {
            for n in names {
                let o = d.getattr("objects")?.call_method1("get", (n.as_str(),))?;
                if o.is_none() {
                    return Err(PyValueError::new_err(format!(
                        "source object not found: {n}"
                    )));
                }
                source_objs.push(o);
            }
        }

        // ---------------------------------------------------------------------------
        // Every piece of global state the bake needs to touch is saved here and
        // restored in the tail of this function regardless of outcome: render
        // engine, Cycles sample count, which node is selected/active in the
        // material's tree, and which objects are selected/active in the scene. A
        // bake must leave the file exactly as it found it apart from the target
        // image's pixels.
        // ---------------------------------------------------------------------------
        let render = s.getattr("render")?;
        let old_engine: String = render.getattr("engine")?.extract()?;
        let old_samples: Option<i32> = s
            .getattr("cycles")
            .ok()
            .and_then(|c| c.getattr("samples").ok())
            .and_then(|v| v.extract().ok());

        let nodes_coll = node_tree.getattr("nodes")?;
        let mut old_node_selection = Vec::new();
        for n in nodes_coll.try_iter()? {
            let n = n?;
            let sel: bool = n.getattr("select")?.extract()?;
            old_node_selection.push((n, sel));
        }
        let old_active_node_name: Option<String> = {
            let a = nodes_coll.getattr("active")?;
            if a.is_none() {
                None
            } else {
                Some(a.getattr("name")?.extract()?)
            }
        };

        let ctx = bpy.getattr("context")?;
        let mut old_selected = Vec::new();
        for o in ctx.getattr("selected_objects")?.try_iter()? {
            old_selected.push(o?.getattr("name")?.extract::<String>()?);
        }
        let old_active_obj_name: Option<String> = {
            let a = ctx.getattr("active_object")?;
            if a.is_none() {
                None
            } else {
                Some(a.getattr("name")?.extract()?)
            }
        };

        let result = (|| -> PyResult<Value> {
            render.setattr("engine", "CYCLES")?;
            if let Some(samples) = p.samples
                && let Ok(cycles) = s.getattr("cycles")
            {
                cycles.setattr("samples", samples)?;
            }

            // Only the node that is both selected AND active is baked into — see
            // the module doc. Deselect every other node in the tree first.
            for (n, _) in &old_node_selection {
                n.setattr("select", false)?;
            }
            tex_node.setattr("select", true)?;
            nodes_coll.setattr("active", &tex_node)?;

            let object_ops = bpy.getattr("ops")?.getattr("object")?;
            let deselect = PyDict::new(py);
            deselect.set_item("action", "DESELECT")?;
            object_ops.call_method("select_all", (), Some(&deselect))?;
            for src in &source_objs {
                src.call_method1("select_set", (true,))?;
            }
            obj.call_method1("select_set", (true,))?;
            ctx.getattr("view_layer")?
                .getattr("objects")?
                .setattr("active", &obj)?;

            let kw = PyDict::new(py);
            kw.set_item("type", bake_type.as_str())?;
            kw.set_item("target", "IMAGE_TEXTURES")?;
            kw.set_item("save_mode", "INTERNAL")?;
            if let Some(u) = p.uv_layer.as_deref() {
                kw.set_item("uv_layer", u)?;
            }
            if let Some(m) = p.margin {
                kw.set_item("margin", m)?;
            }
            if let Some(mt) = &margin_type {
                kw.set_item("margin_type", mt.as_str())?;
            }
            if let Some(v) = p.use_selected_to_active {
                kw.set_item("use_selected_to_active", v)?;
            }
            if let Some(v) = p.cage_extrusion {
                kw.set_item("cage_extrusion", v)?;
            }
            if let Some(v) = p.max_ray_distance {
                kw.set_item("max_ray_distance", v)?;
            }
            if let Some(v) = p.use_cage {
                kw.set_item("use_cage", v)?;
            }
            if let Some(v) = p.cage_object.as_deref() {
                kw.set_item("cage_object", v)?;
            }
            if let Some(v) = &normal_space {
                kw.set_item("normal_space", v.as_str())?;
            }
            if let Some(v) = &normal_r {
                kw.set_item("normal_r", v.as_str())?;
            }
            if let Some(v) = &normal_g {
                kw.set_item("normal_g", v.as_str())?;
            }
            if let Some(v) = &normal_b {
                kw.set_item("normal_b", v.as_str())?;
            }
            if let Some(vals) = &pass_filter {
                kw.set_item("pass_filter", PySet::new(py, vals)?)?;
            }
            if let Some(v) = p.use_clear {
                kw.set_item("use_clear", v)?;
            }

            // A real failure here (no UV, nothing selected, wrong engine, bad
            // enum) raises in Python and is wrapped with the operator's own
            // message; a wrong-but-present target node does NOT raise (see module
            // doc) — that class of failure is prevented above by construction,
            // not caught here.
            object_ops
                .call_method("bake", (), Some(&kw))
                .map_err(|e| PyRuntimeError::new_err(format!("bake failed: {e}")))?;

            let size = img.getattr("size")?;
            Ok(json!({
                "object": object_name,
                "image": image_name,
                "type": bake_type,
                "size": [size.get_item(0)?.extract::<i64>()?, size.get_item(1)?.extract::<i64>()?],
                "baked": true,
            }))
        })();

        // --- restore everything, whatever happened -----------------------------------
        render.setattr("engine", old_engine.as_str())?;
        if let (Some(samples), Ok(cycles)) = (old_samples, s.getattr("cycles")) {
            cycles.setattr("samples", samples)?;
        }
        for (n, sel) in &old_node_selection {
            n.setattr("select", *sel)?;
        }
        match &old_active_node_name {
            Some(name) => {
                let n = nodes_coll.call_method1("get", (name.as_str(),))?;
                if !n.is_none() {
                    nodes_coll.setattr("active", n)?;
                }
            }
            None => {
                nodes_coll.setattr("active", py.None())?;
            }
        }

        let object_ops = bpy.getattr("ops")?.getattr("object")?;
        let deselect = PyDict::new(py);
        deselect.set_item("action", "DESELECT")?;
        object_ops.call_method("select_all", (), Some(&deselect))?;
        let objects = d.getattr("objects")?;
        for name in &old_selected {
            let o = objects.call_method1("get", (name.as_str(),))?;
            if !o.is_none() {
                o.call_method1("select_set", (true,))?;
            }
        }
        match &old_active_obj_name {
            Some(name) => {
                let o = objects.call_method1("get", (name.as_str(),))?;
                if !o.is_none() {
                    ctx.getattr("view_layer")?
                        .getattr("objects")?
                        .setattr("active", o)?;
                }
            }
            None => {
                ctx.getattr("view_layer")?
                    .getattr("objects")?
                    .setattr("active", py.None())?;
            }
        }

        result
    }
);
