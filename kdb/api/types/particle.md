---
area: api-types
group: particle
version: 5.2.2
---

# Blender 5.2.2 RNA types: particle

## Particle

Particle in a particle system

- `location` · float · [3]
- `velocity` · float · [3]
- `angular_velocity` · float · [3]
- `rotation` · float · [4]
- `prev_location` · float · [3]
- `prev_velocity` · float · [3]
- `prev_angular_velocity` · float · [3]
- `prev_rotation` · float · [4]
- `hair_keys` · collection · read-only
- `particle_keys` · collection · read-only
- `birth_time` · float
- `lifetime` · float
- `die_time` · float
- `size` · float
- `is_exist` · boolean · read-only
- `is_visible` · boolean · read-only
- `alive_state` · enum · {DEAD, UNBORN, ALIVE, DYING}

## ParticleDupliWeight

Weight of a particle instance object in a collection

- `name` · string · read-only — Particle instance object name
- `count` · int — The number of times this object is repeated with respect to other objects

## ParticleEdit

Properties of particle editing mode

- `tool` · enum · {COMB, SMOOTH, ADD, LENGTH, PUFF, CUT, WEIGHT}
- `select_mode` · enum · {PATH, POINT, TIP} — Particle select and display mode
- `use_preserve_length` · boolean — Keep path lengths constant
- `use_preserve_root` · boolean — Keep root keys unmodified
- `use_emitter_deflect` · boolean — Keep paths from intersecting the emitter
- `emitter_distance` · float — Distance to keep particles away from the emitter
- `use_fade_time` · boolean — Fade paths and keys further away from current frame
- `use_auto_velocity` · boolean — Calculate point velocities automatically
- `show_particles` · boolean — Display actual particles
- `use_default_interpolate` · boolean — Interpolate new particles from the existing ones
- `default_key_count` · int — How many keys to make new particles with
- `brush` · pointer · -> ParticleBrush · read-only
- `display_step` · int — How many steps to display the path with
- `fade_frames` · int — How many frames to fade
- `type` · enum · {PARTICLES, SOFT_BODY, CLOTH}
- `is_editable` · boolean · read-only — A valid edit mode exists
- `is_hair` · boolean · read-only — Editing hair
- `object` · pointer · -> Object · read-only — The edited object
- `shape_object` · pointer · -> Object — Outer shape to use for tools

## ParticleHairKey

Particle key for hair particle system

- `time` · float — Relative time of key over hair length
- `weight` · float — Weight for cloth simulation
- `co` · float · [3] — Location of the hair key in object space
- `co_local` · float · [3] — Location of the hair key in its local coordinate system, relative to the emitting face

## ParticleKey

Key location for a particle over time

- `location` · float · [3] — Key location
- `velocity` · float · [3] — Key velocity
- `rotation` · float · [4] — Key rotation quaternion
- `angular_velocity` · float · [3] — Key angular velocity
- `time` · float — Time of key over the simulation

## ParticleSettings

Particle settings, reusable by multiple particle systems

- `texture_slots` · collection · [] of ParticleSettingsTextureSlots · read-only — Texture slots defining the mapping and influence of textures
- `active_texture` · pointer · -> Texture — Active texture slot being displayed
- `active_texture_index` · int — Index of active texture slot
- `is_fluid` · boolean · read-only — Particles were created by a fluid simulation
- `use_react_start_end` · boolean — Give birth to unreacted particles eventually
- `use_react_multiple` · boolean — React multiple times
- `use_regrow_hair` · boolean — Regrow hair for each frame
- `show_unborn` · boolean — Show particles before they are emitted
- `use_dead` · boolean — Show particles after they have died
- `use_emit_random` · boolean — Emit in random order of elements
- `use_even_distribution` · boolean — Use even distribution from faces based on face areas or edge lengths
- `use_die_on_collision` · boolean — Particles die when they collide with a deflector object
- `use_size_deflect` · boolean — Use particle's size in deflection
- `use_rotations` · boolean — Calculate particle rotations
- `use_dynamic_rotation` · boolean — Particle rotations are affected by collisions and effectors
- `use_multiply_size_mass` · boolean — Multiply mass by particle size
- `use_advanced_hair` · boolean — Use full physics calculations for growing hair
- `lock_boids_to_surface` · boolean — Constrain boids to a surface
- `use_hair_bspline` · boolean — Interpolate hair using B-Splines
- `invert_grid` · boolean — Invert what is considered object and what is not
- `hexagonal_grid` · boolean — Create the grid in a hexagonal pattern
- `apply_effector_to_children` · boolean — Apply effectors to children
- `create_long_hair_children` · boolean — Calculate children that suit long hair well
- `apply_guide_to_children` · boolean
- `use_self_effect` · boolean — Particle effectors affect themselves
- `type` · enum · {EMITTER, HAIR} — Particle type
- `emit_from` · enum · {VERT, FACE, VOLUME} — Where to emit particles from
- `distribution` · enum · {JIT, RAND, GRID} — How to distribute particles on selected element
- `physics_type` · enum · {NO, NEWTON, KEYED, BOIDS, FLUID} — Particle physics type
- `rotation_mode` · enum · {NONE, NOR, NOR_TAN, VEL, GLOB_X, GLOB_Y, GLOB_Z, OB_X, OB_Y, OB_Z} — Particle orientation axis (does not affect Explode modifier's results)
- `angular_velocity_mode` · enum · {NONE, VELOCITY, HORIZONTAL, VERTICAL, GLOBAL_X, GLOBAL_Y, GLOBAL_Z, RAND} — What axis is used to change particle rotation with time
- `react_event` · enum · {DEATH, COLLIDE, NEAR} — The event of target particles to react on
- `show_guide_hairs` · boolean — Show guide hairs
- `show_hair_grid` · boolean — Show hair simulation grid
- `show_velocity` · boolean — Show particle velocity
- `show_size` · boolean — Show particle size
- `show_health` · boolean — Display boid health
- `use_absolute_path_time` · boolean — Path timing is in absolute frames
- `use_parent_particles` · boolean — Render parent particles
- `show_number` · boolean — Show particle number
- `use_collection_pick_random` · boolean — Pick objects from collection randomly
- `use_collection_count` · boolean — Use object multiple times in the same collection
- `use_global_instance` · boolean — Use object's global coordinates for duplication
- `use_rotation_instance` · boolean — Use object's rotation for duplication (global x-axis is aligned particle rotation axis)
- `use_scale_instance` · boolean — Use object's scale for duplication
- `use_render_adaptive` · boolean — Display steps of the particle path
- `use_velocity_length` · boolean — Multiply line length by particle speed
- `use_whole_collection` · boolean — Use whole collection at once
- `use_strand_primitive` · boolean — Use the strand primitive for rendering
- `display_method` · enum · {NONE, RENDER, DOT, CIRC, CROSS, AXIS} — How particles are displayed in viewport
- `render_type` · enum · {NONE, HALO, LINE, PATH, OBJECT, COLLECTION} — How particles are rendered
- `display_color` · enum · {NONE, MATERIAL, VELOCITY, ACCELERATION} — Display additional particle data as a color
- `display_size` · float — Size of particles on viewport
- `child_type` · enum · {NONE, SIMPLE, INTERPOLATED} — Create child particles
- `display_step` · int — How many steps paths are displayed with (power of 2)
- `render_step` · int — How many steps paths are rendered with (power of 2)
- `hair_step` · int — Number of hair segments
- `bending_random` · float — Random stiffness of hairs
- `keys_step` · int
- `adaptive_angle` · int — How many degrees path has to curve to make another render segment
- `adaptive_pixel` · int — How many pixels path has to cover to make another render segment
- `display_percentage` · int — Percentage of particles to display in 3D view
- `material` · int — Index of material slot used for rendering particles
- `material_slot` · enum · {DEFAULT} — Material slot used for rendering particles
- `integrator` · enum · {EULER, VERLET, MIDPOINT, RK4} — Algorithm used to calculate physics, from the fastest to the most stable and accurate: Midpoint, Euler, Verlet, RK4
- `kink` · enum · {NO, CURL, RADIAL, WAVE, BRAID, SPIRAL} — Type of periodic offset on the path
- `kink_axis` · enum · {X, Y, Z} — Which axis to use for offset
- `color_maximum` · float — Maximum length of the particle color vector
- `frame_start` · float — Frame number to start emitting particles
- `frame_end` · float — Frame number to stop emitting particles
- `lifetime` · float — Life span of the particles
- `lifetime_random` · float — Give the particle life a random variation
- `time_tweak` · float — A multiplier for physics timestep (1.0 means one frame = 1/25 seconds)
- `timestep` · float — The simulation timestep per frame (seconds per frame)
- `use_adaptive_subframes` · boolean — Automatically set the number of subframes
- `subframes` · int — Subframes to simulate for improved stability and finer granularity simulations (dt = timestep / (subframes + 1))
- `courant_target` · float — The relative distance a particle can move before requiring more subframes (target Courant number); 0.01 to 0.3 is the recommended range
- `jitter_factor` · float — Amount of jitter applied to the sampling
- `effect_hair` · float — Hair stiffness for effectors
- `count` · int — Total number of particles
- `userjit` · int — Emission locations per face (0 = automatic)
- `grid_resolution` · int — The resolution of the particle grid
- `grid_random` · float — Add random offset to the grid locations
- `effector_amount` · int — How many particles are effectors (0 is all particles)
- `normal_factor` · float — Let the surface normal give the particle a starting velocity
- `object_factor` · float — Let the object give the particle a starting velocity
- `factor_random` · float — Give the starting velocity a random variation
- `particle_factor` · float — Let the target particle give the particle a starting velocity
- `tangent_factor` · float — Let the surface tangent give the particle a starting velocity
- `tangent_phase` · float — Rotate the surface tangent
- `reactor_factor` · float — Let the vector away from the target particle's location give the particle a starting velocity
- `object_align_factor` · float · [3] — Let the emitter object orientation give the particle a starting velocity
- `angular_velocity_factor` · float — Angular velocity amount (in radians per second)
- `phase_factor` · float — Rotation around the chosen orientation axis
- `rotation_factor_random` · float — Randomize particle orientation
- `phase_factor_random` · float — Randomize rotation around the chosen orientation axis
- `hair_length` · float — Length of the hair
- `mass` · float — Mass of the particles
- `particle_size` · float — The size of the particles
- `size_random` · float — Give the particle size a random variation
- `collision_collection` · pointer · -> Collection — Limit colliders to this collection
- `drag_factor` · float — Amount of air drag
- `brownian_factor` · float — Amount of random, erratic particle movement
- `damping` · float — Amount of damping
- `length_random` · float — Give path length a random variation
- `child_percent` · int — Number of children per parent
- `rendered_child_count` · int — Number of children per parent for rendering
- `virtual_parents` · float — Relative amount of virtual parents
- `child_size` · float — A multiplier for the child particle size
- `child_size_random` · float — Random variation to the size of the child particles
- `child_radius` · float — Radius of children around parent
- `child_roundness` · float — Roundness of children around parent
- `clump_factor` · float — Amount of clumping
- `clump_shape` · float — Shape of clumping
- `use_clump_curve` · boolean — Use a curve to define clump tapering
- `clump_curve` · pointer · -> CurveMapping · read-only — Curve defining clump tapering
- `use_clump_noise` · boolean — Create random clumps around the parent
- `clump_noise_size` · float — Size of clump noise
- `kink_amplitude` · float — The amplitude of the offset
- `kink_amplitude_clump` · float — How much clump affects kink amplitude
- `kink_amplitude_random` · float — Random variation of the amplitude
- `kink_frequency` · float — The frequency of the offset (1/total length)
- `kink_shape` · float — Adjust the offset to the beginning/end
- `kink_flat` · float — How flat the hairs are
- `kink_extra_steps` · int — Extra steps for resolution of special kink features
- `kink_axis_random` · float — Random variation of the orientation
- `roughness_1` · float — Amount of location dependent roughness
- `roughness_1_size` · float — Size of location dependent roughness
- `roughness_2` · float — Amount of random roughness
- `roughness_2_size` · float — Size of random roughness
- `roughness_2_threshold` · float — Amount of particles left untouched by random roughness
- `roughness_endpoint` · float — Amount of endpoint roughness
- `roughness_end_shape` · float — Shape of endpoint roughness
- `use_roughness_curve` · boolean — Use a curve to define roughness
- `roughness_curve` · pointer · -> CurveMapping · read-only — Curve defining roughness
- `child_length` · float — Length of child paths
- `child_length_threshold` · float — Amount of particles left untouched by child path length
- `child_parting_factor` · float — Create parting in the children based on parent strands
- `child_parting_min` · float — Minimum root to tip angle (tip distance/root distance for long hair)
- `child_parting_max` · float — Maximum root to tip angle (tip distance/root distance for long hair)
- `branch_threshold` · float — Threshold of branching
- `line_length_tail` · float — Length of the line's tail
- `line_length_head` · float — Length of the line's head
- `path_start` · float — Starting time of path
- `path_end` · float — End time of path
- `trail_count` · int — Number of trail particles
- `keyed_loops` · int — Number of times the keys are looped
- `use_modifier_stack` · boolean — Emit particles from mesh with modifiers applied (must use same subdivision surface level for viewport and render for correct results)
- `instance_collection` · pointer · -> Collection — Show objects in this collection in place of particles
- `instance_weights` · collection · read-only — Weights for all of the objects in the instance collection
- `active_instanceweight` · pointer · -> ParticleDupliWeight · read-only
- `active_instanceweight_index` · int
- `instance_object` · pointer · -> Object — Show this object in place of particles
- `boids` · pointer · -> BoidSettings · read-only
- `fluid` · pointer · -> SPHFluidSettings · read-only
- `effector_weights` · pointer · -> EffectorWeights · read-only
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `force_field_1` · pointer · -> FieldSettings · read-only
- `force_field_2` · pointer · -> FieldSettings · read-only
- `twist` · float — Number of turns around parent along the strand
- `use_twist_curve` · boolean — Use a curve to define twist
- `twist_curve` · pointer · -> CurveMapping · read-only — Curve defining twist
- `use_close_tip` · boolean — Set tip radius to zero
- `shape` · float — Strand shape parameter
- `root_radius` · float — Strand diameter width at the root
- `tip_radius` · float — Strand diameter width at the tip
- `radius_scale` · float — Multiplier of diameter properties

## ParticleSettingsTextureSlot

Texture slot for textures in a Particle Settings data-block

- `texture_coords` · enum · {GLOBAL, OBJECT, UV, ORCO, STRAND} — Texture coordinates used to map the texture onto the background
- `object` · pointer · -> Object — Object to use for mapping with Object texture coordinates
- `uv_layer` · string — UV map to use for mapping with UV texture coordinates
- `mapping_x` · enum · {NONE, X, Y, Z}
- `mapping_y` · enum · {NONE, X, Y, Z}
- `mapping_z` · enum · {NONE, X, Y, Z}
- `mapping` · enum · {FLAT, CUBE, TUBE, SPHERE}
- `use_map_time` · boolean — Affect the emission time of the particles
- `use_map_life` · boolean — Affect the life time of the particles
- `use_map_density` · boolean — Affect the density of the particles
- `use_map_size` · boolean — Affect the particle size
- `use_map_velocity` · boolean — Affect the particle initial velocity
- `use_map_field` · boolean — Affect the particle force fields
- `use_map_gravity` · boolean — Affect the particle gravity
- `use_map_damp` · boolean — Affect the particle velocity damping
- `use_map_clump` · boolean — Affect the child clumping
- `use_map_kink_amp` · boolean — Affect the child kink amplitude
- `use_map_kink_freq` · boolean — Affect the child kink frequency
- `use_map_rough` · boolean — Affect the child rough
- `use_map_length` · boolean — Affect the child hair length
- `use_map_twist` · boolean — Affect the child twist
- `time_factor` · float — Amount texture affects particle emission time
- `life_factor` · float — Amount texture affects particle life time
- `density_factor` · float — Amount texture affects particle density
- `size_factor` · float — Amount texture affects physical particle size
- `velocity_factor` · float — Amount texture affects particle initial velocity
- `field_factor` · float — Amount texture affects particle force fields
- `gravity_factor` · float — Amount texture affects particle gravity
- `damp_factor` · float — Amount texture affects particle damping
- `length_factor` · float — Amount texture affects child hair length
- `clump_factor` · float — Amount texture affects child clump
- `kink_amp_factor` · float — Amount texture affects child kink amplitude
- `kink_freq_factor` · float — Amount texture affects child kink frequency
- `rough_factor` · float — Amount texture affects child roughness
- `twist_factor` · float — Amount texture affects child twist

## ParticleSystem

Particle system in an object

- `name` · string — Particle system name
- `settings` · pointer · -> ParticleSettings — Particle system settings
- `particles` · collection · read-only — Particles generated by the particle system
- `child_particles` · collection · read-only — Child particles generated by the particle system
- `seed` · int — Offset in the random number table, to get a different randomized result
- `child_seed` · int — Offset in the random number table for child particles, to get a different randomized result
- `is_global_hair` · boolean · read-only — Hair keys are in global coordinate space
- `use_hair_dynamics` · boolean — Enable hair dynamics using cloth simulation
- `cloth` · pointer · -> ClothModifier · read-only — Cloth dynamics for hair
- `reactor_target_object` · pointer · -> Object — For reactor systems, the object that has the target particle system (empty if same object)
- `reactor_target_particle_system` · int — For reactor systems, index of particle system on the target object
- `use_keyed_timing` · boolean — Use key times
- `targets` · collection · read-only — Target particle systems
- `active_particle_target` · pointer · -> ParticleTarget · read-only
- `active_particle_target_index` · int
- `vertex_group_density` · string — Vertex group to control density
- `invert_vertex_group_density` · boolean — Negate the effect of the density vertex group
- `vertex_group_velocity` · string — Vertex group to control velocity
- `invert_vertex_group_velocity` · boolean — Negate the effect of the velocity vertex group
- `vertex_group_length` · string — Vertex group to control length
- `invert_vertex_group_length` · boolean — Negate the effect of the length vertex group
- `vertex_group_clump` · string — Vertex group to control clump
- `invert_vertex_group_clump` · boolean — Negate the effect of the clump vertex group
- `vertex_group_kink` · string — Vertex group to control kink
- `invert_vertex_group_kink` · boolean — Negate the effect of the kink vertex group
- `vertex_group_roughness_1` · string — Vertex group to control roughness 1
- `invert_vertex_group_roughness_1` · boolean — Negate the effect of the roughness 1 vertex group
- `vertex_group_roughness_2` · string — Vertex group to control roughness 2
- `invert_vertex_group_roughness_2` · boolean — Negate the effect of the roughness 2 vertex group
- `vertex_group_roughness_end` · string — Vertex group to control roughness end
- `invert_vertex_group_roughness_end` · boolean — Negate the effect of the roughness end vertex group
- `vertex_group_size` · string — Vertex group to control size
- `invert_vertex_group_size` · boolean — Negate the effect of the size vertex group
- `vertex_group_tangent` · string — Vertex group to control tangent
- `invert_vertex_group_tangent` · boolean — Negate the effect of the tangent vertex group
- `vertex_group_rotation` · string — Vertex group to control rotation
- `invert_vertex_group_rotation` · boolean — Negate the effect of the rotation vertex group
- `vertex_group_field` · string — Vertex group to control field
- `invert_vertex_group_field` · boolean — Negate the effect of the field vertex group
- `vertex_group_twist` · string — Vertex group to control twist
- `invert_vertex_group_twist` · boolean — Negate the effect of the twist vertex group
- `point_cache` · pointer · -> PointCache · read-only
- `has_multiple_caches` · boolean · read-only — Particle system has multiple point caches
- `parent` · pointer · -> Object — Use this object's coordinate system instead of global coordinate system
- `is_editable` · boolean · read-only — Particle system can be edited in particle mode
- `is_edited` · boolean · read-only — Particle system has been edited in particle mode
- `dt_frac` · float · read-only — The current simulation time step size, as a fraction of a frame

## ParticleSystems

Collection of particle systems

- `active` · pointer · -> ParticleSystem · read-only — Active particle system being displayed
- `active_index` · int — Index of active particle system slot

## ParticleTarget

Target particle system

- `name` · string · read-only — Particle target name
- `object` · pointer · -> Object — The object that has the target particle system (empty if same object)
- `system` · int — The index of particle system on the target object
- `time` · float
- `duration` · float
- `is_valid` · boolean — Keyed particles target is valid
- `alliance` · enum · {FRIEND, NEUTRAL, ENEMY}
