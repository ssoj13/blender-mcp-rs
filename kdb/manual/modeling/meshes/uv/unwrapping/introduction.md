---
area: manual-modeling
source: blender-manual
version: 5.2
---

************
Introduction
************

The first step is to unwrap your mesh. Generally, it is recommended to start unwrapping
when only minor adjustments to the geometry of your model are required.
If you do add faces or subdivide existing faces when a model is already unwrapped,
Blender will add those new faces for you,
but you may need to do additional mapping or editing. In this fashion,
you can use the UV texture image to guide additional geometry changes.

About UVs
=========

Every point in the UV map corresponds to a vertex in the mesh.
The lines joining the UVs correspond to edges in the mesh.
Each face in the UV map corresponds to a mesh face.
Think of a UV map as projecting the surface of your 3D model onto a 2D image.

Each face of a mesh can have many UV textures. Each UV texture can have an individual image assigned to it.
When you unwrap a face to a UV texture in the UV Editor, each face of the mesh is automatically assigned
four UV coordinates: These coordinates define the way an image or a texture is mapped onto the face.
To distinguish from XYZ coordinates, the U and V axes are used to mark the coordinates of each point.
Hence the name, UV unwrapping. These coordinates can be used for rendering or for real-time viewport
display as well.

Every face in Blender can have a link to a different image.
The UV coordinates define how this image is mapped onto the face.
This image then can be rendered or displayed in real-time.
A 3D Viewport has to be in "Face Select" mode to be able to assign Images or
change UV coordinates of the active mesh object.
This allows a face to participate in many UV textures.
A face at the hairline of a character might participate in the facial UV texture,
*and* in the scalp/hair UV texture.

These are described more fully in the next sections.

Getting Started
===============

By default, meshes are not created with UVs. First you must map the faces, then
you can edit them </modeling/meshes/uv/editing>.
The process of unwrapping your model is done within Edit Mode in the 3D Viewport.
This process creates one or more UV Islands in the UV Editor <editors-uv-index>.

To begin, choose the *UV Editing* workspace </interface/window_system/workspaces>
from the selection list at the top of your screen in the Preferences header.
This sets one of the areas to show you the UV Editor, and the other area to the 3D Viewport.

Enter *Edit Mode*, as all unwrapping is done in Edit Mode.
You can be in vertex, face, or edge selection mode.

Workflow
--------

The general workflow is as follows, but know that different models may require
different approaches to unwrapping:

#. Mark Seams if necessary. See more about marking seams </modeling/meshes/uv/unwrapping/seams>.
#. Select mesh faces in the 3D Viewport.
#. Select a UV mapping method from the UV --> Unwrap menu or
   the UV menu in the 3D Viewport.
#. Adjust the unwrap settings in the Adjust Last Operation <bpy.ops.screen.redo_last> panel.
#. Add a test image to see if there will be any distortion.
   See Applying Images to UVs </modeling/meshes/uv/applying_image>.
#. Adjust UVs in the UV editor. See Editing UVs </modeling/meshes/uv/editing>.
