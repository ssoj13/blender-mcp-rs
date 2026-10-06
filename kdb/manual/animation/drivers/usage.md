---
area: manual-animation
source: blender-manual
version: 5.2
---

*****
Usage
*****

Drivers can be added to properties via their context menu, a shortcut, copy-pasted,
or by typing an expression directly into the property's value.

After adding drivers, they are usually modified in the Drivers editor </editors/drivers_editor>,
or via a simplified *Edit Driver* popover invoked from the property context menu.

.. _bpy.ops.anim.driver_button_add:

Add Driver
==========

The usual way to add a driver to a property is to RMB click a property,
then choose *Add Driver* in the context menu.
Drivers can also be added by pressing Ctrl-D with the mouse over the property.

This operation adds a driver with a single variable (which needs to be filled in),
and displays the *Edit Driver* popover.

.. _bpy.ops.anim.driver_button_edit:

Edit Driver
===========

Displays a popover window that allows editing the custom expression and input variables
of the driver without opening the full *Drivers Editor*.

Many drivers don't use their F-Curve </editors/graph_editor/fcurves/introduction>
component, so this reduced interface is sufficient.

.. _bpy.ops.screen.drivers_editor_show:

Open Drivers Editor
===================

Opens a new window with the *Drivers Editor* and
selects the driver associated with the property.

.. _bpy.ops.anim.copy_driver_button:
.. _bpy.ops.anim.paste_driver_button:

Copy & Paste
============

Drivers can be copied and pasted via the context menu.
When adding drivers with the same settings, this can save time modifying settings.

.. _bpy.ops.ui.copy_driver_to_selected_button:

Copy Driver to Selected
=======================

Copy the property's driver from the active item to the same
property of all selected items, if the same property exists.

.. _bpy.ops.ui.copy_as_driver_button:

Copy As New Driver
==================

A driver that sets the property value to the value of a different property can be
quickly created by using the *Copy As New Driver* context menu option of the input
property, and then pasting the result onto the output property via *Paste Driver*.

It is also possible to add the new driver variable to an existing driver using
the Paste Driver Variables <drivers-variables> button in the editor panel.

Expression
==========

This is a quick way to add drivers with a scripted expression.
First click the property you want to add a driver to, then type a hash `#` and a scripted expression.

Some examples:

- `#frame`
- `#frame / 20.0`
- `#sin(frame)`
- `#cos(frame)`

.. _bpy.ops.anim.driver_button_remove:

Removing Drivers
================

Removes driver(s) associated with the property, either for the single selected property
or sub-channel, or all components of a vector.
