# Worked example: a city district

A full scene built end to end through the tools: a night-time city district of 100+
instanced buildings, street and neon lighting, cameras, an animated flyover, and a render.
It doubles as a broad integration test — ~40 distinct tools across most domains.

## Prerequisites
- Blender connected. Ask the agent to `blender_launch` (or start Blender by hand and
  `MCP > Start`), then confirm with `blender_status` (`connected: true`).

---

## Test Sequence

### Phase 1: Scene Setup & Cleanup

```
1. scene_get_info - check initial state
2. obj_list - see default objects
3. obj_delete(name="Cube") - remove default cube
4. obj_delete(name="Light") - remove default light
```

### Phase 2: Ground & Environment

```
4. obj_create_primitive(type="plane", name="Ground", scale=[200,200,1])
5. mat_create(name="Asphalt", color=[0.08,0.08,0.08], roughness=0.95)
6. mat_assign(object="Ground", material="Asphalt")

# Night city atmosphere
7. world_set_background(color=[0.01,0.015,0.03], strength=1.0)
```

### Phase 3: Building Archetypes (Base meshes for instancing)

Create 8 base building types that will be instanced across the city:

```
# === SKYSCRAPERS (tall, glass) ===
8.  obj_create_primitive(type="cube", name="Base_Skyscraper_A", location=[0,0,15], scale=[3,3,15])
9.  mat_create(name="Glass_Blue", color=[0.05,0.1,0.2], metallic=0.95, roughness=0.05, transmission=0.3)
10. mat_assign(object="Base_Skyscraper_A", material="Glass_Blue")
11. mod_add(name="Base_Skyscraper_A", type="BEVEL", properties={"width":0.15, "segments":2})

12. obj_create_primitive(type="cube", name="Base_Skyscraper_B", location=[0,0,20], scale=[4,2,20])
13. mat_create(name="Glass_Gold", color=[0.3,0.25,0.1], metallic=0.9, roughness=0.1)
14. mat_assign(object="Base_Skyscraper_B", material="Glass_Gold")
15. mod_add(name="Base_Skyscraper_B", type="BEVEL", properties={"width":0.1, "segments":2})

# === OFFICE BUILDINGS (medium, concrete/glass mix) ===
16. obj_create_primitive(type="cube", name="Base_Office_A", location=[0,0,8], scale=[5,4,8])
17. mat_create(name="Concrete_Light", color=[0.5,0.5,0.5], roughness=0.85)
18. mat_assign(object="Base_Office_A", material="Concrete_Light")
19. mod_add(name="Base_Office_A", type="BEVEL", properties={"width":0.08})

20. obj_create_primitive(type="cube", name="Base_Office_B", location=[0,0,6], scale=[6,3,6])
21. mat_create(name="Concrete_Dark", color=[0.25,0.25,0.28], roughness=0.9)
22. mat_assign(object="Base_Office_B", material="Concrete_Dark")

# === APARTMENTS (medium, brick/plaster) ===
23. obj_create_primitive(type="cube", name="Base_Apartment_A", location=[0,0,10], scale=[4,6,10])
24. mat_create(name="Brick_Red", color=[0.45,0.18,0.12], roughness=0.92)
25. mat_assign(object="Base_Apartment_A", material="Brick_Red")

26. obj_create_primitive(type="cube", name="Base_Apartment_B", location=[0,0,7], scale=[8,4,7])
27. mat_create(name="Plaster_Beige", color=[0.7,0.65,0.55], roughness=0.88)
28. mat_assign(object="Base_Apartment_B", material="Plaster_Beige")

# === SHOPS/LOW BUILDINGS ===
29. obj_create_primitive(type="cube", name="Base_Shop_A", location=[0,0,2], scale=[3,2,2])
30. mat_create(name="Storefront", color=[0.2,0.22,0.25], metallic=0.3, roughness=0.4)
31. mat_assign(object="Base_Shop_A", material="Storefront")

32. obj_create_primitive(type="cube", name="Base_Shop_B", location=[0,0,1.5], scale=[4,3,1.5])
33. mat_assign(object="Base_Shop_B", material="Concrete_Light")
```

### Phase 4: Organize Base Buildings

```
34. col_create(name="Base_Buildings")
35. col_move_to(object="Base_Skyscraper_A", collection="Base_Buildings")
36. col_move_to(object="Base_Skyscraper_B", collection="Base_Buildings")
37. col_move_to(object="Base_Office_A", collection="Base_Buildings")
38. col_move_to(object="Base_Office_B", collection="Base_Buildings")
39. col_move_to(object="Base_Apartment_A", collection="Base_Buildings")
40. col_move_to(object="Base_Apartment_B", collection="Base_Buildings")
41. col_move_to(object="Base_Shop_A", collection="Base_Buildings")
42. col_move_to(object="Base_Shop_B", collection="Base_Buildings")

# Hide base buildings (only instances will be visible)
43. obj_update(name="Base_Skyscraper_A", visible=false)
44. obj_update(name="Base_Skyscraper_B", visible=false)
45. obj_update(name="Base_Office_A", visible=false)
46. obj_update(name="Base_Office_B", visible=false)
47. obj_update(name="Base_Apartment_A", visible=false)
48. obj_update(name="Base_Apartment_B", visible=false)
49. obj_update(name="Base_Shop_A", visible=false)
50. obj_update(name="Base_Shop_B", visible=false)
```

### Phase 5: City Grid - Instance Points

Create empty objects as placement points for building instances:

```
51. col_create(name="City_Instances")
52. col_create(name="Downtown", parent="City_Instances")
53. col_create(name="Business_District", parent="City_Instances")
54. col_create(name="Residential", parent="City_Instances")
55. col_create(name="Commercial", parent="City_Instances")

# === DOWNTOWN (Skyscrapers) - Central area ===
# Grid: 5x5 with spacing ~20 units
56-80. [Loop] For x in [-40,-20,0,20,40], y in [-40,-20,0,20,40]:
    obj_create_primitive(type="empty", name=f"Inst_Downtown_{i}", location=[x,y,0])
    # Random rotation for variety
    obj_update(name=f"Inst_Downtown_{i}", rotation=[0,0, random(0, 1.57)])
    col_move_to(object=f"Inst_Downtown_{i}", collection="Downtown")

# === BUSINESS DISTRICT (Offices) - Ring around downtown ===
# 20 buildings around perimeter
81-100. [Loop] For positions around [-60..60] perimeter:
    obj_create_primitive(type="empty", name=f"Inst_Business_{i}", location=[x,y,0])
    col_move_to(object=f"Inst_Business_{i}", collection="Business_District")

# === RESIDENTIAL (Apartments) - Outer ring ===
# 30 buildings in residential zones
101-130. [Loop] For x in [-80,-60,-40...60,80], y in [-80,80] and vice versa:
    obj_create_primitive(type="empty", name=f"Inst_Residential_{i}", location=[x,y,0])
    col_move_to(object=f"Inst_Residential_{i}", collection="Residential")

# === COMMERCIAL (Shops) - Street level scattered ===
# 40 small buildings filling gaps
131-170. [Loop] Scattered positions between major buildings:
    obj_create_primitive(type="empty", name=f"Inst_Commercial_{i}", location=[x,y,0])
    col_move_to(object=f"Inst_Commercial_{i}", collection="Commercial")
```

### Phase 6: Apply Instances

Link base buildings to instance points:

```
# Downtown -> Skyscrapers (alternating A/B)
171. obj_instance_to_points(source="Base_Skyscraper_A", target="Downtown", pick_instance=true)
    OR use util_execute_python for custom instancing logic:

172. util_execute_python(code="""
import bpy
import random

downtown = bpy.data.collections['Downtown']
skyscrapers = ['Base_Skyscraper_A', 'Base_Skyscraper_B']

for empty in downtown.objects:
    if empty.type == 'EMPTY':
        base = bpy.data.objects[random.choice(skyscrapers)]
        empty.instance_type = 'COLLECTION'
        # Or create linked duplicate
        new_obj = bpy.data.objects.new(empty.name + '_inst', base.data)
        new_obj.location = empty.location
        new_obj.rotation_euler = empty.rotation_euler
        new_obj.scale = [random.uniform(0.8, 1.2)] * 3
        bpy.context.scene.collection.objects.link(new_obj)
""")

# Business District -> Offices
173. util_execute_python(code="""
import bpy
import random

district = bpy.data.collections['Business_District']
offices = ['Base_Office_A', 'Base_Office_B']

for empty in district.objects:
    base = bpy.data.objects[random.choice(offices)]
    new_obj = bpy.data.objects.new(empty.name + '_inst', base.data)
    new_obj.location = empty.location
    new_obj.rotation_euler.z = random.uniform(0, 3.14)
    new_obj.scale = [random.uniform(0.9, 1.1)] * 3
    bpy.context.scene.collection.objects.link(new_obj)
    # Copy material
    if base.data.materials:
        new_obj.data.materials.append(base.data.materials[0])
""")

# Residential -> Apartments
174. util_execute_python(code="""
import bpy
import random

residential = bpy.data.collections['Residential']
apartments = ['Base_Apartment_A', 'Base_Apartment_B']

for empty in residential.objects:
    base = bpy.data.objects[random.choice(apartments)]
    new_obj = bpy.data.objects.new(empty.name + '_inst', base.data)
    new_obj.location = empty.location
    new_obj.rotation_euler.z = random.uniform(0, 6.28)
    scale = random.uniform(0.85, 1.15)
    new_obj.scale = [scale, scale, random.uniform(0.7, 1.3)]
    bpy.context.scene.collection.objects.link(new_obj)
""")

# Commercial -> Shops
175. util_execute_python(code="""
import bpy
import random

commercial = bpy.data.collections['Commercial']
shops = ['Base_Shop_A', 'Base_Shop_B']

for empty in commercial.objects:
    base = bpy.data.objects[random.choice(shops)]
    new_obj = bpy.data.objects.new(empty.name + '_inst', base.data)
    new_obj.location = empty.location
    new_obj.rotation_euler.z = random.uniform(0, 6.28)
    bpy.context.scene.collection.objects.link(new_obj)
""")
```

### Phase 7: Linked Duplicates (Alternative to util_execute_python)

Using MCP tools directly for instancing:

```
# Create linked duplicates of skyscrapers for downtown
176-200. [Loop] obj_duplicate(name="Base_Skyscraper_A", new_name="Tower_D{i}",
         offset=[x,y,0], linked=true)

# Randomize each instance
201-225. [Loop] obj_update(name="Tower_D{i}",
         rotation=[0,0,random], scale=[s,s,random_height])
```

### Phase 8: Street Lights Grid

```
226. light_create(type="POINT", name="Base_StreetLight", location=[0,0,6],
     energy=800, color=[1.0,0.85,0.7], radius=0.5)
227. obj_update(name="Base_StreetLight", visible=false)

228. col_create(name="StreetLights")

# Create street light grid along roads
229-280. [Loop] For every 15 units along X and Y axes:
    obj_duplicate(name="Base_StreetLight", new_name=f"SL_{i}",
                    offset=[x,y,0], linked=true)
    col_move_to(object=f"SL_{i}", collection="StreetLights")
```

### Phase 9: Neon Signs & Accent Lights

```
281. col_create(name="NeonLights")

# Colorful neon accents on commercial buildings
282. light_create(type="AREA", name="Neon_Red", location=[15,-65,3],
     energy=150, color=[1,0.1,0.2], size=2)
283. light_create(type="AREA", name="Neon_Blue", location=[-25,55,4],
     energy=150, color=[0.1,0.3,1], size=1.5)
284. light_create(type="AREA", name="Neon_Pink", location=[45,-30,3],
     energy=200, color=[1,0.2,0.6], size=2.5)
285. light_create(type="AREA", name="Neon_Cyan", location=[-55,25,3],
     energy=180, color=[0.2,0.8,1], size=2)
286. light_create(type="AREA", name="Neon_Yellow", location=[30,70,4],
     energy=120, color=[1,0.9,0.3], size=1.8)

# Move to collection
287-291. col_move_to(object="Neon_*", collection="NeonLights")

# Main city light (moon/ambient)
292. light_create(type="SUN", name="MoonLight", energy=0.5, color=[0.7,0.8,1.0])
293. obj_update(name="MoonLight", rotation=[0.8, 0.2, -0.5])
```

### Phase 10: Camera Setup

```
294. cam_create(name="Aerial_View", location=[150,150,100], lens=50)
295. cam_look_at(object="Aerial_View", target=[0,0,10])
296. cam_set_active(name="Aerial_View")

297. cam_create(name="Street_Level", location=[10,-90,3], lens=24)
298. cam_look_at(object="Street_Level", target=[0,0,15])

299. cam_create(name="Downtown_Close", location=[-30,-30,40], lens=35)
300. cam_look_at(object="Downtown_Close", target=[0,0,20])

301. cam_create(name="Flyover", location=[100,0,50], lens=35)
302. cam_look_at(object="Flyover", target=[0,0,0])

303. col_create(name="Cameras")
304-307. col_move_to for all cameras
```

### Phase 11: Animation - City Flyover

```
308. anim_set_frame(frame=1)
309. obj_update(name="Flyover", location=[120,120,80])
310. cam_look_at(object="Flyover", target=[0,0,15])
311. anim_set_keyframe(object="Flyover", property="location", frame=1)
312. anim_set_keyframe(object="Flyover", property="rotation_euler", frame=1)

313. anim_set_frame(frame=60)
314. obj_update(name="Flyover", location=[0,100,40])
315. cam_look_at(object="Flyover", target=[0,0,20])
316. anim_set_keyframe(object="Flyover", property="location", frame=60)
317. anim_set_keyframe(object="Flyover", property="rotation_euler", frame=60)

318. anim_set_frame(frame=120)
319. obj_update(name="Flyover", location=[-80,40,25])
320. cam_look_at(object="Flyover", target=[0,0,10])
321. anim_set_keyframe(object="Flyover", property="location", frame=120)
322. anim_set_keyframe(object="Flyover", property="rotation_euler", frame=120)

323. anim_set_frame(frame=180)
324. obj_update(name="Flyover", location=[-50,-80,15])
325. cam_look_at(object="Flyover", target=[20,0,25])
326. anim_set_keyframe(object="Flyover", property="location", frame=180)
327. anim_set_keyframe(object="Flyover", property="rotation_euler", frame=180)

328. anim_set_frame(frame=240)
329. obj_update(name="Flyover", location=[60,-60,60])
330. cam_look_at(object="Flyover", target=[0,0,15])
331. anim_set_keyframe(object="Flyover", property="location", frame=240)
332. anim_set_keyframe(object="Flyover", property="rotation_euler", frame=240)
```

### Phase 12: Props - Street Furniture

```
333. col_create(name="Props")

# Light poles (cylinders)
334. obj_create_primitive(type="cylinder", name="Base_Pole", location=[0,0,3], scale=[0.15,0.15,3])
335. mat_create(name="Metal_Dark", color=[0.15,0.15,0.18], metallic=1.0, roughness=0.5)
336. mat_assign(object="Base_Pole", material="Metal_Dark")
337. obj_update(name="Base_Pole", visible=false)

# Instance poles at street light positions
338-380. obj_duplicate(name="Base_Pole", new_name=f"Pole_{i}", linked=true)

# Benches / barriers (boxes)
381. obj_create_primitive(type="cube", name="Base_Barrier", location=[0,0,0.5], scale=[2,0.3,0.5])
382. mat_assign(object="Base_Barrier", material="Concrete_Dark")
383-400. scatter barriers around commercial areas
```

### Phase 13: Material Verification

```
401. mat_list - should show 10+ materials
402. mat_get_objects_by(materials=["Glass_Blue"]) - all skyscraper instances
403. mat_get_objects_by(materials=["Brick_Red", "Plaster_Beige"]) - residential
404. mat_get_object(name="Tower_D1") - verify linked material
```

### Phase 14: Rendering & Export

```
405. scene_screenshot(type="viewport") - quick preview

# Set render settings via util_execute_python
406. util_execute_python(code="""
import bpy
bpy.context.scene.render.resolution_x = 1920
bpy.context.scene.render.resolution_y = 1080
bpy.context.scene.render.film_transparent = False
bpy.context.scene.cycles.samples = 256
bpy.context.scene.cycles.use_denoising = True
""")

407. cam_set_active(name="Aerial_View")
408. scene_render(output="C:/temp/city_aerial.png", engine="CYCLES", samples=256)

409. cam_set_active(name="Street_Level")
410. scene_render(output="C:/temp/city_street.png", engine="CYCLES", samples=256)

411. cam_set_active(name="Downtown_Close")
412. scene_render(output="C:/temp/city_downtown.png", engine="CYCLES", samples=256)

413. scene_screenshot(type="render") - capture last render

414. file_save(path="C:/temp/city_district.blend")
415. file_export(path="C:/temp/city_district.glb", format="gltf")
```

### Phase 15: Final Verification

```
416. col_list - verify full hierarchy
417. obj_list - count all objects (should be 150+)
418. scene_get_info - final statistics
419. anim_get_keyframes(object="Flyover") - verify animation
420. anim_get_data(object="Flyover")
```

---

## Expected Results

### Building Count
| Zone | Base Type | Instances | Total |
|------|-----------|-----------|-------|
| Downtown | Skyscraper A/B | 25 | 25 |
| Business | Office A/B | 20 | 20 |
| Residential | Apartment A/B | 30 | 30 |
| Commercial | Shop A/B | 40 | 40 |
| **Total Buildings** | | | **115** |

### Other Objects
- Street lights: ~50
- Light poles: ~50
- Barriers/props: ~20
- Cameras: 4
- Ground: 1
- **Total: ~240 objects**

### Materials (10)
1. Asphalt (ground)
2. Glass_Blue (skyscrapers)
3. Glass_Gold (skyscrapers)
4. Concrete_Light (offices)
5. Concrete_Dark (offices, barriers)
6. Brick_Red (apartments)
7. Plaster_Beige (apartments)
8. Storefront (shops)
9. Metal_Dark (poles)
10. (emissive materials for windows - optional)

### Collections Hierarchy
```
Scene Collection
├── Base_Buildings (hidden)
│   ├── Base_Skyscraper_A
│   ├── Base_Skyscraper_B
│   ├── Base_Office_A
│   ├── Base_Office_B
│   ├── Base_Apartment_A
│   ├── Base_Apartment_B
│   ├── Base_Shop_A
│   └── Base_Shop_B
├── City_Instances
│   ├── Downtown (25 instances)
│   ├── Business_District (20 instances)
│   ├── Residential (30 instances)
│   └── Commercial (40 instances)
├── StreetLights (~50)
├── NeonLights (5)
├── Props (~70)
├── Cameras (4)
└── Environment
    └── Ground
```

### Files Generated
- city_aerial.png (1920x1080 render)
- city_street.png (street level view)
- city_downtown.png (downtown closeup)
- city_district.blend (full scene)
- city_district.glb (GLTF export)

---

## Tool Coverage

| Category | Tools Used |
|----------|------------|
| Scene | scene_get_info, obj_list |
| Objects | obj_create_primitive, obj_update, obj_delete, obj_duplicate (linked!) |
| Materials | mat_create, mat_assign, mat_update, mat_list, mat_get_object, mat_get_objects_by |
| Modifiers | mod_add |
| Collections | col_create, col_move_to, col_list |
| Lights | light_create, light_update |
| Cameras | cam_create, cam_set_active, cam_look_at |
| World | world_set_background |
| Animation | anim_set_frame, anim_set_keyframe, anim_get_keyframes, anim_get_data |
| I/O | scene_screenshot, scene_render, file_save, file_export |
| Utility | util_execute_python |
| Instancing | obj_duplicate(linked=true), obj_instance_to_points |

**Total: ~40 unique tools tested**

---

## Performance Notes

- Use `linked=true` for duplicates to share mesh data
- Hide base objects to avoid double rendering
- ~240 objects should render in reasonable time with Cycles
- For faster preview, use EEVEE engine
- Consider using geometry nodes for even more instances (via util_execute_python)

---

## Variations

### Daytime Version
```
world_set_hdri(path="hdri/city_day.hdr")
light_create(type="SUN", name="Sun", energy=5, color=[1,0.95,0.9])
# Reduce/disable neon lights
```

### Rain/Fog
```
util_execute_python(code="""
import bpy
bpy.context.scene.world.mist_settings.use_mist = True
bpy.context.scene.world.mist_settings.intensity = 0.5
bpy.context.scene.world.mist_settings.depth = 100
""")
phys_add_particles(name="Ground", count=50000)
```

### Traffic (animated boxes)
```
obj_create_primitive(type="cube", name="Car", scale=[2,1,0.8])
# Animate along path
con_add(name="Car", type="FOLLOW_PATH", target="Road_Curve")
```

### Building Lights (emissive windows)
```
mat_create(name="Window_Lit", color=[1,0.9,0.7], emission=[1,0.9,0.7], emission_strength=5)
# Apply to random window faces via util_execute_python
```
