---
area: manual-modeling
source: blender-manual
version: 5.2
---

*******************
UVs & Texture Space
*******************

.. _uv-maps-panel:

UV Maps
=======

If you have a mesh object selected, you'll find its UV maps in the Data tab of the
Properties editor </editors/properties_editor>. After selecting a map,
you can view and edit it in the UV editor </editors/uv/introduction>.

One mesh can have multiple UV maps (e.g. one map per texture), although it's
also possible to reuse a UV map for multiple textures.

.. _bpy.types.MeshUVLoopLayer.active_render:

:bl-icon:`restrict_render_off`/:bl-icon:`restrict_render_on` Active Render
   Marks the UV map as the default one for rendering. The *Active Render* UV map is used for:

   - The UV Pass <bpy.types.ViewLayer.use_pass_uv> for compositing
   - The /render/shader_nodes/input/texture_coordinate for material shading.
   - The /render/shader_nodes/input/uv_map for material shading when no UV map is specified.

:bl-icon:`add` Add UV Map
   Duplicates the selected UV map, or creates a new one if the list is empty.

:bl-icon:`remove` Remove UV Map
   Removes the selected UV map.

.. _properties-texture-space:

Texture Space
=============

This panel lets you configure the object's Texture Space, which is a 3D box
used for generating texture coordinates without the use of a UV map.
You can visualize the texture space using the option in the
/scene_layout/object/properties/display panel.

.. _bpy.types.Mesh.texture_mesh:

Texture Mesh Mesh objects
   Use another mesh for texture indices. The vertices of the two objects must be perfectly aligned
   or the UV map will be distorted.

.. _bpy.types.Mesh.use_auto_texspace:

Auto Texture Space
   Calculates the texture space automatically.

.. _bpy.types.Mesh.texspace_location:
.. _bpy.types.Mesh.texspace_size:

Location X, Y, Z, Size X, Y, Z
   Lets you define the texture space manually, relative to the object.
   Note that you can also edit it in the 3D Viewport -- see `Editing`_ below.

.. _bpy.ops.curve.match_texture_space:

Match Texture Space Curve objects
   Modifies the *Location* and *Size* to match the object's bounding box.
   This disables Auto Texture Space.

   .. is Match Texture Space the same thing as Auto Texture Space?

.. _properties-texture-space-editing:

Editing
-------

Click one of these menu items, then move the mouse to adjust the texture space
and press LMB to confirm. While transforming, you can use keyboard shortcuts to lock
certain axes; see the status bar.

Accessing
---------

When setting up a material shader, you can use the *Generated* output of the
/render/shader_nodes/input/texture_coordinate to read the 3D coordinate
inside the object's texture space. You can then pass this coordinate to a texture
node.

**Tip:**

   Texture spaces do not have rotation support. You can use a
   /render/shader_nodes/utilities/vector/mapping to manually rotate the coordinate
   in the material shader instead.
