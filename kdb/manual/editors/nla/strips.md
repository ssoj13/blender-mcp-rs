---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.types.NlaStrip:

******
Strips
******

A strip tells the animation when something happens and for how long.
There are a few different types which are described below.

.. _bpy.ops.nla.actionclip_add:

Action Strips
=============

An action strip plays the keyframes inside an action </animation/actions>.
You can create one using Add --> Action. Another way is to click
Push Down Action <bpy.ops.nla.action_pushdown> in the NLA's Action Track --
this will create a strip based on the object's active action.

Multiple strips can reference the same action, so that you can potentially change
multiple parts of the animation by editing a single set of keyframes.

A strip can be shorter than its underlying action, be it through cropping,
speeding up, or both. It can also be longer than its underlying action,
be it through extending, slowing down, or both. See the
Sidebar <nla-sidebar-action-clip> for details.

.. _bpy.ops.nla.transition_add:

Transition Strips
=================

A transition strip interpolates between two neighboring action strips.
Select them and click Add --> Transition.

.. _bpy.ops.nla.soundclip_add:

Sound Strips
============

These strips control when a /render/output/audio/speaker starts to playback the audio.
Playback continues the length of audio file and does not take into account the length of the sound strip.

Meta Strips
===========

A meta strip groups other strips together, letting you move, scale, and copy them as one combined unit.
