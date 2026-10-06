---
area: api-ops
group: extensions
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.extensions

- `bpy.ops.extensions.package_disable()` — Turn off this extension
- `bpy.ops.extensions.package_install(repo_directory: string, repo_index: int, pkg_id: string, enable_on_install: boolean, url: string, do_legacy_replace: boolean)` — Download and install the extension
- `bpy.ops.extensions.package_install_files(filter_glob: string, directory: string, files: collection, filepath: string, repo: enum{}, enable_on_install: boolean, target: enum{}, overwrite: boolean, url: string)` — Install extensions from files into a locally managed repository
- `bpy.ops.extensions.package_install_marked(enable_on_install: boolean)` — (undocumented operator)
- `bpy.ops.extensions.package_mark_clear(pkg_id: string, repo_index: int)` — (undocumented operator)
- `bpy.ops.extensions.package_mark_clear_all()` — (undocumented operator)
- `bpy.ops.extensions.package_mark_set(pkg_id: string, repo_index: int)` — (undocumented operator)
- `bpy.ops.extensions.package_mark_set_all()` — (undocumented operator)
- `bpy.ops.extensions.package_obsolete_marked()` — Zeroes package versions, useful for development - to test upgrading
- `bpy.ops.extensions.package_show_clear(pkg_id: string, repo_index: int)` — (undocumented operator)
- `bpy.ops.extensions.package_show_set(pkg_id: string, repo_index: int)` — (undocumented operator)
- `bpy.ops.extensions.package_show_settings(pkg_id: string, repo_index: int)` — (undocumented operator)
- `bpy.ops.extensions.package_theme_disable(pkg_id: string, repo_index: int)` — Reset to the default theme if this theme is active
- `bpy.ops.extensions.package_theme_enable(pkg_id: string, repo_index: int)` — Turn on this theme
- `bpy.ops.extensions.package_uninstall(repo_directory: string, repo_index: int, pkg_id: string)` — Disable and uninstall the extension
- `bpy.ops.extensions.package_uninstall_marked()` — (undocumented operator)
- `bpy.ops.extensions.package_uninstall_system()` — (undocumented operator)
- `bpy.ops.extensions.package_upgrade_all(use_active_only: boolean)` — Upgrade installed extensions to their latest version from remote repositories
- `bpy.ops.extensions.repo_enable_from_drop(repo_index: int)` — (undocumented operator)
- `bpy.ops.extensions.repo_lock_all()` — Lock repositories - to test locking
- `bpy.ops.extensions.repo_refresh_all(use_active_only: boolean)` — Refresh extension & legacy add-ons, reloading modules & meta-data (similar to restarting)
- `bpy.ops.extensions.repo_sync(repo_directory: string, repo_index: int)` — (undocumented operator)
- `bpy.ops.extensions.repo_sync_all(use_active_only: boolean)` — Refresh the list of extensions for all the remote repositories
- `bpy.ops.extensions.repo_unlock()` — Remove the repository file-system lock
- `bpy.ops.extensions.repo_unlock_all()` — Unlock repositories - to test unlocking
- `bpy.ops.extensions.status_clear()` — (undocumented operator)
- `bpy.ops.extensions.status_clear_errors()` — (undocumented operator)
- `bpy.ops.extensions.userpref_allow_online()` — Allow Blender to access the internet. Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule.
- `bpy.ops.extensions.userpref_allow_online_popup()` — Allow Blender to access the internet. Add-ons that follow this setting will only connect to the internet if enabled. However, Blender cannot prevent third-party add-ons from violating this rule.
- `bpy.ops.extensions.userpref_show_for_update()` — Open extensions preferences
- `bpy.ops.extensions.userpref_show_online()` — Show system preferences "Network" panel to allow online access
- `bpy.ops.extensions.userpref_tags_set(value: boolean, data_path: string)` — Set the value of all tags