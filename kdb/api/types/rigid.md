---
area: api-types
group: rigid
version: 5.2.2
---

# Blender 5.2.2 RNA types: rigid

## RigidBodyWorld

Self-contained rigid body simulation environment and settings

- `collection` · pointer · -> Collection — Collection containing objects participating in this simulation
- `constraints` · pointer · -> Collection — Collection containing rigid body constraint objects
- `enabled` · boolean — Simulation will be evaluated
- `time_scale` · float — Change the speed of the simulation
- `substeps_per_frame` · int — Number of simulation steps taken per frame (higher values are more accurate but slower)
- `solver_iterations` · int — Number of constraint solver iterations made per simulation step (higher values are more accurate but slower)
- `use_split_impulse` · boolean — Reduce extra velocity that can build up when objects collide (lowers simulation stability a little so use only when necessary)
- `point_cache` · pointer · -> PointCache · read-only
- `effector_weights` · pointer · -> EffectorWeights · read-only
