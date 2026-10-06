---
area: manual-modeling
source: blender-manual
version: 5.2
---

*********************
Musgrave Texture Node
*********************

The Musgrave texture node was replaced by the
Noise Texture </modeling/geometry_nodes/texture/noise> node,
which includes all the same functionality.

- The Dimension input was replaced by a Roughness input, where Roughness = Lacunarity^{-Dimension}.
- The Detail input value must be subtracted by 1 compared to the old Musgrave Texture node.
