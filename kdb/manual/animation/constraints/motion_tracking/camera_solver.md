---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.CameraSolverConstraint:

************************
Camera Solver Constraint
************************

The *Camera Solver* constraint makes a Blender camera imitate the motion of a real-world camera.

Usage
=====

Start by loading a video file into the Movie Clip Editor </editors/clip/introduction>
and using motion tracking </movie_clip/tracking/introduction> to track at least
eight markers </movie_clip/tracking/clip/marker> in the real-world scene.
Then use bpy.ops.clip.solve_camera to reconstruct the motion of the physical camera,
and finally add this constraint to a Blender camera.

Options
=======

Active Clip
   Whether to follow the physical camera of the scene's Active Clip <bpy.types.Scene.active_clip>.
   If unchecked, a selector appears for choosing another clip.

Constraint to F-Curve
   Replaces the constraint by a set of equivalent keyframes </animation/keyframes/introduction>.

bpy.types.constraint.influence
   How strongly the constraint affects the Blender camera.
