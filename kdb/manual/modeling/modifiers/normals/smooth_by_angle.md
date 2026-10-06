---
area: manual-modeling
source: blender-manual
version: 5.2
---

************************
Smooth By Angle Modifier
************************

Sets the sharpness of mesh edges based on the angle between the neighboring faces.

**Note:**

   This is a geometry nodes asset that is included in the bundled "Essentials" asset library <assets-bundled>.

**Tip:**

   This modifier can easily be added to an object with bpy.ops.object.shade_auto_smooth
   or removed with bpy.ops.object.shade_smooth or bpy.ops.object.shade_flat.

Options
=======

Angle
   Maximum angle between face normals that will be considered as smooth.
Ignore Sharpness
   Smooth all edges, even if they have been marked as Sharp <bpy.ops.mesh.mark_sharp>.
