---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

****
Grab
****

Drag geometry across the screen, following the cursor.
*Grab* only moves the vertices that are under the brush radius at the start of the stroke.
This is an essential sculpting brush to be used frequently to build shapes and adjust proportions.

**Note:**

   The effect is similar to moving geometry in Edit Mode with Proportional Editing enabled,
   except that *Grab* can make use of other Sculpt Mode options and brush settings.

Brush Settings
==============

General
-------

Size
   Pressure Sensitivity is not supported for this brush type. More info at Size <bpy.types.Brush.size>.

Strength
   Pressure Sensitivity is not supported for this brush type. More info at Strength <bpy.types.Brush.strength>.

Normal Radius
   For this brush, this setting is a purely visual change.
   It does not alter the brush behavior. More info at Normal Radius <bpy.types.Brush.normal_radius_factor>.

Auto-Smooth
   This setting is not supported. More info at Auto-Smooth <bpy.types.Brush.auto_smooth_factor>.

**Note:**

   More info at sculpt-tool-settings-brush-settings-general brush settings
   and on sculpt-tool-settings-brush-settings-advanced brush settings.

Unique
------

.. _bpy.types.Brush.use_grab_active_vertex:

Grab Active Vertex
   Applies the maximum strength of the brush to the highlighted active vertex,
   making it easier to manipulate low-poly models or meshes with modifiers.

   Enabling this option also enables a white wireframe overlay within the brush radius.
   This helps to visualize the real base geometry that is being manipulated
   while sculpting with Modifiers </modeling/modifiers/index>.

.. _bpy.types.Brush.use_grab_silhouette:

Grab Silhouette
   Preserves the object's silhouette shape by only grabbing vertices on one side of the mesh curvature.
   The shape of the silhouette is determined by the orientation of the 3D Viewport and the start of the stroke.

Note how in the image only the bottom side of the leg is pulled down, despite the size of the brush.

**Tip:**

   This setting is also useful for grabbing a single side of a crease
   and pushing it further inwards, creating a more pinched crease.

Additional Workflows
====================

2D Grab Brush
   If the Falloff Shape <bpy.types.Brush.falloff_shape> is set to *Projected*,
   the brush can grab infinitely deep into the viewport.
   This is especially useful for much broader changes to a sculpt.

   The stroke can also be started outside of the mesh (like in empty 3D space)
   and grab the vertices within the brush radius.
   This can be useful for sculpting flat and tube-like meshes.
