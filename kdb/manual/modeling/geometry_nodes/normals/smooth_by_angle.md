---
area: manual-modeling
source: blender-manual
version: 5.2
---

**************************
Smooth By Angle Node Group
**************************

Set the sharpness of mesh edges based on the angle between the neighboring faces.

**Note:**

   This is a node group asset that is included in the bundled "Essentials" asset library <assets-bundled>.

Inputs
======

Mesh
   Standard geometry input.
Angle
   Maximum angle between face normals that will be considered as smooth.
Ignore Sharpness
   Smooth all edges, even if they have been marked as sharp.

Outputs
=======

Mesh
   Standard geometry output.
