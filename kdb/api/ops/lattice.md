---
area: api-ops
group: lattice
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.lattice

- `bpy.ops.lattice.flip(axis: enum{U, V, W})` — Mirror all control points without inverting the lattice deform
- `bpy.ops.lattice.make_regular()` — Set UVW control points a uniform distance apart
- `bpy.ops.lattice.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Change selection of all UVW control points
- `bpy.ops.lattice.select_less()` — Deselect vertices at the boundary of each selection region
- `bpy.ops.lattice.select_mirror(axis: enum{X, Y, Z}, extend: boolean)` — Select mirrored lattice points
- `bpy.ops.lattice.select_more()` — Select vertices directly linked to already selected ones
- `bpy.ops.lattice.select_random(ratio: float, seed: int, action: enum{SELECT, DESELECT})` — Randomly select UVW control points
- `bpy.ops.lattice.select_ungrouped(extend: boolean)` — Select vertices without a group