//! Images and textures.
//!
//! Without these there was no way to bring a texture into Blender at all, which meant
//! the single most-used shader node — Image Texture — could never be pointed at
//! anything. A shader without textures is half a shader.
//!
//! The node layer takes a datablock by NAME (`{"image": "brick.png"}`), so these tools
//! are what create the names it can refer to.

use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use serde::Deserialize;
use serde_json::{Value, json};

use super::bpy::{data, py_len};

/// Everything the model needs to decide whether this is the image it wants.
fn image_info(img: &Bound<'_, PyAny>) -> PyResult<Value> {
    let size = img.getattr("size")?;
    Ok(json!({
        "name": img.getattr("name")?.extract::<String>()?,
        "filepath": img.getattr("filepath")?.extract::<String>().unwrap_or_default(),
        "size": [
            size.get_item(0)?.extract::<i64>()?,
            size.get_item(1)?.extract::<i64>()?,
        ],
        "channels": img.getattr("channels")?.extract::<i64>().unwrap_or(0),
        // `has_data` is false for an image whose file is missing — the classic pink
        // texture. Worth surfacing: it looks fine in the node tree and renders wrong.
        "has_data": img.getattr("has_data")?.extract::<bool>().unwrap_or(false),
        "source": img.getattr("source")?.extract::<String>().unwrap_or_default(),
    }))
}

// ============================================================================
// img_load
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ImgLoad {
    /// Path to the image file on disk.
    pub path: String,
    /// Name for the datablock. Defaults to the file name; this is the name the node
    /// tools take (`{"image": "<name>"}`).
    pub name: Option<String>,
    /// Pack the pixels into the .blend, so the file no longer has to exist beside
    /// it. Default false.
    pub pack: Option<bool>,
    /// Colour space name. The list comes from the OCIO config, not from Blender's
    /// own code, so it is not fixed — an unknown value is rejected with Blender's
    /// error. On a stock Blender 5.2 config the ones that matter are `sRGB`
    /// (the default) and `Non-Color`; `Rec.1886`, `Linear Rec.709`, `ACEScg`,
    /// `AgX Base sRGB` and ~55 more also exist. A roughness/normal/metallic map
    /// MUST be Non-Color, or Blender applies a gamma curve to data that is not
    /// colour and the material comes out subtly wrong.
    pub colorspace: Option<String>,
}

crate::tool!(
    "img_load",
    "Load an image file into Blender so a node can use it. Returns the datablock name, which node_graph_build takes as the `image` property of a ShaderNodeTexImage.",
    ImgLoad,
    |py: Python<'_>, p: ImgLoad| {
        let bpy = py.import("bpy")?;
        let images = data(&bpy)?.getattr("images")?;

        // Blender reports a missing file by handing back an image with no data, rather
        // than raising — which is exactly how a broken texture reaches a render.
        let img = images
            .call_method1("load", (p.path.as_str(),))
            .map_err(|e| PyIOError::new_err(format!("cannot load `{}`: {e}", p.path)))?;
        // Blender 5 loads pixels lazily: `has_data` reads false immediately after
        // `load()` even for a perfectly good file, and only turns accurate once
        // something touches `.pixels` — confirmed live: a good PNG stays
        // has_data=False until `len(img.pixels)` is read (then True, real pixel
        // count); a genuinely corrupt file stays has_data=False either way. Force
        // the load here so the check below means what it says.
        py_len(py, &img.getattr("pixels")?)?;
        if !img.getattr("has_data")?.extract::<bool>().unwrap_or(false) {
            images.call_method1("remove", (&img,))?;
            return Err(PyIOError::new_err(format!(
                "`{}` loaded no pixels — is the path right, and is it an image Blender reads?",
                p.path
            )));
        }

        if let Some(name) = &p.name {
            img.setattr("name", name.as_str())?;
        }
        if let Some(cs) = &p.colorspace {
            img.getattr("colorspace_settings")?
                .setattr("name", cs.as_str())
                .map_err(|e| PyValueError::new_err(format!("unknown colour space `{cs}`: {e}")))?;
        }
        if p.pack.unwrap_or(false) {
            img.call_method0("pack")?;
        }

        image_info(&img)
    }
);

// ============================================================================
// img_new
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ImgNew {
    /// Datablock name.
    pub name: String,
    /// Width in pixels. Default 1024.
    pub width: Option<i64>,
    /// Height in pixels. Default 1024.
    pub height: Option<i64>,
    /// Fill colour as [r, g, b, a], each 0-1. Default [0, 0, 0, 1] (opaque black).
    pub color: Option<Vec<f64>>,
    /// Make it a 32-bit float (HDR) buffer instead of 8-bit. Default false.
    /// A NORMAL or POSITION bake target wants this.
    pub float: Option<bool>,
    /// Give the image an alpha channel. Default true.
    pub alpha: Option<bool>,
}

crate::tool!(
    "img_new",
    "Create a blank image datablock (a bake target, a canvas to paint into, a \
     placeholder). bake_texture requires its target image to already exist, and \
     this is what makes it. Use img_load for an image that is already a file on disk.",
    ImgNew,
    |py: Python<'_>, p: ImgNew| {
        let bpy = py.import("bpy")?;
        let images = data(&bpy)?.getattr("images")?;

        let kw = PyDict::new(py);
        kw.set_item("name", p.name.as_str())?;
        kw.set_item("width", p.width.unwrap_or(1024))?;
        kw.set_item("height", p.height.unwrap_or(1024))?;
        kw.set_item("alpha", p.alpha.unwrap_or(true))?;
        kw.set_item("float_buffer", p.float.unwrap_or(false))?;
        let img = images.call_method("new", (), Some(&kw))?;

        if let Some(c) = &p.color {
            let mut rgba = [0.0, 0.0, 0.0, 1.0];
            for (i, v) in c.iter().take(4).enumerate() {
                rgba[i] = *v;
            }
            img.setattr("generated_color", pyo3::types::PyTuple::new(py, rgba)?)?;
        }

        image_info(&img)
    }
);

// ============================================================================
// img_list
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ImgList {}

crate::tool!(
    "img_list",
    "Every image datablock in the file, with its size, path and whether its pixels actually loaded. These names are what the node tools refer to.",
    ImgList,
    |py: Python<'_>, _p: ImgList| {
        let bpy = py.import("bpy")?;
        let images = data(&bpy)?.getattr("images")?;
        let mut out = Vec::new();
        for img in images.try_iter()? {
            out.push(image_info(&img?)?);
        }
        Ok(json!(out))
    }
);

// ============================================================================
// img_save
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct ImgSave {
    /// Datablock name (see img_list).
    pub name: String,
    /// Absolute or Blender-relative path to write to; the extension picks the
    /// format. Omit to save back over the image's own filepath, which errors if it
    /// has none (an img_new image does not).
    pub path: Option<String>,
}

crate::tool!(
    "img_save",
    "Write an image datablock to disk (a bake result, a painted texture, a \
     generated map). A bake_texture result lives only in memory until this runs.",
    ImgSave,
    |py: Python<'_>, p: ImgSave| {
        let bpy = py.import("bpy")?;
        let images = data(&bpy)?.getattr("images")?;
        let img = images.call_method1("get", (p.name.as_str(),))?;
        if img.is_none() {
            return Err(PyValueError::new_err(format!(
                "image not found: {}",
                p.name
            )));
        }

        match &p.path {
            Some(path) => {
                let kw = PyDict::new(py);
                kw.set_item("filepath", path.as_str())?;
                img.call_method("save", (), Some(&kw))?;
            }
            None => {
                let fp: String = img.getattr("filepath")?.extract().unwrap_or_default();
                if fp.is_empty() {
                    return Err(PyValueError::new_err(format!(
                        "`{}` has no filepath of its own — give `path`",
                        p.name
                    )));
                }
                img.call_method0("save")?;
            }
        }

        image_info(&img)
    }
);
