---
area: manual-render
source: blender-manual
version: 5.2
---

************
Introduction
************

Rendering is the process of turning a 3D scene into a 2D image.
Blender includes three render engines with different strengths:

- EEVEE </render/eevee/index> is a physically based realtime renderer.
- Cycles </render/cycles/index> is a physically based path tracer.
- Workbench </render/workbench/index> is designed for layout, modeling and previews.

More renderers from third-party developers are available as
add-ons </editors/preferences/extensions>.
Each renderer has its own render settings to control render quality and performance.

What the render looks like is defined by cameras </render/cameras>,
lights </render/lights/index> and materials </render/materials/index>.
These are shared between EEVEE and Cycles, however some features are only supported in one or the other.

Renders can be split up into layers and passes </render/layers/index>, which can then
be composited </compositing/index> together for creative control, or to combine
with real footage. Freestyle </render/freestyle/index> can be used to
add non-photorealistic line rendering.

Blender supports interactive 3D viewport rendering for all render engines, for quick iteration
on lighting and shading. Once this is done, the final quality image or animation can
be rendered and output </render/output/index>.
