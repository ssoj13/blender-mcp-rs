"""Animation domain: keyframes, F-curves, motion paths, NLA, Grease Pencil.

Covers every tool in `src/tools/animation.rs` and `src/tools/anim_extra.rs`. Blender 5
removed `Action.fcurves` (curves now live behind a slotted-action channelbag) and
rewrote Grease Pencil (v3: `GPENCIL` -> `GREASEPENCIL`, `frame.strokes` ->
`frame.drawing.strokes`, GP modifiers merged into the regular stack with a
`GREASE_PENCIL_*` enum). Every assertion here reads that ground truth straight off
`bpy`, not off a tool's own echoed JSON, since a broken tool can still return a
plausible-looking dict.

The factory scene (`read_factory_settings(use_empty=False)`) already has a "Cube",
"Light" and "Camera" — test objects use their own names to avoid Blender silently
renaming a collision to "Cube.001".
"""

import bpy


def get_fcurves(action):
    """Every F-Curve of an Action, across Blender's two Action layouts. Mirrors
    `super::bpy::action_fcurves` in Rust — Action.fcurves is gone in Blender 5, so a
    test that read it directly would not actually be checking what the tool checks."""
    if hasattr(action, "layers"):
        out = []
        for layer in action.layers:
            for strip in layer.strips:
                for slot in action.slots:
                    try:
                        bag = strip.channelbag(slot)
                    except TypeError:
                        continue
                    if bag is None:
                        continue
                    out.extend(bag.fcurves)
        return out
    return list(action.fcurves)


def fcurve_at(action, data_path, index):
    for fc in get_fcurves(action):
        if fc.data_path == data_path and fc.array_index == index:
            return fc
    return None


def run(h):
    h.section("animation")

    # -- primitive helper -------------------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "AnimTestCube"})

    # -- anim_set_keyframe / anim_get_keyframes / anim_get_data ------------------
    # Two keyframes on the same object so a later delete leaves the curve alive:
    # frame 1 keys whatever location is there (factory default, [0,0,0]); frame 10
    # keys an explicit value.
    h.call("anim_set_keyframe", {"name": "AnimTestCube", "frame": 1})
    h.call(
        "anim_set_keyframe",
        {"name": "AnimTestCube", "frame": 10, "value": [1.0, 2.0, 3.0]},
    )

    cube = bpy.data.objects["AnimTestCube"]
    h.check("keyframe_insert wrote a value before keying",
            list(cube.location) == [1.0, 2.0, 3.0])
    ad = cube.animation_data
    h.check("animation_data + action created", ad is not None and ad.action is not None)
    action = ad.action if ad else None

    fc0 = fcurve_at(action, "location", 0) if action else None
    h.check("location[0] fcurve exists (Blender 5 slotted-action path)", fc0 is not None)
    h.check("location[0] has 2 keyframes", fc0 is not None and len(fc0.keyframe_points) == 2)
    if fc0 is not None:
        frames = sorted(kp.co[0] for kp in fc0.keyframe_points)
        h.check("location[0] keyed at frames 1 and 10", frames == [1.0, 10.0])

    kf = h.call("anim_get_keyframes", {"name": "AnimTestCube"})
    loc_kfs = kf.get("keyframes", {}).get("location", [])
    h.check("anim_get_keyframes returns all 6 location keys (3 axes x 2 frames)",
            len(loc_kfs) == 6)
    h.check("anim_get_keyframes reports the frame-10 value on axis 0",
            any(e["frame"] == 10.0 and e["index"] == 0 and e["value"] == 1.0
                for e in loc_kfs))

    gd = h.call("anim_get_data", {"name": "AnimTestCube"})
    h.check("anim_get_data: has_animation true", gd.get("has_animation") is True)
    h.check("anim_get_data: action name matches", gd.get("action") == action.name)
    fcurves_info = gd.get("fcurves", [])
    h.check("anim_get_data: 3 location fcurves reported", len(
        [f for f in fcurves_info if f["data_path"] == "location"]) == 3)
    h.check("anim_get_data: each location fcurve has keyframe_count 2",
            all(f["keyframe_count"] == 2 for f in fcurves_info
                if f["data_path"] == "location"))

    # -- anim_set_frame -----------------------------------------------------------
    h.call("anim_set_frame", {"frame": 42})
    h.check("scene.frame_current updated", bpy.context.scene.frame_current == 42)

    # -- anim_delete_keyframe ------------------------------------------------------
    h.call("anim_delete_keyframe", {"name": "AnimTestCube", "property": "location", "frame": 10})
    fc0_after = fcurve_at(action, "location", 0)
    h.check("frame-10 key removed, curve survives (frame-1 key remains)",
            fc0_after is not None and len(fc0_after.keyframe_points) == 1
            and fc0_after.keyframe_points[0].co[0] == 1.0)

    # -- anim_add_fcurve_noise / _cycles / _envelope --------------------------------
    # These touch the removed Action.fcurves; ensure_fcurve()/find_fcurve() must
    # route through the slotted-action channelbag chain on Blender 5.
    #
    # Order matters here for a real Blender reason, not a test artifact: a "Cycles"
    # F-Curve modifier may only ever be first in the stack, so it goes on before
    # Noise/Envelope. Adding it out of order makes Blender's data API *decline*
    # (print an RPT_ERROR and return None) rather than raise -- exactly the trap
    # `new_fmodifier()` in anim_extra.rs now guards against.
    h.call("anim_add_fcurve_cycles", {
        "name": "AnimTestCube", "data_path": "location", "index": 0,
        "mode_before": "REPEAT", "mode_after": "MIRROR",
    })
    fc0 = fcurve_at(action, "location", 0)
    cycles_mods = [m for m in fc0.modifiers if m.type == "CYCLES"] if fc0 else []
    h.check("CYCLES modifier added to location[0] fcurve", len(cycles_mods) == 1)
    h.check("CYCLES modifier mode_before/after applied",
            cycles_mods and cycles_mods[0].mode_before == "REPEAT"
            and cycles_mods[0].mode_after == "MIRROR")

    h.call("anim_add_fcurve_noise", {
        "name": "AnimTestCube", "data_path": "location", "index": 0,
        "strength": 2.0, "scale": 3.0, "phase": 0.5, "offset": 0.1,
    })
    fc0 = fcurve_at(action, "location", 0)
    noise_mods = [m for m in fc0.modifiers if m.type == "NOISE"] if fc0 else []
    h.check("NOISE modifier added to the same fcurve (now 2 modifiers)",
            len(noise_mods) == 1 and len(fc0.modifiers) == 2)
    h.check("NOISE modifier params applied",
            noise_mods and noise_mods[0].strength == 2.0 and noise_mods[0].scale == 3.0)

    h.call("anim_add_fcurve_envelope", {
        "name": "AnimTestCube", "data_path": "location", "index": 0,
        "default_min": -1.0, "default_max": 1.0,
        "points": [{"frame": 1, "min": -2.0, "max": 2.0}, {"frame": 10, "min": -3.0, "max": 3.0}],
    })
    fc0 = fcurve_at(action, "location", 0)
    env_mods = [m for m in fc0.modifiers if m.type == "ENVELOPE"] if fc0 else []
    h.check("ENVELOPE modifier added (fcurve now has 3 modifiers)",
            len(env_mods) == 1 and len(fc0.modifiers) == 3)
    h.check("ENVELOPE control points match",
            env_mods and len(env_mods[0].control_points) == 2)

    # anim_add_fcurve_cycles / _envelope must refuse a missing curve rather than
    # silently no-op.
    h.errors("anim_add_fcurve_cycles on a nonexistent fcurve errors",
             "anim_add_fcurve_cycles",
             {"name": "AnimTestCube", "data_path": "scale", "index": 0})

    # A CYCLES modifier is only ever valid as the FIRST modifier on a curve, and
    # Blender reports the refusal differently per version: 5.0/5.1 returned None from
    # `modifiers.new()`, while 5.2 *creates* the modifier and flags it
    # `is_valid == False` — verified live on 5.2.2: adding CYCLES after NOISE gives
    # [('NOISE', True), ('CYCLES', False)]. The second form is a failure dressed as
    # success (the tool's JSON looks perfect, the modifier does nothing), so
    # new_fmodifier() now checks is_valid, removes the dead modifier again and errors.
    before = len(fcurve_at(action, "location", 0).modifiers)
    h.errors("a CYCLES modifier added after NOISE/ENVELOPE errors instead of "
             "silently doing nothing (Blender 5.2 is_valid == False)",
             "anim_add_fcurve_cycles",
             {"name": "AnimTestCube", "data_path": "location", "index": 0})
    fc_cyc = fcurve_at(action, "location", 0)
    cyc = [m for m in fc_cyc.modifiers if m.type == "CYCLES"] if fc_cyc else []
    h.check("the rejected CYCLES modifier was removed again, stack unchanged",
            len(cyc) == 1 and len(fc_cyc.modifiers) == before)
    h.check("the surviving CYCLES modifier is the valid first one",
            fc_cyc.modifiers[0].type == "CYCLES" and fc_cyc.modifiers[0].is_valid)

    # -- motion paths ----------------------------------------------------------
    # Previously broken: `paths_calculate` was called with start_frame/end_frame
    # kwargs it does not accept on Blender 5 (its RNA is display_type + range); the
    # fix routes the range through animation_visualization.motion_path instead.
    h.call("anim_show_motion_path", {
        "name": "AnimTestCube", "frame_start": 1, "frame_end": 10, "frame_step": 1,
    })
    cube = bpy.data.objects["AnimTestCube"]
    h.check("motion_path created", cube.motion_path is not None)
    # Verified live: MotionPath.frame_end reports one past the requested end frame
    # (frame_start=1, frame_end=10 in the request -> stored frame_end=11, 10 points),
    # a Blender quirk in how it reports the computed range, not something the tool
    # controls.
    h.check("motion_path frame range honoured",
            cube.motion_path is not None
            and cube.motion_path.frame_start == 1
            and cube.motion_path.frame_end == 11
            and len(cube.motion_path.points) == 10)

    # Recomputes an already-existing path; the path's presence is asserted just above
    # (frame range/points) and just below (cleared). "Ran" is the honest check here —
    # a recompute over unchanged animation produces the same points, so there is no new
    # effect to read.
    h.ran("anim_update_motion_paths", "anim_update_motion_paths", {"names": ["AnimTestCube"]})

    h.call("anim_clear_motion_paths", {"name": "AnimTestCube"})
    cube = bpy.data.objects["AnimTestCube"]
    h.check("motion_path cleared", cube.motion_path is None)

    # -- anim_clear ----------------------------------------------------------------
    h.call("anim_clear", {"name": "AnimTestCube"})
    cube = bpy.data.objects["AnimTestCube"]
    h.check("animation_data removed", cube.animation_data is None)

    # -- batch mode: anim_set_keyframe / anim_clear ---------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "KFBatchA"})
    h.call("obj_create_primitive", {"type": "cube", "name": "KFBatchB"})
    batch_res = h.call("anim_set_keyframe", {
        "keyframes": [
            {"name": "KFBatchA", "frame": 5},
            {"name": "KFBatchB", "frame": 5},
        ],
    })
    h.check("batch anim_set_keyframe sets both", batch_res.get("set") == 2)
    h.check("KFBatchA keyed",
            bpy.data.objects["KFBatchA"].animation_data is not None
            and bpy.data.objects["KFBatchA"].animation_data.action is not None)
    h.check("KFBatchB keyed",
            bpy.data.objects["KFBatchB"].animation_data is not None
            and bpy.data.objects["KFBatchB"].animation_data.action is not None)

    clear_res = h.call("anim_clear", {"names": ["KFBatchA", "KFBatchB"]})
    h.check("batch anim_clear clears both", clear_res.get("cleared") == 2)
    h.check("KFBatchA animation_data gone",
            bpy.data.objects["KFBatchA"].animation_data is None)
    h.check("KFBatchB animation_data gone",
            bpy.data.objects["KFBatchB"].animation_data is None)

    # -- error contract --------------------------------------------------------
    h.errors("anim_set_keyframe on missing object errors",
             "anim_set_keyframe", {"name": "NoSuchObject", "frame": 1}, "not found")
    h.errors("anim_get_keyframes on missing object errors",
             "anim_get_keyframes", {"name": "NoSuchObject"}, "not found")

    # ===========================================================================
    # NLA
    # ===========================================================================

    h.call("obj_create_primitive", {"type": "cube", "name": "NlaTestCube"})

    nla_action = h.call("nla_create_action", {"name": "WalkAction", "object": "NlaTestCube"})
    h.check("action created in bpy.data.actions", "WalkAction" in bpy.data.actions)
    nla_cube = bpy.data.objects["NlaTestCube"]
    h.check("action assigned to object",
            nla_cube.animation_data is not None
            and nla_cube.animation_data.action is not None
            and nla_cube.animation_data.action.name == "WalkAction")

    push_res = h.call("nla_push", {
        "object": "NlaTestCube", "action": "WalkAction",
        "track_name": "Track1", "start_frame": 1,
    })
    h.check("NLA track created", "Track1" in nla_cube.animation_data.nla_tracks)
    track = nla_cube.animation_data.nla_tracks["Track1"]
    h.check("NLA strip created on the track", len(track.strips) == 1)
    strip_name = push_res.get("strip")
    h.check("pushed strip carries the action", track.strips[0].action.name == "WalkAction")
    h.check("active action cleared after push (it moved to the strip)",
            nla_cube.animation_data.action is None)

    list_res = h.call("nla_list_tracks", {"object": "NlaTestCube"})
    track_names = [t["name"] for t in list_res.get("tracks", [])]
    h.check("nla_list_tracks reports Track1", "Track1" in track_names)
    strips_reported = next(
        (t["strips"] for t in list_res.get("tracks", []) if t["name"] == "Track1"), [])
    h.check("nla_list_tracks reports the strip with its action",
            any(s["action"] == "WalkAction" for s in strips_reported))

    h.call("nla_set_strip_props", {
        "object": "NlaTestCube", "track": "Track1", "strip": strip_name,
        "influence": 0.5, "mute": True, "blend_type": "ADD",
    })
    strip = track.strips[0]
    h.check("strip influence set", strip.influence == 0.5)
    h.check("strip mute set", strip.mute is True)
    h.check("strip blend_type set", strip.blend_type == "ADD")

    h.call("nla_delete_strip", {"object": "NlaTestCube", "track": "Track1", "strip": strip_name})
    h.check("strip removed", len(track.strips) == 0)

    h.errors("nla_push with no action on object and none specified errors",
             "nla_push", {"object": "NlaTestCube"}, "action")

    # ===========================================================================
    # Grease Pencil (rewritten in Blender 4.3/5 -- highest risk in this domain)
    # ===========================================================================

    h.call("gp_create", {"name": "GPTest", "location": [1.0, 2.0, 3.0]})
    gp_obj = bpy.data.objects.get("GPTest")
    h.check("GP object created with Blender-5 type string GREASEPENCIL",
            gp_obj is not None and gp_obj.type == "GREASEPENCIL")
    h.check("GP object location applied", list(gp_obj.location) == [1.0, 2.0, 3.0])
    h.check("GP object has a default layer", len(gp_obj.data.layers) == 1)

    r_layer = h.call("gp_add_layer", {"object": "GPTest", "name": "Layer2", "color": [1.0, 0.0, 0.0]})
    h.check("second layer added", "Layer2" in [ly.name for ly in gp_obj.data.layers])
    layer2 = gp_obj.data.layers["Layer2"]
    # Blender 5.2 made channel_color read-only, so the tool applies the colour as the
    # layer tint there and reports which attribute it used.
    applied = r_layer.get("color_applied_as")
    got = list(getattr(layer2, applied)) if applied else []
    h.check("layer color applied (%s)" % applied, got[:3] == [1.0, 0.0, 0.0])

    h.call("gp_add_frame", {"object": "GPTest", "layer": "Layer2", "frame": 5})
    frame_nums = [f.frame_number for f in layer2.frames]
    h.check("frame 5 added to Layer2", 5 in frame_nums)

    h.call("gp_draw_stroke", {
        "object": "GPTest", "layer": "Layer2", "frame": 5,
        "points": [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
        "line_width": 20,
    })
    frame5 = next(f for f in layer2.frames if f.frame_number == 5)
    # Blender 4.3+ compatibility shim: a frame no longer owns strokes directly.
    h.check("stroke drawn via frame.drawing.strokes (Blender 4.3+ shim)",
            len(frame5.drawing.strokes) == 1)
    stroke = frame5.drawing.strokes[0]
    h.check("stroke has all 3 points", len(stroke.points) == 3)
    # Blender 5's Grease Pencil v3 stroke has no `line_width`; the tool folds it
    # into each point's `.radius` (replaces the old line_width * pressure combo).
    h.check("point radius reflects line_width (Blender 5 has no stroke.line_width)",
            all(round(p.radius, 6) == round(20 / 2000, 6) for p in stroke.points))
    h.check("stroke point positions match (Blender 5: point.position, not .co)",
            [tuple(round(c, 4) for c in p.position) for p in stroke.points]
            == [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0)])

    layers_res = h.call("gp_list_layers", {"object": "GPTest"})
    layer2_info = next(
        (ly for ly in layers_res.get("layers", []) if ly["name"] == "Layer2"), None)
    h.check("gp_list_layers reports Layer2", layer2_info is not None)
    h.check("gp_list_layers reports frame 5 with 1 stroke",
            layer2_info is not None
            and any(f["frame"] == 5 and f["strokes"] == 1 for f in layer2_info["frames"]))

    h.call("gp_add_modifier", {"object": "GPTest", "type": "noise", "name": "MyNoise"})
    gp_mods = [m for m in gp_obj.modifiers if m.name == "MyNoise"]
    h.check("GP modifier lives on the regular modifier stack (obj.modifiers)",
            len(gp_mods) == 1)
    h.check("GP modifier type renamed to GREASE_PENCIL_* scheme",
            gp_mods and gp_mods[0].type == "GREASE_PENCIL_NOISE")

    h.errors("gp_add_layer on a non-Grease-Pencil object errors",
             "gp_add_layer", {"object": "AnimTestCube", "name": "X"}, "Grease Pencil")
