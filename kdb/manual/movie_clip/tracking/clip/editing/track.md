---
area: manual-movie_clip
source: blender-manual
version: 5.2
---

*****
Track
*****

Transform
=========

.. _bpy.ops.clip.track_markers:

Track Motion
============

The Track Motion menu is used to perform tracking of selected tracks
(i.e. following the selected feature from frame to frame).

This operator depends on settings from the Tracking Settings panel.
If during sequence tracking the algorithm fails to track some markers,
they will be disabled and tracking will continue for the rest of the markers.
If the algorithm fails when tracking frame-by-frame, the marker is not disabled,
and the most likely position of the feature on the next frame is used.

Backwards
---------

Tracks the motion backward along the sequence.

Frame Backwards
---------------

Tracks the motion backward by one frame.

Forwards
--------

Tracks the motion forward along the whole sequence.

Frame Forwards
--------------

Tracks the motion forward one frame.

Clear
=====

.. _bpy.ops.clip.clear_track_path:

Before
------

Deletes all tracked and keyframed markers after the current frame for all selected tracks.

Clear Active
   Limits clear action to only active track (as opposite to all selected ones).

After
-----

Deletes all tracked and keyframed markers before the current frame for all selected tracks.

Clear Active
   Limits clear action to only active track (as opposite to all selected ones).

Track Path
----------

Clears all markers except the current one from all selected tracks.

Clear Active
   Limits clear action to only active track (as opposite to all selected ones).

Clear Solution
--------------

.. _bpy.ops.clip.refine_markers:

Refine
======

This operator will run a tracker from previous keyframe to current frame for all selected markers.
Current markers positions are considering initial position guess
which could be updated by a tracker for better match.

Useful in cases when feature disappears from the frame and then appears again. Usage in this case is the following:

- When feature point re-appeared on frame, manually place marker on it.
- Use Refine Markers operation to allow tracker to find a better match.

Depending on direction of tracking use either *Forwards* or *Backwards* refining.
Accordingly if tracking happens forwards, use *Refine Forwards*, otherwise use *Refine Backwards*.

Backwards
---------

Refine the track backwards.

Forwards
--------

Refine the track forwards.

.. _bpy.ops.clip.add_marker_move:

Add Marker
==========

Places a new marker at the position of the mouse
(which is under the button in this case, not ideal but it is just how things work)
and then it can be moved to the needed location. When it is moved to the desired position,
LMB can be used to finish placing the new marker.
Also, Return and Spacebar can be used to finish placing the marker.
But it is faster to use Ctrl-LMB to place markers directly on the footage.
This shortcut will place the marker in the place you have clicked.

In addition to this until you have released the mouse button,
you can adjust the marker position by moving the mouse and
using the track preview widget to control how accurately the marker is placed.

.. _bpy.ops.clip.detect_features:

Detect Features
===============

Detects all possible features on the current frame and places markers at these features.
This operator does not take other frames into account,
so it might place markers on features which belong to moving objects.
If the camera is turning away from this shot,
no markers could be present within the frames after the camera moved away.

There are several properties for this operator:

Placement
   Controls where to place markers.

   Whole Frame
      Places markers throughout the whole frame.
   Inside Annotated Area
      Places markers inside the area outlined with the tool-annotate.
      This can be used to outline some areas with interesting features
      and place markers only inside the outlined area.
   Outside Annotated Area
      Places markers outside the area outlined with the tool-annotate.
      This can be used to outline areas of no interest (like trees, humans, etc.)
      and place markers outside of these areas.
Margin
   Controls the distance from the image boundary for created markers.
   If markers are placed too close to the image boundary,
   they will fail to track really quickly and they should be deleted manually.
   To reduce the amount of manual clean-up, this parameter can be used.
Threshold
   Limits minimal threshold for placing markers.
   This value comes from the feature detection algorithm and it means:
   low values means most probably this feature would fail to track very soon,
   high value means it is not much such track.
   Amount of markers to be added can be controlled with this value.
Distance
   Defines the minimal distance between placed markers.
   It is needed to prevent markers from being placed too close to each other
   (such placement can confuse the camera solver).

.. _clip-tracking-plane:
.. _bpy.ops.clip.create_plane_track:

Create Plane Track
==================

The *Create Plane Track* operator creates a new plane track.
Planar tracking takes advantage of the fact that there are often planar surfaces in footage,
by attaching markers to points on these flat planes.
It can be used to replace things like billboards and screens on the footage with another image or video.
It also might be used for masking.

This button will create a plane object
which is deforming in the same way as plane defined by all selected point tracks.
At least four feature points tracked across the footage which belongs to
the plane you want to replace are needed. More tracks will give better estimation of plane motion.

Feature points used to estimate plane motion could be used from any place on the plane,
meaning it's not necessarily need to be corners. Corners are not always easy to be tracked,
they might be occluded. In this case you can position tracked features that lay on the same plane
far away from the actual plane which should be replaced.

This provides more information about the possible deformation of the marker in following frames,
and such markers can be tracked even if partially occluded (appear and disappear during the time).
It is only required that two neighbor frames have at least four common tracks.

An image can be projected onto the plane with
the /compositing/types/tracking/plane_track_deform compositing node.

.. _bpy.ops.clip.solve_camera:

Solve Camera/Object Motion
==========================

The *Solve Camera/Object Motion* operator solves the motion of the camera or the
selected tracked object </movie_clip/tracking/clip/sidebar/track/objects>,
using all tracks placed on the footage and two keyframes specified on this panel.
There are some requirements:

- There should be at least eight common tracks on the both of the selected keyframes.
- There should be noticeable parallax effects between these two keyframes.

If everything goes smoothly during the solve, the average reprojection error is reported to
the information space and to the Clip editor header. Reprojection error means the average
distance between reconstructed 3D position of tracks projected back to footage and
original position of tracks. Basically, reprojection error below 0.3 means accurate reprojection,
(0.3 - 3.0) means quite nice solving which still can be used.
Values above 3 means some tracks should be tracked more accurately,
or that values for focal length or distortion coefficients were set incorrectly.

.. (todo 2.62) object solver

.. _bpy.ops.clip.join_tracks:

Join Tracks
===========

This operator joins all selected tracks into one.
Selected tracks should not have common tracked or keyframed markers at the same frame.

.. (wip)
   Joining two tracks now works better for tracks which have got intersection by frames:
   coordinates of joined track would be interpolated linearly on segments with intersection.
   This is still not perfect from accurate solving point of view,
   but this allows to prevent camera jump which is much more annoying than sight camera slide.

.. _bpy.ops.clip.average_tracks:

Average Tracks
==============

The Average Tracks operator creates a new tracking marker by averaging the data from the selected tracks.
This can be used to improve stability of tracking on blurry or non-very-sharp feature shapes.
The operator takes into account all Marker properties </movie_clip/tracking/clip/sidebar/track/marker>
however, disabled markers do not affect the averaging.

Gaps in the original tracks will be linearly interpolated, to reduce result track jump.
Note that this only applies to gaps "in between".
This means that if a track does not have markers in the beginning or end of it,
there is nothing to interpolate with and the resulting track will jump.

Keep Original
   When enabled, the selected tracks are *not* deleted.

Copy Tracks
===========

Paste Tracks
============

Animation
=========

Show/Hide
=========

Clean Up
========

.. _bpy.ops.clip.clean_tracks:

Clean Tracks
------------

Identifies all tracks which match settings from above and performs desired action on them.

Tracked Frames
   Tracks or tracked segments shorter than this number of frames will be removed.
Reprojection Error
   Tracks which have reprojection error higher than this value will be removed.
Action
   Several actions can be performed for bad tracks.

   Select
      They can simply be selected.
   Delete Track
      The whole track can be deleted.
   Delete Segments
      Bad segments of tracked sequence can be removed.

.. _bpy.ops.clip.filter_tracks:

Filter Tracks
-------------

This operator deletes obviously bad tracks (for example, the ones which are too short).
Additionally, it identifies tracks which have suspicious spikes in their motion and selects them.

.. _bpy.ops.clip.delete_track:

Delete Track
============

Delete all selected tracks.

.. _bpy.ops.clip.delete_marker:

Delete Marker
=============
