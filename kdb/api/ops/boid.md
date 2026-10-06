---
area: api-ops
group: boid
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.boid

- `bpy.ops.boid.rule_add(type: enum{GOAL, AVOID, AVOID_COLLISION, SEPARATE, FLOCK, FOLLOW_LEADER, AVERAGE_SPEED, FIGHT})` — Add a boid rule to the current boid state
- `bpy.ops.boid.rule_del()` — Delete current boid rule
- `bpy.ops.boid.rule_move_down()` — Move boid rule down in the list
- `bpy.ops.boid.rule_move_up()` — Move boid rule up in the list
- `bpy.ops.boid.state_add()` — Add a boid state to the particle system
- `bpy.ops.boid.state_del()` — Delete current boid state
- `bpy.ops.boid.state_move_down()` — Move boid state down in the list
- `bpy.ops.boid.state_move_up()` — Move boid state up in the list