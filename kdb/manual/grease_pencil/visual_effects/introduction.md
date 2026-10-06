---
area: manual-grease_pencil
source: blender-manual
version: 5.2
---

************
Introduction
************

Grease Pencil has a special set of viewport real-time visual effects that can be apply to the object.

These effects treat the object as if it was just an image, for that reason they
have an effect on the whole object and cannot limit their influence
on certain parts like layers, materials or vertex group as with modifiers.
Also unlike modifiers, they can not be applied to the object.

Their main purpose is to have a quick way to apply visual effects on your drawings
like blurring, pixelation, wave distortion, among others.

**Note:**

   Visual Effects best fit for quick viewport visualization. You can use it for final renders
   but if you want more precision with effects it is still recommended to use
   the Compositor </compositing/introduction>.

Interface
=========

The visual effects panels and interface are similar to modifiers.
Each effect shares the same basic interface components similar to modifiers for meshes.

See Modifiers Interface <bpy.types.Modifier.show> for more information.
