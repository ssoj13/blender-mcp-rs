//! Ask Blender what nodes exist and what they take. No tables.
//!
//! The project rule is "do not infer Blender's API — ask the running Blender". These
//! two tools hand that rule to the model: it can stop working from memory, which for
//! node sockets is reliably wrong on Blender 5.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use serde::Deserialize;
use serde_json::{Map, Value, json};

use super::{enabled_sockets, socket_info};

/// The RNA base class each family of nodes derives from. This is the whole "table":
/// five names, and Blender fills in the 506 types.
const FAMILIES: [(&str, &str, &str); 5] = [
    // (kind, RNA base class, the node tree these live in)
    ("shader", "ShaderNode", "ShaderNodeTree"),
    ("geometry", "GeometryNode", "GeometryNodeTree"),
    ("compositor", "CompositorNode", "CompositorNodeTree"),
    ("texture", "TextureNode", "TextureNodeTree"),
    // Function nodes (maths, comparisons, string ops) live inside geometry trees.
    ("function", "FunctionNode", "GeometryNodeTree"),
];

fn family(kind: &str) -> PyResult<(&'static str, &'static str)> {
    FAMILIES
        .iter()
        .find(|(k, _, _)| *k == kind)
        .map(|(_, base, tree)| (*base, *tree))
        .ok_or_else(|| {
            PyValueError::new_err(format!(
                "unknown node kind `{kind}`: expected one of {}",
                FAMILIES
                    .iter()
                    .map(|(k, _, _)| *k)
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })
}

/// One advertised node type: kind, `bl_idname`, human label.
type Catalog = Vec<(&'static str, String, String)>;

/// The node types this Blender can actually create.
///
/// Enumerating the RNA classes is not enough: some are **registered but dead**.
/// `TextureNodeTexMusgrave` still has a class in `bpy.types`, yet `nodes.new()` on it
/// raises — Musgrave was removed in Blender 4.1. Advertising it would be exactly the
/// lie the old hand-written table told.
///
/// So the catalog is built by *actually creating* every class once, in a scratch tree,
/// and keeping only the ones that work. Measured at 235 ms for all 506 (500 live, 6
/// dead), paid once per process and cached.
fn catalog(py: Python<'_>) -> PyResult<&'static Catalog> {
    static CATALOG: std::sync::OnceLock<Catalog> = std::sync::OnceLock::new();
    if let Some(c) = CATALOG.get() {
        return Ok(c);
    }

    let groups = py.import("bpy")?.getattr("data")?.getattr("node_groups")?;
    let mut out: Catalog = Vec::new();

    for (kind, base, tree_kind) in FAMILIES {
        let tree = groups.call_method1("new", ("__mcp_probe__", tree_kind))?;
        let nodes = tree.getattr("nodes")?;
        let probe = (|| -> PyResult<()> {
            for cls in classes_of(py, base)? {
                let rna = cls.getattr("bl_rna")?;
                let id: String = rna.getattr("identifier")?.extract()?;
                if nodes.call_method1("new", (id.as_str(),)).is_err() {
                    continue; // registered but not creatable — do not advertise it
                }
                let label: String = rna
                    .getattr("name")
                    .and_then(|l| l.extract())
                    .unwrap_or_default();
                out.push((kind, id, label));
            }
            Ok(())
        })();
        groups.call_method1("remove", (&tree,))?;
        probe?;
    }

    Ok(CATALOG.get_or_init(|| out))
}

/// Every node class Blender knows in a family, read out of `bpy.types` at runtime.
fn classes_of<'py>(py: Python<'py>, base_name: &str) -> PyResult<Vec<Bound<'py, PyAny>>> {
    let types = py.import("bpy")?.getattr("types")?;
    let base = types.getattr(base_name)?;
    let builtins = py.import("builtins")?;

    let mut out = Vec::new();
    for name in types.dir()? {
        let name: String = name.extract()?;
        let Ok(cls) = types.getattr(name.as_str()) else {
            continue;
        };
        if !builtins
            .call_method1("isinstance", (&cls, builtins.getattr("type")?))?
            .extract::<bool>()?
        {
            continue;
        }
        let is_sub: bool = builtins
            .call_method1("issubclass", (&cls, &base))?
            .extract()?;
        if is_sub && !cls.is(&base) {
            out.push(cls);
        }
    }
    Ok(out)
}

/// Which family a `bl_idname` belongs to, so `node_schema` knows which kind of tree
/// to instantiate it in.
fn family_of(py: Python<'_>, bl_idname: &str) -> PyResult<(&'static str, &'static str)> {
    let types = py.import("bpy")?.getattr("types")?;
    let builtins = py.import("builtins")?;
    let Ok(cls) = types.getattr(bl_idname) else {
        return Err(PyValueError::new_err(format!(
            "unknown node type `{bl_idname}` (use node_types to search for it)"
        )));
    };
    for (_, base_name, tree) in FAMILIES {
        let base = types.getattr(base_name)?;
        if builtins
            .call_method1("issubclass", (&cls, &base))?
            .extract::<bool>()?
        {
            return Ok((base_name, tree));
        }
    }
    Err(PyValueError::new_err(format!(
        "`{bl_idname}` is not a node type"
    )))
}

/// A scratch tree of the right kind, so a node can be instantiated and interrogated.
/// It never enters the user's file — it is removed before we return.
fn with_scratch_node<T>(
    py: Python<'_>,
    bl_idname: &str,
    props: Option<&Map<String, Value>>,
    f: impl FnOnce(&Bound<'_, PyAny>) -> PyResult<T>,
) -> PyResult<T> {
    let (_, tree_kind) = family_of(py, bl_idname)?;
    let bpy = py.import("bpy")?;
    let groups = bpy.getattr("data")?.getattr("node_groups")?;

    let tree = groups.call_method1("new", ("__mcp_scratch__", tree_kind))?;
    let result = (|| -> PyResult<T> {
        let node = tree
            .getattr("nodes")?
            .call_method1("new", (bl_idname,))
            .map_err(|e| PyValueError::new_err(format!("cannot create `{bl_idname}`: {e}")))?;
        // Sockets depend on the node's configuration (ShaderNodeMix shows Color
        // sockets only when data_type is RGBA), so apply the caller's props first.
        if let Some(p) = props {
            for (k, v) in p {
                super::set_node_prop(&node, k, v)?;
            }
        }
        f(&node)
    })();

    // Remove the scratch tree whatever happened.
    groups.call_method1("remove", (&tree,))?;
    result
}

// ============================================================================
// node_types
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeTypes {
    /// One of: shader, geometry, compositor, texture, function. Omit for all of
    /// them. These are this tool's own groupings of Blender's node base classes,
    /// not an RNA enum.
    pub kind: Option<String>,
    /// Case-insensitive substring, matched against the type name and its label.
    /// Blender has 506 node types — search rather than asking for all of them.
    pub search: Option<String>,
    /// Cap on returned entries. Default 60. The result always reports how many
    /// matched, so a truncated list is visible as such.
    pub limit: Option<usize>,
}

crate::tool!(
    "node_types",
    "Search Blender's node types (506 of them, read live from Blender - shader, geometry, compositor, texture, function). Returns each type's bl_idname, which node_graph_build takes.",
    NodeTypes,
    |py: Python<'_>, p: NodeTypes| {
        if let Some(k) = p.kind.as_deref() {
            family(k)?; // validate up front, so a typo is an error and not an empty list
        }
        let needle = p.search.as_deref().map(str::to_lowercase);
        let limit = p.limit.unwrap_or(60);

        let mut matched = 0usize;
        let mut out: Vec<Value> = Vec::new();

        for (kind, id, label) in catalog(py)? {
            if let Some(want) = p.kind.as_deref()
                && *kind != want
            {
                continue;
            }
            if let Some(n) = &needle
                && !id.to_lowercase().contains(n)
                && !label.to_lowercase().contains(n)
            {
                continue;
            }
            matched += 1;
            if out.len() < limit {
                out.push(json!({ "type": id, "label": label, "kind": kind }));
            }
        }

        // Never truncate silently: a caller that sees 60 of 237 must know it.
        Ok(json!({
            "matched": matched,
            "returned": out.len(),
            "types": out,
        }))
    }
);

// ============================================================================
// node_schema
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeSchema {
    /// bl_idname, e.g. ShaderNodeBsdfPrincipled. Find it with node_types.
    pub r#type: String,
    /// The property configuration you intend to use, as {name: value}. Sockets
    /// depend on it: ShaderNodeMix only shows its Color sockets when data_type is
    /// RGBA. Pass the same map you will give node_graph_build's `props`.
    pub props: Option<Map<String, Value>>,
}

crate::tool!(
    "node_schema",
    "The real inputs, outputs and properties of a node type, read from Blender. \
     Call this BEFORE node_graph_build or node_graph_edit rather than guessing \
     socket names: Blender 5 renamed most of them (Principled's 'Specular' is now \
     'Specular IOR Level', 'Emission' split into 'Emission Color' and 'Emission \
     Strength'). node_types finds the bl_idname this takes.",
    NodeSchema,
    |py: Python<'_>, p: NodeSchema| {
        let ty = p.r#type.clone();
        with_scratch_node(py, &ty, p.props.as_ref(), |node| {
            let mut inputs = Vec::new();
            for s in enabled_sockets(node, true)? {
                inputs.push(socket_info(&s)?);
            }
            let mut outputs = Vec::new();
            for s in enabled_sockets(node, false)? {
                outputs.push(socket_info(&s)?);
            }

            // Only the node's own properties: everything on the base Node class
            // (location, label, mute…) is noise the model does not need here.
            let base = py.import("bpy")?.getattr("types")?.getattr("Node")?;
            let mut common = std::collections::HashSet::new();
            for name in base.getattr("bl_rna")?.getattr("properties")?.try_iter()? {
                common.insert(name?.getattr("identifier")?.extract::<String>()?);
            }

            let mut props = Vec::new();
            for prop in node.getattr("bl_rna")?.getattr("properties")?.try_iter()? {
                let prop = prop?;
                let id: String = prop.getattr("identifier")?.extract()?;
                if common.contains(&id) || prop.getattr("is_readonly")?.extract::<bool>()? {
                    continue;
                }
                let kind: String = prop.getattr("type")?.extract()?;
                let mut info = json!({ "id": id, "type": kind });
                // An enum's allowed values are the second place a model's memory goes
                // stale, right after socket names — so hand them over with the property.
                if let Some(vals) = super::enum_values(node, &id)? {
                    info["enum"] = json!(vals);
                }
                // A pointer property (Image Texture's `image`, a group node's
                // `node_tree`…) takes the NAME of an existing datablock. Say which kind,
                // or the caller has no way to know what to pass.
                if kind == "POINTER"
                    && let Ok(fixed) = prop.getattr("fixed_type")
                    && !fixed.is_none()
                {
                    info["id_type"] = json!(fixed.getattr("identifier")?.extract::<String>()?);
                    info["takes"] = json!("the name of an existing datablock");
                }
                props.push(info);
            }

            Ok(json!({
                "type": ty,
                "label": node.getattr("bl_label")?.extract::<String>()?,
                "inputs": inputs,
                "outputs": outputs,
                "props": props,
            }))
        })
    }
);
