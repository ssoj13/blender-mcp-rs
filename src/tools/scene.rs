//! Scene tools. Registered automatically — nothing outside this file names them.
//!
//! ## Why these exist
//! `scene_get_info` used to be read-only: nothing in the registry could write frame
//! range, fps, render settings, colour management or view-layer passes. That left a
//! model unable to key an animation at the right length/fps, bake a sim over the
//! wrong range, change the look (AgX/Filmic/Standard) of a render, or turn on any
//! pass beyond Combined/Alpha for the compositor's Render Layers node. `scene_set`,
//! `render_set`, `color_set` and `view_layer_passes` close that gap; `scene_get_info`
//! grows to report back what they can now set, so a caller can verify a write landed.
//!
//! Unlike `scene_render` (`misc.rs`), which mutates render settings only for the
//! duration of one render and restores them afterwards, `render_set` and `color_set`
//! persist — that is the point of a *settings* tool.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::Deserialize;
use serde_json::json;

use super::bpy::{data, py_len, render_engine, scene};

// ============================================================================
// Shared helpers
// ============================================================================

/// Sets an RNA attribute Blender validates as an enum at assignment time, turning a
/// rejected value into an error that lists what this Blender actually offers —
/// lifted verbatim out of Blender's own validation message.
///
/// Several enums here (`view_transform`, `look`, `render.engine`) are populated by a
/// runtime callback (`itemf`) rather than a static list: which OCIO looks exist
/// depends on the active colour config, which render engines exist depends on which
/// engine add-ons are registered. The normal introspection path —
/// `bl_rna.properties[name].enum_items` — does not carry the instance/context the
/// callback needs and silently returns a stale placeholder. Verified live against
/// Blender 5: it reports only `BLENDER_EEVEE` for `render.engine` on a build where
/// `CYCLES` and `BLENDER_WORKBENCH` are both registered and settable. Attempting the
/// assignment and relaying Blender's *own* rejection message is the only path that is
/// guaranteed to reflect what this Blender actually offers right now — Blender
/// validates every enum write against the live item list regardless of what
/// introspection can see ahead of time.
fn set_enum(obj: &Bound<'_, PyAny>, attr: &str, value: &str, label: &str) -> PyResult<()> {
    obj.setattr(attr, value).map_err(|e| {
        let msg = e.to_string();
        match (msg.find('('), msg.rfind(')')) {
            (Some(a), Some(b)) if a < b => PyValueError::new_err(format!(
                "invalid {label} `{value}`. This Blender offers: {}",
                &msg[a + 1..b]
            )),
            _ => PyValueError::new_err(format!("invalid {label} `{value}`: {msg}")),
        }
    })
}

/// Blender's own live list of valid values for an enum-valued RNA attribute,
/// recovered the same way `set_enum` recovers a rejection message: assign a value
/// no real item can ever equal and read the list back out of Blender's validation
/// error. Used where a caller needs the live set itself — alias resolution — not
/// just a formatted rejection.
fn live_enum_options(obj: &Bound<'_, PyAny>, attr: &str) -> PyResult<Vec<String>> {
    const SENTINEL: &str = "\u{0}__blender_mcp_probe__\u{0}";
    match obj.setattr(attr, SENTINEL) {
        // No real enum value can equal the sentinel, so acceptance means `attr`
        // is not validated as a fixed-item enum at all — a caller bug, not a
        // live-options question this function can answer.
        Ok(()) => Err(PyValueError::new_err(format!(
            "`{attr}` accepted an arbitrary string — it is not a validated enum"
        ))),
        Err(e) => {
            let msg = e.to_string();
            let (Some(a), Some(b)) = (msg.find('('), msg.rfind(')')) else {
                return Err(PyValueError::new_err(format!(
                    "could not read live options for `{attr}`: {msg}"
                )));
            };
            Ok(msg[a + 1..b]
                .split(',')
                .map(|s| s.trim().trim_matches('\'').to_string())
                .collect())
        }
    }
}

/// Resolves a friendly engine name (`"eevee"`, `"cycles"`) against this Blender's
/// *actual* live engine list, obtained via [`live_enum_options`]. Mirrors
/// `super::bpy::render_engine()`'s alias logic (exact match, then substring match)
/// but sourced from Blender's own runtime validation rather than
/// `bl_rna.properties["engine"].enum_items` — see [`set_enum`]'s doc comment for why
/// that introspection path under-reports engines registered by an add-on (verified
/// live: Cycles and Workbench were both missing from it). Used only as a fallback
/// when `render_engine()` itself fails to resolve — its alias handling remains the
/// first attempt so behaviour is unchanged for the common case it was built for.
fn resolve_engine_alias(render: &Bound<'_, PyAny>, want: &str) -> PyResult<String> {
    let options = live_enum_options(render, "engine")?;
    let want_upper = want.to_uppercase();
    if let Some(exact) = options.iter().find(|o| **o == want_upper) {
        return Ok(exact.clone());
    }
    if let Some(alias) = options.iter().find(|o| o.contains(&want_upper)) {
        return Ok(alias.clone());
    }
    Err(PyValueError::new_err(format!(
        "unknown render engine `{want}`. This Blender has: {}",
        options.join(", ")
    )))
}

/// `(view_layer_passes field name, ViewLayer.use_pass_* attribute)`. Single source
/// of truth: `view_layer_passes` uses it to apply toggles, `scene_get_info` uses it
/// to report which are currently enabled — order matters, see the comment on
/// `ViewLayerPasses`'s handler.
///
/// Verified live against Blender 5: every one of these lives directly on
/// `ViewLayer` for **both** EEVEE Next and Cycles — unlike older Blender, where
/// passes such as Cryptomatte lived only under `view_layer.cycles`. The genuinely
/// engine-specific passes that remain under `view_layer.cycles`
/// (`use_pass_volume_*`, `use_pass_shadow_catcher`) are Cycles-only and were not
/// requested, so they are intentionally not in this table.
const PASS_ATTRS: &[(&str, &str)] = &[
    ("combined", "use_pass_combined"),
    ("z", "use_pass_z"),
    ("mist", "use_pass_mist"),
    ("normal", "use_pass_normal"),
    ("position", "use_pass_position"),
    ("vector", "use_pass_vector"),
    ("uv", "use_pass_uv"),
    ("object_index", "use_pass_object_index"),
    ("material_index", "use_pass_material_index"),
    ("diffuse_direct", "use_pass_diffuse_direct"),
    ("diffuse_indirect", "use_pass_diffuse_indirect"),
    ("diffuse_color", "use_pass_diffuse_color"),
    ("glossy_direct", "use_pass_glossy_direct"),
    ("glossy_indirect", "use_pass_glossy_indirect"),
    ("glossy_color", "use_pass_glossy_color"),
    ("transmission_direct", "use_pass_transmission_direct"),
    ("transmission_indirect", "use_pass_transmission_indirect"),
    ("transmission_color", "use_pass_transmission_color"),
    ("emit", "use_pass_emit"),
    ("environment", "use_pass_environment"),
    ("ao", "use_pass_ambient_occlusion"),
    ("shadow", "use_pass_shadow"),
    ("cryptomatte_object", "use_pass_cryptomatte_object"),
    ("cryptomatte_material", "use_pass_cryptomatte_material"),
    ("cryptomatte_asset", "use_pass_cryptomatte_asset"),
];

/// Reads the current enabled/disabled state of every pass in [`PASS_ATTRS`] that
/// exists on `view_layer`, for `scene_get_info`'s read-back.
fn read_enabled_passes(
    view_layer: &Bound<'_, PyAny>,
) -> PyResult<serde_json::Map<String, serde_json::Value>> {
    let mut out = serde_json::Map::new();
    for (key, attr) in PASS_ATTRS {
        if view_layer.hasattr(*attr)? {
            out.insert(
                (*key).to_string(),
                json!(view_layer.getattr(*attr)?.extract::<bool>()?),
            );
        }
    }
    Ok(out)
}

// ============================================================================
// scene_get_info
// ============================================================================

/// `scene_get_info` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneGetInfo {}

crate::tool!(
    "scene_get_info",
    "Read the active scene back: name, fps, frame range, current frame, resolution, \
     engine, view transform, film transparency, enabled render passes, and \
     object/material counts. Takes no arguments. The read-back counterpart of \
     scene_set, render_set, color_set and view_layer_passes — call it to confirm a \
     write landed",
    SceneGetInfo,
    |py: Python<'_>, _p: SceneGetInfo| {
        let bpy = py.import("bpy")?;
        let sc = scene(&bpy)?;
        let d = data(&bpy)?;

        let render = sc.getattr("render")?;
        let view_settings = sc.getattr("view_settings")?;
        let view_layer = bpy.getattr("context")?.getattr("view_layer")?;
        let objects = d.getattr("objects")?;
        let materials = d.getattr("materials")?;

        Ok(json!({
            "name": sc.getattr("name")?.extract::<String>()?,
            "fps": render.getattr("fps")?.extract::<i32>()?,
            "fps_base": render.getattr("fps_base")?.extract::<f64>()?,
            "frame_start": sc.getattr("frame_start")?.extract::<i32>()?,
            "frame_end": sc.getattr("frame_end")?.extract::<i32>()?,
            "frame_current": sc.getattr("frame_current")?.extract::<i32>()?,
            "resolution": [
                render.getattr("resolution_x")?.extract::<i32>()?,
                render.getattr("resolution_y")?.extract::<i32>()?,
            ],
            "engine": render.getattr("engine")?.extract::<String>()?,
            "view_transform": view_settings.getattr("view_transform")?.extract::<String>()?,
            "film_transparent": render.getattr("film_transparent")?.extract::<bool>()?,
            "enabled_passes": read_enabled_passes(&view_layer)?,
            "objects": py_len(py, &objects)?,
            "materials": py_len(py, &materials)?,
        }))
    }
);

// ============================================================================
// scene_set — frame range, current frame, fps
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SceneSet {
    /// First frame of the playback/render range
    pub frame_start: Option<i32>,
    /// Last frame of the playback/render range. Must not be less than the
    /// resulting frame_start, or this call errors instead of silently swapping
    /// or clamping the range (Blender's own UI auto-corrects one when you edit
    /// the other; a model calling this wants an explicit error, not a range it
    /// didn't ask for)
    pub frame_end: Option<i32>,
    /// Current frame (playhead)
    pub frame_current: Option<i32>,
    /// Frames per second (integer part)
    pub fps: Option<i32>,
    /// Frames per second divisor — actual fps is fps / fps_base (e.g. 30 / 1.001
    /// for 29.97)
    pub fps_base: Option<f64>,
}

crate::tool!(
    "scene_set",
    "Set the scene's frame range, current frame and fps. These are the values \
     bakes and playback read, so set them before phys_bake or sim_bake (both of \
     which default their range to this one). Read them back with scene_get_info",
    SceneSet,
    |py: Python<'_>, p: SceneSet| {
        let bpy = py.import("bpy")?;
        let sc = scene(&bpy)?;

        let cur_start: i32 = sc.getattr("frame_start")?.extract()?;
        let cur_end: i32 = sc.getattr("frame_end")?.extract()?;
        let new_start = p.frame_start.unwrap_or(cur_start);
        let new_end = p.frame_end.unwrap_or(cur_end);
        if new_end < new_start {
            return Err(PyValueError::new_err(format!(
                "frame_end ({new_end}) < frame_start ({new_start})"
            )));
        }

        // Blender couples frame_start/frame_end: setting one past the other's
        // *current* value auto-clamps it. The target range was already validated
        // above, so pick the assignment order that never passes through an
        // inverted intermediate state and triggers that clamp.
        if let Some(v) = p.frame_start {
            if new_start > cur_end {
                if let Some(e) = p.frame_end {
                    sc.setattr("frame_end", e)?;
                }
                sc.setattr("frame_start", v)?;
            } else {
                sc.setattr("frame_start", v)?;
                if let Some(e) = p.frame_end {
                    sc.setattr("frame_end", e)?;
                }
            }
        } else if let Some(e) = p.frame_end {
            sc.setattr("frame_end", e)?;
        }

        if let Some(v) = p.frame_current {
            sc.setattr("frame_current", v)?;
        }

        let render = sc.getattr("render")?;
        if let Some(v) = p.fps {
            render.setattr("fps", v)?;
        }
        if let Some(v) = p.fps_base {
            render.setattr("fps_base", v)?;
        }

        Ok(json!({
            "frame_start": sc.getattr("frame_start")?.extract::<i32>()?,
            "frame_end": sc.getattr("frame_end")?.extract::<i32>()?,
            "frame_current": sc.getattr("frame_current")?.extract::<i32>()?,
            "fps": render.getattr("fps")?.extract::<i32>()?,
            "fps_base": render.getattr("fps_base")?.extract::<f64>()?,
        }))
    }
);

// ============================================================================
// render_set — engine, resolution, samples, film transparency, motion blur, output
// ============================================================================

/// Applies `samples` to the RNA path the active engine actually keeps it in — EEVEE
/// and Cycles store their render sample count in different places
/// (`scene.eevee.taa_render_samples` vs `scene.cycles.samples`), and Workbench has
/// no free integer sample count at all (its anti-aliasing is a fixed enum level, see
/// `scene.display.render_aa`), so a caller asking for it there gets an explicit
/// error rather than a value that is silently dropped.
fn set_render_samples(sc: &Bound<'_, PyAny>, engine: &str, samples: i32) -> PyResult<()> {
    if engine.contains("EEVEE") {
        sc.getattr("eevee")?.setattr("taa_render_samples", samples)
    } else if engine == "CYCLES" {
        sc.getattr("cycles")?.setattr("samples", samples)
    } else {
        Err(PyValueError::new_err(format!(
            "render_set: engine `{engine}` has no integer `samples` setting (its \
             anti-aliasing, if any, is a fixed level rather than a free sample count)"
        )))
    }
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct RenderSet {
    /// Output width in pixels, before resolution_percentage. Integer. Omit to
    /// leave unchanged
    pub resolution_x: Option<i32>,
    /// Output height in pixels, before resolution_percentage. Integer. Omit to
    /// leave unchanged
    pub resolution_y: Option<i32>,
    /// Percentage scale applied to resolution_x/y at render time. Integer,
    /// Blender default 100. Omit to leave unchanged
    pub resolution_percentage: Option<i32>,
    /// Render engine, e.g. EEVEE, CYCLES, WORKBENCH — resolved against this
    /// Blender's actual engines. On Blender 5.2 (factory startup) the live ids
    /// are BLENDER_EEVEE, BLENDER_WORKBENCH and CYCLES; EEVEE's exact id has
    /// moved across versions (BLENDER_EEVEE / BLENDER_EEVEE_NEXT / BLENDER_EEVEE
    /// again in 5.0), so pass the friendly name and it is resolved for you. Omit
    /// to leave unchanged
    pub engine: Option<String>,
    /// Render sample count. Integer. Where it is stored depends on the (possibly
    /// just-set) engine: EEVEE's taa_render_samples (5.2 default 64), Cycles'
    /// samples. Not supported for Workbench, which errors instead. Omit to leave
    /// unchanged
    pub samples: Option<i32>,
    /// Render with a transparent background (alpha channel) instead of the world
    /// background — what the compositor needs to key or composite over other
    /// footage. Default: unchanged
    pub film_transparent: Option<bool>,
    /// Enable motion blur. Default: unchanged
    pub use_motion_blur: Option<bool>,
    /// Motion blur shutter time, as a fraction of a frame (Blender default 0.5).
    /// Omit to leave unchanged
    pub motion_blur_shutter: Option<f64>,
    /// Output file path. Blender's `//` prefix means relative to the saved
    /// .blend. Omit to leave unchanged
    pub filepath: Option<String>,
    /// Output image file format, upper-cased before use. Blender 5.2 accepts
    /// AVIF, JPEG, OPEN_EXR, OPEN_EXR_MULTILAYER, PNG, WEBP, BMP, CINEON, DPX,
    /// IRIS, JPEG2000, HDR, TARGA, TARGA_RAW, TIFF, FFMPEG. A bad value errors
    /// listing this Blender's actual supported formats
    pub file_format: Option<String>,
    /// Output colour depth in bits, as a string. Blender 5.2's full set is "8",
    /// "10", "12", "16", "32"; which of them a given file_format accepts is
    /// narrower (PNG: 8/16, OPEN_EXR: 16/32), and a bad combination errors
    /// listing what this Blender accepts
    pub color_depth: Option<String>,
}

crate::tool!(
    "render_set",
    "Set render settings that persist on the scene: engine, resolution, samples, \
     film transparency, motion blur, output path/format. Only the fields you pass \
     are written. Use this when later renders should all share the settings; \
     scene_render is the one-shot tool that applies its own arguments for a single \
     render and restores them afterwards, so anything set here stays put",
    RenderSet,
    |py: Python<'_>, p: RenderSet| {
        let bpy = py.import("bpy")?;
        let sc = scene(&bpy)?;
        let render = sc.getattr("render")?;

        if let Some(v) = p.resolution_x {
            render.setattr("resolution_x", v)?;
        }
        if let Some(v) = p.resolution_y {
            render.setattr("resolution_y", v)?;
        }
        if let Some(v) = p.resolution_percentage {
            render.setattr("resolution_percentage", v)?;
        }

        if let Some(want) = &p.engine {
            // `render_engine()` resolves the EEVEE-naming drift this Blender build
            // may have; but its introspection reads `bl_rna.properties["engine"]`,
            // which — like the other itemf-driven enums documented on `set_enum` —
            // under-reports engines registered by a Python add-on. Verified live: it
            // saw only `BLENDER_EEVEE` on a Blender where Cycles and Workbench were
            // both already enabled and directly settable. Fall back to
            // `resolve_engine_alias`, which resolves the same alias (e.g. `"cycles"`
            // -> `"CYCLES"`, `"workbench"` -> `"BLENDER_WORKBENCH"`) against
            // Blender's own live engine list, rather than let a stale cache block a
            // legitimate engine switch.
            let resolved = match render_engine(&sc, want) {
                Ok(resolved) => resolved,
                Err(_) => resolve_engine_alias(&render, want)?,
            };
            render.setattr("engine", resolved)?;
        }

        if let Some(samples) = p.samples {
            let engine: String = render.getattr("engine")?.extract()?;
            set_render_samples(&sc, &engine, samples)?;
        }

        if let Some(v) = p.film_transparent {
            render.setattr("film_transparent", v)?;
        }
        if let Some(v) = p.use_motion_blur {
            render.setattr("use_motion_blur", v)?;
        }
        if let Some(v) = p.motion_blur_shutter {
            render.setattr("motion_blur_shutter", v)?;
        }
        if let Some(v) = &p.filepath {
            render.setattr("filepath", v.as_str())?;
        }

        let image_settings = render.getattr("image_settings")?;
        if let Some(v) = &p.file_format {
            set_enum(
                &image_settings,
                "file_format",
                &v.to_uppercase(),
                "file_format",
            )?;
        }
        if let Some(v) = &p.color_depth {
            set_enum(&image_settings, "color_depth", v, "color_depth")?;
        }

        Ok(json!({
            "engine": render.getattr("engine")?.extract::<String>()?,
            "resolution": [
                render.getattr("resolution_x")?.extract::<i32>()?,
                render.getattr("resolution_y")?.extract::<i32>()?,
            ],
            "resolution_percentage": render.getattr("resolution_percentage")?.extract::<i32>()?,
            "film_transparent": render.getattr("film_transparent")?.extract::<bool>()?,
            "use_motion_blur": render.getattr("use_motion_blur")?.extract::<bool>()?,
            "filepath": render.getattr("filepath")?.extract::<String>()?,
            "file_format": image_settings.getattr("file_format")?.extract::<String>()?,
            "color_depth": image_settings.getattr("color_depth")?.extract::<String>()?,
        }))
    }
);

// ============================================================================
// color_set — colour management (view transform, look, exposure, gamma)
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ColorSet {
    /// Colour view transform. Case- and space-sensitive; Blender 5.2's stock
    /// OCIO config offers exactly: Standard, ACES 1.3, ACES 2.0, Khronos PBR
    /// Neutral, AgX (the default), Filmic, Filmic Log, False Color, Raw. A bad
    /// value errors listing this Blender's actual options — they depend on the
    /// active OCIO config, not just the Blender version. Omit to leave unchanged
    pub view_transform: Option<String>,
    /// Look applied on top of the view transform, or "None" to clear it. Which
    /// looks are valid depends on the active view_transform: under AgX on
    /// Blender 5.2 they are None, AgX - Punchy, AgX - Greyscale, AgX - Very High
    /// Contrast, AgX - High Contrast, AgX - Medium High Contrast, AgX - Base
    /// Contrast, AgX - Medium Low Contrast, AgX - Low Contrast, AgX - Very Low
    /// Contrast. Pass view_transform in the same call when changing both — it is
    /// applied first, so look is checked against the transform that will
    /// actually be active. Omit to leave unchanged
    pub look: Option<String>,
    /// Exposure adjustment in stops (log2), applied before the view transform.
    /// Blender default 0.0. Omit to leave unchanged
    pub exposure: Option<f64>,
    /// Gamma correction applied after the view transform. Blender default 1.0
    /// (no change). Omit to leave unchanged
    pub gamma: Option<f64>,
}

crate::tool!(
    "color_set",
    "Set the scene's colour management — view transform (AgX/Filmic/Standard/...), \
     look, exposure, gamma. These persist like render_set's settings and change how \
     every subsequent render and screenshot is tone-mapped; an invalid \
     view_transform or look errors listing this Blender's real options",
    ColorSet,
    |py: Python<'_>, p: ColorSet| {
        let bpy = py.import("bpy")?;
        let sc = scene(&bpy)?;
        let vs = sc.getattr("view_settings")?;

        if let Some(v) = &p.view_transform {
            set_enum(&vs, "view_transform", v, "view_transform")?;
        }
        if let Some(v) = &p.look {
            set_enum(&vs, "look", v, "look")?;
        }
        if let Some(v) = p.exposure {
            vs.setattr("exposure", v)?;
        }
        if let Some(v) = p.gamma {
            vs.setattr("gamma", v)?;
        }

        Ok(json!({
            "view_transform": vs.getattr("view_transform")?.extract::<String>()?,
            "look": vs.getattr("look")?.extract::<String>()?,
            "exposure": vs.getattr("exposure")?.extract::<f64>()?,
            "gamma": vs.getattr("gamma")?.extract::<f64>()?,
        }))
    }
);

// ============================================================================
// view_layer_passes — render passes on the active view layer
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ViewLayerPasses {
    /// Combined (beauty) pass. Boolean; omit to leave this pass unchanged.
    pub combined: Option<bool>,
    /// Depth (Z) pass. Boolean; omit to leave this pass unchanged.
    pub z: Option<bool>,
    /// Mist/fog factor pass. Boolean; omit to leave this pass unchanged.
    pub mist: Option<bool>,
    /// Surface normal pass. Boolean; omit to leave this pass unchanged.
    pub normal: Option<bool>,
    /// World-space position pass. Boolean; omit to leave this pass unchanged.
    pub position: Option<bool>,
    /// Motion vector pass. Boolean; omit to leave this pass unchanged.
    pub vector: Option<bool>,
    /// UV coordinate pass. Boolean; omit to leave this pass unchanged.
    pub uv: Option<bool>,
    /// Per-object index pass (Object Properties > Relations > Pass Index). Boolean; omit to leave this pass unchanged.
    pub object_index: Option<bool>,
    /// Per-material index pass (Material Properties > Settings > Pass Index). Boolean; omit to leave this pass unchanged.
    pub material_index: Option<bool>,
    /// Direct diffuse lighting pass. Boolean; omit to leave this pass unchanged.
    pub diffuse_direct: Option<bool>,
    /// Indirect diffuse lighting pass. Boolean; omit to leave this pass unchanged.
    pub diffuse_indirect: Option<bool>,
    /// Diffuse surface colour pass, without lighting. Boolean; omit to leave this pass unchanged.
    pub diffuse_color: Option<bool>,
    /// Direct glossy/specular lighting pass. Boolean; omit to leave this pass unchanged.
    pub glossy_direct: Option<bool>,
    /// Indirect glossy/specular lighting pass. Boolean; omit to leave this pass unchanged.
    pub glossy_indirect: Option<bool>,
    /// Glossy surface colour pass, without lighting. Boolean; omit to leave this pass unchanged.
    pub glossy_color: Option<bool>,
    /// Direct transmission lighting pass. Boolean; omit to leave this pass unchanged.
    pub transmission_direct: Option<bool>,
    /// Indirect transmission lighting pass. Boolean; omit to leave this pass unchanged.
    pub transmission_indirect: Option<bool>,
    /// Transmission surface colour pass, without lighting. Boolean; omit to leave this pass unchanged.
    pub transmission_color: Option<bool>,
    /// Emission pass. Boolean; omit to leave this pass unchanged.
    pub emit: Option<bool>,
    /// World/environment lighting contribution pass. Boolean; omit to leave this pass unchanged.
    pub environment: Option<bool>,
    /// Ambient occlusion pass. Boolean; omit to leave this pass unchanged.
    pub ao: Option<bool>,
    /// Shadow pass. Boolean; omit to leave this pass unchanged.
    pub shadow: Option<bool>,
    /// Cryptomatte: per-object ID matte, for isolating individual objects in comp. Boolean; omit to leave this pass unchanged.
    pub cryptomatte_object: Option<bool>,
    /// Cryptomatte: per-material ID matte. Boolean; omit to leave this pass unchanged.
    pub cryptomatte_material: Option<bool>,
    /// Cryptomatte: per-asset ID matte. Boolean; omit to leave this pass unchanged.
    pub cryptomatte_asset: Option<bool>,
}

crate::tool!(
    "view_layer_passes",
    "Enable/disable render passes on the active view layer — what the compositor's \
     Render Layers node can expose beyond Image/Alpha. A pass this Blender version \
     does not have is reported in `unavailable`, not silently skipped",
    ViewLayerPasses,
    |py: Python<'_>, p: ViewLayerPasses| {
        let bpy = py.import("bpy")?;
        let view_layer = bpy.getattr("context")?.getattr("view_layer")?;

        // Order must match `PASS_ATTRS` exactly: this is the caller-supplied half
        // of the (attribute, requested value) pairing that PASS_ATTRS provides the
        // RNA attribute name for.
        let requested: [Option<bool>; 25] = [
            p.combined,
            p.z,
            p.mist,
            p.normal,
            p.position,
            p.vector,
            p.uv,
            p.object_index,
            p.material_index,
            p.diffuse_direct,
            p.diffuse_indirect,
            p.diffuse_color,
            p.glossy_direct,
            p.glossy_indirect,
            p.glossy_color,
            p.transmission_direct,
            p.transmission_indirect,
            p.transmission_color,
            p.emit,
            p.environment,
            p.ao,
            p.shadow,
            p.cryptomatte_object,
            p.cryptomatte_material,
            p.cryptomatte_asset,
        ];

        let mut applied = Vec::new();
        let mut unavailable = Vec::new();
        for ((key, attr), value) in PASS_ATTRS.iter().zip(requested.iter()) {
            let Some(value) = value else { continue };
            if !view_layer.hasattr(*attr)? {
                unavailable.push(*key);
                continue;
            }
            view_layer.setattr(*attr, *value)?;
            applied.push(*key);
        }

        Ok(json!({
            "applied": applied,
            "unavailable": unavailable,
            "enabled_passes": read_enabled_passes(&view_layer)?,
        }))
    }
);
