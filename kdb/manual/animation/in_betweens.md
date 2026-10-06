---
area: manual-animation
source: blender-manual
version: 5.2
---

***********
In-Betweens
***********

These tools create a new pose which is based on the keys surrounding the current frame.
This can help to quickly build up in-betweens from already existing key poses.

.. _bpy.ops.pose.blend_with_rest:

Blend Pose with Rest Pose
=========================

*Blend with Rest Pose* linearly interpolates the current pose to the rest position.
When blending with a negative factor, the pose is moved away from the rest position.
Only one keyframe is needed for this tool unlike two for the other.

.. _bpy.ops.pose.push:

Push Pose from Breakdown
========================

*Push Pose* interpolates the current pose away from the *linear interpolation between the surrounding keys*.
On your current frame, imagine a pose that is linearly interpolated from the surrounding keys.
That is the base pose that this tool interpolates away from.

.. _bpy.ops.pose.relax:

Relax Pose to Breakdown
=======================

*Relax Pose* is the opposite of *Push Pose from Breakdown*.
Hence at full influence, the resulting pose will be identical to a linear interpolation
between the surrounding keys.

.. _bpy.ops.pose.breakdown:

Pose Breakdowner
================

Creates a suitable breakdown pose on the current frame.

The Breakdowner tool can be constrained to work on specific transforms and axes,
by pressing the following keys while the tool is active:

- G, R, S: move, rotate, scale
- B: Bendy bones
- C: custom properties
- X, Y, Z: to the corresponding axes

.. _bpy.ops.pose.blend_to_neighbor:

Blend to Neighbor
=================

Transitions the current pose with the neighboring keyframes in the timeline.
In order for this operator to work, there must be a keyframe before and after the current frame.
