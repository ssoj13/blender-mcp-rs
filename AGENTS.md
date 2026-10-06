# blender-mcp-rs

MCP server for Blender. **241 tools, all in Rust** (233 scene tools + 5 process
control + 3 knowledge-base tools). Python remains only where it cannot be removed: Blender loads add-ons
exclusively as Python modules.

## The core idea (the code makes no sense without it)

A tool is declared **once** — in `src/tools/`. The JSON schema the model sees is
generated from the parameter type (`schemars`); registration is automatic
(`inventory`).

The tool catalog is needed by the **server process** (`main.rs`, no `bpy` there), while
execution happens **inside Blender** (`.pyd`). But both are Rust and both are built from
one crate (`crate-type = ["cdylib", "rlib"]`), so **one registry serves
both**: the server takes name+description+schema, Blender takes the handler. Schema and
code cannot drift apart — this is not discipline, it is construction.

```
              src/tools — registry
             /                  \
   blender-mcp-rs.exe        blender_mcp.pyd
   tools/list, tools/call    executes against bpy
        \______ named pipe ______/
```

## Layout

| Path | What |
|---|---|
| `src/tools/` | **Tool registry.** Read `src/tools/AGENTS.md` — the rules are there |
| `src/control.rs` | 5 server-side Blender process control tools (launch/quit/kill/status/restart) |
| `crates/blender-control` | Internal workspace crate: pure process control (find/launch/kill), no `bpy`/MCP |
| `src/ipc.rs` | Transport: named pipe, shared by both processes |
| `src/protocol.rs` | `Command` / `Response` |
| `src/main.rs` | Server: `rmcp` on stdio + pipe listener |
| `src/rmcp_bridge.rs` | `ServerHandler`; builds `tools/list` from the registry |
| `src/mcp_catalog.rs` | **Resources and prompts only.** NO tools here |
| `src/kdb.rs` | **Knowledge base**: BM25 search over `kdb/`, and the `kdb_*` tools |
| `kdb/` | The corpus itself: generated API reference + release notes + manual, and hand-written `kdb/guides/` |
| `src/lib.rs` | `.pyd`: pipe client + `execute_tool` |
| `addon/mcp_server/` | Thin add-on: bridge + UI. Zero `bpy` calls |

## Commands

Everything goes through `bootstrap.py` (stdlib-only, cross-platform):

```bash
python bootstrap.py b     # build: server binary + .pyd (one crate, two artifacts)
python bootstrap.py d     # deploy add-on + .pyd into Blender's add-ons directory
python bootstrap.py t     # tests: registry + stdio-smoke + tools in a live Blender
python bootstrap.py c     # cargo fmt --check + clippy -D warnings
python bootstrap.py i     # register the server in agents (mcp-setup)
python python/kdb_build.py --blender <exe>   # regenerate kdb/ (not kdb/guides/)
```

Blender: `C:\Programs\Blender5` (5.2 LTS, Python 3.13). Overridden by `$BLENDER` or
`--blender PATH`.

## Two kinds of borrowed code

`mcp-setup-rs` is **vendored**, not a dependency: a verbatim copy of the private
`ssoj13/mcp-setup-rs` lives in `src/mcp_setup/`, pinned to one upstream commit — see
`src/mcp_setup/VENDOR.md`. The reason is crates.io: every dependency of a published
crate must itself be published, and that repo is deliberately private, so a `git =`
dependency cannot ship. Do not "fix" it back into a git or `path` dependency. Fixes
belong upstream, in the private repo; then re-vendor from there. `filesystem-mcp-rs`
and `shotgrid-mcp-rs` carry the same copy at the same commit, so a fix made here and
not upstream silently forks three servers.

It gives the binary the `install` / `uninstall` / `status` subcommands — the server
registers itself in 18 agents (Claude Code, Codex, Cursor, VS Code, Zed…), each in
its own dialect, and can clean up after itself exactly what it added. On `install`,
`BLENDER_PATH` (the discovered Blender path) is baked into the agent config so the
control tools find Blender without `--blender`.

`blender-control` is the other kind: an **internal workspace member**
(`crates/blender-control`), written here rather than borrowed. It is built and
versioned with this repo, sharing `target`/`Cargo.lock`, and is a separate crate only
so that pure process control does not drag in `pyo3`/MCP and stays reusable.

## Blender control (the LLM brings Blender up itself)

Five tools (`src/control.rs`) execute **in the server process**, not in Blender:
`blender_launch` (arg `background` — headless), `blender_status`, `blender_quit`,
`blender_kill`, `blender_restart`. Blender path: `--blender PATH` → `$BLENDER_PATH` /
`$BLENDER` → standard locations. Details and pitfalls (isolation from a foreign Blender,
null-stdio, headless-loop) — in `src/AGENTS.md` and `addon/AGENTS.md`.

## Rules paid for in blood

- **Do not guess the Blender API.** There is a live Blender — ask it:
  `blender.exe --background --python probe.py`. Agents confidently invent enum
  names; four of them were caught exactly this way.
- **Target version is Blender 5.2 LTS.** Every release moves the API under the tools:
  Grease Pencil rewritten in 4.3, VSE renamed in 4.4, `Action.fcurves` removed in
  5.0, the `musgrave` node removed in 4.1, and in 5.2 a Geometry Nodes modifier's
  inputs moved from `mod["Socket_N"]` to `mod.properties.inputs.Socket_N.value`
  while a Grease Pencil layer's `channel_color` became read-only. The list is in
  `src/tools/AGENTS.md`.
- **`scene_screenshot`'s viewport/window modes are verified only in the GUI.** They read
  the screen, so they need an OpenGL context and cannot run in `--background` — which is
  exactly why a breakage there once survived 26 headless runs. `mode: "render"` is the
  exception: it renders through the camera and works headless too (verified on 5.2).
- **`bpy` is not thread-safe.** Tools execute on Blender's main thread
  (a `bpy.app.timers` callback). The pipe reader thread only enqueues commands.
- **Call failure = `Err`.** Never `Ok({"error": ...})` — that is a success whose text
  mentions an error, and the model reads right past it. There is a guard test.
- **No panics** in tool code: `unwrap`/`expect`/`let _ =` on fallible calls are
  forbidden. A panic inside Blender takes down more than just the tool.
- **No FFI.** ZeroMQ was thrown out for exactly this reason; the transport is pure Rust.

## Status

The rebuild is complete and merged into `main`. Architecture and numbers — `docs/` (mdbook: whitepaper),
history — in git.
