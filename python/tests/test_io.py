"""util.rs / misc.rs / bake.rs regression coverage.

Every assertion reads state back off `bpy`, not off a tool's own echoed JSON — a
broken tool can still return a plausible-looking dict. Ground truth (enum values,
operator signatures) was read live off a running Blender 5.0, never assumed:
`StripsTopLevel.new_effect`'s real signature is `(name, type, channel, frame_start,
length=0, input1=None, input2=None)` — no `frame_end`, and the two-strip inputs are
`input1`/`input2`, not the pre-4.4 `seq1`/`seq2` the legacy handler used. See
`src/tools/AGENTS.md` for the rest of the Blender-5 VSE rename.
"""

import bpy


def run(h):
    # ------------------------------------------------------------------
    # util: align / distribute / snap / randomize
    # ------------------------------------------------------------------
    h.section("util-transform")

    h.call("obj_create_primitive", {"type": "cube", "name": "A", "location": [0, 0, 0]})
    h.call("obj_create_primitive", {"type": "cube", "name": "B", "location": [5, 3, 7]})
    h.call("obj_create_primitive", {"type": "cube", "name": "C", "location": [-2, 1, 4]})

    h.call("util_align", {"names": ["A", "B", "C"], "axis": "X", "align_to": "MIN"})
    xs = [bpy.data.objects[n].location.x for n in ("A", "B", "C")]
    h.check("util_align MIN: all X equal to the minimum", all(x == min(xs) for x in xs))

    h.call("util_distribute", {"names": ["A", "B", "C"], "axis": "Y"})
    ys = sorted(bpy.data.objects[n].location.y for n in ("A", "B", "C"))
    h.check("util_distribute: evenly spaced along Y",
            abs((ys[1] - ys[0]) - (ys[2] - ys[1])) < 1e-6)

    bpy.data.objects["A"].location.z = 5.0
    # `matrix_world` is not recomputed synchronously when `location` is set
    # directly (worse in --background, where nothing else ever forces the
    # depsgraph to resolve) — refresh it explicitly before reading it back,
    # both here and after the tool call below, or this check would be testing
    # a stale transform rather than the tool's actual effect.
    bpy.context.view_layer.update()
    h.call("util_snap_to_ground", {"name": "A", "target_z": 0.0})
    bpy.context.view_layer.update()
    h.check("util_snap_to_ground: bottom of bbox at target Z",
            abs(min((bpy.data.objects["A"].matrix_world @ v.co).z
                    for v in bpy.data.objects["A"].data.vertices)) < 1e-5)

    before = bpy.data.objects["B"].location.copy()
    h.call("util_randomize_transform", {"name": "B", "loc_range": [1, 1, 1], "seed": 42})
    after = bpy.data.objects["B"].location
    h.check("util_randomize_transform: location actually perturbed",
            (before - after).length > 1e-9)

    h.call("util_randomize_transform", {"names": ["A", "C"], "loc_range": [0.5, 0, 0], "seed": 1})
    h.check("util_randomize_transform batch: still exactly 3 objects", len(bpy.data.objects) >= 3)

    # ------------------------------------------------------------------
    # util: boolean
    # ------------------------------------------------------------------
    h.section("util-boolean")

    h.call("obj_create_primitive", {"type": "cube", "name": "BoolA", "location": [0, 0, 0]})
    h.call("obj_create_primitive", {"type": "cube", "name": "BoolB", "location": [0.5, 0, 0]})
    h.call("util_boolean", {"object": "BoolA", "target": "BoolB"})
    mods = bpy.data.objects["BoolA"].modifiers
    h.check("util_boolean: BOOLEAN modifier added with default (EXACT) solver",
            len(mods) == 1 and mods[0].type == "BOOLEAN" and mods[0].solver == "EXACT")

    h.call("obj_create_primitive", {"type": "cube", "name": "BoolC", "location": [0, 0, 0]})
    h.call("obj_create_primitive", {"type": "cube", "name": "BoolD", "location": [0.5, 0, 0]})
    h.call("util_boolean", {"object": "BoolC", "target": "BoolD", "apply": True})
    h.check("util_boolean apply=True: modifier applied (gone from stack)",
            len(bpy.data.objects["BoolC"].modifiers) == 0)

    h.call("util_apply_booleans", {"name": "BoolA"})
    h.check("util_apply_booleans single: BOOLEAN modifier gone",
            all(m.type != "BOOLEAN" for m in bpy.data.objects["BoolA"].modifiers))

    # ------------------------------------------------------------------
    # asset browser
    # ------------------------------------------------------------------
    h.section("asset")

    h.call("asset_mark", {"name": "A", "type": "OBJECT", "description": "test asset",
                           "tags": ["foo", "bar"]})
    h.check("asset_mark: asset_data present with description/tags",
            bpy.data.objects["A"].asset_data is not None
            and bpy.data.objects["A"].asset_data.description == "test asset"
            and {t.name for t in bpy.data.objects["A"].asset_data.tags} == {"foo", "bar"})

    listing = h.call("asset_list", {"type": "OBJECT"})
    h.check("asset_list: marked object shows up",
            any(a["name"] == "A" for a in listing["assets"]))

    h.call("asset_unmark", {"name": "A", "type": "OBJECT"})
    h.check("asset_unmark: asset_data cleared", bpy.data.objects["A"].asset_data is None)

    h.errors("asset_mark on unknown object errors",
              "asset_mark", {"name": "NoSuchObject", "type": "OBJECT"}, "not found")
    h.errors("asset_mark with bad type errors",
              "asset_mark", {"name": "A", "type": "NOPE"}, "invalid type")

    # ------------------------------------------------------------------
    # VSE — the confirmed-broken area. Ground truth for `new_effect` read live:
    # `StripsTopLevel.new_effect(name, type, channel, frame_start, length=0,
    # input1=None, input2=None)`. No `frame_end` kwarg exists at all; the old
    # `seq1`/`seq2` handler-level names are `input1`/`input2` on this API.
    # ------------------------------------------------------------------
    h.section("vse")

    h.call("vse_setup", {"clear": True})
    se = bpy.context.scene.sequence_editor
    strips_coll = se.strips if hasattr(se, "strips") else se.sequences

    color = h.call("vse_add_strip", {
        "type": "COLOR", "channel": 1, "frame_start": 1,
        "color": [1.0, 0.0, 0.0], "length": 40, "name": "RedColor",
    })
    h.check("vse_add_strip COLOR: strip actually created in bpy",
            "RedColor" in strips_coll and strips_coll["RedColor"].type == "COLOR")
    h.check("vse_add_strip COLOR: color applied",
            tuple(round(c, 3) for c in strips_coll["RedColor"].color) == (1.0, 0.0, 0.0))
    h.check("vse_add_strip COLOR: frame_end reported matches frame_start+length",
            color["frame_end"] == 1 + 40)

    color2 = h.call("vse_add_strip", {
        "type": "COLOR", "channel": 2, "frame_start": 1,
        "color": [0.0, 1.0, 0.0], "length": 40, "name": "GreenColor",
    })
    h.check("vse_add_strip COLOR #2 created", "GreenColor" in strips_coll)

    h.call("vse_add_strip", {
        "type": "TEXT", "channel": 3, "frame_start": 1, "length": 30,
        "text": "hello", "name": "HelloText",
    })
    h.check("vse_add_strip TEXT: strip created with the right text",
            "HelloText" in strips_coll and strips_coll["HelloText"].text == "hello")

    h.call("vse_add_effect", {
        "type": "CROSS", "seq1": "RedColor", "seq2": "GreenColor", "name": "CrossFade",
    })
    h.check("vse_add_effect two-input (CROSS): strip created, spans both inputs",
            "CrossFade" in strips_coll
            and strips_coll["CrossFade"].frame_start
            == min(strips_coll["RedColor"].frame_start, strips_coll["GreenColor"].frame_start)
            and strips_coll["CrossFade"].frame_final_end
            == max(strips_coll["RedColor"].frame_final_end,
                    strips_coll["GreenColor"].frame_final_end))

    h.call("vse_add_effect", {"type": "GLOW", "seq1": "RedColor", "name": "GlowFx"})
    h.check("vse_add_effect single-input (GLOW): strip created",
            "GlowFx" in strips_coll and strips_coll["GlowFx"].type == "GLOW")

    listed = h.call("vse_list_strips")
    h.check("vse_list_strips: sees every strip created above",
            {s["name"] for s in listed["strips"]}
            >= {"RedColor", "GreenColor", "HelloText", "CrossFade", "GlowFx"})

    h.call("vse_set_strip_props", {"name": "RedColor", "mute": True, "blend_type": "ALPHA_OVER"})
    h.check("vse_set_strip_props: mute + blend_type persisted",
            strips_coll["RedColor"].mute is True
            and strips_coll["RedColor"].blend_type == "ALPHA_OVER")

    h.call("vse_delete_strip", {"name": "GlowFx"})
    h.check("vse_delete_strip: strip actually removed", "GlowFx" not in strips_coll)

    h.errors("vse_add_strip COLOR requires nothing extra but bad type errors",
              "vse_add_strip", {"type": "NOPE_STRIP_TYPE"}, "unknown strip type")
    h.errors("vse_add_effect on missing input strip errors",
              "vse_add_effect", {"type": "GLOW", "seq1": "NoSuchStrip"}, "not found")

    # ------------------------------------------------------------------
    # misc: scene_render (needs a camera to point at something concrete, but
    # EEVEE will render regardless)
    # ------------------------------------------------------------------
    h.section("misc-render")

    import tempfile
    from pathlib import Path

    tmp_dir = Path(tempfile.mkdtemp(prefix="mcp_io_test_"))
    render_out = tmp_dir / "render.png"
    old_engine = bpy.context.scene.render.engine
    old_fp = bpy.context.scene.render.filepath
    h.call("scene_render", {"output": str(render_out), "width": 64, "height": 64})
    h.check("scene_render: file actually written", render_out.exists() and render_out.stat().st_size > 0)
    h.check("scene_render: render engine restored after the call",
            bpy.context.scene.render.engine == old_engine)
    h.check("scene_render: filepath restored after the call",
            bpy.context.scene.render.filepath == old_fp)

    # ------------------------------------------------------------------
    # misc: modifiers
    # ------------------------------------------------------------------
    h.section("misc-modifiers")

    h.call("obj_create_primitive", {"type": "cube", "name": "ModObj"})
    h.call("mod_add", {"name": "ModObj", "type": "SUBSURF", "mod_name": "Sub1",
                        "properties": {"levels": 2}})
    h.check("mod_add: modifier present with property applied",
            "Sub1" in bpy.data.objects["ModObj"].modifiers
            and bpy.data.objects["ModObj"].modifiers["Sub1"].levels == 2)

    listed_mods = h.call("mod_list", {"name": "ModObj"})
    h.check("mod_list: reports the modifier just added",
            any(m["name"] == "Sub1" for m in listed_mods))

    h.call("mod_update", {"name": "ModObj", "modifier": "Sub1", "properties": {"levels": 3}})
    h.check("mod_update: property actually changed",
            bpy.data.objects["ModObj"].modifiers["Sub1"].levels == 3)

    h.call("mod_apply", {"name": "ModObj", "modifier": "Sub1"})
    h.check("mod_apply: modifier gone from the stack (applied)",
            "Sub1" not in bpy.data.objects["ModObj"].modifiers)

    h.call("mod_add", {"name": "ModObj", "type": "BEVEL", "mod_name": "Bev1"})
    h.call("mod_delete", {"name": "ModObj", "modifier": "Bev1"})
    h.check("mod_delete: modifier actually removed",
            "Bev1" not in bpy.data.objects["ModObj"].modifiers)

    h.errors("mod_add on missing object errors",
              "mod_add", {"name": "NoSuchObj", "type": "SUBSURF"}, "not found")

    # ------------------------------------------------------------------
    # misc: collections
    # ------------------------------------------------------------------
    h.section("misc-collections")

    h.call("col_create", {"name": "TestColl"})
    h.check("col_create: collection exists in bpy.data", "TestColl" in bpy.data.collections)

    listed_cols = h.call("col_list")
    h.check("col_list: returns the scene root collection", isinstance(listed_cols, list) and len(listed_cols) == 1)

    h.call("col_move_to", {"object": "ModObj", "collection": "TestColl"})
    h.check("col_move_to: object actually linked into the collection",
            "ModObj" in [o.name for o in bpy.data.collections["TestColl"].objects])

    h.call("col_rename", {"name": "TestColl", "new_name": "RenamedColl"})
    h.check("col_rename: new name present, old name gone",
            "RenamedColl" in bpy.data.collections and "TestColl" not in bpy.data.collections)

    h.call("col_delete", {"name": "RenamedColl"})
    h.check("col_delete: collection actually removed", "RenamedColl" not in bpy.data.collections)

    # ------------------------------------------------------------------
    # misc: file I/O round trip (OBJ: simple, no external deps)
    # ------------------------------------------------------------------
    h.section("misc-fileio")

    export_path = tmp_dir / "roundtrip.obj"
    h.call("obj_create_primitive", {"type": "cube", "name": "ExportMe"})
    h.call("file_export", {"path": str(export_path), "format": "obj", "selected_only": False})
    h.check("file_export obj: file actually written",
            export_path.exists() and export_path.stat().st_size > 0)

    before_names = set(bpy.data.objects.keys())
    h.call("file_import", {"path": str(export_path), "format": "obj"})
    after_names = set(bpy.data.objects.keys())
    h.check("file_import obj: at least one new object appeared", len(after_names - before_names) >= 1)

    for fmt in ("stl", "ply"):
        interchange_path = tmp_dir / f"roundtrip.{fmt}"
        h.call("file_export", {"path": str(interchange_path), "format": fmt})
        h.check(f"file_export {fmt}: file actually written",
                interchange_path.exists() and interchange_path.stat().st_size > 0)
        before_names = set(bpy.data.objects.keys())
        h.call("file_import", {"path": str(interchange_path)})
        h.check(f"file_import {fmt}: mesh object appeared",
                any(bpy.data.objects[name].type == "MESH"
                    for name in set(bpy.data.objects.keys()) - before_names))

    save_path = tmp_dir / "saved.blend"
    h.call("file_save", {"path": str(save_path)})
    h.check("file_save: .blend actually written", save_path.exists() and save_path.stat().st_size > 0)

    h.errors("file_export rejects unknown format",
              "file_export", {"path": str(tmp_dir / "x.xyz"), "format": "xyz"}, "unknown format")

    # ------------------------------------------------------------------
    # misc: util_execute_python / util_set_mode / undo-redo
    # ------------------------------------------------------------------
    h.section("misc-util")

    result = h.call("util_execute_python", {"code": "__result__ = 1 + 41"})
    h.check("util_execute_python: __result__ round-trips through JSON", result["result"] == 42)

    h.call("obj_create_primitive", {"type": "cube", "name": "ModeObj"})
    bpy.context.view_layer.objects.active = bpy.data.objects["ModeObj"]
    h.call("util_set_mode", {"mode": "EDIT"})
    h.check("util_set_mode: object actually entered EDIT mode",
            bpy.data.objects["ModeObj"].mode == "EDIT")
    h.call("util_set_mode", {"mode": "OBJECT"})
    h.check("util_set_mode: object actually back in OBJECT mode",
            bpy.data.objects["ModeObj"].mode == "OBJECT")

    # Documented Blender limitation, not a bug: undo is disabled at startup in a
    # freshly loaded/factory scene, so ed.undo/ed.redo raise. Assert they RAISE.
    h.errors("util_undo raises (undo disabled at startup, headless)", "util_undo", {})
    h.errors("util_redo raises (undo disabled at startup, headless)", "util_redo", {})

    # ------------------------------------------------------------------
    # bake: EMIT bake onto a fresh image, verified by diffing pixels
    # ------------------------------------------------------------------
    h.section("bake")

    h.call("obj_create_primitive", {"type": "cube", "name": "BakeObj"})
    # The default cube primitive already carries a UV map from the creation
    # operator, so no explicit mesh_unwrap is needed here — but call it anyway
    # to exercise that tool too, since it's one of this module's own.
    h.call("mesh_unwrap", {"name": "BakeObj", "method": "angle_based"})

    h.call("img_new", {"name": "BakeTarget", "width": 32, "height": 32,
                        "color": [0.0, 0.0, 0.0, 1.0]})
    before_pixels = list(bpy.data.images["BakeTarget"].pixels)

    h.call("mat_create", {"name": "BakeMat"})
    h.call("mat_assign", {"object": "BakeObj", "material": "BakeMat"})
    h.call("node_graph_build", {
        "tree": "material:BakeMat",
        "clear": True,
        "nodes": [
            {"id": "emit", "type": "ShaderNodeEmission",
             "inputs": {"Color": [1.0, 0.0, 0.0, 1.0], "Strength": 4.0}},
            {"id": "out", "type": "ShaderNodeOutputMaterial"},
            {"id": "tex", "type": "ShaderNodeTexImage", "props": {"image": "BakeTarget"}},
        ],
        "links": [["emit.Emission", "out.Surface"]],
    })

    h.call("bake_texture", {"object": "BakeObj", "image": "BakeTarget", "type": "EMIT",
                             "samples": 4})
    after_pixels = list(bpy.data.images["BakeTarget"].pixels)
    h.check("bake_texture EMIT: pixels actually changed (not a silent no-op FINISHED)",
            before_pixels != after_pixels)
    h.check("bake_texture EMIT: some pixel picked up red emission",
            any(after_pixels[i] > 0.5 for i in range(0, len(after_pixels), 4)))

    # A mesh with zero UV layers, built directly (bpy.ops primitives always come
    # with a default UV map, so this fixture is built by hand instead of through
    # a tool call — the tool under test is the bake guard, not mesh creation).
    no_uv_mesh = bpy.data.meshes.new("NoUvMesh")
    no_uv_obj = bpy.data.objects.new("NoUvObj", no_uv_mesh)
    bpy.context.scene.collection.objects.link(no_uv_obj)
    no_uv_mesh.from_pydata([(0, 0, 0), (1, 0, 0), (1, 1, 0)], [], [(0, 1, 2)])
    no_uv_mesh.update()
    h.check("fixture: NoUvObj really has no UV layer", len(no_uv_mesh.uv_layers) == 0)
    h.errors("bake_texture on object with no UV errors",
              "bake_texture", {"object": "NoUvObj", "image": "BakeTarget", "type": "EMIT"},
              "UV")
    h.errors("bake_texture on missing image errors",
              "bake_texture", {"object": "BakeObj", "image": "NoSuchImage", "type": "EMIT"},
              "not found")
    h.errors("bake_texture on missing object errors",
              "bake_texture", {"object": "NoSuchObj", "image": "BakeTarget", "type": "EMIT"},
              "not found")
