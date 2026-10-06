//! RMCP stdio server, bridging MCP requests to Blender over the local named pipe.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use blender_mcp::{Command, Response};
use rmcp::ErrorData as McpError;
use rmcp::ServiceExt;
use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, ContentBlock, GetPromptRequestParams, GetPromptResult,
    Implementation, ListPromptsResult, ListResourcesResult, ListToolsResult,
    PaginatedRequestParams, Prompt, ReadResourceRequestParams, ReadResourceResult, Resource,
    ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::{RequestContext, RoleServer};
use serde_json::Value;
use tracing::{debug, error, info, warn};

use blender_mcp::{ipc, mcp_catalog};

pub type PendingMap = HashMap<u64, tokio::sync::oneshot::Sender<Response>>;

/// A poisoned lock means another thread panicked while holding it. Report it as a
/// tool error rather than propagating the panic across the whole server.
fn lock<T>(m: &Mutex<T>) -> Result<std::sync::MutexGuard<'_, T>, String> {
    m.lock().map_err(|_| "internal lock poisoned".to_string())
}

/// The tool list the model sees, built straight from the registry that also
/// executes the tools inside Blender. One declaration, both processes — there is
/// no second list to keep in sync.
fn tools_cached() -> &'static [Tool] {
    static TOOLS: OnceLock<Vec<Tool>> = OnceLock::new();
    TOOLS
        .get_or_init(|| {
            // Scene tools (executed inside Blender) plus the server-side control tools
            // (launch/quit/kill/status/restart). Both appear in one `tools/list`.
            let scene = blender_mcp::tools::all().map(|t| {
                let mut schema = (t.schema)();
                sanitize_schema(&mut schema);
                Tool::new(t.name, t.description, rmcp::model::object(schema))
                    .annotate(annotations_for(t.name))
            });
            let control = crate::control::control_tools()
                .into_iter()
                .map(|(n, d, mut s)| {
                    sanitize_schema(&mut s);
                    Tool::new(n.to_string(), d.to_string(), rmcp::model::object(s))
                        .annotate(annotations_for(n))
                });
            // Knowledge tools answer without Blender running at all, which is the
            // point: read how something is done before launching anything.
            let kdb = crate::kdb::kdb_tools().into_iter().map(|(n, d, mut s)| {
                sanitize_schema(&mut s);
                Tool::new(n.to_string(), d, rmcp::model::object(s)).annotate(annotations_for(n))
            });
            scene.chain(control).chain(kdb).collect()
        })
        .as_slice()
}

/// MCP tool annotations, derived from what the tool's name says it does.
///
/// These are hints a client uses to decide what to confirm with its user, so the
/// cost of being wrong is asymmetric: calling a destructive tool "read-only" hides
/// a deletion behind no prompt, while the reverse only adds a prompt. Every rule
/// below therefore errs toward "this changes something".
///
/// - **read-only**: reports state and writes nothing — `*_list`, `*_get*`,
///   `kdb_*`, `node_schema`, `blender_status`, the analysis tools. `scene_screenshot`
///   counts: it mutates render settings but restores them, and produces an image.
/// - **destructive**: removes or overwrites something a caller cannot get back by
///   calling again — delete/remove/clear/purge/kill, `mod_apply`, `file_save`,
///   `util_execute_python` (arbitrary code: assume the worst).
/// - everything else is additive.
///
/// `open_world` is false for the scene and knowledge tools (one Blender, one
/// corpus) and true for the ones that reach the filesystem or spawn a process.
fn annotations_for(name: &str) -> rmcp::model::ToolAnnotations {
    use rmcp::model::ToolAnnotations;

    const READ_ONLY_EXACT: &[&str] = &[
        "blender_status",
        "node_schema",
        "node_types",
        "node_graph_get",
        "node_tree_list",
        "scene_analyze",
        "scene_detect_issues",
        "scene_get_info",
        "scene_get_stats",
        "scene_list_templates",
        "scene_list_unused",
        "scene_screenshot",
        "mat_preview",
        "world_get",
        "rig_get_pose",
        "obj_get_selection",
        "mesh_get_data",
        "mesh_get_uvs",
    ];
    const DESTRUCTIVE_EXACT: &[&str] = &[
        "mod_apply",
        "util_apply_booleans",
        "obj_apply_transform",
        "file_save",
        "file_export",
        "util_execute_python",
        "util_undo",
        "util_redo",
        "library_relocate",
        "library_reload",
        "sculpt_remesh",
        "phys_bake",
        "sim_bake",
        "mesh_set_data",
    ];
    const OPEN_WORLD_PREFIXES: &[&str] = &[
        "file_",
        "img_",
        "sound_",
        "movieclip_",
        "vol_",
        "library_",
        "asset_",
        "vse_render",
        "render_queue",
        "scene_render",
        "preset_",
        "blender_",
    ];

    let read_only = READ_ONLY_EXACT.contains(&name)
        || name.ends_with("_list")
        || name.ends_with("_get")
        || name.contains("_get_")
        || name.starts_with("kdb_");
    let destructive = !read_only
        && (DESTRUCTIVE_EXACT.contains(&name)
            || name.contains("delete")
            || name.contains("remove")
            || name.contains("clear")
            || name.contains("purge")
            || name.contains("kill")
            || name.contains("quit")
            || name.contains("restart")
            || name.contains("unmark"));
    let open_world = OPEN_WORLD_PREFIXES.iter().any(|p| name.starts_with(p));

    ToolAnnotations::new()
        .read_only(read_only)
        .destructive(destructive)
        .open_world(open_world)
}

/// Make a schemars-generated schema safe for strict MCP clients.
///
/// An unconstrained `serde_json::Value` param serialises to the JSON-Schema **boolean**
/// `true` (a valid schema meaning "anything"). Some clients — Claude Code among them —
/// model the values of a `properties` map as *object* schemas and reject a boolean
/// there, which fails the entire `tools/list` (observed: tool `hair_add_modifier`,
/// param `properties`). Replace any boolean sitting **inside a `properties` map** with
/// `{}` — the empty schema, semantically identical to `true` but an object the client
/// accepts. Booleans elsewhere (`items`, `additionalProperties`, `$defs`) are left
/// alone: those positions are accepted, and non-schema booleans (`uniqueItems`, …) must
/// not be touched. Recursive, so nested `$defs`/sub-objects are covered.
fn sanitize_schema(node: &mut Value) {
    if let Value::Object(map) = node {
        if let Some(Value::Object(props)) = map.get_mut("properties") {
            for v in props.values_mut() {
                if v.is_boolean() {
                    *v = Value::Object(serde_json::Map::new());
                }
            }
        }
        for v in map.values_mut() {
            sanitize_schema(v);
        }
    } else if let Value::Array(arr) = node {
        arr.iter_mut().for_each(sanitize_schema);
    }
}

fn resources_cached() -> Vec<Resource> {
    let v = mcp_catalog::get_resources();
    let arr = v
        .as_array()
        .expect("get_resources must return a JSON array");
    arr.iter()
        .filter_map(|item| serde_json::from_value(item.clone()).ok())
        .collect()
}

fn prompts_cached() -> Vec<Prompt> {
    let v = mcp_catalog::get_prompts();
    let arr = v.as_array().expect("get_prompts must return a JSON array");
    arr.iter()
        .filter_map(|item| serde_json::from_value(item.clone()).ok())
        .collect()
}

/// MCP server state. Holds the [`ControlCtx`], which carries the pipe writer, the
/// in-flight request map, the id sequence, and the process handle — everything both a
/// pipe tool call and a server-side control tool need.
///
/// The writer inside is `None` whenever Blender is not attached — a scene tool call
/// then fails fast with a clear message instead of hanging until the timeout.
#[derive(Clone)]
pub struct BlenderMcpServer {
    pub control: crate::control::ControlCtx,
}

/// What the model reads before it has called anything.
///
/// `tools/list` says what exists; none of it says that the scene tools need a running
/// Blender, that guessing an enum name is the most common way to fail here, or that
/// the server carries a searchable corpus which answers those guesses. That is what
/// belongs here — the things no tool schema can tell you.
fn instructions() -> String {
    let lines = [
        "Drives Blender 5.2 LTS: scene tools executed inside Blender, 5 process-control",
        "tools, and 3 knowledge tools that work with Blender closed.",
        "",
        "**The scene tools need Blender running and connected.** `blender_status` says",
        "whether it is; `blender_launch` starts it (`background: true` for headless,",
        "which has no viewport and therefore no `scene_screenshot`). If a scene tool",
        "answers \"Blender is not connected\", launch it and retry.",
        "",
        "**Do not guess Blender's API — this server ships the answers.** `kdb_search`",
        "searches the built-in knowledge base: hand-written guides, the API reference",
        "dumped from this exact Blender build (enum values, operator signatures, real",
        "node socket names), the release notes, and the user manual. `kdb_read` opens a",
        "hit; `kdb_guide` gives an area briefing. Use them before inventing an enum",
        "value or a socket name, and whenever a call fails in a way that smells like a",
        "version difference — these names move between Blender releases.",
        "",
        "**Habits that pay here:**",
        "- Look instead of assuming: `scene_screenshot` returns the viewport as an image;",
        "  `scene_get_info`, `obj_list` and `obj_get` report real state.",
        "- Most tools take a batch (`names`, `objects`, `updates`, …). One call that",
        "  touches twenty objects beats twenty calls.",
        "- Node work goes through `node_schema` (a node type's real sockets),",
        "  `node_graph_build` (a whole graph in one call) and `node_graph_get`.",
        "- A failed call is an error, never a success carrying an error message. Read it:",
        "  a tool answers an unknown name by listing the real ones.",
        "- `util_execute_python` runs arbitrary `bpy` code when no tool fits.",
    ];
    format!(
        "{}

({} scene tools, {} knowledge-base documents.)",
        lines.join(
            "
"
        ),
        blender_mcp::tools::all().count(),
        crate::kdb::index().len(),
    )
}

impl ServerHandler for BlenderMcpServer {
    fn get_info(&self) -> ServerInfo {
        let capabilities = ServerCapabilities::builder()
            .enable_tools()
            .enable_resources()
            .enable_prompts()
            .build();
        ServerInfo::new(capabilities)
            .with_server_info(Implementation::new(
                "blender-mcp",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(instructions())
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        tools_cached()
            .iter()
            .find(|t| t.name.as_ref() == name)
            .cloned()
    }

    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, McpError>> + Send + '_ {
        let tools: Vec<Tool> = tools_cached().to_vec();
        std::future::ready(Ok(ListToolsResult::with_all_items(tools)))
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, McpError>> + Send + '_ {
        let control = self.control.clone();
        let name = request.name.to_string();
        let args: Value = request
            .arguments
            .map(Value::Object)
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));

        async move {
            // Control tools run in this process, not inside Blender. Dispatch them
            // first; `None` means the name is a scene tool and goes to the pipe.
            // Knowledge tools need neither Blender nor the pipe.
            if let Some(outcome) = crate::kdb::dispatch(&name, &args) {
                return match outcome {
                    Ok(text) => Ok(CallToolResult::success(vec![ContentBlock::text(text)])),
                    Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(e)])),
                };
            }

            if let Some(outcome) = crate::control::dispatch(&control, &name, args.clone()).await {
                return match outcome {
                    Ok(text) => Ok(CallToolResult::success(vec![ContentBlock::text(text)])),
                    Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(e)])),
                };
            }

            match call_blender(
                &name,
                args,
                &control.writer,
                &control.pending,
                &control.next_id,
            )
            .await
            {
                // A tool that produced an image (screenshot, render) marks its result
                // with `__image__`, so it reaches the model as an image block rather
                // than a wall of base64 text.
                Ok(result) => {
                    let image = serde_json::from_str::<Value>(&result)
                        .ok()
                        .filter(|v| v.get("__image__").and_then(Value::as_bool).unwrap_or(false));
                    if let Some(img) = image {
                        let data = img.get("data").and_then(Value::as_str).unwrap_or("");
                        let mime = img
                            .get("mimeType")
                            .and_then(Value::as_str)
                            .unwrap_or("image/png");
                        return Ok(CallToolResult::success(vec![ContentBlock::image(
                            data, mime,
                        )]));
                    }
                    Ok(CallToolResult::success(vec![ContentBlock::text(result)]))
                }
                Err(e) => Ok(CallToolResult::error(vec![ContentBlock::text(e)])),
            }
        }
    }

    fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListResourcesResult, McpError>> + Send + '_ {
        let resources = resources_cached();
        std::future::ready(Ok(ListResourcesResult::with_all_items(resources)))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ReadResourceResult, McpError>> + Send + '_ {
        let uri = request.uri.clone();
        std::future::ready(match mcp_catalog::read_resource(&uri) {
            Some(v) => serde_json::from_value(v)
                .map_err(|e| McpError::internal_error(format!("read_resource JSON: {e}"), None)),
            None => Err(McpError::resource_not_found(
                format!("Resource not found: {uri}"),
                None,
            )),
        })
    }

    fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListPromptsResult, McpError>> + Send + '_ {
        let prompts = prompts_cached();
        std::future::ready(Ok(ListPromptsResult::with_all_items(prompts)))
    }

    fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<GetPromptResult, McpError>> + Send + '_ {
        let args: Value = request
            .arguments
            .map(Value::Object)
            .unwrap_or_else(|| Value::Object(serde_json::Map::new()));
        let name = request.name.clone();
        std::future::ready(match mcp_catalog::get_prompt(&name, &args) {
            Some(v) => serde_json::from_value(v)
                .map_err(|e| McpError::internal_error(format!("get_prompt JSON: {e}"), None)),
            None => Err(McpError::resource_not_found(
                format!("Prompt not found: {name}"),
                None,
            )),
        })
    }
}

/// How long a tool may take inside Blender before we give up on it. A bake or a
/// render can legitimately be slow, but an unbounded wait would wedge the server.
const CALL_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

pub async fn call_blender(
    method: &str,
    params: serde_json::Value,
    writer: &Arc<Mutex<Option<ipc::Writer>>>,
    pending: &Arc<Mutex<PendingMap>>,
    next_id: &Arc<AtomicU64>,
) -> Result<String, String> {
    let id = next_id.fetch_add(1, Ordering::SeqCst);
    info!("call_blender: method={method} id={id}");

    let cmd = Command {
        id,
        method: method.to_string(),
        params: serde_json::to_string(&params).map_err(|e| e.to_string())?,
    };

    let (tx, rx) = tokio::sync::oneshot::channel();
    lock(pending)?.insert(id, tx);

    // Send, and drop the pending entry if we never got the command out — otherwise
    // the id would leak and the caller would wait out the full timeout for nothing.
    let sent = {
        let mut guard = lock(writer)?;
        match guard.as_mut() {
            Some(w) => ipc::send_line(w, &cmd).map_err(|e| e.to_string()),
            None => Err("Blender is not connected (start the MCP client in Blender)".to_string()),
        }
    };
    if let Err(e) = sent {
        error!("send failed for id={id}: {e}");
        lock(pending)?.remove(&id);
        return Err(e);
    }
    debug!("command id={id} sent, awaiting response");

    let resp = match tokio::time::timeout(CALL_TIMEOUT, rx).await {
        Ok(Ok(resp)) => resp,
        Ok(Err(_)) => {
            error!("response channel closed for id={id}");
            lock(pending)?.remove(&id);
            return Err("Response channel closed".to_string());
        }
        Err(_) => {
            error!("timeout waiting for Blender, id={id}");
            lock(pending)?.remove(&id);
            return Err("Timeout waiting for Blender (is the addon running?)".to_string());
        }
    };

    info!(
        "Got response id={} has_result={} has_error={}",
        resp.id,
        resp.result.is_some(),
        resp.error.is_some()
    );

    if let Some(err) = resp.error {
        warn!("Blender returned error: {}", err);
        Err(err)
    } else {
        let result = resp.result.ok_or_else(|| "Empty response".to_string())?;
        debug!("Blender result: {}", result);
        Ok(result)
    }
}

/// Listen on the pipe and serve whatever Blender attaches, forever.
///
/// Runs on a plain thread: `interprocess`'s sync API blocks, and the work here is
/// one line of JSON at a time — a tokio task would buy nothing. Blender may come
/// and go (addon restart, Blender restart); each new connection simply replaces the
/// writer, and any request still pending from the old one falls to its timeout.
/// How many tags are tried before giving up when the preferred one is taken.
const TAG_FALLBACK_LIMIT: u32 = 32;

pub fn spawn_listener(
    tag: String,
    writer: Arc<Mutex<Option<ipc::Writer>>>,
    pending: Arc<Mutex<PendingMap>>,
) -> anyhow::Result<String> {
    use interprocess::local_socket::ListenerOptions;

    // A second server instance on the same machine — another agent session, or the
    // test suite next to a running one — finds the pipe already owned and, on
    // Windows, is told "Access is denied" rather than "address in use". Failing here
    // would take the whole MCP server down with it, which the agent sees only as a
    // connection that closed. So the preferred tag is a preference: if it is taken,
    // claim the next free one and say so. A Blender started by `blender_launch` is
    // handed this tag, so it still lands on us; only a hand-started Blender, which
    // always connects to `blender`, stays with whoever owns that pipe.
    let mut last_err = None;
    for attempt in 0..TAG_FALLBACK_LIMIT {
        let candidate = if attempt == 0 {
            tag.clone()
        } else {
            format!("{tag}-{}", attempt + 1)
        };
        let listener = match ListenerOptions::new()
            .name(ipc::ns_name(&candidate)?)
            .create_sync()
        {
            Ok(l) => l,
            Err(e) => {
                warn!("cannot listen on {}: {e}", ipc::pipe_name(&candidate));
                last_err = Some((candidate, e));
                continue;
            }
        };
        if attempt > 0 {
            warn!(
                "pipe `{}` is taken by another server; using `{}` instead",
                ipc::pipe_name(&tag),
                ipc::pipe_name(&candidate)
            );
        }
        return serve(listener, candidate, writer, pending);
    }

    let (candidate, e) = last_err.expect("the loop runs at least once");
    Err(anyhow::anyhow!(
        "cannot listen on {} (nor on {TAG_FALLBACK_LIMIT} fallbacks, last tried {}): {e}",
        ipc::pipe_name(&tag),
        ipc::pipe_name(&candidate)
    ))
}

/// Accept connections on an already-bound listener, forever, on its own thread.
fn serve(
    listener: interprocess::local_socket::Listener,
    tag: String,
    writer: Arc<Mutex<Option<ipc::Writer>>>,
    pending: Arc<Mutex<PendingMap>>,
) -> anyhow::Result<String> {
    use interprocess::local_socket::traits::{Listener as _, Stream as _};

    info!("listening on {}", ipc::pipe_name(&tag));
    let claimed = tag.clone();

    std::thread::spawn(move || {
        loop {
            let conn = match listener.accept() {
                Ok(c) => c,
                Err(e) => {
                    error!("accept failed: {e}");
                    continue;
                }
            };
            info!("Blender connected");

            let (recv, send) = conn.split();
            match writer.lock() {
                Ok(mut g) => *g = Some(Box::new(send) as ipc::Writer),
                Err(_) => {
                    error!("writer lock poisoned; dropping connection");
                    continue;
                }
            }

            // Blocks until Blender goes away, then we loop back to accept again.
            read_responses(std::io::BufReader::new(recv), &pending);

            if let Ok(mut g) = writer.lock() {
                *g = None;
            }
            warn!("Blender disconnected");
        }
    });

    Ok(claimed)
}

/// Resolve pending calls as responses arrive. A response with no matching id means
/// the caller already timed out and gave up — worth a warning, not an error.
fn read_responses(reader: impl std::io::BufRead, pending: &Arc<Mutex<PendingMap>>) {
    for line in reader.lines() {
        let Ok(line) = line else { return };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<Response>(line) {
            Ok(resp) => {
                let id = resp.id;
                let waiter = match pending.lock() {
                    Ok(mut p) => p.remove(&id),
                    Err(_) => {
                        error!("pending lock poisoned");
                        return;
                    }
                };
                match waiter {
                    Some(tx) => {
                        if tx.send(resp).is_err() {
                            debug!("caller for id={id} is gone");
                        }
                    }
                    None => warn!("no pending request for id={id} (timed out?)"),
                }
            }
            Err(e) => error!("malformed response from Blender: {e}"),
        }
    }
}

pub async fn run_stdio(server: BlenderMcpServer) -> anyhow::Result<()> {
    let t = rmcp::transport::stdio();
    let service = server
        .serve(t)
        .await
        .map_err(|e| anyhow::anyhow!("rmcp serve: {e:?}"))?;
    service
        .waiting()
        .await
        .map_err(|e| anyhow::anyhow!("rmcp waiting: {e:?}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Recursively report the path of any boolean sitting as a value inside a
    /// `properties` map — the shape strict MCP clients reject.
    fn boolean_props(node: &Value, path: &str, out: &mut Vec<String>) {
        match node {
            Value::Object(map) => {
                if let Some(Value::Object(props)) = map.get("properties") {
                    for (k, v) in props {
                        if v.is_boolean() {
                            out.push(format!("{path}.properties.{k}"));
                        }
                    }
                }
                for (k, v) in map {
                    boolean_props(v, &format!("{path}.{k}"), out);
                }
            }
            Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    boolean_props(v, &format!("{path}[{i}]"), out);
                }
            }
            _ => {}
        }
    }

    #[test]
    fn sanitize_replaces_boolean_property_with_empty_object() {
        let mut s = serde_json::json!({
            "type": "object",
            "properties": { "free": true, "kept": {"type": "string"} },
            "additionalProperties": true
        });
        sanitize_schema(&mut s);
        assert_eq!(s["properties"]["free"], serde_json::json!({}));
        assert_eq!(
            s["properties"]["kept"],
            serde_json::json!({"type": "string"})
        );
        // additionalProperties booleans are a different, accepted position — untouched.
        assert_eq!(s["additionalProperties"], serde_json::json!(true));
    }

    #[test]
    fn no_tool_schema_has_a_boolean_property() {
        // The real registry, sanitized exactly as it is served, must have zero booleans
        // inside any `properties` map (Claude Code fails the whole tools/list otherwise).
        let mut offenders = Vec::new();
        for t in tools_cached() {
            let schema = Value::Object((*t.input_schema).clone());
            boolean_props(&schema, t.name.as_ref(), &mut offenders);
        }
        assert!(
            offenders.is_empty(),
            "boolean schemas inside a properties map (rejected by strict clients): {offenders:?}"
        );
    }
}

#[cfg(test)]
mod annotation_tests {
    use super::*;

    /// The hints are what a client decides to prompt about, so the two that matter
    /// are pinned by name rather than left to the rules to reproduce by accident.
    #[test]
    fn reads_are_read_only_and_deletes_are_destructive() {
        for name in ["obj_list", "scene_get_info", "kdb_search", "blender_status"] {
            let a = annotations_for(name);
            assert_eq!(a.read_only_hint, Some(true), "{name} should be read-only");
            assert_eq!(
                a.destructive_hint,
                Some(false),
                "{name} is read-only, so it cannot be destructive"
            );
        }
        for name in [
            "obj_delete",
            "scene_purge_unused",
            "mod_apply",
            "blender_kill",
        ] {
            let a = annotations_for(name);
            assert_eq!(a.read_only_hint, Some(false), "{name} changes things");
            assert_eq!(
                a.destructive_hint,
                Some(true),
                "{name} should be destructive"
            );
        }
    }

    /// Every tool the model sees carries hints; an unannotated one silently falls
    /// back to the spec's defaults (not read-only, destructive), which is wrong for
    /// most of this catalog.
    #[test]
    fn every_tool_is_annotated() {
        for t in tools_cached() {
            let a = t
                .annotations
                .as_ref()
                .unwrap_or_else(|| panic!("{} has no annotations", t.name));
            assert!(a.read_only_hint.is_some(), "{} lacks readOnlyHint", t.name);
        }
    }
}
