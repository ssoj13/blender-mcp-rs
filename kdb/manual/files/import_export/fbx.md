---
area: manual-files
source: blender-manual
version: 5.2
---

***
FBX
***

The *FBX* (Filmbox) format is widely used for exchanging 3D data between applications,
especially for animated characters and complex scene data. It is supported by software
such as Autodesk Maya, 3ds Max, Cinema 4D, and game engines like Unity and Unreal Engine.

Blender's FBX importer supports a broad range of FBX features and is designed to be
fast, memory-efficient, and highly compatible with both modern and legacy FBX files.

.. _bpy.ops.wm.fbx_import:

Import
======

General
-------

Scale
   Value by which to scale the imported objects in relation to the world's origin.

Custom Properties
   Import user properties as Custom Properties </files/custom_properties>.

Enums As Strings
   Store custom property enumeration values as strings.

Geometry
--------

Custom Normals
   Import custom normals <modeling_meshes_normals_custom>,
   if available (otherwise Blender will compute them).

Subdivision Data
   Import FBX subdivision information as
   Subdivision Surface Modifiers </modeling/modifiers/generate/subdivision_surface>.

Vertex Colors
   Import vertex color attributes.

   :None: Do not import color attributes.
   :sRGB: Vertex colors in the file are in sRGB Color Space
   :Linear: Vertex colors in the file are in Linear Color Space

Materials
---------

Material Name Collision
   Behavior when the name of an imported material conflicts with an existing material.

   :Make Unique: Import each FBX material as a unique Blender material.
   :Reference Existing: If a material with the same name already exists, reference that instead of importing.

Animation
---------

Offset
   Offset to apply to animation timestamps, in frames.

**Admonition:**
   :class: note

   Each FBX "take" is imported as a separate Action </animation/actions>,
   with each animated object assigned to its own Action Slot <animation-actions-slots>.

Armature
--------

Ignore Leaf Bones
   Ignore the last bone at the end of each chain (used to mark the length of the previous bone).

Export
======

To export FBX files use the FBX Add-on <bpy.ops.export_scene.fbx>.
