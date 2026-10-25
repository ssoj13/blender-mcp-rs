# Vendored: `mcp-setup-rs`

This directory is a **verbatim copy** of the private crate
[`ssoj13/mcp-setup-rs`](https://github.com/ssoj13/mcp-setup-rs), pinned at:

    commit  76fb174cb08af44c26f2928d274ad15345000c21   ("fix(bootstrap): run cargo metadata before cargo update -p mcp-setup-rs")
    origin  ssh://git@github.com/ssoj13/mcp-setup-rs.git   (branch main, PRIVATE)

## Why vendored instead of a dependency

`blender-mcp-rs` embeds a `pyo3` cdylib and is distributed like the rest of this MCP-server family
(`shotgrid-mcp-rs`, `filesystem-mcp-rs`), which are published on crates.io — and **crates.io
rejects git dependencies**: every dependency of a published crate must itself be a published
crate (the `git` spec is stripped at publish time and the version is resolved from the registry).
`mcp-setup-rs` is deliberately private, so it cannot be that dependency. Code that must ship
inside a published crate has to live inside it. Hence: copy, not link.

The private repo remains the **upstream and the source of truth**, shared with the other MCP
servers in this family. `filesystem-mcp-rs`'s and `shotgrid-mcp-rs`'s `src/mcp_setup/` are pinned
at this exact commit too — verified in sync when this copy was made.

## Rules

- **Do not fix bugs here.** Fix them upstream in the private repo, then re-vendor. A local edit is
  silently lost on the next resync and diverges every other consumer from this copy.
- The only edits applied to the upstream sources are mechanical and reproducible by the resync
  procedure below:
  1. `crate::` → `crate::mcp_setup::` in every file (the files address what was their own crate
     root and is now this module);
  2. the crate root `src/mcp_setup.rs` becomes this directory's `mod.rs`, with the upstream `cli`
     cargo feature gate dropped (this host always builds the CLI) and a vendoring note added;
  3. upstream `tests/roundtrip.rs` becomes `tests.rs`, a `#[cfg(test)]` submodule, with
     `use mcp_setup::` → `use crate::mcp_setup::`.
- Upstream dependencies are mirrored in this repo's `Cargo.toml` (`toml_edit`, `jsonc-parser`,
  `dirs`; `serde`, `serde_json`, `clap` were already present, `thiserror` was added). Bumping them
  upstream means bumping them here.
- This module is declared directly in `src/main.rs` (`mod mcp_setup;`) — `blender-mcp-rs` has no
  separate lib crate that needs it, unlike `shotgrid-mcp-rs`.

## Resync procedure

```sh
git clone ssh://git@github.com/ssoj13/mcp-setup-rs.git /tmp/mcp-setup-rs
cd /tmp/mcp-setup-rs && git rev-parse HEAD          # record this in the pin above

D=src/mcp_setup
rm -rf "$D" && mkdir -p "$D"
cp -r /tmp/mcp-setup-rs/src/. "$D/"
mv "$D/mcp_setup.rs" "$D/mod.rs"
cp /tmp/mcp-setup-rs/tests/roundtrip.rs "$D/tests.rs"
find "$D" -name '*.rs' -exec sed -i 's/\bcrate::/crate::mcp_setup::/g' {} +
sed -i 's/^use mcp_setup::/use crate::mcp_setup::/' "$D/tests.rs"
```

Then re-apply the three edits listed above to `mod.rs` (drop `#[cfg(feature = "cli")]`, add
`#[cfg(test)] mod tests;`, keep the vendoring note), restore this `VENDOR.md`, reconcile
`Cargo.toml` with the upstream manifest, and run `cargo test mcp_setup`.
