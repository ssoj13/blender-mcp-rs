---
area: api-ops
group: mball
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.mball

- `bpy.ops.mball.delete_metaelems(confirm: boolean)` — Delete selected metaball element(s)
- `bpy.ops.mball.duplicate_metaelems()` — Duplicate selected metaball element(s)
- `bpy.ops.mball.duplicate_move(MBALL_OT_duplicate_metaelems: pointer, TRANSFORM_OT_translate: pointer)` — Make copies of the selected metaball elements and move them
- `bpy.ops.mball.hide_metaelems(unselected: boolean)` — Hide (un)selected metaball element(s)
- `bpy.ops.mball.reveal_metaelems(select: boolean)` — Reveal all hidden metaball elements
- `bpy.ops.mball.select_all(action: enum{TOGGLE, SELECT, DESELECT, INVERT})` — Change selection of all metaball elements
- `bpy.ops.mball.select_random_metaelems(ratio: float, seed: int, action: enum{SELECT, DESELECT})` — Randomly select metaball elements
- `bpy.ops.mball.select_similar(type: enum{TYPE, RADIUS, STIFFNESS, ROTATION}, threshold: float)` — Select similar metaballs by property types