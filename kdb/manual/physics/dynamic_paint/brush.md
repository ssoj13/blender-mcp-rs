---
area: manual-physics
source: blender-manual
version: 5.2
---

.. _bpy.types.DynamicPaintBrushSettings:

*****
Brush
*****

The Brush type makes object apply paint on the canvas.

From the first brush panel you can define how brush affects canvas color surfaces.

Paint Color
   Color of the paint.
Alpha
   Defines brush alpha or visibility. Final wetness is also affected by alpha.
Wetness
   Defines how "wet" new paint is. Wetness is visible on "Paint" surface "wetmap".
   Speed of "Drip" and "Spread" effects also depends on how wet the paint is.
Absolute Alpha
   This setting limits brush alpha influence.
   Without it, brush is "added" on surface over and over again each frame,
   increasing alpha and therefore influence of brush on canvas. In many cases however,
   it is preferred to not increase brush alpha if it already is on brushes level.
Erase Paint
   Makes brush dissolve existing paint instead of adding it.

Source
======

*Paint* source setting lets you define how brush influence/intersection is defined.

The Brush affects all surface point inside the mesh volume.

Only uses defined distance to the closest point on brush mesh surface.
Note that inside of the volume is not necessarily affected because it is not close to the surface.

Same as volume type, but also has influence over defined distance.

Inner Proximity
   Applies proximity inside the mesh volume.
Negate Volume
   Negates brush alpha within mesh volume.

.. list-table::

   * - .. figure:: /images/physics_dynamic-paint_brush_source-mesh-volume-proximity-1.png

          The Volume + Proximity brush with no additional settings.

     - .. figure:: /images/physics_dynamic-paint_brush_source-mesh-volume-proximity-2.png

          Inner Proximity. Proximity falloff is now visible inside the volume.

   * - .. figure:: /images/physics_dynamic-paint_brush_source-mesh-volume-proximity-3.png

          Negate Volume. Inner side of the volume has become completely transparent.

     - .. figure:: /images/physics_dynamic-paint_brush_source-mesh-volume-proximity-4.png

          Inner Proximity and Negate Volume enabled together.

Instead of calculating proximity to the brush object mesh, which can be quite slow in some cases,
only the distance to the center is calculated. This is much faster and often good enough.

Brush influence is defined by particles from a selected particle system.

Effect Solid Radius
   Defines the distance, inside which paint is solid color.

Use Particle Radius
   Uses the settings in the particle panel to determine solid radius size.
   Solid Radius size disabled while Particle Radius enabled.

Smooth Radius
   An additional radius outside Solid Radius to add a smooth falloff.

   If you set "Smooth Radius" to zero, particle will be painted as a solid sphere.
   If you set "Solid Radius" to zero, it gets painted as a smooth halo.

Common Options
--------------

Paint Distance
   The maximum distance to mesh surface to affect paint.

Project
   Projects brush to the canvas from a defined direction.
   Basically this can be considered as "direction aligned" proximity.

Falloff
   :Sharp:
      Paints solid paint within the defined distance.
   :Smooth:
      Makes paint to linearly fade out until becoming completely invisible
      when it reaches the maximum distance.
   :Color Ramp:
      Allows you to manually make a custom falloff behavior.

Velocity
========

This panel shows brush options that are based on object velocity.

On top you have a color ramp and several related settings.
Basically the color ramp represents brush velocity values:
left side being zero velocity and right side being the "Max velocity".
Speed is measured in "units per frame".

Checkboxes above can be used to define color ramp influence.

Multiply Alpha
   Uses color ramp's alpha value depending on current velocity and multiplies brush alpha with it.
Replace Color
   Replaces the brush color with the values from the ui-color-ramp-widget.
Multiply Depth
   Multiplies brushes "depth intersection" effect.
   Basically you can adjust displace and wave strength depending on brush speed.
Do Smudge
   Enabling Smudge makes the brush "smudge" (or "smear") existing colors on the surface as it moves.
   The strength of this effect can be defined from the *Smudge Strength* property.

   Even when smudge is enabled brush still does its normal paint effect.
   If you want a purely smudging brush use zero alpha.
   It is also possible to have *Erase* option enabled together with smudge.

Waves
=====

This panel is used to adjust brush influence to "Wave" surfaces.

.. _bpy.types.DynamicPaintBrushSettings.wave_type:

Wave Type
   Select what effect the brush creates in the wave simulation.

   :Depth Change:
      The brush creates waves when the intersection depth with the surface is *changed* on that point.
      If the brush is not moved, it will have no effect.

      Using a negative "Factor" with this type can create a nice looking "wake" for moving objects like ships.
   :Obstacle:
      Constantly affects surface whenever intersecting.
      Waves are also reflected off this brush type.
      However, due to the nature of wave simulation algorithm this type creates
      an unnatural "dent" in the surface if the brush is not moved.
   :Force:
      Directly affects the velocity of wave motion.
      Therefore the effect is not one-to-one with brush intersection depth, yet the force strength depends on it.
   :Reflect Only:
      This type has no visible effect on the surface alone but reflects waves that are already on the surface.

.. _bpy.types.DynamicPaintBrushSettings.wave_factor:

Factor
   Adjusts how strongly brush "depth" affects the simulation.
   You can also use negative values to make brush pull water up instead of down.

.. _bpy.types.DynamicPaintBrushSettings.wave_clamp:

Clamp Waves
   In some cases the brush goes very deep inside the surface messing whole simulation up.
   You can use this setting to "limit" influence to only certain depth.
