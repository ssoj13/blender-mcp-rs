//! Library overrides and linked-library management.
//!
//! [`asset_import`](super::util::asset_import) can already link a datablock
//! (`bpy.ops.wm.link`) instead of appending it, but a linked datablock is
//! read-only until it is overridden — this module is the other half of that
//! workflow: turning a linked rig/asset into something that can be posed or
//! animated locally, and managing the libraries that linked data comes from.
//!
//! ## API facts verified live (Blender 5.0, `--background`)
//!
//! These were checked against a running Blender rather than assumed — the
//! override API is fiddly and version-specific, and every one of these traps
//! a guess would have missed:
//!
//! - **`bpy.ops.object.make_override_library()` needs an active+selected
//!   object and mutates `bpy.context` selection to do it** (poll is `False`
//!   until the linked object is both selected and active). The data-level
//!   equivalent, **`id.override_hierarchy_create(scene, view_layer)`, needs no
//!   selection at all** and runs headless just as well — it is what
//!   [`library_override_create`] uses, precisely so this tool has no side
//!   effect on whatever the caller had selected.
//! - **`override_hierarchy_create` overrides the whole linked hierarchy**, not
//!   just the requested object: overriding a linked armature also overrides
//!   its linked mesh children and re-parents the new mesh override under the
//!   new armature override — verified by linking an armature + child mesh and
//!   overriding just the armature.
//! - **`do_fully_editable` controls whether the result is a "system
//!   override"** (`is_system_override == True`) or a fully local one. Blender
//!   itself defaults this to off. A system override can still be *written to*
//!   from a script — `obj.location.x = 5` "worked" either way in testing —
//!   but it is the data Blender's override-resync machinery considers safe to
//!   regenerate later, i.e. the opposite of what "posed locally, and it
//!   sticks" needs. [`library_override_create`] defaults it on.
//! - **A scripted property write is not auto-tracked as an override
//!   operation.** Setting `override_obj.location.x = 42.0` from Python and
//!   then calling `override_library.reset()` (or the
//!   `reset_override_library` operator) is a silent no-op — the location
//!   stays 42.0. Blender only diffs the live value against the reference
//!   inside `override_library.operations_update()`; without that call first,
//!   `reset()` finds nothing to revert. [`library_override_reset`] always
//!   calls `operations_update()` immediately before `reset()`.
//! - **`IDOverrideLibrary.destroy()` is not a drop-in for "clear override".**
//!   It frees the override datablock immediately — any further attribute
//!   access on that Python object raises `ReferenceError`. Went with
//!   `bpy.ops.object.clear_override_library()` instead (which, like
//!   `make_override_library`, needs the override selected and active).
//! - **Clearing does NOT cascade through a hierarchy the way creating does.**
//!   `override_hierarchy_create` overrides a whole rig in one call, but
//!   `clear_override_library()` on the root only clears the root — the
//!   child overrides it created are left behind, still `override_library is
//!   not None`, still parented under nothing (the root reverted to the plain
//!   linked object). [`library_override_clear`] therefore only ever clears
//!   the one object it is asked for; a caller that overrode a hierarchy
//!   needs to clear every member (`library_list`'s `overrides` array shows
//!   what is left) before the library is fully back to plain links.
//! - **Reloading or relocating a library while ANY partially-cleared
//!   hierarchy override exists ANYWHERE in the file is a native crash**
//!   (`EXCEPTION_ACCESS_VIOLATION`, null-pointer read) — not a Python
//!   exception, not something a `Result` can catch, and confirmed to bring
//!   the whole Blender process down. Reproduced with bare `library.reload()`
//!   / `bpy.ops.wm.lib_relocate()` calls, no Rust involved. The crash
//!   backtrace is identical for both:
//!   `Library_reload_call`/`wm.lib_relocate` -> `WM_lib_reload` /
//!   `BKE_blendfile_library_relocate` -> `blendfile_relocate_postprocess_liboverrides`
//!   -> `BKE_lib_override_library_main_resync` ->
//!   `lib_override_library_main_resync_on_library_indirect_level` (crash) —
//!   reload is implemented on top of the same relocate-to-the-same-path
//!   machinery, so both tools share the exact same bug.
//!
//!   The precise trigger, mapped by direct testing (create a linked
//!   armature+mesh hierarchy override, then vary what is reloaded):
//!   | override state anywhere in the file | reload same lib | reload unrelated lib |
//!   |---|---|---|
//!   | none | safe | safe |
//!   | fully intact (never cleared) | safe | safe |
//!   | partially cleared (root cleared, child still overridden) | **CRASH** | **CRASH** |
//!
//!   So the resync pass that runs on reload/relocate walks the *whole*
//!   `bpy.data`, not just the library being touched, and a dangling
//!   "orphaned" hierarchy override anywhere is enough to crash it — even one
//!   that has nothing to do with the library being reloaded. There is a
//!   per-override signature that looks like it identifies an orphan
//!   (`override_library.hierarchy_root.override_library is None` when the
//!   override is not itself the hierarchy root), but it was only checked
//!   against a 2-member hierarchy, and a false "looks safe" verdict here
//!   means a full process crash, not a wrong answer — an unacceptable
//!   trade-off for a heuristic that has not been proven across every
//!   hierarchy shape. [`library_reload`] and [`library_relocate`] therefore
//!   both take the blunt, fully-verified-safe option instead: **refuse to
//!   run at all while any library override exists anywhere in the file**,
//!   even a perfectly healthy one. Clear every override
//!   (`library_override_clear`, checking `library_list`'s `overrides` array
//!   is empty) before calling either tool.
//! - **`Library.users_id` is a tuple property, not a method** — it sits next
//!   to real methods in `dir(library)`, and calling it as one
//!   (`lib.users_id()`) raises `TypeError: 'tuple' object is not callable`.
//!   [`library_list`] reads it as a plain attribute to enumerate what a
//!   library provides.
//! - **`bpy.ops.wm.lib_relocate` runs headless with no selection/context
//!   setup** (`poll()` is `True` immediately) — confirmed by relocating a
//!   linked library to a copy of itself at a new path and reading the new
//!   `filepath`/`name` back.
//! - **Linking a file into itself is a native crash**, not a Python
//!   exception — this is exactly why [`asset_import`](super::util) refuses a
//!   self-link, and the same hazard applies to overriding/reloading: this
//!   module never opens or links files itself, it only manages libraries
//!   already linked into the current session.

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyIterator};
use serde::Deserialize;
use serde_json::json;

use super::bpy::{data, op_ok};

// ============================================================================
// Shared helpers
// ============================================================================

/// Look up an object by name, or fail with a message naming what was searched
/// for — every tool in this file needs exactly this.
fn find_object<'py>(objects: &Bound<'py, PyAny>, name: &str) -> PyResult<Bound<'py, PyAny>> {
    let obj = objects.call_method1("get", (name,))?;
    if obj.is_none() {
        return Err(PyValueError::new_err(format!("object not found: {name}")));
    }
    Ok(obj)
}

/// A linked library's real key in `bpy.data.libraries` is its local ID name
/// (`lib.blend`, `lib.blend.001` after a name clash), which a caller has no
/// way to predict. `filepath` is what [`library_list`] reports and what a
/// caller actually has, so every tool here resolves a library by scanning for
/// an exact filepath match instead.
fn find_library<'py>(data: &Bound<'py, PyAny>, filepath: &str) -> PyResult<Bound<'py, PyAny>> {
    for lib in PyIterator::from_object(&data.getattr("libraries")?)? {
        let lib = lib?;
        let lib_fp: String = lib.getattr("filepath")?.extract()?;
        if lib_fp == filepath {
            return Ok(lib);
        }
    }
    Err(PyValueError::new_err(format!(
        "no linked library with filepath: {filepath}"
    )))
}

/// The Python class name of an ID datablock (`Object`, `Mesh`,
/// `Collection`, ...), read the same way `type(x).__name__` would in Python —
/// `bpy.data` mixes every datablock type in one `user_map()`/`users_id`
/// result, and this is what tells them apart in the JSON output.
fn id_type_name(id: &Bound<'_, PyAny>) -> PyResult<String> {
    id.getattr("__class__")?.getattr("__name__")?.extract()
}

/// Refuse to proceed if ANY library override exists anywhere in the file.
///
/// This guards [`library_reload`] and [`library_relocate`] against a real,
/// reproduced native crash (`EXCEPTION_ACCESS_VIOLATION`) in Blender's
/// override-resync pass, which walks the whole file, not just the library
/// being touched — see the module doc comment for the exact backtrace and
/// the test matrix that pinned the trigger down. A crashed Blender process
/// cannot be turned into an `Err` after the fact, so this runs *before*
/// either operation and blocks unconditionally: even a healthy, never-
/// touched override is refused, because the cheaper per-override "is this
/// one orphaned" check was only verified on a single hierarchy shape and a
/// false negative there means a process crash, not a wrong answer.
fn refuse_if_any_override_exists(data: &Bound<'_, PyAny>, action: &str) -> PyResult<()> {
    for id in PyIterator::from_object(&data.call_method0("user_map")?)? {
        let id = id?;
        if !id.getattr("override_library")?.is_none() {
            let name: String = id.getattr("name")?.extract()?;
            return Err(PyValueError::new_err(format!(
                "refusing to {action}: '{name}' is a library override. Reloading or \
                 relocating a library while ANY override exists anywhere in the file is a \
                 known native Blender crash (not just overrides tied to this library) — \
                 clear every override with library_override_clear first (library_list's \
                 `overrides` array must be empty)."
            )));
        }
    }
    Ok(())
}

// ============================================================================
// library_list
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct LibraryList {}

crate::tool!(
    "library_list",
    "List every linked library in the file (filepath + the datablocks it \
     provides) and every library-override datablock currently in the file. Takes \
     no arguments. Start here: the `filepath` strings it returns are what \
     library_reload and library_relocate take, and its `overrides` array must be \
     empty before either of those will run.",
    LibraryList,
    |py: Python<'_>, _p: LibraryList| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;

        let mut libraries = Vec::new();
        for lib in PyIterator::from_object(&d.getattr("libraries")?)? {
            let lib = lib?;
            let mut datablocks = Vec::new();
            // `users_id` is a tuple attribute, not a method — see the module
            // doc comment. It lists every ID this library provides.
            for id in PyIterator::from_object(&lib.getattr("users_id")?)? {
                let id = id?;
                datablocks.push(json!({
                    "name": id.getattr("name")?.extract::<String>()?,
                    "type": id_type_name(&id)?,
                }));
            }
            libraries.push(json!({
                "name": lib.getattr("name")?.extract::<String>()?,
                "filepath": lib.getattr("filepath")?.extract::<String>()?,
                "count": datablocks.len(),
                "datablocks": datablocks,
            }));
        }

        // `user_map()` enumerates every ID datablock in the file, of every
        // type, in one call — the only generic way to sweep the whole file
        // without hand-listing every `bpy.data.*` collection name and risking
        // missing one.
        let mut overrides = Vec::new();
        for id in PyIterator::from_object(&d.call_method0("user_map")?)? {
            let id = id?;
            let ovlib = id.getattr("override_library")?;
            if ovlib.is_none() {
                continue;
            }
            overrides.push(json!({
                "name": id.getattr("name")?.extract::<String>()?,
                "type": id_type_name(&id)?,
                "reference": ovlib.getattr("reference")?.getattr("name")?.extract::<String>()?,
                "is_system_override": ovlib.getattr("is_system_override")?.extract::<bool>()?,
            }));
        }

        Ok(json!({ "libraries": libraries, "overrides": overrides }))
    }
);

// ============================================================================
// library_override_create
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct LibraryOverrideCreate {
    /// Name of the LINKED object to override, in `bpy.data.objects`. It must
    /// come from a library (see library_list) and must not already be an
    /// override; either case is an error.
    pub object: String,
    /// true: make the whole resulting hierarchy fully editable locally. false:
    /// produce a "system override" (`is_system_override` in the result), which
    /// Blender's resync machinery may later regenerate — Blender's own default,
    /// but the opposite of what "pose it and it sticks" needs. Default: true.
    pub fully_editable: Option<bool>,
}

crate::tool!(
    "library_override_create",
    "Create a library override of a linked object so it (and its linked \
     parents/children, e.g. a rig's armature + mesh) can be posed or animated \
     locally instead of staying read-only. The companion to asset_import with \
     link:true — a datablock brought in by APPENDING (asset_import link:false) or \
     by file_import is already local and needs no override. Undo local edits with \
     library_override_reset; remove the override with library_override_clear.",
    LibraryOverrideCreate,
    |py: Python<'_>, p: LibraryOverrideCreate| {
        let bpy = py.import("bpy")?;
        let objects = data(&bpy)?.getattr("objects")?;
        let obj = find_object(&objects, &p.object)?;

        if !obj.getattr("override_library")?.is_none() {
            return Err(PyValueError::new_err(format!(
                "'{}' is already a library override",
                p.object
            )));
        }
        if obj.getattr("library")?.is_none() {
            return Err(PyValueError::new_err(format!(
                "'{}' is not linked from a library — nothing to override",
                p.object
            )));
        }

        // The data-level call needs the current scene/view_layer, but — unlike
        // `bpy.ops.object.make_override_library()` — never touches selection
        // or the active object, so this has no side effect on the caller's
        // current selection.
        let ctx = bpy.getattr("context")?;
        let kw = PyDict::new(py);
        kw.set_item("do_fully_editable", p.fully_editable.unwrap_or(true))?;
        let result = obj.call_method(
            "override_hierarchy_create",
            (ctx.getattr("scene")?, ctx.getattr("view_layer")?),
            Some(&kw),
        )?;
        if result.is_none() {
            return Err(PyRuntimeError::new_err(format!(
                "Blender declined to create a library override for '{}'",
                p.object
            )));
        }

        let ovlib = result.getattr("override_library")?;
        Ok(json!({
            "object": result.getattr("name")?.extract::<String>()?,
            "reference": ovlib.getattr("reference")?.getattr("name")?.extract::<String>()?,
            "is_system_override": ovlib.getattr("is_system_override")?.extract::<bool>()?,
        }))
    }
);

// ============================================================================
// library_override_reset
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct LibraryOverrideReset {
    /// Name of the override object to reset, as library_list's `overrides`
    /// array reports it. An object that is not an override is an error.
    pub object: String,
}

crate::tool!(
    "library_override_reset",
    "Revert an override object's locally changed properties to the linked \
     reference's values, keeping the override itself in place. To take the \
     override away entirely use library_override_clear; to create one, \
     library_override_create.",
    LibraryOverrideReset,
    |py: Python<'_>, p: LibraryOverrideReset| {
        let bpy = py.import("bpy")?;
        let objects = data(&bpy)?.getattr("objects")?;
        let obj = find_object(&objects, &p.object)?;

        let ovlib = obj.getattr("override_library")?;
        if ovlib.is_none() {
            return Err(PyValueError::new_err(format!(
                "'{}' is not a library override",
                p.object
            )));
        }

        // `reset()` only reverts properties it knows differ from the
        // reference, and a plain scripted write (as every MCP tool makes)
        // does not register as a diff on its own — `operations_update()`
        // must run first to compute that diff, or `reset()` silently no-ops.
        // Verified live: without this call, resetting a location changed by
        // `obj.location.x = 42.0` left it at 42.0.
        ovlib.call_method0("operations_update")?;
        ovlib.call_method0("reset")?;

        Ok(json!({ "object": p.object, "reset": true }))
    }
);

// ============================================================================
// library_override_clear
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct LibraryOverrideClear {
    /// Name of the override object to clear, as library_list's `overrides`
    /// array reports it. An object that is not an override is an error.
    pub object: String,
}

crate::tool!(
    "library_override_clear",
    "Remove a library override, reverting the object back to the read-only linked \
     reference. Only clears the one object named — if it was overridden as part of \
     a hierarchy (see library_override_create), its children keep their own \
     overrides and must be cleared the same way, one call per object; a leftover \
     mix of cleared/still-overridden members in the same hierarchy crashes Blender \
     if you then call library_reload on that library. This tool reports any such \
     leftovers it can see so they can be cleared next.",
    LibraryOverrideClear,
    |py: Python<'_>, p: LibraryOverrideClear| {
        let bpy = py.import("bpy")?;
        let objects = data(&bpy)?.getattr("objects")?;
        let obj = find_object(&objects, &p.object)?;

        if obj.getattr("override_library")?.is_none() {
            return Err(PyValueError::new_err(format!(
                "'{}' is not a library override",
                p.object
            )));
        }

        // Overridden children (from `override_hierarchy_create`) are not
        // cleared along with their parent — see the module doc comment.
        // Snapshot which of them are overrides *before* clearing `obj`, so
        // the result can point at exactly what is left to clear.
        let mut sibling_overrides = Vec::new();
        for child in PyIterator::from_object(&obj.getattr("children_recursive")?)? {
            let child = child?;
            if !child.getattr("override_library")?.is_none() {
                sibling_overrides.push(child.getattr("name")?.extract::<String>()?);
            }
        }

        // `IDOverrideLibrary.destroy()` (the data-level call) invalidates the
        // Python wrapper immediately and does not clean up the same way the
        // operator does, so this goes through the operator instead — at the
        // cost of needing the target selected and active, exactly like
        // `make_override_library`. Selection is left as this leaves it,
        // matching `obj_set_selection`'s contract elsewhere in this crate.
        let ops_object = bpy.getattr("ops")?.getattr("object")?;
        let deselect_kw = PyDict::new(py);
        deselect_kw.set_item("action", "DESELECT")?;
        ops_object.call_method("select_all", (), Some(&deselect_kw))?;
        obj.call_method1("select_set", (true,))?;
        bpy.getattr("context")?
            .getattr("view_layer")?
            .getattr("objects")?
            .setattr("active", &obj)?;

        let result = ops_object.call_method0("clear_override_library")?;
        op_ok(&result, &format!("clear override on '{}'", p.object))?;

        // Re-check by name (the pre-clear `obj`/`child` wrappers may now be
        // stale) which of the snapshotted siblings are still overridden.
        let still_overridden: Vec<String> = sibling_overrides
            .into_iter()
            .filter(|name| {
                objects
                    .call_method1("get", (name.as_str(),))
                    .ok()
                    .filter(|o| !o.is_none())
                    .and_then(|o| o.getattr("override_library").ok())
                    .is_some_and(|ov| !ov.is_none())
            })
            .collect();

        Ok(json!({
            "object": p.object,
            "cleared": true,
            "still_overridden": still_overridden,
        }))
    }
);

// ============================================================================
// library_reload
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct LibraryReload {
    /// Filepath of the linked library to reload, matched EXACTLY against the
    /// `filepath` library_list reports. A library's `name` is an unpredictable
    /// local id ("lib.blend.001" after a clash) and is not accepted here.
    pub filepath: String,
}

crate::tool!(
    "library_reload",
    "Reload a linked library from disk so upstream changes to the source .blend \
     propagate. REFUSES to run if ANY library override exists anywhere in the file \
     (not just this library) — reloading with an override present is a known native \
     Blender crash that takes the whole process down. Clear every override with \
     library_override_clear first; library_list's `overrides` array must be empty.",
    LibraryReload,
    |py: Python<'_>, p: LibraryReload| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let lib = find_library(&d, &p.filepath)?;
        refuse_if_any_override_exists(&d, "reload a library")?;
        // `reload()` recreates the datablocks this library provides — any
        // Python wrapper for one of them held from before this call is
        // stale afterward, so nothing here reads through `lib` for anything
        // beyond confirming the call itself succeeded.
        lib.call_method0("reload")?;
        Ok(json!({ "filepath": p.filepath, "reloaded": true }))
    }
);

// ============================================================================
// library_relocate
// ============================================================================

#[derive(Deserialize, schemars::JsonSchema)]
pub struct LibraryRelocate {
    /// Current filepath of the linked library, matched EXACTLY against the
    /// `filepath` library_list reports (its `name` is not accepted).
    pub filepath: String,
    /// Path of the .blend to repoint to. Must have both a parent directory and
    /// a file name — the operator is addressed by a directory/filename split,
    /// the same way asset_import addresses a link.
    pub new_filepath: String,
}

crate::tool!(
    "library_relocate",
    "Repoint a linked library to a new .blend file on disk, re-resolving every \
     datablock it provides. REFUSES to run if ANY library override exists anywhere \
     in the file (not just this library) — relocating with an override present is a \
     known native Blender crash that takes the whole process down (it shares the \
     same internal code path as library_reload). Clear every override with \
     library_override_clear first; library_list's `overrides` array must be empty.",
    LibraryRelocate,
    |py: Python<'_>, p: LibraryRelocate| {
        let bpy = py.import("bpy")?;
        let d = data(&bpy)?;
        let lib = find_library(&d, &p.filepath)?;
        refuse_if_any_override_exists(&d, "relocate a library")?;
        let lib_name: String = lib.getattr("name")?.extract()?;

        // `wm.lib_relocate` addresses the target the same file-browser way
        // `wm.link`/`wm.append` do: a `directory` + `filename` split of the
        // new path (see `asset_import` in `util.rs` for the same shape).
        let new_path = std::path::Path::new(&p.new_filepath);
        let directory = match new_path.parent() {
            Some(dir) => format!("{}\\", dir.display()),
            None => {
                return Err(PyValueError::new_err(
                    "new_filepath has no parent directory",
                ));
            }
        };
        let filename = new_path
            .file_name()
            .and_then(|f| f.to_str())
            .ok_or_else(|| PyValueError::new_err("new_filepath has no file name"))?;

        let kw = PyDict::new(py);
        kw.set_item("library", lib_name.as_str())?;
        kw.set_item("filepath", p.new_filepath.as_str())?;
        kw.set_item("directory", directory.as_str())?;
        kw.set_item("filename", filename)?;
        let result =
            bpy.getattr("ops")?
                .getattr("wm")?
                .call_method("lib_relocate", (), Some(&kw))?;
        op_ok(
            &result,
            &format!("relocate library '{lib_name}' to {}", p.new_filepath),
        )?;

        Ok(json!({ "old_filepath": p.filepath, "new_filepath": p.new_filepath }))
    }
);
