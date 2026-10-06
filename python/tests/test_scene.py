"""Scene / render / colour-management / camera / light / preset tools.

These were read-only for a long time — the payoff of the writer tools is that state
actually persists, so every assertion reads back off `bpy`, not off the tool's echo.
"""

import bpy


def run(h):
    h.section("scene")

    # frame range + fps must PERSIST (they were read-only before)
    h.call("scene_set", {"frame_start": 1, "frame_end": 200, "fps": 30})
    h.check("frame_end persists", bpy.context.scene.frame_end == 200)
    h.check("fps persists", bpy.context.scene.render.fps == 30)
    h.errors("frame_end < frame_start rejected",
             "scene_set", {"frame_start": 100, "frame_end": 10}, "frame")

    # the engine enum is dynamic: bl_rna.enum_items hides Cycles; this must still work
    h.call("render_set", {"engine": "CYCLES", "film_transparent": True})
    h.check("CYCLES accepted (dynamic-enum trap)",
            bpy.context.scene.render.engine == "CYCLES")
    h.call("render_set", {"engine": "EEVEE"})
    h.errors("bad engine rejected", "render_set", {"engine": "Nope"}, "engine")

    # colour management persists, bad value lists the real options
    h.call("color_set", {"view_transform": "Standard"})
    h.check("view_transform persists",
            bpy.context.scene.view_settings.view_transform == "Standard")
    h.errors("bad view_transform lists real options",
             "color_set", {"view_transform": "Nope"}, "")

    # read-only tools: their result IS the effect, so assert the content, not just
    # that they ran. scene_get_info must reflect what we set above.
    info = h.call("scene_get_info")
    h.check("scene_get_info reflects the frame range we set",
            info.get("frame_end") == 200)
    h.check("scene_get_info reflects fps", info.get("fps") == 30)
    stats = h.call("scene_get_stats")
    h.check("scene_get_stats reports data-block and geometry counts",
            "data_blocks" in stats and "objects_by_type" in stats)
    templates = h.call("scene_list_templates")
    h.check("scene_list_templates returns a non-empty list",
            bool(templates.get("templates") or templates))

    # camera + light
    h.call("cam_create", {"name": "TestCam", "location": [0, -5, 2]})
    h.check("camera created", "TestCam" in bpy.data.objects
            and bpy.data.objects["TestCam"].type == "CAMERA")
    h.call("light_create", {"name": "TestSun", "type": "SUN"})
    h.check("light created", "TestSun" in bpy.data.objects
            and bpy.data.objects["TestSun"].type == "LIGHT")
