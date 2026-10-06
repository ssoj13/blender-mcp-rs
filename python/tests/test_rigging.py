"""Rigging / attributes domain: rig.rs (constraints, drivers, armatures), attribute.rs
(vertex groups, colour attributes, named attributes), custom_prop.rs (ID properties).

Every assertion reads the real Blender state back off `bpy`, not the tool's own
echoed JSON -- a broken tool can still return a plausible-looking dict.
"""

import bpy


def run(h):
    h.section("rigging")

    # ------------------------------------------------------------------
    # rig_create_armature / rig_add_bone / rig_remove_bone / rig_update_bone
    # ------------------------------------------------------------------
    h.call("rig_create_armature", {"name": "Rig", "add_root_bone": True})
    rig = bpy.data.objects["Rig"]
    h.check("rig_create_armature made an ARMATURE object", rig.type == "ARMATURE")
    h.check("root bone count read off the armature", len(rig.data.bones) == 1)
    h.check("root bone is named Root", "Root" in rig.data.bones)

    h.call("rig_add_bone", {
        "name": "Rig", "bone_name": "Child", "parent": "Root",
        "head": [0, 0, 1], "tail": [0, 0, 2], "connected": True, "roll": 0.25,
    })
    h.check("rig_add_bone: bone exists in edit_bones (armature.data.bones once "
            "back in Object mode)", "Child" in rig.data.bones)
    h.check("rig_add_bone: parent wired up", rig.data.bones["Child"].parent.name == "Root")

    h.call("rig_update_bone", {"name": "Rig", "bone_name": "Child", "tail": [0, 0, 3]})
    h.check("rig_update_bone: tail persists",
            abs(rig.data.bones["Child"].tail_local.z - 3.0) < 1e-5)

    h.call("rig_add_bone", {"name": "Rig", "bone_name": "Doomed", "parent": "Root"})
    h.check("bone to be removed exists first", "Doomed" in rig.data.bones)
    h.call("rig_remove_bone", {"name": "Rig", "bone_name": "Doomed"})
    h.check("rig_remove_bone: gone from edit_bones", "Doomed" not in rig.data.bones)
    h.errors("rig_remove_bone: missing bone errors", "rig_remove_bone",
             {"name": "Rig", "bone_name": "NoSuchBone"}, "not found")

    # ------------------------------------------------------------------
    # rig_create_biped -- bone count read off the armature, not the echo
    # ------------------------------------------------------------------
    h.call("rig_create_biped", {"name": "Biped", "height": 2.0})
    biped = bpy.data.objects["Biped"]
    h.check("rig_create_biped: bone count read off the armature",
            len(biped.data.bones) > 0 and len(biped.data.bones) == 19)
    for expect in ("root", "spine", "chest", "neck", "head",
                   "shoulder.L", "upper_arm.L", "forearm.L", "hand.L",
                   "thigh.R", "shin.R", "foot.R"):
        h.check(f"biped has bone `{expect}`", expect in biped.data.bones)

    # ------------------------------------------------------------------
    # rig_pose_bone / rig_get_pose
    # ------------------------------------------------------------------
    h.call("rig_pose_bone", {
        "name": "Rig", "bone_name": "Child",
        "location": [0.1, 0.2, 0.3], "rotation": [0, 0, 0.5],
    })
    pb = rig.pose.bones["Child"]
    h.check("rig_pose_bone: location persists",
            abs(pb.location.x - 0.1) < 1e-6 and abs(pb.location.y - 0.2) < 1e-6)
    h.check("rig_pose_bone: rotation persists", abs(pb.rotation_euler.z - 0.5) < 1e-6)
    # The pose must actually MOVE, not just have the euler field written. A bone
    # defaults to rotation_mode QUATERNION, so writing rotation_euler without switching
    # the mode is silently ignored — the field reads back 0.5 while the pose stays put.
    # Checking matrix_basis is what catches that; the euler read above does not.
    bpy.context.view_layer.update()
    h.check("rig_pose_bone: pose actually moved",
            any(abs(pb.matrix_basis[i][j] - (1.0 if i == j else 0.0)) > 1e-4
                for i in range(4) for j in range(4)))

    pose = h.call("rig_get_pose", {"name": "Rig"})
    h.check("rig_get_pose: reports every pose bone", len(pose["bones"]) == len(rig.pose.bones))
    child_entry = next(b for b in pose["bones"] if b["name"] == "Child")
    h.check("rig_get_pose: echoes the pose we just set",
            abs(child_entry["location"][0] - 0.1) < 1e-6)

    # ------------------------------------------------------------------
    # rig_add_ik -- the pose bone must carry a real IK constraint
    # ------------------------------------------------------------------
    h.call("rig_add_bone", {"name": "Rig", "bone_name": "IkTip", "parent": "Child", "connected": True})
    h.call("rig_add_ik", {
        "name": "Rig", "bone_name": "IkTip", "chain_length": 2,
        "target": "Rig", "subtarget": "Root",
    })
    ik_bone = rig.pose.bones["IkTip"]
    ik_cons = [c for c in ik_bone.constraints if c.type == "IK"]
    h.check("rig_add_ik: pose bone has an IK constraint", len(ik_cons) == 1)
    h.check("rig_add_ik: IK targets the requested armature/bone",
            ik_cons and ik_cons[0].target == rig and ik_cons[0].subtarget == "Root")

    # ------------------------------------------------------------------
    # con_add on pose bones -- the actual payoff: real constraint types,
    # generic properties applied through RNA, subtarget aiming at a bone.
    # ------------------------------------------------------------------
    con_r = h.call("con_add", {
        "object": "Rig", "bone": "Child", "type": "copy_rotation",
        "target": "Rig", "subtarget": "Root",
        "properties": {"influence": 0.5, "use_x": False},
    })
    h.check("con_add: lowercase type is upper-cased", con_r["constraint"]["type"] == "COPY_ROTATION")
    child_con = rig.pose.bones["Child"].constraints[0]
    h.check("con_add: constraint really landed on the pose bone", child_con.type == "COPY_ROTATION")
    h.check("con_add: influence applied via properties", abs(child_con.influence - 0.5) < 1e-6)
    h.check("con_add: use_x applied via properties", child_con.use_x is False)
    h.check("con_add: subtarget aims at the requested bone", child_con.subtarget == "Root")
    h.check("con_add: target resolved to the armature object", child_con.target == rig)

    # A bad property name must error and list the constraint's REAL settings,
    # and must not leave a half-made constraint behind.
    before = len(ik_bone.constraints)
    h.errors("con_add: unknown property lists real settings", "con_add", {
        "object": "Rig", "bone": "IkTip", "type": "DAMPED_TRACK",
        "properties": {"not_a_real_setting": 1},
    }, "not_a_real_setting")
    h.check("con_add: failed call left no half-made constraint behind",
            len(ik_bone.constraints) == before)

    h.errors("con_add: unknown constraint type lists the real enum", "con_add", {
        "object": "Rig", "bone": "Child", "type": "NOT_A_REAL_TYPE",
    }, "NOT_A_REAL_TYPE")

    # con_list / con_update / con_remove
    h.call("con_add", {"object": "Rig", "bone": "Child", "type": "LIMIT_ROTATION",
                        "con_name": "Limiter", "properties": {"use_limit_x": True, "min_x": -1.0}})
    listed = h.call("con_list", {"object": "Rig", "bone": "Child"})
    h.check("con_list: sees both constraints on the bone", len(listed["constraints"]) == 2)

    h.call("con_update", {"object": "Rig", "bone": "Child", "con_name": "Limiter",
                           "properties": {"min_x": -2.0}})
    h.check("con_update: property change persists",
            abs(rig.pose.bones["Child"].constraints["Limiter"].min_x - (-2.0)) < 1e-6)
    h.errors("con_update: unknown constraint name lists what the bone has",
             "con_update", {"object": "Rig", "bone": "Child", "con_name": "NopeCon"}, "Limiter")

    before_count = len(rig.pose.bones["Child"].constraints)
    h.call("con_remove", {"object": "Rig", "bone": "Child", "names": ["Limiter"]})
    h.check("con_remove: constraint actually gone",
            len(rig.pose.bones["Child"].constraints) == before_count - 1)

    # con_add on the OBJECT itself (no `bone`) -- the other owner shape
    bpy.ops.mesh.primitive_cube_add(location=(10, 0, 0))
    tracker = bpy.context.active_object
    tracker.name = "Tracker"
    h.call("con_add", {"object": "Tracker", "type": "TRACK_TO", "target": "Rig"})
    h.check("con_add on an object (no bone) lands in obj.constraints",
            len(tracker.constraints) == 1 and tracker.constraints[0].type == "TRACK_TO")

    h.errors("con_add: bone on a non-armature object errors", "con_add",
             {"object": "Tracker", "bone": "Whatever", "type": "COPY_LOCATION"}, "armature")

    # ------------------------------------------------------------------
    # Drivers -- read back off obj.animation_data.drivers, not Action.fcurves
    # (Blender 5 removed the flat Action.fcurves attribute entirely).
    # ------------------------------------------------------------------
    bpy.ops.mesh.primitive_cube_add(location=(11, 0, 0))
    drv_obj = bpy.context.active_object
    drv_obj.name = "DrvObj"

    h.call("drv_add", {
        "name": "DrvObj", "data_path": "location", "index": 0,
        "driver_type": "SCRIPTED", "expression": "2.0",
    })
    h.check("drv_add: driver exists on animation_data.drivers",
            drv_obj.animation_data is not None and len(drv_obj.animation_data.drivers) == 1)
    h.check("drv_add: expression persists",
            drv_obj.animation_data.drivers[0].driver.expression == "2.0")

    # index -1 ("all") on a vector property must configure every resulting
    # F-Curve, not crash on driver_add returning a list.
    h.call("drv_add", {
        "name": "DrvObj", "data_path": "scale", "index": -1,
        "driver_type": "SCRIPTED", "expression": "1.5",
    })
    scale_drivers = [d for d in drv_obj.animation_data.drivers if d.data_path == "scale"]
    h.check("drv_add index=-1 configures every component of a vector property",
            len(scale_drivers) == 3
            and all(d.driver.expression == "1.5" for d in scale_drivers))

    h.call("drv_remove", {"name": "DrvObj", "data_path": "location", "index": 0})
    h.check("drv_remove: location driver gone",
            not any(d.data_path == "location" for d in drv_obj.animation_data.drivers))

    # drv_create with a TRANSFORMS variable
    bpy.ops.mesh.primitive_cube_add(location=(12, 0, 0))
    drv_target = bpy.context.active_object
    drv_target.name = "DrvTarget"
    h.call("drv_create", {
        "target": "DrvTarget", "target_prop": "location", "prop_index": 1,
        "expression": "var * 2", "variables": [
            {"name": "var", "type": "TRANSFORMS", "source": "DrvObj", "transform_type": "LOC_X"},
        ],
    })
    dt_drivers = [d for d in drv_target.animation_data.drivers if d.array_index == 1]
    h.check("drv_create: driver landed on the requested index", len(dt_drivers) == 1)
    h.check("drv_create: expression persists", dt_drivers[0].driver.expression == "var * 2")
    h.check("drv_create: variable created", len(dt_drivers[0].driver.variables) == 1)

    # drv_copy onto a VECTOR property must not crash: driver_add(path) with no
    # index defaults to -1 and returns a list of F-Curves for an array property,
    # not a single one.
    h.call("drv_copy", {
        "source": "DrvObj", "source_prop": "scale",
        "target": "DrvTarget", "target_prop": "scale",
    })
    h.check("drv_copy onto a vector property lands a real driver",
            any(d.data_path == "scale" for d in drv_target.animation_data.drivers))
    h.errors("drv_copy: no driver at that path errors", "drv_copy", {
        "source": "DrvObj", "source_prop": "rotation_euler",
        "target": "DrvTarget", "target_prop": "location",
    }, "not found")

    # ------------------------------------------------------------------
    # Vertex groups -- the payoff: a real MASK modifier actually masks.
    # ------------------------------------------------------------------
    bpy.ops.mesh.primitive_cube_add(location=(0, 5, 0))
    vg_obj = bpy.context.active_object
    vg_obj.name = "VgMesh"
    vert_count_before = len(vg_obj.data.vertices)
    h.check("cube starts with 8 verts", vert_count_before == 8)

    h.call("vgroup_add", {"object": "VgMesh", "name": "Keep"})
    h.check("vgroup_add: group exists on the object", "Keep" in vg_obj.vertex_groups)
    listed_vg = h.call("vgroup_list", {"object": "VgMesh"})
    h.check("vgroup_list: sees it", any(g["name"] == "Keep" for g in listed_vg["groups"]))

    h.call("vgroup_set", {"object": "VgMesh", "name": "Keep", "indices": [0, 1, 2, 3], "weight": 1.0})
    h.call("mod_add", {"name": "VgMesh", "type": "MASK", "mod_name": "Masker",
                        "properties": {"vertex_group": "Keep"}})
    h.call("mod_apply", {"name": "VgMesh", "modifier": "Masker"})
    h.check("vgroup_set + MASK modifier actually masks: vert count dropped",
            len(vg_obj.data.vertices) == 4 < vert_count_before)

    # per-index weights (hand-painted falloff)
    bpy.ops.mesh.primitive_cube_add(location=(1, 5, 0))
    vg_obj2 = bpy.context.active_object
    vg_obj2.name = "VgMesh2"
    h.call("vgroup_add", {"object": "VgMesh2", "name": "Falloff"})
    h.call("vgroup_set", {
        "object": "VgMesh2", "name": "Falloff",
        "indices": [0, 1, 2], "weights": [0.1, 0.5, 1.0],
    })
    vg2 = vg_obj2.vertex_groups["Falloff"]
    h.check("vgroup_set per-index weights: vertex 0", abs(vg2.weight(0) - 0.1) < 1e-6)
    h.check("vgroup_set per-index weights: vertex 2", abs(vg2.weight(2) - 1.0) < 1e-6)
    h.errors("vgroup_set: weight and weights together is rejected", "vgroup_set", {
        "object": "VgMesh2", "name": "Falloff", "indices": [0], "weight": 1.0, "weights": [0.5],
    })

    # vgroup_get for a vertex not in the group must not blow up
    got = h.call("vgroup_get", {"object": "VgMesh2", "name": "Falloff", "indices": [0, 7]})
    h.check("vgroup_get: does not raise for a non-member vertex", True)
    h.check("vgroup_get: member vertex 0 present", any(w["index"] == 0 for w in got["weights"]))

    h.call("vgroup_rename", {"object": "VgMesh2", "name": "Falloff", "new_name": "Renamed"})
    h.check("vgroup_rename persists", "Renamed" in vg_obj2.vertex_groups
            and "Falloff" not in vg_obj2.vertex_groups)
    h.call("vgroup_remove", {"object": "VgMesh2", "name": "Renamed"})
    h.check("vgroup_remove: group gone", "Renamed" not in vg_obj2.vertex_groups)
    h.errors("vgroup_remove: unknown group lists the real groups", "vgroup_remove",
             {"object": "VgMesh2", "name": "NoSuchGroup"}, "none")

    # ------------------------------------------------------------------
    # Colour attributes
    # ------------------------------------------------------------------
    bpy.ops.mesh.primitive_cube_add(location=(2, 5, 0))
    ca_obj = bpy.context.active_object
    ca_obj.name = "ColMesh"
    n_verts = len(ca_obj.data.vertices)

    h.call("color_attr_add", {"object": "ColMesh", "name": "Paint"})
    h.check("color_attr_add: attribute exists", "Paint" in ca_obj.data.color_attributes)
    h.check("color_attr_add: defaults to FLOAT_COLOR/POINT",
            ca_obj.data.color_attributes["Paint"].data_type == "FLOAT_COLOR"
            and ca_obj.data.color_attributes["Paint"].domain == "POINT")

    colors = [[1.0, 0.0, 0.0, 1.0]] * n_verts
    h.call("color_attr_set", {"object": "ColMesh", "name": "Paint", "colors": colors})
    written = ca_obj.data.color_attributes["Paint"].data[0].color
    h.check("color_attr_set: colour actually written",
            abs(written[0] - 1.0) < 1e-5 and abs(written[1] - 0.0) < 1e-5)

    listed_ca = h.call("color_attr_list", {"object": "ColMesh"})
    h.check("color_attr_list: sees the attribute", any(a["name"] == "Paint" for a in listed_ca["attributes"]))

    h.call("color_attr_remove", {"object": "ColMesh", "name": "Paint"})
    h.check("color_attr_remove: gone", "Paint" not in ca_obj.data.color_attributes)

    # ------------------------------------------------------------------
    # Generic named attributes
    # ------------------------------------------------------------------
    bpy.ops.mesh.primitive_cube_add(location=(3, 5, 0))
    at_obj = bpy.context.active_object
    at_obj.name = "AttrMesh"

    h.call("attr_add", {"object": "AttrMesh", "name": "Heat", "type": "FLOAT", "domain": "POINT"})
    h.check("attr_add: attribute exists on the mesh", "Heat" in at_obj.data.attributes)
    h.check("attr_add: right data_type/domain",
            at_obj.data.attributes["Heat"].data_type == "FLOAT"
            and at_obj.data.attributes["Heat"].domain == "POINT")

    h.call("attr_set", {"object": "AttrMesh", "name": "Heat", "indices": [0, 1], "values": [0.25, 0.75]})
    h.check("attr_set: value 0 persists", abs(at_obj.data.attributes["Heat"].data[0].value - 0.25) < 1e-6)
    h.check("attr_set: value 1 persists", abs(at_obj.data.attributes["Heat"].data[1].value - 0.75) < 1e-6)

    h.call("attr_add", {"object": "AttrMesh", "name": "Transform", "type": "FLOAT4X4"})
    flat_matrix = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 2, 3, 4, 1]
    h.call("attr_set", {"object": "AttrMesh", "name": "Transform",
                        "indices": [0], "values": [flat_matrix]})
    stored_matrix = at_obj.data.attributes["Transform"].data[0].value
    h.check("attr_set FLOAT4X4 folds flat values into matrix columns",
            all(abs(stored_matrix[i][j] - flat_matrix[j * 4 + i]) < 1e-6
                for i in range(4) for j in range(4)))

    h.errors("attr_add: unknown data_type lists this Blender's real list", "attr_add",
             {"object": "AttrMesh", "name": "Bad", "type": "NOT_A_TYPE"}, "NOT_A_TYPE")

    listed_attrs = h.call("attr_list", {"object": "AttrMesh"})
    h.check("attr_list: sees Heat plus Blender's own built-ins",
            any(a["name"] == "Heat" for a in listed_attrs["attributes"])
            and len(listed_attrs["attributes"]) > 1)

    h.call("attr_remove", {"object": "AttrMesh", "name": "Heat"})
    h.check("attr_remove: gone", "Heat" not in at_obj.data.attributes)
    h.errors("attr_remove: unknown attribute lists what the mesh has", "attr_remove",
             {"object": "AttrMesh", "name": "NoSuchAttr"}, "no attribute")

    # ------------------------------------------------------------------
    # Custom (ID) properties -- on an object and on a pose bone.
    # ------------------------------------------------------------------
    bpy.ops.mesh.primitive_cube_add(location=(4, 5, 0))
    prop_obj = bpy.context.active_object
    prop_obj.name = "PropObj"

    h.call("prop_set", {
        "target": "objects:PropObj", "props": {"hp": 100},
        "min": 0, "max": 200, "description": "hit points",
    })
    h.check("prop_set: value on the object", prop_obj["hp"] == 100)
    ui = prop_obj.id_properties_ui("hp").as_dict()
    h.check("prop_set: INT property gets an INT range, not float",
            isinstance(ui["min"], int) and ui["min"] == 0 and ui["max"] == 200)

    h.call("rig_add_bone", {"name": "Rig", "bone_name": "Ctrl", "parent": "Root"})
    h.call("prop_set", {
        "target": "bone:Rig/Ctrl", "props": {"speed": 1.5},
        "min": 0.0, "max": 10.0,
    })
    ctrl_bone = rig.pose.bones["Ctrl"]
    h.check("prop_set: value landed on the pose bone, not the object",
            abs(ctrl_bone["speed"] - 1.5) < 1e-6 and "speed" not in rig)
    ui2 = ctrl_bone.id_properties_ui("speed").as_dict()
    h.check("prop_set: FLOAT property gets a float range",
            isinstance(ui2["max"], float) and abs(ui2["max"] - 10.0) < 1e-6)

    listed_props = h.call("prop_list", {"target": "objects:PropObj"})
    h.check("prop_list: sees hp", listed_props["props"]["hp"] == 100)

    h.call("prop_remove", {"target": "objects:PropObj", "names": ["hp"]})
    h.check("prop_remove: gone from the object", "hp" not in prop_obj.keys())
    h.errors("prop_remove: removing a missing key errors", "prop_remove",
             {"target": "objects:PropObj", "names": ["hp"]})

    h.errors("prop_set: unknown bpy.data collection lists the real ones", "prop_set",
             {"target": "not_a_collection:Whatever", "props": {"x": 1}}, "objects")
    h.errors("prop_set: malformed bone target errors", "prop_set",
             {"target": "bone:Rig", "props": {"x": 1}})
    h.errors("prop_set: unknown bone on a real armature lists its bones", "prop_set",
             {"target": "bone:Rig/NoSuchBone", "props": {"x": 1}}, "Root")

    # ------------------------------------------------------------------
    # rig_auto_weights / rig_mirror_weights -- run fine headless (no viewport
    # needed, unlike scene_screenshot).
    # ------------------------------------------------------------------
    bpy.ops.mesh.primitive_cube_add(location=(0, 0, 5))
    skin_obj = bpy.context.active_object
    skin_obj.name = "SkinMesh"
    h.call("rig_auto_weights", {"mesh": "SkinMesh", "armature": "Rig"})
    h.check("rig_auto_weights: mesh parented to the armature with an ARMATURE modifier",
            skin_obj.parent == rig
            and any(m.type == "ARMATURE" for m in skin_obj.modifiers))
    h.errors("rig_auto_weights: missing mesh errors", "rig_auto_weights",
             {"mesh": "NoSuchMesh", "armature": "Rig"}, "not found")

    # Mirrors L/R weight groups via bpy.ops.object.vertex_group_mirror. Reading the
    # effect back needs a symmetric mesh with matching .L/.R groups set up first;
    # that fixture is disproportionate here, so "ran without error" is the honest
    # check that the operator path works headless (it returns FINISHED, checked by
    # op_ok inside the tool).
    h.ran("rig_mirror_weights: runs headless without a viewport",
          "rig_mirror_weights", {"mesh": "SkinMesh"})
