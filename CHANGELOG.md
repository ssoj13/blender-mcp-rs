# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); the project is pre-release, so
everything currently sits under **Unreleased** against the `0.1.0` baseline.

## [Unreleased]

### Formats, matrices and tool annotations

- **`file_import` / `file_export` cover stl and ply.** `wm.stl_import`, `wm.ply_import` and
  the matching exporters exist on 5.2 but were unreachable through the tools. Verified by a
  real round trip on 5.2.2.
- **`attr_set` can write `FLOAT4X4` at last.** Blender wants a nested 4x4 and answered a flat
  sequence with `expected a sequence of float, not float`, so every matrix write failed. Both
  shapes are accepted now — nested, or 16 numbers folded into rows. The docs record what the
  live check showed: Blender stores the groups **column-major**, so a transform's translation
  is the last group, not the last element of each.
- **Every tool now carries MCP annotations** (`readOnlyHint` / `destructiveHint` /
  `openWorldHint`), derived from what its name says it does: 48 read-only, 45 destructive,
  39 open-world, 0 contradictory. Clients use these to decide what to confirm with their
  user, so the rules err toward "this changes something"; two guard tests pin the behaviour.
- **`cam_create` no longer drops `focus_distance` / `aperture` when DOF is off.** `cam_update`
  always applied them, `cam_create` only with `dof_enabled` — the same two values behaved
  differently depending on which tool you reached for, and `cam_create` silently discarded
  what the caller passed. They are ordinary camera properties: written either way now, so a
  camera can be set up first and have its depth of field switched on later.

### Documentation caught up with the code

- `mcp-setup-rs` is **vendored**, not an SSH git dependency — `AGENTS.md` still described the
  old arrangement and warned against changing it. The section now gives the real reason
  (crates.io rejects git dependencies; that repo is private) and the part that bites:
  `filesystem-mcp-rs` and `shotgrid-mcp-rs` carry the same copy at the same commit, so a fix
  made here instead of upstream forks three servers.
- Tool counts said 238; it is **241** (233 scene + 5 control + 3 knowledge), counted from a
  live `tools/list`.
- Node-type counts said 506, a Blender 5.0 number. On 5.2 there are **570** registered
  (shader 101, geometry 267, compositor 91, texture 37, function 56, generic 18), **555** of
  them creatable — counted live, not recalled.
- The user guide has a section on `kdb`: what the three tools do, what is in the corpus, how
  to add your own notes through `$BLENDER_MCP_KDB`, and how to regenerate it.

### Every tool and parameter description rewritten against a live Blender 5.2

`tools/list` is the only thing a model reads before choosing a tool, and it was written for
Blender 4.x and never re-checked. **828 parameters, 0 now without a description** (277 had
none); the median tool description went from 43 to 230 characters. Every enum-valued string
was re-read from a live 5.2.2 instead of copied forward — which is exactly how the stale ones
below surfaced.

- **Descriptions cross-reference the sibling tool for the adjacent job**, so a model that lands
  on the wrong one is told where to go instead of forcing the tool it found: `mat_assign` →
  `mat_slot_add` → `mat_assign_faces`, `mesh_subdivide` → `mesh_edit`, `phys_bake` vs
  `sim_bake`, `drv_add` vs `drv_create`, `render_set` vs `scene_render`, `anim_set_frame` vs
  `scene_set`, `obj_update` vs `obj_apply_transform`.
- **`anim_add_fcurve_cycles` reported a failure as success.** On 5.2 a CYCLES F-modifier added
  after a NOISE one is *created* with `is_valid == False` and does nothing at all — where
  5.0/5.1 returned `None` and the tool errored. The modifier was in the stack, every `setattr`
  on it worked and the JSON looked perfect. `new_fmodifier` now checks `is_valid`, removes the
  dead modifier again and errors, naming the rule (a CYCLES modifier is only ever valid first
  in the stack).
- **`mesh_unwrap` passed its angle in degrees to an RNA property measured in radians**, so any
  value above 90 was silently clamped. Now converted.
- Enum values that no longer exist in 5.2 and were being advertised: `mat_settings` exposed
  `shadow_method`; `phys_add_force_field` offered `MAGNETIC` and `SMOKE_FLOW` (now mapped onto
  the real `MAGNET` / `FLUID`); `vse_add_effect` offered `TRANSFORM` and treated `OVER_DROP` as
  a two-input effect; `util_boolean` offered a solver 5.2 removed.
- Enum values that exist but were undocumented: `obj_list` described 4 object types out of 16;
  `mesh_unwrap` omitted `MINIMUM_STRETCH`; the `attr_*` tools omitted `FLOAT4`; the `cam_*`
  tools omitted the `CUSTOM` camera type.
- **`scene_screenshot` mode `render` does work headless.** The docs claimed it could not;
  `viewport` and `window` now fail with a message that points at the mode that does.

### `kdb` — an embedded Blender knowledge base (3 tools)

`tools/list` says what exists; nothing said how Blender *works*, so an agent guessed enum values
and socket names. That guess is the failure this project keeps paying for.

- **`kdb/`: 2444 Markdown documents (6.9 MB)** built by `python/kdb_build.py` — the API
  reference dumped from a **live Blender 5.2.2** (RNA types with their real enum values,
  operator signatures, actual node socket names), release notes 5.0–5.3, and the 5.2 user
  manual flattened out of reStructuredText.
- **11 hand-written area guides** (`kdb/guides/`): getting-started, objects-and-scene,
  modelling, uv-and-attributes, shading-and-materials, geometry-nodes, animation, rigging,
  physics-and-simulation, rendering-and-cameras, pipeline-io. Each names only tools that exist,
  cross-links the sibling tool for the job, and carries traps verified on 5.2.2 rather than
  recalled. Guides outrank generated reference in search, so "how do I" lands on one. The
  builder never touches them.
- **`src/kdb.rs`: BM25 search over the embedded corpus**, plus an optional runtime overlay from
  `$BLENDER_MCP_KDB`. Lexical, in-process — no model, no FFI, no network. The three tools
  (`kdb_search` / `kdb_read` / `kdb_guide`) answer **with Blender closed**, so an agent can read
  how to do something before launching anything.
- **The server now sends MCP instructions**: that the scene tools need a running Blender, that
  kdb comes before guessing an API name, and the habits that pay here (look, batch, read the
  error).

Writing the guides is also what turned up most of the description defects fixed above.

### Blender 5.2 LTS is the target

Verified against a live 5.2.2. Both API moves keep 5.0/5.1 working by picking the address this
Blender actually has.

- **Geometry Nodes modifier inputs moved.** `modifier["Socket_N"]` now raises
  `TypeError: this type doesn't support IDProperties`; the value lives at
  `modifier.properties.inputs.Socket_N.value`. New `bpy::write_modifier_input` /
  `read_modifier_input` hit `geo_modifier_set` and `obj_shade` (its "Smooth by Angle" modifier).
- **`GreasePencilLayer.channel_color` is read-only in 5.2.** `gp_add_layer` falls back to
  `tint_color` (+ `tint_factor`) and reports which attribute took the colour as
  `color_applied_as`.
- **The server no longer dies when another server owns the pipe.** `spawn_listener` failing (on
  Windows: "Access is denied") took the whole MCP server down with it, and reached the agent
  only as a closed connection. It now claims the next free tag and returns it, so
  `blender_launch` hands a launched Blender the tag we really own and `blender_status` reports
  it. This also unblocks the stdio smoke test next to a running server.
- Two test expectations were testing Blender rather than this project, and were corrected: the
  registered node-type count is now counted live instead of hardcoded to 5.0's 506, and the
  CYCLES F-modifier case is now asserted through `is_valid` (see above).

**Tool count is now 241** — 233 scene + 5 process-control + 3 kdb (`kdb_search`,
`kdb_read`, `kdb_guide`). The "238" still quoted in README/AGENTS/docs predates the kdb
tools and needs the same correction.


### `install --force` — take over a hand-written MCP entry

- **`install` refuses when the key already holds an entry we do not own** (what `status` calls
  `conflict`), because silently replacing someone's own configuration is worse than failing
  loudly. `--force` is the explicit "this key is mine now". The previous content goes to a
  timestamped backup first, and the apply report carries `overwrote an unmanaged entry
  (--force)` so a takeover is never silent. Honoured by both the JSON and TOML ownership guards;
  library callers use `HostSpec::with_force(true)`. `uninstall` is deliberately not covered —
  refusing to delete an entry we never created is a different risk from refusing to overwrite
  one.
- Binary resolution is now testable without touching `CARGO_HOME`
  (`resolve_installed_exe_in(exe, install_dir)`); the old test mutated that process-global and
  raced the rest of the suite.
- Vendored copy pinned at upstream `27b95a0`.

### Vendored `mcp-setup-rs` resync — dangling MCP entries are now detected, and no longer created

- **`install` registers the installed binary, not the running one.** `HostSpec::from_current_exe`
  used to record whatever executable happened to be running, so a server registering itself from
  `target/release` baked a build-tree path into every agent config — a path that dies on the next
  `cargo clean`, branch switch or repo move. Resolution is now `$CARGO_HOME/bin` → `PATH` → the
  running binary (the last only as a fallback for a checkout with nothing installed).
- **`status` reports `broken` for an entry whose command is gone.** `StatusReport` gained
  `stored_command` and `command_resolves`; an entry we own that names a binary which cannot be
  launched is no longer indistinguishable from a healthy one. This was found live: a
  `~/.claude.json` had pointed at a `target/release` path for weeks after the repo moved, and
  `status` called it `installed` the whole time because the key and the install id were both
  present and nothing ever checked the path. `install` already rewrites a differing entry, so it
  is also the repair.
- **`normalize_path` strips Windows' `\?\` verbatim prefix.** Live configs carried
  `\?\C:\Users\…\*.exe`. Legal for `CreateProcess`, but not universally understood by the Node /
  Python launchers that actually read these files.
- **`cli::run` returns `Outcome::{AllOk, SomeFailed}` instead of raising on failed targets.** A
  broken or unreachable client is a result the table already reported; raising it made every host
  print an error chain — and, under `RUST_BACKTRACE=1`, a full Rust backtrace — on top of a
  perfectly readable table. Hosts now turn the outcome into an exit code. *Breaking for
  embedders:* `run` returns `CliResult<Outcome>`, not `CliResult<()>`.
- PATH lookup walks `PATH` directly (honouring `PATHEXT` on Windows) rather than shelling out to
  `where` / `which`: no subprocess, nothing locale-dependent to parse.
- Pinned at upstream `aebe269`; see the vendored `VENDOR.md`. Upstream is now rustfmt-clean, so
  this tree was formatted to match — future resyncs no longer land as formatting diffs.

### Added
- **Process control (5 tools).** `blender_status`, `blender_launch`, `blender_quit`,
  `blender_kill`, `blender_restart` let the agent manage Blender itself. They run in the
  server process (not over the pipe) via the internal `blender-control` crate.
- **`BLENDER_PATH` support.** Blender is located via `--blender`, then `$BLENDER_PATH` /
  `$BLENDER`, then standard locations; `install` bakes the discovered path into the agent
  config.
- **`addon/autostart.py`.** Baked into the server binary and handed to a launched Blender
  with `--python`, so it connects without the add-on being enabled in Preferences.
  Headless Blender is supported via a blocking main-thread drain loop (timers do not tick
  in `--background`).
- **`crates/blender-control`.** Internal workspace crate for pure process control
  (`std` + `sysinfo`, no `bpy`/MCP/FFI).
- **mdbook documentation** under `docs/` (Introduction, Whitepaper, User & Developer
  guides, recipes, worked example); `CHANGELOG`, and dual `LICENSE` files.

### Changed
- **Dual-licensed** BSD-3-Clause OR commercial (was MIT).
- Documentation refreshed to the current architecture; tool count is **238** (233 scene +
  5 control). Stale planning docs describing the pre-rebuild architecture were removed.

### Fixed
- Launched Blender gets null stdio — an inherited server stdout is the MCP channel, and an
  undrained inherited stderr hangs Blender mid-startup.
- Process status trusts the launched handle, not the process table, so a user's own open
  Blender is never conflated with the one the server launched.

## [0.1.0] — 2026-07-14 (baseline)

The architectural rebuild, merged to `main`.

### Changed
- **One registry, two processes.** Every tool is declared once in `src/tools/`; the JSON
  Schema is generated from the params struct and the same registry compiles into both the
  server binary and the `.pyd`. Schema and handler can no longer drift apart.
- **Transport: ZeroMQ → named pipe** (`interprocess`). Pure Rust — no FFI, no TCP port,
  no firewall prompt.
- **Python monolith removed.** The 6751-line add-on became a ~380-line bridge + UI; all
  tool logic is Rust. An unknown tool name is an error, not a silent legacy fallback.
- **Unified node layer.** One family of tools covers all 506 Blender node types by reading
  sockets/enums from Blender's RNA instead of hand-maintained tables.
- **Self-registration** via `mcp-setup-rs`: `install` / `uninstall` / `status` across 18
  agents.

### Fixed
- Blender-5 API breakages surfaced by the port: Grease Pencil v3 rename, VSE
  `sequences`→`strips`, `Action.fcurves` removal, `musgrave` node removal,
  `scene_screenshot` window mode, and dynamic-enum introspection (Cycles was being
  rejected on every render).
