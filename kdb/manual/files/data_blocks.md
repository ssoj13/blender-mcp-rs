---
area: manual-files
source: blender-manual
version: 5.2
---

.. _bpy.types.ID:
.. _bpy.types.BlendData:

***********
Data-Blocks
***********

The base unit for any Blender project is the data-block. Examples of data-blocks include:
meshes, objects, materials, textures, node trees, scenes, texts, brushes, and even Workspaces.

A data-block is a generic abstraction of very different kinds of data,
which features a common set of basic features, properties and behaviors.

Some common characteristics:

- They are the primary contents of the blend-file.
- They can reference each other, for reuse and instancing.
  (Child/parent, object/object-data, materials/images, in modifiers or constraints too...)
- Their names are unique within a blend-file, for a given type.
- They can be added/removed/edited/duplicated.
- They can be linked between files (only enabled for a limited set of data-blocks).
- They can have their own animation data.
- They can have /files/custom_properties.

You will typically interact with the higher level data types (objects, meshes, etc.).
When doing more complex projects, managing data-blocks becomes more important,
especially when inter-linking blend-files.
The main editor for that is the Outliner </editors/outliner/index>.

Not all data in Blender is a data-block. E.g.
bones, sequence strips or vertex groups are not;
they belong to armature, scene and mesh types respectively.

.. _data-system-datablock-types:

Data-Block Types
================

.. Editor's Note:
   Mostly we want to avoid long lists of data -- but in this case,
   it is the only comprehensive list of data-blocks, and something which you cannot
   find directly just through looking at the interface.
   ::
   Use bpy.types.SpaceOutliner.filter_id_type as a reference list.

For reference, here is a table of data-blocks types stored in blend-files.

Link
   Library Linking, supports being linked into other blend-files.
Pack
   File Packing, supports file contents being packed into the blend-file
   (**not** applicable for most data-blocks which have no file reference).

.. Editor's Note:
   For each data-block, we have 2 lines.
   1) a terse description.
   2) how its used.
   ::
   Keep these short.

.. |tick|  unicode:: U+2713
.. |cross| unicode:: U+2717
.. |none|  unicode:: U+2014

.. list-table::
   :header-rows: 1
   :class: valign
   :widths: 20 5 5 70

   * - Type
     - Link
     - Pack
     - Description
   * - :bl-icon:`action` Action
     - |tick|
     - |none|
     - Stores animation F-Curves.
       Used as data-block animation data, and the Nonlinear Animation editor.
   * - :bl-icon:`armature_data` Armature
     - |tick|
     - |none|
     - Skeleton used to deform meshes.
       Used as data of armature objects, and by the Armature Modifier.
   * - :bl-icon:`brush_data` Brush
     - |tick|
     - |none|
     - Used as brush assets in sculpt and paint modes.
   * - :bl-icon:`file_blank` Cache File
     - |tick|
     - |none|
     - Used by Mesh Cache modifiers.
   * - :bl-icon:`camera_data` Camera
     - |tick|
     - |none|
     - Used as data by camera objects.
   * - :bl-icon:`outliner_collection` Collection
     - |tick|
     - |none|
     - Group and organize objects in scenes.
       Used to instance objects, and in library linking.
   * - :bl-icon:`curve_data` Curve
     - |tick|
     - |none|
     - Used as data by curve, font & surface objects.
   * - :bl-icon:`curve_data` Curves
     - |tick|
     - |none|
     - New curve data type (replacing curve).
   * - :bl-icon:`font_data` Font
     - |tick|
     - |tick|
     - References font files.
       Used by curve object-data of text objects.
   * - :bl-icon:`greasepencil` Grease Pencil
     - |tick|
     - |none|
     - 2D/3D sketch data used by Grease Pencil objects.
       Used as overlay *helper* info, by the 3D Viewport, Image, Sequencer & Movie Clip editors.
   * - :bl-icon:`greasepencil` Grease Pencil v3
     - |tick|
     - |none|
     - 2D/3D sketch data used by Grease Pencil objects.
       Used as overlay *helper* info, by the 3D Viewport, Image, Sequencer & Movie Clip editors.
   * - :bl-icon:`image_data` Image
     - |tick|
     - |tick|
     - Image files.
       Used by shader nodes and textures.
   * - :bl-icon:`shapekey_data` Key (Shape Keys)
     - |cross|
     - |none|
     - Geometry shape storage, which can be animated.
       Used by mesh, curve, and lattice objects.
   * - :bl-icon:`lattice_data` Lattice
     - |tick|
     - |none|
     - Grid based lattice deformation.
       Used as data of lattice objects, and by the Lattice Modifier.
   * - :bl-icon:`library_data_direct` Library
     - |cross|
     - |tick|
     - References to an external blend-file.
       Access from the Outliner's *Blender File* view.
   * - :bl-icon:`light_data` Light
     - |tick|
     - |none|
     - Used as object data by light objects.
   * - :bl-icon:`lightprobe_sphere` Light Probe
     - |tick|
     - |none|
     - Help achieve complex real-time lighting in EEVEE.
   * - :bl-icon:`line_data` Line Style
     - |tick|
     - |none|
     - Used by the Freestyle renderer.
   * - :bl-icon:`mod_mask` Mask
     - |tick|
     - |none|
     - 2D animated mask curves.
       Used by compositing nodes & sequencer strip.
   * - :bl-icon:`material_data` Material
     - |tick|
     - |none|
     - Set shading and texturing render properties.
       Used by objects, meshes & curves.
   * - :bl-icon:`mesh_data` Mesh
     - |tick|
     - |none|
     - Geometry made of vertices/edges/faces.
       Used as data of mesh objects.
   * - :bl-icon:`meta_data` Metaball
     - |tick|
     - |none|
     - An isosurface in 3D space.
       Used as data of metaball objects.
   * - :bl-icon:`tracker` Movie Clip
     - |tick|
     - |cross|
     - Reference to an image sequence or video file.
       Used in the *Movie Clip* editor.
   * - :bl-icon:`nodetree` Node Tree
     - |tick|
     - |none|
     - Groups of re-usable nodes.
       Used in the node editors.
   * - :bl-icon:`object_data` Object
     - |tick|
     - |none|
     - An entity in the scene with location, scale, rotation.
       Used by scenes & collections.
   * - :bl-icon:`curve_bezcurve` Paint Curve
     - |tick|
     - |none|
     - Stores a paint or sculpt stroke.
       Access from the paint tools.
   * - :bl-icon:`color` Palette
     - |tick|
     - |none|
     - Store color presets.
       Access from the paint tools.
   * - :bl-icon:`particles` Particle
     - |tick|
     - |none|
     - Particle settings.
       Used by particle systems.
   * - :bl-icon:`pointcloud_data` Point Cloud
     - |tick|
     - |none|
     - Collection of points in 3D space.
   * - :bl-icon:`scene_data` Scene
     - |tick|
     - |none|
     - Primary store of all data displayed and animated.
       Used as top-level storage for objects & animation.
   * - :bl-icon:`workspace` Screen
     - |tick|
     - |none|
     - Low level user interface storage.
   * - :bl-icon:`sound` Sound
     - |tick|
     - |tick|
     - Reference to sound files.
       Used as data of speaker objects.
   * - :bl-icon:`speaker` Speaker
     - |tick|
     - |none|
     - Sound sources for a 3D scene.
       Used as data of speaker object.
   * - :bl-icon:`text` Text
     - |tick|
     - |cross|
     - Text data.
       Used by Python scripts and OSL shaders.
   * - :bl-icon:`texture_data` Texture
     - |tick|
     - |none|
     - 2D/3D textures.
       Used by brushes and modifiers.
   * - :bl-icon:`volume_data` Volume
     - |tick|
     - |none|
     - Volumetric objects used to contain grids of data.
   * - :bl-icon:`window` Window Manager
     - |cross|
     - |none|
     - The overarching manager for all of Blender's user interface.
       Includes Workspaces, notification system, operators, and keymaps.
   * - :bl-icon:`workspace` Workspace
     - |cross|
     - |none|
     - UI layout.
       Used by each window, which has its own workspace.
   * - :bl-icon:`world_data` World
     - |tick|
     - |none|
     - Define global render environment settings.

.. _data-system-datablock-life-time:

Life Time
=========

Every data-block has its usage counted (reference count), when there is more than one,
you can see the number of current users of a data-block to the right of its name in the interface.
Blender follows the general rule that unused data is eventually removed.

Since it is common to add and remove a lot of data while working,
this has the advantage of not having to manually manage every single data-block.
This works by skipping zero user data-blocks when writing blend-files.

.. _data-system-datablock-fake-user:

Protected
---------

Since zero user data-blocks are not saved,
there are times when you want to force the data to be kept irrespective of its users.

If you are building a blend-file to serve as a library of assets that you intend to link to and from other files,
you will need to make sure that they do not accidentally get deleted from the library file.

To protect a data-block, use the button with the shield icon next to its name.
The data-block will then never be silently deleted by Blender,
but you can still manually remove it if needed.

**Note:**

   Linked data <bpy.ops.wm.link> cannot be protected that way.

.. _data-system-datablock-name-and-rename:

Name & Rename
=============

A data-block's name is unique within its namespace. A data-block namespace is defined by its type, and the blendfile
it is stored in.

This means that there can be for example an Object and a Mesh named the same, but there cannot be two local objects
named the same in a blendfile. However, it is possible to have one local and several linked Objects sharing the same
name.

Data-block names have a fixed length of 255 bytes and are UTF-8 encoded. This is room for 255 basic ASCII characters,
or less when using diacritics or non-latin glyphs, which use more than one byte per character. Data-block names cannot
contain a Null character (`U+0000`).

When Blender has to name a new data-block or rename an existing one, it will check for name collisions. If a
data-block with the same name already exists, the (re)named data-block will get a numeric extension added as a
post-fix to its 'root name', like e.g. `.001`. The first available index is used up to the `999` value - after that
the postfix index values are simply incremented until no collision happens anymore.

In case adding the numeric suffix would make the data-block name too long, the root name part will be shortened as
needed.

Blender will never rename another data-block when doing automatic naming. So e.g. when adding a new `Cube` object and
there are already `Cube` and `Cube.001` local objects, the new one will be named `Cube.002`.

Local data-blocks can be renamed by the user in several places in the UI (like the ID selection widget, or the
Outliner view). When renamed from the UI, the behavior in case of name collision is as follows:

- If the original root name is different than in the new requested name, the renamed data-block gets the first
  available numerical suffix.

  - E.g. assuming that there are three objects named `Sphere`, `Cube` and `Cube.001`, renaming `Sphere` to `Cube` will
    rename the data-block to `Cube.002`.

- If the original root name is the same as in the new requested name, the renamed data-block gets the requested name,
  and the conflicting data-block is renamed accordingly.

  - E.g. assuming that there are three objects named `Sphere`, `Cube` and `Cube.001`, renaming `Cube.001` to `Cube`
    will rename the data-block to `Cube`, and the other data-block to `Cube.001`.

.. _data-system-datablock-name-hidden:

Dot-Prefixed Names (Hidden Data)
--------------------------------

Data-block names that begin with a dot (`.`) are considered *hidden*.

Hidden data-blocks:

- Are not shown in File Browsers by default.
- Are hidden in most data-block selection menus.
- Are commonly used for internal, temporary, or helper data.

This naming convention is useful for marking implementation details
that should not normally be selected or edited directly by users.

Hidden data-blocks can be made visible through user preferences:

- Enable Defaults -- Show Hidden Files/Data-Blocks <bpy.types.PreferencesFilePaths.show_hidden_files_datablocks>
  to display dot-prefixed data-blocks in File Browsers and data ID selectors.
- Enable Search -- Show Hidden <bpy.types.Preferences.show_hidden_ids>
  to display dot-prefixed data-blocks in search menus.

Even when hidden in the interface, dot-prefixed data-blocks behave like any other data-block
in terms of linking, usage counting, and saving.

Sharing
=======

Data-blocks can be shared among other data-blocks.

Examples where sharing data is common:

- Sharing textures among materials.
- Sharing meshes between objects (instances).
- Sharing animated actions between objects,
  for example to make all the lights dim together.

You can also share data-blocks between files, see
linked libraries </files/linked_libraries/index>.

.. _data-system-datablock-make-single-user:

Making Single User
==================

When a data-block is shared between several users, you can make a copy of it for a given user.
To do so, click on the user count button to the right of its name.
This will duplicate that data-block and assign the newly created copy to that usage only.

**Note:**

   Objects have a set of more advanced actions to become single-user,
   see their documentation <bpy.ops.object.make_single_user>.

Removing Data-Blocks
====================

As covered in `Life Time`_, data-blocks are typically removed when they are no longer used.
They can also be manually *unlinked* or *deleted*.

Unlinking a data-block means that its user won't use it anymore.
This can be achieved by clicking on the "X" icon next to a data-block's name.
If you unlink a data-block from all of its users,
it will eventually be deleted by Blender as described above (unless it is a protected one).

Deleting a data-block directly erases it from the blend-file, automatically unlinking it from all of its users.
This can be achieved by Shift-LMB on the "X" icon next to its name.

**Warning:**

   Deleting some data-blocks can lead to deletion of some of their users, which would become invalid without them.
   The main example is that object-data deletion (like mesh, curve, camera...) will also delete all objects using it.

Those two operations are also available in the context menu
when RMB-clicking on a data-block in the *Outliner*.
