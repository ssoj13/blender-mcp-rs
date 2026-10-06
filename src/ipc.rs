//! Local IPC between the MCP server process and Blender.
//!
//! ## Why not ZMQ
//! The old transport was ZeroMQ over two TCP loopback ports (PUB :8765 for
//! commands, PULL :8766 for responses). That meant: an FFI dependency on a C
//! library built from source (`zeromq-src`), two listening TCP ports — which
//! Windows Firewall prompts about and which collide when a second server starts —
//! and a one-way socket pair standing in for what is really a request/response
//! conversation.
//!
//! This is a single duplex local socket: a named pipe on Windows, a Unix domain
//! socket elsewhere. Pure Rust, no FFI, no TCP port, no firewall prompt. Two
//! Blender instances can coexist by using different tags.
//!
//! ## Framing
//! Newline-delimited JSON. `Command` goes server -> Blender, `Response` comes back,
//! correlated by `id`. Trivially inspectable, and a stray byte cannot desynchronise
//! more than one message.
//!
//! ## Who listens
//! The **server** listens; **Blender** connects. Blender retries forever, so the two
//! may start in either order and Blender survives a server restart.
//!
//! ## Threading
//! `bpy` is not thread-safe, so on the Blender side a reader thread only *queues*
//! commands; the addon's `bpy.app.timers` callback drains that queue and executes
//! them on Blender's main thread.

use std::io::{BufRead, BufReader, Write};

use interprocess::local_socket::traits::Stream as _;
use interprocess::local_socket::{GenericNamespaced, Name, Stream, ToNsName};

use crate::protocol::Command;

/// One pipe per Blender instance, so several can be driven independently.
pub fn pipe_name(tag: &str) -> String {
    format!("blender-mcp-{tag}.sock")
}

pub fn ns_name(tag: &str) -> std::io::Result<Name<'static>> {
    pipe_name(tag).to_ns_name::<GenericNamespaced>()
}

/// The write half, shared between whoever wants to send on the connection.
pub type Writer = Box<dyn Write + Send>;

/// Send one newline-delimited JSON message.
pub fn send_line<T: serde::Serialize>(w: &mut Writer, msg: &T) -> std::io::Result<()> {
    let mut line = serde_json::to_string(msg)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    line.push('\n');
    w.write_all(line.as_bytes())?;
    w.flush()
}

/// Connect to the server's pipe. `Err` simply means the server is not up yet —
/// callers are expected to retry rather than treat this as fatal.
pub fn connect(tag: &str) -> std::io::Result<(BufReader<impl std::io::Read>, Writer)> {
    let stream = Stream::connect(ns_name(tag)?)?;
    let (recv, send) = stream.split();
    Ok((BufReader::new(recv), Box::new(send) as Writer))
}

/// Read `Command`s off the wire until the peer goes away, handing each to `sink`.
///
/// A malformed line is logged and skipped rather than killing the connection: one
/// bad message must not take Blender's link down with it.
pub fn read_commands<R: BufRead>(mut reader: R, mut sink: impl FnMut(Command) -> bool) {
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => return, // peer closed — caller reconnects
            Ok(_) => {}
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        match serde_json::from_str::<Command>(trimmed) {
            Ok(cmd) => {
                if !sink(cmd) {
                    return;
                }
            }
            Err(e) => eprintln!("[blender-mcp] skipping malformed command: {e}"),
        }
    }
}
