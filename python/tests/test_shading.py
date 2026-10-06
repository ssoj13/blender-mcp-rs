"""Materials, images, and the unified node-graph layer (material.rs, image.rs,
nodes/*.rs).

Every assertion reads ground truth off `bpy` directly — material node inputs,
`obj.data.polygons[i].material_index`, `tree.nodes`/`tree.links`, datablock counts —
never off a tool's own echoed JSON. A broken tool can still hand back a plausible dict;
`bpy` cannot.

Socket and property names below (e.g. Principled's "Specular IOR Level", Noise's
"Factor" output, Mix's per-data_type "A") were confirmed against a live Blender 5.0.0,
not guessed — see the project rule in src/tools/nodes/AGENTS.md.
"""

import base64
import tempfile
from pathlib import Path

import bpy

import harness


def run(h):
    h.section("shading")
    _materials(h)
    _material_slots(h)
    _material_settings(h)
    _images(h)
    _node_intro(h)
    _node_graph(h)
    _geometry_zones(h)
    _node_groups_and_geo(h)
    _mat_preview(h)


# ============================================================================
# material.rs
# ============================================================================

def _materials(h):
    h.call("mat_create", {"name": "Rust", "color": [0.6, 0.1, 0.05, 1.0],
                           "metallic": 0.9, "roughness": 0.3})
    bsdf = bpy.data.materials["Rust"].node_tree.nodes["Principled BSDF"]
    h.check("mat_create sets base color",
            tuple(round(c, 3) for c in bsdf.inputs["Base Color"].default_value) == (0.6, 0.1, 0.05, 1.0))
    h.check("mat_create sets metallic", round(bsdf.inputs["Metallic"].default_value, 3) == 0.9)
    h.check("mat_create sets roughness", round(bsdf.inputs["Roughness"].default_value, 3) == 0.3)

    h.call("mat_create", {"materials": [{"name": "BatchA"}, {"name": "BatchB", "color": [1, 0, 0]}]})
    h.check("mat_create batch made both", "BatchA" in bpy.data.materials and "BatchB" in bpy.data.materials)
    bb_bsdf = bpy.data.materials["BatchB"].node_tree.nodes["Principled BSDF"]
    h.check("batch color applied", tuple(bb_bsdf.inputs["Base Color"].default_value) == (1.0, 0.0, 0.0, 1.0))
    h.errors("mat_create needs a name outside batch mode", "mat_create", {}, "name")

    listed = h.call("mat_list")
    h.check("mat_list finds Rust with its BSDF properties",
            any(m["name"] == "Rust" and m.get("base_color") for m in listed))

    h.call("obj_create_primitive", {"type": "cube", "name": "AssignObj"})
    h.call("mat_assign", {"object": "AssignObj", "material": "Rust"})
    h.check("mat_assign fills slot 0", bpy.data.objects["AssignObj"].data.materials[0].name == "Rust")

    h.call("obj_create_primitive", {"type": "cube", "name": "AssignObj2"})
    h.call("mat_assign", {"objects": ["AssignObj", "AssignObj2"], "material": "BatchA"})
    h.check("mat_assign batch reassigns both",
            bpy.data.objects["AssignObj"].data.materials[0].name == "BatchA"
            and bpy.data.objects["AssignObj2"].data.materials[0].name == "BatchA")
    h.errors("mat_assign unknown material", "mat_assign", {"object": "AssignObj", "material": "Nope"}, "not found")
    h.errors("mat_assign unknown object", "mat_assign", {"object": "NoSuchObj", "material": "Rust"}, "not found")

    h.call("mat_update", {
        "name": "Rust", "color": [0.2, 0.3, 0.4], "metallic": 0.5, "roughness": 0.6,
        "emission": [1.0, 0.5, 0.0], "emission_strength": 2.0, "alpha": 0.4,
        "ior": 1.6, "transmission": 0.7,
    })
    bsdf = bpy.data.materials["Rust"].node_tree.nodes["Principled BSDF"]
    inp = bsdf.inputs
    h.check("mat_update base color (3-comp keeps alpha at 1)",
            tuple(round(c, 3) for c in inp["Base Color"].default_value) == (0.2, 0.3, 0.4, 1.0))
    h.check("mat_update metallic", round(inp["Metallic"].default_value, 3) == 0.5)
    h.check("mat_update roughness", round(inp["Roughness"].default_value, 3) == 0.6)
    h.check("mat_update emission color",
            tuple(round(c, 3) for c in inp["Emission Color"].default_value)[:3] == (1.0, 0.5, 0.0))
    h.check("mat_update emission strength", round(inp["Emission Strength"].default_value, 3) == 2.0)
    h.check("mat_update alpha", round(inp["Alpha"].default_value, 3) == 0.4)
    h.check("mat_update alpha < 1 flips blend_method to BLEND",
            bpy.data.materials["Rust"].blend_method == "BLEND")
    h.check("mat_update ior", round(inp["IOR"].default_value, 3) == 1.6)
    h.check("mat_update transmission", round(inp["Transmission Weight"].default_value, 3) == 0.7)
    h.errors("mat_update unknown material", "mat_update", {"name": "Nope"}, "not found")
    h.errors("mat_update bad color length", "mat_update", {"name": "Rust", "color": [1, 0]}, "color")

    got = h.call("mat_get_object", {"name": "AssignObj"})
    h.check("mat_get_object reports slot 0's material", got["materials"][0]["name"] == "BatchA")
    h.errors("mat_get_object unknown object", "mat_get_object", {"name": "Nope"}, "not found")

    by = h.call("mat_get_objects_by", {"materials": ["BatchA"]})
    names = {o["object"] for o in by["materials"]["BatchA"]}
    h.check("mat_get_objects_by finds both assignees", names == {"AssignObj", "AssignObj2"})
    h.errors("mat_get_objects_by unknown material", "mat_get_objects_by", {"materials": ["Nope"]}, "not found")

    h.call("mat_create", {"name": "ToDelete"})
    h.call("mat_delete", {"name": "ToDelete"})
    h.check("mat_delete removes it", "ToDelete" not in bpy.data.materials)
    h.call("mat_create", {"materials": [{"name": "Del1"}, {"name": "Del2"}]})
    h.call("mat_delete", {"names": ["Del1", "Del2"]})
    h.check("mat_delete batch removes both", "Del1" not in bpy.data.materials and "Del2" not in bpy.data.materials)
    h.errors("mat_delete unknown material", "mat_delete", {"name": "NeverExisted"}, "not found")


def _material_slots(h):
    h.call("obj_create_primitive", {"type": "cube", "name": "SlotCube"})
    h.call("mat_create", {"name": "SlotMatA"})
    h.call("mat_create", {"name": "SlotMatB"})
    h.call("mat_assign", {"object": "SlotCube", "material": "SlotMatA"})

    slots = h.call("mat_slots", {"object": "SlotCube"})
    h.check("mat_slots sees slot 0 = SlotMatA", slots["slots"][0]["material"] == "SlotMatA")

    added = h.call("mat_slot_add", {"object": "SlotCube", "material": "SlotMatB"})
    h.check("mat_slot_add appends at index 1", added["index"] == 1)
    obj = bpy.data.objects["SlotCube"]
    h.check("cube now carries 2 material slots", len(obj.data.materials) == 2)

    h.call("mat_assign_faces", {"object": "SlotCube", "slot_index": 0, "face_start": 0, "face_end": 2})
    h.call("mat_assign_faces", {"object": "SlotCube", "slot_index": 1, "face_start": 3, "face_end": 5})
    polys = obj.data.polygons
    h.check("faces 0-2 painted with slot 0", all(polys[i].material_index == 0 for i in range(3)))
    h.check("faces 3-5 painted with slot 1", all(polys[i].material_index == 1 for i in range(3, 6)))
    h.check("the cube really ends up with TWO distinct materials in use",
            {obj.data.materials[polys[i].material_index].name for i in range(6)} == {"SlotMatA", "SlotMatB"})

    h.errors("mat_assign_faces slot_index out of range", "mat_assign_faces",
             {"object": "SlotCube", "slot_index": 9, "face_start": 0, "face_end": 1}, "out of range")
    h.errors("mat_assign_faces face index out of range", "mat_assign_faces",
             {"object": "SlotCube", "slot_index": 0, "face_start": 0, "face_end": 99}, "out of range")
    h.errors("mat_assign_faces needs face_indices or a range", "mat_assign_faces",
             {"object": "SlotCube", "slot_index": 0}, "face_indices")

    h.call("mat_slot_set", {"object": "SlotCube", "index": 1, "material": None})
    h.check("mat_slot_set clears a slot", obj.material_slots[1].material is None)
    h.check("clearing a slot's material does not disturb face painting",
            all(polys[i].material_index == 1 for i in range(3, 6)))
    h.call("mat_slot_set", {"object": "SlotCube", "index": 1, "material": "SlotMatB"})
    h.check("mat_slot_set restores it", obj.material_slots[1].material.name == "SlotMatB")
    h.errors("mat_slot_set index out of range", "mat_slot_set",
             {"object": "SlotCube", "index": 9, "material": "SlotMatB"}, "out of range")
    h.errors("mat_slot_set unknown material", "mat_slot_set",
             {"object": "SlotCube", "index": 1, "material": "Nope"}, "not found")

    removed = h.call("mat_slot_remove", {"object": "SlotCube", "index": 1})
    h.check("mat_slot_remove reports what it removed", removed["removed"] == "SlotMatB")
    h.check("cube is back to 1 material slot", len(obj.data.materials) == 1)
    h.errors("mat_slot_remove index out of range", "mat_slot_remove", {"object": "SlotCube", "index": 9}, "out of range")

    h.call("cam_create", {"name": "NoSlotsCam"})
    h.errors("mat_slots on an object with no material slots", "mat_slots", {"object": "NoSlotsCam"}, "no material slots")
    h.errors("mat_slot_add on an object with no material slots", "mat_slot_add",
             {"object": "NoSlotsCam", "material": "SlotMatA"}, "no material slots")


def _material_settings(h):
    h.call("mat_create", {"name": "SettingsMat"})
    h.call("mat_settings", {"name": "SettingsMat", "blend_method": "blend",
                             "backface_culling": True, "displacement_method": "both"})
    mat = bpy.data.materials["SettingsMat"]
    h.check("mat_settings blend_method persists", mat.blend_method == "BLEND")
    h.check("mat_settings backface_culling persists", mat.use_backface_culling is True)
    h.check("mat_settings displacement_method persists", mat.displacement_method == "BOTH")
    h.errors("mat_settings bad blend_method lists this Blender's real values", "mat_settings",
             {"name": "SettingsMat", "blend_method": "NOPE"}, "OPAQUE")
    # Blender 5's EEVEE Next dropped Material.shadow_method outright (verified live
    # on 5.2.2), so mat_settings no longer advertises the parameter at all. Guard the
    # premise: if a future Blender brings the property back, this fails and the
    # parameter should return with it.
    h.check("Material.shadow_method is really gone from this Blender",
            "shadow_method" not in mat.bl_rna.properties)
    h.errors("mat_settings unknown material", "mat_settings", {"name": "Nope"}, "not found")

    resp = h.call("mat_settings", {"name": "SettingsMat", "use_nodes": False})
    h.check("mat_settings re-reads use_nodes instead of echoing the request "
            "(Blender 5 made the setter a no-op)",
            resp["updated"]["use_nodes"] == bpy.data.materials["SettingsMat"].use_nodes)


# ============================================================================
# image.rs
# ============================================================================

def _images(h):
    created = h.call("img_new", {"name": "Brick", "width": 32, "height": 16, "color": [0.5, 0.25, 0.1, 1.0]})
    img = bpy.data.images["Brick"]
    h.check("img_new size persists", tuple(img.size) == (32, 16))
    # `alpha` isn't a queryable flag on Image; a 32-bit depth (vs 24) is what an
    # alpha-enabled image actually looks like from bpy's side.
    h.check("img_new alpha defaults true (32-bit depth, not 24)", img.depth == 32)
    h.check("img_new response matches bpy", created["size"] == [32, 16])

    listed = h.call("img_list")
    h.check("img_list finds Brick", any(i["name"] == "Brick" for i in listed))

    tmp_dir = Path(tempfile.mkdtemp(prefix="mcp_shading_"))
    save_path = tmp_dir / "brick.png"
    h.call("img_save", {"name": "Brick", "path": str(save_path)})
    h.check("img_save writes a real PNG to disk",
            save_path.exists() and save_path.read_bytes()[:8] == b"\x89PNG\r\n\x1a\n")
    h.errors("img_save unknown image", "img_save", {"name": "Nope"}, "not found")

    h.call("img_new", {"name": "NoPath"})
    h.errors("img_save with no path and no filepath of its own", "img_save", {"name": "NoPath"}, "path")

    h.call("img_load", {"path": str(save_path), "name": "BrickReloaded", "colorspace": "Non-Color"})
    reimg = bpy.data.images["BrickReloaded"]
    h.check("img_load reads real pixels back", reimg.has_data)
    h.check("img_load size matches the file on disk", tuple(reimg.size) == (32, 16))
    h.check("img_load colorspace persists", reimg.colorspace_settings.name == "Non-Color")
    h.errors("img_load missing file", "img_load", {"path": str(tmp_dir / "does_not_exist.png")}, "cannot load")

    bogus = tmp_dir / "not_an_image.txt"
    bogus.write_text("hello")
    h.errors("img_load a file Blender cannot decode as an image", "img_load", {"path": str(bogus)})


# ============================================================================
# nodes/introspect.rs
# ============================================================================

def _node_intro(h):
    all_types = h.call("node_types", {})
    # The registered-class count moves with the Blender version (506 on 5.0, 570 on
    # 5.2), so count it live rather than hardcoding: nearly all of them must be
    # creatable, and a handful are registered-but-dead (e.g. shader Musgrave).
    import bpy.types as _t
    registered = sum(1 for _n in dir(_t)
                     if isinstance(getattr(_t, _n), type)
                     and issubclass(getattr(_t, _n), _t.Node)
                     and getattr(_t, _n) is not _t.Node)
    h.check("node_types finds most of the %d registered node types creatable "
            "(a handful are registered-but-dead, e.g. shader Musgrave)" % registered,
            0.9 * registered <= all_types["matched"] <= registered)
    h.check("node_types never silently truncates: `returned` says the real cap",
            all_types["returned"] == min(60, all_types["matched"]))

    limited = h.call("node_types", {"limit": 5})
    h.check("node_types respects an explicit limit", limited["returned"] == 5 and limited["matched"] > 5)

    shader_types = h.call("node_types", {"kind": "shader"})
    h.check("kind=shader only returns shader nodes",
            len(shader_types["types"]) > 0 and all(t["kind"] == "shader" for t in shader_types["types"]))
    h.errors("node_types unknown kind", "node_types", {"kind": "nope"}, "unknown node kind")

    search = h.call("node_types", {"search": "noise"})
    h.check("search matches by substring in type or label",
            any("noise" in t["type"].lower() or "noise" in t["label"].lower() for t in search["types"]))

    schema = h.call("node_schema", {"type": "ShaderNodeBsdfPrincipled"})
    names = {s["name"] for s in schema["inputs"]}
    h.check("Blender 5 renamed Specular -> Specular IOR Level",
            "Specular IOR Level" in names and "Specular" not in names)
    h.check("Emission split into Emission Color + Emission Strength",
            {"Emission Color", "Emission Strength"} <= names)
    h.check("Transmission -> Transmission Weight",
            "Transmission Weight" in names and "Transmission" not in names)
    h.errors("node_schema unknown type", "node_schema", {"type": "NotARealNode"}, "unknown node type")
    h.errors("node_schema bad prop enum lists this Blender's real values", "node_schema",
             {"type": "ShaderNodeMix", "props": {"data_type": "BOGUS"}}, "FLOAT")

    rgba_schema = h.call("node_schema", {"type": "ShaderNodeMix", "props": {"data_type": "RGBA"}})
    float_schema = h.call("node_schema", {"type": "ShaderNodeMix", "props": {"data_type": "FLOAT"}})
    rgba_a = next(s for s in rgba_schema["inputs"] if s["name"] == "A")
    float_a = next(s for s in float_schema["inputs"] if s["name"] == "A")
    h.check("Mix's `A` socket is RGBA-typed when data_type=RGBA", rgba_a["type"] == "RGBA")
    h.check("the same `A` socket is FLOAT-typed when data_type=FLOAT", float_a["type"] == "VALUE")


# ============================================================================
# nodes/graph.rs
# ============================================================================

def _node_graph(h):
    created = h.call("node_tree_create", {"kind": "material", "name": "ShaderMat"})
    h.check("node_tree_create(material) address", created["address"] == "material:ShaderMat")
    h.check("the material really has a shader tree", bpy.data.materials["ShaderMat"].node_tree is not None)

    h.call("node_tree_create", {"kind": "group", "name": "PlainGroup"})
    h.check("node_tree_create(group) lands in bpy.data.node_groups", "PlainGroup" in bpy.data.node_groups)
    h.check("group defaults to ShaderNodeTree", bpy.data.node_groups["PlainGroup"].bl_idname == "ShaderNodeTree")

    h.call("obj_create_primitive", {"type": "cube", "name": "GeoObj"})
    geo = h.call("node_tree_create", {"kind": "geometry_modifier", "name": "GeoMod", "object": "GeoObj"})
    h.check("node_tree_create(geometry_modifier) address", geo["address"] == "object:GeoObj/GeoMod")
    gm = bpy.data.objects["GeoObj"].modifiers["GeoMod"]
    h.check("geometry_modifier got a real node_group assigned", gm.node_group is not None)
    h.check("geometry_modifier ships Group Input -> Group Output already wired",
            len(gm.node_group.nodes) == 2 and len(gm.node_group.links) == 1)
    h.errors("node_tree_create(geometry_modifier) requires `object`", "node_tree_create",
             {"kind": "geometry_modifier", "name": "NoObj"}, "requires")
    h.errors("node_tree_create unknown kind", "node_tree_create", {"kind": "nope", "name": "X"}, "unknown kind")

    tree_list = h.call("node_tree_list")
    addrs = {t["address"] for t in tree_list["trees"]}
    h.check("node_tree_list sees the material, group and geometry_modifier trees",
            {"material:ShaderMat", "group:PlainGroup", "object:GeoObj/GeoMod"} <= addrs)

    # Blender 5: scene.node_tree is gone. resolve_tree("compositor") must create
    # scene.compositing_node_group on first touch rather than raise.
    h.call("node_graph_get", {"tree": "compositor"})
    h.check("compositor tree materialized on scene.compositing_node_group",
            bpy.context.scene.compositing_node_group is not None)
    comp_addrs = {t["address"] for t in h.call("node_tree_list")["trees"]}
    real_comp_name = bpy.context.scene.compositing_node_group.name
    h.check("node_tree_list reports it once as `compositor`, not also as `group:<name>`",
            "compositor" in comp_addrs and f"group:{real_comp_name}" not in comp_addrs)

    world_info = h.call("node_graph_get", {"tree": "world"})
    h.check("the world tree resolves too (every factory scene ships a World)",
            world_info["kind"] == "ShaderNodeTree")

    _build_procedural_shader(h)
    _node_graph_guards(h)
    _node_graph_edit(h)


def _build_procedural_shader(h):
    """tex coord -> noise -> ramp -> principled -> output, in one node_graph_build call."""
    result = h.call("node_graph_build", {
        "tree": "material:ShaderMat",
        "clear": True,
        "nodes": [
            {"id": "TexCoord", "type": "ShaderNodeTexCoord"},
            {"id": "Noise", "type": "ShaderNodeTexNoise", "inputs": {"Scale": 8.0}},
            {"id": "Ramp", "type": "ShaderNodeValToRGB"},
            {"id": "BSDF", "type": "ShaderNodeBsdfPrincipled",
             "inputs": {"Base Color": [1.0, 0.0, 0.0], "Roughness": 0.4}},
            {"id": "Output", "type": "ShaderNodeOutputMaterial"},
        ],
        "links": [
            ["TexCoord.Generated", "Noise.Vector"],
            ["Noise.Factor", "Ramp.Factor"],
            ["Ramp.Color", "BSDF.Base Color"],
            ["BSDF.BSDF", "Output.Surface"],
        ],
    })
    h.check("node_graph_build wired every requested link", result["links"] == 4)
    h.check("a fully-connected build has no dangling-node warnings", result["warnings"] == [])

    tree = bpy.data.materials["ShaderMat"].node_tree
    node_ids = {"TexCoord", "Noise", "Ramp", "BSDF", "Output"}
    names = {n.name for n in tree.nodes}
    h.check("every requested node landed under its id as the real Blender name", node_ids <= names)
    h.check("node_graph_build actually wired the links in bpy", len(tree.links) == 4)

    locs = {n.name: tuple(n.location) for n in tree.nodes if n.name in node_ids}
    h.check("auto-layout gives every node a distinct location, none piled on top of another",
            len(set(locs.values())) == len(locs))

    h.check("Noise.Scale input value applied", round(tree.nodes["Noise"].inputs["Scale"].default_value, 3) == 8.0)
    bc = tree.nodes["BSDF"].inputs["Base Color"].default_value
    h.check("BSDF Base Color applied, RGB padded with alpha=1", tuple(round(c, 3) for c in bc) == (1.0, 0.0, 0.0, 1.0))

    view = h.call("node_graph_get", {"tree": "material:ShaderMat"})
    h.check("node_graph_get's node count matches bpy, not its own cached echo",
            len(view["nodes"]) == len(tree.nodes))
    h.check("node_graph_get's link count matches bpy", len(view["links"]) == len(tree.links))

    # Pointer values: an Image Texture's `image` prop takes a datablock NAME.
    h.call("node_graph_build", {
        "tree": "material:ShaderMat",
        "nodes": [{"id": "TexImg", "type": "ShaderNodeTexImage", "props": {"image": "Brick"}}],
    })
    h.check("image pointer resolved by name to the real datablock",
            tree.nodes["TexImg"].image is not None and tree.nodes["TexImg"].image.name == "Brick")
    h.errors("unknown image name lists what this file actually has loaded", "node_graph_build", {
        "tree": "material:ShaderMat",
        "nodes": [{"id": "BadImg", "type": "ShaderNodeTexImage", "props": {"image": "NoSuchImage"}}],
    }, "Brick")


def _node_graph_guards(h):
    h.errors("node_graph_build unknown node type", "node_graph_build", {
        "tree": "material:ShaderMat", "nodes": [{"id": "X", "type": "ShaderNodeDoesNotExist"}],
    }, "unknown node type")

    h.errors("node_graph_build unknown input socket lists the node's real ones", "node_graph_build", {
        "tree": "material:ShaderMat",
        "nodes": [{"id": "P2", "type": "ShaderNodeBsdfPrincipled", "inputs": {"Specular": 1.0}}],
    }, "Specular IOR Level")

    h.errors("node_graph_build duplicate id in one request", "node_graph_build", {
        "tree": "material:ShaderMat",
        "nodes": [{"id": "Dup", "type": "ShaderNodeValue"}, {"id": "Dup", "type": "ShaderNodeValue"}],
    }, "duplicate")

    h.errors("node_graph_build link to an id that doesn't exist anywhere", "node_graph_build", {
        "tree": "material:ShaderMat",
        "nodes": [{"id": "Solo", "type": "ShaderNodeValue"}],
        "links": [["Solo.Value", "Ghost.Value"]],
    }, "not a node id")

    h.errors("node_graph_build unknown property key", "node_graph_build", {
        "tree": "material:ShaderMat",
        "nodes": [{"id": "BadProp", "type": "ShaderNodeMath", "props": {"nonexistent_prop": 1}}],
    }, "no property")

    h.errors("node_graph_build bad enum prop lists real values", "node_graph_build", {
        "tree": "material:ShaderMat",
        "nodes": [{"id": "BadEnum", "type": "ShaderNodeMath", "props": {"operation": "NOPE"}}],
    }, "ADD")

    # Ramp.Color already feeds BSDF; wiring it back into Noise (which already feeds
    # Ramp) closes a real cycle: Noise -> Ramp -> Noise.
    h.errors("node_graph_build rejects a cycle", "node_graph_build", {
        "tree": "material:ShaderMat", "nodes": [],
        "links": [["Ramp.Color", "Noise.Vector"]],
    }, "cycle")

    h.call("node_tree_create", {"kind": "group", "name": "RollbackTree"})
    before = len(bpy.data.node_groups["RollbackTree"].nodes)
    h.errors("a failing build rolls back every node it created", "node_graph_build", {
        "tree": "group:RollbackTree",
        "nodes": [{"id": "Good", "type": "ShaderNodeValue"}, {"id": "Bad", "type": "NotReal"}],
    })
    h.check("rollback leaves the tree exactly as it was before the call",
            len(bpy.data.node_groups["RollbackTree"].nodes) == before)

    h.call("node_tree_create", {"kind": "group", "name": "NoOutputTree"})
    warned = h.call("node_graph_build", {
        "tree": "group:NoOutputTree",
        "nodes": [{"id": "Lonely", "type": "ShaderNodeTexNoise"}],
    })
    h.check("a tree with no output node at all is a WARNING, not a call failure",
            any("no output node" in w for w in warned["warnings"]))

    h.call("node_tree_create", {"kind": "material", "name": "OrphanMat"})
    orphan = h.call("node_graph_build", {
        "tree": "material:OrphanMat", "clear": True,
        "nodes": [
            {"id": "Keep", "type": "ShaderNodeOutputMaterial"},
            {"id": "Stray", "type": "ShaderNodeValue"},
        ],
    })
    h.check("an unreachable node is flagged by its id, the output node is not",
            any("Stray" in w for w in orphan["warnings"])
            and not any("Keep" in w for w in orphan["warnings"]))


def _node_graph_edit(h):
    tree = bpy.data.materials["ShaderMat"].node_tree

    h.call("node_graph_edit", {"tree": "material:ShaderMat", "mute": [{"node": "Ramp", "muted": True}]})
    h.check("node_graph_edit mute persists", tree.nodes["Ramp"].mute is True)

    h.call("node_graph_edit", {"tree": "material:ShaderMat",
                                "move": [{"node": "Noise", "location": [500.0, -500.0]}]})
    h.check("node_graph_edit move persists", tuple(tree.nodes["Noise"].location) == (500.0, -500.0))

    h.call("node_graph_edit", {"tree": "material:ShaderMat",
                                "set_props": [{"node": "Noise", "props": {"noise_dimensions": "4D"}}]})
    h.check("node_graph_edit set_props persists", tree.nodes["Noise"].noise_dimensions == "4D")

    h.call("node_graph_edit", {"tree": "material:ShaderMat",
                                "set_inputs": [{"node": "BSDF", "inputs": {"Roughness": 0.15}}]})
    h.check("node_graph_edit set_inputs persists",
            round(tree.nodes["BSDF"].inputs["Roughness"].default_value, 3) == 0.15)

    before_links = len(tree.links)
    h.call("node_graph_edit", {"tree": "material:ShaderMat", "unlink": [["Noise.Factor", "Ramp.Factor"]]})
    h.check("node_graph_edit unlink removes exactly that link", len(tree.links) == before_links - 1)
    h.errors("node_graph_edit unlink of a link that no longer exists", "node_graph_edit",
             {"tree": "material:ShaderMat", "unlink": [["Noise.Factor", "Ramp.Factor"]]}, "no link")

    h.call("node_graph_edit", {"tree": "material:ShaderMat", "link": [["Noise.Factor", "Ramp.Factor"]]})
    h.check("node_graph_edit link restores it", len(tree.links) == before_links)
    h.errors("node_graph_edit link rejects a cycle", "node_graph_edit",
             {"tree": "material:ShaderMat", "link": [["Ramp.Color", "Noise.Vector"]]}, "cycle")

    h.call("node_graph_build", {"tree": "material:ShaderMat", "nodes": [{"id": "Scratch", "type": "ShaderNodeValue"}]})
    h.call("node_graph_edit", {"tree": "material:ShaderMat", "remove": ["Scratch"]})
    h.check("node_graph_edit remove deletes the node", "Scratch" not in tree.nodes)

    h.errors("node_graph_edit unknown node lists the tree's real names", "node_graph_edit",
             {"tree": "material:ShaderMat", "mute": [{"node": "Ghost", "muted": True}]}, "Nodes in this tree")


# ============================================================================
# Geometry zones + pointer sockets (nodes/mod.rs + graph.rs)
# ============================================================================

def _geometry_zones(h):
    h.call("obj_create_primitive", {"type": "cube", "name": "ProbeObj"})
    h.call("node_tree_create", {"kind": "group", "name": "ObjInfoTree", "node_tree_type": "GeometryNodeTree"})
    h.call("node_graph_build", {
        "tree": "group:ObjInfoTree",
        "nodes": [{"id": "OI", "type": "GeometryNodeObjectInfo", "inputs": {"Object": "ProbeObj"}}],
    })
    oi = bpy.data.node_groups["ObjInfoTree"].nodes["OI"]
    h.check("an Object socket resolves a datablock by name",
            oi.inputs["Object"].default_value is not None and oi.inputs["Object"].default_value.name == "ProbeObj")
    h.errors("an Object socket rejects an unknown name and lists what exists", "node_graph_build", {
        "tree": "group:ObjInfoTree",
        "nodes": [{"id": "OI2", "type": "GeometryNodeObjectInfo", "inputs": {"Object": "NoSuchObject"}}],
    }, "ProbeObj")

    # Simulation zone: a bare Input has no geometry sockets until paired with its Output.
    h.call("node_tree_create", {"kind": "group", "name": "ZoneTree", "node_tree_type": "GeometryNodeTree"})
    h.call("node_graph_build", {
        "tree": "group:ZoneTree",
        "nodes": [
            {"id": "SimIn", "type": "GeometryNodeSimulationInput"},
            {"id": "SimOut", "type": "GeometryNodeSimulationOutput"},
        ],
    })
    zt = bpy.data.node_groups["ZoneTree"]
    h.check("pairing gives the zone Input a real Geometry OUTPUT socket",
            "Geometry" in {s.name for s in zt.nodes["SimIn"].outputs if s.enabled})
    h.check("pairing gives the zone Output a real Geometry INPUT socket",
            "Geometry" in {s.name for s in zt.nodes["SimOut"].inputs if s.enabled})

    h.call("node_tree_create", {"kind": "group", "name": "UnpairedZone", "node_tree_type": "GeometryNodeTree"})
    h.errors("an unpaired zone input (no matching Output in the build) must error", "node_graph_build", {
        "tree": "group:UnpairedZone",
        "nodes": [{"id": "Lonely", "type": "GeometryNodeSimulationInput"}],
    }, "matching")

    h.call("node_tree_create", {"kind": "group", "name": "BadPairZone", "node_tree_type": "GeometryNodeTree"})
    h.errors("`pair` referencing an id outside this build errors", "node_graph_build", {
        "tree": "group:BadPairZone",
        "nodes": [
            {"id": "A", "type": "GeometryNodeSimulationInput", "pair": "NoSuchId"},
            {"id": "B", "type": "GeometryNodeSimulationOutput"},
        ],
    }, "not in this build")

    h.call("node_tree_create", {"kind": "group", "name": "NotAZoneTree", "node_tree_type": "GeometryNodeTree"})
    h.errors("`pair` on a node that isn't a zone input is rejected", "node_graph_build", {
        "tree": "group:NotAZoneTree",
        "nodes": [{"id": "X", "type": "GeometryNodeTransform", "pair": "Y"}],
    }, "not a zone input")


# ============================================================================
# nodes/group.rs
# ============================================================================

def _node_groups_and_geo(h):
    added = h.call("node_interface_add", {
        "group": "PlainGroup", "name": "Amount", "in_out": "INPUT",
        "socket_type": "NodeSocketFloat", "default": 0.5, "min": 0.0, "max": 10.0,
    })
    grp = bpy.data.node_groups["PlainGroup"]
    iface_item = grp.interface.items_tree[added["identifier"]]
    h.check("node_interface_add really adds the socket to the group's interface",
            iface_item.name == "Amount" and iface_item.in_out == "INPUT")
    h.check("node_interface_add's default_value persists", round(iface_item.default_value, 3) == 0.5)
    h.errors("node_interface_add rejects a socket_type invalid for this tree kind, and lists valid ones",
             "node_interface_add",
             {"group": "PlainGroup", "name": "Bad", "in_out": "INPUT", "socket_type": "NodeSocketGeometry"},
             "not a valid socket_type")

    node = h.call("node_group_add", {
        "tree": "material:ShaderMat", "group": "PlainGroup",
        "location": [700.0, 300.0], "inputs": {"Amount": 0.75},
    })
    added_node = bpy.data.materials["ShaderMat"].node_tree.nodes[node["node"]]
    h.check("node_group_add instances the right node type for a shader tree",
            added_node.bl_idname == "ShaderNodeGroup")
    h.check("node_group_add points at the real group", added_node.node_tree.name == "PlainGroup")
    h.check("node_group_add's `inputs` sets the new node's socket value",
            round(added_node.inputs["Amount"].default_value, 3) == 0.75)
    h.errors("node_group_add unknown group", "node_group_add",
             {"tree": "material:ShaderMat", "group": "NoSuchGroup"}, "not found")

    # node_group_make: collapse two connected nodes into a reusable group, wired
    # in place of the originals.
    # ShaderNodeMath's two inputs are BOTH literally named "Value" (only their index
    # tells them apart), so they are left at their defaults here rather than fought
    # over through the by-name `inputs` map - this test is about the group boundary,
    # not the arithmetic.
    h.call("node_graph_build", {
        "tree": "material:ShaderMat",
        "nodes": [
            {"id": "GA", "type": "ShaderNodeMath", "props": {"operation": "ADD"}},
            {"id": "GB", "type": "ShaderNodeMath", "props": {"operation": "MULTIPLY"}},
        ],
        "links": [
            ["GA.Value", "GB.Value"],
            ["GB.Value", "BSDF.Roughness"],
        ],
    })
    made = h.call("node_group_make", {"tree": "material:ShaderMat", "nodes": ["GA", "GB"], "name": "MathGroup"})
    tree = bpy.data.materials["ShaderMat"].node_tree
    h.check("node_group_make creates a real node group", "MathGroup" in bpy.data.node_groups)
    h.check("the moved nodes are gone from the original tree",
            "GA" not in tree.nodes and "GB" not in tree.nodes)
    gnode = tree.nodes[made["node"]]
    h.check("a group node stands in their place", gnode.bl_idname == "ShaderNodeGroup" and gnode.node_tree.name == "MathGroup")
    h.check("the external consumer (BSDF.Roughness) is now fed by the group node",
            tree.nodes["BSDF"].inputs["Roughness"].is_linked
            and tree.nodes["BSDF"].inputs["Roughness"].links[0].from_node.name == gnode.name)

    h.errors("node_group_make refuses to silently rename onto an existing group", "node_group_make",
             {"tree": "material:ShaderMat", "nodes": ["TexCoord"], "name": "PlainGroup"}, "already exists")
    h.check("a rejected name collision does not touch the tree", "TexCoord" in tree.nodes)
    h.errors("node_group_make needs at least one node", "node_group_make",
             {"tree": "material:ShaderMat", "nodes": [], "name": "EmptyGroup"}, "at least one")

    # geo_modifier_set: reuses the geometry_modifier tree from _node_graph.
    group_name = bpy.data.objects["GeoObj"].modifiers["GeoMod"].node_group.name
    added2 = h.call("node_interface_add", {
        "group": group_name, "name": "Amount", "in_out": "INPUT",
        "socket_type": "NodeSocketFloat", "default": 1.0, "min": 0.0, "max": 10.0,
    })
    h.call("geo_modifier_set", {"object": "GeoObj", "modifier": "GeoMod", "values": {"Amount": 2.5}})
    gm = bpy.data.objects["GeoObj"].modifiers["GeoMod"]
    h.check("geo_modifier_set writes the modifier input at its real Blender-native address",
            round(harness.modifier_input(gm, added2["identifier"]), 3) == 2.5)
    h.errors("geo_modifier_set unknown input name lists the modifier's real inputs", "geo_modifier_set",
             {"object": "GeoObj", "modifier": "GeoMod", "values": {"NoSuchInput": 1}}, "Amount")
    h.errors("geo_modifier_set type mismatch", "geo_modifier_set",
             {"object": "GeoObj", "modifier": "GeoMod", "values": {"Amount": "not-a-number"}}, "number")
    h.errors("geo_modifier_set unknown modifier lists the object's real modifiers", "geo_modifier_set",
             {"object": "GeoObj", "modifier": "NoSuchMod", "values": {}}, "GeoMod")
    h.errors("geo_modifier_set unknown object", "geo_modifier_set",
             {"object": "NoSuchObj", "modifier": "GeoMod", "values": {}}, "not found")


# ============================================================================
# nodes/preview.rs
# ============================================================================

def _mat_preview(h):
    before = (len(bpy.data.scenes), len(bpy.data.objects), len(bpy.data.materials),
              len(bpy.data.meshes), len(bpy.data.cameras), len(bpy.data.lights), len(bpy.data.worlds))

    result = h.call("mat_preview", {"material": "ShaderMat", "shape": "cube", "resolution": 32, "samples": 4})
    png = base64.b64decode(result["data"])
    h.check("mat_preview returns a real PNG (magic bytes decode)", png[:8] == b"\x89PNG\r\n\x1a\n")
    h.check("mat_preview honors the requested resolution", result["width"] == 32 and result["height"] == 32)

    after = (len(bpy.data.scenes), len(bpy.data.objects), len(bpy.data.materials),
             len(bpy.data.meshes), len(bpy.data.cameras), len(bpy.data.lights), len(bpy.data.worlds))
    h.check("mat_preview leaves every datablock count exactly as it found them",
            before == after)

    h.errors("mat_preview unknown material", "mat_preview", {"material": "Nope"}, "not found")
    h.errors("mat_preview unknown shape", "mat_preview", {"material": "ShaderMat", "shape": "hexagon"}, "unknown shape")
    h.errors("mat_preview unknown engine", "mat_preview", {"material": "ShaderMat", "engine": "POVRAY"}, "unknown engine")
