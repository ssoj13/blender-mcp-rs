//! # protocol.rs - the two messages that cross the pipe
//!
//! One request, one response, correlated by `id`. The wire format is
//! newline-delimited JSON over the local named pipe (see `ipc.rs`).
//!
//! ```text
//! main.rs (MCP server)  --Command-->  lib.rs (.pyd in Blender)
//!                                          |
//!                                          | tools::execute_tool -> bpy
//!                                          v
//! main.rs               <--Response--  lib.rs
//! ```
//!
//! Both sides are built from this crate, so the types cannot drift apart.
//!
//! `params` is a JSON *string*, not a nested object: the server forwards whatever
//! the MCP client sent without re-encoding it, and the Blender side parses it once,
//! into the tool's typed params struct.
//!
//! Command: `{"id": 1, "method": "scene_get_info", "params": "{}"}`
//! Response: `{"id": 1, "result": "{...}", "error": null}`

use serde::{Deserialize, Serialize};

/// Command sent from MCP server to Blender addon.
///
/// - `id`: unique request ID, used to match responses
/// - `method`: tool name (e.g. "scene_info", "create_primitive")
/// - `params`: JSON-encoded arguments for the tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: u64,
    pub method: String,
    pub params: String,
}

/// Response from Blender addon back to MCP server.
///
/// - `id`: matches the Command.id
/// - `result`: JSON-encoded result on success (None if error)
/// - `error`: error message on failure (None if success)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: u64,
    pub result: Option<String>,
    pub error: Option<String>,
}
