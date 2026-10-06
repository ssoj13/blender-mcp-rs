---
area: api-ops
group: sculpt_curves
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.sculpt_curves

- `bpy.ops.sculpt_curves.brush_stroke(stroke: collection, mode: enum{NORMAL, INVERT}, brush_toggle: enum{None, SMOOTH, ERASE, MASK}, pen_flip: boolean)` — Sculpt curves using a brush
- `bpy.ops.sculpt_curves.min_distance_edit()` — Change the minimum distance used by the density brush
- `bpy.ops.sculpt_curves.select_grow(distance: float)` — Select curves which are close to curves that are selected already
- `bpy.ops.sculpt_curves.select_random(seed: int, partial: boolean, probability: float, min: float, constant_per_curve: boolean)` — Randomizes existing selection or create new random selection