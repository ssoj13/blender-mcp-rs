#!/usr/bin/env python3
"""Exercise the tool registry against a real Blender scene.

Run through bootstrap:  python bootstrap.py t
Or directly:            blender --background --factory-startup \
                            --python python/blender_smoke.py -- <dir-with-blender_mcp.pyd>

Covers the three shapes most likely to break: single mode, batch mode (typed nested
structs), and the error contract (a failed single-target call must be an error, a
batch with one bad item must still succeed and carry the failure as data).

`scene_screenshot` is NOT here on purpose: it needs an OpenGL context and does not
run under --background at all. It has to be checked in the GUI.
"""
import json
import sys
import traceback

import bpy

# Blender passes script args after "--".
argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
if not argv:
    print("SMOKE_FAIL: no extension directory given")
    sys.exit(1)
sys.path.insert(0, argv[0])

import blender_mcp  # noqa: E402  (must come after sys.path is set)

ok = 0
fail = 0


def call(tool: str, params: dict | None = None, expect=None):
    """Call a tool; a raised exception is a failure unless `expect` says otherwise."""
    global ok, fail
    try:
        result = json.loads(blender_mcp.execute_tool(tool, json.dumps(params or {})))
    except Exception as e:  # noqa: BLE001 - any failure is a test failure
        print(f"SMOKE_FAIL {tool}: {type(e).__name__}: {e}")
        traceback.print_exc()
        fail += 1
        return None

    if expect and not expect(result):
        print(f"SMOKE_FAIL {tool}: unexpected {str(result)[:160]}")
        fail += 1
        return None

    print(f"SMOKE_OK   {tool}: {str(result)[:100]}")
    ok += 1
    return result


def call_expecting_error(tool: str, params: dict, why: str):
    """A failed call must raise. Returning a 'successful' result carrying an error
    key is exactly the anti-pattern the registry forbids."""
    global ok, fail
    try:
        result = blender_mcp.execute_tool(tool, json.dumps(params))
    except Exception:  # noqa: BLE001 - this is the expected outcome
        print(f"SMOKE_OK   {tool}: errors as it should ({why})")
        ok += 1
        return
    print(f"SMOKE_FAIL {tool}: should have failed ({why}), returned {result[:120]}")
    fail += 1


print(f"SMOKE_BLENDER: {bpy.app.version_string}")
print(f"SMOKE_TOOLS: {len(bpy.data.objects)} objects in the startup scene")

call("scene_get_info", {}, lambda r: r["objects"] >= 1)

# Batch mode: the port turned these into typed nested structs - the risky part.
call("obj_create_primitive", {"primitives": [
    {"type": "cube", "name": "B1", "location": [0, 0, 0]},
    {"type": "sphere", "name": "B2", "location": [3, 0, 0]},
]})
call("obj_list", {"type": "MESH"}, lambda r: isinstance(r, list) and len(r) >= 2)
call("obj_update", {"updates": [{"name": "B1", "location": [1, 1, 1], "scale": [2, 2, 2]}]})
call("obj_get", {"name": "B1"}, lambda r: r["name"] == "B1")
call("obj_duplicate", {"name": "B1", "new_name": "B1_copy", "offset": [0, 3, 0]})
call("obj_parent", {"operations": [{"parent": "B1", "children": ["B2"]}]})
call("obj_set_selection", {"names": ["B1"]})
call("obj_get_selection", {})

call("mat_create", {"name": "Steel", "color": [0.6, 0.6, 0.7, 1.0],
                    "metallic": 1.0, "roughness": 0.3})
call("mat_assign", {"material": "Steel", "objects": ["B1"]})
call("mat_list", {})
call("mat_get_object", {"name": "B1"})
call("mat_update", {"name": "Steel", "roughness": 0.9})

call("anim_set_frame", {"frame": 5})
call("anim_set_keyframe", {"name": "B1", "property": "location", "value": [0, 0, 5], "frame": 10})
# Blender 5 removed Action.fcurves; these two used to raise AttributeError.
call("anim_get_keyframes", {"name": "B1"})
call("anim_get_data", {"name": "B1"})

call("mod_add", {"name": "B1", "type": "SUBSURF", "properties": {"levels": 2}})
call("mod_list", {"name": "B1"}, lambda r: isinstance(r, list) and len(r) >= 1)

call("col_create", {"name": "Props"})
call("col_move_to", {"objects": ["B2"], "collection": "Props"})
call("col_list", {})

call("util_execute_python", {"code": "result = len(bpy.data.objects)"})
call("util_set_mode", {"mode": "OBJECT"})

# --- the error contract ---
call_expecting_error("obj_get", {"name": "NoSuchObject"}, "object does not exist")
call_expecting_error("no_such_tool", {}, "unknown tool name")

# A batch where one item fails must still SUCCEED - the failure is per-item data.
batch = call("obj_delete", {"names": ["B1_copy", "Ghost"]},
             lambda r: r.get("deleted") == 1
             and any("error" in item for item in r.get("results", [])))
if batch:
    print("SMOKE_OK   batch keeps per-item failures as data")

# --- the unified node layer ---------------------------------------------------
# Discover, do not guess: Blender 5 renamed most socket names, and a model working
# from memory reliably gets them wrong.
call("node_types", {"kind": "shader", "search": "noise"},
     lambda r: r["matched"] >= 2 and r["types"])
call("node_schema", {"type": "ShaderNodeTexNoise"},
     lambda r: [o["name"] for o in r["outputs"]] == ["Factor", "Color"])
# Principled's sockets: the old names are GONE in Blender 5.
call("node_schema", {"type": "ShaderNodeBsdfPrincipled"},
     lambda r: "Specular IOR Level" in [i["name"] for i in r["inputs"]]
     and "Specular" not in [i["name"] for i in r["inputs"]])
# Sockets follow the node's config: Mix only shows Color sockets when data_type is RGBA.
call("node_schema", {"type": "ShaderNodeMix", "props": {"data_type": "RGBA"}},
     lambda r: [i["type"] for i in r["inputs"]] == ["VALUE", "RGBA", "RGBA"])

bpy.data.materials.new("SmokeRust")
call("node_graph_build", {
    "tree": "material:SmokeRust", "clear": True,
    "nodes": [
        {"id": "coord", "type": "ShaderNodeTexCoord"},
        {"id": "noise", "type": "ShaderNodeTexNoise", "inputs": {"Scale": 12.0}},
        {"id": "ramp", "type": "ShaderNodeValToRGB"},
        {"id": "bsdf", "type": "ShaderNodeBsdfPrincipled", "inputs": {"Metallic": 1.0}},
        {"id": "out", "type": "ShaderNodeOutputMaterial"},
    ],
    "links": [["coord.Object", "noise.Vector"], ["noise.Factor", "ramp.Factor"],
              ["ramp.Color", "bsdf.Base Color"], ["bsdf.BSDF", "out.Surface"]],
}, lambda r: r.get("warnings") == [])
call("node_graph_get", {"tree": "material:SmokeRust"},
     lambda r: len(r["nodes"]) == 5 and len(r["links"]) == 4
     # auto-layout must spread the nodes, not pile them at the origin
     and len({tuple(n["location"]) for n in r["nodes"]}) == 5)

call_expecting_error("node_graph_build",
                     {"tree": "material:SmokeRust",
                      "nodes": [{"id": "x", "type": "ShaderNodeNotAThing"}]},
                     "unknown node type")
call_expecting_error("node_graph_build",
                     {"tree": "material:SmokeRust",
                      "nodes": [{"id": "b", "type": "ShaderNodeBsdfPrincipled",
                                 "inputs": {"Specular": 0.5}}]},
                     "socket 'Specular' no longer exists in Blender 5")
call_expecting_error("node_graph_get", {"tree": "material:NoSuchMaterial"},
                     "material does not exist")

# mat_preview closes the loop: build -> look -> adjust. Renders in its own throwaway
# scene, so the user's scene is untouched.
scenes_before = len(bpy.data.scenes)
r = call("mat_preview", {"material": "SmokeRust", "shape": "sphere", "resolution": 128},
         lambda r: r.get("__image__") is True)
if r:
    import base64
    png = base64.b64decode(r["data"])
    if png[:8] != bytes([137, 80, 78, 71, 13, 10, 26, 10]):
        print("SMOKE_FAIL mat_preview: not a real PNG")
        fail += 1
    elif len(bpy.data.scenes) != scenes_before:
        print("SMOKE_FAIL mat_preview: left a scene behind")
        fail += 1
    else:
        print(f"SMOKE_OK   mat_preview: {len(png)} byte PNG, scene untouched")
        ok += 1

# --- pointers: a datablock is referred to by NAME ------------------------------
# Image Texture is the most-used shader node after Principled, and its `image` is a
# POINTER, not a string: node.image = "brick" raises. Same for an Object socket.
call("img_new", {"name": "SmokeTex", "width": 64, "height": 64},
     lambda r: r["size"] == [64, 64])
call("img_list", {}, lambda r: any(i["name"] == "SmokeTex" for i in r))

bpy.data.materials.new("SmokeTexMat")
call("node_graph_build", {
    "tree": "material:SmokeTexMat", "clear": True,
    "nodes": [
        {"id": "tex", "type": "ShaderNodeTexImage", "props": {"image": "SmokeTex"}},
        {"id": "bsdf", "type": "ShaderNodeBsdfPrincipled"},
        {"id": "out", "type": "ShaderNodeOutputMaterial"},
    ],
    "links": [["tex.Color", "bsdf.Base Color"], ["bsdf.BSDF", "out.Surface"]],
})
if bpy.data.materials["SmokeTexMat"].node_tree.nodes["tex"].image.name == "SmokeTex":
    print("SMOKE_OK   image really assigned to the node")
    ok += 1
else:
    print("SMOKE_FAIL image not assigned")
    fail += 1
call_expecting_error("node_graph_edit",
                     {"tree": "material:SmokeTexMat",
                      "set_props": [{"node": "tex", "props": {"image": "NoSuchImage"}}]},
                     "image does not exist")

# A pointer in a SOCKET (Object Info takes an Object, not a string).
bpy.ops.mesh.primitive_cube_add()
bpy.context.object.name = "SmokeSrc"
bpy.data.node_groups.new("SmokeG", "GeometryNodeTree")
call("node_graph_build", {"tree": "group:SmokeG", "clear": True, "nodes": [
    {"id": "oi", "type": "GeometryNodeObjectInfo", "inputs": {"Object": "SmokeSrc"}}]})
if bpy.data.node_groups["SmokeG"].nodes["oi"].inputs["Object"].default_value.name == "SmokeSrc":
    print("SMOKE_OK   object really assigned to the socket")
    ok += 1
else:
    print("SMOKE_FAIL object not assigned")
    fail += 1

# --- zones are node PAIRS -----------------------------------------------------
# A Simulation/Repeat/For-Each input has NO geometry sockets until it is paired with
# its output, so an unpaired zone silently cannot be wired at all.
bpy.data.node_groups.new("SmokeZ", "GeometryNodeTree")
call("node_graph_build", {"tree": "group:SmokeZ", "clear": True, "nodes": [
    {"id": "si", "type": "GeometryNodeSimulationInput"},
    {"id": "so", "type": "GeometryNodeSimulationOutput"}]})
sim_sockets = [s.name for s in bpy.data.node_groups["SmokeZ"].nodes["si"].outputs]
if "Geometry" in sim_sockets:
    print("SMOKE_OK   simulation zone auto-paired (has a Geometry socket)")
    ok += 1
else:
    print(f"SMOKE_FAIL zone not paired: {sim_sockets}")
    fail += 1
call_expecting_error("node_graph_build",
                     {"tree": "group:SmokeZ", "clear": True,
                      "nodes": [{"id": "si", "type": "GeometryNodeSimulationInput"}]},
                     "zone input with no matching output")

# --- scene / render / colour management ---------------------------------------
# These were READ-ONLY. A model could key 200 frames and the scene still played 1-250
# at the wrong fps; the view transform silently changed the look of every render.
call("scene_set", {"frame_start": 1, "frame_end": 200, "fps": 30})
if bpy.context.scene.frame_end == 200 and bpy.context.scene.render.fps == 30:
    print("SMOKE_OK   scene frame range and fps really persist")
    ok += 1
else:
    print("SMOKE_FAIL scene settings did not persist")
    fail += 1
call_expecting_error("scene_set", {"frame_start": 100, "frame_end": 10},
                     "frame_end before frame_start")

# The engine enum is DYNAMIC: reading bl_rna.enum_items hides Cycles entirely, and code
# that trusted it rejected every Cycles render.
call("render_set", {"engine": "CYCLES", "film_transparent": True})
if bpy.context.scene.render.engine == "CYCLES":
    print("SMOKE_OK   CYCLES accepted (the dynamic-enum trap)")
    ok += 1
else:
    print(f"SMOKE_FAIL engine is {bpy.context.scene.render.engine}")
    fail += 1
call("render_set", {"engine": "EEVEE"})
call_expecting_error("render_set", {"engine": "NoSuchEngine"}, "unknown engine")

# --- vertex groups: the payoff is that a MASK modifier can finally use one ---------
bpy.ops.mesh.primitive_cube_add()
bpy.context.object.name = "SmokeVG"
call("vgroup_add", {"object": "SmokeVG", "name": "Half"})
call("vgroup_set", {"object": "SmokeVG", "name": "Half",
                    "indices": [0, 1, 2, 3], "weight": 1.0})
call("vgroup_get", {"object": "SmokeVG", "name": "Half"},
     lambda r: len(r.get("weights", r.get("vertices", []))) == 4)

# --- mesh topology: this is what forced a model into raw Python before -------------
call("mesh_create", {"name": "SmokePyramid",
                     "verts": [[-1, -1, 0], [1, -1, 0], [1, 1, 0], [-1, 1, 0], [0, 0, 2]],
                     "faces": [[0, 1, 2, 3], [0, 1, 4], [1, 2, 4], [2, 3, 4], [3, 0, 4]]},
     lambda r: r.get("verts") == 5 or r.get("vertices") == 5)
if "SmokePyramid" in bpy.data.objects and len(bpy.data.objects["SmokePyramid"].data.polygons) == 5:
    print("SMOKE_OK   mesh built from raw verts/faces")
    ok += 1
else:
    print("SMOKE_FAIL pyramid not built")
    fail += 1
call_expecting_error("mesh_create", {"name": "Bad", "verts": [[0, 0, 0]],
                                     "faces": [[0, 7, 9]]},
                     "face index out of range")

# --- objects: every sphere used to render faceted, with no way to fix it -----------
call("obj_create_primitive", {"type": "sphere", "name": "SmokeBall"})
call("obj_shade", {"name": "SmokeBall", "mode": "SMOOTH"})
if all(p.use_smooth for p in bpy.data.objects["SmokeBall"].data.polygons):
    print("SMOKE_OK   sphere really is smooth-shaded")
    ok += 1
else:
    print("SMOKE_FAIL still faceted")
    fail += 1

# Un-applied scale silently corrupts Bevel/Solidify/Array widths.
bpy.data.objects["SmokeBall"].scale = (2.0, 2.0, 2.0)
call("obj_apply_transform", {"name": "SmokeBall", "scale": True})
if tuple(round(v, 4) for v in bpy.data.objects["SmokeBall"].scale) == (1.0, 1.0, 1.0):
    print("SMOKE_OK   scale really applied")
    ok += 1
else:
    print(f"SMOKE_FAIL scale is {tuple(bpy.data.objects['SmokeBall'].scale)}")
    fail += 1

# --- custom properties: a rig control IS a custom property on a bone ---------------
call("prop_set", {"target": "objects:SmokeBall", "props": {"hp": 100}, "min": 0, "max": 200})
if bpy.data.objects["SmokeBall"].get("hp") == 100:
    print("SMOKE_OK   custom property really set")
    ok += 1
else:
    print("SMOKE_FAIL custom property missing")
    fail += 1

# --- UV: a mesh could only ever have the one layer Blender made for it -------------
call("uv_add", {"object": "SmokeVG", "name": "Lightmap"})
if "Lightmap" in [l.name for l in bpy.data.objects["SmokeVG"].data.uv_layers]:
    print("SMOKE_OK   second UV map added")
    ok += 1
else:
    print("SMOKE_FAIL no second UV map")
    fail += 1

print(f"SMOKE_TOTAL ok={ok} fail={fail}")
sys.exit(1 if fail else 0)
