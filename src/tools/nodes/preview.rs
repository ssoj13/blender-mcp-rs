//! `mat_preview` — render a material in isolation, without touching the user's scene.
//!
//! ## Why the model needs this
//! Before this tool, seeing a material meant creating an object, a light and a
//! camera in the user's own scene and screenshotting the viewport — several calls,
//! and litter left behind if any of them was forgotten. This closes the loop:
//! build the shader graph, look at it, adjust — one call, nothing left over.
//!
//! ## Route chosen: a disposable scene, not `Material.preview`
//! Blender already has a preview system (`Material.preview_ensure()`, the thumbnails
//! in the material browser), and its `preview_render_type` enum even matches most of
//! the shapes offered here — `FLAT`, `SPHERE`, `CUBE`, `CLOTH`. It was checked live
//! on Blender 5.0 and rejected for three concrete reasons:
//!   - no `MONKEY` option (Blender ships `SHADERBALL` instead these days);
//!   - the image size is whatever `ImagePreview.image_size` decides, not something
//!     the caller can ask for — this tool's `resolution` parameter would be a lie;
//!   - no control over engine or sample count, both of which matter for an
//!     iteration-loop tool that has to stay fast.
//!
//! So this builds a real, temporary `bpy.data.scenes` entry with its own object,
//! camera and lights, renders it, and deletes every datablock it created.
//!
//! ## Why the user's scene is never touched
//! `bpy.ops.render.render` takes a `scene` *operator property* (confirmed on the
//! live Blender 5.0 RNA: `scene: STRING`) — passing the temp scene's name renders
//! that scene directly. It never reads or writes `bpy.context.window.scene` or
//! `bpy.context.scene`, so there is nothing to save and restore around it. This was
//! verified end-to-end in `--background`: the active window scene's name was
//! identical before and after the call, and `bpy.data` scene/object/material/mesh/
//! camera/light/world counts were identical before and after — see
//! `docs/mcp-preview-verification.md`-style notes in the tool's implementation
//! report, not reproduced here since it requires a running Blender.
//!
//! Every object and datablock this tool creates (mesh, camera, lights, world, the
//! scene itself) is tracked in `Scratch` and torn down in `Scratch::cleanup`, which
//! runs whether the render succeeded or failed — see the doc comment on `Scratch`.
//!
//! ## Rendering works in `--background`
//! Unlike `scene_screenshot`'s `viewport`/`window` modes (which need a live OpenGL
//! window and cannot run headless — see `src/tools/misc.rs`), `bpy.ops.render.render`
//! is Blender's own offline render path, the same one `blender -b file.blend -f 1`
//! uses from the command line. It was exercised in `--background` for both EEVEE and
//! CYCLES and produced valid PNGs in both cases.

use base64::{Engine, engine::general_purpose::STANDARD as B64};
use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyTuple};
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::tools::bpy::op_ok;

/// Point `obj` at `target` by aiming its local -Z axis there, matching the
/// `to_track_quat('-Z', 'Y')` idiom `cam_look_at` uses in `scene_extra.rs` — the
/// convention Blender cameras and lights both follow (a light with no rotation
/// shines straight down its local -Z).
fn point_at(
    py: Python<'_>,
    obj: &Bound<'_, PyAny>,
    loc: (f64, f64, f64),
    target: (f64, f64, f64),
) -> PyResult<()> {
    obj.setattr("location", PyTuple::new(py, [loc.0, loc.1, loc.2])?)?;
    let vector = py.import("mathutils")?.getattr("Vector")?;
    let loc_v = vector.call1((PyTuple::new(py, [loc.0, loc.1, loc.2])?,))?;
    let target_v = vector.call1((PyTuple::new(py, [target.0, target.1, target.2])?,))?;
    let direction = target_v.call_method1("__sub__", (loc_v,))?;
    let rot_quat = direction.call_method1("to_track_quat", ("-Z", "Y"))?;
    let euler = rot_quat.call_method0("to_euler")?;
    obj.setattr("rotation_euler", &euler)?;
    Ok(())
}

/// Every scratch datablock a single `mat_preview` call has created, torn down
/// together by `cleanup` regardless of where building the scene or rendering it
/// failed. The alternative — a hand-written removal call before every early
/// return — is exactly the kind of place a leaked temp scene hides for months.
///
/// Removing something this call just created should never fail. If it somehow
/// does, that is reported to stderr rather than allowed to mask whatever error the
/// render itself produced — the same trade-off `scene_screenshot` makes for its
/// temp file in `misc.rs`.
#[derive(Default)]
struct Scratch<'py> {
    scene: Option<Bound<'py, PyAny>>,
    worlds: Vec<Bound<'py, PyAny>>,
    objects: Vec<Bound<'py, PyAny>>,
    meshes: Vec<Bound<'py, PyAny>>,
    cameras: Vec<Bound<'py, PyAny>>,
    lights: Vec<Bound<'py, PyAny>>,
    images: Vec<Bound<'py, PyAny>>,
}

/// Remove one datablock, reporting rather than propagating a failure — see the
/// doc comment on `Scratch`.
fn remove_reporting(kind: &str, item: &Bound<'_, PyAny>, r: PyResult<Bound<'_, PyAny>>) {
    if let Err(e) = r {
        let name = item
            .getattr("name")
            .and_then(|n| n.extract::<String>())
            .unwrap_or_default();
        eprintln!("[blender-mcp] mat_preview cleanup: could not remove {kind} `{name}`: {e}");
    }
}

impl<'py> Scratch<'py> {
    /// Tear down every tracked datablock: objects first (nothing else references
    /// them), then the data they pointed at, then the scene last (its own
    /// `collection` is owned by the scene and disappears with it). Every
    /// `bpy.data.*.remove()` here defaults to `do_unlink=True` on Blender 5.0 —
    /// confirmed live, not assumed — so no explicit kwarg is needed.
    fn cleanup(self, py: Python<'py>) -> PyResult<()> {
        let d = py.import("bpy")?.getattr("data")?;

        for obj in &self.objects {
            remove_reporting(
                "object",
                obj,
                d.getattr("objects")?.call_method1("remove", (obj,)),
            );
        }
        for m in &self.meshes {
            remove_reporting("mesh", m, d.getattr("meshes")?.call_method1("remove", (m,)));
        }
        for c in &self.cameras {
            remove_reporting(
                "camera",
                c,
                d.getattr("cameras")?.call_method1("remove", (c,)),
            );
        }
        for l in &self.lights {
            remove_reporting(
                "light",
                l,
                d.getattr("lights")?.call_method1("remove", (l,)),
            );
        }
        for w in &self.worlds {
            remove_reporting(
                "world",
                w,
                d.getattr("worlds")?.call_method1("remove", (w,)),
            );
        }
        for im in &self.images {
            remove_reporting(
                "image",
                im,
                d.getattr("images")?.call_method1("remove", (im,)),
            );
        }
        if let Some(sc) = &self.scene {
            remove_reporting(
                "scene",
                sc,
                d.getattr("scenes")?.call_method1("remove", (sc,)),
            );
        }
        Ok(())
    }
}

/// `EEVEE` (default) or `CYCLES`, mapped to the `RenderSettings.engine` identifier
/// Blender 5.0 actually accepts. Confirmed live: the enum is exactly
/// `('BLENDER_EEVEE', 'BLENDER_WORKBENCH', 'CYCLES')` — there is no
/// `BLENDER_EEVEE_NEXT` on this version (EEVEE Next was renamed back to plain EEVEE
/// for the 5.0 release).
fn resolve_engine(engine: Option<&str>) -> PyResult<&'static str> {
    match engine.unwrap_or("EEVEE").to_uppercase().as_str() {
        "EEVEE" => Ok("BLENDER_EEVEE"),
        "CYCLES" => Ok("CYCLES"),
        other => Err(PyValueError::new_err(format!(
            "unknown engine `{other}`: expected EEVEE or CYCLES"
        ))),
    }
}

/// `sphere` (default), `cube`, `plane`, `cloth` or `monkey`, validated up front so a
/// typo fails before any scratch resource is created.
fn resolve_shape(shape: Option<&str>) -> PyResult<&'static str> {
    match shape.unwrap_or("sphere").to_lowercase().as_str() {
        "sphere" => Ok("sphere"),
        "cube" => Ok("cube"),
        "plane" => Ok("plane"),
        "cloth" => Ok("cloth"),
        "monkey" => Ok("monkey"),
        other => Err(PyValueError::new_err(format!(
            "unknown shape `{other}`: expected sphere, cube, plane, cloth or monkey"
        ))),
    }
}

/// Build `shape` into `bm` with `bmesh.ops` — no `bpy.ops.mesh.primitive_*_add`,
/// because those operators create their object in `bpy.context.collection`, i.e.
/// whatever collection the user currently has active. There is no way to redirect
/// that to the scratch scene without a context override, and background mode has no
/// window to override. Building geometry straight into a `bmesh.new()` sidesteps
/// context entirely — it is pure data, linked into the scratch scene explicitly.
fn build_shape(py: Python<'_>, bm: &Bound<'_, PyAny>, shape: &str) -> PyResult<()> {
    let ops = py.import("bmesh")?.getattr("ops")?;
    let kw = PyDict::new(py);
    kw.set_item("calc_uvs", true)?;
    match shape {
        "cube" => {
            kw.set_item("size", 2.0)?;
            ops.call_method("create_cube", (bm,), Some(&kw))?;
        }
        "plane" => {
            kw.set_item("x_segments", 1)?;
            kw.set_item("y_segments", 1)?;
            kw.set_item("size", 1.0)?;
            ops.call_method("create_grid", (bm,), Some(&kw))?;
        }
        "cloth" => {
            // A densely subdivided plane, not a physics-draped simulation — an
            // actual cloth sim costs seconds per call, and this tool exists to stay
            // inside a fast build -> look -> adjust loop. The extra subdivision
            // (vs. `plane`) gives fabric/subsurface-scattering materials enough
            // geometry to shade correctly without the wait.
            kw.set_item("x_segments", 24)?;
            kw.set_item("y_segments", 24)?;
            kw.set_item("size", 1.2)?;
            ops.call_method("create_grid", (bm,), Some(&kw))?;
        }
        "monkey" => {
            ops.call_method("create_monkey", (bm,), Some(&kw))?;
        }
        _ => {
            kw.set_item("u_segments", 32)?;
            kw.set_item("v_segments", 16)?;
            kw.set_item("radius", 1.0)?;
            ops.call_method("create_uvsphere", (bm,), Some(&kw))?;
        }
    }
    Ok(())
}

/// `(name, energy in watts, location, area light size)`.
type StudioLight = (&'static str, f64, (f64, f64, f64), f64);

/// The default three-point studio rig: a bright key light, a soft fill on the
/// opposite side, and a rim light behind the subject for edge separation. Energies
/// are tuned by eye against all five shapes (see the implementation report), not
/// physically calibrated — this is a preview, not a lighting reference.
const STUDIO_RIG: [StudioLight; 3] = [
    ("key", 1000.0, (2.5, -3.0, 3.0), 2.0),
    ("fill", 300.0, (-3.0, -2.0, 1.5), 3.0),
    ("rim", 500.0, (0.0, 3.0, 2.5), 1.5),
];

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MatPreview {
    /// Name of an existing material to render (see mat_list / mat_create).
    pub material: String,
    /// Preview object, lower-case, exactly one of: sphere (default), cube, plane,
    /// cloth, monkey.
    pub shape: Option<String>,
    /// Square output size in pixels. Default 256, capped at 1024 — this is meant
    /// for a fast iteration loop, not a final render.
    pub resolution: Option<u32>,
    /// Render samples. Low by default (24 for EEVEE, 32 for CYCLES with denoising)
    /// to keep this fast; raise it if the preview looks noisy.
    pub samples: Option<u32>,
    /// Render engine for the preview: EEVEE (default, fast) or CYCLES. These two
    /// are all this tool offers; the set of engines a Blender really has is
    /// dynamic (add-ons register their own), so anything else is rejected with
    /// what this tool accepts rather than silently falling back.
    pub engine: Option<String>,
    /// Path on disk to an equirectangular HDRI image to light the preview with.
    /// Omit for the built-in three-point studio light rig.
    pub hdri: Option<String>,
}

crate::tool!(
    "mat_preview",
    "Render a material on a sphere/cube/plane/cloth/monkey with studio lighting and \
     return the image directly. Nothing is added to or changed in the user's scene: \
     the render happens in a disposable temporary scene that is deleted \
     afterwards. The way to SEE what mat_create / mat_update / node_graph_build \
     actually produced, without touching the user's render settings.",
    MatPreview,
    |py: Python<'_>, p: MatPreview| {
        let bpy = py.import("bpy")?;
        let d = bpy.getattr("data")?;

        // Fail fast on bad input, before creating anything.
        let mat = d
            .getattr("materials")?
            .call_method1("get", (p.material.as_str(),))?;
        if mat.is_none() {
            return Err(PyValueError::new_err(format!(
                "material not found: {}",
                p.material
            )));
        }
        let engine = resolve_engine(p.engine.as_deref())?;
        let shape = resolve_shape(p.shape.as_deref())?;
        let resolution = p.resolution.unwrap_or(256).clamp(1, 1024);
        let samples = p
            .samples
            .unwrap_or(if engine == "CYCLES" { 32 } else { 24 })
            .max(1);

        let mut scratch = Scratch::default();

        let result: PyResult<Value> = (|| -> PyResult<Value> {
            let tmp_scene = d
                .getattr("scenes")?
                .call_method1("new", ("__mcp_preview__",))?;
            scratch.scene = Some(tmp_scene.clone());

            let render = tmp_scene.getattr("render")?;
            render.setattr("engine", engine)?;
            render.setattr("resolution_x", resolution)?;
            render.setattr("resolution_y", resolution)?;
            render.setattr("film_transparent", false)?;
            render
                .getattr("image_settings")?
                .setattr("file_format", "PNG")?;
            if engine == "BLENDER_EEVEE" {
                tmp_scene
                    .getattr("eevee")?
                    .setattr("taa_render_samples", samples)?;
            } else {
                let cycles = tmp_scene.getattr("cycles")?;
                cycles.setattr("samples", samples)?;
                cycles.setattr("use_denoising", true)?;
            }

            // Geometry: built with bmesh (see build_shape's doc comment for why not
            // bpy.ops.mesh.primitive_*_add), linked straight into the scratch scene.
            let bm = py.import("bmesh")?.call_method0("new")?;
            build_shape(py, &bm, shape)?;
            let mesh = d
                .getattr("meshes")?
                .call_method1("new", ("__mcp_preview_mesh__",))?;
            scratch.meshes.push(mesh.clone());
            bm.call_method1("to_mesh", (&mesh,))?;
            bm.call_method0("free")?;
            mesh.getattr("materials")?.call_method1("append", (&mat,))?;

            let obj = d
                .getattr("objects")?
                .call_method1("new", ("__mcp_preview_obj__", &mesh))?;
            scratch.objects.push(obj.clone());
            tmp_scene
                .getattr("collection")?
                .getattr("objects")?
                .call_method1("link", (&obj,))?;

            // Camera: one fixed rig framing all five shapes, each of which fits
            // inside roughly a 2.7-unit box centered on the origin (measured live).
            let cam_data = d
                .getattr("cameras")?
                .call_method1("new", ("__mcp_preview_cam__",))?;
            scratch.cameras.push(cam_data.clone());
            cam_data.setattr("lens", 35.0)?;
            let cam_obj = d
                .getattr("objects")?
                .call_method1("new", ("__mcp_preview_cam__", &cam_data))?;
            scratch.objects.push(cam_obj.clone());
            tmp_scene
                .getattr("collection")?
                .getattr("objects")?
                .call_method1("link", (&cam_obj,))?;
            point_at(py, &cam_obj, (0.0, -3.6, 1.4), (0.0, 0.0, 0.0))?;
            tmp_scene.setattr("camera", &cam_obj)?;

            // Lighting: an HDRI world when one is given, otherwise a flat dark
            // world plus the three-point studio rig.
            let world = d
                .getattr("worlds")?
                .call_method1("new", ("__mcp_preview_world__",))?;
            scratch.worlds.push(world.clone());
            world.setattr("use_nodes", true)?;
            let wnodes = world.getattr("node_tree")?.getattr("nodes")?;
            let bg = wnodes.call_method1("get", ("Background",))?;

            if let Some(hdri_path) = p.hdri.as_deref() {
                // `check_existing=False` is already Blender 5.0's default (confirmed
                // live), but it is passed explicitly: this call must always create a
                // fresh Image datablock it owns and can safely remove afterwards,
                // never reuse (and then delete) an image the user already had loaded
                // under the same path.
                let load_kw = PyDict::new(py);
                load_kw.set_item("check_existing", false)?;
                let img = d
                    .getattr("images")?
                    .call_method("load", (hdri_path,), Some(&load_kw))
                    .map_err(|e| {
                        PyValueError::new_err(format!("could not load hdri `{hdri_path}`: {e}"))
                    })?;
                scratch.images.push(img.clone());

                let env_tex = wnodes.call_method1("new", ("ShaderNodeTexEnvironment",))?;
                env_tex.setattr("image", &img)?;
                let links = world.getattr("node_tree")?.getattr("links")?;
                links.call_method1(
                    "new",
                    (
                        env_tex
                            .getattr("outputs")?
                            .call_method1("get", ("Color",))?,
                        bg.getattr("inputs")?.call_method1("get", ("Color",))?,
                    ),
                )?;
            } else {
                bg.getattr("inputs")?
                    .call_method1("get", ("Color",))?
                    .setattr("default_value", PyTuple::new(py, [0.05, 0.05, 0.05, 1.0])?)?;
                bg.getattr("inputs")?
                    .call_method1("get", ("Strength",))?
                    .setattr("default_value", 1.0)?;

                for (name, energy, loc, size) in STUDIO_RIG {
                    let lname = format!("__mcp_preview_{name}__");
                    let ldata = d
                        .getattr("lights")?
                        .call_method1("new", (lname.as_str(), "AREA"))?;
                    scratch.lights.push(ldata.clone());
                    ldata.setattr("energy", energy)?;
                    ldata.setattr("size", size)?;
                    let lobj = d
                        .getattr("objects")?
                        .call_method1("new", (lname.as_str(), &ldata))?;
                    scratch.objects.push(lobj.clone());
                    tmp_scene
                        .getattr("collection")?
                        .getattr("objects")?
                        .call_method1("link", (&lobj,))?;
                    point_at(py, &lobj, loc, (0.0, 0.0, 0.0))?;
                }
            }
            tmp_scene.setattr("world", &world)?;

            // Render. `scene=<name>` is an operator property (not a context
            // override): it renders that scene directly without ever touching
            // bpy.context.window.scene or bpy.context.scene — verified live in
            // --background, where the active window scene was unchanged by the call.
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0);
            let tmp_path = std::env::temp_dir().join(format!("mcp_preview_{nanos}.png"));
            let tmp_str = tmp_path.to_string_lossy().to_string();
            render.setattr("filepath", tmp_str.as_str())?;

            let scene_name: String = tmp_scene.getattr("name")?.extract()?;
            let render_kw = PyDict::new(py);
            render_kw.set_item("write_still", true)?;
            render_kw.set_item("scene", scene_name.as_str())?;
            let result = bpy.getattr("ops")?.getattr("render")?.call_method(
                "render",
                (),
                Some(&render_kw),
            )?;
            op_ok(&result, "render material preview")?;

            let bytes = fs::read(&tmp_path)
                .map_err(|e| PyIOError::new_err(format!("read preview render: {e}")))?;
            // The render is already in memory; a stale temp file must not fail the
            // call, but it must not vanish silently either — matches scene_screenshot.
            if let Err(e) = fs::remove_file(&tmp_path) {
                eprintln!("[blender-mcp] could not remove temp preview render {tmp_path:?}: {e}");
            }

            Ok(json!({
                "__image__": true,
                "data": B64.encode(&bytes),
                "mimeType": "image/png",
                "width": resolution,
                "height": resolution,
                "material": p.material,
                "shape": shape,
                "engine": engine,
                "samples": samples,
            }))
        })();

        scratch.cleanup(py)?;
        result
    }
);
