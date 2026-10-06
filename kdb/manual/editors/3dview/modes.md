---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.ops.object.mode_set:

************
Object Modes
************

.. _fig-view3d-mode-select:

Modes allow editing different aspects of objects. While Object Mode allows
you to position/rotate/scale them, Edit Mode allows changing their geometry,
Pose Mode allows posing them, and so on.

You can change the current mode using the *Mode* selector in the 3D Viewport header.
Which modes are available depends on the object's type. The complete list
is shown below.

Apart from using the selector, you can also press Ctrl-Tab to bring up
a pie menu around the cursor for faster access. (If the selected object is an
Armature </animation/armatures/introduction>, this shortcut will instead
switch between Object Mode and Pose Mode.)

Pressing Tab will toggle Edit Mode for objects that support it.

Modes can affect many things in Blender:

- Each mode changes the header and Toolbar to show its own unique set of menus and tools.
  This also means it affects the available keyboard shortcuts.
- Modes can completely change the look of the viewport. For example, Weight Paint mode
  will shade the object to show its vertex weights, which are not normally visible.
- Modes can affect other editors. For example, the UV Editor </editors/uv/introduction>
  can only be used if the 3D Viewport is in Edit Mode. In the
  Properties </editors/properties_editor> editor, too, certain buttons and panels
  can only be used in certain modes.

Object Mode List
================

.. _tab-view3d-modes:

.. list-table:: Blender's Modes
   :header-rows: 1
   :class: valign
   :widths: 5 24 50

   * - Icon
     - Name
     - Details
   * - .. rubric:: :bl-icon:`object_datamode`
     - Object Mode
     - The default mode, available for all object types.
       Allows editing position, rotation and scale, duplicating objects, and so on.
   * - .. rubric:: :bl-icon:`editmode_hlt`
     - Edit Mode
     - A mode for editing an object's shape
       (vertices/edges/faces for meshes, control points for curves/surfaces,
       points/strokes for Grease Pencil, etc.).
   * - .. rubric:: :bl-icon:`sculptmode_hlt`
     - Sculpt Mode
     - Provides an alternative toolset for editing an object's shape (only for meshes).
   * - .. rubric:: :bl-icon:`vpaint_hlt`
     - Vertex Paint Mode
     - A mesh-only mode that allows you to set your mesh's vertex colors (i.e. to "paint" them).
   * - .. rubric:: :bl-icon:`wpaint_hlt`
     - Weight Paint Mode
     - A mesh-only mode, dedicated to vertex group weighting.
   * - .. rubric:: :bl-icon:`tpaint_hlt`
     - Texture Paint Mode
     - A mesh-only mode that allows you to paint a texture directly on the model, in the 3D Viewport.
   * - .. rubric:: :bl-icon:`particlemode`
     - Particle Edit Mode
     - A mesh-only mode dedicated to particle systems, useful for editable systems (hair).
   * - .. rubric:: :bl-icon:`pose_hlt`
     - Pose Mode
     - An armature-only mode, dedicated to posing.
   * - .. rubric:: :bl-icon:`greasepencil`
     - Draw Mode
     - A Grease Pencil-only mode, dedicated to creating Grease Pencil strokes.
   * - .. rubric:: :bl-icon:`sculptmode_hlt`
     - Sculpt Mode (Grease Pencil)
     - A Grease Pencil-only mode, used to deform and shape existing strokes more organically.
   * - .. rubric:: :bl-icon:`editmode_hlt`
     - Edit Mode (Grease Pencil)
     - A Grease Pencil-only mode, dedicated to modifying individual strokes and points of Grease Pencil objects.
   * - .. rubric:: :bl-icon:`vpaint_hlt`
     - Vertex Paint Mode (Grease Pencil)
     - A Grease Pencil-only mode, dedicated to adding color to the vertices of strokes directly.
   * - .. rubric:: :bl-icon:`wpaint_hlt`
     - Weight Paint Mode (Grease Pencil)
     - A Grease Pencil-only mode, dedicated to assigning vertex weights to stroke.

**Note:**

   The cursor becomes a brush
   in Paint and Sculpt Modes </sculpt_paint/index>.

We will not go into any more detail on mode usages here,
because they are dealt with in their own sections.

**Hint:**

   If you are reading this manual and some button or menu option is referenced
   that does not appear on your screen, it may be that you are not in the proper
   mode for that option to be valid.

.. _bpy.ops.object.transfer_mode:

Switching Objects
=================

If you enter a mode such as Weight Paint for an object and then select another
object, Blender will typically switch back to Object Mode.
This means that, if you want to weight paint the other object too,
you have to enter the mode a second time.

There is a way of avoiding this, however. Once you enter a mode, the
Outliner </editors/outliner/introduction> will show a dot next
to other objects that also support it. By clicking such a dot, you can
switch over to another object without leaving the mode.

Alternatively, you can hover over the other object in the 3D Viewport
and press Alt-Q.

**Seealso:**

   Lock Object Modes <bpy.types.ToolSettings.lock_object_mode> for
   preventing *accidental* mode changes.

.. _3dview-multi-object-mode:

Multi-Object Editing
====================

Edit Mode and Pose Mode let you work with multiple objects even more
easily than described above, as they can have multiple objects in the mode
at the same time.

There are two ways of accomplishing this:

- If you're not yet in the mode, you can simply select all the objects
  and enter it.
- If you're already in the mode, you can bring other objects into it
  by clicking Ctrl-LMB on the dot in the
  Outliner </editors/outliner/introduction>.
  Removing objects from the mode works in the same way.

Some points of note:

- The /editors/properties_editor editor will only ever show the details
  (shape keys, UV maps...) of the active object, not of all the selected ones.
- Selecting any element from an object will make it the active one.
- There are limits to the edits you can make.
  For example, you can't create an edge that connects vertices from
  different objects.
