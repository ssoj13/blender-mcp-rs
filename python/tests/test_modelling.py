"""Object / mesh / mesh-topology-editing tool tests.

Covers every tool in `src/tools/object.rs`, `src/tools/mesh.rs` and
`src/tools/mesh_edit.rs`. Every assertion reads ground truth straight off `bpy` (vertex/
edge/face counts, positions, flags read from `mesh.polygons`/`bmesh` state, object
transforms) rather than trusting a tool's own echoed JSON — a broken tool can still
return a plausible-looking dict.

`cube_verts_faces()` below is a hand-built, outward-normal cube (verified live against
Blender 5 by reading `polygon.normal` for every face before trusting it here — an
earlier, naively-wound version of this cube had every face normal pointing inward,
which would have silently selected the wrong faces in every `mesh_select by=normal`
assertion in this file).
"""

import math

import bpy

import harness


def cube_verts_faces():
    """A hand-built cube: 8 verts at +-1, 6 quads, outward face normals. Winding
    verified live (`polygon.normal` read back per face) -- do not "simplify" the
    winding order without re-checking, an inward-facing cube passes `mesh_create`
    (from_pydata does not validate outward-ness) and silently breaks every
    normal-based selection test below."""
    verts = [
        [-1, -1, -1], [1, -1, -1], [1, 1, -1], [-1, 1, -1],
        [-1, -1, 1], [1, -1, 1], [1, 1, 1], [-1, 1, 1],
    ]
    faces = [
        [0, 3, 2, 1],   # bottom z=-1, normal -Z
        [4, 5, 6, 7],   # top z=1, normal +Z
        [0, 1, 5, 4],   # front y=-1, normal -Y
        [2, 3, 7, 6],   # back y=1, normal +Y
        [0, 4, 7, 3],   # left x=-1, normal -X
        [1, 2, 6, 5],   # right x=1, normal +X
    ]
    return verts, faces


def fresh_cube(h, name):
    verts, faces = cube_verts_faces()
    r = h.call("mesh_create", {"name": name, "verts": verts, "faces": faces})
    return r["name"]


def select_top_face(h, name):
    """Select the single top face (normal +Z) via the predicate every extrude/inset/
    bevel/etc. sub-test below builds on."""
    return h.call("mesh_select", {
        "name": name, "mode": "FACE", "by": "normal",
        "normal_axis": [0, 0, 1], "normal_angle": 10,
    })


def run(h):
    h.section("modelling")
    run_object_tools(h)
    run_mesh_tools(h)
    run_mesh_edit_tools(h)


# ============================================================================
# object.rs
# ============================================================================

def run_object_tools(h):
    # -- obj_create_primitive ------------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "Cube1", "location": [1, 2, 3]})
    ob = bpy.data.objects.get("Cube1")
    h.check("obj_create_primitive: cube is a MESH object", ob is not None and ob.type == "MESH")
    h.check("obj_create_primitive: location persisted",
            tuple(round(x, 4) for x in ob.location) == (1.0, 2.0, 3.0))
    h.check("obj_create_primitive: cube has exactly 8 vertices", len(ob.data.vertices) == 8)

    h.call("obj_create_primitive", {"type": "icosphere", "name": "Ico1"})
    ico = bpy.data.objects.get("Ico1")
    h.check("obj_create_primitive: icosphere is MESH with real geometry",
            ico is not None and ico.type == "MESH" and len(ico.data.vertices) > 8)

    r = h.call("obj_create_primitive", {"primitives": [
        {"type": "sphere", "name": "SphB"},
        {"type": "cone", "name": "ConeB"},
    ]})
    h.check("obj_create_primitive: batch created 2/2", r["created"] == 2)
    h.check("obj_create_primitive: batch objects exist and are MESH",
            bpy.data.objects.get("SphB") is not None
            and bpy.data.objects["SphB"].type == "MESH"
            and bpy.data.objects.get("ConeB") is not None
            and bpy.data.objects["ConeB"].type == "MESH")

    h.errors("obj_create_primitive: unknown type rejected",
             "obj_create_primitive", {"type": "dodecahedron"}, "unknown type")

    # -- obj_list --------------------------------------------------------------
    r = h.call("obj_list", {"type": "MESH"})
    names = {row["name"] for row in r}
    h.check("obj_list: filters by type, ground truth",
            "Cube1" in names and all(bpy.data.objects[n].type == "MESH" for n in names))

    coll = bpy.data.collections.new("FilterColl")
    bpy.context.scene.collection.children.link(coll)
    coll.objects.link(bpy.data.objects["ConeB"])
    r = h.call("obj_list", {"collection": "FilterColl"})
    h.check("obj_list: filters by collection",
            {row["name"] for row in r} == {"ConeB"})

    # -- obj_get -----------------------------------------------------------
    r = h.call("obj_get", {"name": "Cube1"})
    h.check("obj_get: mesh vertex count", r["mesh"]["verts"] == 8)
    h.check("obj_get: location matches bpy", r["location"] == [1.0, 2.0, 3.0])
    h.errors("obj_get: unknown object errors", "obj_get", {"name": "Nope404"}, "not found")

    # -- obj_update ----------------------------------------------------------
    h.call("obj_update", {
        "name": "Cube1", "location": [5, 5, 5], "rotation": [0, 0, math.pi / 2],
        "scale": [2, 2, 2], "visible": False,
    })
    ob = bpy.data.objects["Cube1"]
    h.check("obj_update: location persists", tuple(round(x, 4) for x in ob.location) == (5, 5, 5))
    h.check("obj_update: rotation persists", round(ob.rotation_euler.z, 4) == round(math.pi / 2, 4))
    h.check("obj_update: scale persists", tuple(ob.scale) == (2.0, 2.0, 2.0))
    h.check("obj_update: visible=False hides in viewport and render",
            ob.hide_viewport and ob.hide_render)
    h.call("obj_update", {"name": "Cube1", "visible": True})
    ob = bpy.data.objects["Cube1"]
    h.check("obj_update: visible=True un-hides", not ob.hide_viewport and not ob.hide_render)

    h.call("obj_update", {"updates": [
        {"name": "SphB", "location": [9, 9, 9]},
        {"name": "ConeB", "location": [8, 8, 8]},
    ]})
    h.check("obj_update: batch item 1 persists", tuple(bpy.data.objects["SphB"].location) == (9.0, 9.0, 9.0))
    h.check("obj_update: batch item 2 persists", tuple(bpy.data.objects["ConeB"].location) == (8.0, 8.0, 8.0))

    h.errors("obj_update: unknown object errors", "obj_update", {"name": "Nope404", "location": [0, 0, 0]}, "not found")

    # -- obj_get_selection / obj_set_selection --------------------------------
    h.call("obj_set_selection", {"names": ["Cube1", "Ico1"], "active": "Ico1"})
    h.check("obj_set_selection: ground truth selected_objects",
            {o.name for o in bpy.context.selected_objects} == {"Cube1", "Ico1"})
    h.check("obj_set_selection: ground truth active object",
            bpy.context.view_layer.objects.active.name == "Ico1")

    r = h.call("obj_get_selection")
    h.check("obj_get_selection: reports the same selection",
            set(r["selected"]) == {"Cube1", "Ico1"} and r["active"] == "Ico1" and r["count"] == 2)

    r2 = h.call("obj_get_selection")
    h.check("obj_get_selection: selection persists across separate calls",
            set(r2["selected"]) == {"Cube1", "Ico1"})

    h.call("obj_set_selection", {"names": ["DoesNotExist"]})
    h.check("obj_set_selection: unknown names are silently skipped, not an error",
            len(bpy.context.selected_objects) == 0)

    # -- obj_duplicate ---------------------------------------------------------
    h.call("obj_duplicate", {"name": "Cube1", "new_name": "Cube1Dup", "offset": [1, 0, 0]})
    dup = bpy.data.objects.get("Cube1Dup")
    orig = bpy.data.objects["Cube1"]
    h.check("obj_duplicate: duplicate exists", dup is not None)
    h.check("obj_duplicate: offset applied relative to source",
            tuple(round(x, 4) for x in dup.location)
            == tuple(round(orig.location[i] + [1, 0, 0][i], 4) for i in range(3)))
    h.check("obj_duplicate: unlinked duplicate owns separate mesh data",
            dup.data != orig.data)

    h.call("obj_duplicate", {"name": "Cube1", "new_name": "Cube1Linked", "linked": True})
    linked = bpy.data.objects["Cube1Linked"]
    h.check("obj_duplicate: linked=true shares mesh data", linked.data == orig.data)

    r = h.call("obj_duplicate", {"duplicates": [
        {"name": "Cube1", "new_name": "Cube1BatchA"},
        {"name": "Cube1", "new_name": "Cube1BatchB"},
    ]})
    h.check("obj_duplicate: batch created 2/2", r["created"] == 2)
    h.check("obj_duplicate: batch objects exist",
            "Cube1BatchA" in bpy.data.objects and "Cube1BatchB" in bpy.data.objects)

    h.errors("obj_duplicate: unknown source errors", "obj_duplicate", {"name": "Nope404"}, "not found")

    # -- obj_parent ------------------------------------------------------------
    h.call("obj_parent", {"parent": "Cube1", "children": ["Ico1"]})
    h.check("obj_parent: child.parent ground truth",
            bpy.data.objects["Ico1"].parent == bpy.data.objects["Cube1"])
    h.errors("obj_parent: unknown parent errors",
             "obj_parent", {"parent": "Nope404", "children": ["Ico1"]}, "not found")

    h.call("obj_create_primitive", {"type": "cube", "name": "ParentBatchP"})
    h.call("obj_create_primitive", {"type": "cube", "name": "ParentBatchC"})
    h.call("obj_parent", {"operations": [
        {"parent": "ParentBatchP", "children": ["ParentBatchC"]},
    ]})
    h.check("obj_parent: batch operations applied",
            bpy.data.objects["ParentBatchC"].parent == bpy.data.objects["ParentBatchP"])

    # -- obj_shade ---------------------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "ShadeCube"})
    h.call("obj_shade", {"name": "ShadeCube", "mode": "SMOOTH"})
    mesh = bpy.data.objects["ShadeCube"].data
    h.check("obj_shade SMOOTH: every polygon.use_smooth is True",
            all(p.use_smooth for p in mesh.polygons))

    h.call("obj_shade", {"name": "ShadeCube", "mode": "FLAT"})
    h.check("obj_shade FLAT: every polygon.use_smooth is False",
            all(not p.use_smooth for p in mesh.polygons))

    h.call("obj_shade", {"name": "ShadeCube", "mode": "AUTO_SMOOTH", "angle": 45})
    shobj = bpy.data.objects["ShadeCube"]
    mod = shobj.modifiers.get("Smooth by Angle")
    h.check("obj_shade AUTO_SMOOTH: adds the 'Smooth by Angle' geo-nodes modifier",
            mod is not None and mod.type == "NODES")
    h.check("obj_shade AUTO_SMOOTH: angle baked into the modifier input",
            mod is not None
            and abs(harness.modifier_input(mod, "Input_1") - math.radians(45)) < 1e-6)

    r = h.call("obj_shade", {"objects": [{"name": "Cube1BatchA", "mode": "SMOOTH"}]})
    h.check("obj_shade: batch mode shaded 1/1", r["shaded"] == 1)
    h.check("obj_shade: batch mode ground truth",
            all(p.use_smooth for p in bpy.data.objects["Cube1BatchA"].data.polygons))

    bpy.ops.object.empty_add()
    empty = bpy.context.active_object
    empty.name = "GuardEmpty"
    h.errors("obj_shade: non-MESH object rejected", "obj_shade", {"name": "GuardEmpty", "mode": "SMOOTH"}, "MESH")

    # -- obj_apply_transform -----------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "ApplyCube", "scale": [2, 3, 4]})
    h.call("obj_apply_transform", {"name": "ApplyCube"})
    ob = bpy.data.objects["ApplyCube"]
    h.check("obj_apply_transform: scale resets to identity",
            tuple(round(x, 6) for x in ob.scale) == (1.0, 1.0, 1.0))
    xs = [v.co.x for v in ob.data.vertices]
    h.check("obj_apply_transform: scale baked into mesh geometry (x extent now 2)",
            abs(max(abs(x) for x in xs) - 2.0) < 1e-6)

    # -- obj_join ------------------------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "JoinA"})
    h.call("obj_create_primitive", {"type": "cube", "name": "JoinB", "location": [5, 0, 0]})
    h.call("obj_join", {"target": "JoinA", "sources": ["JoinB"]})
    h.check("obj_join: source removed from the scene", "JoinB" not in bpy.data.objects)
    h.check("obj_join: target survives", "JoinA" in bpy.data.objects)
    h.check("obj_join: vertex counts merged (8+8=16)",
            len(bpy.data.objects["JoinA"].data.vertices) == 16)
    h.errors("obj_join: empty sources rejected", "obj_join", {"target": "JoinA", "sources": []}, "empty")
    h.errors("obj_join: unknown target errors", "obj_join", {"target": "Nope404", "sources": ["JoinA"]}, "not found")

    # -- obj_separate --------------------------------------------------------
    verts, faces = cube_verts_faces()
    verts2 = verts + [[v[0] + 20, v[1], v[2]] for v in verts]
    faces2 = faces + [[i + 8 for i in f] for f in faces]
    h.call("mesh_create", {"name": "LooseParts", "verts": verts2, "faces": faces2})
    before = set(bpy.data.objects.keys())
    r = h.call("obj_separate", {"name": "LooseParts", "mode": "LOOSE"})
    after = set(bpy.data.objects.keys())
    h.check("obj_separate LOOSE: created exactly one new object (ground truth)",
            len(after - before) == 1)
    h.check("obj_separate LOOSE: tool reports the same new object(s)",
            set(r["created"]) == (after - before))

    h.call("mesh_create", {"name": "SelSep", "verts": [
        [0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0],
        [10, 0, 0], [11, 0, 0], [11, 1, 0], [10, 1, 0],
    ], "faces": [[0, 1, 2, 3], [4, 5, 6, 7]]})
    h.call("mesh_select", {"name": "SelSep", "mode": "FACE", "indices": [1]})
    before = set(bpy.data.objects.keys())
    h.call("obj_separate", {"name": "SelSep", "mode": "SELECTED"})
    after = set(bpy.data.objects.keys())
    h.check("obj_separate SELECTED: honours a selection made by an earlier mesh_select call",
            len(after - before) == 1)

    h.errors("obj_separate: non-MESH object rejected", "obj_separate", {"name": "GuardEmpty", "mode": "LOOSE"}, "MESH")

    # -- obj_origin ----------------------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "OriginCube", "location": [5, 5, 5]})
    h.call("obj_origin", {"name": "OriginCube", "mode": "CURSOR", "cursor_position": [1, 2, 3]})
    ob = bpy.data.objects["OriginCube"]
    h.check("obj_origin CURSOR: object location moves to cursor_position",
            tuple(round(x, 4) for x in ob.location) == (1.0, 2.0, 3.0))
    h.check("obj_origin CURSOR: 3D cursor left at that position",
            tuple(round(x, 4) for x in bpy.context.scene.cursor.location) == (1.0, 2.0, 3.0))

    h.call("obj_origin", {"name": "OriginCube", "mode": "GEOMETRY", "center": "BOUNDS"})
    # Origin-to-geometry must leave the origin at the geometric centre: the mean of the
    # local vertex coordinates is then ~0. Checking that proves the op did its job, not
    # just that obj_get returned a dict.
    ob = bpy.data.objects["OriginCube"]
    n = len(ob.data.vertices)
    centroid = [sum(v.co[i] for v in ob.data.vertices) / n for i in range(3)]
    h.check("obj_origin GEOMETRY BOUNDS: origin sits at the geometric centre",
            all(abs(c) < 1e-4 for c in centroid))
    h.errors("obj_origin: unknown object errors", "obj_origin", {"name": "Nope404", "mode": "CURSOR"}, "not found")

    # -- obj_delete ------------------------------------------------------------
    h.call("obj_delete", {"name": "JoinA"})
    h.check("obj_delete: object removed from bpy.data.objects", "JoinA" not in bpy.data.objects)
    h.errors("obj_delete: unknown object errors", "obj_delete", {"name": "Nope404"}, "not found")

    h.call("obj_delete", {"names": ["SphB", "ConeB"]})
    h.check("obj_delete: batch removed both",
            "SphB" not in bpy.data.objects and "ConeB" not in bpy.data.objects)


# ============================================================================
# mesh.rs
# ============================================================================

def run_mesh_tools(h):
    h.call("obj_create_primitive", {"type": "cube", "name": "MeshOpsCube"})

    # -- mesh_get_data / mesh_set_data ----------------------------------------
    r = h.call("mesh_get_data", {
        "name": "MeshOpsCube", "include_verts": True, "include_faces": True, "include_normals": True,
    })
    h.check("mesh_get_data: counts match a default cube", r["vertices"] == 8 and r["polygons"] == 6)
    h.check("mesh_get_data: vertex_positions has one entry per vertex", len(r["vertex_positions"]) == 8)
    h.check("mesh_get_data: face_indices present", len(r["face_indices"]) == 6)
    h.check("mesh_get_data: vertex_normals present", len(r["vertex_normals"]) == 8)

    positions = r["vertex_positions"]
    positions[0] = [42.0, 42.0, 42.0]
    h.call("mesh_set_data", {"name": "MeshOpsCube", "vertex_positions": positions})
    h.check("mesh_set_data: ground truth vertex moved",
            tuple(round(x, 4) for x in bpy.data.objects["MeshOpsCube"].data.vertices[0].co)
            == (42.0, 42.0, 42.0))
    h.errors("mesh_set_data: vertex-count mismatch rejected",
             "mesh_set_data", {"name": "MeshOpsCube", "vertex_positions": [[0, 0, 0]]}, "mismatch")

    # -- mesh_extrude / mesh_subdivide ----------------------------------------
    before = len(bpy.data.objects["MeshOpsCube"].data.vertices)
    h.call("mesh_extrude", {"name": "MeshOpsCube", "direction": [0, 0, 1]})
    after = len(bpy.data.objects["MeshOpsCube"].data.vertices)
    h.check("mesh_extrude: geometry actually grew", after > before)

    h.call("obj_create_primitive", {"type": "cube", "name": "SubdivCube"})
    h.call("mesh_subdivide", {"name": "SubdivCube", "cuts": 1})
    h.check("mesh_subdivide: exact vertex count after one cut (8 orig + 12 edge mids + 6 face centers)",
            len(bpy.data.objects["SubdivCube"].data.vertices) == 26)

    # -- mesh_get_uvs / mesh_unwrap --------------------------------------------
    # A raw mesh_create mesh has no UV layer; `obj_create_primitive` cubes are NOT a
    # valid "before" fixture here -- Blender's own primitive_cube_add operator already
    # generates a default "UVMap" layer (verified live), which would make this
    # assertion pass even if mesh_get_uvs' has_uvs detection were broken.
    verts, faces = cube_verts_faces()
    h.call("mesh_create", {"name": "UvCube", "verts": verts, "faces": faces})
    r = h.call("mesh_get_uvs", {"name": "UvCube"})
    h.check("mesh_get_uvs: no UVs before unwrap (raw mesh_create mesh has none)", r["has_uvs"] is False)
    h.call("mesh_unwrap", {"name": "UvCube", "method": "angle_based"})
    r2 = h.call("mesh_get_uvs", {"name": "UvCube"})
    h.check("mesh_unwrap angle_based: UVs now exist", r2["has_uvs"] is True and r2["uv_count"] > 0)

    h.call("obj_create_primitive", {"type": "cube", "name": "UvCube2"})
    h.call("mesh_unwrap", {"name": "UvCube2", "method": "smart", "angle": 66})
    r3 = h.call("mesh_get_uvs", {"name": "UvCube2"})
    h.check("mesh_unwrap smart project: UVs now exist", r3["has_uvs"] is True)

    # -- curve_create / curve_add_point / curve_update_point ------------------
    h.call("curve_create", {
        "type": "BEZIER", "name": "MyCurve", "points": [[0, 0, 0], [1, 0, 0], [2, 1, 0]],
        "bevel_depth": 0.1,
    })
    cob = bpy.data.objects["MyCurve"]
    h.check("curve_create: object is a CURVE", cob.type == "CURVE")
    h.check("curve_create: bezier point count matches input", len(cob.data.splines[0].bezier_points) == 3)
    h.check("curve_create: bevel_depth persists (falsy-0.0 special case does not eat a real value)",
            abs(cob.data.bevel_depth - 0.1) < 1e-6)
    h.check("curve_create regression: bezier handles use `handle_left_type`/`handle_right_type` "
            "(the real Blender 5 RNA name -- `handle_type_left` does not exist and raises)",
            cob.data.splines[0].bezier_points[0].handle_left_type == "AUTO"
            and cob.data.splines[0].bezier_points[0].handle_right_type == "AUTO")

    h.call("curve_create", {"type": "NURBS", "name": "MyNurbs", "points": [[0, 0, 0], [1, 0, 0]]})
    nob = bpy.data.objects["MyNurbs"]
    h.check("curve_create NURBS: spline type ground truth", nob.data.splines[0].type == "NURBS")
    h.check("curve_create NURBS: point count matches input", len(nob.data.splines[0].points) == 2)

    h.call("curve_add_point", {"name": "MyCurve", "position": [3, 2, 0]})
    h.check("curve_add_point: bezier point count grew by 1",
            len(bpy.data.objects["MyCurve"].data.splines[0].bezier_points) == 4)

    h.call("curve_update_point", {
        "name": "MyCurve", "point_index": 0, "position": [9, 9, 0], "handle_type": "VECTOR",
    })
    bp = bpy.data.objects["MyCurve"].data.splines[0].bezier_points[0]
    h.check("curve_update_point: position ground truth",
            tuple(round(x, 4) for x in bp.co) == (9.0, 9.0, 0.0))
    h.check("curve_update_point: handle_type ground truth",
            bp.handle_left_type == "VECTOR" and bp.handle_right_type == "VECTOR")

    # -- text_create / text_update --------------------------------------------
    h.call("text_create", {"body": "Hello", "name": "MyText", "size": 2.0})
    tob = bpy.data.objects["MyText"]
    h.check("text_create: object is FONT type", tob.type == "FONT")
    h.check("text_create: body ground truth", tob.data.body == "Hello")
    h.check("text_create: size ground truth", abs(tob.data.size - 2.0) < 1e-6)
    h.check("text_create: default align_y is a real Blender 5 enum value (TOP_BASELINE)",
            tob.data.align_y == "TOP_BASELINE")

    h.call("text_update", {"name": "MyText", "body": "World"})
    h.check("text_update: body ground truth updates", bpy.data.objects["MyText"].data.body == "World")
    h.errors("text_update: non-FONT object rejected", "text_update", {"name": "MeshOpsCube", "body": "x"}, "not found")

    # -- mesh_convert_to -------------------------------------------------------
    h.call("mesh_convert_to", {"name": "MyCurve"})
    h.check("mesh_convert_to: curve becomes MESH", bpy.data.objects["MyCurve"].type == "MESH")
    h.errors("mesh_convert_to: unsupported type rejected",
             "mesh_convert_to", {"name": "MeshOpsCube"}, "cannot")

    r = h.call("mesh_convert_to", {"names": ["MyNurbs", "MyText"]})
    h.check("mesh_convert_to: batch converted 2/2", r["converted"] == 2)
    h.check("mesh_convert_to: batch ground truth (NURBS)", bpy.data.objects["MyNurbs"].type == "MESH")
    h.check("mesh_convert_to: batch ground truth (FONT)", bpy.data.objects["MyText"].type == "MESH")

    # -- shapekey_add / shapekey_update / shapekey_list / shapekey_remove -----
    h.call("obj_create_primitive", {"type": "cube", "name": "SkCube"})
    h.call("shapekey_add", {"name": "SkCube", "key_name": "Key1", "value": 0.5})
    skob = bpy.data.objects["SkCube"]
    h.check("shapekey_add: creates an implicit Basis plus the named key",
            [k.name for k in skob.data.shape_keys.key_blocks] == ["Basis", "Key1"])
    h.check("shapekey_add: value ground truth",
            abs(skob.data.shape_keys.key_blocks["Key1"].value - 0.5) < 1e-6)

    h.call("shapekey_update", {
        "name": "SkCube", "key_name": "Key1", "value": 0.8,
        "slider_min": -1.0, "slider_max": 2.0, "mute": True,
    })
    kb = bpy.data.objects["SkCube"].data.shape_keys.key_blocks["Key1"]
    h.check("shapekey_update: ground truth",
            abs(kb.value - 0.8) < 1e-6 and abs(kb.slider_min + 1.0) < 1e-6
            and abs(kb.slider_max - 2.0) < 1e-6 and kb.mute is True)

    r = h.call("shapekey_list", {"name": "SkCube"})
    h.check("shapekey_list: reports both keys", r["has_shape_keys"] is True and len(r["keys"]) == 2)

    h.call("shapekey_remove", {"name": "SkCube", "key_name": "Key1"})
    h.check("shapekey_remove: ground truth removal",
            "Key1" not in [k.name for k in bpy.data.objects["SkCube"].data.shape_keys.key_blocks])
    h.errors("shapekey_remove: unknown key errors",
             "shapekey_remove", {"name": "SkCube", "key_name": "Nope"}, "not found")
    h.errors("shapekey_remove: object with no shape keys at all errors cleanly",
             "shapekey_remove", {"name": "MeshOpsCube", "key_name": "X"}, "no shape keys")

    r = h.call("shapekey_list", {"name": "MeshOpsCube"})
    h.check("shapekey_list: an object with no shape keys reports an empty list, not an error",
            r["has_shape_keys"] is False and r["keys"] == [])

    # -- obj_instance_to_points / obj_make_instances_real ----------------------
    h.call("obj_create_primitive", {"type": "plane", "name": "InstBase"})
    h.call("mesh_subdivide", {"name": "InstBase", "cuts": 2})
    h.call("obj_create_primitive", {"type": "cube", "name": "InstChild"})
    h.call("obj_instance_to_points", {"name": "InstBase", "instance": "InstChild", "instance_type": "VERTS"})
    base = bpy.data.objects["InstBase"]
    child = bpy.data.objects["InstChild"]
    h.check("obj_instance_to_points: instance_type ground truth", base.instance_type == "VERTS")
    h.check("obj_instance_to_points: child re-parented to base", child.parent == base)
    h.check("obj_instance_to_points: child hidden from render",
            child.hide_render is True)
    h.check(
        "obj_instance_to_points regression: child must NOT be `hide_viewport` -- that excludes "
        "it from the depsgraph and silently generates zero dupli-instances (verified live: "
        "duplicates_make_real reports FINISHED but creates nothing). The per-view-layer "
        "`hide_get()` state hides the child's own copy without breaking instancing",
        child.hide_viewport is False and child.hide_get() is True,
    )

    before = set(bpy.data.objects.keys())
    h.call("obj_make_instances_real", {"name": "InstBase"})
    after = set(bpy.data.objects.keys())
    h.check("obj_make_instances_real: ground truth -- real objects actually created "
            "(one per base vertex, not zero)", len(after - before) == len(base.data.vertices))

    h.errors("obj_instance_to_points: unknown base object errors",
             "obj_instance_to_points", {"name": "Nope404", "instance": "InstChild"}, "not found")
    h.errors("obj_instance_to_points: unknown instance object errors",
             "obj_instance_to_points", {"name": "InstBase", "instance": "Nope404"}, "not found")


# ============================================================================
# mesh_edit.rs
# ============================================================================

def run_mesh_edit_tools(h):
    # -- mesh_create -------------------------------------------------------
    verts, faces = cube_verts_faces()
    r = h.call("mesh_create", {"name": "RawCube", "verts": verts, "faces": faces})
    h.check("mesh_create: tool-reported counts are exact", r["vertices"] == 8 and r["faces"] == 6)
    ob = bpy.data.objects["RawCube"]
    h.check("mesh_create: ground truth vertex/face counts", len(ob.data.vertices) == 8 and len(ob.data.polygons) == 6)
    h.check("mesh_create: ground truth edge count (12 cube edges)", len(ob.data.edges) == 12)

    h.errors("mesh_create: empty verts rejected", "mesh_create", {"name": "Bad", "verts": []}, "empty")
    h.errors("mesh_create: wrong vertex component count rejected",
             "mesh_create", {"name": "Bad2", "verts": [[0, 0]]}, "component")
    h.errors("mesh_create: out-of-range face index rejected",
             "mesh_create", {"name": "Bad3", "verts": [[0, 0, 0], [1, 0, 0], [2, 0, 0]], "faces": [[0, 1, 5]]},
             "index")
    h.errors("mesh_create: degenerate face (<3 verts) rejected",
             "mesh_create", {"name": "Bad4", "verts": [[0, 0, 0], [1, 0, 0], [2, 0, 0]], "faces": [[0, 1]]},
             "3")

    # -- mesh_select: explicit indices, persistence, predicates ------------
    r = h.call("mesh_select", {"name": "RawCube", "mode": "VERT", "indices": [0, 1, 2, 3]})
    h.check("mesh_select by indices: tool-reported count", r["selected_verts"] == 4)
    h.check("mesh_select by indices: ground truth off bpy",
            sum(1 for v in bpy.data.objects["RawCube"].data.vertices if v.select) == 4)

    r = select_top_face(h, "RawCube")
    h.check("mesh_select by=normal: selects exactly the one +Z face", r["selected_faces"] == 1)
    r = h.call("mesh_select", {"name": "RawCube", "mode": "FACE", "by": "invert"})
    h.check("mesh_select by=invert: flips a 1-of-6 selection to 5-of-6", r["selected_faces"] == 5)
    r = h.call("mesh_select", {"name": "RawCube", "mode": "FACE", "by": "none"})
    h.check("mesh_select by=none: clears the selection", r["selected_faces"] == 0)
    r = h.call("mesh_select", {"name": "RawCube", "mode": "FACE", "by": "all"})
    h.check("mesh_select by=all: selects every face", r["selected_faces"] == 6)
    r = h.call("mesh_select", {"name": "RawCube", "mode": "FACE", "by": "material_index", "material_index": 0})
    h.check("mesh_select by=material_index: all faces are slot 0 by default", r["selected_faces"] == 6)
    r = h.call("mesh_select", {
        "name": "RawCube", "mode": "VERT", "by": "bbox", "bbox_min": [-2, -2, 0.5], "bbox_max": [2, 2, 2],
    })
    h.check("mesh_select by=bbox: picks exactly the 4 top-face verts", r["selected_verts"] == 4)

    h.call("mesh_create", {"name": "OpenPlane", "verts": [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]],
                            "faces": [[0, 1, 2, 3]]})
    r = h.call("mesh_select", {"name": "OpenPlane", "mode": "EDGE", "by": "boundary"})
    h.check("mesh_select by=boundary: a single quad's edges are all boundary edges", r["selected_edges"] == 4)
    r = h.call("mesh_select", {"name": "OpenPlane", "mode": "EDGE", "by": "non_manifold"})
    h.check("mesh_select by=non_manifold: a single quad's edges are non-manifold too "
            "(only 1 linked face, manifold needs exactly 2)", r["selected_edges"] == 4)

    h.call("mesh_create", {"name": "LooseGeom", "verts": [[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 1, 0]],
                            "edges": [[2, 3]], "faces": [[0, 1, 2]]})
    r = h.call("mesh_select", {"name": "LooseGeom", "mode": "VERT", "by": "loose"})
    h.check("mesh_select by=loose VERT: only the vertex touching zero faces is loose "
            "(vertex 3, not verts 0/1/2 which are part of the triangle)", r["selected_verts"] == 1)
    ob = bpy.data.objects["LooseGeom"]
    h.check("mesh_select by=loose VERT: ground truth is exactly vertex index 3",
            [v.index for v in ob.data.vertices if v.select] == [3])
    r = h.call("mesh_select", {"name": "LooseGeom", "mode": "EDGE", "by": "loose"})
    h.check("mesh_select by=loose EDGE: the (2,3) edge is wire (touches no face)", r["selected_edges"] == 1)

    h.errors("mesh_select: unknown mode rejected", "mesh_select", {"name": "RawCube", "mode": "XYZ"}, "mode")
    h.errors("mesh_select: neither indices nor by given", "mesh_select", {"name": "RawCube", "mode": "FACE"}, "")
    h.errors("mesh_select: material_index requires FACE mode",
             "mesh_select", {"name": "RawCube", "mode": "VERT", "by": "material_index", "material_index": 0},
             "FACE")
    h.errors("mesh_select: out-of-range index rejected",
             "mesh_select", {"name": "RawCube", "mode": "VERT", "indices": [999]}, "range")
    h.errors("mesh_select: unknown object errors", "mesh_select", {"name": "Nope404", "mode": "VERT", "by": "all"},
             "not found")

    # -- mesh_edit: every op, each against a fresh cube ------------------------
    name = fresh_cube(h, "CubeExtrudeRegion")
    select_top_face(h, name)
    h.call("mesh_edit", {"name": name, "ops": [{"op": "extrude", "distance": 1.0}]})
    d = h.call("mesh_get_data", {"name": name, "include_verts": True})
    h.check("mesh_edit extrude (region): vertex count grows by 4 (8 -> 12)", d["vertices"] == 12)
    top_z = sorted(v[2] for v in d["vertex_positions"])[-4:]
    h.check("mesh_edit extrude (region): moved face is now at old height + distance (z=2)",
            all(abs(z - 2.0) < 1e-6 for z in top_z))

    name = fresh_cube(h, "CubeExtrudeIndividual")
    select_top_face(h, name)
    r = h.call("mesh_edit", {"name": name, "ops": [{"op": "extrude", "extrude_mode": "individual", "distance": 1.0}]})
    h.check("mesh_edit extrude (individual): reports the new face count", r["ops"][0]["new_faces"] == 1)
    h.check("mesh_edit extrude (individual): vertex count grows by 4",
            len(bpy.data.objects[name].data.vertices) == 12)

    name = fresh_cube(h, "CubeInset")
    select_top_face(h, name)
    r = h.call("mesh_edit", {"name": name, "ops": [{"op": "inset", "thickness": 0.2}]})
    h.check("mesh_edit inset: reports 4 new faces (one quad inset into 4)", r["ops"][0]["new_faces"] == 4)
    h.check("mesh_edit inset: ground truth face count (5 untouched + 1 shrunk cap + 4 ring faces = 10)",
            len(bpy.data.objects[name].data.polygons) == 10)

    name = fresh_cube(h, "CubeBevel")
    h.call("mesh_select", {"name": name, "mode": "EDGE", "by": "all"})
    h.call("mesh_edit", {"name": name, "ops": [{"op": "bevel", "thickness": 0.1, "segments": 2, "affect": "edges"}]})
    ob = bpy.data.objects[name]
    h.check("mesh_edit bevel: geometry grows substantially (8 -> 32 verts, all 12 edges beveled)",
            len(ob.data.vertices) == 32 and len(ob.data.polygons) == 30)

    name = fresh_cube(h, "CubeSubdivideEdit")
    h.call("mesh_select", {"name": name, "mode": "EDGE", "by": "all"})
    h.call("mesh_edit", {"name": name, "ops": [{"op": "subdivide", "cuts": 1}]})
    h.check("mesh_edit subdivide: exact vertex count (matches obj mesh_subdivide result)",
            len(bpy.data.objects[name].data.vertices) == 26)

    verts_dup = [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0], [0.00005, 0.00005, 0]]
    h.call("mesh_create", {"name": "MergeMesh", "verts": verts_dup, "faces": [[0, 1, 2, 3]]})
    h.call("mesh_select", {"name": "MergeMesh", "mode": "VERT", "by": "all"})
    r = h.call("mesh_edit", {"name": "MergeMesh", "ops": [{"op": "merge", "merge_distance": 0.001}]})
    h.check("mesh_edit merge: reports 1 vertex removed (the near-duplicate)",
            r["ops"][0]["vertices_removed"] == 1)
    h.check("mesh_edit merge: ground truth vertex count dropped to 4",
            len(bpy.data.objects["MergeMesh"].data.vertices) == 4)

    name = fresh_cube(h, "CubeDelete")
    select_top_face(h, name)
    r = h.call("mesh_edit", {"name": name, "ops": [{"op": "delete", "delete_mode": "faces"}]})
    h.check("mesh_edit delete (faces): reports 1 face deleted", r["ops"][0]["deleted"] == 1)
    ob = bpy.data.objects[name]
    h.check("mesh_edit delete (faces): only the face is gone, shared verts/edges remain",
            len(ob.data.vertices) == 8 and len(ob.data.edges) == 12 and len(ob.data.polygons) == 5)

    name = fresh_cube(h, "CubeDuplicateEdit")
    select_top_face(h, name)
    h.call("mesh_edit", {"name": name, "ops": [{"op": "duplicate"}]})
    h.check("mesh_edit duplicate: vertex count grows by 4 (the duplicated face's 4 verts)",
            len(bpy.data.objects[name].data.vertices) == 12)

    h.call("mesh_create", {"name": "SpinProfile", "verts": [[1, 0, 0], [1, 0, 1]], "edges": [[0, 1]]})
    h.call("mesh_select", {"name": "SpinProfile", "mode": "EDGE", "by": "all"})
    h.call("mesh_edit", {"name": "SpinProfile", "ops": [
        {"op": "spin", "vector": [0, 0, 1], "center": [0, 0, 0], "angle": 360, "steps": 8},
    ]})
    ob = bpy.data.objects["SpinProfile"]
    h.check("mesh_edit spin: lathes a 2-vert profile into a closed 8-step surface",
            len(ob.data.vertices) == 18 and len(ob.data.polygons) == 8)

    h.call("mesh_create", {"name": "BridgeMesh", "verts": [
        [0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0],
        [0, 0, 2], [1, 0, 2], [1, 1, 2], [0, 1, 2],
    ], "faces": [[0, 1, 2, 3], [4, 5, 6, 7]]})
    h.call("mesh_select", {"name": "BridgeMesh", "mode": "EDGE", "by": "boundary"})
    r = h.call("mesh_edit", {"name": "BridgeMesh", "ops": [{"op": "bridge"}]})
    h.check("mesh_edit bridge: reports 4 new wall faces", r["ops"][0]["new_faces"] == 4)
    h.check("mesh_edit bridge: ground truth (2 caps + 4 walls = 6 faces)",
            len(bpy.data.objects["BridgeMesh"].data.polygons) == 6)

    name = fresh_cube(h, "CubeTri")
    h.call("mesh_select", {"name": name, "mode": "FACE", "by": "all"})
    h.call("mesh_edit", {"name": name, "ops": [{"op": "triangulate"}]})
    h.check("mesh_edit triangulate: 6 quads become 12 triangles, verts unchanged",
            len(bpy.data.objects[name].data.polygons) == 12
            and len(bpy.data.objects[name].data.vertices) == 8)

    name = fresh_cube(h, "CubeFlip")
    h.call("mesh_select", {"name": name, "mode": "FACE", "by": "all"})
    ob = bpy.data.objects[name]
    normals_before = [tuple(round(x, 3) for x in p.normal) for p in ob.data.polygons]
    h.call("mesh_edit", {"name": name, "ops": [{"op": "flip_normals"}]})
    normals_after = [tuple(round(x, 3) for x in p.normal) for p in ob.data.polygons]
    h.check("mesh_edit flip_normals: every face normal reverses",
            all(tuple(-c for c in b) == a for a, b in zip(normals_before, normals_after)))

    name = fresh_cube(h, "CubeRecalc")
    h.call("mesh_select", {"name": name, "mode": "FACE", "by": "all"})
    h.call("mesh_edit", {"name": name, "ops": [{"op": "flip_normals"}, {"op": "recalc_normals"}]})
    ob = bpy.data.objects[name]
    h.check("mesh_edit recalc_normals: restores consistent outward normals after a flip",
            all(round(p.normal.dot(p.center), 3) > 0 for p in ob.data.polygons))

    name = fresh_cube(h, "CubeDissolveFaces")
    h.call("mesh_select", {"name": name, "mode": "FACE", "indices": [2, 5]})  # two adjacent side faces
    h.call("mesh_edit", {"name": name, "ops": [{"op": "dissolve", "dissolve_mode": "faces"}]})
    ob = bpy.data.objects[name]
    h.check("mesh_edit dissolve (faces): two adjacent quads merge into one hexagon",
            len(ob.data.polygons) == 5 and len(ob.data.edges) == 11)

    h.call("mesh_create", {"name": "DissolveEdgesMesh", "verts": [
        [0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0], [2, 0, 0], [2, 1, 0],
    ], "faces": [[0, 1, 2, 3], [1, 4, 5, 2]]})
    h.call("mesh_select", {"name": "DissolveEdgesMesh", "mode": "EDGE", "by": "bbox",
                            "bbox_min": [0.9, -1, -1], "bbox_max": [1.1, 2, 1]})
    h.call("mesh_edit", {"name": "DissolveEdgesMesh", "ops": [{"op": "dissolve", "dissolve_mode": "edges"}]})
    ob = bpy.data.objects["DissolveEdgesMesh"]
    h.check("mesh_edit dissolve (edges): shared edge removed, two quads merge into one hexagon",
            len(ob.data.polygons) == 1 and len(ob.data.edges) == 6)

    h.call("mesh_create", {"name": "DissolveLimitedMesh",
                            "verts": [[0, 0, 0], [2, 0, 0], [2, 2, 0], [0, 2, 0]],
                            "faces": [[0, 1, 2], [0, 2, 3]]})
    h.call("mesh_select", {"name": "DissolveLimitedMesh", "mode": "EDGE", "by": "all"})
    h.call("mesh_edit", {"name": "DissolveLimitedMesh", "ops": [
        {"op": "dissolve", "dissolve_mode": "limited", "angle_limit": 5},
    ]})
    ob = bpy.data.objects["DissolveLimitedMesh"]
    h.check("mesh_edit dissolve (limited): coplanar diagonal removed, 2 triangles become 1 quad",
            len(ob.data.polygons) == 1 and len(ob.data.edges) == 4)

    h.errors("mesh_edit: unknown op rejected", "mesh_edit", {"name": "RawCube", "ops": [{"op": "not_a_real_op"}]},
             "unknown op")
    h.call("mesh_select", {"name": "RawCube", "mode": "FACE", "by": "none"})
    h.errors("mesh_edit: extrude with nothing selected errors",
             "mesh_edit", {"name": "RawCube", "ops": [{"op": "extrude", "distance": 1}]}, "no faces selected")
    h.errors("mesh_edit: unknown object errors",
             "mesh_edit", {"name": "Nope404", "ops": [{"op": "extrude"}]}, "not found")

    # -- mesh_transform_selection ----------------------------------------------
    h.call("obj_create_primitive", {"type": "cube", "name": "XformSelCube"})
    h.call("mesh_select", {"name": "XformSelCube", "mode": "VERT", "by": "all"})
    h.call("mesh_transform_selection", {"name": "XformSelCube", "translate": [5, 0, 0]})
    ob = bpy.data.objects["XformSelCube"]
    xs = sorted(round(v.co.x, 4) for v in ob.data.vertices)
    h.check("mesh_transform_selection translate: every vertex moved by the vector",
            xs == [4.0, 4.0, 4.0, 4.0, 6.0, 6.0, 6.0, 6.0])

    h.call("obj_create_primitive", {"type": "cube", "name": "XformRotCube"})
    h.call("mesh_select", {"name": "XformRotCube", "mode": "VERT", "by": "all"})
    h.call("mesh_transform_selection", {
        "name": "XformRotCube", "rotate_angle": 90, "rotate_axis": [0, 0, 1], "pivot": [0, 0, 0],
    })
    ob = bpy.data.objects["XformRotCube"]
    h.check("mesh_transform_selection rotate: local-space 90deg about Z maps (1,-1,-1) to (1,1,-1)-ish",
            any(round(v.co.x, 3) == 1.0 and round(v.co.y, 3) == 1.0 and round(v.co.z, 3) == -1.0
                for v in ob.data.vertices))

    h.call("obj_create_primitive", {"type": "cube", "name": "XformScaleCube"})
    h.call("mesh_select", {"name": "XformScaleCube", "mode": "VERT", "by": "all"})
    h.call("mesh_transform_selection", {"name": "XformScaleCube", "scale": [2, 1, 1], "pivot": [0, 0, 0]})
    ob = bpy.data.objects["XformScaleCube"]
    h.check("mesh_transform_selection scale: local-space X extent doubles",
            round(max(abs(v.co.x) for v in ob.data.vertices), 4) == 2.0)

    h.call("obj_create_primitive", {"type": "cube", "name": "XformGlobalCube"})
    h.call("obj_update", {"name": "XformGlobalCube", "rotation": [0, 0, math.radians(45)], "scale": [1, 2, 1]})
    h.call("mesh_select", {"name": "XformGlobalCube", "mode": "VERT", "by": "all"})
    xob = bpy.data.objects["XformGlobalCube"]
    before = [tuple(v.co) for v in xob.data.vertices]
    h.call("mesh_transform_selection",
           {"name": "XformGlobalCube", "space": "global", "rotate_angle": 90, "rotate_axis": [0, 0, 1]})
    after = [tuple(v.co) for v in xob.data.vertices]
    # A 90-degree global rotation on a rotated+scaled object must actually move the
    # verts. "It ran" would pass even if the transform silently did nothing.
    moved = sum(1 for b, a in zip(before, after)
                if any(abs(bi - ai) > 1e-4 for bi, ai in zip(b, a)))
    h.check("mesh_transform_selection: global rotate actually moved the verts",
            moved == len(before))

    h.errors("mesh_transform_selection: no verb given errors",
             "mesh_transform_selection", {"name": "RawCube"}, "translate")
    h.call("mesh_select", {"name": "RawCube", "mode": "VERT", "by": "none"})
    h.errors("mesh_transform_selection: nothing selected errors",
             "mesh_transform_selection", {"name": "RawCube", "translate": [1, 0, 0]}, "selected")

    # -- mesh_mark ---------------------------------------------------------
    name = fresh_cube(h, "CubeMark")
    h.call("mesh_select", {"name": name, "mode": "EDGE", "by": "all"})
    h.call("mesh_mark", {"name": name, "seam": True, "sharp": True, "crease": 0.7})
    ob = bpy.data.objects[name]
    h.check("mesh_mark seam: every edge.use_seam is True", all(e.use_seam for e in ob.data.edges))
    h.check("mesh_mark sharp: every edge.use_edge_sharp is True (bmesh smooth=False -> sharp)",
            all(e.use_edge_sharp for e in ob.data.edges))
    crease_attr = ob.data.attributes.get("crease_edge")
    h.check("mesh_mark crease: crease_edge attribute exists and is clamped/stored correctly",
            crease_attr is not None and all(abs(d.value - 0.7) < 1e-5 for d in crease_attr.data))

    h.call("mesh_mark", {"name": name, "crease": 5.0})  # out of [0,1], must clamp
    ob = bpy.data.objects[name]
    crease_attr = ob.data.attributes.get("crease_edge")
    h.check("mesh_mark crease: values above 1.0 are clamped to 1.0",
            all(abs(d.value - 1.0) < 1e-5 for d in crease_attr.data))

    h.call("mesh_mark", {"name": name, "seam": False})
    h.check("mesh_mark seam=False clears it", all(not e.use_seam for e in bpy.data.objects[name].data.edges))

    h.errors("mesh_mark: no flag given errors", "mesh_mark", {"name": name}, "seam")
    h.call("mesh_select", {"name": name, "mode": "EDGE", "by": "none"})
    h.errors("mesh_mark: nothing selected errors", "mesh_mark", {"name": name, "seam": True}, "selected")
    h.errors("mesh_mark: unknown object errors", "mesh_mark", {"name": "Nope404", "seam": True}, "not found")
