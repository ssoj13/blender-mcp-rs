//! Graph-level tools: enumerate trees, create them, read a whole graph back, and
//! build/edit one in a single call.
//!
//! `node_graph_build` is the reason this module exists. Building a shader/geometry
//! graph one node at a time is one MCP round trip per node plus one per link — for a
//! ten-node graph that is dozens of calls. This lets the model describe the whole
//! graph once; everything else here (`node_tree_list`, `node_tree_create`,
//! `node_graph_get`, `node_graph_edit`) is the addressing, creation and inspection
//! that `node_graph_build` needs around it.
//!
//! No node-type table lives here, on purpose: node creation goes straight through
//! `tree.nodes.new(bl_idname)` and Blender's own error is what tells us a type is
//! bad. Socket lookup goes straight through `find_socket` in `mod.rs`. This module
//! adds graph-shaped concerns only: local ids, links, cycles, layout, and the
//! validation report.

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyTuple};
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::collections::{HashMap, HashSet};

use super::{
    enabled_sockets, find_socket, node_by_name, py_to_json, resolve_tree, set_node_prop,
    set_socket_value, socket_info, tree_kind,
};
use crate::tools::bpy::py_len;

// ============================================================================
// Small shared helpers
// ============================================================================

/// `[x, y]` -> `(x, y)`, with a message that says what was actually given.
fn parse_location(loc: &[f64]) -> PyResult<(f64, f64)> {
    match loc {
        [x, y] => Ok((*x, *y)),
        other => Err(PyValueError::new_err(format!(
            "location must be [x, y], got {} value(s)",
            other.len()
        ))),
    }
}

fn set_location(node: &Bound<'_, PyAny>, loc: &[f64]) -> PyResult<()> {
    let (x, y) = parse_location(loc)?;
    node.setattr("location", PyTuple::new(node.py(), [x, y])?)
}

/// A `[from, to]` pair out of a JSON array of arbitrary length — callers write
/// `links: [["a.X", "b.Y"]]`, and a wrong arity here is a caller mistake, not a
/// Blender one, so it is caught before anything Blender-related runs.
fn pair<'a>(v: &'a [String], what: &str) -> PyResult<(&'a str, &'a str)> {
    match v {
        [a, b] => Ok((a.as_str(), b.as_str())),
        other => Err(PyValueError::new_err(format!(
            "{what} entry must be [from, to], got {} item(s)",
            other.len()
        ))),
    }
}

/// `"id.SocketName"` -> `("id", "SocketName")`.
fn split_endpoint(spec: &str) -> PyResult<(&str, &str)> {
    spec.split_once('.').ok_or_else(|| {
        PyValueError::new_err(format!(
            "link endpoint `{spec}` must be `id.SocketName` (a node id or name, a dot, \
             then the socket name)"
        ))
    })
}

/// Whether two Bound values are the SAME underlying Blender struct.
///
/// Python `is` on bpy RNA wrappers is **not** reliable for this: Blender mints a
/// fresh Python wrapper object on almost every attribute access, so two back-to-back
/// reads of the very same node (`tree.nodes["A"] is tree.nodes["A"]`) come back
/// `False` — verified live on Blender 5. `as_pointer()` returns the underlying C
/// pointer and is Blender's own documented way to compare "is this the same thing".
fn same(a: &Bound<'_, PyAny>, b: &Bound<'_, PyAny>) -> PyResult<bool> {
    let pa: u64 = a.call_method0("as_pointer")?.extract()?;
    let pb: u64 = b.call_method0("as_pointer")?.extract()?;
    Ok(pa == pb)
}

fn contains(haystack: &[Bound<'_, PyAny>], needle: &Bound<'_, PyAny>) -> PyResult<bool> {
    for h in haystack {
        if same(h, needle)? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// True if adding a `from -> to` link would close a cycle: walk the tree's *existing*
/// links forward from `to`, and see whether that walk can reach `from`. If it can,
/// `from` already sits downstream of `to`, and wiring `from -> to` would feed `to`
/// back into itself.
fn would_cycle<'py>(
    tree: &Bound<'py, PyAny>,
    from: &Bound<'py, PyAny>,
    to: &Bound<'py, PyAny>,
) -> PyResult<bool> {
    if same(from, to)? {
        return Ok(true);
    }
    let links = tree.getattr("links")?;
    let mut frontier = vec![to.clone()];
    let mut seen: Vec<Bound<'py, PyAny>> = Vec::new();
    while let Some(cur) = frontier.pop() {
        if contains(&seen, &cur)? {
            continue;
        }
        for link in links.try_iter()? {
            let link = link?;
            if same(&link.getattr("from_node")?, &cur)? {
                let next = link.getattr("to_node")?;
                if same(&next, from)? {
                    return Ok(true);
                }
                frontier.push(next);
            }
        }
        seen.push(cur);
    }
    Ok(false)
}

/// The existing link between exactly these two sockets, if any.
fn find_link<'py>(
    tree: &Bound<'py, PyAny>,
    from_socket: &Bound<'py, PyAny>,
    to_socket: &Bound<'py, PyAny>,
) -> PyResult<Option<Bound<'py, PyAny>>> {
    for link in tree.getattr("links")?.try_iter()? {
        let link = link?;
        if same(&link.getattr("from_socket")?, from_socket)?
            && same(&link.getattr("to_socket")?, to_socket)?
        {
            return Ok(Some(link));
        }
    }
    Ok(None)
}

/// The node's own properties that differ from their RNA default. Mirrors the
/// filtering `node_schema` uses (skip base-`Node` properties and read-only ones),
/// so a caller sees exactly the configuration that makes this node *this* node.
fn nondefault_props(node: &Bound<'_, PyAny>) -> PyResult<Map<String, Value>> {
    let py = node.py();
    let base = py.import("bpy")?.getattr("types")?.getattr("Node")?;
    let mut common = HashSet::new();
    for name in base.getattr("bl_rna")?.getattr("properties")?.try_iter()? {
        common.insert(name?.getattr("identifier")?.extract::<String>()?);
    }

    let mut out = Map::new();
    for prop in node.getattr("bl_rna")?.getattr("properties")?.try_iter()? {
        let prop = prop?;
        let id: String = prop.getattr("identifier")?.extract()?;
        if common.contains(&id) || prop.getattr("is_readonly")?.extract::<bool>()? {
            continue;
        }
        let current = node.getattr(id.as_str())?;
        let ptype: String = prop.getattr("type")?.extract()?;

        if ptype == "POINTER" {
            // No comparable "default" for a pointer — report it only when it is
            // actually set, under whatever name that data-block has.
            if !current.is_none()
                && let Ok(name) = current.getattr("name")
                && let Ok(name) = name.extract::<String>()
            {
                out.insert(id, json!(name));
            }
            continue;
        }

        let is_array = prop
            .getattr("is_array")
            .and_then(|v| v.extract())
            .unwrap_or(false);
        let default = if is_array {
            prop.getattr("default_array").ok()
        } else {
            prop.getattr("default").ok()
        };
        // No default to compare against (e.g. a COLLECTION-typed property) — report
        // it rather than silently drop it.
        let differs = match &default {
            Some(d) => py_to_json(d) != py_to_json(&current),
            None => true,
        };
        if differs && let Some(v) = py_to_json(&current) {
            out.insert(id, v);
        }
    }
    Ok(out)
}

// ============================================================================
// node_tree_list
// ============================================================================

/// `node_tree_list` takes no arguments.
#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeTreeList {}

fn tree_row(address: &str, tree: &Bound<'_, PyAny>) -> PyResult<Value> {
    Ok(json!({
        "address": address,
        "kind": tree_kind(tree)?,
        "nodes": py_len(tree.py(), &tree.getattr("nodes")?)?,
    }))
}

crate::tool!(
    "node_tree_list",
    "Every node tree in the file, with its address: material:<name>, world, \
     compositor, group:<name>, object:<object>/<modifier>. Each entry has the \
     tree's kind (bl_idname) and node count.",
    NodeTreeList,
    |py: Python<'_>, _p: NodeTreeList| {
        let bpy = py.import("bpy")?;
        let data = bpy.getattr("data")?;
        let mut out = Vec::new();

        for mat in data.getattr("materials")?.try_iter()? {
            let mat = mat?;
            let nt = mat.getattr("node_tree")?;
            if nt.is_none() {
                continue;
            }
            let name: String = mat.getattr("name")?.extract()?;
            out.push(tree_row(&format!("material:{name}"), &nt)?);
        }

        let scene = bpy.getattr("context")?.getattr("scene")?;
        let world = scene.getattr("world")?;
        if !world.is_none() {
            let nt = world.getattr("node_tree")?;
            if !nt.is_none() {
                out.push(tree_row("world", &nt)?);
            }
        }

        // Blender 5: the compositor tree hangs off the scene but is *also* a real
        // entry in bpy.data.node_groups. Remember it so the group loop below does
        // not list the same tree a second time under `group:<name>`.
        let compositor = scene.getattr("compositing_node_group")?;
        if !compositor.is_none() {
            out.push(tree_row("compositor", &compositor)?);
        }

        for grp in data.getattr("node_groups")?.try_iter()? {
            let grp = grp?;
            if !compositor.is_none() && same(&grp, &compositor)? {
                continue;
            }
            let name: String = grp.getattr("name")?.extract()?;
            out.push(tree_row(&format!("group:{name}"), &grp)?);
        }

        for obj in data.getattr("objects")?.try_iter()? {
            let obj = obj?;
            for m in obj.getattr("modifiers")?.try_iter()? {
                let m = m?;
                if m.getattr("type")?.extract::<String>()? != "NODES" {
                    continue;
                }
                let group = m.getattr("node_group")?;
                if group.is_none() {
                    continue;
                }
                let obj_name: String = obj.getattr("name")?.extract()?;
                let mod_name: String = m.getattr("name")?.extract()?;
                out.push(tree_row(&format!("object:{obj_name}/{mod_name}"), &group)?);
            }
        }

        Ok(json!({ "trees": out }))
    }
);

// ============================================================================
// node_tree_create
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeTreeCreate {
    /// What to create, exactly one of: `material`, `group`, `geometry_modifier`.
    pub kind: String,
    /// Name for the new material / node group / modifier. Blender renames on
    /// collision, so the address in the response is the authority.
    pub name: String,
    /// Required when kind=geometry_modifier: the object to add the NODES modifier
    /// to. Ignored for the other kinds.
    pub object: Option<String>,
    /// kind=group only: the tree's bl_idname. Default ShaderNodeTree; Blender 5.2
    /// accepts exactly ShaderNodeTree, GeometryNodeTree, CompositorNodeTree,
    /// TextureNodeTree (verified live). A group's kind fixes which socket types
    /// node_interface_add will take.
    pub node_tree_type: Option<String>,
}

crate::tool!(
    "node_tree_create",
    "Create a node tree and return its address. kind=material makes the material's \
     shader tree (Blender fills in a default Principled BSDF + Material Output). \
     kind=group makes a standalone node group. kind=geometry_modifier adds a NODES \
     modifier on `object` with a fresh geometry tree that already has Group Input \
     -> Group Output wired through a Geometry socket, so the modifier is usable \
     immediately rather than an empty shell.",
    NodeTreeCreate,
    |py: Python<'_>, p: NodeTreeCreate| {
        let bpy = py.import("bpy")?;
        let data = bpy.getattr("data")?;

        match p.kind.as_str() {
            "material" => {
                let mat = data
                    .getattr("materials")?
                    .call_method1("new", (p.name.as_str(),))?;
                mat.setattr("use_nodes", true)?;
                let actual: String = mat.getattr("name")?.extract()?;
                let tree = mat.getattr("node_tree")?;
                Ok(json!({
                    "address": format!("material:{actual}"),
                    "kind": tree_kind(&tree)?,
                }))
            }
            "group" => {
                let tt = p.node_tree_type.as_deref().unwrap_or("ShaderNodeTree");
                let grp = data
                    .getattr("node_groups")?
                    .call_method1("new", (p.name.as_str(), tt))
                    .map_err(|e| {
                        PyValueError::new_err(format!(
                            "cannot create a `{tt}` node group: {e} (try ShaderNodeTree, \
                             GeometryNodeTree, CompositorNodeTree or TextureNodeTree)"
                        ))
                    })?;
                let actual: String = grp.getattr("name")?.extract()?;
                Ok(json!({
                    "address": format!("group:{actual}"),
                    "kind": tree_kind(&grp)?,
                }))
            }
            "geometry_modifier" => {
                let obj_name = p.object.as_deref().ok_or_else(|| {
                    PyValueError::new_err("kind=geometry_modifier requires `object`")
                })?;
                let obj = data.getattr("objects")?.call_method1("get", (obj_name,))?;
                if obj.is_none() {
                    return Err(PyValueError::new_err(format!(
                        "object not found: {obj_name}"
                    )));
                }

                // Verified on Blender 5: a NODES modifier starts with node_group =
                // None. The tree has to be created and assigned by hand, and needs
                // its own Geometry in/out sockets or the modifier does nothing.
                let tree = data
                    .getattr("node_groups")?
                    .call_method1("new", (p.name.as_str(), "GeometryNodeTree"))?;
                let interface = tree.getattr("interface")?;
                for in_out in ["INPUT", "OUTPUT"] {
                    let kw = PyDict::new(py);
                    kw.set_item("name", "Geometry")?;
                    kw.set_item("in_out", in_out)?;
                    kw.set_item("socket_type", "NodeSocketGeometry")?;
                    interface.call_method("new_socket", (), Some(&kw))?;
                }

                let nodes = tree.getattr("nodes")?;
                let gin = nodes.call_method1("new", ("NodeGroupInput",))?;
                set_location(&gin, &[-300.0, 0.0])?;
                let gout = nodes.call_method1("new", ("NodeGroupOutput",))?;
                set_location(&gout, &[300.0, 0.0])?;

                // Wire the pass-through so the modifier does something useful the
                // moment it exists, not just once someone edits the tree.
                let from_socket = find_socket(&gin, &json!("Geometry"), false)?;
                let to_socket = find_socket(&gout, &json!("Geometry"), true)?;
                tree.getattr("links")?
                    .call_method1("new", (from_socket, to_socket))?;

                let modifiers = obj.getattr("modifiers")?;
                let kw = PyDict::new(py);
                kw.set_item("name", p.name.as_str())?;
                kw.set_item("type", "NODES")?;
                let m = modifiers.call_method("new", (), Some(&kw))?;
                m.setattr("node_group", &tree)?;

                let actual_obj: String = obj.getattr("name")?.extract()?;
                let actual_mod: String = m.getattr("name")?.extract()?;
                Ok(json!({
                    "address": format!("object:{actual_obj}/{actual_mod}"),
                    "kind": tree_kind(&tree)?,
                }))
            }
            other => Err(PyValueError::new_err(format!(
                "unknown kind `{other}`: expected material, group or geometry_modifier"
            ))),
        }
    }
);

// ============================================================================
// node_graph_get
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeGraphGet {
    /// Tree address as node_tree_list prints it: `material:<name>`, `world`,
    /// `compositor`, `group:<name>` or `object:<object>/<modifier>`.
    pub tree: String,
}

crate::tool!(
    "node_graph_get",
    "Full view of a node tree: every node's name, type, label, location, \
     non-default properties, and its enabled input/output sockets (name, type, \
     current value, whether linked) — plus every link as \
     \"FromNode.Socket -> ToNode.Socket\". The state-of-the-tree read before a \
     node_graph_edit; node_schema answers what a node type COULD have, this \
     answers what this tree DOES have.",
    NodeGraphGet,
    |py: Python<'_>, p: NodeGraphGet| {
        let tree = resolve_tree(py, &p.tree)?;

        let mut nodes_out = Vec::new();
        for node in tree.getattr("nodes")?.try_iter()? {
            let node = node?;
            let loc = node.getattr("location")?;
            let x: f64 = loc.get_item(0)?.extract()?;
            let y: f64 = loc.get_item(1)?.extract()?;

            let mut inputs = Vec::new();
            for s in enabled_sockets(&node, true)? {
                inputs.push(socket_info(&s)?);
            }
            let mut outputs = Vec::new();
            for s in enabled_sockets(&node, false)? {
                outputs.push(socket_info(&s)?);
            }

            nodes_out.push(json!({
                "name": node.getattr("name")?.extract::<String>()?,
                "type": node.getattr("bl_idname")?.extract::<String>()?,
                "label": node.getattr("label")?.extract::<String>()?,
                "location": [x, y],
                "props": nondefault_props(&node)?,
                "inputs": inputs,
                "outputs": outputs,
            }));
        }

        let mut links_out = Vec::new();
        for link in tree.getattr("links")?.try_iter()? {
            let link = link?;
            let fn_name: String = link.getattr("from_node")?.getattr("name")?.extract()?;
            let fs_name: String = link.getattr("from_socket")?.getattr("name")?.extract()?;
            let tn_name: String = link.getattr("to_node")?.getattr("name")?.extract()?;
            let ts_name: String = link.getattr("to_socket")?.getattr("name")?.extract()?;
            links_out.push(json!(format!("{fn_name}.{fs_name} -> {tn_name}.{ts_name}")));
        }

        Ok(json!({
            "tree": p.tree,
            "kind": tree_kind(&tree)?,
            "nodes": nodes_out,
            "links": links_out,
        }))
    }
);

// ============================================================================
// node_graph_build
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct GraphNodeSpec {
    /// Local handle for this node, used to reference it from `links` and returned
    /// in `created`. This is NOT the Blender node name — Blender renames on
    /// collision, so the real name may differ; `created` reports what it actually
    /// ended up being.
    pub id: String,
    /// bl_idname, e.g. ShaderNodeTexNoise. Search node_types for valid values, and
    /// node_schema for the socket and property names this type really has.
    pub r#type: String,
    /// Properties applied right after creation, before `inputs` — sockets can
    /// depend on props (ShaderNodeMix only shows Color sockets when data_type=RGBA).
    pub props: Option<Map<String, Value>>,
    /// Unlinked input socket values, keyed by socket NAME exactly as node_schema
    /// reports it (Blender 5 renamed many: `Specular IOR Level`, `Emission Color`,
    /// `Transmission Weight`, `Factor` not `Fac`). Numbers, booleans, strings,
    /// [x, y, z] vectors and [r, g, b, a] colours; a datablock socket takes the
    /// datablock's name (e.g. `image`: the name img_load returned).
    pub inputs: Option<Map<String, Value>>,
    /// [x, y] position in the node editor. Omitted nodes are auto-laid-out in
    /// columns by how many links feed into them.
    pub location: Option<Vec<f64>>,
    /// Only for a geometry ZONE input (Simulation / Repeat / For Each): the `id` of
    /// its matching Output node. Zones are node PAIRS, and an unpaired input has no
    /// geometry sockets at all. Omit it and the pairing is inferred when exactly one
    /// matching Output node is in the same build.
    pub pair: Option<String>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeGraphBuild {
    /// Tree address as node_tree_list prints it: `material:<name>`, `world`,
    /// `compositor`, `group:<name>` or `object:<object>/<modifier>`.
    pub tree: String,
    /// Wipe every existing node first. Default false: add to what is already
    /// there. With clear:true a `links` endpoint can only name an id declared in
    /// this same call, and a material loses its `Principled BSDF` node, which is
    /// what mat_update needs by name.
    pub clear: Option<bool>,
    /// The nodes to create, in order. Each is {id, type, props?, inputs?,
    /// location?, pair?}. May be empty when the call only adds links between
    /// existing nodes.
    pub nodes: Vec<GraphNodeSpec>,
    /// [["from_id.Socket", "to_id.Socket"], ...]. An id is either a node declared
    /// above, or — only when clear is false — an existing node's real Blender name.
    pub links: Option<Vec<Vec<String>>>,
}

/// Resolve a link endpoint's id to a node: one created earlier in this same call, or
/// (when the tree was not wiped) an existing node addressed by its real Blender
/// name. A request id always wins over a same-named existing node — the id is the
/// caller's handle and must mean what the caller thinks it means, even if Blender
/// had to rename the node to something else on creation.
fn resolve_node_ref<'py>(
    tree: &Bound<'py, PyAny>,
    created: &HashMap<String, Bound<'py, PyAny>>,
    id: &str,
    allow_existing: bool,
) -> PyResult<Bound<'py, PyAny>> {
    if let Some(n) = created.get(id) {
        return Ok(n.clone());
    }
    if allow_existing {
        return node_by_name(tree, id);
    }
    let known: Vec<&str> = created.keys().map(String::as_str).collect();
    Err(PyValueError::new_err(format!(
        "link references unknown id `{id}`. Ids in this request: {}",
        known.join(", ")
    )))
}

/// Column-by-depth auto layout: a node sits one column right of everything that
/// feeds it, and nodes sharing a column are spread out vertically. Only touches
/// nodes the caller did not already place explicitly. Depth is a longest-path
/// relaxation over the request's own links, which terminates cleanly because
/// `would_cycle` already refused any link that would make this graph cyclic.
fn layout(
    specs: &[GraphNodeSpec],
    created: &HashMap<String, Bound<'_, PyAny>>,
    links: &[(String, String, String, String)],
) -> PyResult<()> {
    const COL_W: f64 = 300.0;
    const ROW_H: f64 = 220.0;

    let mut depth: HashMap<&str, u32> = specs.iter().map(|s| (s.id.as_str(), 0)).collect();
    let mut changed = true;
    while changed {
        changed = false;
        for (from, _, to, _) in links {
            let (Some(&fd), Some(&td)) = (depth.get(from.as_str()), depth.get(to.as_str())) else {
                continue; // endpoint outside this request (an existing node)
            };
            if fd + 1 > td {
                depth.insert(to.as_str(), fd + 1);
                changed = true;
            }
        }
    }

    let mut next_row: HashMap<u32, u32> = HashMap::new();
    for spec in specs {
        if spec.location.is_some() {
            continue; // caller placed it explicitly
        }
        let Some(node) = created.get(&spec.id) else {
            continue;
        };
        let col = depth[spec.id.as_str()];
        let row = next_row.entry(col).or_insert(0);
        let x = f64::from(col) * COL_W;
        let y = -f64::from(*row) * ROW_H;
        *row += 1;
        node.setattr("location", PyTuple::new(node.py(), [x, y])?)?;
    }
    Ok(())
}

/// True if this node has no enabled output sockets — the generic, table-free
/// definition of an "output"/terminal node: Material Output, Group Output and
/// Composite all qualify, without naming any of them.
fn is_output_node(node: &Bound<'_, PyAny>) -> PyResult<bool> {
    Ok(enabled_sockets(node, false)?.is_empty())
}

/// BFS forward through `node`'s outgoing links until an output node is reached.
fn reaches_output(tree: &Bound<'_, PyAny>, node: &Bound<'_, PyAny>) -> PyResult<bool> {
    if is_output_node(node)? {
        return Ok(true);
    }
    let links = tree.getattr("links")?;
    let mut frontier = vec![node.clone()];
    let mut seen: Vec<Bound<'_, PyAny>> = Vec::new();
    while let Some(cur) = frontier.pop() {
        if contains(&seen, &cur)? {
            continue;
        }
        for link in links.try_iter()? {
            let link = link?;
            if same(&link.getattr("from_node")?, &cur)? {
                let next = link.getattr("to_node")?;
                if is_output_node(&next)? {
                    return Ok(true);
                }
                frontier.push(next);
            }
        }
        seen.push(cur);
    }
    Ok(false)
}

/// Data, not errors: which of the nodes this call created cannot reach an output
/// node, and whether the tree has an output node at all. Scoped to this call's own
/// nodes so a build against a large pre-existing tree is not buried in unrelated
/// warnings about nodes nobody touched.
fn graph_warnings(
    tree: &Bound<'_, PyAny>,
    created: &HashMap<String, Bound<'_, PyAny>>,
) -> PyResult<Vec<Value>> {
    let mut warnings = Vec::new();

    let mut has_output = false;
    for n in tree.getattr("nodes")?.try_iter()? {
        if is_output_node(&n?)? {
            has_output = true;
            break;
        }
    }
    if !has_output {
        warnings.push(json!(
            "tree has no output node (a node with no output sockets, e.g. \
             Material Output / Group Output / Composite)"
        ));
        return Ok(warnings); // nothing can "reach" an output that does not exist
    }

    for (id, node) in created {
        if reaches_output(tree, node)? {
            continue;
        }
        let name: String = node.getattr("name")?.extract()?;
        warnings.push(json!(format!(
            "node `{name}` (id `{id}`) has no path to an output node"
        )));
    }
    Ok(warnings)
}

crate::tool!(
    "node_graph_build",
    "Build a whole node graph in one call: create nodes, apply their properties, \
     set input values, wire links, and auto-layout the result. The main way to \
     CONSTRUCT a shader/geometry/compositor graph — call node_schema first for the \
     real socket and property names, and node_graph_edit afterwards to adjust a \
     tree that already exists rather than rebuilding it. \
     `clear: true` wipes the tree first (see `clear` docs for what that means if a \
     later part of the same call fails). Every node gets a `location` unless given \
     one explicitly. Guards (duplicate id, malformed link, unknown id, cycle) are \
     checked before any node is created; an unknown node type or socket surfaces \
     only once creation reaches it, and any failure rolls back every node this call \
     created, so the tree is left as if the call had not run (minus an explicit \
     `clear`).",
    NodeGraphBuild,
    |py: Python<'_>, p: NodeGraphBuild| {
        let tree = resolve_tree(py, &p.tree)?;
        let clear = p.clear.unwrap_or(false);
        let raw_links = p.links.unwrap_or_default();

        // ---- guards that need no Blender mutation ----
        let mut ids = HashSet::new();
        for n in &p.nodes {
            if !ids.insert(n.id.as_str()) {
                return Err(PyValueError::new_err(format!(
                    "duplicate node id in request: `{}`",
                    n.id
                )));
            }
        }

        let mut parsed_links = Vec::with_capacity(raw_links.len());
        for l in &raw_links {
            let (from, to) = pair(l, "links")?;
            let (from_id, from_sock) = split_endpoint(from)?;
            let (to_id, to_sock) = split_endpoint(to)?;
            for (id, side) in [(from_id, "from"), (to_id, "to")] {
                if ids.contains(id) {
                    continue;
                }
                // Not declared in this request. When the tree is about to be
                // wiped, nothing outside this request will exist by link time —
                // that makes it an unknown id right now, before Blender is touched.
                if clear || node_by_name(&tree, id).is_err() {
                    let extra = if clear {
                        ""
                    } else {
                        " and no such node exists in the tree"
                    };
                    return Err(PyValueError::new_err(format!(
                        "link {side} `{id}` is not a node id in this request{extra}"
                    )));
                }
            }
            parsed_links.push((
                from_id.to_string(),
                from_sock.to_string(),
                to_id.to_string(),
                to_sock.to_string(),
            ));
        }

        // ---- mutate ----
        if clear {
            let nodes = tree.getattr("nodes")?;
            for n in nodes.try_iter()?.collect::<PyResult<Vec<_>>>()? {
                nodes.call_method1("remove", (n,))?;
            }
        }

        let mut created: HashMap<String, Bound<'_, PyAny>> = HashMap::new();
        let build = (|| -> PyResult<()> {
            for spec in &p.nodes {
                let node = tree
                    .getattr("nodes")?
                    .call_method1("new", (spec.r#type.as_str(),))
                    .map_err(|e| {
                        PyValueError::new_err(format!(
                            "unknown node type `{}` (search node_types for a valid \
                             bl_idname): {e}",
                            spec.r#type
                        ))
                    })?;
                // Track it for rollback *before* anything else that can fail: the
                // node already exists in the real tree the moment `new()` returns,
                // regardless of whether its props/inputs go on to succeed.
                created.insert(spec.id.clone(), node.clone());

                node.setattr("name", spec.id.as_str())?;
                if let Some(loc) = &spec.location {
                    set_location(&node, loc)?;
                }
                if let Some(props) = &spec.props {
                    for (k, v) in props {
                        set_node_prop(&node, k, v)?;
                    }
                }
                if let Some(inputs) = &spec.inputs {
                    for (k, v) in inputs {
                        let socket = find_socket(&node, &Value::String(k.clone()), true)?;
                        set_socket_value(&socket, v)?;
                    }
                }
            }

            // Zones (Simulation / Repeat / For Each) are node PAIRS. Until an input is
            // paired with its output, it has no geometry sockets at all — so this has
            // to happen before any link is made, or the links have nothing to attach
            // to. Blender exposes `pair_with_output` on exactly the three zone inputs,
            // so we ask the node rather than keeping a list.
            for spec in &p.nodes {
                let Some(node) = created.get(&spec.id) else {
                    continue;
                };
                if !node.hasattr("pair_with_output")? {
                    if spec.pair.is_some() {
                        return Err(PyValueError::new_err(format!(
                            "`{}` ({}) is not a zone input, so it takes no `pair`",
                            spec.id, spec.r#type
                        )));
                    }
                    continue;
                }

                // The partner type is the input type with Input -> Output; that holds
                // for all three zones.
                let want_type = spec
                    .r#type
                    .strip_suffix("Input")
                    .map(|stem| format!("{stem}Output"))
                    .unwrap_or_default();

                let partner = match &spec.pair {
                    Some(id) => created.get(id).ok_or_else(|| {
                        PyValueError::new_err(format!(
                            "`{}`.pair refers to `{id}`, which is not in this build",
                            spec.id
                        ))
                    })?,
                    None => {
                        let mut matches = Vec::new();
                        for other in &p.nodes {
                            if other.r#type == want_type {
                                matches.push(&other.id);
                            }
                        }
                        match matches.as_slice() {
                            [only] => created.get(*only).ok_or_else(|| {
                                PyRuntimeError::new_err("paired node vanished mid-build")
                            })?,
                            [] => {
                                return Err(PyValueError::new_err(format!(
                                    "`{}` ({}) is a zone input and needs a matching {want_type} \
                                     in the same build — a zone is a pair of nodes, and an \
                                     unpaired input has no geometry sockets",
                                    spec.id, spec.r#type
                                )));
                            }
                            _ => {
                                return Err(PyValueError::new_err(format!(
                                    "`{}` has several candidate {want_type} nodes; say which \
                                     one with \"pair\": \"<id>\"",
                                    spec.id
                                )));
                            }
                        }
                    }
                };

                node.call_method1("pair_with_output", (partner,))?;
            }

            let links_coll = tree.getattr("links")?;
            for (from_id, from_sock, to_id, to_sock) in &parsed_links {
                let from_node = resolve_node_ref(&tree, &created, from_id, !clear)?;
                let to_node = resolve_node_ref(&tree, &created, to_id, !clear)?;
                let from_socket =
                    find_socket(&from_node, &Value::String(from_sock.clone()), false)?;
                let to_socket = find_socket(&to_node, &Value::String(to_sock.clone()), true)?;
                if would_cycle(&tree, &from_node, &to_node)? {
                    return Err(PyValueError::new_err(format!(
                        "link `{from_id}.{from_sock}` -> `{to_id}.{to_sock}` would create a cycle"
                    )));
                }
                links_coll.call_method1("new", (from_socket, to_socket))?;
            }

            layout(&p.nodes, &created, &parsed_links)
        })();

        if let Err(e) = build {
            // Roll back this call's own additions so a mid-build failure leaves the
            // tree as it was before the call — except for an explicit `clear`,
            // which the caller asked for outright and which this does not undo
            // (util_undo does, if that is needed).
            let nodes = tree.getattr("nodes")?;
            for (id, node) in &created {
                if let Err(re) = nodes.call_method1("remove", (node,)) {
                    eprintln!(
                        "[blender-mcp] node_graph_build rollback: could not remove `{id}`: {re}"
                    );
                }
            }
            return Err(e);
        }

        let mut id_map = Map::new();
        for (id, node) in &created {
            let real: String = node.getattr("name")?.extract()?;
            id_map.insert(id.clone(), json!(real));
        }

        Ok(json!({
            "tree": p.tree,
            "created": id_map,
            "links": parsed_links.len(),
            "warnings": graph_warnings(&tree, &created)?,
        }))
    }
);

// ============================================================================
// node_graph_edit
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MuteEntry {
    /// Real Blender node name, as node_graph_get reports it.
    pub node: String,
    /// true mutes the node (it passes its input through), false un-mutes it.
    pub muted: bool,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct MoveEntry {
    /// Real Blender node name, as node_graph_get reports it.
    pub node: String,
    /// [x, y] in node-editor units. Cosmetic: it changes nothing about the render.
    pub location: Vec<f64>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SetPropsEntry {
    /// Real Blender node name, as node_graph_get reports it.
    pub node: String,
    /// Node PROPERTIES (RNA fields such as `operation`, `data_type`,
    /// `noise_dimensions`), not sockets — node_schema lists both separately.
    /// Changing one can add or remove sockets.
    pub props: Map<String, Value>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct SetInputsEntry {
    /// Real Blender node name, as node_graph_get reports it.
    pub node: String,
    /// Input socket NAME -> value, same shapes as node_graph_build's `inputs`.
    /// A linked socket's value is ignored by Blender until the link is removed.
    pub inputs: Map<String, Value>,
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeGraphEdit {
    /// Tree address as node_tree_list prints it: `material:<name>`, `world`,
    /// `compositor`, `group:<name>` or `object:<object>/<modifier>`.
    pub tree: String,
    /// Real Blender node names to delete. Their links go with them.
    pub remove: Option<Vec<String>>,
    /// [{node, muted: true|false}, ...].
    pub mute: Option<Vec<MuteEntry>>,
    /// [{node, location: [x, y]}, ...].
    pub r#move: Option<Vec<MoveEntry>>,
    /// [{node, props: {name: value, ...}}, ...] — RNA properties, not sockets.
    pub set_props: Option<Vec<SetPropsEntry>>,
    /// [{node, inputs: {"Socket Name": value, ...}}, ...].
    pub set_inputs: Option<Vec<SetInputsEntry>>,
    /// [["from_node.Socket", "to_node.Socket"], ...] — node names are real Blender
    /// names here (this edits an existing tree; there are no local ids). A link
    /// that would close a cycle is refused.
    pub link: Option<Vec<Vec<String>>>,
    /// [["from_node.Socket", "to_node.Socket"], ...] — the same endpoint spelling
    /// as `link`. Removes those links; an endpoint pair with no link is an error,
    /// not a silent no-op.
    pub unlink: Option<Vec<Vec<String>>>,
}

crate::tool!(
    "node_graph_edit",
    "Incremental edits on an existing node tree: remove, mute, move, set_props, \
     set_inputs, link, unlink. Use this to CHANGE a tree that already exists — \
     node_graph_build creates nodes, node_graph_get shows what is there now, and \
     node_schema gives the socket/property names both of them take. \
     Every list is optional and, when present, is applied \
     in that order — this is a short script against the tree, not an atomic \
     transaction, so a failure partway through leaves the steps before it applied.",
    NodeGraphEdit,
    |py: Python<'_>, p: NodeGraphEdit| {
        let tree = resolve_tree(py, &p.tree)?;
        let nodes = tree.getattr("nodes")?;

        let mut removed = 0usize;
        for name in p.remove.unwrap_or_default() {
            let node = node_by_name(&tree, &name)?;
            nodes.call_method1("remove", (node,))?;
            removed += 1;
        }

        let mut muted = 0usize;
        for e in p.mute.unwrap_or_default() {
            let node = node_by_name(&tree, &e.node)?;
            node.setattr("mute", e.muted)?;
            muted += 1;
        }

        let mut moved = 0usize;
        for e in p.r#move.unwrap_or_default() {
            let node = node_by_name(&tree, &e.node)?;
            set_location(&node, &e.location)?;
            moved += 1;
        }

        let mut props_set = 0usize;
        for e in p.set_props.unwrap_or_default() {
            let node = node_by_name(&tree, &e.node)?;
            for (k, v) in &e.props {
                set_node_prop(&node, k, v)?;
            }
            props_set += 1;
        }

        let mut inputs_set = 0usize;
        for e in p.set_inputs.unwrap_or_default() {
            let node = node_by_name(&tree, &e.node)?;
            for (k, v) in &e.inputs {
                let socket = find_socket(&node, &Value::String(k.clone()), true)?;
                set_socket_value(&socket, v)?;
            }
            inputs_set += 1;
        }

        let links_coll = tree.getattr("links")?;

        let mut linked = 0usize;
        for l in p.link.unwrap_or_default() {
            let (from, to) = pair(&l, "link")?;
            let (from_id, from_sock) = split_endpoint(from)?;
            let (to_id, to_sock) = split_endpoint(to)?;
            let from_node = node_by_name(&tree, from_id)?;
            let to_node = node_by_name(&tree, to_id)?;
            let from_socket = find_socket(&from_node, &json!(from_sock), false)?;
            let to_socket = find_socket(&to_node, &json!(to_sock), true)?;
            if would_cycle(&tree, &from_node, &to_node)? {
                return Err(PyValueError::new_err(format!(
                    "link `{from}` -> `{to}` would create a cycle"
                )));
            }
            links_coll.call_method1("new", (from_socket, to_socket))?;
            linked += 1;
        }

        let mut unlinked = 0usize;
        for l in p.unlink.unwrap_or_default() {
            let (from, to) = pair(&l, "unlink")?;
            let (from_id, from_sock) = split_endpoint(from)?;
            let (to_id, to_sock) = split_endpoint(to)?;
            let from_node = node_by_name(&tree, from_id)?;
            let to_node = node_by_name(&tree, to_id)?;
            let from_socket = find_socket(&from_node, &json!(from_sock), false)?;
            let to_socket = find_socket(&to_node, &json!(to_sock), true)?;
            let Some(link) = find_link(&tree, &from_socket, &to_socket)? else {
                return Err(PyValueError::new_err(format!(
                    "no link `{from}` -> `{to}` to remove"
                )));
            };
            links_coll.call_method1("remove", (link,))?;
            unlinked += 1;
        }

        Ok(json!({
            "tree": p.tree,
            "removed": removed,
            "muted": muted,
            "moved": moved,
            "props_set": props_set,
            "inputs_set": inputs_set,
            "linked": linked,
            "unlinked": unlinked,
        }))
    }
);
