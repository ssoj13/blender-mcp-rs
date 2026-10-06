---
area: api-ops
group: import_anim
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.import_anim

- `bpy.ops.import_anim.bvh(filepath: string, filter_glob: string, target: enum{ARMATURE, OBJECT}, global_scale: float, frame_start: int, use_fps_scale: boolean, update_scene_fps: boolean, update_scene_duration: boolean, use_cyclic: boolean, rotate_mode: enum{QUATERNION, NATIVE, XYZ, XZY, YXZ, YZX, ZXY, ZYX}, axis_forward: enum{X, Y, Z, -X, -Y, -Z}, axis_up: enum{X, Y, Z, -X, -Y, -Z})` — Load a BVH motion capture file