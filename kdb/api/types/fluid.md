---
area: api-types
group: fluid
version: 5.2.2
---

# Blender 5.2.2 RNA types: fluid

## FluidDomainSettings

Fluid domain settings

- `effector_weights` · pointer · -> EffectorWeights · read-only
- `effector_group` · pointer · -> Collection — Limit effectors to this collection
- `fluid_group` · pointer · -> Collection — Limit fluid objects to this collection
- `force_collection` · pointer · -> Collection — Limit forces to this collection
- `density_grid` · float · [32] · read-only — Smoke density grid
- `velocity_grid` · float · [32] · read-only — Smoke velocity grid
- `flame_grid` · float · [32] · read-only — Smoke flame grid
- `color_grid` · float · [32] · read-only — Smoke color grid
- `heat_grid` · float · [32] · read-only — Smoke heat grid
- `temperature_grid` · float · [32] · read-only — Smoke temperature grid, range 0 to 1 represents 0 to 1000K
- `start_point` · float · [3] · read-only — Start point
- `cell_size` · float · [3] · read-only — Cell Size
- `domain_resolution` · int · [3] · read-only — Smoke Grid Resolution
- `additional_res` · int — Maximum number of additional cells
- `adapt_margin` · int — Margin added around fluid to minimize boundary interference
- `adapt_threshold` · float — Minimum amount of fluid grid values (smoke density, fuel and heat) a cell can contain, before it is considered empty
- `use_adaptive_domain` · boolean — Adapt simulation resolution and size to fluid
- `resolution_max` · int — Resolution used for the fluid domain. Value corresponds to the longest domain side (resolution for other domain sides is calculated automatically).
- `use_collision_border_front` · boolean — Enable collisions with front domain border
- `use_collision_border_back` · boolean — Enable collisions with back domain border
- `use_collision_border_right` · boolean — Enable collisions with right domain border
- `use_collision_border_left` · boolean — Enable collisions with left domain border
- `use_collision_border_top` · boolean — Enable collisions with top domain border
- `use_collision_border_bottom` · boolean — Enable collisions with bottom domain border
- `gravity` · float · [3] — Gravity in X, Y and Z direction
- `domain_type` · enum · {GAS, LIQUID} — Change domain type of the simulation
- `delete_in_obstacle` · boolean — Delete fluid inside obstacles
- `alpha` · float — Buoyant force based on smoke density (higher value results in faster rising smoke)
- `beta` · float — Buoyant force based on smoke heat (higher value results in faster rising smoke)
- `dissolve_speed` · int — Determine how quickly the smoke dissolves (lower value makes smoke disappear faster)
- `vorticity` · float — Amount of turbulence and rotation in smoke
- `highres_sampling` · enum · {FULLSAMPLE, LINEAR, NEAREST} — Method for sampling the high resolution flow
- `use_dissolve_smoke` · boolean — Let smoke disappear over time
- `use_dissolve_smoke_log` · boolean — Dissolve smoke in a logarithmic fashion. Dissolves quickly at first, but lingers longer.
- `burning_rate` · float — Speed of the burning reaction (higher value results in smaller flames)
- `flame_smoke` · float — Amount of smoke created by burning fuel
- `flame_vorticity` · float — Additional vorticity for the flames
- `flame_ignition` · float — Minimum temperature of the flames (higher value results in faster rising flames)
- `flame_max_temp` · float — Maximum temperature of the flames (higher value results in faster rising flames)
- `flame_smoke_color` · float · [3] — Color of smoke emitted from burning fuel
- `noise_strength` · float — Strength of noise
- `noise_pos_scale` · float — Scale of noise (higher value results in larger vortices)
- `noise_time_anim` · float — Animation time of noise
- `noise_scale` · int — The noise simulation is scaled up by this factor (compared to the base resolution of the domain)
- `use_noise` · boolean — Enable fluid noise (using amplification)
- `simulation_method` · enum · {FLIP, APIC} — Change the underlying simulation method
- `flip_ratio` · float — PIC/FLIP Ratio. A value of 1.0 will result in a completely FLIP based simulation. Use a lower value for simulations which should produce smaller splashes.
- `particle_randomness` · float — Randomness factor for particle sampling
- `particle_number` · int — Particle number factor (higher value results in more particles)
- `particle_min` · int — Minimum number of particles per cell (ensures that each cell has at least this amount of particles)
- `particle_max` · int — Maximum number of particles per cell (ensures that each cell has at most this amount of particles)
- `particle_radius` · float — Particle radius factor. Increase this value if the simulation appears to leak volume, decrease it if the simulation seems to gain volume.
- `particle_band_width` · float — Particle (narrow) band width (higher value results in thicker band and more particles)
- `use_flip_particles` · boolean — Create liquid particle system
- `use_fractions` · boolean — Fractional obstacles improve and smoothen the fluid-obstacle boundary
- `fractions_threshold` · float — Determines how much fluid is allowed in an obstacle cell (higher values will tag a boundary cell as an obstacle easier and reduce the boundary smoothening effect)
- `fractions_distance` · float — Determines how far apart fluid and obstacle are (higher values will result in fluid being further away from obstacles, smaller values will let fluid move towards the inside of obstacles)
- `sys_particle_maximum` · int — Maximum number of fluid particles that are allowed in this simulation
- `use_viscosity` · boolean — Simulate fluids with high viscosity using a special solver
- `viscosity_value` · float — Viscosity of liquid (higher values result in more viscous fluids, a value of 0 will still apply some viscosity)
- `use_diffusion` · boolean — Enable fluid diffusion settings (e.g. viscosity, surface tension)
- `surface_tension` · float — Surface tension of liquid (higher value results in greater hydrophobic behavior)
- `viscosity_base` · float — Viscosity setting: value that is multiplied by 10 to the power of (exponent*-1)
- `viscosity_exponent` · int — Negative exponent for the viscosity value (to simplify entering small values e.g. 5*10^-6)
- `mesh_concave_upper` · float — Upper mesh concavity bound (high values tend to smoothen and fill out concave regions)
- `mesh_concave_lower` · float — Lower mesh concavity bound (high values tend to smoothen and fill out concave regions)
- `mesh_smoothen_pos` · int — Positive mesh smoothening
- `mesh_smoothen_neg` · int — Negative mesh smoothening
- `mesh_scale` · int — The mesh simulation is scaled up by this factor (compared to the base resolution of the domain). For best meshing, it is recommended to adjust the mesh particle radius alongside this value.
- `mesh_generator` · enum · {IMPROVED, UNION} — Which particle level set generator to use
- `use_mesh` · boolean — Enable fluid mesh (using amplification)
- `use_speed_vectors` · boolean — Caches velocities of mesh vertices. These will be used (automatically) when rendering with motion blur enabled.
- `mesh_particle_radius` · float — Particle radius factor (higher value results in larger (meshed) particles). Needs to be adjusted after changing the mesh scale.
- `sndparticle_potential_min_wavecrest` · float — Lower clamping threshold for marking fluid cells as wave crests (lower value results in more marked cells)
- `sndparticle_potential_max_wavecrest` · float — Upper clamping threshold for marking fluid cells as wave crests (higher value results in less marked cells)
- `sndparticle_potential_min_trappedair` · float — Lower clamping threshold for marking fluid cells where air is trapped (lower value results in more marked cells)
- `sndparticle_potential_max_trappedair` · float — Upper clamping threshold for marking fluid cells where air is trapped (higher value results in less marked cells)
- `sndparticle_potential_min_energy` · float — Lower clamping threshold that indicates the fluid speed where cells start to emit particles (lower values result in generally more particles)
- `sndparticle_potential_max_energy` · float — Upper clamping threshold that indicates the fluid speed where cells no longer emit more particles (higher value results in generally less particles)
- `sndparticle_sampling_wavecrest` · int — Maximum number of particles generated per wave crest cell per frame
- `sndparticle_sampling_trappedair` · int — Maximum number of particles generated per trapped air cell per frame
- `sndparticle_bubble_buoyancy` · float — Amount of buoyancy force that rises bubbles (high value results in bubble movement mainly upwards)
- `sndparticle_bubble_drag` · float — Amount of drag force that moves bubbles along with the fluid (high value results in bubble movement mainly along with the fluid)
- `sndparticle_life_min` · float — Lowest possible particle lifetime
- `sndparticle_life_max` · float — Highest possible particle lifetime
- `sndparticle_boundary` · enum · {DELETE, PUSHOUT} — How particles that left the domain are treated
- `sndparticle_combined_export` · enum · {OFF, SPRAY_FOAM, SPRAY_BUBBLES, FOAM_BUBBLES, SPRAY_FOAM_BUBBLES} — Determines which particle systems are created from secondary particles
- `sndparticle_potential_radius` · int — Radius to compute potential for each cell (higher values are slower but create smoother potential grids)
- `sndparticle_update_radius` · int — Radius to compute position update for each particle (higher values are slower but particles move less chaotic)
- `particle_scale` · int — The particle simulation is scaled up by this factor (compared to the base resolution of the domain)
- `use_spray_particles` · boolean — Create spray particle system
- `use_bubble_particles` · boolean — Create bubble particle system
- `use_foam_particles` · boolean — Create foam particle system
- `use_tracer_particles` · boolean — Create tracer particle system
- `guide_alpha` · float — Guiding weight (higher value results in greater lag)
- `guide_beta` · int — Guiding size (higher value results in larger vortices)
- `guide_vel_factor` · float — Guiding velocity factor (higher value results in greater guiding velocities)
- `guide_source` · enum · {DOMAIN, EFFECTOR} — Choose where to get guiding velocities from
- `guide_parent` · pointer · -> Object — Use velocities from this object for the guiding effect (object needs to have fluid modifier and be of type domain))
- `use_guide` · boolean — Enable fluid guiding
- `cache_frame_start` · int — Frame on which the simulation starts (first frame baked)
- `cache_frame_end` · int — Frame on which the simulation stops (last frame baked)
- `cache_frame_offset` · int — Frame offset that is used when loading the simulation from the cache. It is not considered when baking the simulation, only when loading it.
- `cache_frame_pause_data` · int
- `cache_frame_pause_noise` · int
- `cache_frame_pause_mesh` · int
- `cache_frame_pause_particles` · int
- `cache_frame_pause_guide` · int
- `cache_mesh_format` · enum · {UNI, OPENVDB, RAW} — Select the file format to be used for caching surface data
- `cache_data_format` · enum · {UNI, OPENVDB, RAW} — Select the file format to be used for caching volumetric data
- `cache_particle_format` · enum · {UNI, OPENVDB, RAW} — Select the file format to be used for caching particle data
- `cache_noise_format` · enum · {UNI, OPENVDB, RAW} — Select the file format to be used for caching noise data
- `cache_type` · enum · {REPLAY, MODULAR, ALL} — Change the cache type of the simulation
- `cache_resumable` · boolean — Additional data will be saved so that the bake jobs can be resumed after pausing. Because more data will be written to disk it is recommended to avoid enabling this option when baking at high resolutions.
- `cache_directory` · string — Directory that contains fluid cache files
- `is_cache_baking_data` · boolean
- `has_cache_baked_data` · boolean
- `is_cache_baking_noise` · boolean
- `has_cache_baked_noise` · boolean
- `is_cache_baking_mesh` · boolean
- `has_cache_baked_mesh` · boolean
- `is_cache_baking_particles` · boolean
- `has_cache_baked_particles` · boolean
- `is_cache_baking_guide` · boolean
- `has_cache_baked_guide` · boolean
- `is_cache_baking_any` · boolean
- `has_cache_baked_any` · boolean
- `export_manta_script` · boolean — Generate and export Mantaflow script from current domain settings during bake. This is only needed if you plan to analyze the cache (e.g. view grids, velocity vectors, particles) in Mantaflow directly (outside of Blender) after baking the simulation.
- `openvdb_cache_compress_type` · enum · {ZIP, BLOSC, NONE} — Compression method to be used
- `openvdb_data_depth` · enum · {NONE} — Bit depth for fluid particles and grids (lower bit values reduce file size)
- `time_scale` · float — Adjust simulation speed
- `use_adaptive_timesteps` · boolean — Automatically decide when to perform multiple simulation steps per frame
- `cfl_condition` · float — Maximal velocity per cell (greater CFL numbers will minimize the number of simulation steps and the computation time.)
- `timesteps_min` · int — Minimum number of simulation steps to perform for one frame
- `timesteps_max` · int — Maximum number of simulation steps to perform for one frame
- `use_slice` · boolean — Perform a single slice of the domain object
- `slice_axis` · enum · {AUTO, X, Y, Z}
- `slice_per_voxel` · float — How many slices per voxel should be generated
- `slice_depth` · float — Position of the slice
- `display_thickness` · float — Thickness of smoke display in the viewport
- `display_interpolation` · enum · {LINEAR, CUBIC, CLOSEST} — Interpolation method to use for smoke/fire volumes in solid mode
- `show_gridlines` · boolean — Show gridlines
- `show_velocity` · boolean — Visualize vector fields
- `vector_display_type` · enum · {NEEDLE, STREAMLINE, MAC}
- `vector_field` · enum · {FLUID_VELOCITY, GUIDE_VELOCITY, FORCE} — Vector field to be represented by the display vectors
- `vector_scale_with_magnitude` · boolean — Scale vectors with their magnitudes
- `vector_show_mac_x` · boolean — Show X-component of MAC Grid
- `vector_show_mac_y` · boolean — Show Y-component of MAC Grid
- `vector_show_mac_z` · boolean — Show Z-component of MAC Grid
- `vector_scale` · float — Multiplier for scaling the vectors
- `use_color_ramp` · boolean — Render a simulation field while mapping its voxels values to the colors of a ramp or using a predefined color code
- `color_ramp_field` · enum · {NONE} — Simulation field to color map
- `color_ramp_field_scale` · float — Multiplier for scaling the selected field to color map
- `color_ramp` · pointer · -> ColorRamp · read-only
- `clipping` · float — Value under which voxels are considered empty space to optimize rendering
- `gridlines_color_field` · enum · {NONE, FLAGS, RANGE} — Simulation field to color map onto gridlines
- `gridlines_lower_bound` · float — Lower bound of the highlighting range
- `gridlines_upper_bound` · float — Upper bound of the highlighting range
- `gridlines_range_color` · float · [4] — Color used to highlight the range
- `gridlines_cell_filter` · enum · {NONE, FLUID, OBSTACLE, EMPTY, INFLOW, OUTFLOW} — Cell type to be highlighted
- `velocity_scale` · float — Factor to control the amount of motion blur

## FluidEffectorSettings

Smoke collision settings

- `effector_type` · enum · {COLLISION, GUIDE} — Change type of effector in the simulation
- `surface_distance` · float — Additional distance around mesh surface to consider as effector
- `use_plane_init` · boolean — Treat this object as a planar, unclosed mesh
- `velocity_factor` · float — Multiplier of obstacle velocity
- `guide_mode` · enum · {MAXIMUM, MINIMUM, OVERRIDE, AVERAGED} — How to create guiding velocities
- `use_effector` · boolean — Control when to apply the effector
- `subframes` · int — Number of additional samples to take between frames to improve quality of fast moving effector objects

## FluidFlowSettings

Fluid flow settings

- `density` · float
- `smoke_color` · float · [3] — Color of smoke
- `fuel_amount` · float
- `temperature` · float — Temperature difference to ambient temperature
- `particle_system` · pointer · -> ParticleSystem — Particle systems emitted from the object
- `flow_type` · enum · {SMOKE, BOTH, FIRE, LIQUID} — Change type of fluid in the simulation
- `flow_behavior` · enum · {INFLOW, OUTFLOW, GEOMETRY} — Change flow behavior in the simulation
- `flow_source` · enum · {NONE} — Change how fluid is emitted
- `use_absolute` · boolean — Only allow given density value in emitter area and will not add up
- `use_initial_velocity` · boolean — Fluid has some initial velocity when it is emitted
- `velocity_factor` · float — Multiplier of source velocity passed to fluid (source velocity is non-zero only if object is moving)
- `velocity_normal` · float — Amount of normal directional velocity
- `velocity_random` · float — Amount of random velocity
- `velocity_coord` · float · [3] — Additional initial velocity in X, Y and Z direction (added to source velocity)
- `volume_density` · float — Controls fluid emission from within the mesh (higher value results in greater emissions from inside the mesh)
- `surface_distance` · float — Height (in domain grid units) of fluid emission above the mesh surface. Higher values result in emission further away from the mesh surface. If this value and the emitter size are smaller than the domain grid unit, fluid will not be created
- `use_plane_init` · boolean — Treat this object as a planar and unclosed mesh. Fluid will only be emitted from the mesh surface and based on the surface emission value.
- `particle_size` · float — Particle size in simulation cells
- `use_particle_size` · boolean — Set particle size in simulation cells or use nearest cell
- `use_inflow` · boolean — Control when to apply fluid flow
- `subframes` · int — Number of additional samples to take between frames to improve quality of fast moving flows
- `density_vertex_group` · string — Name of vertex group which determines surface emission rate
- `use_texture` · boolean — Use a texture to control emission strength
- `texture_map_type` · enum · {AUTO, UV} — Texture mapping type
- `uv_layer` · string — UV map name
- `noise_texture` · pointer · -> Texture — Texture that controls emission strength
- `texture_size` · float — Size of texture mapping
- `texture_offset` · float — Z-offset of texture mapping

## SPHFluidSettings

Settings for particle fluids physics

- `solver` · enum · {DDR, CLASSICAL} — The code used to calculate internal forces on particles
- `spring_force` · float — Spring force
- `fluid_radius` · float — Fluid interaction radius
- `rest_length` · float — Spring rest length (factor of particle radius)
- `use_viscoelastic_springs` · boolean — Use viscoelastic springs instead of Hooke's springs
- `use_initial_rest_length` · boolean — Use the initial length as spring rest length instead of 2 * particle size
- `plasticity` · float — How much the spring rest length can change after the elastic limit is crossed
- `yield_ratio` · float — How much the spring has to be stretched/compressed in order to change its rest length
- `spring_frames` · int — Create springs for this number of frames since particles birth (0 is always)
- `linear_viscosity` · float — Linear viscosity
- `stiff_viscosity` · float — Creates viscosity for expanding fluid
- `stiffness` · float — How incompressible the fluid is (speed of sound)
- `repulsion` · float — How strongly the fluid tries to keep from clustering (factor of stiffness)
- `rest_density` · float — Fluid rest density
- `buoyancy` · float — Artificial buoyancy force in negative gravity direction based on pressure differences inside the fluid
- `use_factor_repulsion` · boolean — Repulsion is a factor of stiffness
- `use_factor_density` · boolean — Density is calculated as a factor of default density (depends on particle size)
- `use_factor_radius` · boolean — Interaction radius is a factor of 4 * particle size
- `use_factor_stiff_viscosity` · boolean — Stiff viscosity is a factor of normal viscosity
- `use_factor_rest_length` · boolean — Spring rest length is a factor of 2 * particle size
