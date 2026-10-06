# Node & shader recipes

A cookbook of Blender 5 Geometry-Nodes and Shader-Nodes setups to copy from — Fresnel and
rim effects, procedural materials, scattering and instancing, hybrid geo→shader tricks.
Build them with `node_graph_build` and check socket names with `node_schema` (Blender 5
renamed most of them). The text below is kept verbatim as a prompt/reference base.

---

General rules for all prompts (System Prompt base):

"You are an expert in Blender 5 Geometry Nodes and Shader Nodes. Answer only with node setups, node groups, exact steps. Don't invent nonexistent nodes. Use Fields where possible. For materials — Principled BSDF + Mix Shader. Check IOR, roughness, subsurface. Add Node Groups for reusability. If needed — describe math nodes precisely."
"Don't wander: answer only the request, give a minimal working setup + variations. Use Color Ramp, Map Range, Vector Math, Mix RGB."
"For Geo Nodes: always start with Group Input → Transform/Modify → Group Output. Capture attributes to pass data to the shader."
"Version: Blender 5.x. Mind performance (Realize Instances sparingly, Instance on Points)."

1. Shading Nodes / Materials — Fresnel & Rim effects (example from your request)

Fresnel facing ratio on Color Ramp (rim/fresnel glow): Layer Weight (Facing) → Color Ramp (black to white, eased) → Emission Strength or Mix Shader (Base + Emission/Glossy). Factor = 1 - Facing for rim.
Custom Fresnel dielectric: Fresnel node (IOR) → Mix Shader (Glossy + Refraction/Principled). For roughness — separate glossy mix.
Rim lighting glowing: Layer Weight Facing → Map Range (0.8-1.0 to 0-1) → Color Ramp (color to bright) → Add Shader or Emission.
Fresnel + roughness control: Fresnel → Power node (for falloff) → Mix with Base Color/Roughness.
Metallic edge wear: Fresnel (high IOR) → Mix RGB (darken metallic) on Roughness/Metallic input.
Glass with absorption: Principled BSDF (Transmission=1, IOR=1.45) + Volume Absorption (color tint) inside.
Thin film iridescence: Layer Weight + Noise/Musgrave → Color Ramp (rainbow) mixed onto Base Color + Fresnel factor.
Velvet/anisotropic rim: Principled + Anisotropic BSDF mixed with Fresnel.
Fake subsurface rim: Fresnel → Subsurface Radius/Scale boost.
Toon rim outline: Fresnel → Color Ramp (sharp steps) → Emission black/white for outlines.

More shading (20+):
11. Procedural wood: Noise + Wave Texture (distorted) → Color Ramp (brown tones) + Bump.
12. Procedural marble: Voronoi + Musgrave mix → Color Ramp.
13. Rust/wear: Noise (high detail) → Color Ramp (base to rust) mixed by Vertex Colors or AO.
14. Leather: Voronoi cracks + Noise displacement + Glossy mix.
15. Snow/ice: Principled (high Roughness, Transmission) + White Noise for sparkle.
16. Fabric/cloth: Musgrave + Bump + low Subsurface.
17. Metal scratches: Wave Texture anisotropic + Mix on Roughness.
18. Water puddle: Glossy + Normal Map from Noise + Reflection.
19. Procedural brick: Brick Texture + Displacement.
20. Moss/organic: Voronoi cells + Green Color Ramp + Displacement.
21. Subsurface multi-layer: Mix several Principled with different Radius (red/green/blue scatter).
22. Emission flicker: Noise Texture (low frequency) → Math (sin or map) on Emission Strength.
23. AO fake shadows: Ambient Occlusion node mixed onto Color.
24. Curvature wear: Geometry node attribute (Curvature) captured → Shader for edge damage.
25. Object-mapped texture: Texture Coordinate (Object) + Mapping node for triplanar or decal projection.
(Continue in the same style: combine Noise/Voronoi/Musgrave/Wave + Ramps + Mix + Bump/Normal.)
2. Geometry Nodes — Scattering, Instancing, Deformation
From the "cookbook" style.

Simple scatter on surface: Distribute Points on Faces (Poisson) → Instance on Points (collection) → Random Rotation/Scale.
Density by height/slope: Capture Position/Normal → Math (dot with Up vector) → Map Range → Density.
Phyllotaxis pattern (plants): Math nodes for spiral (golden angle) + Instance.
Curve extrusion with profile: Curve to Mesh + Set Curve Radius + Profile curve.
Noise displacement surface: Set Position → Noise Texture (object coords) → Vector Math multiply → Offset.
Quad-sphere primitive: Subdiv Cube → Position normalize (optionally tan trick for even faces).
Points inside volume/mesh: Distribute Points in Volume or Mesh to Volume + Points.
Custom instance probabilities: Random Value + Switch or Delete by probability.
Knurl/knurling effect: Small cylinders or patterns on surface via instance + boolean.
Shortest path selection: Shortest Path node for vines/roads.
Spline resampling random: Sample Curve with Factor + Random.
Merge by proximity/index: Merge by Distance + Capture Index.
Manifold solidify: Extrude + Scale + Boolean clean.
Procedural grass: Curve lines + Instance grass blades + Wind force via Simulation or Set Position sine.
Rock/cliff generation: Voronoi fracture + Extrude + Noise displace.

More Geo (many variations):
41-50: Various primitives (point cube/sphere, grid variations, icosphere from points).
51. UV unwrapping in Geo: Unwrap node or attribute pass to shader.
52. SDF organic shapes: Math distance fields for blobs.
53. Scatter with weight paint: Capture Vertex Group → Density.
54. Roads on terrain: Curve from spline + Project to surface.
55. Foliage variation: Capture seed per instance → Random color/scale per point.
56. Array with rotation: Duplicate Elements + Rotate Instances.
57. Boolean knife project: Mesh Boolean in Geo.
58. Vortex/attraction forces: Simulation Zone + Forces.
59. LOD instances: Distance to camera → Switch low/high poly.
60. Procedural city blocks: Grid + Random extrude heights.
3. Hybrid: Geo → Shader + Advanced Tricks

Capture curvature/edges for shader: In Geo — Edge Angle or Mean Curvature attribute → Pass to material for wear.
Position-based coloring: Capture Position → Map to Color Ramp in shader.
Instance-aware materials: Random per instance attribute → Shader variation.
Triplanar mapping procedural: Vector Math + separate XYZ + Mix.
Fake volume edges: Geometry nodes thin shell + Fresnel in shader.
66-80: Combinations of Noise + Distortion (Twist, Bend modifiers via Geo), wave propagation, particle trails, etc.

Math-heavy (for LLM stability):
81. Map Range everywhere: Input 0-1 → desired range + Clamp.
82. Vector Math for directions: Normalize, Dot Product for angles/Fresnel fake.
83. Power/Exp for falloff: Instead of linear ramps.
84. Modulo for tiling patterns.
85. Fractal noise layering: Multiple Noise + different scales + add/multiply.
Performance & Organization:
86. Node Groups: All reusable setups into groups (Fresnel Rim, Procedural Base, Scatter Setup).
87. Attributes instead of drivers where possible.
88. Viewer Node for debug (in Geo 3.0+).
89. Realize Instances only at the end.
90. Collection Info for asset libraries.