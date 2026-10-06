---
area: api-types
group: cloth
version: 5.2.2
---

# Blender 5.2.2 RNA types: cloth

## ClothCollisionSettings

Cloth simulation settings for self collision and collision with other objects

- `use_collision` · boolean — Enable collisions with other objects
- `distance_min` · float — Minimum distance between collision objects before collision response takes effect
- `friction` · float — Friction force if a collision happened (higher = less movement)
- `damping` · float — Amount of velocity lost on collision
- `collision_quality` · int — How many collision iterations should be done (higher is better quality but slower)
- `impulse_clamp` · float — Clamp collision impulses to avoid instability (0.0 to disable clamping)
- `use_self_collision` · boolean — Enable self collisions
- `self_distance_min` · float — Minimum distance between cloth faces before collision response takes effect
- `self_friction` · float — Friction with self contact
- `collection` · pointer · -> Collection — Limit colliders to this Collection
- `vertex_group_self_collisions` · string — Triangles with all vertices in this group are not used during self collisions
- `vertex_group_object_collisions` · string — Triangles with all vertices in this group are not used during object collisions
- `self_impulse_clamp` · float — Clamp collision impulses to avoid instability (0.0 to disable clamping)

## ClothSettings

Cloth simulation settings for an object

- `goal_min` · float — Goal minimum, vertex group weights are scaled to match this range
- `goal_max` · float — Goal maximum, vertex group weights are scaled to match this range
- `goal_default` · float — Default Goal (vertex target position) value, when no Vertex Group used
- `goal_spring` · float — Goal (vertex target position) spring stiffness
- `goal_friction` · float — Goal (vertex target position) friction
- `internal_friction` · float
- `collider_friction` · float
- `density_target` · float — Maximum density of hair
- `density_strength` · float — Influence of target density on the simulation
- `mass` · float — The mass of each vertex on the cloth material
- `vertex_group_mass` · string — Vertex Group for pinning of vertices
- `gravity` · float · [3] — Gravity or external force vector
- `air_damping` · float — Air has normally some thickness which slows falling things down
- `pin_stiffness` · float — Pin (vertex target position) spring stiffness
- `quality` · int — Quality of the simulation in steps per frame (higher is better quality but slower)
- `time_scale` · float — Cloth speed is multiplied by this value
- `vertex_group_shrink` · string — Vertex Group for shrinking cloth
- `shrink_min` · float — Factor by which to shrink cloth
- `shrink_max` · float — Max amount to shrink cloth by
- `voxel_cell_size` · float — Size of the voxel grid cells for interaction effects
- `tension_damping` · float — Amount of damping in stretching behavior
- `compression_damping` · float — Amount of damping in compression behavior
- `shear_damping` · float — Amount of damping in shearing behavior
- `tension_stiffness` · float — How much the material resists stretching
- `tension_stiffness_max` · float — Maximum tension stiffness value
- `compression_stiffness` · float — How much the material resists compression
- `compression_stiffness_max` · float — Maximum compression stiffness value
- `shear_stiffness` · float — How much the material resists shearing
- `shear_stiffness_max` · float — Maximum shear scaling value
- `sewing_force_max` · float — Maximum sewing force
- `vertex_group_structural_stiffness` · string — Vertex group for fine control over structural stiffness
- `vertex_group_shear_stiffness` · string — Vertex group for fine control over shear stiffness
- `bending_stiffness` · float — How much the material resists bending
- `bending_stiffness_max` · float — Maximum bending stiffness value
- `bending_damping` · float — Amount of damping in bending behavior
- `use_sewing_springs` · boolean — Pulls loose edges together
- `vertex_group_bending` · string — Vertex group for fine control over bending stiffness
- `effector_weights` · pointer · -> EffectorWeights · read-only
- `rest_shape_key` · pointer · -> ShapeKey — Shape key to use the rest spring lengths from
- `use_dynamic_mesh` · boolean — Make simulation respect deformations in the base mesh
- `bending_model` · enum · {ANGULAR, LINEAR} — Physical model for simulating bending forces
- `use_internal_springs` · boolean — Simulate an internal volume structure by creating springs connecting the opposite sides of the mesh
- `internal_spring_normal_check` · boolean — Require the points the internal springs connect to have opposite normal directions
- `internal_spring_max_length` · float — The maximum length an internal spring can have during creation. If the distance between internal points is greater than this, no internal spring will be created between these points. A length of zero means that there is no length limit.
- `internal_spring_max_diversion` · float — How much the rays used to connect the internal points can diverge from the vertex normal
- `internal_tension_stiffness` · float — How much the material resists stretching
- `internal_tension_stiffness_max` · float — Maximum tension stiffness value
- `internal_compression_stiffness` · float — How much the material resists compression
- `internal_compression_stiffness_max` · float — Maximum compression stiffness value
- `vertex_group_intern` · string — Vertex group for fine control over the internal spring stiffness
- `use_pressure` · boolean — Simulate pressure inside a closed cloth mesh
- `use_pressure_volume` · boolean — Use the Target Volume parameter as the initial volume, instead of calculating it from the mesh itself
- `uniform_pressure_force` · float — The uniform pressure that is constantly applied to the mesh, in units of Pressure Scale. Can be negative.
- `target_volume` · float — The mesh volume where the inner/outer pressure will be the same. If set to zero the change in volume will not affect pressure.
- `pressure_factor` · float — Ambient pressure (kPa) that balances out between the inside and outside of the object when it has the target volume
- `fluid_density` · float — Density (kg/l) of the fluid contained inside the object, used to create a hydrostatic pressure gradient simulating the weight of the internal fluid, or buoyancy from the surrounding fluid if negative
- `vertex_group_pressure` · string — Vertex Group for where to apply pressure. Zero weight means no pressure while a weight of one means full pressure. Faces with a vertex that has zero weight will be excluded from the volume calculation.

## ClothSolverResult

Result of cloth solver iteration

- `status` · enum · {SUCCESS, NUMERICAL_ISSUE, NO_CONVERGENCE, INVALID_INPUT} · read-only — Status of the solver iteration
- `max_error` · float · read-only — Maximum error during substeps
- `min_error` · float · read-only — Minimum error during substeps
- `avg_error` · float · read-only — Average error during substeps
- `max_iterations` · int · read-only — Maximum iterations during substeps
- `min_iterations` · int · read-only — Minimum iterations during substeps
- `avg_iterations` · float · read-only — Average iterations during substeps
