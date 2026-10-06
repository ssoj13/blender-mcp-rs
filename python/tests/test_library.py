"""library.rs coverage: library overrides and linked-library management.

A library override cannot be created against the CURRENT file (linking/appending
a .blend into itself is a native crash, not a Python exception — the same hazard
`asset_import` in `util.rs` guards against). So this test builds a second, throwaway
"library" .blend on disk first, links a rig (armature + mesh child) from it into the
current session, and only then exercises the override tools against that link.

Ground truth for the override API itself was read live off a running Blender 5.0,
never assumed — see the module doc comment on `src/tools/library.rs` for the list of
traps that caught (make_override_library needing selection but its data-level
equivalent not needing any; `reset()` silently no-op'ing on a scripted property write
unless `operations_update()` runs first; `Library.users_id` being a tuple, not a
method).
"""

import tempfile
from pathlib import Path

import bpy


def _build_library(lib_path: Path):
    """A tiny "rig": an armature with a mesh child, parented, saved as its own
    .blend. Built by hand (not via a tool call) since this is the fixture the
    tools under test are exercised against, not the thing being tested."""
    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.ops.object.armature_add(location=(0, 0, 0))
    arm = bpy.context.active_object
    arm.name = "RigArmature"
    bpy.ops.mesh.primitive_cube_add(location=(0, 0, 2))
    mesh_obj = bpy.context.active_object
    mesh_obj.name = "RigMesh"
    mesh_obj.parent = arm
    bpy.ops.wm.save_as_mainfile(filepath=str(lib_path))


def run(h):
    h.section("library")

    tmp_dir = Path(tempfile.mkdtemp(prefix="mcp_library_test_"))
    lib_path = tmp_dir / "rig_lib.blend"
    _build_library(lib_path)

    # Fresh scene (run.py's reset_scene already gave us one, but _build_library
    # just clobbered it with read_factory_settings while saving the fixture).
    bpy.ops.wm.read_factory_settings(use_empty=True)

    # ------------------------------------------------------------------
    # link the rig — read-only until overridden
    # ------------------------------------------------------------------
    directory = str(lib_path) + "\\Object\\"
    bpy.ops.wm.link(filepath=str(lib_path), directory=directory, filename="RigArmature")
    bpy.ops.wm.link(filepath=str(lib_path), directory=directory, filename="RigMesh")
    h.check("fixture: RigArmature linked and read-only",
            "RigArmature" in bpy.data.objects
            and bpy.data.objects["RigArmature"].library is not None)
    h.check("fixture: RigMesh linked and read-only",
            "RigMesh" in bpy.data.objects
            and bpy.data.objects["RigMesh"].library is not None)

    # ------------------------------------------------------------------
    # library_list: sees the linked library and what it provides
    # ------------------------------------------------------------------
    listing = h.call("library_list")
    libs = listing["libraries"]
    h.check("library_list: the rig library is reported",
            any(lib["filepath"] == str(lib_path) for lib in libs))
    rig_lib = next((lib for lib in libs if lib["filepath"] == str(lib_path)), None)
    h.check("library_list: it reports both linked datablocks",
            rig_lib is not None
            and {d["name"] for d in rig_lib["datablocks"]} >= {"RigArmature", "RigMesh"})
    h.check("library_list: no overrides yet", listing["overrides"] == [])

    # ------------------------------------------------------------------
    # library_override_create: turns the linked rig into something editable
    # ------------------------------------------------------------------
    h.errors("library_override_create on a non-linked object errors",
              "library_override_create", {"object": "NoSuchObject"}, "not found")

    result = h.call("library_override_create", {"object": "RigArmature"})
    h.check("library_override_create: reports the reference it overrode",
            result["reference"] == "RigArmature")
    h.check("library_override_create: fully_editable default -> not a system override",
            result["is_system_override"] is False)

    override_name = result["object"]
    override_obj = bpy.data.objects[override_name]
    h.check("library_override_create: the override is local (no .library)",
            override_obj.library is None)
    h.check("library_override_create: override_library is actually set",
            override_obj.override_library is not None)
    h.check("library_override_create: the original linked datablock still exists",
            any(o.name == "RigArmature" and o.library is not None for o in bpy.data.objects))

    # Verify it cascaded onto the linked child too (the whole hierarchy, not
    # just the requested object).
    mesh_override = next(
        (o for o in bpy.data.objects if o.name == "RigMesh" and o.override_library is not None),
        None,
    )
    h.check("library_override_create: the linked mesh child was overridden too",
            mesh_override is not None and mesh_override.parent == override_obj)

    h.errors("library_override_create on an already-overridden object errors",
              "library_override_create", {"object": override_name}, "already")

    # ------------------------------------------------------------------
    # library_reload / library_relocate MUST refuse while any override
    # exists anywhere in the file, even one that was never partially
    # cleared and looks perfectly healthy.
    #
    # Verified live (see src/tools/library.rs's module doc comment): a
    # PARTIALLY cleared hierarchy override anywhere in the file crashes
    # Blender natively (EXCEPTION_ACCESS_VIOLATION, not a Python exception)
    # on the next library_reload/library_relocate call — reproduced with
    # bare library.reload()/bpy.ops.wm.lib_relocate() calls, no Rust
    # involved, and the crash hits even when reloading/relocating a
    # DIFFERENT, unrelated library. Since a crash can't become an Err after
    # the fact, both tools refuse unconditionally while ANY override
    # exists, healthy or not — proven here while `override_obj` is still
    # fully intact (never reset or cleared), so this is exercising the
    # guard itself, not dodging a real crash.
    # ------------------------------------------------------------------
    h.errors("library_reload refuses while an override exists (even a healthy one)",
              "library_reload", {"filepath": str(lib_path)}, "override")
    h.errors("library_relocate refuses while an override exists (even a healthy one)",
              "library_relocate", {"filepath": str(lib_path),
                                    "new_filepath": str(tmp_dir / "unused_target.blend")},
              "override")
    h.check("guard: refusing left the library filepath untouched",
            bpy.data.libraries[0].filepath == str(lib_path))

    # ------------------------------------------------------------------
    # pose it locally — this is the whole point: it must actually stick
    # ------------------------------------------------------------------
    override_obj.location.x = 12.5
    bpy.context.view_layer.update()
    h.check("posed locally: location actually changed and holds",
            abs(override_obj.location.x - 12.5) < 1e-6)

    # ------------------------------------------------------------------
    # library_override_reset: reverts the pose back to the reference
    # ------------------------------------------------------------------
    # RigMesh is itself an override too (the hierarchy cascaded onto it), so
    # the guard needs a genuinely plain local object instead.
    bpy.ops.mesh.primitive_cube_add()
    bpy.context.active_object.name = "PlainLocalCube"
    h.errors("library_override_reset on a non-override object errors",
              "library_override_reset", {"object": "PlainLocalCube"}, "not")

    h.call("library_override_reset", {"object": override_name})
    bpy.context.view_layer.update()
    h.check("library_override_reset: location actually reverted to the reference (0,0,0)",
            abs(bpy.data.objects[override_name].location.x) < 1e-6)

    # ------------------------------------------------------------------
    # library_override_clear: back to a plain read-only link
    #
    # Verified live: clearing does NOT cascade through a hierarchy the way
    # creating does. Clearing just the armature leaves the mesh's override
    # behind (still override_library is not None) — clear_result reports
    # that leftover in "still_overridden" so it can be cleared next. Leaving
    # a hierarchy half-cleared and then reloading the library is a native
    # Blender crash (reproduced with a bare `library.reload()` call, nothing
    # to do with this Rust code) — see src/tools/library.rs's module doc
    # comment. Clearing every reported leftover before reload avoids it.
    # ------------------------------------------------------------------
    before_count = len([o for o in bpy.data.objects if o.name == override_name])
    clear_result = h.call("library_override_clear", {"object": override_name})
    h.check("library_override_clear: reports the mesh override left behind",
            clear_result["still_overridden"] == ["RigMesh"])
    remaining = [o for o in bpy.data.objects if o.name == "RigArmature"]
    h.check("library_override_clear: the override datablock is gone",
            all(o.override_library is None for o in remaining))
    h.check("library_override_clear: the plain linked object is still there",
            any(o.library is not None for o in remaining))
    h.check("library_override_clear: fewer RigArmature entries than before (override freed)",
            len(remaining) < before_count or before_count <= 1)

    h.errors("library_override_clear on an already-cleared object errors",
              "library_override_clear", {"object": "RigArmature"}, "not")

    # Clear the reported leftover too, or the reload below crashes Blender.
    mesh_clear = h.call("library_override_clear", {"object": "RigMesh"})
    h.check("library_override_clear on the leftover mesh: nothing left behind",
            mesh_clear["still_overridden"] == [])
    h.check("library_override_clear: whole hierarchy is now plain linked data again",
            all(o.override_library is None for o in bpy.data.objects
                if o.name in ("RigArmature", "RigMesh")))

    # ------------------------------------------------------------------
    # library_reload: re-reads the library from disk
    # ------------------------------------------------------------------
    h.errors("library_reload on an unknown filepath errors",
              "library_reload", {"filepath": str(tmp_dir / "nope.blend")}, "no linked library")

    reload_result = h.call("library_reload", {"filepath": str(lib_path)})
    h.check("library_reload: reports the filepath back", reload_result["filepath"] == str(lib_path))
    h.check("library_reload: the linked datablock survives the round trip",
            "RigArmature" in bpy.data.objects)

    # ------------------------------------------------------------------
    # library_relocate: repoint to a copy of the library at a new path
    # ------------------------------------------------------------------
    import shutil
    relocated_path = tmp_dir / "rig_lib_relocated.blend"
    shutil.copy2(lib_path, relocated_path)

    h.errors("library_relocate on an unknown filepath errors",
              "library_relocate", {"filepath": str(tmp_dir / "nope.blend"),
                                    "new_filepath": str(relocated_path)},
              "no linked library")

    h.call("library_relocate", {"filepath": str(lib_path), "new_filepath": str(relocated_path)})
    relocated = h.call("library_list")
    h.check("library_relocate: the library now reports the new filepath",
            any(lib["filepath"] == str(relocated_path) for lib in relocated["libraries"]))
    h.check("library_relocate: the old filepath is gone",
            all(lib["filepath"] != str(lib_path) for lib in relocated["libraries"]))
    h.check("library_relocate: the linked object is still resolvable",
            "RigArmature" in bpy.data.objects)
