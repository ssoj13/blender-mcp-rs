//! Node groups and the geometry-modifier bridge.
//!
//! Four tools:
//! - `node_group_make` — collapse existing nodes into a reusable group (built by
//!   hand: `bpy.ops.node.group_make` needs a `NODE_EDITOR` area, and
//!   `bpy.ops.node.group_make.poll()` was verified to return `False` with no window
//!   present — i.e. always, when this server runs against `blender --background`, the
//!   normal way this project drives Blender. See "Why `node_group_make` is manual"
//!   below).
//! - `node_group_add` — instance an existing group as a node.
//! - `node_interface_add` — add an input/output socket to a group's interface.
//! - `geo_modifier_set` — set a Geometry Nodes modifier's input values by socket
//!   name, on the object (not the tree — this is the one operation in the node
//!   layer that is not about a node tree at all).
//!
//! ## Facts this file depends on, and how they were checked
//! Every claim below was verified against a live Blender 5.0.0
//! (`blender.exe --background --factory-startup --python probe.py`), not assumed:
//!
//! - `tree.interface.new_socket(name, in_out=.., socket_type=..)` returns the created
//!   `NodeTreeInterfaceSocket`; its `.identifier` (e.g. `Socket_0`) is what a
//!   Geometry Nodes modifier's custom properties are keyed by.
//! - **Valid `socket_type` values are tree-kind-dependent.** A `GeometryNodeTree`
//!   accepts 16 types (`NodeSocketGeometry`, `NodeSocketObject`,
//!   `NodeSocketCollection`, ... among them); a `ShaderNodeTree` accepts a different
//!   9, including `NodeSocketShader`, which no other tree kind accepts; a
//!   `CompositorNodeTree` accepts a different 7. There is no single master list.
//! - **A node's own socket is usually a *subtype*, not a valid interface
//!   `socket_type`.** `ShaderNodeMix`'s `Factor` input is `NodeSocketFloatFactor`;
//!   `GeometryNodeTransform`'s `Translation` is `NodeSocketVectorTranslation`. Both
//!   are rejected by `new_socket()` — confirmed live: passing
//!   `socket_type='NodeSocketFloatFactor'` raises `NodeTreeInterface.new_socket():
//!   ... enum "NodeSocketFloatFactor" not found in (...)`. The subtype's class does
//!   **not** inherit from its base type's class either (`NodeSocketFloatFactor` and
//!   `NodeSocketFloat` are siblings under `NodeSocketStandard`), so there is no name
//!   pattern or MRO walk that recovers the base type from the subtype's identifier.
//!   What does work, verified live: every socket (subtype or not) reports a `.type`
//!   RNA enum (`VALUE`, `VECTOR`, `RGBA`, `GEOMETRY`, ...), and creating a plain
//!   interface socket of each valid base type, wiring it through one real group
//!   node, and reading `group_node.inputs[i].type` back gives the exact
//!   `.type -> base identifier` correspondence Blender itself uses. See
//!   `base_socket_type_by_rna_type`.
//! - A geometry-nodes **modifier**'s per-input values live in
//!   `modifier.properties.inputs.<identifier>.value` on Blender 5.2+, and in
//!   `modifier[identifier]` on 5.0/5.1 (see `bpy::write_modifier_input`)
//!   — confirmed by creating a real `NODES` modifier, assigning a group with a
//!   `NodeSocketFloat` and a `NodeSocketVector` input, and reading `modifier.keys()`:
//!   they show up as `Socket_0`, `Socket_2`, ... exactly matching the interface
//!   identifiers. **Only value-type inputs appear as keys** — the `NodeSocketGeometry`
//!   input in that same test did not, which is why `geo_modifier_set` guards on the
//!   interface socket having a `default_value` at all.
//! - The group-node type for a tree kind follows `<Prefix>NodeTree -> <Prefix>NodeGroup`
//!   (`GeometryNodeTree -> GeometryNodeGroup`, etc.) for all 4 tree kinds that support
//!   groups — confirmed by checking `hasattr(bpy.types, candidate)` for each. This is
//!   still verified at call time rather than trusted blindly (see `group_node_bl_idname`).
//! - Assigning `.node_tree` on a freshly-created group node populates `.inputs` /
//!   `.outputs` to match the interface 1:1, **in interface order**, with the same
//!   `.identifier` on each side — confirmed by creating a group node, checking it has
//!   0 sockets before assignment, and 5 inputs + 1 output with matching identifiers
//!   after.
//!
//! ## Why `node_group_make` is manual
//! `bpy.ops.node.group_make` needs an active `NODE_EDITOR` area to poll. There is no
//! window, let alone that area, under `--background`; `bpy.ops.node.group_make.poll()`
//! was checked directly and returns `False` with no context override possible (there
//! is no area to override *to*). Since this server's Blender is routinely driven
//! headless, the operator path is not an option here — not "less reliable", simply
//! unavailable. So this rebuilds what the operator does: create the group tree, copy
//! the selected nodes into it (their properties and socket defaults, generically —
//! nothing here is specific to any one node type), wire `Group Input`/`Group Output`
//! for whatever crossed the boundary, and drop a group node in the original tree in
//! their place.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList, PyTuple};
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::collections::{HashMap, HashSet};

use super::{
    enabled_sockets, find_socket, node_by_name, py_to_json, resolve_tree, set_socket_value,
};

// ============================================================================
// Shared helpers
// ============================================================================

/// The group-node type Blender instantiates for a tree of this kind
/// (`GeometryNodeTree` -> `GeometryNodeGroup`, etc.).
///
/// The naming convention is consistent across all 4 group-capable tree kinds
/// (verified live), but this still confirms the derived name is a real registered
/// type before handing it to `nodes.new()` — a naming miss then fails with a clear
/// message instead of a cryptic `RuntimeError` deep inside `bpy`.
fn group_node_bl_idname(py: Python<'_>, tree_bl_idname: &str) -> PyResult<String> {
    let prefix = tree_bl_idname.strip_suffix("NodeTree").ok_or_else(|| {
        PyValueError::new_err(format!(
            "cannot derive a group-node type for tree kind `{tree_bl_idname}` \
             (expected it to end in `NodeTree`)"
        ))
    })?;
    let candidate = format!("{prefix}NodeGroup");
    let types = py.import("bpy")?.getattr("types")?;
    if !types.hasattr(candidate.as_str())? {
        return Err(PyValueError::new_err(format!(
            "no group-node type registered for `{tree_bl_idname}` (looked for `{candidate}`)"
        )));
    }
    Ok(candidate)
}

/// Every `socket_type` value `tree.interface.new_socket()` actually accepts for a
/// tree of this kind. Tree-kind dependent (see module docs) so this is computed
/// fresh per kind by trying every registered `NodeSocket` subclass on a scratch tree
/// and keeping the ones that succeed, then discarding the tree — the same
/// ask-Blender-don't-guess approach `introspect::catalog` uses for node types.
fn valid_socket_types(py: Python<'_>, tree_bl_idname: &str) -> PyResult<Vec<String>> {
    let bpy = py.import("bpy")?;
    let types = bpy.getattr("types")?;
    let builtins = py.import("builtins")?;
    let base = types.getattr("NodeSocket")?;
    let groups = bpy.getattr("data")?.getattr("node_groups")?;

    let scratch = groups.call_method1("new", ("__mcp_socket_types__", tree_bl_idname))?;
    let result = (|| -> PyResult<Vec<String>> {
        let interface = scratch.getattr("interface")?;
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
            if !is_sub || cls.is(&base) {
                continue;
            }
            let ident: String = cls.getattr("bl_rna")?.getattr("identifier")?.extract()?;
            let kw = PyDict::new(py);
            kw.set_item("in_out", "INPUT")?;
            kw.set_item("socket_type", ident.as_str())?;
            if interface
                .call_method("new_socket", ("__probe__",), Some(&kw))
                .is_ok()
            {
                out.push(ident);
            }
        }
        out.sort();
        Ok(out)
    })();
    groups.call_method1("remove", (&scratch,))?;
    result
}

/// Map a node socket's RNA `type` enum (`VALUE`, `VECTOR`, `RGBA`, `GEOMETRY`, ...) to
/// the base `socket_type` identifier `tree.interface.new_socket()` accepts for a tree
/// of this kind.
///
/// A node's actual socket is usually a *subtype* (`NodeSocketFloatFactor`,
/// `NodeSocketVectorTranslation`, ...) and subtype identifiers are not valid
/// interface `socket_type` values, confirmed live (see module docs) — and they don't
/// share a class hierarchy or name stem with their base type either, so there is no
/// way to derive the base from the subtype's own name.
///
/// So this builds the correspondence Blender itself uses: create every interface-valid
/// base type on a scratch tree, wire that tree through one real group node, and read
/// back the `.type` each resulting node socket reports.
fn base_socket_type_by_rna_type(
    py: Python<'_>,
    tree_bl_idname: &str,
) -> PyResult<HashMap<String, String>> {
    let bases = valid_socket_types(py, tree_bl_idname)?;
    let group_kind = group_node_bl_idname(py, tree_bl_idname)?;

    let bpy = py.import("bpy")?;
    let groups = bpy.getattr("data")?.getattr("node_groups")?;

    let iface_tree = groups.call_method1("new", ("__mcp_base_iface__", tree_bl_idname))?;
    let host_tree = groups.call_method1("new", ("__mcp_base_host__", tree_bl_idname))?;
    let result = (|| -> PyResult<HashMap<String, String>> {
        let interface = iface_tree.getattr("interface")?;
        for base in &bases {
            let kw = PyDict::new(py);
            kw.set_item("in_out", "INPUT")?;
            kw.set_item("socket_type", base.as_str())?;
            interface.call_method("new_socket", (base.as_str(),), Some(&kw))?;
        }
        let gnode = host_tree
            .getattr("nodes")?
            .call_method1("new", (group_kind.as_str(),))?;
        gnode.setattr("node_tree", &iface_tree)?;

        let mut map = HashMap::new();
        for s in gnode.getattr("inputs")?.try_iter()? {
            let s = s?;
            let rna_type: String = s.getattr("type")?.extract()?;
            let bl_id: String = s.getattr("bl_idname")?.extract()?;
            map.insert(rna_type, bl_id);
        }
        Ok(map)
    })();
    groups.call_method1("remove", (&host_tree,))?;
    groups.call_method1("remove", (&iface_tree,))?;
    result
}

/// A socket in `collection` (a node's `inputs`/`outputs`, or a Group Input/Output
/// node's) by interface `.identifier` rather than name — needed wherever two boundary
/// sockets could share a display name (a node's own `Value` input and `Value` output,
/// for instance), which `.identifier` never does.
fn socket_by_identifier<'py>(
    collection: &Bound<'py, PyAny>,
    identifier: &str,
) -> PyResult<Bound<'py, PyAny>> {
    for s in collection.try_iter()? {
        let s = s?;
        if s.getattr("identifier")?.extract::<String>()? == identifier {
            return Ok(s);
        }
    }
    Err(PyValueError::new_err(format!(
        "socket with identifier `{identifier}` not found"
    )))
}

// ============================================================================
// node_group_make
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeGroupMake {
    /// Tree the nodes currently live in, as node_tree_list prints it:
    /// `material:<name>`, `world`, `compositor`, `group:<name>`,
    /// `object:<object>/<modifier>`.
    pub tree: String,
    /// Real Blender names (node_graph_get) of the nodes to collapse into the new
    /// group. At least one; duplicates are ignored. Links crossing the boundary
    /// become the group's interface sockets.
    pub nodes: Vec<String>,
    /// Name for the new node group. An error if a group with this exact name
    /// already exists (rather than Blender silently renaming it to `Name.001`,
    /// which would leave node_group_add(group=name) pointing at the wrong tree).
    pub name: String,
}

/// Copy every writable, non-collection property from `src` onto `dst` (same
/// `bl_idname`). Best-effort: a handful of properties are legitimately unsettable
/// outside interactive context (`select`) or across trees (a frame `parent` that
/// isn't also being moved) — those are collected and returned rather than silently
/// dropped or treated as a hard failure of the whole call, which built the group
/// correctly regardless.
fn copy_node_props(src: &Bound<'_, PyAny>, dst: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
    const SKIP: [&str; 4] = ["type", "select", "dimensions", "internal_links"];
    let mut failed = Vec::new();
    for prop in src.getattr("bl_rna")?.getattr("properties")?.try_iter()? {
        let prop = prop?;
        let id: String = prop.getattr("identifier")?.extract()?;
        if SKIP.contains(&id.as_str()) || prop.getattr("is_readonly")?.extract::<bool>()? {
            continue;
        }
        if prop.getattr("type")?.extract::<String>()? == "COLLECTION" {
            continue;
        }
        let value = src.getattr(id.as_str())?;
        if dst.setattr(id.as_str(), &value).is_err() {
            failed.push(id);
        }
    }
    Ok(failed)
}

/// Copy every enabled socket's `default_value` from `src` to `dst`, matched by
/// position. Both nodes share a `bl_idname` and, after `copy_node_props`, the same
/// configuration (`data_type` and friends), so their enabled sockets line up 1:1.
fn copy_node_socket_values(
    src: &Bound<'_, PyAny>,
    dst: &Bound<'_, PyAny>,
) -> PyResult<Vec<String>> {
    let mut failed = Vec::new();
    for input in [true, false] {
        let s_sockets = enabled_sockets(src, input)?;
        let d_sockets = enabled_sockets(dst, input)?;
        for (s, d) in s_sockets.iter().zip(d_sockets.iter()) {
            if !s.hasattr("default_value")? {
                continue;
            }
            let Some(v) = py_to_json(&s.getattr("default_value")?) else {
                continue;
            };
            if set_socket_value(d, &v).is_err() {
                failed.push(d.getattr("name")?.extract::<String>()?);
            }
        }
    }
    Ok(failed)
}

crate::tool!(
    "node_group_make",
    "Collapse existing nodes in a tree into a reusable node group, replacing them with a group node. Built by hand rather than bpy.ops.node.group_make, which needs a NODE_EDITOR area and cannot poll under --background - this works headless.",
    NodeGroupMake,
    |py: Python<'_>, p: NodeGroupMake| {
        let tree = resolve_tree(py, &p.tree)?;
        let tree_bl_idname: String = tree.getattr("bl_idname")?.extract()?;

        let mut seen = HashSet::new();
        let unique_names: Vec<String> = p
            .nodes
            .iter()
            .filter(|n| seen.insert((*n).clone()))
            .cloned()
            .collect();
        if unique_names.is_empty() {
            return Err(PyValueError::new_err(
                "`nodes` must name at least one node to collapse",
            ));
        }

        let mut moved = Vec::new();
        for name in &unique_names {
            moved.push(node_by_name(&tree, name)?);
        }
        let move_names: HashSet<&str> = unique_names.iter().map(String::as_str).collect();

        // Where the group node lands in the original tree, and the origin the moved
        // nodes are re-centred around inside the new tree (their absolute
        // coordinates would otherwise land on top of Group Input/Output).
        let mut avg = (0.0f64, 0.0f64);
        for n in &moved {
            let loc = n.getattr("location")?;
            avg.0 += loc.get_item(0)?.extract::<f64>()?;
            avg.1 += loc.get_item(1)?.extract::<f64>()?;
        }
        avg.0 /= moved.len() as f64;
        avg.1 /= moved.len() as f64;

        // Classify every link touching a moved node before anything is mutated.
        // Internal (both endpoints moved) is recreated between the copies verbatim.
        // Only-`to`-moved becomes a group INPUT; only-`from`-moved becomes a group
        // OUTPUT (and can fan out to several external consumers). The external
        // socket objects captured here (`from_socket` for incoming, `to_socket` for
        // outgoing) belong to nodes we never touch, so they stay valid across the
        // node removal below — unlike the Link objects themselves, which do not
        // survive removing either endpoint.
        let links = tree.getattr("links")?;
        let mut internal: Vec<(String, String, String, String)> = Vec::new();
        let mut incoming: Vec<(String, String, String, Bound<'_, PyAny>)> = Vec::new();
        let mut outgoing: Vec<(String, String, String, Bound<'_, PyAny>)> = Vec::new();

        for l in links.try_iter()? {
            let l = l?;
            let from_node = l.getattr("from_node")?;
            let to_node = l.getattr("to_node")?;
            let from_name: String = from_node.getattr("name")?.extract()?;
            let to_name: String = to_node.getattr("name")?.extract()?;
            let from_in = move_names.contains(from_name.as_str());
            let to_in = move_names.contains(to_name.as_str());
            if !from_in && !to_in {
                continue;
            }
            let from_socket = l.getattr("from_socket")?;
            let to_socket = l.getattr("to_socket")?;
            if from_in && to_in {
                let from_sock_name: String = from_socket.getattr("name")?.extract()?;
                let to_sock_name: String = to_socket.getattr("name")?.extract()?;
                internal.push((from_name, from_sock_name, to_name, to_sock_name));
            } else if to_in {
                let to_sock_name: String = to_socket.getattr("name")?.extract()?;
                let rna_type: String = to_socket.getattr("type")?.extract()?;
                incoming.push((to_name, to_sock_name, rna_type, from_socket));
            } else {
                let from_sock_name: String = from_socket.getattr("name")?.extract()?;
                let rna_type: String = from_socket.getattr("type")?.extract()?;
                outgoing.push((from_name, from_sock_name, rna_type, to_socket));
            }
        }

        let base_map = if incoming.is_empty() && outgoing.is_empty() {
            HashMap::new()
        } else {
            base_socket_type_by_rna_type(py, &tree_bl_idname)?
        };

        let bpy = py.import("bpy")?;
        let groups = bpy.getattr("data")?.getattr("node_groups")?;
        let new_group = groups.call_method1("new", (p.name.as_str(), tree_bl_idname.as_str()))?;
        let actual_name: String = new_group.getattr("name")?.extract()?;
        if actual_name != p.name {
            groups.call_method1("remove", (&new_group,))?;
            return Err(PyValueError::new_err(format!(
                "a node group named `{}` already exists",
                p.name
            )));
        }

        let build = (|| -> PyResult<Value> {
            let new_nodes = new_group.getattr("nodes")?;
            let gi = new_nodes.call_method1("new", ("NodeGroupInput",))?;
            gi.setattr("location", PyList::new(py, [-400.0, 0.0])?)?;
            let go = new_nodes.call_method1("new", ("NodeGroupOutput",))?;
            go.setattr("location", PyList::new(py, [400.0, 0.0])?)?;

            let mut old_to_new: HashMap<String, Bound<'_, PyAny>> = HashMap::new();
            let mut warnings: Vec<Value> = Vec::new();
            for (name, src) in unique_names.iter().zip(moved.iter()) {
                let bl_idname: String = src.getattr("bl_idname")?.extract()?;
                let dst = new_nodes.call_method1("new", (bl_idname.as_str(),))?;
                let prop_fail = copy_node_props(src, &dst)?;
                let loc = src.getattr("location")?;
                let x: f64 = loc.get_item(0)?.extract()?;
                let y: f64 = loc.get_item(1)?.extract()?;
                dst.setattr("location", PyList::new(py, [x - avg.0, y - avg.1])?)?;
                let value_fail = copy_node_socket_values(src, &dst)?;
                if !prop_fail.is_empty() || !value_fail.is_empty() {
                    warnings.push(json!({
                        "node": name,
                        "properties": prop_fail,
                        "socket_values": value_fail,
                    }));
                }
                old_to_new.insert(name.clone(), dst);
            }

            for (from_name, from_sock, to_name, to_sock) in &internal {
                let src_node = &old_to_new[from_name];
                let dst_node = &old_to_new[to_name];
                let src_socket = find_socket(src_node, &Value::String(from_sock.clone()), false)?;
                let dst_socket = find_socket(dst_node, &Value::String(to_sock.clone()), true)?;
                new_group
                    .getattr("links")?
                    .call_method1("new", (src_socket, dst_socket))?;
            }

            let interface = new_group.getattr("interface")?;
            let mut input_items: Vec<Value> = Vec::new();
            let mut output_items: Vec<Value> = Vec::new();
            let mut final_incoming: Vec<(Bound<'_, PyAny>, String)> = Vec::new();
            let mut final_outgoing: Vec<(Bound<'_, PyAny>, String)> = Vec::new();

            for (to_name, to_sock, rna_type, ext_from) in &incoming {
                let base = base_map.get(rna_type).ok_or_else(|| {
                    PyValueError::new_err(format!(
                        "cannot expose a `{rna_type}` socket (`{to_sock}` on {to_name}) as a \
                         group input on a {tree_bl_idname}"
                    ))
                })?;
                let kw = PyDict::new(py);
                kw.set_item("in_out", "INPUT")?;
                kw.set_item("socket_type", base.as_str())?;
                let item = interface.call_method("new_socket", (to_sock.as_str(),), Some(&kw))?;
                let identifier: String = item.getattr("identifier")?.extract()?;
                input_items.push(json!({ "name": to_sock, "identifier": identifier }));

                let gi_out = socket_by_identifier(&gi.getattr("outputs")?, &identifier)?;
                let dst_node = &old_to_new[to_name];
                let target = find_socket(dst_node, &Value::String(to_sock.clone()), true)?;
                new_group
                    .getattr("links")?
                    .call_method1("new", (gi_out, target))?;

                final_incoming.push((ext_from.clone(), identifier));
            }

            let mut created_out: HashMap<(String, String), String> = HashMap::new();
            for (from_name, from_sock, rna_type, ext_to) in &outgoing {
                let key = (from_name.clone(), from_sock.clone());
                let identifier = if let Some(id) = created_out.get(&key) {
                    id.clone()
                } else {
                    let base = base_map.get(rna_type).ok_or_else(|| {
                        PyValueError::new_err(format!(
                            "cannot expose a `{rna_type}` socket (`{from_sock}` on {from_name}) \
                             as a group output on a {tree_bl_idname}"
                        ))
                    })?;
                    let kw = PyDict::new(py);
                    kw.set_item("in_out", "OUTPUT")?;
                    kw.set_item("socket_type", base.as_str())?;
                    let item =
                        interface.call_method("new_socket", (from_sock.as_str(),), Some(&kw))?;
                    let identifier: String = item.getattr("identifier")?.extract()?;
                    output_items.push(json!({ "name": from_sock, "identifier": identifier }));

                    let go_in = socket_by_identifier(&go.getattr("inputs")?, &identifier)?;
                    let src_node = &old_to_new[from_name];
                    let source = find_socket(src_node, &Value::String(from_sock.clone()), false)?;
                    new_group
                        .getattr("links")?
                        .call_method1("new", (source, go_in))?;

                    created_out.insert(key, identifier.clone());
                    identifier
                };
                final_outgoing.push((ext_to.clone(), identifier));
            }

            // Only now remove the originals: everything above still needed to read
            // their sockets by name.
            let source_nodes = tree.getattr("nodes")?;
            for n in &moved {
                source_nodes.call_method1("remove", (n,))?;
            }

            let gnode_type = group_node_bl_idname(py, &tree_bl_idname)?;
            let gnode = source_nodes.call_method1("new", (gnode_type.as_str(),))?;
            gnode.setattr("node_tree", &new_group)?;
            gnode.setattr("location", PyList::new(py, [avg.0, avg.1])?)?;

            let gnode_inputs = gnode.getattr("inputs")?;
            let gnode_outputs = gnode.getattr("outputs")?;
            let tree_links = tree.getattr("links")?;
            for (ext_from, identifier) in &final_incoming {
                let target = socket_by_identifier(&gnode_inputs, identifier)?;
                tree_links.call_method1("new", (ext_from, &target))?;
            }
            for (ext_to, identifier) in &final_outgoing {
                let source = socket_by_identifier(&gnode_outputs, identifier)?;
                tree_links.call_method1("new", (&source, ext_to))?;
            }

            Ok(json!({
                "tree": p.tree,
                "group": actual_name,
                "node": gnode.getattr("name")?.extract::<String>()?,
                "moved": unique_names,
                "inputs": input_items,
                "outputs": output_items,
                "warnings": warnings,
            }))
        })();

        if build.is_err() {
            // The group tree was created before `build` ran; do not leave a broken
            // half-built group behind on failure.
            groups.call_method1("remove", (&new_group,))?;
        }
        build
    }
);

// ============================================================================
// node_group_add
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeGroupAdd {
    /// Tree to add the group instance into, as node_tree_list prints it:
    /// `material:<name>`, `world`, `compositor`, `group:<name>`,
    /// `object:<object>/<modifier>`.
    pub tree: String,
    /// Name of an existing node group (bpy.data.node_groups) to instance. Its kind
    /// must match the target tree's — a GeometryNodeTree group cannot go into a
    /// material.
    pub group: String,
    /// [x, y] in node-editor units. Default [0, 0].
    pub location: Option<Vec<f64>>,
    /// Input socket NAME -> value, applied to the new group node's inputs. The
    /// names are the ones node_interface_add was given.
    pub inputs: Option<Map<String, Value>>,
}

crate::tool!(
    "node_group_add",
    "Instance an existing node group as a node inside a tree. The node type (ShaderNodeGroup / GeometryNodeGroup / CompositorNodeGroup / TextureNodeGroup) is resolved from the target tree's kind, not guessed.",
    NodeGroupAdd,
    |py: Python<'_>, p: NodeGroupAdd| {
        let tree = resolve_tree(py, &p.tree)?;
        let tree_bl_idname: String = tree.getattr("bl_idname")?.extract()?;

        let bpy = py.import("bpy")?;
        let group = bpy
            .getattr("data")?
            .getattr("node_groups")?
            .call_method1("get", (p.group.as_str(),))?;
        if group.is_none() {
            return Err(PyValueError::new_err(format!(
                "node group not found: {}",
                p.group
            )));
        }

        if let Some(loc) = &p.location
            && loc.len() != 2
        {
            return Err(PyValueError::new_err("location must be [x, y]"));
        }

        let node_type = group_node_bl_idname(py, &tree_bl_idname)?;
        let node = tree
            .getattr("nodes")?
            .call_method1("new", (node_type.as_str(),))?;
        node.setattr("node_tree", &group)?;

        if let Some(loc) = &p.location {
            node.setattr("location", PyList::new(py, loc.clone())?)?;
        }

        if let Some(inputs) = &p.inputs {
            for (name, value) in inputs {
                let socket = find_socket(&node, &Value::String(name.clone()), true)?;
                set_socket_value(&socket, value)?;
            }
        }

        Ok(json!({
            "tree": p.tree,
            "group": p.group,
            "node": node.getattr("name")?.extract::<String>()?,
            "type": node_type,
            "inputs": super::socket_names(&node, true)?,
            "outputs": super::socket_names(&node, false)?,
        }))
    }
);

// ============================================================================
// node_interface_add
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct NodeInterfaceAdd {
    /// Node group name (bpy.data.node_groups), as node_tree_create kind=group or
    /// node_group_make returned it.
    pub group: String,
    /// Socket name shown in the UI, and the name node_group_add's `inputs` and
    /// geo_modifier_set's `values` address it by.
    pub name: String,
    /// INPUT or OUTPUT, case-insensitive.
    pub in_out: String,
    /// Socket bl_idname. Valid values depend on the group's tree kind; on
    /// Blender 5.2 (verified live) a ShaderNodeTree group takes NodeSocketFloat,
    /// Int, Bool, Vector, Color, Menu, Shader, Bundle, Closure; a GeometryNodeTree
    /// group those minus Shader plus Rotation, Matrix, String, Geometry, Object,
    /// Collection, Image, Material, Font, Sound; a CompositorNodeTree group Float,
    /// Int, Bool, Vector, IntVector3D, Color, Rotation, Matrix, String, Menu,
    /// Object, Font; a TextureNodeTree group only Float, Vector, Color. An invalid
    /// value is rejected with the exact list this group accepts, read live.
    pub socket_type: String,
    /// Initial value, shaped like the socket (number, bool, string, [x, y, z],
    /// [r, g, b, a]). Only value-type sockets have one; geometry/object/collection/
    /// ... sockets do not. This is the node-tree default, NOT what an existing
    /// Geometry Nodes modifier will show — see the tool description.
    pub default: Option<Value>,
    /// Lower slider bound for a numeric socket. Silently ignored on a socket type
    /// with no min_value.
    pub min: Option<f64>,
    /// Upper slider bound for a numeric socket. Silently ignored on a socket type
    /// with no max_value.
    pub max: Option<f64>,
}

crate::tool!(
    "node_interface_add",
    "Add an input or output socket to a node group's interface (its Group \
     Input/Output sockets). Returns the socket's identifier (e.g. Socket_3), the \
     address node_group_add's `inputs` and geo_modifier_set resolve names to. \
     Verified live on Blender 5.2: a socket added to a group that a Geometry Nodes \
     modifier ALREADY uses arrives in that modifier as 0.0, not at the `default` \
     given here — follow up with geo_modifier_set on that object+modifier, or the \
     modifier silently evaluates with zero.",
    NodeInterfaceAdd,
    |py: Python<'_>, p: NodeInterfaceAdd| {
        let bpy = py.import("bpy")?;
        let group = bpy
            .getattr("data")?
            .getattr("node_groups")?
            .call_method1("get", (p.group.as_str(),))?;
        if group.is_none() {
            return Err(PyValueError::new_err(format!(
                "node group not found: {}",
                p.group
            )));
        }
        let tree_bl_idname: String = group.getattr("bl_idname")?.extract()?;

        let in_out = p.in_out.to_uppercase();
        if in_out != "INPUT" && in_out != "OUTPUT" {
            return Err(PyValueError::new_err(format!(
                "in_out must be INPUT or OUTPUT, got `{}`",
                p.in_out
            )));
        }

        let interface = group.getattr("interface")?;
        let kw = PyDict::new(py);
        kw.set_item("in_out", in_out.as_str())?;
        kw.set_item("socket_type", p.socket_type.as_str())?;
        let item = match interface.call_method("new_socket", (p.name.as_str(),), Some(&kw)) {
            Ok(item) => item,
            Err(_) => {
                let valid = valid_socket_types(py, &tree_bl_idname)?;
                return Err(PyValueError::new_err(format!(
                    "`{}` is not a valid socket_type for a {tree_bl_idname} group. Valid: {}",
                    p.socket_type,
                    valid.join(", ")
                )));
            }
        };

        if let Some(default) = &p.default {
            set_socket_value(&item, default)?;
        }
        if let Some(min) = p.min
            && item.hasattr("min_value")?
        {
            item.setattr("min_value", min)?;
        }
        if let Some(max) = p.max
            && item.hasattr("max_value")?
        {
            item.setattr("max_value", max)?;
        }

        Ok(json!({
            "group": p.group,
            "name": p.name,
            "identifier": item.getattr("identifier")?.extract::<String>()?,
            "in_out": in_out,
            "socket_type": p.socket_type,
        }))
    }
);

// ============================================================================
// geo_modifier_set
// ============================================================================

/// Object + Geometry Nodes modifier, with the guards a caller working from a stale
/// object/modifier name needs.
fn object_and_modifier<'py>(
    py: Python<'py>,
    object: &str,
    modifier: &str,
) -> PyResult<(Bound<'py, PyAny>, Bound<'py, PyAny>)> {
    let bpy = py.import("bpy")?;
    let obj = bpy
        .getattr("data")?
        .getattr("objects")?
        .call_method1("get", (object,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {object}")));
    }
    let m = obj.getattr("modifiers")?.call_method1("get", (modifier,))?;
    if m.is_none() {
        let mut have = Vec::new();
        for mm in obj.getattr("modifiers")?.try_iter()? {
            have.push(mm?.getattr("name")?.extract::<String>()?);
        }
        return Err(PyValueError::new_err(format!(
            "modifier not found on {object}: {modifier}. Modifiers on this object: {}",
            have.join(", ")
        )));
    }
    let mod_type: String = m.getattr("type")?.extract()?;
    if mod_type != "NODES" {
        return Err(PyValueError::new_err(format!(
            "modifier `{modifier}` on {object} is a {mod_type} modifier, not Geometry Nodes"
        )));
    }
    Ok((obj, m))
}

/// The INPUT sockets of a node group's interface, as (name, identifier, item).
/// Panels (`item_type == "PANEL"`, a group-interface divider with no socket of its
/// own) and OUTPUT sockets are excluded — this is specifically the set a Geometry
/// Nodes modifier accepts values for.
fn interface_inputs<'py>(
    group: &Bound<'py, PyAny>,
) -> PyResult<Vec<(String, String, Bound<'py, PyAny>)>> {
    let mut out = Vec::new();
    for item in group
        .getattr("interface")?
        .getattr("items_tree")?
        .try_iter()?
    {
        let item = item?;
        if item.getattr("item_type")?.extract::<String>()? != "SOCKET" {
            continue;
        }
        if item.getattr("in_out")?.extract::<String>()? != "INPUT" {
            continue;
        }
        let name: String = item.getattr("name")?.extract()?;
        let identifier: String = item.getattr("identifier")?.extract()?;
        out.push((name, identifier, item));
    }
    Ok(out)
}

/// Write a JSON value into `modifier[identifier]`, the Blender-native address of a
/// Geometry Nodes modifier's input.
///
/// The value's expected shape is read from the interface socket's own
/// `default_value` (its Python type is authoritative — float/int/bool are visually
/// identical in Blender's UI, and only the socket itself, not the modifier's current
/// custom-property value, always exists in the right type before a value has ever
/// been set). A socket with no `default_value` at all (geometry/object/collection/...)
/// is not settable this way — confirmed live: those sockets do not even appear as
/// modifier custom-property keys.
fn set_modifier_input(
    modifier: &Bound<'_, PyAny>,
    identifier: &str,
    iface_item: &Bound<'_, PyAny>,
    value: &Value,
) -> PyResult<()> {
    let py = modifier.py();
    if !iface_item.hasattr("default_value")? {
        let name: String = iface_item.getattr("name")?.extract()?;
        let socket_type: String = iface_item.getattr("socket_type")?.extract()?;
        return Err(PyValueError::new_err(format!(
            "input `{name}` is a {socket_type} socket and has no plain value to set \
             (only scalar/vector/color/string inputs are settable this way)"
        )));
    }
    let shape = iface_item.getattr("default_value")?;

    if shape.extract::<bool>().is_ok() {
        let b = value.as_bool().ok_or_else(|| {
            PyValueError::new_err(format!("input `{identifier}` takes a boolean, got {value}"))
        })?;
        crate::tools::bpy::write_modifier_input(modifier, identifier, b)?;
    } else if shape.extract::<i64>().is_ok() {
        let i = value
            .as_i64()
            .or_else(|| value.as_f64().map(|f| f as i64))
            .ok_or_else(|| {
                PyValueError::new_err(format!(
                    "input `{identifier}` takes an integer, got {value}"
                ))
            })?;
        crate::tools::bpy::write_modifier_input(modifier, identifier, i)?;
    } else if shape.extract::<f64>().is_ok() {
        let f = value.as_f64().ok_or_else(|| {
            PyValueError::new_err(format!("input `{identifier}` takes a number, got {value}"))
        })?;
        crate::tools::bpy::write_modifier_input(modifier, identifier, f)?;
    } else if shape.extract::<String>().is_ok() {
        let text = value.as_str().ok_or_else(|| {
            PyValueError::new_err(format!("input `{identifier}` takes a string, got {value}"))
        })?;
        crate::tools::bpy::write_modifier_input(modifier, identifier, text)?;
    } else if let Ok(seq) = shape.extract::<Vec<f64>>() {
        let arr = value.as_array().ok_or_else(|| {
            PyValueError::new_err(format!(
                "input `{identifier}` takes {} numbers, got {value}",
                seq.len()
            ))
        })?;
        let mut nums: Vec<f64> = arr.iter().filter_map(Value::as_f64).collect();
        if nums.len() != arr.len() {
            return Err(PyValueError::new_err(format!(
                "input `{identifier}` takes numbers only"
            )));
        }
        // A colour input takes RGBA; accepting RGB and keeping alpha opaque matches
        // what a caller passing three numbers means (same rule set_socket_value uses).
        if seq.len() == 4 && nums.len() == 3 {
            nums.push(1.0);
        }
        if nums.len() != seq.len() {
            return Err(PyValueError::new_err(format!(
                "input `{identifier}` takes {} numbers, got {}",
                seq.len(),
                nums.len()
            )));
        }
        crate::tools::bpy::write_modifier_input(modifier, identifier, PyTuple::new(py, nums)?)?;
    } else {
        return Err(PyValueError::new_err(format!(
            "input `{identifier}` has a value type this tool cannot set from JSON"
        )));
    }
    Ok(())
}

#[derive(Deserialize, schemars::JsonSchema)]
pub struct GeoModifierSet {
    /// Name of the object carrying the modifier.
    pub object: String,
    /// Modifier name on that object; must be a Geometry Nodes (NODES) modifier.
    /// node_tree_create kind=geometry_modifier returns it in the address after the
    /// slash.
    pub modifier: String,
    /// Input socket NAME -> value, shaped like the socket (number, bool, string,
    /// [x, y, z], [r, g, b, a]). Names are resolved to the group's Socket_N
    /// identifier automatically; the caller never needs to know it. Geometry,
    /// object and collection sockets have no plain value and are rejected by name.
    pub values: Map<String, Value>,
}

crate::tool!(
    "geo_modifier_set",
    "Set input values of a Geometry Nodes modifier on an object, by socket NAME. \
     Modifier inputs are stored on the modifier itself, not on the node tree - \
     this is the one tool in the node layer that addresses an object+modifier \
     rather than a tree, and the required follow-up after node_interface_add adds \
     a socket to a group a modifier is already using.",
    GeoModifierSet,
    |py: Python<'_>, p: GeoModifierSet| {
        let (_, modifier) = object_and_modifier(py, &p.object, &p.modifier)?;
        let group = modifier.getattr("node_group")?;
        if group.is_none() {
            return Err(PyValueError::new_err(format!(
                "modifier `{}` on {} has no node group assigned - create one with \
                 node_group_make or by assigning modifier.node_group first",
                p.modifier, p.object
            )));
        }

        let inputs = interface_inputs(&group)?;

        let mut resolved = Vec::new();
        for (name, value) in &p.values {
            let Some((_, identifier, item)) = inputs.iter().find(|(n, _, _)| n == name) else {
                let have: Vec<&str> = inputs.iter().map(|(n, _, _)| n.as_str()).collect();
                return Err(PyValueError::new_err(format!(
                    "modifier `{}` has no input named `{name}`. Its inputs are: {}",
                    p.modifier,
                    have.join(", ")
                )));
            };
            set_modifier_input(&modifier, identifier, item, value)?;
            resolved.push(json!({ "name": name, "identifier": identifier }));
        }

        Ok(json!({
            "object": p.object,
            "modifier": p.modifier,
            "set": resolved,
        }))
    }
);
