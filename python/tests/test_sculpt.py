"""Sculpt/remesh tool tests -- `src/tools/sculpt.rs`.

Only whole-mesh operators are covered here: brush strokes need a real event loop
and cannot run headless at all. Two operators this file deliberately does NOT call
are `bpy.ops.paint.mask_flood_fill` and `bpy.ops.sculpt.symmetrize` -- both were
found live to crash Blender outright (EXCEPTION_ACCESS_VIOLATION inside their
sculpt-undo push) when run in `--background`, so `sculpt_mask`/`sculpt_symmetrize`
route around them entirely (direct `.sculpt_mask` attribute write / bmesh.ops).
Calling either real operator from this test file would take Blender down with it,
not just fail an assertion -- so ground truth here is read from the *effects* those
tools produce (mesh attribute values, vertex positions), never from re-running the
crashing operators to "double check".

Every assertion reads ground truth straight off `bpy`, and every tool leaves the
object back in OBJECT mode -- checked explicitly after each sculpt-mode-touching
call.
"""

import bpy


def cube_verts_faces():
    """Same hand-built, outward-normal cube as test_modelling.py."""
    verts = [
        [-1, -1, -1], [1, -1, -1], [1, 1, -1], [-1, 1, -1],
        [-1, -1, 1], [1, -1, 1], [1, 1, 1], [-1, 1, 1],
    ]
    faces = [
        [0, 3, 2, 1], [4, 5, 6, 7], [0, 1, 5, 4], [2, 3, 7, 6], [0, 4, 7, 3], [1, 2, 6, 5],
    ]
    return verts, faces


def fresh_cube(h, name):
    verts, faces = cube_verts_faces()
    r = h.call("mesh_create", {"name": name, "verts": verts, "faces": faces})
    return r["name"]


def run(h):
    h.section("sculpt")
    run_remesh(h)
    run_symmetrize(h)
    run_mask(h)
    run_guards(h)


# ============================================================================
# sculpt_remesh
# ============================================================================

def run_remesh(h):
    # -- voxel remesh: small size increases the count, large size decreases it --
    name = fresh_cube(h, "RemeshSmall")
    before = len(bpy.data.objects[name].data.vertices)
    r = h.call("sculpt_remesh", {"name": name, "method": "voxel", "voxel_size": 0.08})
    ob = bpy.data.objects[name]
    h.check("sculpt_remesh voxel small size: vertex count grows a lot (8 -> hundreds+)",
            len(ob.data.vertices) > before + 100)
    h.check("sculpt_remesh voxel: tool-reported counts match ground truth",
            r["vertices"] == len(ob.data.vertices) and r["polygons"] == len(ob.data.polygons))
    h.check("sculpt_remesh voxel: object is back in OBJECT mode", ob.mode == "OBJECT")
    h.check("sculpt_remesh voxel: mesh.remesh_voxel_size ground truth", abs(ob.data.remesh_voxel_size - 0.08) < 1e-6)

    name2 = fresh_cube(h, "RemeshLarge")
    r2 = h.call("sculpt_remesh", {"name": name2, "method": "voxel", "voxel_size": 5.0})
    ob2 = bpy.data.objects[name2]
    h.check("sculpt_remesh voxel large size: mesh survives as one low-poly blob (few verts)",
            0 < len(ob2.data.vertices) < 200)
    h.check("sculpt_remesh voxel large vs small: large voxel size produces far fewer verts",
            len(ob2.data.vertices) < len(bpy.data.objects["RemeshSmall"].data.vertices))

    # -- voxel adaptivity: just needs to run and persist on the mesh --
    name3 = fresh_cube(h, "RemeshAdaptive")
    h.call("sculpt_remesh", {"name": name3, "method": "voxel", "voxel_size": 0.1, "voxel_adaptivity": 0.5})
    ob3 = bpy.data.objects[name3]
    h.check("sculpt_remesh voxel_adaptivity: ground truth persisted",
            abs(ob3.data.remesh_voxel_adaptivity - 0.5) < 1e-6)

    # -- quadriflow remesh: produces a real quad-dominant mesh at the target count --
    name4 = fresh_cube(h, "RemeshQuadriflow")
    r4 = h.call("sculpt_remesh", {"name": name4, "method": "quadriflow", "target_faces": 40})
    ob4 = bpy.data.objects[name4]
    h.check("sculpt_remesh quadriflow: produced real geometry, roughly near the target",
            10 <= len(ob4.data.polygons) <= 200)
    h.check("sculpt_remesh quadriflow: tool-reported polygon count matches ground truth",
            r4["polygons"] == len(ob4.data.polygons))
    h.check("sculpt_remesh quadriflow: object is back in OBJECT mode", ob4.mode == "OBJECT")

    name5 = fresh_cube(h, "RemeshQuadriflowRatio")
    h.call("sculpt_remesh", {"name": name5, "method": "quadriflow", "target_ratio": 2.0})
    h.check("sculpt_remesh quadriflow target_ratio: runs and produces geometry",
            len(bpy.data.objects[name5].data.polygons) > 0)


# ============================================================================
# sculpt_symmetrize
# ============================================================================

def run_symmetrize(h):
    import bmesh

    # Build an asymmetric mesh: a cube with the entire +X half deleted.
    verts, faces = cube_verts_faces()
    r = h.call("mesh_create", {"name": "AsymCube", "verts": verts, "faces": faces})
    name = r["name"]
    ob = bpy.data.objects[name]
    bm = bmesh.new()
    bm.from_mesh(ob.data)
    bm.verts.ensure_lookup_table()
    to_delete = [v for v in bm.verts if v.co.x > 0]
    bmesh.ops.delete(bm, geom=to_delete, context='VERTS')
    bm.to_mesh(ob.data)
    ob.data.update()
    bm.free()

    ob = bpy.data.objects[name]
    xs_before = [round(v.co.x, 4) for v in ob.data.vertices]
    h.check("sculpt_symmetrize setup: mesh is genuinely asymmetric before the call",
            all(x <= 0.0 for x in xs_before) and any(x < 0 for x in xs_before))

    r = h.call("sculpt_symmetrize", {"name": name, "direction": "NEGATIVE_X"})
    ob = bpy.data.objects[name]
    xs_after = [round(v.co.x, 4) for v in ob.data.vertices]
    h.check("sculpt_symmetrize: mesh now has vertices on both sides of x=0",
            any(x < -1e-4 for x in xs_after) and any(x > 1e-4 for x in xs_after))
    # Symmetry check: every vertex has a mirrored counterpart at (-x, y, z).
    coords = {(round(v.co.x, 3), round(v.co.y, 3), round(v.co.z, 3)) for v in ob.data.vertices}
    mirrored_ok = all((-x, y, z) in coords for (x, y, z) in coords)
    h.check("sculpt_symmetrize: every vertex has a mirror counterpart across x=0 (true symmetry)",
            mirrored_ok)
    h.check("sculpt_symmetrize: tool-reported vertex count matches ground truth",
            r["vertices"] == len(ob.data.vertices))
    h.check("sculpt_symmetrize: object is back in OBJECT mode (never entered a sculpt mode at all)",
            ob.mode == "OBJECT")

    # Other axes/directions should at least run cleanly.
    name2 = fresh_cube(h, "SymCubeY")
    h.call("sculpt_symmetrize", {"name": name2, "direction": "POSITIVE_Y"})
    h.check("sculpt_symmetrize POSITIVE_Y: runs on an already-symmetric mesh without error",
            len(bpy.data.objects[name2].data.vertices) > 0)


# ============================================================================
# sculpt_mask
# ============================================================================

def run_mask(h):
    name = fresh_cube(h, "MaskCube")
    ob = bpy.data.objects[name]
    h.check("sculpt_mask setup: no .sculpt_mask attribute exists yet on a fresh mesh",
            ob.data.attributes.get(".sculpt_mask") is None)

    r = h.call("sculpt_mask", {"name": name, "value": 1.0})
    ob = bpy.data.objects[name]
    mask_attr = ob.data.attributes.get(".sculpt_mask")
    h.check("sculpt_mask value=1.0: attribute now exists", mask_attr is not None)
    h.check("sculpt_mask value=1.0: every vertex masked (ground truth)",
            all(abs(d.value - 1.0) < 1e-6 for d in mask_attr.data))
    h.check("sculpt_mask value=1.0: tool reports created_attribute=True the first time",
            r["created_attribute"] is True)
    h.check("sculpt_mask: object never left OBJECT mode", ob.mode == "OBJECT")

    r2 = h.call("sculpt_mask", {"name": name, "value": 0.0})
    ob = bpy.data.objects[name]
    mask_attr = ob.data.attributes.get(".sculpt_mask")
    h.check("sculpt_mask value=0.0: every vertex cleared (ground truth, values actually changed)",
            all(abs(d.value - 0.0) < 1e-6 for d in mask_attr.data))
    h.check("sculpt_mask value=0.0: tool reports created_attribute=False the second time",
            r2["created_attribute"] is False)

    r3 = h.call("sculpt_mask", {"name": name, "value": 5.0})
    ob = bpy.data.objects[name]
    mask_attr = ob.data.attributes.get(".sculpt_mask")
    h.check("sculpt_mask: out-of-range value is clamped to 1.0, not rejected",
            r3["value"] == 1.0 and all(abs(d.value - 1.0) < 1e-6 for d in mask_attr.data))


# ============================================================================
# Guards
# ============================================================================

def run_guards(h):
    h.errors("sculpt_remesh: unknown object errors",
             "sculpt_remesh", {"name": "Nope404"}, "not found")

    bpy.ops.object.empty_add()
    empty = bpy.context.active_object
    empty.name = "SculptGuardEmpty"
    h.errors("sculpt_remesh: non-mesh object rejected",
             "sculpt_remesh", {"name": "SculptGuardEmpty"}, "not found")
    h.errors("sculpt_symmetrize: non-mesh object rejected",
             "sculpt_symmetrize", {"name": "SculptGuardEmpty"}, "not found")
    h.errors("sculpt_mask: non-mesh object rejected",
             "sculpt_mask", {"name": "SculptGuardEmpty", "value": 1.0}, "not found")

    name = fresh_cube(h, "GuardCube")
    h.errors("sculpt_remesh: unknown method rejected",
             "sculpt_remesh", {"name": name, "method": "brush"}, "voxel")
    h.errors("sculpt_remesh: voxel_size <= 0 rejected",
             "sculpt_remesh", {"name": name, "method": "voxel", "voxel_size": 0.0}, "voxel_size")
    h.errors("sculpt_remesh: quadriflow params on method=voxel rejected",
             "sculpt_remesh", {"name": name, "method": "voxel", "target_faces": 100}, "quadriflow")
    h.errors("sculpt_remesh: voxel params on method=quadriflow rejected",
             "sculpt_remesh", {"name": name, "method": "quadriflow", "voxel_size": 0.1}, "voxel")
    h.errors("sculpt_remesh: more than one target_* rejected",
             "sculpt_remesh",
             {"name": name, "method": "quadriflow", "target_faces": 10, "target_ratio": 0.5},
             "at most one")

    h.errors("sculpt_symmetrize: unknown direction rejected",
             "sculpt_symmetrize", {"name": name, "direction": "SIDEWAYS"}, "direction")
    h.errors("sculpt_symmetrize: negative merge_distance rejected",
             "sculpt_symmetrize", {"name": name, "merge_distance": -1.0}, "merge_distance")

    # An empty mesh (no polygons) legitimately declines voxel_remesh -- Blender
    # returns {'CANCELLED'}, not a raise, so op_ok is what must turn this into Err.
    import bmesh
    empty_mesh_r = h.call("mesh_create", {
        "name": "EmptyMeshBase", "verts": [[0, 0, 0], [1, 0, 0], [2, 0, 0]],
    })
    h.errors("sculpt_remesh: voxel remesh on a mesh with zero faces errors (CANCELLED caught by op_ok)",
             "sculpt_remesh", {"name": empty_mesh_r["name"], "method": "voxel", "voxel_size": 0.1},
             "did nothing")
