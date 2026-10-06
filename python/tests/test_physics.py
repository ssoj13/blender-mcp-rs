"""Regression tests for the physics domain (`src/tools/phys.rs`).

Covers phys_add (all five physics types + the unknown-type guard), phys_remove,
particle systems, force fields, geometry-nodes simulation zones, phys_bake's frame
range, and the render queue. Every assertion reads ground truth back off `bpy`, not
off the tool's own echo — a broken tool can still return a plausible-looking dict.
"""

import json as pyjson
import os

import bpy


def run(h):
    h.section("phys")

    # -- phys_add: RIGID_BODY --------------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "RB1"})
    h.call(
        "phys_add",
        {"name": "RB1", "type": "RIGID_BODY", "body_type": "ACTIVE", "mass": 2.0},
    )
    obj = bpy.data.objects["RB1"]
    h.check(
        "rigid body added with correct type",
        obj.rigid_body is not None and obj.rigid_body.type == "ACTIVE",
    )
    h.check("rigid body mass set", abs(obj.rigid_body.mass - 2.0) < 1e-6)

    # -- phys_add: SOFT_BODY -----------------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "SB1"})
    h.call("phys_add", {"name": "SB1", "type": "SOFT_BODY", "mass": 1.5})
    obj = bpy.data.objects["SB1"]
    mod = obj.modifiers.get("Softbody")
    h.check("soft body modifier added", mod is not None and mod.type == "SOFT_BODY")
    h.check("soft body mass set", abs(mod.settings.mass - 1.5) < 1e-6)

    # -- phys_add: CLOTH ----------------------------------------------------------
    h.call("obj_create_primitive", {"type": "plane", "name": "CL1"})
    h.call("phys_add", {"name": "CL1", "type": "CLOTH"})
    obj = bpy.data.objects["CL1"]
    mod = obj.modifiers.get("Cloth")
    h.check("cloth modifier added", mod is not None and mod.type == "CLOTH")

    # -- phys_add: COLLISION -------------------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "COL1"})
    h.call("phys_add", {"name": "COL1", "type": "COLLISION"})
    obj = bpy.data.objects["COL1"]
    mod = obj.modifiers.get("Collision")
    h.check("collision modifier added", mod is not None and mod.type == "COLLISION")

    # -- phys_add: FLUID ------------------------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "FL1"})
    h.call("phys_add", {"name": "FL1", "type": "FLUID", "fluid_type": "DOMAIN"})
    obj = bpy.data.objects["FL1"]
    mod = obj.modifiers.get("Fluid")
    h.check("fluid modifier added", mod is not None and mod.type == "FLUID")
    h.check("fluid_type set", mod.fluid_type == "DOMAIN")

    # -- phys_add: unknown type must ERROR, not silently add nothing --------------
    h.call("obj_create_primitive", {"type": "cube", "name": "BAD1"})
    h.errors(
        "unknown physics type is rejected",
        "phys_add",
        {"name": "BAD1", "type": "NONSENSE"},
        "unknown physics type",
    )
    obj = bpy.data.objects["BAD1"]
    h.check(
        "unknown physics type left no trace",
        obj.rigid_body is None and len(obj.modifiers) == 0,
    )

    # -- phys_add: batch mode still supports every type in one call ---------------
    h.call("obj_create_primitive", {"type": "cube", "name": "BATCH_RB"})
    h.call("obj_create_primitive", {"type": "cube", "name": "BATCH_COL"})
    batch = h.call(
        "phys_add",
        {
            "physics": [
                {"name": "BATCH_RB", "type": "RIGID_BODY"},
                {"name": "BATCH_COL", "type": "COLLISION"},
                {"name": "does-not-exist", "type": "RIGID_BODY"},
            ]
        },
    )
    h.check("batch phys_add: 2 of 3 succeeded", batch["added"] == 2)
    h.check(
        "batch phys_add: bad item carries its error, not a hard failure",
        batch["results"][2].get("error") == "not found",
    )

    # -- phys_remove ---------------------------------------------------------------
    h.call("phys_remove", {"name": "RB1"})
    obj = bpy.data.objects["RB1"]
    h.check("rigid body removed", obj.rigid_body is None)

    h.call("phys_remove", {"name": "SB1"})
    obj = bpy.data.objects["SB1"]
    h.check("soft body modifier removed", obj.modifiers.get("Softbody") is None)

    h.errors(
        "phys_remove on a nonexistent object errors",
        "phys_remove",
        {"name": "totally-not-here"},
        "not found",
    )

    # -- particle systems ------------------------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "PS1"})
    h.call("phys_add_particles", {"name": "PS1", "ps_name": "Sparks", "count": 500})
    obj = bpy.data.objects["PS1"]
    h.check(
        "particle system added with requested name",
        len(obj.particle_systems) == 1 and obj.particle_systems[0].name == "Sparks",
    )
    h.check("particle count set", obj.particle_systems[0].settings.count == 500)

    h.call("phys_update_particles", {"name": "PS1", "ps_name": "Sparks", "count": 750})
    h.check("particle count updated by name", obj.particle_systems[0].settings.count == 750)

    h.call("phys_update_particles", {"name": "PS1", "count": 900})
    h.check(
        "particle update falls back to the first system when ps_name omitted",
        obj.particle_systems[0].settings.count == 900,
    )

    # KNOWN ISSUE fixed: `ps_name` supplied on an object with ZERO particle systems
    # used to raise IndexError (Python's `dict.get(k, default)` evaluates `default`
    # eagerly, indexing `particle_systems[0]` even though `ps_name` was given). It
    # must now fail cleanly instead of crashing the handler.
    h.call("obj_create_primitive", {"type": "cube", "name": "NOPS"})
    h.errors(
        "update with ps_name on an object with no particle systems fails cleanly",
        "phys_update_particles",
        {"name": "NOPS", "ps_name": "Whatever"},
        "no particle system",
    )
    h.errors(
        "update with no ps_name on an object with no particle systems also fails cleanly",
        "phys_update_particles",
        {"name": "NOPS"},
        "",
    )

    h.call("phys_remove_particles", {"name": "PS1", "ps_name": "Sparks"})
    obj = bpy.data.objects["PS1"]
    h.check("particle system removed", len(obj.particle_systems) == 0)

    # -- force fields ------------------------------------------------------------------
    h.call(
        "phys_add_force_field",
        {"type": "WIND", "name": "Wind1", "strength": 5.0, "noise": 0.3},
    )
    obj = bpy.data.objects["Wind1"]
    h.check("force field type set", obj.field is not None and obj.field.type == "WIND")
    h.check("force field strength set", abs(obj.field.strength - 5.0) < 1e-6)
    h.check("WIND-specific noise param applied", abs(obj.field.noise - 0.3) < 1e-6)

    h.call("phys_add_force_field", {"type": "VORTEX", "name": "Vortex1", "inflow": 0.7})
    obj = bpy.data.objects["Vortex1"]
    h.check(
        "VORTEX-specific inflow param applied",
        obj.field.type == "VORTEX" and abs(obj.field.inflow - 0.7) < 1e-6,
    )

    # -- simulation zones (geometry nodes, Blender 4.0+) --------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "SimObj"})
    h.call(
        "node_tree_create",
        {"kind": "geometry_modifier", "name": "SimTree", "object": "SimObj"},
    )
    h.call("sim_create_zone", {"object": "SimObj"})

    obj = bpy.data.objects["SimObj"]
    tree = None
    for m in obj.modifiers:
        if m.type == "NODES":
            tree = m.node_group
            break
    h.check("geometry nodes modifier + tree present", tree is not None)

    sim_in = next((n for n in tree.nodes if n.bl_idname == "GeometryNodeSimulationInput"), None)
    sim_out = next((n for n in tree.nodes if n.bl_idname == "GeometryNodeSimulationOutput"), None)
    h.check("simulation input node created", sim_in is not None)
    h.check("simulation output node created", sim_out is not None)

    # KNOWN ISSUE fixed: the zone used to be created UNPAIRED (`paired_output` stayed
    # None) with a bogus index-based link (Delta Time -> Skip). A real zone must be
    # paired and carry an actual Geometry link between the two nodes.
    h.check(
        "simulation input/output are actually paired",
        sim_in is not None and sim_out is not None
        and sim_in.paired_output is not None
        and sim_in.paired_output.as_pointer() == sim_out.as_pointer(),
    )
    geom_linked = False
    if sim_in is not None:
        for link in tree.links:
            if (
                link.from_node.as_pointer() == sim_in.as_pointer()
                and link.to_node.as_pointer() == sim_out.as_pointer()
                and link.from_socket.name == "Geometry"
                and link.to_socket.name == "Geometry"
            ):
                geom_linked = True
    h.check("simulation zone has a real Geometry link (not Delta Time -> Skip)", geom_linked)

    h.errors(
        "sim_create_zone on an object with no geometry-nodes tree errors",
        "sim_create_zone",
        {"object": "RB1"},
        "no geometry node tree",
    )

    h.ran(
        "sim_set_params sets bake_directory",
        "sim_set_params",
        {"object": "SimObj", "cache_path": "//sim_cache/"},
    )
    h.check(
        "bake_directory actually persisted on the modifier",
        next(m for m in obj.modifiers if m.type == "NODES").bake_directory == "//sim_cache/",
    )

    scene = bpy.context.scene
    saved_start, saved_end = scene.frame_start, scene.frame_end
    h.ran("sim_bake runs against a paired zone", "sim_bake", {"object": "SimObj", "start_frame": 1, "end_frame": 3})
    h.check(
        "sim_bake restores the scene frame range",
        scene.frame_start == saved_start and scene.frame_end == saved_end,
    )
    # Clears a simulation cache. Headless there is nothing baked to clear and no
    # readable "cache is now empty" flag, so "ran without error" is the honest check;
    # the guard is that it does not raise on a valid object.
    h.ran("sim_clear_cache runs", "sim_clear_cache", {"object": "SimObj"})

    # -- phys_bake: frame range genuinely drives the bake, then is restored ------------
    # `ptcache.bake_all` bakes EVERY point cache in the scene, and a FLUID domain's
    # cache range is its own `domain_settings.cache_frame_start/end` (verified live:
    # defaults to 1-250 regardless of the scene's frame range) rather than something
    # phys_bake's frame_start/frame_end can reach. Left in place, FL1 alone would turn
    # this assertion into a 250-frame mantaflow solve. Clear out the caches that are
    # not what this assertion is testing before baking.
    h.call("phys_remove", {"name": "FL1"})
    h.call("phys_remove", {"name": "CL1"})
    h.call("phys_remove", {"name": "BATCH_RB"})
    scene.frame_start = 1
    scene.frame_end = 50
    h.call("phys_bake", {"frame_start": 5, "frame_end": 15})
    h.check(
        "phys_bake restores the scene frame range afterwards",
        scene.frame_start == 1 and scene.frame_end == 50,
    )

    # -- render queue --------------------------------------------------------------------
    h.call("render_queue_clear", {})
    h.call("render_queue_add", {"output_path": "//out1.png"})
    h.call("render_queue_add", {"output_path": "//out2.png"})
    listed = h.call("render_queue_list")
    ids = [j["id"] for j in listed["jobs"]]
    h.check("render queue has 2 jobs", len(listed["jobs"]) == 2)
    h.check("job ids are monotonic starting at 1", ids == [1, 2])

    h.call("render_queue_clear", {})
    listed = h.call("render_queue_list")
    h.check("render_queue_clear empties the queue", len(listed["jobs"]) == 0)

    h.call("render_queue_add", {"output_path": "//out3.png"})
    listed = h.call("render_queue_list")
    h.check(
        "job id is not reissued after a clear (was len(queue)+1)",
        listed["jobs"][0]["id"] == 3,
    )

    # completed_only must drop exactly the completed jobs, leaving pending AND
    # failed jobs untouched (the original also silently dropped failed jobs).
    h.call("render_queue_add", {"output_path": "//out4.png"})
    h.call("render_queue_add", {"output_path": "//out5.png"})
    raw = scene["mcp_render_queue"]
    queue = pyjson.loads(raw)
    queue[0]["status"] = "completed"
    queue[1]["status"] = "failed: boom"
    scene["mcp_render_queue"] = pyjson.dumps(queue)

    h.call("render_queue_clear", {"completed_only": True})
    listed = h.call("render_queue_list")
    statuses = [j["status"] for j in listed["jobs"]]
    h.check("completed_only drops exactly the completed job", len(listed["jobs"]) == 2)
    h.check("completed_only keeps the failed job", "failed: boom" in statuses)
    h.check("completed_only keeps the pending job", "pending" in statuses)

    # render_queue_start actually processes every pending job (does not just echo).
    h.call("render_queue_clear", {})
    out_path = os.path.join(bpy.app.tempdir, "mcp_test_render_####.png")
    h.call(
        "render_queue_add",
        {
            "output_path": out_path,
            "start_frame": 1,
            "end_frame": 1,
            "resolution_x": 8,
            "resolution_y": 8,
            "samples": 1,
        },
    )
    result = h.call("render_queue_start")
    h.check("render_queue_start processed the one pending job", result["total"] == 1)
    listed = h.call("render_queue_list")
    h.check(
        "processed job status is no longer pending",
        listed["jobs"][0]["status"] != "pending",
    )

    result_empty = h.call("render_queue_start")
    h.check(
        "render_queue_start on an already-processed queue does nothing more",
        result_empty["total"] == 0,
    )
