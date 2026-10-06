//! MCP resources and prompts (docs and guides served to the model).
//!
//! Tools are NOT here: they live in `crate::tools`, declared once and consumed
//! by both the server and the Blender-side .pyd.
/// All MCP tools - ~70 tools for full Blender control
pub fn get_resources() -> serde_json::Value {
    serde_json::json!([
    // === Guides ===
    {
        "uri": "blender://guide/quickstart",
        "name": "Quick Start Guide",
        "description": "How to use Blender MCP tools effectively",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://guide/materials",
        "name": "Materials Guide",
        "description": "Creating PBR materials with Principled BSDF",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://guide/animation",
        "name": "Animation Guide",
        "description": "Keyframing, fcurves, and animation workflow",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://guide/lighting",
        "name": "Lighting Guide",
        "description": "Light types, HDRI, and lighting setups",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://guide/modifiers",
        "name": "Modifiers Guide",
        "description": "Non-destructive modeling with modifiers",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://guide/physics",
        "name": "Physics Guide",
        "description": "Rigid body, cloth, particles, force fields",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://guide/rigging",
        "name": "Rigging Guide",
        "description": "Armatures, bones, IK, and weight painting",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://guide/instancing",
        "name": "Instancing Guide",
        "description": "Efficient scene building with linked duplicates",
        "mimeType": "text/markdown"
    },
    // === Examples ===
    {
        "uri": "blender://examples/scene-setup",
        "name": "Scene Setup Example",
        "description": "Complete product visualization scene",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://examples/city-builder",
        "name": "City Builder Example",
        "description": "Building a city with instances and collections",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://examples/character-rig",
        "name": "Character Rig Example",
        "description": "Simple biped armature setup",
        "mimeType": "text/markdown"
    },
    // === Reference ===
    {
        "uri": "blender://ref/primitives",
        "name": "Primitives Reference",
        "description": "All available primitive types and parameters",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://ref/modifier-types",
        "name": "Modifier Types Reference",
        "description": "All modifier types and their properties",
        "mimeType": "text/markdown"
    },
    {
        "uri": "blender://ref/constraint-types",
        "name": "Constraint Types Reference",
        "description": "All constraint types for rigging and animation",
        "mimeType": "text/markdown"
    },
    // === Creative Tools ===
                                                        ])
}

/// Read resource content by URI
pub fn read_resource(uri: &str) -> Option<serde_json::Value> {
    let content = match uri {
        "blender://guide/quickstart" => {
            r#"# Blender MCP Quick Start

## Basic Workflow
1. Use `scene_get_info` to understand current scene state
2. Use `obj_list` to see what exists
3. Create objects with `obj_create_primitive`
4. Apply materials with `mat_create` + `mat_assign`
5. Use `scene_screenshot` to preview (returns image directly!)

## Tips
- Always check scene_get_info first to understand context
- Object names are case-sensitive
- Locations are [x, y, z] arrays
- Rotations are in radians (use math: 90 deg = 1.5708 rad)
- Colors are [r, g, b, a] with values 0-1

## Common Patterns
```
# Create a red cube at origin
obj_create_primitive(type="cube", name="MyCube", location=[0,0,0])
mat_create(name="Red", color=[1,0,0,1])
mat_assign(object="MyCube", material="Red")
scene_screenshot(mode="viewport", shading="material")
```"#
        }

        "blender://guide/materials" => {
            r#"# Materials Guide

## Creating Materials
Use `mat_create` with Principled BSDF properties:
- `name`: Material name (required)
- `color`: [r, g, b, a] base color, values 0-1
- `metallic`: 0 (dielectric) to 1 (metal)
- `roughness`: 0 (glossy) to 1 (matte)

## Examples
```
# Shiny gold
mat_create(name="Gold", color=[1, 0.8, 0.2, 1], metallic=1, roughness=0.2)

# Matte plastic
mat_create(name="Plastic", color=[0.2, 0.5, 0.8, 1], metallic=0, roughness=0.7)

# Chrome
mat_create(name="Chrome", color=[0.9, 0.9, 0.95, 1], metallic=1, roughness=0.05)
```

## Assigning
```
mat_assign(object="Cube", material="Gold")
```"#
        }

        "blender://guide/animation" => {
            r#"# Animation Guide

## Keyframing
Use `anim_set_keyframe` to animate object properties:
- `name`: Object to animate
- `property`: location, rotation_euler, scale
- `frame`: Frame number (optional, uses current)

## Workflow
1. Set object transform with `obj_update`
2. Insert keyframe with `anim_set_keyframe`
3. Move to another frame, update, keyframe again

## Example: Bouncing Ball
```
# Frame 1: ball on ground
obj_update(name="Ball", location=[0, 0, 0])
anim_set_keyframe(name="Ball", property="location", frame=1)

# Frame 12: ball in air
obj_update(name="Ball", location=[0, 0, 3])
anim_set_keyframe(name="Ball", property="location", frame=12)

# Frame 24: back down
obj_update(name="Ball", location=[0, 0, 0])
anim_set_keyframe(name="Ball", property="location", frame=24)
```"#
        }

        "blender://examples/scene-setup" => {
            r#"# Complete Scene Setup Example

This creates a simple product visualization scene.

## Step 1: Clear and Setup
```
scene_get_info()
obj_list()
```

## Step 2: Create Ground
```
obj_create_primitive(type="plane", name="Ground", scale=[5, 5, 1])
mat_create(name="Ground_Mat", color=[0.15, 0.15, 0.15, 1], roughness=0.9)
mat_assign(object="Ground", material="Ground_Mat")
```

## Step 3: Create Product
```
obj_create_primitive(type="sphere", name="Product", location=[0, 0, 1])
mat_create(name="Product_Mat", color=[0.8, 0.2, 0.1, 1], metallic=0.3, roughness=0.4)
mat_assign(object="Product", material="Product_Mat")
```

## Step 4: Add Lighting
```
light_create(type="AREA", name="KeyLight", location=[3, -3, 4], energy=200, color=[1,1,1])
light_update(name="KeyLight", size=2)
light_create(type="AREA", name="FillLight", location=[-2, 2, 3], energy=50, color=[0.9,0.95,1])
```

## Step 5: Preview
```
scene_screenshot(type="viewport")
```"#
        }

        "blender://guide/lighting" => {
            r#"# Lighting Guide

## Light Types
- **POINT**: Omnidirectional, like a light bulb
- **SUN**: Parallel rays, outdoor daylight
- **SPOT**: Cone of light, stage lighting
- **AREA**: Soft shadows, studio lighting

## Creating Lights
```
light_create(type="AREA", name="Key", location=[3,-3,4], energy=200, color=[1,0.95,0.9])
light_create(type="POINT", name="Fill", location=[-2,2,2], energy=50)
light_create(type="SUN", name="Sun", energy=3)
obj_update(name="Sun", rotation=[0.8, 0, 0.5])
```

## Three-Point Lighting
1. **Key Light**: Main light, 45 deg from camera, high
2. **Fill Light**: Opposite side, softer, lower intensity
3. **Rim/Back Light**: Behind subject, creates edge separation

## HDRI Environment
```
world_set_hdri(path="/path/to/studio.hdr", strength=1.0)
```

## Tips
- Area lights = soft shadows (increase size for softer)
- Sun = hard parallel shadows (good for outdoor)
- Energy values: 100-500 for indoor, 1-5 for sun
- Color temperature: warm [1,0.9,0.8], cool [0.9,0.95,1]"#
        }

        "blender://guide/modifiers" => {
            r#"# Modifiers Guide

## Common Modifiers
| Type | Use Case |
|------|----------|
| SUBSURF | Smooth subdivision |
| MIRROR | Symmetrical modeling |
| ARRAY | Repeat object |
| BEVEL | Rounded edges |
| BOOLEAN | Cut/join meshes |
| SOLIDIFY | Add thickness |
| DECIMATE | Reduce polygons |

## Adding Modifiers
```
mod_add(name="Cube", type="SUBSURF", properties={"levels": 2, "render_levels": 3})
mod_add(name="Cube", type="BEVEL", properties={"width": 0.1, "segments": 3})
mod_add(name="Cube", type="MIRROR", properties={"use_axis": [true, false, false]})
```

## Array for Repetition
```
mod_add(name="Pillar", type="ARRAY", properties={
    "count": 10,
    "relative_offset_displace": [1.5, 0, 0]
})
```

## Modifier Stack
- Order matters! Top applies first
- Use `mod_apply` to make permanent
- Use `update_modifier` to change properties"#
        }

        "blender://guide/physics" => {
            r#"# Physics Guide

## Physics Types
- **RIGID_BODY**: Solid objects (falling, collisions)
- **SOFT_BODY**: Deformable (jelly, cloth-like)
- **CLOTH**: Fabric simulation
- **FLUID**: Liquids and smoke
- **COLLISION**: Static colliders

## Rigid Body Setup
```
# Dynamic object (falls, bounces)
phys_add(name="Ball", type="RIGID_BODY", body_type="ACTIVE", mass=1.0, friction=0.5, bounciness=0.8)

# Static collider (ground)
phys_add(name="Ground", type="RIGID_BODY", body_type="PASSIVE")
```

## Particle System
```
phys_add_particles(name="Emitter", ps_name="Sparks", count=1000, lifetime=50, emit_from="FACE")
```

## Force Fields
```
phys_add_force_field(type="WIND", name="Wind", location=[0,0,0], strength=10)
phys_add_force_field(type="VORTEX", name="Swirl", location=[0,0,2], strength=5)
phys_add_force_field(type="TURBULENCE", name="Chaos", strength=3)
```

## Baking
```
phys_bake(frame_start=1, frame_end=250)
```"#
        }

        "blender://guide/rigging" => {
            r#"# Rigging Guide

## Creating Armature
```
rig_create_armature(name="Skeleton", location=[0,0,0])
```

## Adding Bones
```
# Spine
rig_add_bone(name="Skeleton", bone_name="Spine", head=[0,0,0], tail=[0,0,1])
rig_add_bone(name="Skeleton", bone_name="Chest", head=[0,0,1], tail=[0,0,1.5], parent="Spine")

# Arm chain
rig_add_bone(name="Skeleton", bone_name="UpperArm.L", head=[0.3,0,1.4], tail=[0.8,0,1.2], parent="Chest")
rig_add_bone(name="Skeleton", bone_name="LowerArm.L", head=[0.8,0,1.2], tail=[1.3,0,1.4], parent="UpperArm.L")
rig_add_bone(name="Skeleton", bone_name="Hand.L", head=[1.3,0,1.4], tail=[1.5,0,1.4], parent="LowerArm.L")
```

## IK (Inverse Kinematics)
```
rig_add_ik(name="Skeleton", bone_name="LowerArm.L", target="IK_Hand.L", chain_length=2)
```

## Weight Painting
```
# Parent mesh to armature with automatic weights
rig_auto_weights(mesh="Character", armature="Skeleton")
```

## Posing
```
pose_bone(armature="Skeleton", bone="UpperArm.L", rotation=[0, 0, 0.5])
anim_set_keyframe(object="Skeleton", property="pose", frame=1)
```"#
        }

        "blender://guide/instancing" => {
            r#"# Instancing Guide

## Why Instances?
- Share mesh data = less memory
- Fast scene building
- Easy to update all at once

## Linked Duplicates
```
# Create base object
obj_create_primitive(type="cube", name="Base_Building", scale=[2,2,5])
mod_add(name="Base_Building", type="BEVEL", properties={"width": 0.1})
mat_create(name="Concrete", color=[0.5,0.5,0.5])
mat_assign(object="Base_Building", material="Concrete")

# Hide base
obj_update(name="Base_Building", visible=false)

# Create instances (linked=true shares mesh data)
obj_duplicate(name="Base_Building", new_name="Building_01", offset=[0,0,0], linked=true)
obj_duplicate(name="Base_Building", new_name="Building_02", offset=[10,0,0], linked=true)
obj_duplicate(name="Base_Building", new_name="Building_03", offset=[20,0,0], linked=true)

# Each instance can have unique transform
obj_update(name="Building_02", scale=[2,2,8])
obj_update(name="Building_03", rotation=[0,0,0.5])
```

## Collections for Organization
```
col_create(name="Buildings")
col_move_to(object="Building_01", collection="Buildings")
col_move_to(object="Building_02", collection="Buildings")
```

## Batch Creation Pattern
```
# Use util_execute_python for mass instancing
util_execute_python(code='''
import bpy
import random
base = bpy.data.objects["Base_Tree"]
for i in range(100):
    obj = bpy.data.objects.new(f"Tree_{i}", base.data)
    obj.location = [random.uniform(-50,50), random.uniform(-50,50), 0]
    obj.rotation_euler.z = random.uniform(0, 6.28)
    obj.scale = [1,1,random.uniform(0.8,1.2)]
    bpy.context.scene.collection.objects.link(obj)
''')
```"#
        }

        "blender://examples/city-builder" => {
            r#"# City Builder Example

Build a city district using base buildings + instances.

## 1. Create Base Buildings
```
# Skyscraper base
obj_create_primitive(type="cube", name="Base_Tower", location=[0,0,15], scale=[3,3,15])
mat_create(name="Glass", color=[0.1,0.15,0.2], metallic=0.9, roughness=0.1)
mat_assign(object="Base_Tower", material="Glass")
mod_add(name="Base_Tower", type="BEVEL", properties={"width":0.1})

# Office base
obj_create_primitive(type="cube", name="Base_Office", location=[0,0,6], scale=[5,4,6])
mat_create(name="Concrete", color=[0.4,0.4,0.4], roughness=0.85)
mat_assign(object="Base_Office", material="Concrete")

# Hide bases
obj_update(name="Base_Tower", visible=false)
obj_update(name="Base_Office", visible=false)
```

## 2. Create Ground
```
obj_create_primitive(type="plane", name="Ground", scale=[100,100,1])
mat_create(name="Asphalt", color=[0.1,0.1,0.1], roughness=0.95)
mat_assign(object="Ground", material="Asphalt")
```

## 3. Instance Buildings
```
# Downtown towers (5x5 grid)
for x in [-30,-15,0,15,30]:
    for y in [-30,-15,0,15,30]:
        obj_duplicate(name="Base_Tower", new_name=f"Tower_{x}_{y}",
                        offset=[x,y,0], linked=true)

# Surrounding offices
for x in [-50,50]:
    for y in range(-40,41,20):
        obj_duplicate(name="Base_Office", new_name=f"Office_{x}_{y}",
                        offset=[x,y,0], linked=true)
```

## 4. Add Lighting
```
light_create(type="SUN", name="Moon", energy=0.3, color=[0.7,0.8,1])
world_set_background(color=[0.01,0.02,0.04])

# Street lights
for x in range(-40,41,20):
    light_create(type="POINT", name=f"Street_{x}", location=[x,-45,5], energy=300)
```

## 5. Camera
```
cam_create(name="Overview", location=[80,80,60], lens=35)
cam_look_at(object="Overview", target=[0,0,15])
cam_set_active(name="Overview")
scene_screenshot(type="viewport")
```"#
        }

        "blender://examples/character-rig" => {
            r#"# Simple Character Rig

Create a basic biped armature.

## 1. Create Armature
```
rig_create_armature(name="Rig", location=[0,0,0])
```

## 2. Spine Chain
```
rig_add_bone(name="Rig", bone_name="Root", head=[0,0,0], tail=[0,0,0.2])
rig_add_bone(name="Rig", bone_name="Spine", head=[0,0,0.2], tail=[0,0,0.6], parent="Root")
rig_add_bone(name="Rig", bone_name="Chest", head=[0,0,0.6], tail=[0,0,1.0], parent="Spine")
rig_add_bone(name="Rig", bone_name="Neck", head=[0,0,1.0], tail=[0,0,1.2], parent="Chest")
rig_add_bone(name="Rig", bone_name="Head", head=[0,0,1.2], tail=[0,0,1.5], parent="Neck")
```

## 3. Legs
```
# Left leg
rig_add_bone(name="Rig", bone_name="Thigh.L", head=[0.15,0,0.2], tail=[0.15,0.1,-0.3], parent="Root")
rig_add_bone(name="Rig", bone_name="Shin.L", head=[0.15,0.1,-0.3], tail=[0.15,0,-0.8], parent="Thigh.L")
rig_add_bone(name="Rig", bone_name="Foot.L", head=[0.15,0,-0.8], tail=[0.15,-0.15,-0.85], parent="Shin.L")

# Right leg (mirror)
rig_add_bone(name="Rig", bone_name="Thigh.R", head=[-0.15,0,0.2], tail=[-0.15,0.1,-0.3], parent="Root")
rig_add_bone(name="Rig", bone_name="Shin.R", head=[-0.15,0.1,-0.3], tail=[-0.15,0,-0.8], parent="Thigh.R")
rig_add_bone(name="Rig", bone_name="Foot.R", head=[-0.15,0,-0.8], tail=[-0.15,-0.15,-0.85], parent="Shin.R")
```

## 4. Arms
```
# Left arm
rig_add_bone(name="Rig", bone_name="Shoulder.L", head=[0.1,0,0.95], tail=[0.25,0,0.9], parent="Chest")
rig_add_bone(name="Rig", bone_name="UpperArm.L", head=[0.25,0,0.9], tail=[0.55,0,0.85], parent="Shoulder.L")
rig_add_bone(name="Rig", bone_name="LowerArm.L", head=[0.55,0,0.85], tail=[0.85,0,0.9], parent="UpperArm.L")
rig_add_bone(name="Rig", bone_name="Hand.L", head=[0.85,0,0.9], tail=[0.95,0,0.9], parent="LowerArm.L")

# Right arm (mirror)
rig_add_bone(name="Rig", bone_name="Shoulder.R", head=[-0.1,0,0.95], tail=[-0.25,0,0.9], parent="Chest")
rig_add_bone(name="Rig", bone_name="UpperArm.R", head=[-0.25,0,0.9], tail=[-0.55,0,0.85], parent="Shoulder.R")
rig_add_bone(name="Rig", bone_name="LowerArm.R", head=[-0.55,0,0.85], tail=[-0.85,0,0.9], parent="UpperArm.R")
rig_add_bone(name="Rig", bone_name="Hand.R", head=[-0.85,0,0.9], tail=[-0.95,0,0.9], parent="LowerArm.R")
```

## 5. Add IK
```
# Create IK targets
obj_create_primitive(type="sphere", name="IK_Foot.L", location=[0.15,-0.15,-0.85], scale=[0.05,0.05,0.05])
obj_create_primitive(type="sphere", name="IK_Foot.R", location=[-0.15,-0.15,-0.85], scale=[0.05,0.05,0.05])
obj_create_primitive(type="sphere", name="IK_Hand.L", location=[0.85,0,0.9], scale=[0.05,0.05,0.05])
obj_create_primitive(type="sphere", name="IK_Hand.R", location=[-0.85,0,0.9], scale=[0.05,0.05,0.05])

# Add IK constraints
rig_add_ik(name="Rig", bone_name="Shin.L", target="IK_Foot.L", chain_length=2)
rig_add_ik(name="Rig", bone_name="Shin.R", target="IK_Foot.R", chain_length=2)
rig_add_ik(name="Rig", bone_name="LowerArm.L", target="IK_Hand.L", chain_length=2)
rig_add_ik(name="Rig", bone_name="LowerArm.R", target="IK_Hand.R", chain_length=2)
```"#
        }

        "blender://ref/primitives" => {
            r#"# Primitives Reference

## Available Types
| Type | Description |
|------|-------------|
| cube | Box, default 2x2x2 |
| sphere | UV sphere |
| icosphere | Icosahedron-based sphere |
| cylinder | Cylinder with caps |
| cone | Cone with base |
| torus | Donut shape |
| plane | Flat quad |
| circle | Edge loop (no fill) |
| grid | Subdivided plane |
| monkey | Suzanne test mesh |
| empty | Null object (no geometry) |

## Parameters
```
obj_create_primitive(
    type="cube",           # Required
    name="MyCube",         # Optional, auto-generated if omitted
    location=[0, 0, 0],    # [x, y, z]
    rotation=[0, 0, 0],    # Euler radians [x, y, z]
    scale=[1, 1, 1]        # [x, y, z]
)
```

## Type-Specific Notes
- **sphere**: segments=32, rings=16 by default
- **cylinder**: vertices=32, depth=2
- **torus**: major_radius=1, minor_radius=0.25
- **grid**: x_subdivisions=10, y_subdivisions=10
- **circle**: vertices=32, fill_type='NOTHING'"#
        }

        "blender://ref/modifier-types" => {
            r#"# Modifier Types Reference

## Generate
| Type | Key Params |
|------|------------|
| ARRAY | count, relative_offset_displace, use_object_offset |
| BEVEL | width, segments, limit_method |
| BOOLEAN | operation (DIFFERENCE/UNION/INTERSECT), object |
| BUILD | frame_start, frame_duration |
| DECIMATE | ratio, decimate_type |
| MIRROR | use_axis [x,y,z], mirror_object |
| REMESH | mode, octree_depth |
| SCREW | angle, steps, axis |
| SKIN | - |
| SOLIDIFY | thickness, offset |
| SUBSURF | levels, render_levels, uv_smooth |
| TRIANGULATE | quad_method, ngon_method |
| WIREFRAME | thickness |

## Deform
| Type | Key Params |
|------|------------|
| ARMATURE | object, use_vertex_groups |
| CAST | cast_type, factor |
| CURVE | object, deform_axis |
| DISPLACE | texture, strength, mid_level |
| HOOK | object, vertex_group |
| LATTICE | object |
| SHRINKWRAP | target, wrap_method |
| SIMPLE_DEFORM | deform_method, angle |
| SMOOTH | factor, iterations |
| WAVE | height, width |

## Example
```
mod_add(name="Cube", type="SUBSURF", properties={
    "levels": 2,
    "render_levels": 3,
    "uv_smooth": "PRESERVE_CORNERS"
})
```"#
        }

        "blender://ref/constraint-types" => {
            r#"# Constraint Types Reference

## Transform Constraints
| Type | Use |
|------|-----|
| COPY_LOCATION | Copy position from target |
| COPY_ROTATION | Copy rotation from target |
| COPY_SCALE | Copy scale from target |
| COPY_TRANSFORMS | Copy all transforms |
| LIMIT_LOCATION | Clamp position |
| LIMIT_ROTATION | Clamp rotation |
| LIMIT_SCALE | Clamp scale |

## Tracking Constraints
| Type | Use |
|------|-----|
| TRACK_TO | Point axis at target |
| DAMPED_TRACK | Smooth tracking |
| LOCKED_TRACK | Track with locked axis |
| STRETCH_TO | Point and stretch |
| CLAMP_TO | Follow curve |

## Relationship Constraints
| Type | Use |
|------|-----|
| CHILD_OF | Parent without hierarchy |
| FLOOR | Stay above target |
| FOLLOW_PATH | Animate along curve |
| PIVOT | Change rotation center |
| SHRINKWRAP | Stick to surface |

## Example
```
con_add(name="Camera", type="TRACK_TO", target="Subject", track_axis="TRACK_NEGATIVE_Z", up_axis="UP_Y")

con_add(name="Eye", type="DAMPED_TRACK", target="LookTarget", track_axis="TRACK_Z")
```"#
        }

        // ========== CREATIVE TOOLS RESOURCES ==========
        _ => return None,
    };

    Some(serde_json::json!({
        "contents": [{
            "uri": uri,
            "mimeType": "text/markdown",
            "text": content
        }]
    }))
}

/// MCP Prompts - reusable prompt templates
pub fn get_prompts() -> serde_json::Value {
    serde_json::json!([
        // === Scene Creation ===
        {
            "name": "create-scene",
            "description": "Create a 3D scene from description",
            "arguments": [
                {"name": "description", "description": "What to create", "required": true}
            ]
        },
        {
            "name": "analyze-scene",
            "description": "Analyze current Blender scene and suggest improvements",
            "arguments": []
        },
        {
            "name": "build-environment",
            "description": "Build a complete environment (city, forest, interior, etc.)",
            "arguments": [
                {"name": "type", "description": "city, forest, interior, desert, ocean", "required": true},
                {"name": "scale", "description": "small, medium, large", "required": false}
            ]
        },
        // === Lighting ===
        {
            "name": "setup-lighting",
            "description": "Set up professional lighting for the scene",
            "arguments": [
                {"name": "style", "description": "studio, outdoor, dramatic, soft, night", "required": false}
            ]
        },
        {
            "name": "setup-hdri",
            "description": "Configure HDRI environment lighting",
            "arguments": [
                {"name": "mood", "description": "sunny, overcast, sunset, night, studio", "required": false}
            ]
        },
        // === Materials ===
        {
            "name": "create-material",
            "description": "Create a material from description",
            "arguments": [
                {"name": "description", "description": "Material description (e.g. 'brushed gold', 'red plastic')", "required": true}
            ]
        },
        {
            "name": "material-library",
            "description": "Create a set of related materials",
            "arguments": [
                {"name": "theme", "description": "metals, woods, fabrics, stones, plastics", "required": true}
            ]
        },
        // === Animation ===
        {
            "name": "animate-camera",
            "description": "Create camera animation (flyover, orbit, dolly)",
            "arguments": [
                {"name": "type", "description": "flyover, orbit, dolly, tracking", "required": true},
                {"name": "duration", "description": "Animation length in frames", "required": false}
            ]
        },
        {
            "name": "animate-object",
            "description": "Animate object movement or transformation",
            "arguments": [
                {"name": "object", "description": "Object name to animate", "required": true},
                {"name": "animation", "description": "bounce, rotate, scale-pulse, path-follow", "required": true}
            ]
        },
        // === Modeling ===
        {
            "name": "create-character",
            "description": "Create a simple character with basic rig",
            "arguments": [
                {"name": "style", "description": "humanoid, quadruped, robot, cartoon", "required": false}
            ]
        },
        {
            "name": "create-vehicle",
            "description": "Create a vehicle model",
            "arguments": [
                {"name": "type", "description": "car, truck, plane, boat, spaceship", "required": true}
            ]
        },
        {
            "name": "create-furniture",
            "description": "Create furniture piece",
            "arguments": [
                {"name": "type", "description": "chair, table, sofa, bed, shelf", "required": true},
                {"name": "style", "description": "modern, classic, industrial", "required": false}
            ]
        },
        // === Effects ===
        {
            "name": "add-particles",
            "description": "Add particle effects (fire, smoke, rain, sparks)",
            "arguments": [
                {"name": "effect", "description": "fire, smoke, rain, snow, sparks, dust", "required": true},
                {"name": "object", "description": "Emitter object name", "required": false}
            ]
        },
        {
            "name": "setup-physics",
            "description": "Set up physics simulation",
            "arguments": [
                {"name": "type", "description": "falling-objects, cloth, fluid, destruction", "required": true}
            ]
        },
        // === Rendering ===
        {
            "name": "render-turntable",
            "description": "Create turntable animation of object/scene",
            "arguments": [
                {"name": "frames", "description": "Number of frames (default 120)", "required": false}
            ]
        },
        {
            "name": "render-preview",
            "description": "Quick preview render with multiple angles",
            "arguments": []
        },
        // === Utility ===
        {
            "name": "cleanup-scene",
            "description": "Clean up scene: remove unused data, fix issues",
            "arguments": []
        },
        {
            "name": "organize-scene",
            "description": "Organize objects into collections by type",
            "arguments": []
        }
    ])
}

/// Get prompt content by name
pub fn get_prompt(name: &str, args: &serde_json::Value) -> Option<serde_json::Value> {
    let messages = match name {
        "create-scene" => {
            let desc = args
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("a simple scene");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Create a 3D scene in Blender: {}

Instructions:
1. First call scene_get_info() to check current state
2. Create necessary primitives with obj_create_primitive()
3. Create and assign appropriate materials
4. Position objects logically in 3D space
5. Take a scene_screenshot to show the result

Use realistic proportions and pleasing composition."#, desc)
                }
            })]
        }

        "analyze-scene" => {
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": r#"Analyze the current Blender scene:

1. Call scene_get_info() to get overview
2. Call obj_list() to see all objects
3. Call mat_list() to check materials
4. Take a scene_screenshot() to see visually

Then provide:
- Summary of scene contents
- Potential issues (missing materials, bad transforms, etc.)
- Suggestions for improvement
- Next steps if this is a work in progress"#
                }
            })]
        }

        "setup-lighting" => {
            let style = args
                .get("style")
                .and_then(|v| v.as_str())
                .unwrap_or("studio");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Set up {} lighting in Blender:

Use util_execute_python() to add lights. Common setups:

Studio: Key light (strong, angled), fill light (soft, opposite), rim light (behind)
Outdoor: Sun lamp with sky texture
Dramatic: Single strong directional light with shadows
Soft: Large area lights surrounding subject

After setup, take a scene_screenshot with shading="rendered" to preview."#, style)
                }
            })]
        }

        "create-material" => {
            let desc = args
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("default material");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Create a Blender material: {}

Use mat_create() with appropriate values:
- color: [r, g, b, a] values 0-1
- metallic: 0 for plastic/wood/fabric, 1 for metals
- roughness: 0 for mirrors/chrome, 0.3-0.5 for polished, 0.7+ for matte

Common recipes:
- Gold: color=[1, 0.8, 0.3, 1], metallic=1, roughness=0.3
- Plastic: color=[...], metallic=0, roughness=0.4
- Rubber: color=[...], metallic=0, roughness=0.9
- Glass: color=[1,1,1,0.1], metallic=0, roughness=0

After creating, assign to an object and take a scene_screenshot with material preview."#, desc)
                }
            })]
        }

        "build-environment" => {
            let env_type = args.get("type").and_then(|v| v.as_str()).unwrap_or("city");
            let scale = args
                .get("scale")
                .and_then(|v| v.as_str())
                .unwrap_or("medium");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Build a {} environment at {} scale in Blender.

Steps:
1. Create ground/terrain with appropriate material
2. Create base building/tree/furniture types
3. Use obj_duplicate(linked=true) for efficient instancing
4. Organize into collections (Buildings, Props, Lights, etc.)
5. Add appropriate lighting (sun for outdoor, area lights for interior)
6. Set world background color or HDRI
7. Create camera and position for best view
8. Take scene_screenshot to preview

Use the instancing guide pattern: create base objects, hide them, then instance across the scene."#, env_type, scale)
                }
            })]
        }

        "setup-hdri" => {
            let mood = args
                .get("mood")
                .and_then(|v| v.as_str())
                .unwrap_or("studio");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Set up HDRI environment for {} mood.

If HDRI file available:
  world_set_hdri(path="/path/to/hdri.hdr", strength=1.0)

Otherwise, create procedural sky with world_set_background():
- sunny: color=[0.5, 0.7, 1.0], strength=1.5
- overcast: color=[0.6, 0.6, 0.65], strength=1.0
- sunset: color=[1.0, 0.6, 0.3], strength=1.2
- night: color=[0.02, 0.03, 0.05], strength=0.5
- studio: color=[0.15, 0.15, 0.15], strength=0.8

Add complementary lights as needed. Take scene_screenshot to preview."#, mood)
                }
            })]
        }

        "material-library" => {
            let theme = args
                .get("theme")
                .and_then(|v| v.as_str())
                .unwrap_or("metals");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Create a {} material library in Blender.

Create 5-8 materials in this category:

Metals: Gold, Silver, Copper, Bronze, Steel, Chrome, Brushed Aluminum
Woods: Oak, Walnut, Pine, Mahogany, Bamboo, Plywood
Fabrics: Cotton, Silk, Velvet, Denim, Leather, Wool
Stones: Marble, Granite, Concrete, Brick, Slate, Sandstone
Plastics: Glossy, Matte, Rubber, Translucent, Metallic

For each material, use mat_create() with appropriate:
- color (base tone)
- metallic (1 for metals, 0 otherwise)
- roughness (varies by finish)

Optionally create test spheres to display each material, arranged in a row.
Take scene_screenshot showing all materials."#, theme)
                }
            })]
        }

        "animate-camera" => {
            let anim_type = args.get("type").and_then(|v| v.as_str()).unwrap_or("orbit");
            let duration = args
                .get("duration")
                .and_then(|v| v.as_str())
                .unwrap_or("120");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Create {} camera animation over {} frames.

Animation types:
- orbit: Circle around subject, constant height, always look at center
- flyover: Start high/far, descend and approach
- dolly: Move forward/backward in straight line
- tracking: Follow a moving object

Steps:
1. Create camera with cam_create()
2. Position at start with obj_update()
3. Use cam_look_at() to aim at subject
4. anim_set_keyframe(property="location", frame=1)
5. anim_set_keyframe(property="rotation_euler", frame=1)
6. Move to key frames, update position, look_at, keyframe again
7. Set as active camera with cam_set_active()

For orbit, use util_execute_python with math to calculate circular positions."#, anim_type, duration)
                }
            })]
        }

        "animate-object" => {
            let obj = args
                .get("object")
                .and_then(|v| v.as_str())
                .unwrap_or("Cube");
            let anim = args
                .get("animation")
                .and_then(|v| v.as_str())
                .unwrap_or("bounce");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Animate object '{}' with {} animation.

Bounce (location):
- Frame 1: ground position, key location
- Frame 12: high position, key location
- Frame 24: ground, key location (repeat)

Rotate (rotation_euler):
- Frame 1: rotation=[0,0,0], key
- Frame 60: rotation=[0,0,6.28], key (full rotation)

Scale-pulse (scale):
- Frame 1: scale=[1,1,1], key
- Frame 15: scale=[1.2,1.2,1.2], key
- Frame 30: scale=[1,1,1], key

Path-follow:
- Create curve with create_curve()
- Add FOLLOW_PATH constraint

Use anim_set_frame(), obj_update(), anim_set_keyframe() for each pose."#, obj, anim)
                }
            })]
        }

        "create-character" => {
            let style = args
                .get("style")
                .and_then(|v| v.as_str())
                .unwrap_or("humanoid");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Create a simple {} character with basic rig.

Steps:
1. Create body parts using primitives (spheres, cylinders, cubes)
2. Position and scale to form character shape
3. Create armature with rig_create_armature()
4. Add bones: root, spine, chest, neck, head
5. Add arm bones: shoulder, upper_arm, lower_arm, hand (both sides)
6. Add leg bones: thigh, shin, foot (both sides)
7. Use rig_add_ik() for hands and feet
8. Use rig_auto_weights() to bind mesh to armature

For {} style, adjust proportions:
- humanoid: realistic proportions
- cartoon: big head, small body
- robot: boxy shapes, no organic curves
- quadruped: horizontal spine, 4 legs

Take scene_screenshot showing the character."#, style, style)
                }
            })]
        }

        "create-vehicle" => {
            let vtype = args.get("type").and_then(|v| v.as_str()).unwrap_or("car");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Create a simple {} model.

Build from primitives:
- Car: cube body, cylinder wheels, cube windows
- Truck: longer cube body, larger wheels, cab section
- Plane: cylinder fuselage, flat wing cubes, tail
- Boat: stretched cube hull, cube cabin
- Spaceship: creative combination of shapes

Steps:
1. Create main body shape
2. Add details (wheels/wings/etc.)
3. Use modifiers: BEVEL for smooth edges, MIRROR for symmetry
4. Create materials: body paint, glass, metal trim
5. Organize parts into collection
6. Parent all to main body
7. Position camera and take scene_screenshot

Use appropriate scale (car ~4m long, plane ~15m wingspan)."#, vtype)
                }
            })]
        }

        "create-furniture" => {
            let ftype = args.get("type").and_then(|v| v.as_str()).unwrap_or("chair");
            let style = args
                .get("style")
                .and_then(|v| v.as_str())
                .unwrap_or("modern");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Create a {} {} furniture piece.

Build from primitives:
- Chair: seat cube, back cube, 4 leg cylinders
- Table: top cube, 4 leg cylinders (or pedestal)
- Sofa: seat cube, back cube, arm cubes, legs
- Bed: mattress cube, headboard cube, legs
- Shelf: back panel, horizontal shelves, sides

Style adjustments:
- modern: clean lines, thin profiles, minimal detail
- classic: curves, ornate, thicker elements
- industrial: raw materials, metal, visible hardware

Steps:
1. Create base shapes for each part
2. Add BEVEL modifier for rounded edges
3. Create wood/fabric/metal materials
4. Assign materials to parts
5. Parent all to main piece
6. Scale to realistic dimensions

Take scene_screenshot with material preview."#, style, ftype)
                }
            })]
        }

        "add-particles" => {
            let effect = args
                .get("effect")
                .and_then(|v| v.as_str())
                .unwrap_or("sparks");
            let obj = args.get("object").and_then(|v| v.as_str()).unwrap_or("");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Add {} particle effect{}.

Effect settings:
- fire: count=500, lifetime=30, velocity_normal=3, size_random=0.5
- smoke: count=200, lifetime=100, velocity_normal=1, brownian=1
- rain: count=5000, lifetime=50, velocity=[0,0,-15]
- snow: count=2000, lifetime=200, velocity=[0.5,0,-2], brownian=0.5
- sparks: count=100, lifetime=20, velocity_normal=8, size=0.02
- dust: count=300, lifetime=150, velocity_normal=0.5, brownian=2

Steps:
1. Select or create emitter object
2. phys_add_particles(name=emitter, ps_name="{}", ...)
3. Optionally add force_field for wind/turbulence
4. Set frame range for animation
5. Take scene_screenshot or render to preview"#,
                        effect,
                        if obj.is_empty() { String::new() } else { format!(" on '{}'", obj) },
                        effect)
                }
            })]
        }

        "setup-physics" => {
            let ptype = args
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("falling-objects");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Set up {} physics simulation.

Setup by type:

falling-objects:
1. Create ground plane
2. phys_add(name="Ground", type="RIGID_BODY", body_type="PASSIVE")
3. Create falling objects (cubes, spheres)
4. phys_add(name=obj, type="RIGID_BODY", body_type="ACTIVE", mass=1)
5. Position objects above ground
6. phys_bake(frame_start=1, frame_end=250)

cloth:
1. Create cloth mesh (subdivided plane)
2. Create collision object below
3. phys_add(name="Cloth", type="CLOTH")
4. phys_add(name="Collider", type="COLLISION")
5. Bake simulation

fluid:
1. Create domain cube (large)
2. Create flow object (sphere)
3. phys_add on each with type="FLUID" and appropriate params
4. Bake simulation

After setup, play animation or bake to see results."#, ptype)
                }
            })]
        }

        "render-turntable" => {
            let frames = args.get("frames").and_then(|v| v.as_str()).unwrap_or("120");
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": format!(r#"Create turntable animation ({} frames).

Steps:
1. Create camera positioned to see subject
2. Create empty at world origin (pivot point)
3. Parent camera to empty
4. Animate empty's Z rotation:
   - Frame 1: rotation=[0,0,0], keyframe
   - Frame {}: rotation=[0,0,6.28], keyframe
5. Set as active camera
6. Configure render settings
7. Render animation or take scene_screenshots at intervals

Alternative: keep camera static, rotate the object/collection instead.

For product shots, also animate subtle lighting changes."#, frames, frames)
                }
            })]
        }

        "render-preview" => {
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": r#"Create quick preview renders from multiple angles.

Steps:
1. Get current scene bounds with scene_get_info() and obj_list()
2. Create cameras at standard positions:
   - Front: [0, -10, 2]
   - Side: [10, 0, 2]
   - Top: [0, 0, 15]
   - 3/4 view: [8, -8, 5]
3. For each camera:
   - cam_set_active()
   - cam_look_at() targeting scene center
   - scene_screenshot(type="viewport") or scene_screenshot(type="render")
4. Return all screenshots

Use EEVEE for fast previews, CYCLES for quality."#
                }
            })]
        }

        "cleanup-scene" => {
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": r#"Clean up the Blender scene.

Steps:
1. scene_get_info() - check current state
2. obj_list() - find unused objects
3. mat_list() - find unused materials

Cleanup tasks:
- Delete objects with default names that seem unused (Cube.001, etc.)
- Remove materials not assigned to any object
- Fix objects at origin that should be elsewhere
- Remove empty collections
- Apply scale on objects (via util_execute_python)

Use util_execute_python for:
```python
import bpy
# Remove unused data blocks
bpy.ops.outliner.orphans_purge(do_recursive=True)
# Apply transforms
bpy.ops.object.transform_apply(location=False, rotation=True, scale=True)
```

Report what was cleaned up."#
                }
            })]
        }

        "organize-scene" => {
            vec![serde_json::json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": r#"Organize scene objects into collections by type.

Steps:
1. obj_list() to get all objects
2. Create collections:
   - col_create(name="Meshes")
   - col_create(name="Lights")
   - col_create(name="Cameras")
   - col_create(name="Empties")
   - col_create(name="Armatures")
   - col_create(name="Curves")
3. Sort objects by type:
   - col_move_to() for each object
4. Create sub-collections if needed:
   - "Props", "Characters", "Environment" under Meshes
5. Verify with col_list()

Naming convention suggestions:
- Use PascalCase for collections
- Use snake_case for objects
- Prefix related objects (building_wall, building_roof)"#
                }
            })]
        }

        _ => return None,
    };

    Some(serde_json::json!({
        "description": format!("Prompt: {}", name),
        "messages": messages
    }))
}
