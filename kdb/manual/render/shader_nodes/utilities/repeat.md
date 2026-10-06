---
area: manual-render
source: blender-manual
version: 5.2
---

Cycles and EEVEE Differences
============================

The number of iterations in *Cycles* must evaluate to a constant value. This means the number of iterations can't be
driven by /render/shader_nodes/input/index or /render/shader_nodes/textures/index.

*EEVEE* doesn't have this limitation, so the number of iterations can be driven by any type of node.
