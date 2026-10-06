//! World / environment tools: background colour, strength, HDRI.
//!
//! These are the *convenience* layer over the world's shader tree — the tree itself is
//! reachable as `world` through the unified node tools (`node_graph_get`,
//! `node_graph_build`, …), which is what to use for anything beyond a flat colour or
//! a plain HDRI.
//!
//! Extracted from the old `shading.rs` when `shader_*` and `comp_*` were deleted:
//! those hand-maintained a table of node types, which the node layer now reads from
//! Blender's RNA instead.
//!
//! Verified live on Blender 5.2.2 (`--background --factory-startup`): a node-backed
//! world's Background node has inputs `Color` (RGBA), `Strength` and `Weight`, which is
//! what `world_get`/`world_set_background` read and write. `World.use_nodes` is
//! deprecated and scheduled for removal in Blender 6.0 — these tools still set it,
//! because on 5.2 a world without it has no node tree to write into at all.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyIterator, PyList};
use serde::Deserialize;
use serde_json::json;

use super::bpy::{data, py_vec, scene};

// ============================================================================
// World / environment
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct WorldGet {}

crate::tool!(
    "world_get",
    "Read the active scene's world settings: its name, whether it uses nodes, and — if it does — the Background node's colour and strength. Returns `{has_world: false}` when the scene has no world at all; world_set_background and world_set_hdri both create one in that case. For anything past a flat colour or a plain HDRI, read the tree itself with node_graph_get on the `world` tree.",
    WorldGet,
    |py: Python<'_>, _p: WorldGet| {
        let bpy = py.import("bpy")?;
        let s = scene(&bpy)?;
        let world = s.getattr("world")?;
        if world.is_none() {
            return Ok(json!({ "has_world": false }));
        }

        let name: String = world.getattr("name")?.extract()?;
        let use_nodes: bool = world.getattr("use_nodes")?.extract()?;
        let mut result = json!({ "name": name, "use_nodes": use_nodes });

        if use_nodes {
            let nt = world.getattr("node_tree")?;
            if !nt.is_none() {
                for n in PyIterator::from_object(&nt.getattr("nodes")?)? {
                    let n = n?;
                    let ty: String = n.getattr("type")?.extract()?;
                    if ty == "BACKGROUND" {
                        let inputs = n.getattr("inputs")?;
                        let color = inputs.call_method1("get", ("Color",))?;
                        let strength = inputs.call_method1("get", ("Strength",))?;
                        result["background_color"] =
                            json!(py_vec(&color.getattr("default_value")?)?);
                        result["background_strength"] =
                            json!(strength.getattr("default_value")?.extract::<f64>()?);
                        break;
                    }
                }
            }
        }

        Ok(result)
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct WorldSetBackground {
    /// Background colour as `[r, g, b]`, linear, 0..1 (values above 1 are legal
    /// and read as emission). Alpha is appended as 1.0. Omit to leave the colour
    /// alone.
    pub color: Option<Vec<f64>>,
    /// Background node's Strength — a multiplier on the colour, 1.0 being
    /// unchanged, 0.0 a black environment. Omit to leave it alone.
    pub strength: Option<f64>,
}

crate::tool!(
    "world_set_background",
    "Set the world's flat background colour and/or strength, creating a world (and enabling nodes on it) if the scene has none. Use world_set_hdri instead for image-based lighting — it rebuilds the whole world tree and discards this colour. Read the current values back with world_get.",
    WorldSetBackground,
    |py: Python<'_>, p: WorldSetBackground| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let s = scene(&bpy)?;
        let mut world = s.getattr("world")?;
        if world.is_none() {
            world = d.getattr("worlds")?.call_method1("new", ("World",))?;
            s.setattr("world", &world)?;
        }
        world.setattr("use_nodes", true)?;

        let nt = world.getattr("node_tree")?;
        let mut bg_node: Option<Bound<'_, PyAny>> = None;
        for n in PyIterator::from_object(&nt.getattr("nodes")?)? {
            let n = n?;
            let ty: String = n.getattr("type")?.extract()?;
            if ty == "BACKGROUND" {
                bg_node = Some(n);
                break;
            }
        }
        let Some(bg_node) = bg_node else {
            return Err(PyValueError::new_err("World has no Background node"));
        };

        let inputs = bg_node.getattr("inputs")?;
        if let Some(c) = &p.color {
            let mut rgba: Vec<f64> = c.iter().take(3).copied().collect();
            rgba.push(1.0);
            inputs
                .call_method1("get", ("Color",))?
                .setattr("default_value", PyList::new(py, rgba)?)?;
        }
        if let Some(strength) = p.strength {
            inputs
                .call_method1("get", ("Strength",))?
                .setattr("default_value", strength)?;
        }

        Ok(json!({ "world": world.getattr("name")?.extract::<String>()? }))
    }
);

#[derive(Deserialize, schemars::JsonSchema)]
pub struct WorldSetHdri {
    /// Filesystem path to the environment image, passed straight to
    /// `bpy.data.images.load` — .hdr and .exr are the usual ones, but any format
    /// Blender reads works. A path Blender cannot open is an error.
    pub path: String,
    /// Background node's Strength — a multiplier on the HDRI, 1.0 being the
    /// image's own values. Omit to leave the node default (1.0).
    pub strength: Option<f64>,
    /// Mapping node rotation as `[x, y, z]` in radians; z is the one that spins
    /// the environment around the scene. Omit for no rotation.
    pub rotation: Option<Vec<f64>>,
}

crate::tool!(
    "world_set_hdri",
    "Build an image-based lighting world from an HDRI: Texture Coordinate -> Mapping -> Environment Texture -> Background -> World Output. Destructive — it clears the existing world node tree first, so any colour set by world_set_background is lost. For a tree more elaborate than this chain, build it with node_graph_build on the `world` tree.",
    WorldSetHdri,
    |py: Python<'_>, p: WorldSetHdri| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let s = scene(&bpy)?;
        let mut world = s.getattr("world")?;
        if world.is_none() {
            world = d.getattr("worlds")?.call_method1("new", ("World",))?;
            s.setattr("world", &world)?;
        }
        world.setattr("use_nodes", true)?;

        let tree = world.getattr("node_tree")?;
        tree.getattr("nodes")?.call_method0("clear")?;
        let nodes = tree.getattr("nodes")?;

        let tex_coord = nodes.call_method1("new", ("ShaderNodeTexCoord",))?;
        let mapping = nodes.call_method1("new", ("ShaderNodeMapping",))?;
        let env_tex = nodes.call_method1("new", ("ShaderNodeTexEnvironment",))?;
        let background = nodes.call_method1("new", ("ShaderNodeBackground",))?;
        let output = nodes.call_method1("new", ("ShaderNodeOutputWorld",))?;

        let img = d
            .getattr("images")?
            .call_method1("load", (p.path.as_str(),))?;
        env_tex.setattr("image", img)?;

        tex_coord.setattr("location", PyList::new(py, [-800.0, 0.0])?)?;
        mapping.setattr("location", PyList::new(py, [-600.0, 0.0])?)?;
        env_tex.setattr("location", PyList::new(py, [-300.0, 0.0])?)?;
        background.setattr("location", PyList::new(py, [0.0, 0.0])?)?;
        output.setattr("location", PyList::new(py, [200.0, 0.0])?)?;

        let links = tree.getattr("links")?;
        links.call_method1(
            "new",
            (
                tex_coord
                    .getattr("outputs")?
                    .call_method1("get", ("Generated",))?,
                mapping
                    .getattr("inputs")?
                    .call_method1("get", ("Vector",))?,
            ),
        )?;
        links.call_method1(
            "new",
            (
                mapping
                    .getattr("outputs")?
                    .call_method1("get", ("Vector",))?,
                env_tex
                    .getattr("inputs")?
                    .call_method1("get", ("Vector",))?,
            ),
        )?;
        links.call_method1(
            "new",
            (
                env_tex
                    .getattr("outputs")?
                    .call_method1("get", ("Color",))?,
                background
                    .getattr("inputs")?
                    .call_method1("get", ("Color",))?,
            ),
        )?;
        links.call_method1(
            "new",
            (
                background
                    .getattr("outputs")?
                    .call_method1("get", ("Background",))?,
                output
                    .getattr("inputs")?
                    .call_method1("get", ("Surface",))?,
            ),
        )?;

        if let Some(strength) = p.strength {
            background
                .getattr("inputs")?
                .call_method1("get", ("Strength",))?
                .setattr("default_value", strength)?;
        }
        if let Some(rotation) = &p.rotation {
            mapping
                .getattr("inputs")?
                .call_method1("get", ("Rotation",))?
                .setattr("default_value", PyList::new(py, rotation.iter().copied())?)?;
        }

        Ok(json!({ "world": world.getattr("name")?.extract::<String>()?, "hdri": p.path }))
    }
);
