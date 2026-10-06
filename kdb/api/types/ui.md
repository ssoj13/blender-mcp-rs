---
area: api-types
group: ui
version: 5.2.2
---

# Blender 5.2.2 RNA types: ui

## GPencilSculptGuide

Guides for drawing

- `use_guide` · boolean — Enable speed guides
- `use_snapping` · boolean — Enable snapping to guides angle or spacing options
- `reference_object` · pointer · -> Object — Object used for reference point
- `reference_point` · enum · {CURSOR, CUSTOM, OBJECT} — Type of speed guide
- `type` · enum · {CIRCULAR, RADIAL, PARALLEL, GRID, ISO} — Type of speed guide
- `angle` · float — Direction of lines
- `angle_snap` · float — Angle snapping
- `spacing` · float — Guide spacing
- `location` · float · [3] — Custom reference point for guides

## UILayout

User interface layout in a panel or header

- `active` · boolean
- `active_default` · boolean — When true, an operator button defined after this will be activated when pressing return(use with popup dialogs)
- `activate_init` · boolean — When true, buttons defined in popups will be activated on first display (use so you can type into a field without having to click on it first)
- `operator_context` · enum · {INVOKE_DEFAULT, INVOKE_REGION_WIN, INVOKE_REGION_CHANNELS, INVOKE_REGION_PREVIEW, INVOKE_AREA, INVOKE_SCREEN, EXEC_DEFAULT, EXEC_REGION_WIN, EXEC_REGION_CHANNELS, EXEC_REGION_PREVIEW, EXEC_AREA, EXEC_SCREEN} — Typically set to 'INVOKE_REGION_WIN', except some cases in :class:`bpy.types.Menu` when it's set to 'EXEC_REGION_WIN'.
- `enabled` · boolean — When false, this (sub)layout is grayed out
- `alert` · boolean
- `alignment` · enum · {EXPAND, LEFT, CENTER, RIGHT}
- `direction` · enum · {HORIZONTAL, VERTICAL} · read-only
- `scale_x` · float — Scale factor along the X for items in this (sub)layout
- `scale_y` · float — Scale factor along the Y for items in this (sub)layout
- `ui_units_x` · float — Fixed size along the X for items in this (sub)layout
- `ui_units_y` · float — Fixed size along the Y for items in this (sub)layout
- `emboss` · enum · {NORMAL, NONE, PULLDOWN_MENU, PIE_MENU, NONE_OR_STATUS}
- `use_property_split` · boolean
- `use_property_decorate` · boolean

## UIList

UI list containing the elements of a collection

- `bl_idname` · string — If this is set, the uilist gets a custom ID, otherwise it takes the name of the class used to define the uilist (for example, if the class name is "OBJECT_UL_vgroups", and bl_idname is not set by the script, then bl_idname = "OBJECT_UL_vgroups")
- `list_id` · string · read-only — Identifier of the list, if any was passed to the "list_id" parameter of "template_list()"
- `layout_type` · enum · {DEFAULT, COMPACT} · read-only
- `use_filter_show` · boolean — Show filtering options
- `filter_name` · string — Only show items matching this name (use '*' as wildcard)
- `use_filter_invert` · boolean — Invert filtering (show hidden items, and vice versa)
- `use_filter_sort_alpha` · boolean — Sort items by their name
- `use_filter_sort_reverse` · boolean — Reverse the order of shown items
- `use_filter_sort_lock` · boolean — Lock the order of shown items (user cannot change it)
- `bitflag_filter_item` · int · read-only — The value of the reserved bitflag 'FILTER_ITEM' (in filter_flags values)
- `bitflag_item_never_show` · int · read-only — Skip the item from displaying in the list

## UIPieMenu

- `layout` · pointer · -> UILayout · read-only

## UIPopover

- `layout` · pointer · -> UILayout · read-only

## UIPopupMenu

- `layout` · pointer · -> UILayout · read-only
