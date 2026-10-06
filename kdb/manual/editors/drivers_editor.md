---
area: manual-editors
source: blender-manual
version: 5.2
---

**************
Drivers Editor
**************

This editor lets you set up Drivers </animation/drivers/index>, which calculate the value for a property
based on other properties. In other words, they make a set of source properties "drive" the target property,
and can thus serve as an alternative to animating the property by hand.

The user interface is largely the same as that of the Graph Editor </editors/graph_editor/introduction>,
with two important differences:

- The Sidebar has an additional Drivers tab </animation/drivers/drivers_panel>. This is where
  the source properties are brought together to calculate an intermediate value for the target property.
- The curve doesn't represent the property's value over time, but a mapping from the above intermediate
  value (X axis) to the final value (Y axis).
