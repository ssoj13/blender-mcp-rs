---
area: api-ops
group: poselib
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.poselib

- `bpy.ops.poselib.apply_pose_asset(asset_library_type: enum{ALL, LOCAL, ESSENTIALS, ONLINE_ESSENTIALS, CUSTOM}, asset_library_identifier: string, relative_asset_identifier: string, blend_factor: float, flipped: boolean)` — Apply the given Pose Action to the rig
- `bpy.ops.poselib.asset_delete()` — Delete the selected Pose Asset
- `bpy.ops.poselib.asset_modify(mode: enum{ADJUST, REPLACE, ADD, REMOVE})` — Update the selected pose asset in the asset library from the currently selected bones. The mode defines how the asset is updated
- `bpy.ops.poselib.blend_pose_asset(asset_library_type: enum{ALL, LOCAL, ESSENTIALS, ONLINE_ESSENTIALS, CUSTOM}, asset_library_identifier: string, relative_asset_identifier: string, blend_factor: float, flipped: boolean, release_confirm: boolean)` — Blend the given Pose Action to the rig
- `bpy.ops.poselib.copy_as_asset()` — Create a new pose asset on the clipboard, to be pasted into an Asset Browser
- `bpy.ops.poselib.create_pose_asset(pose_name: string, asset_library_reference: enum{}, catalog_path: string)` — Create a new asset from the selected bones in the scene
- `bpy.ops.poselib.paste_asset()` — Paste the Asset that was previously copied using Copy As Asset
- `bpy.ops.poselib.pose_asset_select_bones(select: boolean, flipped: boolean)` — Select those bones that are used in this pose
- `bpy.ops.poselib.restore_previous_action()` — Switch back to the previous Action, after creating a pose asset