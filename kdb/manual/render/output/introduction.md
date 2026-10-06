---
area: manual-render
source: blender-manual
version: 5.2
---

.. todo: describe the steps to output renders

************
Introduction
************

The first step in the rendering process is to determine and set the output settings.
This includes render size, frame rate, pixel aspect ratio, output location, and file type.

When rendering a single frame, the output should be a single image format and not a video.
Several image formats </files/media/image_formats> are available.

Images can also be used for rendering animations which has a couple advantages.
For example, when rendering animations to image file formats the render job can be canceled
and resumed at the last rendered frame by changing the frame range.
This is useful if the animation takes a long time to render
and the computers resources are needed for something else.

Images can then be encoded to a video by adding the rendered image sequence into
the Video Sequencer </editors/video_sequencer/sequencer/index> and choosing an appropriate
Video Output </files/media/video_formats>.

**Tip:**

   Rendered image sequences can be played back in the Animation Player <bpy.ops.render.play_rendered_anim>.

**Seealso:**

   See topbar-render
