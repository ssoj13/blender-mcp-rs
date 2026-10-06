---
area: api-ops
group: screen
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.screen

- `bpy.ops.screen.actionzone(modifier: int)` — Handle area action zones for mouse actions/gestures
- `bpy.ops.screen.animation_cancel(restore_frame: boolean)` — Cancel animation, returning to the original frame
- `bpy.ops.screen.animation_pause()` — Pause animation, stopping at the current frame
- `bpy.ops.screen.animation_play(reverse: boolean, sync: boolean)` — Play animation
- `bpy.ops.screen.animation_step()` — Step through animation by position
- `bpy.ops.screen.area_close()` — Close selected area
- `bpy.ops.screen.area_dupli()` — Duplicate selected area into new window
- `bpy.ops.screen.area_join(source_xy: int, target_xy: int)` — Join selected areas into new window
- `bpy.ops.screen.area_move(x: int, y: int, delta: int, snap: boolean)` — Move selected area edges
- `bpy.ops.screen.area_options()` — Operations for splitting and merging
- `bpy.ops.screen.area_split(direction: enum{HORIZONTAL, VERTICAL}, factor: float, cursor: int)` — Split selected area into new windows
- `bpy.ops.screen.area_swap(cursor: int)` — Swap selected areas screen positions
- `bpy.ops.screen.back_to_previous()` — Revert back to the original screen layout, before fullscreen area overlay
- `bpy.ops.screen.delete()` — Delete active screen
- `bpy.ops.screen.drivers_editor_show()` — Show drivers editor in a separate window
- `bpy.ops.screen.edge_merge(cursor: int)` — Merge aligned area edges
- `bpy.ops.screen.frame_jump(end: boolean)` — Jump to first/last frame in frame range
- `bpy.ops.screen.frame_offset(delta: int)` — Move current frame forward/backward by a given number
- `bpy.ops.screen.header_toggle_menus()` — Expand or collapse the header pull-down menus
- `bpy.ops.screen.info_log_show()` — Show info log in a separate window
- `bpy.ops.screen.keyframe_jump(next: boolean)` — Jump to previous/next keyframe
- `bpy.ops.screen.marker_jump(next: boolean)` — Jump to previous/next marker
- `bpy.ops.screen.new()` — Add a new screen
- `bpy.ops.screen.quadview_size()` — Resize Quad View areas
- `bpy.ops.screen.redo_last()` — Display parameters for last action performed
- `bpy.ops.screen.region_blend()` — Blend in and out overlapping region
- `bpy.ops.screen.region_context_menu()` — Display region context menu
- `bpy.ops.screen.region_flip()` — Toggle the region's alignment (left/right or top/bottom)
- `bpy.ops.screen.region_quadview()` — Split selected area into camera, front, right, and top views
- `bpy.ops.screen.region_scale()` — Scale selected area
- `bpy.ops.screen.region_toggle(region_type: enum{WINDOW, HEADER, CHANNELS, TEMPORARY, UI, TOOLS, TOOL_PROPS, ASSET_SHELF, ASSET_SHELF_HEADER, PREVIEW, HUD, NAVIGATION_BAR, EXECUTE, FOOTER, TOOL_HEADER, XR, SCRUBBING})` — Hide or unhide the region
- `bpy.ops.screen.repeat_history(index: int)` — Display menu for previous actions performed
- `bpy.ops.screen.repeat_last()` — Repeat last action
- `bpy.ops.screen.screen_full_area(use_hide_panels: boolean)` — Toggle display selected area as fullscreen/maximized
- `bpy.ops.screen.screen_set(delta: int)` — Cycle through available screens
- `bpy.ops.screen.screenshot(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, show_multiview: boolean, use_multiview: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Capture a picture of the whole Blender window
- `bpy.ops.screen.screenshot_area(filepath: string, hide_props_region: boolean, check_existing: boolean, filter_blender: boolean, filter_backup: boolean, filter_image: boolean, filter_movie: boolean, filter_python: boolean, filter_font: boolean, filter_sound: boolean, filter_text: boolean, filter_archive: boolean, filter_btx: boolean, filter_alembic: boolean, filter_usd: boolean, filter_obj: boolean, filter_volume: boolean, filter_folder: boolean, filter_blenlib: boolean, filemode: int, show_multiview: boolean, use_multiview: boolean, display_type: enum{DEFAULT, LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL}, sort_method: enum{})` — Capture a picture of an editor
- `bpy.ops.screen.space_context_cycle(direction: enum{PREV, NEXT})` — Cycle through the editor context by activating the next/previous one
- `bpy.ops.screen.space_type_set_or_cycle(space_type: enum{EMPTY, VIEW_3D, IMAGE_EDITOR, NODE_EDITOR, SEQUENCE_EDITOR, CLIP_EDITOR, DOPESHEET_EDITOR, GRAPH_EDITOR, NLA_EDITOR, TEXT_EDITOR, CONSOLE, INFO, TOPBAR, STATUSBAR, OUTLINER, PROPERTIES, FILE_BROWSER, SPREADSHEET, PREFERENCES})` — Set the space type or cycle subtype
- `bpy.ops.screen.spacedata_cleanup()` — Remove unused settings for invisible editors
- `bpy.ops.screen.time_jump(backward: boolean)` — Jump forward/backward by a given number of frames or seconds
- `bpy.ops.screen.userpref_show(section: enum{INTERFACE, VIEWPORT, LIGHTS, EDITING, ANIMATION, EXTENSIONS, ADDONS, THEMES, ASSETS, INPUT, NAVIGATION, KEYMAP, SYSTEM, SAVE_LOAD, FILE_PATHS, DEVELOPER_TOOLS, EXPERIMENTAL})` — Edit user preferences and system settings
- `bpy.ops.screen.workspace_cycle(direction: enum{PREV, NEXT})` — Cycle through workspaces