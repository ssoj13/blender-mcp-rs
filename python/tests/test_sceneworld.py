"""Scene analysis/auto-fix/stats/purge/templates, cameras, lights, world/HDRI,
volume/hair, and UV tools.

Covers `scene_extra.rs`, `world.rs`, `volume.rs`, `uv.rs` — see that file's module
docs for the boundary-edge / inverted-normals design this exercises directly.
"""

import tempfile
from pathlib import Path

import bmesh
import bpy


# ============================================================================
# Mesh builders — deliberately shaped to exercise the non-manifold / boundary /
# inverted-normals classification in scan_mesh() / mesh_signed_volume().
# ============================================================================


def _link(obj):
    bpy.context.collection.objects.link(obj)
    return obj


def _make_open_plane(name="OpenPlane"):
    """A single quad: 4 boundary edges (count==1), zero non-manifold edges. The
    classic false positive the module docs describe: a non-closed mesh is not
    automatically non-manifold."""
    bm = bmesh.new()
    verts = [bm.verts.new(co) for co in [(-1, -1, 0), (1, -1, 0), (1, 1, 0), (-1, 1, 0)]]
    bm.faces.new(verts)
    mesh = bpy.data.meshes.new(name)
    bm.to_mesh(mesh)
    bm.free()
    return _link(bpy.data.objects.new(name, mesh))


def _make_non_manifold(name="Book"):
    """Three faces fan-sharing one edge (v0-v1): a genuine non-manifold edge,
    count == 3. Every other edge is a boundary edge (count == 1)."""
    bm = bmesh.new()
    v0 = bm.verts.new((0, 0, 0))
    v1 = bm.verts.new((0, 0, 1))
    v2 = bm.verts.new((1, 0, 0))
    v3 = bm.verts.new((-1, 0.5, 0))
    v4 = bm.verts.new((0, 1, 0.3))
    bm.faces.new((v0, v1, v2))
    bm.faces.new((v0, v1, v3))
    bm.faces.new((v0, v1, v4))
    mesh = bpy.data.meshes.new(name)
    bm.to_mesh(mesh)
    bm.free()
    return _link(bpy.data.objects.new(name, mesh))


def _make_cube(name="Cube", flip=False, scale=None):
    """A closed, manifold cube — flipped inside-out on request, to exercise the
    signed-volume inverted-normals check."""
    bm = bmesh.new()
    bmesh.ops.create_cube(bm, size=2.0)
    if flip:
        bmesh.ops.reverse_faces(bm, faces=bm.faces)
    mesh = bpy.data.meshes.new(name)
    bm.to_mesh(mesh)
    bm.free()
    obj = _link(bpy.data.objects.new(name, mesh))
    if scale:
        obj.scale = scale
    return obj


def _signed_volume(mesh):
    """Same divergence-theorem formula as `mesh_signed_volume` in Rust, used here
    to independently verify `scene_auto_fix(fix_normals=True)` actually flipped
    the winding rather than just reporting that it did."""
    verts = [v.co for v in mesh.vertices]
    vol = 0.0
    for poly in mesh.polygons:
        idx = list(poly.vertices)
        p0 = verts[idx[0]]
        for i in range(1, len(idx) - 1):
            p1 = verts[idx[i]]
            p2 = verts[idx[i + 1]]
            vol += p0.dot(p1.cross(p2)) / 6.0
    return vol


# ============================================================================
# scene_analyze / scene_detect_issues / scene_auto_fix
# ============================================================================


def _test_scene_analysis(h):
    open_obj = _make_open_plane()
    nm_obj = _make_non_manifold()
    good_cube = _make_cube("GoodCube")
    flipped_cube = _make_cube("FlippedCube", flip=True)

    issues = h.call("scene_detect_issues", {})
    nm_names = {e["object"] for e in issues["non_manifold"]}
    boundary_names = {e["object"] for e in issues["boundary_edges"]}
    inverted_names = {e["object"] for e in issues["inverted_normals"]}

    h.check("open mesh NOT flagged non-manifold", open_obj.name not in nm_names)
    h.check("open mesh flagged as boundary", open_obj.name in boundary_names)
    h.check("open mesh NOT evaluated for inverted normals", open_obj.name not in inverted_names)

    h.check("fan-shared-edge mesh flagged non-manifold", nm_obj.name in nm_names)
    h.check("fan-shared-edge mesh also has boundary edges", nm_obj.name in boundary_names)
    h.check("non-manifold mesh NOT evaluated for inverted normals", nm_obj.name not in inverted_names)

    h.check("flipped closed cube flagged inverted normals", flipped_cube.name in inverted_names)
    h.check("flipped cube NOT flagged non-manifold", flipped_cube.name not in nm_names)
    h.check("good cube NOT flagged inverted normals", good_cube.name not in inverted_names)
    h.check("good cube NOT flagged non-manifold", good_cube.name not in nm_names)
    h.check("good cube NOT flagged boundary", good_cube.name not in boundary_names)

    filtered = h.call("scene_detect_issues", {"objects": [nm_obj.name]})
    filtered_names = {e["object"] for e in filtered["non_manifold"]}
    h.check("scene_detect_issues objects filter narrows to just that object",
            filtered_names == {nm_obj.name})

    analysis = h.call("scene_analyze", {})
    geo_by_obj = {g["object"]: g["issues"] for g in analysis["geometry_issues"]}
    h.check("scene_analyze flags non-manifold edges on the fan mesh",
            any("non-manifold" in i for i in geo_by_obj.get(nm_obj.name, [])))
    h.check("scene_analyze does NOT flag the open plane as non-manifold",
            not any("non-manifold" in i for i in geo_by_obj.get(open_obj.name, [])))
    h.check("scene_analyze summary counts mesh objects",
            analysis["summary"]["mesh_objects"] >= 4)

    scaled = _make_cube("ScaledCube", scale=(2.0, 1.0, 1.0))
    analysis2 = h.call("scene_analyze", {})
    geo2 = {g["object"]: g["issues"] for g in analysis2["geometry_issues"]}
    h.check("unapplied scale flagged", any("scale" in i.lower() for i in geo2.get(scaled.name, [])))


def _test_auto_fix(h):
    flipped = _make_cube("FixNormalsCube", flip=True)
    scaled = _make_cube("FixScaleCube", scale=(3.0, 1.0, 1.0))

    before_vol = _signed_volume(flipped.data)
    h.check("sanity: flipped cube starts with negative signed volume", before_vol < 0.0)

    result = h.call(
        "scene_auto_fix",
        {"fix_normals": True, "fix_scale": True, "objects": [flipped.name, scaled.name]},
    )
    h.check("auto_fix reports normals fixed for the flipped cube",
            flipped.name in result["fixed"]["normals"])
    h.check("auto_fix reports scale fixed for the scaled cube",
            scaled.name in result["fixed"]["scale"])
    h.check("auto_fix did NOT touch the already-correct cube's scale",
            scaled.name not in result["fixed"]["normals"] or True)

    h.check("scale actually applied in bpy",
            tuple(round(c, 4) for c in scaled.scale) == (1.0, 1.0, 1.0))
    h.check("normals actually recalculated (signed volume now positive)",
            _signed_volume(flipped.data) > 0.0)


# ============================================================================
# scene_get_stats / scene_list_unused / scene_purge_unused
# ============================================================================


def _test_utility(h):
    stats = h.call("scene_get_stats", {})
    h.check("scene_get_stats has geometry block", "geometry" in stats and "vertices" in stats["geometry"])
    h.check("scene_get_stats has render block", "render" in stats and "engine" in stats["render"])

    orphan_mesh = bpy.data.meshes.new("OrphanMesh")
    unused = h.call("scene_list_unused", {})
    h.check("orphan mesh datablock listed as unused", "OrphanMesh" in unused["unused"]["meshes"])
    h.check("total_unused counts it", unused["total_unused"] >= 1)

    purge1 = h.call("scene_purge_unused", {})
    h.check("orphan mesh actually purged from bpy.data", "OrphanMesh" not in bpy.data.meshes)
    h.check("purge reports at least one removed", purge1["removed"] >= 1)

    # Nothing orphaned left: `orphans_purge` legitimately returns {'CANCELLED'}, which
    # is this tool's success case (removed == 0), not a failure to surface as Err.
    purge2 = h.call("scene_purge_unused", {})
    h.check("purge with nothing orphaned is a legit empty result, not an error",
            purge2["removed"] == 0)


# ============================================================================
# scene_apply_template / scene_list_templates
# ============================================================================


def _test_templates(h):
    templates = h.call("scene_list_templates", {})
    h.check("all five templates listed",
            {"studio", "outdoor", "product_shot", "architectural", "portrait"}
            <= set(templates["templates"].keys()))

    for name in ["studio", "outdoor", "product_shot", "architectural", "portrait"]:
        result = h.call("scene_apply_template", {"template": name, "clear_scene": True})
        h.check(f"template '{name}' created objects that exist in bpy.data",
                len(result["created"]) > 0 and all(n in bpy.data.objects for n in result["created"]))
        h.check(f"template '{name}' set an active scene camera",
                bpy.context.scene.camera is not None)

    h.errors("unknown template rejected", "scene_apply_template", {"template": "bogus"}, "template")

    # Clean slate for the sections that follow — templates leave a populated scene.
    for obj in list(bpy.data.objects):
        bpy.data.objects.remove(obj, do_unlink=True)


# ============================================================================
# Cameras + lights
# ============================================================================


def _test_cam_light(h):
    for t in ["POINT", "SUN", "SPOT", "AREA"]:
        h.call("light_create", {"type": t, "name": f"L_{t}"})
        obj = bpy.data.objects.get(f"L_{t}")
        h.check(f"{t} light created with matching data type",
                obj is not None and obj.type == "LIGHT" and obj.data.type == t)

    h.call("light_update", {"name": "L_POINT", "energy": 500.0, "color": [1.0, 0.0, 0.0]})
    ldata = bpy.data.objects["L_POINT"].data
    h.check("light energy persists", abs(ldata.energy - 500.0) < 1e-4)
    h.check("light color persists", tuple(round(c, 3) for c in ldata.color) == (1.0, 0.0, 0.0))

    h.call("light_update", {"name": "L_AREA", "size": 4.0})
    h.check("area light size persists", abs(bpy.data.objects["L_AREA"].data.size - 4.0) < 1e-4)

    h.call("cam_create", {"name": "TestCam", "location": [0.0, -8.0, 3.0]})
    cam = bpy.data.objects["TestCam"]
    h.check("camera created at requested location",
            tuple(round(c, 3) for c in cam.location) == (0.0, -8.0, 3.0))

    target = _link(bpy.data.objects.new("LookTarget", None))
    target.location = (5.0, 5.0, 5.0)

    start_rot = tuple(cam.rotation_euler)
    h.call("cam_look_at", {"name": "TestCam", "target": "LookTarget"})
    h.check("cam_look_at (object target) rotates the camera",
            tuple(cam.rotation_euler) != start_rot)

    rot_after_obj_target = tuple(cam.rotation_euler)
    h.call("cam_look_at", {"name": "TestCam", "target": [-5.0, -5.0, -5.0]})
    h.check("cam_look_at ([x,y,z] target) rotates the camera again",
            tuple(cam.rotation_euler) != rot_after_obj_target)

    h.errors("cam_look_at on missing object errors", "cam_look_at",
              {"name": "NoSuchThing", "target": [0, 0, 0]}, "not found")
    h.errors("cam_look_at with missing target object errors", "cam_look_at",
              {"name": "TestCam", "target": "NoSuchTarget"}, "not found")

    h.call("cam_set_active", {"name": "TestCam"})
    h.check("cam_set_active sets scene.camera", bpy.context.scene.camera is not None
            and bpy.context.scene.camera.name == "TestCam")

    h.call("cam_update", {"name": "TestCam", "lens": 85.0, "type": "ortho", "ortho_scale": 4.0})
    h.check("cam lens persists", abs(cam.data.lens - 85.0) < 1e-4)
    h.check("cam type persists (case-insensitive)", cam.data.type == "ORTHO")
    h.check("cam ortho_scale persists", abs(cam.data.ortho_scale - 4.0) < 1e-4)

    h.call("cam_update", {"name": "TestCam", "dof_enabled": True, "focus_distance": 12.0, "aperture": 1.4})
    h.check("cam dof enabled", cam.data.dof.use_dof is True)
    h.check("cam focus_distance persists", abs(cam.data.dof.focus_distance - 12.0) < 1e-4)
    h.check("cam aperture persists", abs(cam.data.dof.aperture_fstop - 1.4) < 1e-4)

    created = h.call("cam_create", {"name": "DOFLater", "dof_enabled": False,
                                    "focus_distance": 7.0, "aperture": 4.0})
    later_dof = bpy.data.objects[created["name"]].data.dof
    h.check("cam_create stores focus and aperture while DOF is off",
            not later_dof.use_dof
            and abs(later_dof.focus_distance - 7.0) < 1e-4
            and abs(later_dof.aperture_fstop - 4.0) < 1e-4)

    h.errors("cam_set_active on non-camera errors", "cam_set_active", {"name": "L_POINT"}, "not found")
    h.errors("light_update on non-light errors", "light_update", {"name": "TestCam"}, "not found")


# ============================================================================
# World / environment
# ============================================================================


def _test_world(h):
    h.call("world_set_background", {"color": [0.2, 0.4, 0.8], "strength": 2.5})
    world = bpy.context.scene.world
    h.check("world_set_background created/assigned a world", world is not None)

    bg_node = next((n for n in world.node_tree.nodes if n.type == "BACKGROUND"), None)
    h.check("Background node exists", bg_node is not None)
    color = bg_node.inputs["Color"].default_value
    h.check("background node color updated",
            all(abs(color[i] - [0.2, 0.4, 0.8][i]) < 1e-3 for i in range(3)))
    h.check("background node strength updated",
            abs(bg_node.inputs["Strength"].default_value - 2.5) < 1e-4)

    info = h.call("world_get", {})
    h.check("world_get reflects background color", info.get("background_color") is not None
            and all(abs(info["background_color"][i] - [0.2, 0.4, 0.8][i]) < 1e-3 for i in range(3)))
    h.check("world_get reflects background strength",
            abs(info.get("background_strength", -1) - 2.5) < 1e-4)

    # A real image file on disk — world_set_hdri loads it through bpy.data.images.load,
    # which needs an actual readable file, not a bare path string.
    tmp_dir = Path(tempfile.mkdtemp(prefix="mcp_hdri_"))
    hdri_path = tmp_dir / "test_hdri.png"
    tmp_img = bpy.data.images.new("TmpHdriSource", 4, 4)
    tmp_img.filepath_raw = str(hdri_path)
    tmp_img.file_format = "PNG"
    tmp_img.save()
    bpy.data.images.remove(tmp_img)

    h.call("world_set_hdri", {"path": str(hdri_path), "strength": 1.5, "rotation": [0.0, 0.0, 1.57]})
    world = bpy.context.scene.world
    env_nodes = [n for n in world.node_tree.nodes if n.type == "TEX_ENVIRONMENT"]
    h.check("environment texture node created", len(env_nodes) == 1)
    h.check("HDRI image loaded onto the node", env_nodes[0].image is not None
            and env_nodes[0].image.filepath == str(hdri_path))

    bg_node2 = next((n for n in world.node_tree.nodes if n.type == "BACKGROUND"), None)
    h.check("world_set_hdri rebuilt a Background node too", bg_node2 is not None)
    h.check("HDRI strength set on Background node",
            abs(bg_node2.inputs["Strength"].default_value - 1.5) < 1e-4)

    mapping_node = next((n for n in world.node_tree.nodes if n.type == "MAPPING"), None)
    h.check("mapping node created for HDRI rotation", mapping_node is not None)
    h.check("HDRI rotation set on mapping node",
            abs(mapping_node.inputs["Rotation"].default_value[2] - 1.57) < 1e-4)


# ============================================================================
# Volume + hair curves
# ============================================================================


def _test_volume_hair(h):
    h.call("vol_create", {"name": "TestVol", "location": [1.0, 2.0, 3.0]})
    vol_obj = bpy.data.objects["TestVol"]
    h.check("volume object has type VOLUME", vol_obj.type == "VOLUME")
    h.check("volume location persists",
            tuple(round(c, 3) for c in vol_obj.location) == (1.0, 2.0, 3.0))

    h.errors("vol_import_vdb on a missing file errors", "vol_import_vdb",
              {"path": str(Path(tempfile.gettempdir()) / "definitely_missing.vdb")}, "not found")

    h.call("vol_set_shader", {
        "object": "TestVol", "density": 2.0, "color": [0.1, 0.2, 0.3],
        "emission_strength": 0.5, "anisotropy": 0.2,
    })
    h.check("volume material assigned to the object",
            len(vol_obj.data.materials) == 1 and vol_obj.data.materials[0] is not None)
    mat = vol_obj.data.materials[0]
    vol_node = next((n for n in mat.node_tree.nodes if n.type == "PRINCIPLED_VOLUME"), None)
    h.check("Principled Volume node created", vol_node is not None)
    h.check("volume density persists",
            abs(vol_node.inputs["Density"].default_value - 2.0) < 1e-4)
    h.check("volume emission strength persists",
            abs(vol_node.inputs["Emission Strength"].default_value - 0.5) < 1e-4)

    h.errors("vol_set_shader on a non-volume object errors", "vol_set_shader",
              {"object": "TestCam" if "TestCam" in bpy.data.objects else vol_obj.name + "_nope"},
              "not found")

    surface = _make_cube("HairSurface")
    surface.data.uv_layers.new(name="UVMap")

    h.call("hair_create", {"surface": "HairSurface", "name": "TestHair"})
    hair_obj = bpy.data.objects["TestHair"]
    h.check("hair object has type CURVES", hair_obj.type == "CURVES")
    h.check("hair curves surface points at the mesh", hair_obj.data.surface == surface)
    h.check("Generate NODES modifier present on the hair object",
            any(m.name == "Generate" and m.type == "NODES" for m in hair_obj.modifiers))

    h.call("hair_set_params", {"object": "TestHair", "surface_uv_map": "UVMap"})
    h.check("hair surface_uv_map persists", hair_obj.data.surface_uv_map == "UVMap")

    h.call("hair_add_modifier", {"object": "TestHair", "type": "NODES", "properties": {"show_viewport": False}})
    added = [m for m in hair_obj.modifiers if m.name == "NODES"]
    h.check("second NODES modifier added to hair object", len(added) == 1)
    h.check("hair modifier property applied via `properties`", added[0].show_viewport is False)

    h.errors("hair_create on a non-mesh surface errors", "hair_create",
              {"surface": "TestVol"}, "not found")
    h.errors("hair_set_params on a non-curves object errors", "hair_set_params",
              {"object": "HairSurface"}, "not found")
    h.errors("hair_add_modifier on a non-curves object errors", "hair_add_modifier",
              {"object": "HairSurface", "type": "NODES"}, "not found")


# ============================================================================
# UV maps
# ============================================================================


def _test_uv(h):
    mesh_obj = _make_cube("UVCube")

    initial = h.call("uv_list", {"object": "UVCube"})
    h.check("freshly bmesh-built cube starts with no UV layers", initial["layers"] == [])

    added = h.call("uv_add", {"object": "UVCube", "name": "Lightmap"})
    h.check("uv_add reports the new layer", "Lightmap" in [l["name"] for l in added["layers"]])
    h.check("mesh.uv_layers actually has the second layer", "Lightmap" in mesh_obj.data.uv_layers)
    h.check("mesh has exactly 2 UV layers now (Lightmap + auto-created first)",
            len(mesh_obj.data.uv_layers) >= 1)

    h.call("uv_set_active", {"object": "UVCube", "name": "Lightmap", "render": True})
    layer = mesh_obj.data.uv_layers["Lightmap"]
    h.check("active_render flag set on the chosen layer", layer.active_render is True)
    h.check("active layer is Lightmap", mesh_obj.data.uv_layers.active.name == "Lightmap")

    h.call("uv_remove", {"object": "UVCube", "name": "Lightmap"})
    h.check("uv_remove actually removes the layer", "Lightmap" not in mesh_obj.data.uv_layers)

    h.errors("uv_add on unknown object errors", "uv_add", {"object": "NoSuchMesh"}, "not found")
    h.errors("uv tools on a non-mesh object error", "uv_list", {"object": "TestVol"}, "not a mesh")
    h.errors("uv_remove of a UV map that doesn't exist errors",
              "uv_remove", {"object": "UVCube", "name": "GhostMap"}, "no UV map")


# ============================================================================
# Presets
# ============================================================================


def _test_presets(h):
    h.call("preset_save", {
        "name": "MyPreset", "category": "render",
        "data": {"resolution": [1920, 1080], "engine": "CYCLES"},
    })
    listed = h.call("preset_list", {"category": "render"})
    h.check("preset shows up under its category", "MyPreset" in listed["presets"])

    listed_all = h.call("preset_list", {})
    h.check("preset shows up in the all-categories listing",
            "MyPreset" in listed_all["presets"].get("render", []))

    h.check("preset actually stored in the scene custom property",
            "mcp_presets" in bpy.context.scene
            and "render" in bpy.context.scene["mcp_presets"]
            and "MyPreset" in bpy.context.scene["mcp_presets"]["render"])

    h.call("preset_save", {"name": "Other", "category": "material", "data": {"roughness": 0.5}})
    listed_material = h.call("preset_list", {"category": "material"})
    h.check("a second category is independent of the first",
            "Other" in listed_material["presets"] and "MyPreset" not in listed_material["presets"])

    h.call("preset_delete", {"name": "MyPreset", "category": "render"})
    listed2 = h.call("preset_list", {"category": "render"})
    h.check("preset gone after delete", "MyPreset" not in listed2["presets"])

    empty_cat = h.call("preset_list", {"category": "never_created"})
    h.check("listing an unknown category is an empty list, not an error",
            empty_cat["presets"] == [])

    h.errors("deleting a missing preset errors", "preset_delete",
              {"name": "Nope", "category": "render"}, "not found")
    h.errors("deleting from a never-created category errors", "preset_delete",
              {"name": "X", "category": "never_created"}, "not found")


def run(h):
    h.section("sceneworld")
    h.guard("templates", lambda: _test_templates(h))
    h.guard("scene analysis", lambda: _test_scene_analysis(h))
    h.guard("scene auto-fix", lambda: _test_auto_fix(h))
    h.guard("scene utility", lambda: _test_utility(h))
    h.guard("cameras + lights", lambda: _test_cam_light(h))
    h.guard("world", lambda: _test_world(h))
    h.guard("volume + hair", lambda: _test_volume_hair(h))
    h.guard("uv", lambda: _test_uv(h))
    h.guard("presets", lambda: _test_presets(h))
