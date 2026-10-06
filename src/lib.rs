// The catalog is one enormous `json!` literal; the default macro recursion limit
// cannot expand it. Drops back to the default once `mcp_catalog` is deleted.
#![recursion_limit = "512"]

//! # blender_mcp — the Python extension module that runs inside Blender
//!
//! Built with PyO3 as a `.pyd` and imported by the addon. It provides two things:
//! the link to the MCP server process, and the tool implementations themselves.
//!
//! ## Architecture
//! ```text
//! main.rs — MCP server process (rmcp, stdio)
//!     |  tools/list, tools/call  <- from crate::tools (the registry)
//!     |
//!     |  named pipe: blender-mcp-<tag>.sock   (server listens)
//!     v
//! lib.rs — this module, inside Blender's Python (pipe client)
//!     |  reader thread queues Commands (never touches bpy)
//!     v
//! addon/mcp_server/__init__.py
//!     |  bpy.app.timers -> poll() -> execute_tool() -> respond()
//!     v
//! crate::tools — the same registry, now executing against bpy on the main thread
//! ```
//!
//! The registry is compiled into **both** the server binary and this `.pyd`, so a
//! tool is declared once: the server gets its schema, Blender gets its handler.
//!
//! ## Why Rust owns the transport
//! Blender's bundled Python ships no IPC library, and installing one into it is
//! awkward. Shipping the `.pyd` is enough — nothing to pip-install.
//!
//! ## Usage from the addon
//! ```python
//! import blender_mcp
//! mcp = blender_mcp.BlenderMcp("blender")
//! mcp.start()                      # connects in the background, retries forever
//!
//! # in a bpy.app.timers callback (main thread — bpy is not thread-safe):
//! cmd = mcp.poll()                 # non-blocking; None when idle
//! if cmd:
//!     out = blender_mcp.execute_tool(cmd["method"], cmd["params"])
//!     mcp.respond(cmd["id"], out, None)
//!
//! mcp.stop()
//! ```
//!
//! ## Build
//! ```text
//! maturin build --release   # -> target/wheels/blender_mcp-*.whl
//! ```

use pyo3::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Mutex};

pub mod ipc;
mod protocol;
pub mod tools;

// The catalog is consumed by the server binary, and by the parity test that guards
// the migration into `tools`. It lives in the lib so both can reach it; it goes away
// once every tool is registry-declared.
pub mod mcp_catalog;

pub use protocol::{Command, Response};

/// Blender-side end of the named-pipe link to the MCP server.
///
/// A background thread owns the socket and only ever *queues* commands; `poll()`
/// hands them to the addon's timer callback, which runs them on Blender's main
/// thread. `bpy` is not thread-safe, so nothing here may touch it.
#[pyclass]
pub struct BlenderMcp {
    tag: String,
    /// `#[pyclass]` requires `Sync`, and `mpsc::Receiver` is `Send` but not `Sync`.
    rx: Mutex<Option<Receiver<Command>>>,
    writer: Arc<Mutex<Option<ipc::Writer>>>,
    running: Arc<AtomicBool>,
    connected: Arc<AtomicBool>,
}

#[pymethods]
impl BlenderMcp {
    #[new]
    #[pyo3(signature = (tag="default"))]
    fn new(tag: &str) -> PyResult<Self> {
        Ok(Self {
            tag: tag.to_string(),
            rx: Mutex::new(None),
            writer: Arc::new(Mutex::new(None)),
            running: Arc::new(AtomicBool::new(false)),
            connected: Arc::new(AtomicBool::new(false)),
        })
    }

    #[getter]
    fn tag(&self) -> &str {
        &self.tag
    }

    /// Start the reader thread. Returns immediately: it connects in the background
    /// and retries forever, so Blender and the server may start in either order and
    /// Blender survives a server restart.
    fn start(&mut self) -> PyResult<()> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(pyo3::exceptions::PyRuntimeError::new_err("Already running"));
        }

        let (tx, rx) = channel::<Command>();
        *self
            .rx
            .lock()
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("rx lock poisoned"))? = Some(rx);

        let tag = self.tag.clone();
        let writer = self.writer.clone();
        let running = self.running.clone();
        let connected = self.connected.clone();

        std::thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                let Ok((reader, w)) = ipc::connect(&tag) else {
                    // Server not up yet. Retry; start order must not matter.
                    std::thread::sleep(std::time::Duration::from_millis(200));
                    continue;
                };

                if let Ok(mut guard) = writer.lock() {
                    *guard = Some(w);
                }
                connected.store(true, Ordering::SeqCst);
                eprintln!("[BlenderMcp] Connected to MCP server (tag={tag})");

                ipc::read_commands(reader, |cmd| {
                    // Queue only — execution happens on Blender's main thread.
                    tx.send(cmd).is_ok() && running.load(Ordering::SeqCst)
                });

                connected.store(false, Ordering::SeqCst);
                if let Ok(mut guard) = writer.lock() {
                    *guard = None;
                }
                eprintln!("[BlenderMcp] Server disconnected (tag={tag}), reconnecting");
            }
        });
        Ok(())
    }

    fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        self.connected.store(false, Ordering::SeqCst);
        if let Ok(mut guard) = self.writer.lock() {
            *guard = None;
        }
        eprintln!("[BlenderMcp] Disconnected (tag={})", self.tag);
    }

    /// Take one queued command, or `None`. Non-blocking: called from a Blender timer.
    fn poll(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        let guard = self
            .rx
            .lock()
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("rx lock poisoned"))?;
        let Some(rx) = guard.as_ref() else {
            return Ok(None);
        };
        match rx.try_recv() {
            Ok(cmd) => {
                let dict = pyo3::types::PyDict::new(py);
                dict.set_item("id", cmd.id)?;
                dict.set_item("method", cmd.method)?;
                dict.set_item("params", cmd.params)?;
                Ok(Some(dict.into()))
            }
            Err(_) => Ok(None),
        }
    }

    /// Send a response back to the server.
    #[pyo3(signature = (id, result=None, error=None))]
    fn respond(&self, id: u64, result: Option<String>, error: Option<String>) -> PyResult<()> {
        let mut guard = self
            .writer
            .lock()
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("writer lock poisoned"))?;
        let w = guard
            .as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Not connected"))?;

        ipc::send_line(w, &Response { id, result, error })
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Whether the pipe is actually up right now (the reader thread may still be
    /// retrying), so the addon UI can tell "started" from "connected".
    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }
}

/// Execute an MCP tool by name against `bpy`, dispatching through the Rust registry.
/// Every tool lives in the registry; an unknown name is an error, not a fallback.
#[pyfunction]
#[pyo3(signature = (method, params_json))]
fn execute_tool(py: Python<'_>, method: &str, params_json: &str) -> PyResult<String> {
    tools::execute_tool(py, method, params_json)
}

/// Python module
#[pymodule]
fn blender_mcp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<BlenderMcp>()?;
    m.add_function(wrap_pyfunction!(execute_tool, m)?)?;
    Ok(())
}
