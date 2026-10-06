---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.View3DShading.color_type:

************
Object Color
************

The colors that the Workbench uses to render objects can be changed.

:Material:
   Use the color that can be set per material
   in the Viewport Display properties-material-viewport-display panel.
:Object:
   Use the color that can be set per object
   in the Viewport Display properties-object-viewport-display panel.
:Random:
   A random color will be selected for every object in the scene.
:Attribute:
   Display the active Color Attribute
   of an object. When an object has no active Color Attribute it will be rendered in the color set
   in the Viewport Display properties-object-viewport-display panel.
:Texture:
   Show the texture from the active /render/shader_nodes/textures/image
   using the active UV map. If there is no active texture, the object will be
   rendered with the settings in the properties-material-viewport-display's
   Viewport Display panel.
:Custom:
   Render the whole scene using a single color. The color can be chosen.
