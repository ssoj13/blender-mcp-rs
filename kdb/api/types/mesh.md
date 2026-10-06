---
area: api-types
group: mesh
version: 5.2.2
---

# Blender 5.2.2 RNA types: mesh

## AttributeGroupMesh

Group of geometry attributes

- `active` · pointer · -> Attribute — Active attribute
- `active_index` · int — Active attribute index or -1 when none are active
- `active_color` · pointer · -> Attribute — Active color attribute for display and editing
- `active_color_index` · int — Active color attribute index
- `render_color_index` · int — The index of the color attribute used as a fallback for rendering
- `default_color_name` · string — The name of the default color attribute used as a fallback for rendering
- `active_color_name` · string — The name of the active color attribute for display and editing

## Mesh

Mesh data-block defining geometric surfaces

- `vertices` · collection · [] of MeshVertices · read-only — Vertices of the mesh
- `edges` · collection · [] of MeshEdges · read-only — Edges of the mesh
- `loops` · collection · [] of MeshLoops · read-only — Loops of the mesh (face corners)
- `polygons` · collection · [] of MeshPolygons · read-only — Polygons of the mesh
- `normals_domain` · enum · {POINT, FACE, CORNER} · read-only — The attribute domain that gives enough information to represent the mesh's normals
- `vertex_normals` · collection · read-only — The normal direction of each vertex, defined as the average of the surrounding face normals
- `polygon_normals` · collection · read-only — The normal direction of each face, defined by the winding order and position of its vertices
- `corner_normals` · collection · read-only — The "slit" normal direction of each face corner, influenced by vertex normals, sharp faces, sharp edges, and custom normals. May be empty.
- `loop_triangles` · collection · [] of MeshLoopTriangles · read-only — Tessellation of mesh polygons into triangles
- `loop_triangle_polygons` · collection · read-only — The face index for each loop triangle
- `texture_mesh` · pointer · -> Mesh — Use another mesh for texture indices (vertex indices must be aligned)
- `uv_layers` · collection · [] of UVLoopLayers · read-only — All UV loop layers
- `uv_layer_clone` · pointer · -> MeshUVLoopLayer — UV loop layer to be used as cloning source
- `uv_layer_clone_index` · int — Clone UV loop layer index
- `uv_layer_stencil` · pointer · -> MeshUVLoopLayer — UV loop layer to mask the painted area
- `uv_layer_stencil_index` · int — Mask UV loop layer index
- `vertex_colors` · collection · [] of LoopColors · read-only — Legacy vertex color layers. Deprecated, use color attributes instead.
- `skin_vertices` · collection · read-only — All skin vertices
- `attributes` · collection · [] of AttributeGroupMesh · read-only — Geometry attributes
- `color_attributes` · collection · [] of AttributeGroupMesh · read-only — Geometry color attributes
- `remesh_voxel_size` · float — Size of the voxel in object space used for volume evaluation. Lower values preserve finer details.
- `remesh_voxel_adaptivity` · float — Reduces the final face count by simplifying geometry where detail is not needed, generating triangles. A value greater than 0 disables Fix Poles.
- `use_remesh_fix_poles` · boolean — Produces fewer poles and a better topology flow
- `use_remesh_preserve_volume` · boolean — Projects the mesh to preserve the volume and details of the original mesh
- `use_remesh_preserve_attributes` · boolean — Transfer all attributes to the new mesh
- `remesh_mode` · enum · {VOXEL, QUAD}
- `use_mirror_x` · boolean — Enable symmetry in the X axis
- `use_mirror_y` · boolean — Enable symmetry in the Y axis
- `use_mirror_z` · boolean — Enable symmetry in the Z axis
- `use_mirror_vertex_groups` · boolean — Mirror the left/right vertex groups when painting. The symmetry axis is determined by the symmetry settings.
- `radial_symmetry` · int · [3] — Number of mirrored regions around a central axis
- `has_custom_normals` · boolean · read-only — True if there is custom normal data for this mesh
- `texco_mesh` · pointer · -> Mesh — Derive texture coordinates from another mesh
- `shape_keys` · pointer · -> Key · read-only
- `use_auto_texspace` · boolean — Adjust active object's texture space automatically when transforming object
- `use_mirror_topology` · boolean — Use topology based mirroring (for when both sides of mesh have matching, unique topology)
- `use_paint_bone_selection` · boolean — Bone selection during painting
- `use_paint_mask` · boolean — Face selection masking for painting
- `use_paint_mask_vertex` · boolean — Vertex selection masking for painting
- `total_vert_sel` · int · read-only — Selected vertex count in editmode
- `total_edge_sel` · int · read-only — Selected edge count in editmode
- `total_face_sel` · int · read-only — Selected face count in editmode
- `is_editmode` · boolean · read-only — True when used in editmode
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `auto_texspace` · boolean — Adjust active object's texture space automatically when transforming object
- `texspace_location` · float · [3] — Texture space location
- `texspace_size` · float · [3] — Texture space size
- `materials` · collection · [] of IDMaterials · read-only
- `cycles` · pointer · -> CyclesMeshSettings · read-only — Cycles mesh settings

## MeshAutomaskingSettings

Automasking settings for mesh painting & sculpting.

- `use_automasking_topology` · boolean — Affect only vertices connected to the active vertex under the brush
- `use_automasking_face_sets` · boolean — Affect only vertices that share face sets with the active vertex
- `use_automasking_boundary_edges` · boolean — Do not affect non manifold boundary edges
- `use_automasking_boundary_face_sets` · boolean — Do not affect vertices that belong to a face set boundary
- `use_automasking_cavity` · boolean — Do not affect vertices on peaks, based on the surface curvature
- `use_automasking_cavity_inverted` · boolean — Do not affect vertices within crevices, based on the surface curvature
- `use_automasking_custom_cavity_curve` · boolean — Use custom curve
- `use_automasking_start_normal` · boolean — Affect only vertices with a similar normal to where the stroke starts
- `use_automasking_view_normal` · boolean — Affect only vertices with a normal that faces the viewer
- `use_automasking_view_occlusion` · boolean — Only affect vertices that are not occluded by other faces (slower performance)
- `boundary_edges_propagation_steps` · int — Distance where boundary edge automasking is going to protect vertices from the fully masked edge
- `cavity_factor` · float — The contrast of the cavity mask
- `cavity_blur_steps` · int — The number of times the cavity mask is blurred
- `cavity_curve` · pointer · -> CurveMapping · read-only — Curve used for the sensitivity
- `cavity_curve_op` · pointer · -> CurveMapping · read-only — Curve used for the sensitivity
- `start_normal_limit` · float — The range of angles that will be affected
- `start_normal_falloff` · float — Extend the angular range with a falloff gradient
- `view_normal_limit` · float — The range of angles that will be affected
- `view_normal_falloff` · float — Extend the angular range with a falloff gradient

## MeshEdge

Edge in a Mesh data-block

- `vertices` · int · [2] — Vertex indices
- `select` · boolean
- `hide` · boolean
- `use_seam` · boolean — Seam edge for UV unwrapping
- `use_edge_sharp` · boolean — Sharp edge for shading
- `is_loose` · boolean · read-only — Edge is not connected to any faces
- `index` · int · read-only — Index of this edge

## MeshLoop

Loop in a Mesh data-block

- `vertex_index` · int — Vertex index
- `edge_index` · int — Edge index
- `index` · int · read-only — Index of this loop
- `normal` · float · [3] · read-only — The normal direction of the face corner, taking into account sharp faces, sharp edges, and custom normal data
- `tangent` · float · [3] · read-only — Local space unit length tangent vector of this vertex for this face (must be computed beforehand using calc_tangents)
- `bitangent_sign` · float · read-only — Sign of the bitangent vector of this vertex for this face (must be computed beforehand using calc_tangents, bitangent = bitangent_sign * cross(normal, tangent))
- `bitangent` · float · [3] · read-only — Bitangent vector of this vertex for this face (must be computed beforehand using calc_tangents, use it only if really needed, slower access than bitangent_sign)

## MeshLoopColor

Vertex loop colors in a Mesh

- `color` · float · [4] — Color in sRGB color space

## MeshLoopColorLayer

Layer of vertex colors in a Mesh data-block

- `name` · string — Name of Vertex color layer
- `active` · boolean — Sets the layer as active for display and editing
- `active_render` · boolean — Sets the layer as active for rendering
- `data` · collection · read-only

## MeshLoopTriangle

Tessellated triangle in a Mesh data-block

- `vertices` · int · [3] · read-only — Indices of triangle vertices
- `loops` · int · [3] · read-only — Indices of mesh loops that make up the triangle
- `polygon_index` · int · read-only — Index of mesh face that the triangle is a part of
- `normal` · float · [3] · read-only — Local space unit length normal vector for this triangle
- `split_normals` · float · [9] · read-only — Local space unit length custom normal vectors of the face corners of this triangle
- `area` · float · read-only — Area of this triangle
- `index` · int · read-only — Index of this loop triangle
- `material_index` · int · read-only — Material slot index of this triangle
- `use_smooth` · boolean · read-only

## MeshNormalValue

Vector in a mesh normal array

- `vector` · float · [3] · read-only — 3D vector

## MeshPolygon

Polygon in a Mesh data-block

- `vertices` · int · [3] — Vertex indices
- `loop_start` · int — Index of the first loop of this face
- `loop_total` · int · read-only — Number of loops used by this face
- `material_index` · int — Material slot index of this face
- `select` · boolean
- `hide` · boolean
- `use_smooth` · boolean
- `normal` · float · [3] · read-only — Local space unit length normal vector for this face
- `center` · float · [3] · read-only — Center of this face
- `area` · float · read-only — Read only area of this face
- `index` · int · read-only — Index of this face

## MeshPolygons

Collection of mesh polygons

- `active` · int — The active face for this mesh

## MeshSkinVertex

Per-vertex skin data for use with the Skin modifier

- `radius` · float · [2] — Radius of the skin
- `use_root` · boolean — Vertex is a root for rotation calculations and armature generation, setting this flag does not clear other roots in the same mesh island
- `use_loose` · boolean — If vertex has multiple adjacent edges, it is hulled to them directly

## MeshSkinVertexLayer

Per-vertex skin data for use with the Skin modifier

- `name` · string — Name of skin layer
- `data` · collection · read-only

## MeshStatVis

- `type` · enum · {OVERHANG, THICKNESS, INTERSECT, DISTORT, SHARP} — Type of data to visualize/check
- `overhang_min` · float — Minimum angle to display
- `overhang_max` · float — Maximum angle to display
- `overhang_axis` · enum · {POS_X, POS_Y, POS_Z, NEG_X, NEG_Y, NEG_Z}
- `thickness_min` · float — Minimum for measuring thickness
- `thickness_max` · float — Maximum for measuring thickness
- `thickness_samples` · int — Number of samples to test per face
- `distort_min` · float — Minimum angle to display
- `distort_max` · float — Maximum angle to display
- `sharp_min` · float — Minimum angle to display
- `sharp_max` · float — Maximum angle to display

## MeshUVLoop

(Deprecated) Layer of UV coordinates in a Mesh data-block

- `uv` · float · [2]
- `pin_uv` · boolean

## MeshUVLoopLayer

- `data` · collection · read-only — Deprecated, use 'uv', 'vertex_select', 'edge_select' or 'pin' properties instead
- `name` · string — Name of UV map
- `active` · boolean — Set the map as active for display and editing
- `active_render` · boolean — Set the UV map as active for rendering
- `active_clone` · boolean — Set the map as active for cloning
- `uv` · collection · read-only — UV coordinates on face corners
- `pin` · collection · read-only — UV pinned state in the UV editor

## MeshVertex

Vertex in a Mesh data-block

- `co` · float · [3]
- `normal` · float · [3] · read-only — Vertex Normal
- `select` · boolean
- `hide` · boolean
- `groups` · collection · read-only — Weights for the vertex groups this vertex is member of
- `index` · int · read-only — Index of this vertex
- `undeformed_co` · float · [3] · read-only — For meshes with modifiers applied, the coordinate of the vertex with no deforming modifiers applied, as used for generated texture coordinates
