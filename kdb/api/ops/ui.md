---
area: api-ops
group: ui
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.ui

- `bpy.ops.ui.assign_default_button()` — Set this property's current value as the new default
- `bpy.ops.ui.button_execute(skip_depressed: boolean)` — Presses active button
- `bpy.ops.ui.button_string_clear()` — Unsets the text of the active button
- `bpy.ops.ui.copy_as_driver_button()` — Create a new driver with this property as input, and copy it to the internal clipboard. Use Paste Driver to add it to the target property, or Paste Driver Variables to extend an existing driver
- `bpy.ops.ui.copy_data_path_button(full_path: boolean)` — Copy the RNA data path for this property to the clipboard
- `bpy.ops.ui.copy_driver_to_selected_button(all: boolean)` — Copy the property's driver from the active item to the same property of all selected items, if the same property exists
- `bpy.ops.ui.copy_python_command_button()` — Copy the Python command matching this button
- `bpy.ops.ui.copy_to_selected_button(all: boolean)` — Copy the property's value from the active item to the same property of all selected items if the same property exists
- `bpy.ops.ui.drop_color(color: float, gamma: boolean, has_alpha: boolean)` — Drop colors to buttons
- `bpy.ops.ui.drop_material(session_uid: int)` — Drag material to Material slots in Properties
- `bpy.ops.ui.drop_name(string: string)` — Drop name to button
- `bpy.ops.ui.editsource()` — Edit UI source code of the active button
- `bpy.ops.ui.eyedropper_bone()` — Sample a bone from the 3D View or the Outliner to store in a property
- `bpy.ops.ui.eyedropper_color(prop_data_path: string)` — Sample a color from the Blender window to store in a property
- `bpy.ops.ui.eyedropper_colorramp()` — Sample a color band
- `bpy.ops.ui.eyedropper_colorramp_point()` — Point-sample a color band
- `bpy.ops.ui.eyedropper_depth(prop_data_path: string)` — Sample depth from the 3D view
- `bpy.ops.ui.eyedropper_driver(mapping_type: enum{SINGLE_MANY, DIRECT, MATCH, NONE_ALL, NONE_SINGLE})` — Pick a property to use as a driver target
- `bpy.ops.ui.eyedropper_grease_pencil_color(mode: enum{MATERIAL, PALETTE, BRUSH}, material_mode: enum{STROKE, FILL, BOTH})` — Sample a color from the Blender Window and create Grease Pencil material
- `bpy.ops.ui.eyedropper_id()` — Sample a data-block from the 3D View to store in a property
- `bpy.ops.ui.jump_to_target_button()` — Switch to the target object or bone
- `bpy.ops.ui.list_start_filter()` — Start entering filter text for the list in focus
- `bpy.ops.ui.override_add_button(all: boolean)` — Create an override operation
- `bpy.ops.ui.override_idtemplate_clear()` — Delete the selected local override and relink its usages to the linked data-block if possible, else reset it and mark it as non editable
- `bpy.ops.ui.override_idtemplate_make()` — Create a local override of the selected linked data-block, and its hierarchy of dependencies
- `bpy.ops.ui.override_idtemplate_reset()` — Reset the selected local override to its linked reference values
- `bpy.ops.ui.override_remove_button(all: boolean)` — Remove an override operation
- `bpy.ops.ui.reloadtranslation()` — Force a full reload of UI translation
- `bpy.ops.ui.reset_default_button(all: boolean)` — Reset this property's value to its default value
- `bpy.ops.ui.unset_property_button()` — Clear the property and use default or generated value in operators
- `bpy.ops.ui.view_drop()` — Drag and drop onto a data-set or item within the data-set
- `bpy.ops.ui.view_item_delete()` — Delete selected list item
- `bpy.ops.ui.view_item_focus()` — Bring active item into focus by scrolling the view
- `bpy.ops.ui.view_item_navigate(direction: enum{UP, DOWN, LEFT, RIGHT})` — Walk and select view items in given direction
- `bpy.ops.ui.view_item_rename()` — Rename the active item in the data-set view
- `bpy.ops.ui.view_item_select(wait_to_deselect_others: boolean, use_select_on_click: boolean, mouse_x: int, mouse_y: int, extend: boolean, range_select: boolean)` — Activate selected view item
- `bpy.ops.ui.view_scroll()` — (undocumented operator)
- `bpy.ops.ui.view_start_filter()` — Start entering filter text for the data-set in focus