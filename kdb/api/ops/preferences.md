---
area: api-ops
group: preferences
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.preferences

- `bpy.ops.preferences.addon_disable(module: string)` — Turn off this add-on
- `bpy.ops.preferences.addon_enable(module: string)` — Turn on this add-on
- `bpy.ops.preferences.addon_expand(module: string)` — Display information and preferences for this add-on
- `bpy.ops.preferences.addon_install(overwrite: boolean, enable_on_install: boolean, target: enum{}, filepath: string, filter_folder: boolean, filter_python: boolean, filter_glob: string)` — Install an add-on
- `bpy.ops.preferences.addon_refresh()` — Scan add-on directories for new modules
- `bpy.ops.preferences.addon_remove(module: string)` — Delete the add-on from the file system
- `bpy.ops.preferences.addon_show(module: string)` — Show add-on preferences
- `bpy.ops.preferences.app_template_install(overwrite: boolean, filepath: string, filter_folder: boolean, filter_glob: string)` — Install an application template
- `bpy.ops.preferences.asset_library_add(directory: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{}, name: string, remote_url: string, type: enum{REMOTE, LOCAL})` — Add a directory to be used by the Asset Browser as source of assets
- `bpy.ops.preferences.asset_library_remove(index: int)` — Remove a path to a .blend file, so the Asset Browser will not attempt to show it anymore
- `bpy.ops.preferences.associate_blend()` — Use this installation for .blend files and to display thumbnails
- `bpy.ops.preferences.autoexec_path_add()` — Add path to exclude from auto-execution
- `bpy.ops.preferences.autoexec_path_remove(index: int)` — Remove path to exclude from auto-execution
- `bpy.ops.preferences.clear_filter()` — Clear the search filter
- `bpy.ops.preferences.copy_prev()` — Copy settings from previous version
- `bpy.ops.preferences.extension_repo_add(name: string, remote_url: string, use_access_token: boolean, access_token: string, use_sync_on_startup: boolean, use_custom_directory: boolean, custom_directory: string, type: enum{REMOTE, LOCAL})` — Add a new repository used to store extensions
- `bpy.ops.preferences.extension_repo_remove(index: int, remove_files: boolean)` — Remove an extension repository
- `bpy.ops.preferences.extension_url_drop(url: string)` — Handle dropping an extension URL
- `bpy.ops.preferences.keyconfig_activate(filepath: string)` — (undocumented operator)
- `bpy.ops.preferences.keyconfig_export(all: boolean, filepath: string, filter_folder: boolean, filter_text: boolean, filter_python: boolean)` — Export key configuration to a Python script
- `bpy.ops.preferences.keyconfig_import(filepath: string, filter_folder: boolean, filter_text: boolean, filter_python: boolean, keep_original: boolean)` — Import key configuration from a Python script
- `bpy.ops.preferences.keyconfig_remove()` — Remove key config
- `bpy.ops.preferences.keyconfig_test()` — Test key configuration for conflicts
- `bpy.ops.preferences.keyitem_add()` — Add key map item
- `bpy.ops.preferences.keyitem_remove(item_id: int)` — Remove key map item
- `bpy.ops.preferences.keyitem_restore(item_id: int)` — Restore key map item
- `bpy.ops.preferences.keymap_restore(all: boolean)` — Restore key map(s)
- `bpy.ops.preferences.reset_default_theme()` — Reset to the default theme colors
- `bpy.ops.preferences.script_directory_add(directory: string, filter_folder: boolean)` — (undocumented operator)
- `bpy.ops.preferences.script_directory_remove(index: int)` — (undocumented operator)
- `bpy.ops.preferences.start_filter()` — Start entering filter text
- `bpy.ops.preferences.studiolight_copy_settings(index: int)` — Copy Studio Light settings to the Studio Light editor
- `bpy.ops.preferences.studiolight_install(files: collection, directory: string, filter_folder: boolean, filter_glob: string, type: enum{MATCAP, WORLD, STUDIO})` — Install a user defined light
- `bpy.ops.preferences.studiolight_new(filename: string)` — Save custom studio light from the studio light editor settings
- `bpy.ops.preferences.studiolight_uninstall(index: int)` — Delete Studio Light
- `bpy.ops.preferences.theme_install(overwrite: boolean, filepath: string, filter_folder: boolean, filter_glob: string)` — Load and apply a Blender XML theme file
- `bpy.ops.preferences.unassociate_blend()` — Remove this installation's associations with .blend files