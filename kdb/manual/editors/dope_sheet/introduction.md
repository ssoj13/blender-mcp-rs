---
area: manual-editors
source: blender-manual
version: 5.2
---

************
Introduction
************

The Dope Sheet offers a bird's-eye view of the keyframes inside the scene.
It's inspired by classical hand-drawn animation, where animators make use of a chart
showing exactly when each drawing, sound, and camera move will occur, and for how long.
It is also possible to show Playback Controls <animation-editors-footer> to control the playhead.

Dope Sheet Modes
================

The editor has several different modes that can be selected from a dropdown in the header.
The default *Dope Sheet* mode gives an overview of most types of animatable data.
For others, such as masks, you need to switch to a more specific mode.

The modes are as follows:

- Dope Sheet
- Action Editor </editors/dope_sheet/modes/action>
- Shape Key Editor </editors/dope_sheet/modes/shape_key>
- Grease Pencil </editors/dope_sheet/modes/grease_pencil>
- Mask </editors/dope_sheet/modes/mask>
- Cache File: originally meant to show the baked animation data in /files/import_export/alembic
  files, but never implemented.

Main Region
===========

The Dope Sheet Editor shows a stack of channels </editors/graph_editor/channels/index>
(animatable properties), and for each channel, a series of keyframes laid out along the time axis.

Keyframes can take on various colors and shapes:

.. list-table::
   :widths: 20 80

   * - Gray
     - Unselected
   * - Yellow
     - Selected
   * - Other colors
     - Custom keyframe tag set by the user (Key --> Keyframe Type)
   * - Diamond
     - Free Keyframe Handle (Key --> Handle Type)
   * - Round
     - Auto-Clamped Keyframe Handle
   * - Circle
     - Automatic Keyframe Handle
   * - Square
     - Vector Keyframe Handle
   * - Rhombus
     - Aligned Keyframe Handle
   * - Gray bar between keys
     - Held key (the two keyframes are identical)
   * - Green line between keys
     - The curve segment uses custom interpolation (Key --> Interpolation Mode)
   * - Upwards arrow
     - Local maximum in curve (visible if View --> Show Curve Extremes is enabled)
   * - Downwards arrow
     - Local minimum in curve

Keyframes can be selected by clicking and moved by dragging. See the Select <dopesheet-select-menu>
and Key <dopesheet-key-menu> menus for more options.
