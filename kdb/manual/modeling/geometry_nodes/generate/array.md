---
area: manual-modeling
source: blender-manual
version: 5.2
---

**********
Array Node
**********

The *Array* node creates multiple copies of an input geometry and arranges them in
a linear, circular, or custom pattern. It offers flexible options for count, spacing,
and randomization, allowing for both precise and organic duplication of geometry.

Inputs
======

Geometry
   The geometry to duplicate.

.. section-inputs

Shape
   The method used for arranging the duplicates.

   :Line: Arranges copies along a straight line.
   :Circle: Arranges copies around a circle.
   :Curve: Arranges copies along a curve input.
   :Transform: Uses a transform offset or object transform to position each copy.

Offset Method Line
   Determines how copies are spaced along a line.

   :Relative: Offsets each copy relative to the bounding box size of the geometry.
   :Offset: Offsets copies by a fixed world-space distance.
   :Endpoint: Distributes copies evenly between start and end points.

Offset Line Relative
   The amount to move each copy relative to its bounding box size when using *Relative* mode.

Count Method Circle Curve
   Determines how the number of duplicates is defined.

   :Count: Sets a fixed number of copies.
   :Distance: Sets copies based on distance between them.

Count Circle Curve Count
   The number of copies to generate.

Angular Distance Circle Distance
   The angular distance between copies when using *Circle* shape.

Central Axis Circle
   The axis used as the up direction for circular arrangements.

Circle Segment Circle
   Determines whether the copies form a full circle or just a partial arc.

   :Full: Copies are evenly spaced around a full 360° circle.
   :Arc: Copies fan out along a circular arc.

Radius Circle
   The radius of the circle along which the copies are distributed.

Sweep Angle Circle Arc
   The total angle (in radians or degrees) spanned by the arc when *Circle Segment* is set to *Arc*.

Per Curve Curve
   Specify the number of copies for each curve separately.

Curve Object Curve
   Reference curve object for array.

Transform Reference Transform
   Defines how the transform is applied for *Transform* shape.

   :Input: Uses the transform specified directly in the node inputs.
   :Object: Uses the transform of a specified object as the reference.

Translation Inputs Line Offset Endpoint
   The positional offset applied to each copy.

Rotation Inputs Line
   The rotation offset applied per copy.

Scale Inputs Line
   The scale factor applied per copy.

Transform Object Transform Object
   The object whose transform is used when *Transform Reference* is set to *Object*.

Relative Space Transform Object
   Determines whether the transform is applied in object or world space.

Realize Instances
   Converts the generated instances into real geometry.
   This must be enabled for certain operations such as merging or boolean operations.

Align Rotation Circle Curve
---------------------------------------------------

Automatically aligns the rotation of instances to follow the direction of the array distribution.

- For *Circle*, instances are aligned tangentially around the circle while adapting
  their base orientation to the selected *Central Axis*.
- For *Curve*, instances are aligned to the curve tangent and orientation,
  allowing geometry to naturally follow the shape of the curve.

When disabled, all instances keep the original rotation of the input geometry.

Local Rotation X, Y, Z
   Additional local rotation offsets applied after automatic alignment.

   This is useful for correcting the orientation of geometry when the source object
   does not initially face the desired forward or up axis.

Randomize
---------

Offset
   Maximum random translation offset for each axis, defining the variation in instance position.

Rotation
   Maximum random rotation for each axis, defining how much each instance can be rotated.

Scale Axes
   Determines how scaling randomization is applied.

   :Uniform: Applies a single random scale factor uniformly to all axes.
   :Axes: Applies independent random scale factors per axis.

Scale
   Maximum random scaling factor applied to each instance.

Flipping
   Randomly mirrors instances along one or more axes.

Exclude First / Last
   Excludes the first (original) or last copy from randomization effects.

Seed
   Initializes the random number generator.
   Changing the seed produces a different random arrangement while preserving other settings.

Merge
-----

Merges overlapping points or vertices by distance.
Requires *Realize Instances* to be enabled.

Distance
   The distance threshold within which points are merged.

.. section-outputs

Outputs
=======

Geometry
   The resulting geometry containing all duplicated and transformed instances.
