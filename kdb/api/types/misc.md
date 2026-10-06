---
area: api-types
group: misc
version: 5.2.2
---

# Blender 5.2.2 RNA types: misc

## AOV

- `name` · string — Name of the AOV
- `is_valid` · boolean — Is the name of the AOV conflicting
- `type` · enum · {COLOR, VALUE} — Data type of the AOV

## Addon

Python add-ons to be loaded automatically

- `module` · string — Module name
- `preferences` · pointer · -> AddonPreferences · read-only

## AnimData

Animation data for data-block

- `nla_tracks` · collection · [] of NlaTracks · read-only — NLA Tracks (i.e. Animation Layers)
- `action` · pointer · -> Action — Active Action for this data-block
- `action_extrapolation` · enum · {NOTHING, HOLD, HOLD_FORWARD} — Action to take for gaps past the Active Action's range (when evaluating with NLA)
- `action_blend_type` · enum · {REPLACE, COMBINE, ADD, SUBTRACT, MULTIPLY} — Method used for combining Active Action's result with result of NLA stack
- `action_influence` · float — Amount the Active Action contributes to the result of the NLA stack
- `action_tweak_storage` · pointer · -> Action — Storage to temporarily hold the main action while in tweak mode
- `action_slot_handle_tweak_storage` · int — Storage to temporarily hold the main action slot while in tweak mode
- `drivers` · collection · [] of AnimDataDrivers · read-only — The Drivers/Expressions for this data-block
- `use_nla` · boolean — NLA stack is evaluated when evaluating this block
- `use_tweak_mode` · boolean — Whether to enable or disable tweak mode in NLA
- `use_pin` · boolean
- `action_slot_handle` · int — A number that identifies which sub-set of the Action is considered to be for this data-block
- `last_slot_identifier` · string — The identifier of the most recently assigned action slot. The slot identifies which sub-set of the Action is considered to be for this data-block, and its identifier is used to find the right slot when assigning an Action.
- `action_slot` · pointer · -> ActionSlot — The slot identifies which sub-set of the Action is considered to be for this data-block, and its name is used to find the right slot when assigning an Action
- `action_suitable_slots` · collection · read-only — The list of slots in this animation data-block

## AnimViz

Settings for the visualization of motion

- `motion_path` · pointer · -> AnimVizMotionPaths · read-only — Motion Path settings for visualization

## AnimVizMotionPaths

Motion Path settings for animation visualization

- `type` · enum · {CURRENT_FRAME, RANGE} — Type of range to show for Motion Paths
- `range` · enum · {KEYS_ALL, KEYS_SELECTED, SCENE, MANUAL} — Type of range to calculate for Motion Paths
- `bake_location` · enum · {HEADS, TAILS} — When calculating Bone Paths, use Head or Tips
- `show_frame_numbers` · boolean — Show frame numbers on Motion Paths
- `show_keyframe_highlight` · boolean — Emphasize position of keyframes on Motion Paths
- `show_keyframe_numbers` · boolean — Show frame numbers of Keyframes on Motion Paths
- `show_keyframe_action_all` · boolean — For bone motion paths, search whole Action for keyframes instead of in group with matching name only (is slower)
- `frame_step` · int — Number of frames between paths shown (not for 'On Keyframes' Onion-skinning method)
- `frame_start` · int — Starting frame of range of paths to display/calculate (not for 'Around Frame' Onion-skinning method)
- `frame_end` · int — End frame of range of paths to display/calculate (not for 'Around Frame' Onion-skinning method)
- `frame_before` · int — Number of frames to show before the current frame (only for 'Around Frame' Onion-skinning method)
- `frame_after` · int — Number of frames to show after the current frame (only for 'Around Frame' Onion-skinning method)
- `has_motion_paths` · boolean · read-only — Are there any bone paths that will need updating (read-only)
- `use_camera_space_bake` · boolean — Motion path points will be baked into the camera space of the active camera. This means they will only look right when looking through that camera. Switching cameras using markers is not supported.

## Annotation

Freehand annotation sketchbook

- `layers` · collection · [] of AnnotationLayers · read-only
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## AnnotationFrame

Collection of related sketches on a particular frame

- `strokes` · collection · [] of AnnotationStrokes · read-only — Freehand curves defining the sketch on this frame
- `frame_number` · int — The frame on which this sketch appears
- `select` · boolean — Frame is selected for editing in the Dope Sheet

## AnnotationStroke

Freehand curve defining part of a sketch

- `points` · collection · [] of AnnotationStrokePoints · read-only — Stroke data points
- `display_mode` · enum · {3DSPACE, 2DSPACE, 2DIMAGE} — Coordinate space that stroke is in

## AnnotationStrokePoint

Data point for freehand stroke curve

- `co` · float · [3]

## Area

Area in a subdivided screen, containing an editor

- `spaces` · collection · [] of AreaSpaces · read-only — Spaces contained in this area, the first being the active space (NOTE: Useful for example to restore a previously used 3D view space in a certain area to get the old view orientation)
- `regions` · collection · read-only — Regions this area is subdivided in
- `show_menus` · boolean — Show menus in the header
- `type` · enum · {EMPTY, VIEW_3D, IMAGE_EDITOR, NODE_EDITOR, SEQUENCE_EDITOR, CLIP_EDITOR, DOPESHEET_EDITOR, GRAPH_EDITOR, NLA_EDITOR, TEXT_EDITOR, CONSOLE, INFO, TOPBAR, STATUSBAR, OUTLINER, PROPERTIES, FILE_BROWSER, SPREADSHEET, PREFERENCES} — Current editor type for this area
- `ui_type` · enum · {} — Current editor type for this area
- `x` · int · read-only — The window relative vertical location of the area
- `y` · int · read-only — The window relative horizontal location of the area
- `width` · int · read-only — Area width
- `height` · int · read-only — Area height

## AssetLibrary

Container for asset catalogs and assets

- `type` · enum · {ALL, LOCAL, ESSENTIALS, ONLINE_ESSENTIALS, CUSTOM} · read-only
- `is_editable` · boolean · read-only — Assets and catalogs in this library can be edited from the current Blender instance

## AssetMetaData

Additional data stored for an asset data-block

- `author` · string — Name of the creator of the asset
- `description` · string — A description of the asset to be displayed for the user
- `copyright` · string — Copyright notice for this asset. An empty copyright notice does not necessarily indicate that this is copyright-free. Contact the author if any clarification is needed.
- `license` · string — The type of license this asset is distributed under. An empty license name does not necessarily indicate that this is free of licensing terms. Contact the author if any clarification is needed.
- `tags` · collection · [] of AssetTags · read-only — Custom tags (name tokens) for the asset, used for filtering and general asset management
- `active_tag` · int — Index of the tag set for editing
- `catalog_id` · string — Identifier for the asset's catalog, used by Blender to look up the asset's catalog path. Must be a UUID according to RFC4122.
- `catalog_simple_name` · string · read-only — Simple name of the asset's catalog, for debugging and data recovery purposes
- `use_preferred_import_method` · boolean — When "Follow Asset or Preferences" is selected for the import method in the Asset Browser, use the preferred import method of this asset
- `preferred_import_method` · enum · {LINK, APPEND, APPEND_REUSE, PACK}

## AssetRepresentation

Information about an entity that makes it possible for the asset system to deal with the entity as asset

- `name` · string · read-only
- `owner_asset_library` · pointer · -> AssetLibrary · read-only — The asset library containing this asset
- `metadata` · pointer · -> AssetMetaData · read-only — Additional information about the asset
- `id_type` · enum · {ACTION, ARMATURE, BRUSH, CACHEFILE, CAMERA, COLLECTION, CURVE, CURVES, FONT, GREASEPENCIL, GREASEPENCIL_V3, IMAGE, KEY, LATTICE, LIBRARY, LIGHT, LIGHT_PROBE, LINESTYLE, MASK, MATERIAL, MESH, META, MOVIECLIP, NODETREE, OBJECT, PAINTCURVE, PALETTE, PARTICLE, POINTCLOUD, SCENE, SCREEN, SOUND, SPEAKER, TEXT, TEXTURE, VOLUME, WINDOWMANAGER, WORKSPACE, WORLD} · read-only — The type of the data-block, if the asset represents one ('NONE' otherwise)
- `local_id` · pointer · -> ID · read-only — The local data-block this asset represents; only valid if that is a data-block in this file
- `full_library_path` · string · read-only — Absolute path to the .blend file containing this asset
- `full_path` · string · read-only — Absolute path to the .blend file containing this asset extended with the path of the asset inside the file
- `is_online` · boolean · read-only — True if this asset is accessed via internet, not stored on disk

## AssetShelf

Regions for quick access to assets

- `bl_idname` · string — If this is set, the asset gets a custom ID, otherwise it takes the name of the class used to define the asset (for example, if the class name is "OBJECT_AST_hello", and bl_idname is not set by the script, then bl_idname = "OBJECT_AST_hello")
- `bl_space_type` · enum · {EMPTY, VIEW_3D, IMAGE_EDITOR, NODE_EDITOR, SEQUENCE_EDITOR, CLIP_EDITOR, DOPESHEET_EDITOR, GRAPH_EDITOR, NLA_EDITOR, TEXT_EDITOR, CONSOLE, INFO, TOPBAR, STATUSBAR, OUTLINER, PROPERTIES, FILE_BROWSER, SPREADSHEET, PREFERENCES} — The space where the asset shelf will show up in. Ignored for popup asset shelves which can be displayed in any space.
- `bl_options` · enum · {NO_ASSET_DRAG, DEFAULT_VISIBLE, STORE_ENABLED_CATALOGS_IN_PREFERENCES, ACTIVATE_FOR_CONTEXT_MENU} — Options for this asset shelf type
- `bl_activate_operator` · string — Operator to call when activating an item with asset reference properties
- `bl_drag_operator` · string — Operator to call when dragging an item with asset reference properties
- `bl_default_preview_size` · int — Default size of the asset preview thumbnails in pixels
- `filter_action` · boolean — Show Action data-blocks
- `filter_armature` · boolean — Show Armature data-blocks
- `filter_brush` · boolean — Show Brushes data-blocks
- `filter_camera` · boolean — Show Camera data-blocks
- `filter_cachefile` · boolean — Show Cache File data-blocks
- `filter_curve` · boolean — Show Curve data-blocks
- `filter_annotations` · boolean — Show Annotation data-blocks
- `filter_grease_pencil` · boolean — Show Grease Pencil data-blocks
- `filter_group` · boolean — Show Collection data-blocks
- `filter_curves` · boolean — Show/hide Curves data-blocks
- `filter_image` · boolean — Show Image data-blocks
- `filter_light` · boolean — Show Light data-blocks
- `filter_light_probe` · boolean — Show Light Probe data-blocks
- `filter_linestyle` · boolean — Show Freestyle's Line Style data-blocks
- `filter_lattice` · boolean — Show Lattice data-blocks
- `filter_material` · boolean — Show Material data-blocks
- `filter_metaball` · boolean — Show Metaball data-blocks
- `filter_movie_clip` · boolean — Show Movie Clip data-blocks
- `filter_mesh` · boolean — Show Mesh data-blocks
- `filter_mask` · boolean — Show Mask data-blocks
- `filter_node_tree` · boolean — Show Node Tree data-blocks
- `filter_object` · boolean — Show Object data-blocks
- `filter_particle_settings` · boolean — Show Particle Settings data-blocks
- `filter_palette` · boolean — Show Palette data-blocks
- `filter_paint_curve` · boolean — Show Paint Curve data-blocks
- `filter_pointcloud` · boolean — Show/hide Point Cloud data-blocks
- `filter_scene` · boolean — Show Scene data-blocks
- `filter_speaker` · boolean — Show Speaker data-blocks
- `filter_sound` · boolean — Show Sound data-blocks
- `filter_texture` · boolean — Show Texture data-blocks
- `filter_text` · boolean — Show Text data-blocks
- `filter_font` · boolean — Show Font data-blocks
- `filter_volume` · boolean — Show/hide Volume data-blocks
- `filter_world` · boolean — Show World data-blocks
- `filter_work_space` · boolean — Show workspace data-blocks
- `asset_library_reference` · enum · {ALL, LOCAL, ESSENTIALS, ONLINE_ESSENTIALS, CUSTOM} — Choose the asset library to display assets from
- `show_names` · boolean — Show the asset name together with the preview. Otherwise only the preview will be visible.
- `preview_size` · int — Size of the asset preview thumbnails in pixels
- `search_filter` · string — Filter assets by name

## AssetTag

User defined tag (name token)

- `name` · string — The identifier that makes up this tag

## AssetWeakReference

Weak reference to some asset

- `asset_library_type` · enum · {ALL, LOCAL, ESSENTIALS, ONLINE_ESSENTIALS, CUSTOM} · read-only
- `asset_library_identifier` · string · read-only
- `relative_asset_identifier` · string · read-only

## Attribute

Geometry attribute

- `name` · string — Name of the Attribute
- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR} · read-only — Type of data stored in attribute
- `storage_type` · enum · {ARRAY, SINGLE} · read-only — Method used to store the data
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} · read-only — Domain of the Attribute
- `is_internal` · boolean · read-only — The attribute is meant for internal use by Blender
- `is_required` · boolean · read-only — Whether the attribute can be removed or renamed

## AttributeGroupPointCloud

Group of geometry attributes

- `active` · pointer · -> Attribute — Active attribute
- `active_index` · int — Active attribute index or -1 when none are active

## BakeSettings

Bake data for a Scene data-block

- `type` · enum · {NORMALS, DISPLACEMENT, VECTOR_DISPLACEMENT} — Choose shading information to bake into the image
- `cage_object` · pointer · -> Object — Object to use as cage instead of calculating the cage from the active object with cage extrusion
- `filepath` · string — Image filepath to use when saving externally
- `width` · int — Horizontal dimension of the baking map
- `height` · int — Vertical dimension of the baking map
- `margin` · int — Extends the baked result as a post process filter
- `margin_type` · enum · {ADJACENT_FACES, EXTEND} — Algorithm to extend the baked result
- `max_ray_distance` · float — The maximum ray distance for matching points between the active and selected objects. If zero, there is no limit.
- `cage_extrusion` · float — Inflate the active object by the specified distance for baking. This helps matching to points nearer to the outside of the selected object meshes.
- `normal_space` · enum · {OBJECT, TANGENT} — Choose normal space for baking
- `normal_r` · enum · {POS_X, POS_Y, POS_Z, NEG_X, NEG_Y, NEG_Z} — Axis to bake in red channel
- `normal_g` · enum · {POS_X, POS_Y, POS_Z, NEG_X, NEG_Y, NEG_Z} — Axis to bake in green channel
- `normal_b` · enum · {POS_X, POS_Y, POS_Z, NEG_X, NEG_Y, NEG_Z} — Axis to bake in blue channel
- `image_settings` · pointer · -> ImageFormatSettings · read-only
- `target` · enum · {IMAGE_TEXTURES, VERTEX_COLORS} — Where to output the baked map
- `save_mode` · enum · {INTERNAL, EXTERNAL} — Where to save baked image textures
- `view_from` · enum · {ABOVE_SURFACE, ACTIVE_CAMERA} — Source of reflection ray directions
- `use_selected_to_active` · boolean — Bake shading on the surface of selected objects to the active object
- `use_clear` · boolean — Clear Images before baking (internal only)
- `use_split_materials` · boolean — Split external images per material (external only)
- `use_automatic_name` · boolean — Automatically name the output file with the pass type (external only)
- `use_cage` · boolean — Cast rays to active object from a cage
- `use_pass_emit` · boolean — Add emission contribution
- `use_pass_direct` · boolean — Add direct lighting contribution
- `use_pass_indirect` · boolean — Add indirect lighting contribution
- `use_pass_color` · boolean — Color the pass
- `use_pass_diffuse` · boolean — Add diffuse contribution
- `use_pass_glossy` · boolean — Add glossy contribution
- `use_pass_transmission` · boolean — Add transmission contribution
- `pass_filter` · enum · {NONE, EMIT, DIRECT, INDIRECT, COLOR, DIFFUSE, GLOSSY, TRANSMISSION} · read-only — Passes to include in the active baking pass
- `use_multires` · boolean — Bake directly from multires object
- `use_lores_mesh` · boolean — Calculate heights against unsubdivided low resolution mesh
- `displacement_space` · enum · {OBJECT, TANGENT} — Choose displacement space for baking

## BezierSplinePoint

Bézier curve point with two handles

- `select_left_handle` · boolean — Handle 1 selection status
- `select_right_handle` · boolean — Handle 2 selection status
- `select_control_point` · boolean — Control point selection status
- `hide` · boolean — Visibility status
- `handle_left_type` · enum · {FREE, VECTOR, ALIGNED, AUTO} — Handle types
- `handle_right_type` · enum · {FREE, VECTOR, ALIGNED, AUTO} — Handle types
- `handle_left` · float · [3] — Coordinates of the first handle
- `co` · float · [3] — Coordinates of the control point
- `handle_right` · float · [3] — Coordinates of the second handle
- `tilt` · float — Tilt in 3D View
- `weight_softbody` · float — Softbody goal weight
- `radius` · float — Radius for beveling

## BlendData

Main data structure representing a .blend file and all its data-blocks

- `filepath` · string · read-only — Path to the .blend file
- `is_dirty` · boolean · read-only — Have recent edits been saved to disk
- `is_saved` · boolean · read-only — Has the current session been saved to disk as a .blend file
- `use_autopack` · boolean — Automatically pack all external data into .blend file
- `version` · int · [3] · read-only — File format version the .blend file was saved with
- `cameras` · collection · [] of BlendDataCameras · read-only — Camera data-blocks
- `scenes` · collection · [] of BlendDataScenes · read-only — Scene data-blocks
- `objects` · collection · [] of BlendDataObjects · read-only — Object data-blocks
- `materials` · collection · [] of BlendDataMaterials · read-only — Material data-blocks
- `node_groups` · collection · [] of BlendDataNodeTrees · read-only — Node group data-blocks
- `meshes` · collection · [] of BlendDataMeshes · read-only — Mesh data-blocks
- `lights` · collection · [] of BlendDataLights · read-only — Light data-blocks
- `libraries` · collection · [] of BlendDataLibraries · read-only — Library data-blocks
- `screens` · collection · [] of BlendDataScreens · read-only — Screen data-blocks
- `window_managers` · collection · [] of BlendDataWindowManagers · read-only — Window manager data-blocks
- `images` · collection · [] of BlendDataImages · read-only — Image data-blocks
- `lattices` · collection · [] of BlendDataLattices · read-only — Lattice data-blocks
- `curves` · collection · [] of BlendDataCurves · read-only — Curve data-blocks
- `metaballs` · collection · [] of BlendDataMetaBalls · read-only — Metaball data-blocks
- `fonts` · collection · [] of BlendDataFonts · read-only — Vector font data-blocks
- `textures` · collection · [] of BlendDataTextures · read-only — Texture data-blocks
- `brushes` · collection · [] of BlendDataBrushes · read-only — Brush data-blocks
- `worlds` · collection · [] of BlendDataWorlds · read-only — World data-blocks
- `collections` · collection · [] of BlendDataCollections · read-only — Collection data-blocks
- `shape_keys` · collection · read-only — Shape Key data-blocks
- `texts` · collection · [] of BlendDataTexts · read-only — Text data-blocks
- `speakers` · collection · [] of BlendDataSpeakers · read-only — Speaker data-blocks
- `sounds` · collection · [] of BlendDataSounds · read-only — Sound data-blocks
- `armatures` · collection · [] of BlendDataArmatures · read-only — Armature data-blocks
- `actions` · collection · [] of BlendDataActions · read-only — Action data-blocks
- `particles` · collection · [] of BlendDataParticles · read-only — Particle data-blocks
- `palettes` · collection · [] of BlendDataPalettes · read-only — Palette data-blocks
- `annotations` · collection · [] of BlendDataAnnotations · read-only — Annotation data-blocks (legacy Grease Pencil)
- `grease_pencils` · collection · [] of BlendDataGreasePencilsV3 · read-only — Grease Pencil data-blocks
- `movieclips` · collection · [] of BlendDataMovieClips · read-only — Movie Clip data-blocks
- `masks` · collection · [] of BlendDataMasks · read-only — Masks data-blocks
- `linestyles` · collection · [] of BlendDataLineStyles · read-only — Line Style data-blocks
- `cache_files` · collection · [] of BlendDataCacheFiles · read-only — Cache Files data-blocks
- `paint_curves` · collection · [] of BlendDataPaintCurves · read-only — Paint Curves data-blocks
- `workspaces` · collection · [] of BlendDataWorkSpaces · read-only — Workspace data-blocks
- `lightprobes` · collection · [] of BlendDataProbes · read-only — Light Probe data-blocks
- `hair_curves` · collection · [] of BlendDataHairCurves · read-only — Hair curve data-blocks
- `pointclouds` · collection · [] of BlendDataPointClouds · read-only — Point cloud data-blocks
- `volumes` · collection · [] of BlendDataVolumes · read-only — Volume data-blocks
- `colorspace` · pointer · -> BlendFileColorspace · read-only — Information about the color space used for data-blocks in a blend file
- `all_ids` · collection · read-only — Read-only list of all IDs listed in Blender data-base

## BlendImportContext

Contextual data for a blendfile library/linked-data related operation. Currently only exposed as read-only data for the pre/post blendimport handlers

- `import_items` · collection · [] of BlendImportContextItems · read-only
- `options` · enum · {LINK, MAKE_PATHS_RELATIVE, USE_PLACEHOLDERS, FORCE_INDIRECT, APPEND_SET_FAKEUSER, APPEND_RECURSIVE, APPEND_LOCAL_ID_REUSE, APPEND_ASSET_DATA_CLEAR, SELECT_OBJECTS, USE_ACTIVE_COLLECTION, OBDATA_INSTANCE, COLLECTION_INSTANCE} · read-only — Options for this blendfile import operation
- `process_stage` · enum · {INIT, DONE} · read-only — Current stage of the import process

## BlendImportContextItem

An item (representing a data-block) in a BlendImportContext data. Currently only exposed as read-only data for the pre/post linking handlers

- `name` · string · read-only — ID name of the item
- `id_type` · enum · {ACTION, ARMATURE, BRUSH, CACHEFILE, CAMERA, COLLECTION, CURVE, CURVES, FONT, GREASEPENCIL, GREASEPENCIL_V3, IMAGE, KEY, LATTICE, LIBRARY, LIGHT, LIGHT_PROBE, LINESTYLE, MASK, MATERIAL, MESH, META, MOVIECLIP, NODETREE, OBJECT, PAINTCURVE, PALETTE, PARTICLE, POINTCLOUD, SCENE, SCREEN, SOUND, SPEAKER, TEXT, TEXTURE, VOLUME, WINDOWMANAGER, WORKSPACE, WORLD} · read-only — ID type of the item
- `source_libraries` · collection · [] of BlendImportContextLibraries · read-only — List of libraries to search and import that ID from. The ID will be imported from the first file in that list that contains it
- `append_action` · enum · {UNSET, KEEP_LINKED, REUSE_LOCAL, MAKE_LOCAL, COPY_LOCAL} · read-only — How this item has been handled by the append operation. Only set if the data has been appended
- `import_info` · enum · {INDIRECT_USAGE, LIBOVERRIDE_DEPENDENCY, LIBOVERRIDE_DEPENDENCY_ONLY} · read-only — Various status info about an item after it has been imported
- `id` · pointer · -> ID · read-only — The imported ID. None until it has been linked or appended. May be the same as ``reusable_local_id`` when appended
- `source_library` · pointer · -> Library · read-only — Library ID representing the blendfile from which the ID was imported. None until the ID has been linked or appended
- `library_override_id` · pointer · -> ID · read-only — The library override of the linked ID. None until it has been created
- `reusable_local_id` · pointer · -> ID · read-only — The already existing local ID that may be reused in append & reuse case. None until it has been found

## BlendImportContextLibrary

Library (blendfile) reference in a BlendImportContext data. Currently only exposed as read-only data for the pre/post blendimport handlers

- `filepath` · string · read-only

## BlenderRNA

Blender RNA structure definitions

- `structs` · collection · read-only

## BoidRule

- `name` · string — Boid rule name
- `type` · enum · {GOAL, AVOID, AVOID_COLLISION, SEPARATE, FLOCK, FOLLOW_LEADER, AVERAGE_SPEED, FIGHT} · read-only
- `use_in_air` · boolean — Use rule when boid is flying
- `use_on_land` · boolean — Use rule when boid is on land

## BoidRuleAverageSpeed

- `wander` · float — How fast velocity's direction is randomized
- `level` · float — How much velocity's z-component is kept constant
- `speed` · float — Percentage of maximum speed

## BoidRuleAvoid

- `object` · pointer · -> Object — Object to avoid
- `use_predict` · boolean — Predict target movement
- `fear_factor` · float — Avoid object if danger from it is above this threshold

## BoidRuleAvoidCollision

- `use_avoid` · boolean — Avoid collision with other boids
- `use_avoid_collision` · boolean — Avoid collision with deflector objects
- `look_ahead` · float — Time to look ahead in seconds

## BoidRuleFight

- `distance` · float — Attack boids at max this distance
- `flee_distance` · float — Flee to this distance

## BoidRuleFollowLeader

- `object` · pointer · -> Object — Follow this object instead of a boid
- `distance` · float — Distance behind leader to follow
- `queue_count` · int — How many boids in a line
- `use_line` · boolean — Follow leader in a line

## BoidRuleGoal

- `object` · pointer · -> Object — Goal object
- `use_predict` · boolean — Predict target movement

## BoidSettings

Settings for boid physics

- `land_smooth` · float — How smoothly the boids land
- `bank` · float — Amount of rotation around velocity vector on turns
- `pitch` · float — Amount of rotation around side vector
- `height` · float — Boid height relative to particle size
- `states` · collection · read-only
- `active_boid_state` · pointer · -> BoidRule · read-only
- `active_boid_state_index` · int
- `health` · float — Initial boid health when born
- `strength` · float — Maximum caused damage on attack per second
- `aggression` · float — Boid will fight this times stronger enemy
- `accuracy` · float — Accuracy of attack
- `range` · float — Maximum distance from which a boid can attack
- `air_speed_min` · float — Minimum speed in air (relative to maximum speed)
- `air_speed_max` · float — Maximum speed in air
- `air_acc_max` · float — Maximum acceleration in air (relative to maximum speed)
- `air_ave_max` · float — Maximum angular velocity in air (relative to 180 degrees)
- `air_personal_space` · float — Radius of boids personal space in air (% of particle size)
- `land_jump_speed` · float — Maximum speed for jumping
- `land_speed_max` · float — Maximum speed on land
- `land_acc_max` · float — Maximum acceleration on land (relative to maximum speed)
- `land_ave_max` · float — Maximum angular velocity on land (relative to 180 degrees)
- `land_personal_space` · float — Radius of boids personal space on land (% of particle size)
- `land_stick_force` · float — How strong a force must be to start effecting a boid on land
- `use_flight` · boolean — Allow boids to move in air
- `use_land` · boolean — Allow boids to move on land
- `use_climb` · boolean — Allow boids to climb goal objects

## BoidState

Boid state for boid physics

- `name` · string — Boid state name
- `ruleset_type` · enum · {FUZZY, RANDOM, AVERAGE} — How the rules in the list are evaluated
- `rules` · collection · read-only
- `active_boid_rule` · pointer · -> BoidRule · read-only
- `active_boid_rule_index` · int
- `rule_fuzzy` · float
- `volume` · float
- `falloff` · float

## BoolAttribute

Geometry attribute that stores booleans

- `data` · collection · read-only

## BoolAttributeValue

Bool value in geometry attribute

- `value` · boolean

## BoolProperty

RNA boolean property definition

- `default` · boolean · read-only — Default value for this number
- `default_array` · boolean · read-only — Default value for this array
- `array_length` · int · read-only — Maximum length of the array, 0 means unlimited
- `array_dimensions` · int · [3] · read-only — Length of each dimension of the array
- `is_array` · boolean · read-only

## ByteColorAttribute

Geometry attribute that stores RGBA colors as positive integer values using 8-bits per channel

- `data` · collection · read-only

## ByteColorAttributeValue

Color value in geometry attribute

- `color` · float · [4] — RGBA color in scene linear color space
- `color_srgb` · float · [4] — RGBA color in sRGB color space

## ByteIntAttribute

Geometry attribute that stores 8-bit integers

- `data` · collection · read-only

## ByteIntAttributeValue

8-bit value in geometry attribute

- `value` · int

## CacheFile

- `filepath` · string — Path to external displacements file
- `is_sequence` · boolean — Whether the cache is separated in a series of files
- `override_frame` · boolean — Whether to use a custom frame for looking up data in the cache file, instead of using the current scene frame
- `frame` · float — The time to use for looking up the data in the cache file, or to determine which file to use in a file sequence
- `frame_offset` · float — Subtracted from the current frame to use for looking up the data in the cache file, or to determine which file to use in a file sequence
- `forward_axis` · enum · {POS_X, POS_Y, POS_Z, NEG_X, NEG_Y, NEG_Z}
- `up_axis` · enum · {POS_X, POS_Y, POS_Z, NEG_X, NEG_Y, NEG_Z}
- `scale` · float — Value by which to enlarge or shrink the object with respect to the world's origin (only applicable through a Transform Cache constraint)
- `object_paths` · collection · [] of CacheObjectPaths · read-only — Paths of the objects inside the Alembic archive
- `velocity_name` · string — Name of the Alembic attribute used for generating motion blur data
- `velocity_unit` · enum · {SECOND, FRAME} — Define how the velocity vectors are interpreted with regard to time, 'frame' means the delta time is 1 frame, 'second' means the delta time is 1 / FPS
- `layers` · collection · [] of CacheFileLayers · read-only — Layers of the cache
- `active_index` · int
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## CacheFileLayer

Layer of the cache, used to load or override data from the first the first layer

- `filepath` · string — Path to the archive
- `hide_layer` · boolean — Do not load data from this layer

## CacheFileLayers

Collection of cache layers

- `active` · pointer · -> CacheFileLayer — Active layer of the CacheFile

## Collection

Collection of Object data-blocks

- `instance_offset` · float · [3] — Offset from the origin to use when instancing
- `objects` · collection · [] of CollectionObjects · read-only — Objects that are directly in this collection
- `all_objects` · collection · read-only — Objects that are in this collection and its child collections
- `children` · collection · [] of CollectionChildren · read-only — Collections that are immediate children of this collection
- `collection_objects` · collection · read-only — Objects of the collection with their parent-collection-specific settings
- `collection_children` · collection · read-only — Children collections with their parent-collection-specific settings
- `exporters` · collection · [] of CollectionExports · read-only — Export Handlers configured for the collection
- `active_exporter_index` · int — Active index in the exporters list
- `hide_select` · boolean — Disable selection in viewport
- `hide_viewport` · boolean — Globally disable in viewports
- `hide_render` · boolean — Globally disable in renders
- `lineart_usage` · enum · {INCLUDE, OCCLUSION_ONLY, EXCLUDE, INTERSECTION_ONLY, NO_INTERSECTION, FORCE_INTERSECTION} — How to use this collection in Line Art calculation
- `lineart_use_intersection_mask` · boolean — Use custom intersection mask for faces in this collection
- `lineart_intersection_mask` · boolean — Intersection generated by this collection will have this mask value
- `lineart_intersection_priority` · int — The intersection line will be included into the object with the higher intersection priority value
- `use_lineart_intersection_priority` · boolean — Assign intersection priority value for this collection
- `color_tag` · enum · {NONE, COLOR_01, COLOR_02, COLOR_03, COLOR_04, COLOR_05, COLOR_06, COLOR_07, COLOR_08} — Color tag for a collection

## CollectionChild

Child collection with its collection related settings

- `light_linking` · pointer · -> CollectionLightLinking · read-only — Light linking settings of the collection object

## CollectionExport

- `name` · string
- `is_open` · boolean — Whether the panel is expanded or closed
- `export_properties` · pointer · -> PropertyGroup · read-only — Properties associated with the configured exporter
- `filepath` · string — The file path used for exporting

## CollectionImport

Importer configured for the collection

- `is_open` · boolean — Whether the panel is expanded or closed
- `import_properties` · pointer · -> PropertyGroup · read-only — Properties associated with the configured importer

## CollectionProperty

RNA collection property to define lists, arrays and mappings

- `fixed_type` · pointer · -> Struct · read-only — Fixed pointer type, empty if variable type

## CollisionSettings

Collision settings for object in physics simulation

- `use` · boolean — Enable this object as a collider for physics systems
- `damping_factor` · float — Amount of damping during particle collision
- `damping_random` · float — Random variation of damping
- `friction_factor` · float — Amount of friction during particle collision
- `friction_random` · float — Random variation of friction
- `permeability` · float — Chance that the particle will pass through the mesh
- `use_particle_kill` · boolean — Kill collided particles
- `stickiness` · float — Amount of stickiness to surface collision
- `thickness_inner` · float — Inner face thickness (only used by softbodies)
- `thickness_outer` · float — Outer face thickness
- `damping` · float — Amount of damping during collision
- `absorption` · float — How much of effector force gets lost during collision with this object (in percent)
- `cloth_friction` · float — Friction for cloth collisions
- `use_culling` · boolean — Cloth collision acts with respect to the collider normals (improves penetration recovery)
- `use_normal` · boolean — Cloth collision impulses act in the direction of the collider normals (more reliable in some cases)

## ColorManagedDisplaySettings

Color management specific to display device

- `display_device` · enum · {NONE} — Display name. For viewing, this is the display device that will be emulated by limiting the gamut and HDR colors. For image and video output, this is the display space used for writing.
- `emulation` · enum · {OFF, AUTO} — Control how images in the chosen display are mapped to the physical display

## ColorMapping

Color mapping settings

- `use_color_ramp` · boolean — Toggle color ramp operations
- `color_ramp` · pointer · -> ColorRamp · read-only
- `brightness` · float — Adjust the brightness of the texture
- `contrast` · float — Adjust the contrast of the texture
- `saturation` · float — Adjust the saturation of colors in the texture
- `blend_type` · enum · {MIX, DARKEN, MULTIPLY, LIGHTEN, SCREEN, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE} — Mode used to mix with texture output color
- `blend_color` · float · [3] — Blend color to mix with texture output color
- `blend_factor` · float

## ColorRamp

Color ramp mapping a scalar value to a color

- `elements` · collection · [] of ColorRampElements · read-only
- `interpolation` · enum · {EASE, CARDINAL, LINEAR, B_SPLINE, CONSTANT} — Set interpolation between color stops
- `hue_interpolation` · enum · {NEAR, FAR, CW, CCW} — Set color interpolation
- `color_mode` · enum · {RGB, HSV, HSL} — Set color mode to use for interpolation

## ColorRampElement

Element defining a color at a position in the color ramp

- `color` · float · [4] — Set color of selected color stop
- `alpha` · float — Set alpha of selected color stop
- `position` · float — Set position of selected color stop

## ConsoleLine

Input line for the interactive console

- `body` · string — Text in the line
- `current_character` · int
- `type` · enum · {OUTPUT, INPUT, INFO, ERROR} — Console line type when used in scrollback

## Context

Current windowmanager and data context

- `window_manager` · pointer · -> WindowManager · read-only
- `window` · pointer · -> Window · read-only
- `workspace` · pointer · -> WorkSpace · read-only
- `screen` · pointer · -> Screen · read-only
- `area` · pointer · -> Area · read-only
- `space_data` · pointer · -> Space · read-only — The current space, may be None in background-mode, when the cursor is outside the window or when using menu-search
- `region` · pointer · -> Region · read-only
- `region_popup` · pointer · -> Region · read-only — The temporary region for pop-ups (including menus and pop-overs)
- `region_data` · pointer · -> RegionView3D · read-only
- `gizmo_group` · pointer · -> GizmoGroup · read-only
- `asset` · pointer · -> AssetRepresentation · read-only
- `blend_data` · pointer · -> BlendData · read-only
- `scene` · pointer · -> Scene · read-only
- `view_layer` · pointer · -> ViewLayer · read-only
- `engine` · string · read-only
- `collection` · pointer · -> Collection · read-only
- `layer_collection` · pointer · -> LayerCollection · read-only
- `tool_settings` · pointer · -> ToolSettings · read-only
- `preferences` · pointer · -> Preferences · read-only
- `mode` · enum · {EDIT_MESH, EDIT_CURVE, EDIT_CURVES, EDIT_SURFACE, EDIT_TEXT, EDIT_ARMATURE, EDIT_METABALL, EDIT_LATTICE, EDIT_GREASE_PENCIL, EDIT_POINTCLOUD, POSE, SCULPT, PAINT_WEIGHT, PAINT_VERTEX, PAINT_TEXTURE, PARTICLE, OBJECT, PAINT_GPENCIL, EDIT_GPENCIL, SCULPT_GPENCIL, WEIGHT_GPENCIL, VERTEX_GPENCIL, SCULPT_CURVES, PAINT_GREASE_PENCIL, SCULPT_GREASE_PENCIL, WEIGHT_GREASE_PENCIL, VERTEX_GREASE_PENCIL} · read-only

## CryptomatteEntry

- `encoded_hash` · float · read-only
- `name` · string · read-only

## Depsgraph

- `mode` · enum · {VIEWPORT, RENDER} · read-only — Evaluation mode
- `scene` · pointer · -> Scene · read-only — Original scene dependency graph is built for
- `view_layer` · pointer · -> ViewLayer · read-only — Original view layer dependency graph is built for
- `scene_eval` · pointer · -> Scene · read-only — Scene at its evaluated state
- `view_layer_eval` · pointer · -> ViewLayer · read-only — View layer at its evaluated state
- `ids` · collection · read-only — All evaluated data-blocks
- `objects` · collection · read-only — Evaluated objects in the dependency graph
- `object_instances` · collection · read-only — All object instances to display or render (Warning: Only use this as an iterator, never as a sequence, and do not keep any references to its items)
- `updates` · collection · read-only — Updates to data-blocks

## DepsgraphUpdate

Information about ID that was updated

- `id` · pointer · -> ID · read-only — Updated data-block
- `is_updated_transform` · boolean · read-only — Object transformation is updated
- `is_updated_geometry` · boolean · read-only — Object geometry is updated
- `is_updated_shading` · boolean · read-only — Object shading is updated

## DisplaySafeAreas

Safe areas used in 3D view and the sequencer

- `title` · float · [2] — Safe area for text and graphics
- `action` · float · [2] — Safe area for general elements
- `title_center` · float · [2] — Safe area for text and graphics in a different aspect ratio
- `action_center` · float · [2] — Safe area for general elements in a different aspect ratio

## DopeSheet

Settings for filtering the channels shown in animation editors

- `source` · pointer · -> ID · read-only — ID-Block representing source data, usually ID_SCE (i.e. Scene)
- `show_datablock_filters` · boolean — Show options for whether channels related to certain types of data are included
- `show_only_selected` · boolean — Only include channels relating to selected objects and data
- `show_only_slot_of_active_object` · boolean — Only show the slot of the active Object. Otherwise show all the Action's Slots
- `show_hidden` · boolean — Include channels from objects/bone that are not visible
- `use_datablock_sort` · boolean — Alphabetically sorts data-blocks - mainly objects in the scene (disable to increase viewport speed)
- `use_filter_invert` · boolean — Invert filter search
- `show_only_errors` · boolean — Only include F-Curves and drivers that are disabled or have errors
- `filter_collection` · pointer · -> Collection — Collection that included object should be a member of
- `filter_fcurve_name` · string — F-Curve live filtering string
- `filter_text` · string — Live filtering string
- `use_multi_word_filter` · boolean — Perform fuzzy/multi-word matching.
Warning: May be slow
- `show_missing_nla` · boolean — Include animation data-blocks with no NLA data (NLA editor only)
- `show_summary` · boolean — Display an additional 'summary' line (Dope Sheet editors only)
- `show_expanded_summary` · boolean — Collapse summary when shown, so all other channels get hidden (Dope Sheet editors only)
- `show_transforms` · boolean — Include visualization of object-level animation data (mostly transforms)
- `show_shapekeys` · boolean — Include visualization of shape key related animation data
- `show_modifiers` · boolean — Include visualization of animation data related to data-blocks linked to modifiers
- `show_meshes` · boolean — Include visualization of mesh related animation data
- `show_lattices` · boolean — Include visualization of lattice related animation data
- `show_cameras` · boolean — Include visualization of camera related animation data
- `show_materials` · boolean — Include visualization of material related animation data
- `show_lights` · boolean — Include visualization of light related animation data
- `show_linestyles` · boolean — Include visualization of Line Style related Animation data
- `show_textures` · boolean — Include visualization of texture related animation data
- `show_curves` · boolean — Include visualization of curve related animation data
- `show_worlds` · boolean — Include visualization of world related animation data
- `show_scenes` · boolean — Include visualization of scene related animation data
- `show_particles` · boolean — Include visualization of particle related animation data
- `show_metaballs` · boolean — Include visualization of metaball related animation data
- `show_armatures` · boolean — Include visualization of armature related animation data
- `show_nodes` · boolean — Include visualization of node related animation data
- `show_speakers` · boolean — Include visualization of speaker related animation data
- `show_cache_files` · boolean — Include visualization of cache file related animation data
- `show_hair_curves` · boolean — Include visualization of hair related animation data
- `show_pointclouds` · boolean — Include visualization of point cloud related animation data
- `show_volumes` · boolean — Include visualization of volume related animation data
- `show_lightprobes` · boolean — Include visualization of light probe related animation data
- `show_gpencil` · boolean — Include visualization of Grease Pencil related animation data and frames
- `show_movieclips` · boolean — Include visualization of movie clip related animation data
- `show_driver_fallback_as_error` · boolean — Include drivers that relied on any fallback values for their evaluation in the Only Show Errors filter, even if the driver evaluation succeeded

## Driver

Driver for the value of a setting based on an external value

- `type` · enum · {AVERAGE, SUM, SCRIPTED, MIN, MAX} — Driver type
- `expression` · string — Expression to use for Scripted Expression
- `variables` · collection · [] of ChannelDriverVariables · read-only — Properties acting as inputs for this driver
- `use_self` · boolean — Include a 'self' variable in the name-space, so drivers can easily reference the data being modified (object, bone, etc...)
- `is_valid` · boolean — Driver could not be evaluated in past, so should be skipped
- `is_simple_expression` · boolean · read-only — The scripted expression can be evaluated without using the full Python interpreter

## DriverTarget

Source of input values for driver variables

- `id` · pointer · -> ID — ID-block that the specific property used can be found from (id_type property must be set first)
- `id_type` · enum · {ACTION, ARMATURE, BRUSH, CACHEFILE, CAMERA, COLLECTION, CURVE, CURVES, FONT, GREASEPENCIL, GREASEPENCIL_V3, IMAGE, KEY, LATTICE, LIBRARY, LIGHT, LIGHT_PROBE, LINESTYLE, MASK, MATERIAL, MESH, META, MOVIECLIP, NODETREE, OBJECT, PAINTCURVE, PALETTE, PARTICLE, POINTCLOUD, SCENE, SCREEN, SOUND, SPEAKER, TEXT, TEXTURE, VOLUME, WINDOWMANAGER, WORKSPACE, WORLD} — Type of ID-block that can be used
- `data_path` · string — RNA Path (from ID-block) to property used
- `bone_target` · string — Name of PoseBone to use as target
- `transform_type` · enum · {LOC_X, LOC_Y, LOC_Z, ROT_X, ROT_Y, ROT_Z, ROT_W, SCALE_X, SCALE_Y, SCALE_Z, SCALE_AVG} — Driver variable type
- `rotation_mode` · enum · {AUTO, XYZ, XZY, YXZ, YZX, ZXY, ZYX, QUATERNION, SWING_TWIST_X, SWING_TWIST_Y, SWING_TWIST_Z} — Mode for calculating rotation channel values
- `transform_space` · enum · {WORLD_SPACE, TRANSFORM_SPACE, LOCAL_SPACE} — Space in which transforms are used
- `context_property` · enum · {ACTIVE_SCENE, ACTIVE_VIEW_LAYER} — Type of a context-dependent data-block to access property from
- `use_fallback_value` · boolean — Use the fallback value if the data path cannot be resolved, instead of failing to evaluate the driver
- `fallback_value` · float — The value to use if the data path cannot be resolved
- `is_fallback_used` · boolean · read-only — Indicates that the most recent variable evaluation used the fallback value

## DriverVariable

Variable from some source/target for driver relationship

- `name` · string — Name to use in scripted expressions/functions (no spaces or dots are allowed, and must start with a letter)
- `type` · enum · {SINGLE_PROP, TRANSFORMS, ROTATION_DIFF, LOC_DIFF, CONTEXT_PROP} — Driver variable type
- `targets` · collection · read-only — Sources of input data for evaluating this variable
- `is_name_valid` · boolean · read-only — Is this a valid name for a driver variable

## DynamicPaintCanvasSettings

Dynamic Paint canvas settings

- `canvas_surfaces` · collection · [] of DynamicPaintSurfaces · read-only — Paint surface list

## DynamicPaintSurface

A canvas surface layer

- `surface_format` · enum · {VERTEX, IMAGE} — Surface Format
- `surface_type` · enum · {PAINT} — Surface Type
- `is_active` · boolean — Toggle whether surface is processed or ignored
- `name` · string — Surface name
- `brush_collection` · pointer · -> Collection — Only use brush objects from this collection
- `use_dissolve` · boolean — Enable to make surface changes disappear over time
- `dissolve_speed` · int — Approximately in how many frames should dissolve happen
- `use_drying` · boolean — Enable to make surface wetness dry over time
- `dry_speed` · int — Approximately in how many frames should drying happen
- `image_resolution` · int — Output image resolution
- `uv_layer` · string — UV map name
- `frame_start` · int — Simulation start frame
- `frame_end` · int — Simulation end frame
- `frame_substeps` · int — Do extra frames between scene frames to ensure smooth motion
- `use_antialiasing` · boolean — Use 5× multisampling to smooth paint edges
- `brush_influence_scale` · float — Adjust influence brush objects have on this surface
- `brush_radius_scale` · float — Adjust radius of proximity brushes or particles for this surface
- `init_color_type` · enum · {NONE, COLOR, TEXTURE, VERTEX_COLOR}
- `init_color` · float · [4] — Initial color of the surface
- `init_texture` · pointer · -> Texture
- `init_layername` · string
- `effect_ui` · enum · {SPREAD, DRIP, SHRINK}
- `use_dry_log` · boolean — Use logarithmic drying (makes high values to dry faster than low values)
- `use_dissolve_log` · boolean — Use logarithmic dissolve (makes high values to fade faster than low values)
- `use_spread` · boolean — Process spread effect (spread wet paint around surface)
- `spread_speed` · float — How fast spread effect moves on the canvas surface
- `color_dry_threshold` · float — The wetness level when colors start to shift to the background
- `color_spread_speed` · float — How fast colors get mixed within wet paint
- `use_drip` · boolean — Process drip effect (drip wet paint to gravity direction)
- `use_shrink` · boolean — Process shrink effect (shrink paint areas)
- `shrink_speed` · float — How fast shrink effect moves on the canvas surface
- `effector_weights` · pointer · -> EffectorWeights · read-only
- `drip_velocity` · float — How much surface velocity affects dripping
- `drip_acceleration` · float — How much surface acceleration affects dripping
- `use_premultiply` · boolean — Multiply color by alpha (recommended for Blender input)
- `image_output_path` · string — Directory to save the textures
- `output_name_a` · string — Name used to save output from this surface
- `use_output_a` · boolean — Save this output layer
- `output_name_b` · string — Name used to save output from this surface
- `use_output_b` · boolean — Save this output layer
- `depth_clamp` · float — Maximum level of depth intersection in object space (use 0.0 to disable)
- `displace_factor` · float — Strength of displace when applied to the mesh
- `image_fileformat` · enum · {PNG, OPENEXR}
- `displace_type` · enum · {DISPLACE, DEPTH}
- `use_incremental_displace` · boolean — New displace is added cumulatively on top of existing
- `wave_damping` · float — Wave damping factor
- `wave_speed` · float — Wave propagation speed
- `wave_timescale` · float — Wave time scaling factor
- `wave_spring` · float — Spring force that pulls water level back to zero
- `wave_smoothness` · float — Limit maximum steepness of wave slope between simulation points (use higher values for smoother waves at expense of reduced detail)
- `use_wave_open_border` · boolean — Pass waves through mesh edges
- `point_cache` · pointer · -> PointCache · read-only
- `is_cache_user` · boolean · read-only

## DynamicPaintSurfaces

Collection of Dynamic Paint Canvas surfaces

- `active_index` · int
- `active` · pointer · -> DynamicPaintSurface · read-only — Active Dynamic Paint surface being displayed

## EffectorWeights

Effector weights for physics simulation

- `apply_to_hair_growing` · boolean — Use force fields when growing hair
- `collection` · pointer · -> Collection — Limit effectors to this collection
- `gravity` · float — Global gravity weight
- `all` · float — All effector's weight
- `force` · float — Force effector weight
- `vortex` · float — Vortex effector weight
- `magnetic` · float — Magnetic effector weight
- `wind` · float — Wind effector weight
- `curve_guide` · float — Curve guide effector weight
- `texture` · float — Texture effector weight
- `harmonic` · float — Harmonic effector weight
- `charge` · float — Charge effector weight
- `lennardjones` · float — Lennard-Jones effector weight
- `boid` · float — Boid effector weight
- `turbulence` · float — Turbulence effector weight
- `drag` · float — Drag effector weight
- `smokeflow` · float — Fluid Flow effector weight

## EnumProperty

RNA enumeration property definition, to choose from a number of predefined options

- `default` · enum · {DEFAULT} · read-only — Default value for this enum
- `default_flag` · enum · {DEFAULT} · read-only — Default value for this enum
- `enum_items` · collection · read-only — Possible values for the property
- `enum_items_static` · collection · read-only — Possible values for the property (never calls optional dynamic generation of those)
- `enum_items_static_ui` · collection · read-only — Possible values for the property (never calls optional dynamic generation of those). Includes UI elements (separators and section headings).

## EnumPropertyItem

Definition of a choice in an RNA enum property

- `name` · string · read-only — Human readable name
- `description` · string · read-only — Description of the item's purpose
- `identifier` · string · read-only — Unique name used in the code and scripting
- `value` · int · read-only — Value of the item
- `icon` · enum · {NONE, CHAR_NOTDEF, CHAR_REPLACEMENT, NOT_FOUND, BLANK1, AUTOMERGE_OFF, AUTOMERGE_ON, CHECKBOX_DEHLT, CHECKBOX_HLT, CLIPUV_DEHLT, CLIPUV_HLT, DECORATE_UNLOCKED, DECORATE_LOCKED, FAKE_USER_OFF, FAKE_USER_ON, HIDE_ON, HIDE_OFF, INDIRECT_ONLY_OFF, INDIRECT_ONLY_ON, ONIONSKIN_OFF, ONIONSKIN_ON, UNPINNED, PINNED, RADIOBUT_OFF, RADIOBUT_ON, RECORD_OFF, RECORD_ON, RESTRICT_RENDER_ON, RESTRICT_RENDER_OFF, RESTRICT_SELECT_ON, RESTRICT_SELECT_OFF, RESTRICT_VIEW_ON, RESTRICT_VIEW_OFF, RIGHTARROW, DOWNARROW_HLT, SELECT_INTERSECT, SELECT_DIFFERENCE, SNAP_OFF, SNAP_ON, PLAYHEAD_SNAP_OFF, ... (1033 total)} · read-only — Icon of the item

## Event

Window Manager Event

- `ascii` · string · read-only — Single ASCII character for this event
- `unicode` · string · read-only — Single unicode character for this event
- `value` · enum · {ANY, PRESS, RELEASE, CLICK, DOUBLE_CLICK, CLICK_DRAG, NOTHING} · read-only — The type of event, only applies to some
- `value_prev` · enum · {ANY, PRESS, RELEASE, CLICK, DOUBLE_CLICK, CLICK_DRAG, NOTHING} · read-only — The type of event, only applies to some
- `type` · enum · {NONE, LEFTMOUSE, MIDDLEMOUSE, RIGHTMOUSE, BUTTON4MOUSE, BUTTON5MOUSE, BUTTON6MOUSE, BUTTON7MOUSE, PEN, ERASER, MOUSEMOVE, INBETWEEN_MOUSEMOVE, TRACKPADPAN, TRACKPADZOOM, MOUSEROTATE, MOUSESMARTZOOM, WHEELUPMOUSE, WHEELDOWNMOUSE, WHEELINMOUSE, WHEELOUTMOUSE, WHEELLEFTMOUSE, WHEELRIGHTMOUSE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, ... (196 total)} · read-only
- `type_prev` · enum · {NONE, LEFTMOUSE, MIDDLEMOUSE, RIGHTMOUSE, BUTTON4MOUSE, BUTTON5MOUSE, BUTTON6MOUSE, BUTTON7MOUSE, PEN, ERASER, MOUSEMOVE, INBETWEEN_MOUSEMOVE, TRACKPADPAN, TRACKPADZOOM, MOUSEROTATE, MOUSESMARTZOOM, WHEELUPMOUSE, WHEELDOWNMOUSE, WHEELINMOUSE, WHEELOUTMOUSE, WHEELLEFTMOUSE, WHEELRIGHTMOUSE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, ... (196 total)} · read-only
- `direction` · enum · {ANY, NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST} · read-only — The direction (only applies to drag events)
- `is_repeat` · boolean · read-only — The event is generated by holding a key down
- `is_consecutive` · boolean · read-only — Part of a trackpad or NDOF motion, interrupted by cursor motion, button or key press events
- `mouse_x` · int · read-only — The window relative horizontal location of the mouse
- `mouse_y` · int · read-only — The window relative vertical location of the mouse
- `mouse_region_x` · int · read-only — The region relative horizontal location of the mouse
- `mouse_region_y` · int · read-only — The region relative vertical location of the mouse
- `mouse_prev_x` · int · read-only — The window relative horizontal location of the mouse
- `mouse_prev_y` · int · read-only — The window relative vertical location of the mouse
- `mouse_prev_press_x` · int · read-only — The window relative horizontal location of the last press event
- `mouse_prev_press_y` · int · read-only — The window relative vertical location of the last press event
- `pressure` · float · read-only — The pressure of the tablet or 1.0 if no tablet present
- `tilt` · float · [2] · read-only — The pressure of the tablet or zeroes if no tablet present
- `is_tablet` · boolean · read-only — The event has tablet data
- `is_mouse_absolute` · boolean · read-only — The last motion event was an absolute input
- `ndof_motion` · pointer · -> NDOFMotionEventData · read-only — NDOF motion event data
- `xr` · pointer · -> XrEventData · read-only — XR event data
- `shift` · boolean · read-only — True when the Shift key is held
- `ctrl` · boolean · read-only — True when the Ctrl key is held
- `alt` · boolean · read-only — True when the Alt/Option key is held
- `oskey` · boolean · read-only — True when the Cmd key is held
- `hyper` · boolean · read-only — True when the Hyper key is held

## FFmpegSettings

FFmpeg related settings for the scene

- `format` · enum · {MPEG4, MKV, WEBM, AVI, DV, FLASH, MPEG1, MPEG2, OGG, QUICKTIME} — Output file container
- `codec` · enum · {NONE, AV1, H264, H265, WEBM, DNXHD, DV, FFV1, FLASH, HUFFYUV, MPEG1, MPEG2, MPEG4, PNG, PRORES, QTRLE, THEORA} — FFmpeg codec to use for video output
- `video_bitrate` · int — Video bitrate (kbit/s)
- `minrate` · int — Rate control: min rate (kbit/s)
- `maxrate` · int — Rate control: max rate (kbit/s)
- `muxrate` · int — Mux rate (bits/second)
- `gopsize` · int — Distance between key frames, also known as GOP size; influences file size and seekability
- `max_b_frames` · int — Maximum number of B-frames between non-B-frames; influences file size and seekability
- `use_max_b_frames` · boolean — Set a maximum number of B-frames
- `buffersize` · int — Rate control: buffer size (kb)
- `packetsize` · int — Mux packet size (byte)
- `constant_rate_factor` · enum · {NONE, LOSSLESS, PERC_LOSSLESS, HIGH, MEDIUM, LOW, VERYLOW, LOWEST, CUSTOM} — Constant Rate Factor (CRF); tradeoff between video quality and file size
- `custom_constant_rate_factor` · int — A smaller Constant Rate Factor (CRF) results in better video quality but larger file size. The range of allowed CRF values is dependent on the codec.
- `ffmpeg_preset` · enum · {BEST, GOOD, REALTIME} — Tradeoff between encoding speed and compression ratio
- `ffmpeg_prores_profile` · enum · {422_PROXY, 422_LT, 422_STD, 422_HQ, 4444, 4444_XQ} — ProRes Profile
- `use_autosplit` · boolean — Autosplit output at 2GB boundary
- `use_lossless_output` · boolean — Use lossless output for video streams
- `audio_codec` · enum · {NONE, AAC, AC3, FLAC, MP2, MP3, OPUS, PCM, VORBIS} — FFmpeg audio codec to use
- `audio_bitrate` · int — Audio bitrate (kb/s)
- `audio_volume` · float — Audio volume
- `audio_mixrate` · int — Audio sample rate (samples/s)
- `audio_channels` · enum · {MONO, STEREO, SURROUND4, SURROUND51, SURROUND71} — Audio channel count

## FieldSettings

Field settings for an object in physics simulation

- `type` · enum · {NONE, BOID, CHARGE, GUIDE, DRAG, FLUID_FLOW, FORCE, HARMONIC, LENNARDJ, MAGNET, TEXTURE, TURBULENCE, VORTEX, WIND} — Type of field
- `shape` · enum · {POINT, LINE, PLANE, SURFACE, POINTS} — Which direction is used to calculate the effector force
- `falloff_type` · enum · {CONE, SPHERE, TUBE}
- `texture_mode` · enum · {CURL, GRADIENT, RGB} — How the texture effect is calculated (RGB and Curl need a RGB texture, else Gradient will be used instead)
- `z_direction` · enum · {POSITIVE, NEGATIVE, BOTH} — Effect in full or only positive/negative Z direction
- `strength` · float — Strength of force field
- `linear_drag` · float — Drag component proportional to velocity
- `harmonic_damping` · float — Damping of the harmonic force
- `quadratic_drag` · float — Drag component proportional to the square of velocity
- `flow` · float — Convert effector force into air flow velocity
- `wind_factor` · float — How much the force is reduced when acting parallel to a surface, e.g. cloth
- `inflow` · float — Inwards component of the vortex force
- `size` · float — Size of the turbulence
- `rest_length` · float — Rest length of the harmonic force
- `falloff_power` · float — How quickly strength falls off with distance from the force field
- `distance_min` · float — Minimum distance for the field's falloff
- `distance_max` · float — Maximum distance for the field to work
- `radial_min` · float — Minimum radial distance for the field's falloff
- `radial_max` · float — Maximum radial distance for the field to work
- `radial_falloff` · float — Radial falloff power (real gravitational falloff = 2)
- `texture_nabla` · float — Defines size of derivative offset used for calculating gradient and curl
- `noise` · float — Amount of noise for the force strength
- `seed` · int — Seed of the noise
- `use_min_distance` · boolean — Use a minimum distance for the field's falloff
- `use_max_distance` · boolean — Use a maximum distance for the field to work
- `use_radial_min` · boolean — Use a minimum radial distance for the field's falloff
- `use_radial_max` · boolean — Use a maximum radial distance for the field to work
- `use_object_coords` · boolean — Use object/global coordinates for texture
- `use_global_coords` · boolean — Use effector/global coordinates for turbulence
- `use_2d_force` · boolean — Apply force only in 2D
- `use_root_coords` · boolean — Texture coordinates from root particle locations
- `apply_to_location` · boolean — Affect particle's location
- `apply_to_rotation` · boolean — Affect particle's dynamic rotation
- `use_absorption` · boolean — Force gets absorbed by collision objects
- `use_multiple_springs` · boolean — Every point is affected by multiple springs
- `use_smoke_density` · boolean — Adjust force strength based on smoke density
- `use_gravity_falloff` · boolean — Multiply force by 1/distance²
- `texture` · pointer · -> Texture — Texture to use as force
- `source_object` · pointer · -> Object — Select domain object of the smoke simulation
- `guide_minimum` · float — The distance from which particles are affected fully
- `guide_free` · float — Guide-free time from particle life's end
- `use_guide_path_add` · boolean — Based on distance/falloff it adds a portion of the entire path
- `use_guide_path_weight` · boolean — Use curve weights to influence the particle influence along the curve
- `guide_clump_amount` · float — Amount of clumping
- `guide_clump_shape` · float — Shape of clumping
- `guide_kink_type` · enum · {NONE, BRAID, CURL, RADIAL, ROLL, ROTATION, WAVE} — Type of periodic offset on the curve
- `guide_kink_axis` · enum · {X, Y, Z} — Which axis to use for offset
- `guide_kink_frequency` · float — The frequency of the offset (1/total length)
- `guide_kink_shape` · float — Adjust the offset to the beginning/end
- `guide_kink_amplitude` · float — The amplitude of the offset

## FileAssetSelectIDFilter

Which asset types to show/hide, when browsing an asset library

- `filter_action` · boolean — Show Action data-blocks
- `experimental_filter_armature` · boolean — Show Armature data-blocks
- `filter_brush` · boolean — Show Brushes data-blocks
- `experimental_filter_camera` · boolean — Show Camera data-blocks
- `experimental_filter_cachefile` · boolean — Show Cache File data-blocks
- `experimental_filter_curve` · boolean — Show Curve data-blocks
- `experimental_filter_annotations` · boolean — Show Annotation data-blocks
- `experimental_filter_grease_pencil` · boolean — Show Grease Pencil data-blocks
- `filter_group` · boolean — Show Collection data-blocks
- `experimental_filter_curves` · boolean — Show/hide Curves data-blocks
- `experimental_filter_image` · boolean — Show Image data-blocks
- `experimental_filter_light` · boolean — Show Light data-blocks
- `experimental_filter_light_probe` · boolean — Show Light Probe data-blocks
- `experimental_filter_linestyle` · boolean — Show Freestyle's Line Style data-blocks
- `experimental_filter_lattice` · boolean — Show Lattice data-blocks
- `filter_material` · boolean — Show Material data-blocks
- `experimental_filter_metaball` · boolean — Show Metaball data-blocks
- `experimental_filter_movie_clip` · boolean — Show Movie Clip data-blocks
- `experimental_filter_mesh` · boolean — Show Mesh data-blocks
- `experimental_filter_mask` · boolean — Show Mask data-blocks
- `filter_node_tree` · boolean — Show Node Tree data-blocks
- `filter_object` · boolean — Show Object data-blocks
- `experimental_filter_particle_settings` · boolean — Show Particle Settings data-blocks
- `experimental_filter_palette` · boolean — Show Palette data-blocks
- `experimental_filter_paint_curve` · boolean — Show Paint Curve data-blocks
- `experimental_filter_pointcloud` · boolean — Show/hide Point Cloud data-blocks
- `filter_scene` · boolean — Show Scene data-blocks
- `experimental_filter_speaker` · boolean — Show Speaker data-blocks
- `experimental_filter_sound` · boolean — Show Sound data-blocks
- `experimental_filter_texture` · boolean — Show Texture data-blocks
- `experimental_filter_text` · boolean — Show Text data-blocks
- `experimental_filter_font` · boolean — Show Font data-blocks
- `experimental_filter_volume` · boolean — Show/hide Volume data-blocks
- `filter_world` · boolean — Show World data-blocks
- `experimental_filter_work_space` · boolean — Show workspace data-blocks

## FileAssetSelectParams

Settings for the file selection in Asset Browser mode

- `asset_library_reference` · enum · {ALL, LOCAL, ESSENTIALS, ONLINE_ESSENTIALS, CUSTOM}
- `catalog_id` · string — The UUID of the catalog shown in the browser
- `asset_catalog_visibility` · enum · {ALL, CATALOG, UNASSIGNED} — Which assets to show based on catalog filter
- `filter_asset_id` · pointer · -> FileAssetSelectIDFilter · read-only — Which asset types to show/hide, when browsing an asset library
- `import_method` · enum · {FOLLOW_PREFS, LINK, APPEND, APPEND_REUSE, PACK} — Determine how the asset will be imported
- `instance_collections_on_link` · boolean — Create instances for collections when linking, rather than adding them directly to the scene
- `asset_access` · enum · {ALL, ONLY_ONLINE, ONLY_OFFLINE} — Choose the visibility of online and offline assets
- `instance_collections_on_append` · boolean — Create instances for collections when appending, rather than adding them directly to the scene

## FileBrowserFSMenuEntry

File Select Parameters

- `path` · string
- `name` · string
- `icon` · int
- `use_save` · boolean · read-only — Whether this path is saved in bookmarks, or generated from OS

## FileHandler

Extends functionality to operators that manages files, such as adding drag and drop support

- `bl_idname` · string — If this is set, the file handler gets a custom ID, otherwise it takes the name of the class used to define the file handler (for example, if the class name is "OBJECT_FH_hello", and bl_idname is not set by the script, then bl_idname = "OBJECT_FH_hello")
- `bl_import_operator` · string — Operator that can handle import for files with the extensions given in bl_file_extensions
- `bl_export_operator` · string — Operator that can handle export for files with the extensions given in bl_file_extensions
- `bl_label` · string — The file handler label
- `bl_file_extensions` · string — Formatted string of file extensions supported by the file handler, each extension should start with a "." and be separated by ";".
For Example: ``".blend;.ble"``

## FileSelectEntry

A file viewable in the File Browser

- `name` · string · read-only
- `relative_path` · string · read-only — Path relative to the directory currently displayed in the File Browser (includes the file name)
- `preview_icon_id` · int · read-only — Unique integer identifying the preview of this file as an icon (zero means invalid)
- `asset_data` · pointer · -> AssetMetaData · read-only — Asset data, valid if the file represents an asset

## FileSelectIDFilter

Which ID types to show/hide, when browsing a library

- `filter_action` · boolean — Show Action data-blocks
- `filter_armature` · boolean — Show Armature data-blocks
- `filter_brush` · boolean — Show Brushes data-blocks
- `filter_camera` · boolean — Show Camera data-blocks
- `filter_cachefile` · boolean — Show Cache File data-blocks
- `filter_curve` · boolean — Show Curve data-blocks
- `filter_annotations` · boolean — Show Annotation data-blocks
- `filter_grease_pencil` · boolean — Show Grease Pencil data-blocks
- `filter_group` · boolean — Show Collection data-blocks
- `filter_curves` · boolean — Show/hide Curves data-blocks
- `filter_image` · boolean — Show Image data-blocks
- `filter_light` · boolean — Show Light data-blocks
- `filter_light_probe` · boolean — Show Light Probe data-blocks
- `filter_linestyle` · boolean — Show Freestyle's Line Style data-blocks
- `filter_lattice` · boolean — Show Lattice data-blocks
- `filter_material` · boolean — Show Material data-blocks
- `filter_metaball` · boolean — Show Metaball data-blocks
- `filter_movie_clip` · boolean — Show Movie Clip data-blocks
- `filter_mesh` · boolean — Show Mesh data-blocks
- `filter_mask` · boolean — Show Mask data-blocks
- `filter_node_tree` · boolean — Show Node Tree data-blocks
- `filter_object` · boolean — Show Object data-blocks
- `filter_particle_settings` · boolean — Show Particle Settings data-blocks
- `filter_palette` · boolean — Show Palette data-blocks
- `filter_paint_curve` · boolean — Show Paint Curve data-blocks
- `filter_pointcloud` · boolean — Show/hide Point Cloud data-blocks
- `filter_scene` · boolean — Show Scene data-blocks
- `filter_speaker` · boolean — Show Speaker data-blocks
- `filter_sound` · boolean — Show Sound data-blocks
- `filter_texture` · boolean — Show Texture data-blocks
- `filter_text` · boolean — Show Text data-blocks
- `filter_font` · boolean — Show Font data-blocks
- `filter_volume` · boolean — Show/hide Volume data-blocks
- `filter_world` · boolean — Show World data-blocks
- `filter_work_space` · boolean — Show workspace data-blocks
- `category_scene` · boolean — Show scenes
- `category_animation` · boolean — Show animation data
- `category_object` · boolean — Show objects and collections
- `category_geometry` · boolean — Show meshes, curves, lattice, armatures and metaballs data
- `category_shading` · boolean — Show materials, node-trees, textures and Freestyle's line-styles
- `category_image` · boolean — Show images, movie clips, sounds and masks
- `category_environment` · boolean — Show worlds, lights, cameras and speakers
- `category_misc` · boolean — Show other data types

## FileSelectParams

File Select Parameters

- `title` · string · read-only — Title for the file browser
- `directory` · string — Directory displayed in the file browser
- `filename` · string — Active file in the file browser
- `use_library_browsing` · boolean · read-only — Whether we may browse Blender files' content or not
- `display_type` · enum · {LIST_VERTICAL, LIST_HORIZONTAL, THUMBNAIL} — Display mode for the file list
- `recursion_level` · enum · {NONE, BLEND, ALL_1, ALL_2, ALL_3} — Numbers of dirtree levels to show simultaneously
- `show_details_size` · boolean — Show a column listing the size of each file
- `show_details_datetime` · boolean — Show a column listing the date and time of modification for each file
- `use_filter` · boolean — Enable filtering of files
- `show_hidden` · boolean — Show hidden dot files
- `sort_method` · enum · {FILE_SORT_ALPHA, FILE_SORT_EXTENSION, FILE_SORT_TIME, FILE_SORT_SIZE, ASSET_CATALOG}
- `use_sort_invert` · boolean — Sort items descending, from highest value to lowest
- `use_filter_image` · boolean — Show image files
- `use_filter_blender` · boolean — Show .blend files
- `use_filter_backup` · boolean — Show .blend1, .blend2, etc. files
- `use_filter_movie` · boolean — Show movie files
- `use_filter_script` · boolean — Show script files
- `use_filter_font` · boolean — Show font files
- `use_filter_sound` · boolean — Show sound files
- `use_filter_text` · boolean — Show text files
- `use_filter_volume` · boolean — Show 3D volume files
- `use_filter_folder` · boolean — Show folders
- `use_filter_blendid` · boolean — Show .blend files items (objects, materials, etc.)
- `use_filter_asset_only` · boolean — Hide .blend files items that are not data-blocks with asset metadata
- `filter_id` · pointer · -> FileSelectIDFilter · read-only — Which ID types to show/hide, when browsing a library
- `filter_glob` · string — UNIX shell-like filename patterns matching, supports wildcards ('*') and list of patterns separated by ';'
- `filter_search` · string — Filter by name or tag, supports '*' wildcard
- `display_size` · int — Change the size of thumbnails
- `display_size_discrete` · enum · {TINY, SMALL, NORMAL, BIG, LARGE} — Change the size of thumbnails in discrete steps
- `list_display_size` · int — Change the size of thumbnails in list views
- `list_column_size` · int — The width of columns in horizontal list views

## Float2Attribute

Geometry attribute that stores floating-point 2D vectors

- `data` · collection · read-only

## Float2AttributeValue

2D Vector value in geometry attribute

- `vector` · float · [2] — 2D vector

## Float4Attribute

Geometry attribute that stores floating-point 4D vectors

- `data` · collection · read-only

## Float4AttributeValue

4D Vector value in geometry attribute

- `vector` · float · [4] — 4D vector

## Float4x4Attribute

Geometry attribute that stores a 4 by 4 float matrix

- `data` · collection · read-only

## Float4x4AttributeValue

Matrix value in geometry attribute

- `value` · float · [16] — Matrix

## FloatAttribute

Geometry attribute that stores floating-point values

- `data` · collection · read-only

## FloatAttributeValue

Floating-point value in geometry attribute

- `value` · float

## FloatColorAttribute

Geometry attribute that stores RGBA colors as floating-point values using 32-bits per channel

- `data` · collection · read-only

## FloatColorAttributeValue

Color value in geometry attribute

- `color` · float · [4] — RGBA color in scene linear color space
- `color_srgb` · float · [4] — RGBA color in sRGB color space

## FloatProperty

RNA floating-point number (single precision) property definition

- `default` · float · read-only — Default value for this number
- `default_array` · float · [3] · read-only — Default value for this array
- `array_length` · int · read-only — Maximum length of the array, 0 means unlimited
- `array_dimensions` · int · [3] · read-only — Length of each dimension of the array
- `is_array` · boolean · read-only
- `hard_min` · float · read-only — Hard minimum, trying to assign a value below will silently assign this minimum instead
- `hard_max` · float · read-only — Hard maximum, trying to assign a value above will silently assign this maximum instead
- `soft_min` · float · read-only — Soft minimum (>= hard_min), user cannot drag widgets below this value in the UI
- `soft_max` · float · read-only — Soft maximum (<= hard_max), user cannot drag widgets above this value in the UI
- `step` · float · read-only — Step size used by number buttons, for floats 1/100th of the step size
- `precision` · int · read-only — Number of digits after the dot used by buttons. Fraction is automatically hidden for exact integer values of fields with unit 'NONE' or 'TIME' (frame count) and step divisible by 100.

## FloatVectorAttribute

Geometry attribute that stores floating-point 3D vectors

- `data` · collection · read-only

## FloatVectorAttributeValue

Vector value in geometry attribute

- `vector` · float · [3] — 3D vector

## FloatVectorValueReadOnly

- `vector` · float · [3] · read-only — 3D vector

## ForeachGeometryElementGenerationItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Domain that the field is evaluated on

## ForeachGeometryElementInputItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor

## ForeachGeometryElementMainItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor

## FreestyleLineSet

Line set for associating lines and style parameters

- `linestyle` · pointer · -> FreestyleLineStyle — Line style settings
- `name` · string — Line set name
- `show_render` · boolean — Enable or disable this line set during stroke rendering
- `select_by_visibility` · boolean — Select feature edges based on visibility
- `select_by_edge_types` · boolean — Select feature edges based on edge types
- `select_by_collection` · boolean — Select feature edges based on a collection of objects
- `select_by_image_border` · boolean — Select feature edges by image border (less memory consumption)
- `select_by_face_marks` · boolean — Select feature edges by face marks
- `edge_type_negation` · enum · {INCLUSIVE, EXCLUSIVE} — Specify either inclusion or exclusion of feature edges selected by edge types
- `edge_type_combination` · enum · {OR, AND} — Specify a logical combination of selection conditions on feature edge types
- `collection` · pointer · -> Collection — A collection of objects based on which feature edges are selected
- `collection_negation` · enum · {INCLUSIVE, EXCLUSIVE} — Specify either inclusion or exclusion of feature edges belonging to a collection of objects
- `face_mark_negation` · enum · {INCLUSIVE, EXCLUSIVE} — Specify either inclusion or exclusion of feature edges selected by face marks
- `face_mark_condition` · enum · {ONE, BOTH} — Specify a feature edge selection condition based on face marks
- `select_silhouette` · boolean — Select silhouettes (edges at the boundary of visible and hidden faces)
- `select_border` · boolean — Select border edges (open mesh edges)
- `select_crease` · boolean — Select crease edges (those between two faces making an angle smaller than the Crease Angle)
- `select_ridge_valley` · boolean — Select ridges and valleys (boundary lines between convex and concave areas of surface)
- `select_suggestive_contour` · boolean — Select suggestive contours (almost silhouette/contour edges)
- `select_material_boundary` · boolean — Select edges at material boundaries
- `select_contour` · boolean — Select contours (outer silhouettes of each object)
- `select_external_contour` · boolean — Select external contours (outer silhouettes of occluding and occluded objects)
- `select_edge_mark` · boolean — Select edge marks (edges annotated by Freestyle edge marks)
- `exclude_silhouette` · boolean — Exclude silhouette edges
- `exclude_border` · boolean — Exclude border edges
- `exclude_crease` · boolean — Exclude crease edges
- `exclude_ridge_valley` · boolean — Exclude ridges and valleys
- `exclude_suggestive_contour` · boolean — Exclude suggestive contours
- `exclude_material_boundary` · boolean — Exclude edges at material boundaries
- `exclude_contour` · boolean — Exclude contours
- `exclude_external_contour` · boolean — Exclude external contours
- `exclude_edge_mark` · boolean — Exclude edge marks
- `visibility` · enum · {VISIBLE, HIDDEN, RANGE} — Determine how to use visibility for feature edge selection
- `qi_start` · int — First QI value of the QI range
- `qi_end` · int — Last QI value of the QI range

## FreestyleLineStyle

Freestyle line style, reusable by multiple line sets

- `texture_slots` · collection · [] of LineStyleTextureSlots · read-only — Texture slots defining the mapping and influence of textures
- `active_texture` · pointer · -> Texture — Active texture slot being displayed
- `active_texture_index` · int — Index of active texture slot
- `panel` · enum · {STROKES, COLOR, ALPHA, THICKNESS, GEOMETRY, TEXTURE} — Select the property panel to be shown
- `color` · float · [3] — Base line color, possibly modified by line color modifiers
- `alpha` · float — Base alpha transparency, possibly modified by alpha transparency modifiers
- `thickness` · float — Base line thickness, possibly modified by line thickness modifiers
- `thickness_position` · enum · {CENTER, INSIDE, OUTSIDE, RELATIVE} — Thickness position of silhouettes and border edges (applicable when plain chaining is used with the Same Object option)
- `thickness_ratio` · float — A number between 0 (inside) and 1 (outside) specifying the relative position of stroke thickness
- `color_modifiers` · collection · [] of LineStyleColorModifiers · read-only — List of line color modifiers
- `alpha_modifiers` · collection · [] of LineStyleAlphaModifiers · read-only — List of alpha transparency modifiers
- `thickness_modifiers` · collection · [] of LineStyleThicknessModifiers · read-only — List of line thickness modifiers
- `geometry_modifiers` · collection · [] of LineStyleGeometryModifiers · read-only — List of stroke geometry modifiers
- `use_chaining` · boolean — Enable chaining of feature edges
- `chaining` · enum · {PLAIN, SKETCHY} — Select the way how feature edges are jointed to form chains
- `rounds` · int — Number of rounds in a sketchy multiple touch
- `use_same_object` · boolean — If true, only feature edges of the same object are joined
- `use_split_length` · boolean — Enable chain splitting by curvilinear 2D length
- `split_length` · float — Curvilinear 2D length for chain splitting
- `use_angle_min` · boolean — Split chains at points with angles smaller than the minimum 2D angle
- `angle_min` · float — Minimum 2D angle for splitting chains
- `use_angle_max` · boolean — Split chains at points with angles larger than the maximum 2D angle
- `angle_max` · float — Maximum 2D angle for splitting chains
- `use_length_min` · boolean — Enable the selection of chains by a minimum 2D length
- `length_min` · float — Minimum curvilinear 2D length for the selection of chains
- `use_length_max` · boolean — Enable the selection of chains by a maximum 2D length
- `length_max` · float — Maximum curvilinear 2D length for the selection of chains
- `use_chain_count` · boolean — Enable the selection of first N chains
- `chain_count` · int — Chain count for the selection of first N chains
- `use_split_pattern` · boolean — Enable chain splitting by dashed line patterns
- `split_dash1` · int — Length of the 1st dash for splitting
- `split_gap1` · int — Length of the 1st gap for splitting
- `split_dash2` · int — Length of the 2nd dash for splitting
- `split_gap2` · int — Length of the 2nd gap for splitting
- `split_dash3` · int — Length of the 3rd dash for splitting
- `split_gap3` · int — Length of the 3rd gap for splitting
- `material_boundary` · boolean — If true, chains of feature edges are split at material boundaries
- `use_sorting` · boolean — Arrange the stacking order of strokes
- `sort_key` · enum · {DISTANCE_FROM_CAMERA, 2D_LENGTH, PROJECTED_X, PROJECTED_Y} — Select the sort key to determine the stacking order of chains
- `sort_order` · enum · {DEFAULT, REVERSE} — Select the sort order
- `integration_type` · enum · {MEAN, MIN, MAX, FIRST, LAST} — Select the way how the sort key is computed for each chain
- `use_dashed_line` · boolean — Enable or disable dashed line
- `caps` · enum · {BUTT, ROUND, SQUARE} — Select the shape of both ends of strokes
- `dash1` · int — Length of the 1st dash for dashed lines
- `gap1` · int — Length of the 1st gap for dashed lines
- `dash2` · int — Length of the 2nd dash for dashed lines
- `gap2` · int — Length of the 2nd gap for dashed lines
- `dash3` · int — Length of the 3rd dash for dashed lines
- `gap3` · int — Length of the 3rd gap for dashed lines
- `use_texture` · boolean — Enable or disable textured strokes
- `texture_spacing` · float — Spacing for textures along stroke length
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `node_tree` · pointer · -> NodeTree · read-only — Node tree for node-based shaders
- `use_nodes` · boolean — Use shader nodes for the line style

## FreestyleModuleSettings

Style module configuration for specifying a style module

- `script` · pointer · -> Text — Python script to define a style module
- `use` · boolean — Enable or disable this style module during stroke rendering

## FreestyleSettings

Freestyle settings for a ViewLayer data-block

- `modules` · collection · [] of FreestyleModules · read-only — A list of style modules (to be applied from top to bottom)
- `mode` · enum · {SCRIPT, EDITOR} — Select the Freestyle control mode
- `use_culling` · boolean — If enabled, out-of-view edges are ignored
- `use_suggestive_contours` · boolean — Enable suggestive contours
- `use_ridges_and_valleys` · boolean — Enable ridges and valleys
- `use_material_boundaries` · boolean — Enable material boundaries
- `use_smoothness` · boolean — Take face smoothness into account in view map calculation
- `use_view_map_cache` · boolean — Keep the computed view map and avoid recalculating it if mesh geometry is unchanged
- `as_render_pass` · boolean — Renders Freestyle output to a separate pass instead of overlaying it on the Combined pass
- `sphere_radius` · float — Sphere radius for computing curvatures
- `kr_derivative_epsilon` · float — Kr derivative epsilon for computing suggestive contours
- `crease_angle` · float — Angular threshold for detecting crease edges
- `linesets` · collection · [] of Linesets · read-only

## Function

RNA function definition

- `identifier` · string · read-only — Unique name used in the code and scripting
- `description` · string · read-only — Description of the Function's purpose
- `parameters` · collection · read-only — Parameters for the function
- `is_registered` · boolean · read-only — Function is registered as callback as part of type registration
- `is_registered_optional` · boolean · read-only — Function is optionally registered as callback part of type registration
- `use_self` · boolean · read-only — Function does not pass itself as an argument (becomes a static method in Python)
- `use_self_type` · boolean · read-only — Function passes itself type as an argument (becomes a class method in Python if use_self is false)

## GPencilInterpolateSettings

Settings for Grease Pencil interpolation tools

- `interpolation_curve` · pointer · -> CurveMapping · read-only — Custom curve to control 'sequence' interpolation between Grease Pencil frames

## GPencilSculptSettings

General properties for Grease Pencil stroke sculpting tools

- `guide` · pointer · -> GPencilSculptGuide · read-only
- `use_multiframe_falloff` · boolean — Use falloff effect when edit in multiframe mode to compute brush effect by frame
- `use_thickness_curve` · boolean — Use curve to define primitive stroke thickness
- `use_scale_thickness` · boolean — Scale the stroke thickness when transforming strokes
- `use_automasking_stroke` · boolean — Affect only strokes below the cursor
- `use_automasking_layer_stroke` · boolean — Affect only strokes below the cursor
- `use_automasking_material_stroke` · boolean — Affect only strokes below the cursor
- `use_automasking_layer_active` · boolean — Affect only the Active Layer
- `use_automasking_material_active` · boolean — Affect only the Active Material
- `multiframe_falloff_curve` · pointer · -> CurveMapping · read-only — Custom curve to control falloff of brush effect by Grease Pencil frames
- `thickness_primitive_curve` · pointer · -> CurveMapping · read-only — Custom curve to control primitive thickness
- `lock_axis` · enum · {VIEW, AXIS_Y, AXIS_X, AXIS_Z, CURSOR}
- `intersection_threshold` · float — Threshold for stroke intersections

## Gizmo

Collection of gizmos

- `properties` · pointer · -> GizmoProperties · read-only
- `bl_idname` · string
- `group` · pointer · -> GizmoGroup · read-only — Gizmo group this gizmo is a member of
- `color` · float · [3]
- `alpha` · float
- `color_highlight` · float · [3]
- `alpha_highlight` · float
- `matrix_space` · float · [16]
- `matrix_basis` · float · [16]
- `matrix_offset` · float · [16]
- `matrix_world` · float · [16] · read-only
- `scale_basis` · float
- `line_width` · float
- `select_bias` · float — Depth bias used for selection
- `hide` · boolean
- `hide_select` · boolean
- `hide_keymap` · boolean — Ignore the key-map for this gizmo
- `use_grab_cursor` · boolean
- `use_draw_hover` · boolean
- `use_draw_modal` · boolean — Show while dragging
- `use_draw_value` · boolean — Show an indicator for the current value while dragging
- `use_draw_offset_scale` · boolean — Scale the offset matrix (use to apply screen-space offset)
- `use_draw_scale` · boolean — Use scale when calculating the matrix
- `use_select_background` · boolean — Don't write into the depth buffer
- `use_operator_tool_properties` · boolean — Merge active tool properties on activation (does not overwrite existing)
- `use_event_handle_all` · boolean — When highlighted, do not pass events through to be handled by other keymaps
- `use_tooltip` · boolean — Use tooltips when hovering over this gizmo
- `use_undo` · boolean — Push an undo step after each use of the gizmo
- `is_highlight` · boolean · read-only
- `is_modal` · boolean · read-only
- `select` · boolean

## GizmoGroup

Storage of an operator being executed, or registered after execution

- `bl_idname` · string
- `bl_label` · string
- `bl_space_type` · enum · {EMPTY, VIEW_3D, IMAGE_EDITOR, NODE_EDITOR, SEQUENCE_EDITOR, CLIP_EDITOR, DOPESHEET_EDITOR, GRAPH_EDITOR, NLA_EDITOR, TEXT_EDITOR, CONSOLE, INFO, TOPBAR, STATUSBAR, OUTLINER, PROPERTIES, FILE_BROWSER, SPREADSHEET, PREFERENCES} — The space where the panel is going to be used in
- `bl_region_type` · enum · {WINDOW, HEADER, CHANNELS, TEMPORARY, UI, TOOLS, TOOL_PROPS, ASSET_SHELF, ASSET_SHELF_HEADER, PREVIEW, HUD, NAVIGATION_BAR, EXECUTE, FOOTER, TOOL_HEADER, XR, SCRUBBING} — The region where the panel is going to be used in
- `bl_owner_id` · string
- `bl_options` · enum · {3D, SCALE, DEPTH_3D, SELECT, PERSISTENT, SHOW_MODAL_ALL, EXCLUDE_MODAL, TOOL_INIT, TOOL_FALLBACK_KEYMAP, VR_REDRAWS} — Options for this operator type
- `name` · string · read-only
- `gizmos` · collection · [] of Gizmos · read-only — List of gizmos in the Gizmo Map

## GpPaint

- `color_mode` · enum · {MATERIAL, VERTEXCOLOR} — Paint Mode

## Header

Editor header containing UI elements

- `layout` · pointer · -> UILayout · read-only — Structure of the header in the UI
- `bl_idname` · string — If this is set, the header gets a custom ID, otherwise it takes the name of the class used to define the header; for example, if the class name is "OBJECT_HT_hello", and bl_idname is not set by the script, then bl_idname = "OBJECT_HT_hello"
- `bl_space_type` · enum · {EMPTY, VIEW_3D, IMAGE_EDITOR, NODE_EDITOR, SEQUENCE_EDITOR, CLIP_EDITOR, DOPESHEET_EDITOR, GRAPH_EDITOR, NLA_EDITOR, TEXT_EDITOR, CONSOLE, INFO, TOPBAR, STATUSBAR, OUTLINER, PROPERTIES, FILE_BROWSER, SPREADSHEET, PREFERENCES} — The space where the header is going to be used in
- `bl_region_type` · enum · {WINDOW, HEADER, CHANNELS, TEMPORARY, UI, TOOLS, TOOL_PROPS, ASSET_SHELF, ASSET_SHELF_HEADER, PREVIEW, HUD, NAVIGATION_BAR, EXECUTE, FOOTER, TOOL_HEADER, XR, SCRUBBING} — The region where the header is going to be used in (defaults to header region)

## Histogram

Statistical view of the levels of color in an image

- `mode` · enum · {LUMA, RGB, R, G, B, A} — Channels to display in the histogram
- `show_line` · boolean — Display lines rather than filled shapes

## ID

Base type for data-blocks, defining a unique name, linking from other libraries and garbage collection

- `name` · string — Unique data-block ID name (within a same type and library)
- `name_full` · string · read-only — Unique data-block ID name, including library one if any
- `id_type` · enum · {ACTION, ARMATURE, BRUSH, CACHEFILE, CAMERA, COLLECTION, CURVE, CURVES, FONT, GREASEPENCIL, GREASEPENCIL_V3, IMAGE, KEY, LATTICE, LIBRARY, LIGHT, LIGHT_PROBE, LINESTYLE, MASK, MATERIAL, MESH, META, MOVIECLIP, NODETREE, OBJECT, PAINTCURVE, PALETTE, PARTICLE, POINTCLOUD, SCENE, SCREEN, SOUND, SPEAKER, TEXT, TEXTURE, VOLUME, WINDOWMANAGER, WORKSPACE, WORLD} · read-only — Type identifier of this data-block
- `session_uid` · int · read-only — A session-wide unique identifier for the data block that remains the same across renames and internal reallocations, unchanged when reloading the file
- `is_evaluated` · boolean · read-only — Whether this ID is runtime-only, evaluated data-block, or actual data from .blend file
- `original` · pointer · -> ID · read-only — Actual data-block from .blend file (Main database) that generated that evaluated one
- `users` · int · read-only — Number of times this data-block is referenced
- `use_fake_user` · boolean — Save this data-block even if it has no users
- `use_extra_user` · boolean — Indicates whether an extra user is set or not (mainly for internal/debug usages)
- `is_embedded_data` · boolean · read-only — This data-block is not an independent one, but is actually a sub-data of another ID (typical example: root node trees or master collections)
- `is_linked_packed` · boolean · read-only — This data-block is linked and packed into the .blend file
- `is_missing` · boolean · read-only — This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)
- `is_runtime_data` · boolean — This data-block is runtime data, i.e. it won't be saved in .blend file. Note that e.g. evaluated IDs are always runtime, so this value is only editable for data-blocks in Main data-base.
- `is_editable` · boolean · read-only — This data-block is editable in the user interface. Linked data-blocks are not editable, except if they were loaded as editable assets.
- `tag` · boolean — Tools can use this to tag data for their own purposes (initial state is undefined)
- `is_library_indirect` · boolean · read-only — Is this ID block linked indirectly
- `library` · pointer · -> Library · read-only — Library file the data-block is linked from
- `library_weak_reference` · pointer · -> LibraryWeakReference · read-only — Weak reference to a data-block in another library .blend file (used to re-use already appended data instead of appending new copies)
- `asset_data` · pointer · -> AssetMetaData — Additional data for an asset data-block
- `override_library` · pointer · -> IDOverrideLibrary · read-only — Library override data
- `preview` · pointer · -> ImagePreview · read-only — Preview image and icon of this data-block (always None if not supported for this type of data)

## IDOverrideLibrary

Struct gathering all data needed by overridden linked IDs

- `reference` · pointer · -> ID · read-only — Linked ID used as reference by this override
- `hierarchy_root` · pointer · -> ID · read-only — Library override ID used as root of the override hierarchy this ID is a member of
- `is_in_hierarchy` · boolean — Whether this library override is defined as part of a library hierarchy, or as a single, isolated and autonomous override
- `is_system_override` · boolean — Whether this library override exists only for the override hierarchy, or if it is actually editable by the user
- `properties` · collection · [] of IDOverrideLibraryProperties · read-only — List of overridden properties

## IDOverrideLibraryProperty

Description of an overridden property

- `rna_path` · string · read-only — RNA path leading to that property, from owning ID
- `operations` · collection · [] of IDOverrideLibraryPropertyOperations · read-only — List of overriding operations for a property

## IDOverrideLibraryPropertyOperation

Description of an override operation over an overridden property

- `operation` · enum · {NOOP, REPLACE, DIFF_ADD, DIFF_SUB, FACT_MULTIPLY, INSERT_AFTER, INSERT_BEFORE, CUSTOM} · read-only — What override operation is performed
- `flag` · enum · {MANDATORY, LOCKED, IDPOINTER_MATCH_REFERENCE, IDPOINTER_ITEM_USE_ID} · read-only — Status flags
- `subitem_reference_name` · string · read-only — Used to handle changes into collection
- `subitem_local_name` · string · read-only — Used to handle changes into collection
- `subitem_reference_id` · pointer · -> ID · read-only — Collection of IDs only, used to disambiguate between potential IDs with same name from different libraries
- `subitem_local_id` · pointer · -> ID · read-only — Collection of IDs only, used to disambiguate between potential IDs with same name from different libraries
- `subitem_reference_index` · int · read-only — Used to handle changes into collection
- `subitem_local_index` · int · read-only — Used to handle changes into collection
- `label` · string · read-only — UI label to display in dedicated view of the Outliner, in place of the actual UI widget to edit the value
- `tooltip` · string · read-only — UI tooltip to display in dedicated view of the Outliner, when the label itself cannot provide all required information

## IKParam

Base type for IK solver parameters

- `ik_solver` · enum · {LEGACY, ITASC} · read-only — IK solver for which these parameters are defined

## IndexSwitchItem

- `identifier` · int · read-only — Consistent identifier used for the item

## Int2Attribute

Geometry attribute that stores 2D integer vectors

- `data` · collection · read-only

## Int2AttributeValue

2D value in geometry attribute

- `value` · int · [2] — 2D vector

## IntAttribute

Geometry attribute that stores integer values

- `data` · collection · read-only

## IntAttributeValue

Integer value in geometry attribute

- `value` · int

## IntProperty

RNA integer number property definition

- `default` · int · read-only — Default value for this number
- `default_array` · int · [3] · read-only — Default value for this array
- `array_length` · int · read-only — Maximum length of the array, 0 means unlimited
- `array_dimensions` · int · [3] · read-only — Length of each dimension of the array
- `is_array` · boolean · read-only
- `hard_min` · int · read-only — Hard minimum, trying to assign a value below will silently assign this minimum instead
- `hard_max` · int · read-only — Hard maximum, trying to assign a value above will silently assign this maximum instead
- `soft_min` · int · read-only — Soft minimum (>= hard_min), user cannot drag widgets below this value in the UI
- `soft_max` · int · read-only — Soft maximum (<= hard_max), user cannot drag widgets above this value in the UI
- `step` · int · read-only — Step size used by number buttons, for floats 1/100th of the step size

## Itasc

Parameters for the iTaSC IK solver

- `precision` · float — Precision of convergence in case of reiteration
- `iterations` · int — Maximum number of iterations for convergence in case of reiteration
- `step_count` · int — Divide the frame interval into this many steps
- `translate_root_bones` · boolean — Translate root (i.e. parentless) bones to the armature origin
- `mode` · enum · {ANIMATION, SIMULATION}
- `reiteration_method` · enum · {NEVER, INITIAL, ALWAYS} — Defines if the solver is allowed to reiterate (converge until precision is met) on none, first or all frames
- `use_auto_step` · boolean — Automatically determine the optimal number of steps for best performance/accuracy trade off
- `step_min` · float — Lower bound for timestep in second in case of automatic substeps
- `step_max` · float — Higher bound for timestep in second in case of automatic substeps
- `feedback` · float — Feedback coefficient for error correction, average response time is 1/feedback
- `velocity_max` · float — Maximum joint velocity in radians/second
- `solver` · enum · {SDLS, DLS} — Solving method selection: automatic damping or manual damping
- `damping_max` · float — Maximum damping coefficient when singular value is nearly 0 (higher values produce results with more stability, less reactivity)
- `damping_epsilon` · float — Singular value under which damping is progressively applied (higher values produce results with more stability, less reactivity)

## Lattice

Lattice data-block defining a grid for deforming other objects

- `points_u` · int — Points in U direction (cannot be changed when there are shape keys)
- `points_v` · int — Points in V direction (cannot be changed when there are shape keys)
- `points_w` · int — Points in W direction (cannot be changed when there are shape keys)
- `interpolation_type_u` · enum · {KEY_LINEAR, KEY_CARDINAL, KEY_CATMULL_ROM, KEY_BSPLINE}
- `interpolation_type_v` · enum · {KEY_LINEAR, KEY_CARDINAL, KEY_CATMULL_ROM, KEY_BSPLINE}
- `interpolation_type_w` · enum · {KEY_LINEAR, KEY_CARDINAL, KEY_CATMULL_ROM, KEY_BSPLINE}
- `use_outside` · boolean — Only display and take into account the outer vertices
- `vertex_group` · string — Vertex group to apply the influence of the lattice
- `shape_keys` · pointer · -> Key · read-only
- `points` · collection · read-only — Points of the lattice
- `is_editmode` · boolean · read-only — True when used in editmode
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## LatticePoint

Point in the lattice grid

- `select` · boolean — Selection status
- `co` · float · [3] · read-only — Original undeformed location used to calculate the strength of the deform effect (edit/animate the Deformed Location instead)
- `co_deform` · float · [3]
- `weight_softbody` · float — Softbody goal weight
- `groups` · collection · read-only — Weights for the vertex groups this point is member of

## LayerCollection

Layer collection

- `collection` · pointer · -> Collection · read-only — Collection this layer collection is wrapping
- `name` · string · read-only — Name of this layer collection (same as its collection one)
- `children` · collection · read-only — Layer collection children
- `exclude` · boolean — Exclude from view layer
- `holdout` · boolean — Mask out objects in collection from view layer
- `indirect_only` · boolean — Objects in collection only contribute indirectly (through shadows and reflections) in the view layer
- `hide_viewport` · boolean — Temporarily hide in viewport
- `is_visible` · boolean · read-only — Whether this collection is visible for the view layer, take into account the collection parent

## LayoutPanelState

- `is_open` · boolean

## Library

External .blend file from which data is linked

- `filepath` · string — Path to the library .blend file
- `parent` · pointer · -> Library · read-only
- `packed_file` · pointer · -> PackedFile · read-only
- `version` · int · [3] · read-only — Version of Blender the library .blend was saved with
- `needs_liboverride_resync` · boolean — True if this library contains library overrides that are linked in current blendfile, and that had to be recursively resynced on load (it is recommended to open and re-save that library blendfile then)
- `is_archive` · boolean · read-only — This library is an 'archive' storage for packed linked IDs originally linked from its 'archive parent' library.
- `archive_parent_library` · pointer · -> Library · read-only — Source library from which this archive of packed IDs was generated
- `archive_libraries` · collection · read-only — Archive libraries of packed IDs, generated (and owned) by this source library

## LibraryWeakReference

Read-only external reference to a linked data-block and its library file

- `filepath` · string — Path to the library .blend file
- `id_name` · string — Full ID name in the library .blend file (including the two leading 'id type' chars)

## Linesets

Line sets for associating lines and style parameters

- `active` · pointer · -> FreestyleLineSet · read-only — Active line set being displayed
- `active_index` · int — Index of active line set slot

## LoopColors

Collection of vertex colors

- `active` · pointer · -> MeshLoopColorLayer — Active vertex color layer
- `active_index` · int — Active vertex color index

## Macro

Storage of a macro operator being executed, or registered after execution

- `name` · string · read-only
- `properties` · pointer · -> OperatorProperties · read-only
- `has_reports` · boolean · read-only — Operator has a set of reports (warnings and errors) from last execution
- `bl_idname` · string
- `bl_label` · string
- `bl_translation_context` · string
- `bl_description` · string
- `bl_undo_group` · string
- `bl_options` · enum · {REGISTER, UNDO, UNDO_GROUPED, BLOCKING, MACRO, GRAB_CURSOR, GRAB_CURSOR_X, GRAB_CURSOR_Y, DEPENDS_ON_CURSOR, PRESET, INTERNAL, MODAL_PRIORITY} — Options for this operator type
- `bl_cursor_pending` · enum · {DEFAULT, NONE, WAIT, CROSSHAIR, MOVE_X, MOVE_Y, KNIFE, TEXT, PAINT_BRUSH, PAINT_CROSS, DOT, ERASER, HAND, HAND_POINT, HAND_CLOSED, SCROLL_X, SCROLL_Y, SCROLL_XY, EYEDROPPER, PICK_AREA, STOP, COPY, CROSS, MUTE, ZOOM_IN, ZOOM_OUT} — Cursor to use when waiting for the user to select a location to activate the operator (when ``bl_options`` has ``DEPENDS_ON_CURSOR`` set)

## Mask

Mask data-block defining mask for compositing

- `layers` · collection · [] of MaskLayers · read-only — Collection of layers which defines this mask
- `active_layer_index` · int — Index of active layer in list of all mask's layers
- `frame_start` · int — First frame of the mask (used for sequencer)
- `frame_end` · int — Final frame of the mask (used for sequencer)
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## MaskLayer

Single layer used for masking pixels

- `name` · string — Unique name of layer
- `splines` · collection · [] of MaskSplines · read-only — Collection of splines which defines this layer
- `hide` · boolean — Restrict visibility in the viewport
- `hide_select` · boolean — Restrict selection in the viewport
- `hide_render` · boolean — Restrict renderability
- `select` · boolean — Layer is selected for editing in the Dope Sheet
- `alpha` · float — Render Opacity
- `blend` · enum · {MERGE_ADD, MERGE_SUBTRACT, ADD, SUBTRACT, LIGHTEN, DARKEN, MUL, REPLACE, DIFFERENCE} — Method of blending mask layers
- `invert` · boolean — Invert the mask black/white
- `falloff` · enum · {SMOOTH, SPHERE, ROOT, INVERSE_SQUARE, SHARP, LINEAR} — Falloff type of the feather
- `fill_solver` · enum · {SWEEP_LINE, CDT} — Triangulation solver for filling 2D curves
- `use_fill_holes` · boolean — Calculate holes when filling overlapping curves
- `use_fill_overlap` · boolean — Calculate self intersections and overlap before filling (only for the sweep-line solver)

## MaskLayers

Collection of layers used by mask

- `active` · pointer · -> MaskLayer — Active layer in this mask

## MaskParent

Parenting settings for masking element

- `id` · pointer · -> ID — ID-block to which masking element would be parented to or to its property
- `id_type` · enum · {MOVIECLIP} — Type of ID-block that can be used
- `type` · enum · {POINT_TRACK, PLANE_TRACK} — Parent Type
- `parent` · string — Name of parent object in specified data-block to which parenting happens
- `sub_parent` · string — Name of parent sub-object in specified data-block to which parenting happens

## MaskSpline

Single spline used for defining mask shape

- `offset_mode` · enum · {EVEN, SMOOTH} — The method used for calculating the feather offset
- `weight_interpolation` · enum · {LINEAR, EASE} — The type of weight interpolation for spline
- `use_cyclic` · boolean — Make this spline a closed loop
- `use_fill` · boolean — Make this spline filled
- `use_self_intersection_check` · boolean — Prevent feather from self-intersections
- `points` · collection · [] of MaskSplinePoints · read-only — Collection of points

## MaskSplinePoint

Single point in spline used for defining mask

- `handle_left` · float · [2] — Coordinates of the first handle
- `co` · float · [2] — Coordinates of the control point
- `handle_right` · float · [2] — Coordinates of the second handle
- `handle_type` · enum · {AUTO, VECTOR, ALIGNED, ALIGNED_DOUBLESIDE, FREE} — Handle type
- `handle_left_type` · enum · {AUTO, VECTOR, ALIGNED, ALIGNED_DOUBLESIDE, FREE} — Handle type
- `handle_right_type` · enum · {AUTO, VECTOR, ALIGNED, ALIGNED_DOUBLESIDE, FREE} — Handle type
- `weight` · float — Weight of the point
- `select` · boolean — Selection status of the control point. (Deprecated: use Select Control Point instead)
- `select_left_handle` · boolean — Selection status of the left handle
- `select_control_point` · boolean — Selection status of the control point
- `select_right_handle` · boolean — Selection status of the right handle
- `select_single_handle` · boolean — Selection status of the Aligned Single handle
- `parent` · pointer · -> MaskParent · read-only
- `feather_points` · collection · read-only — Points defining feather

## MaskSplinePointUW

Single point in spline segment defining feather

- `u` · float — U coordinate of point along spline segment
- `weight` · float — Weight of feather point
- `select` · boolean — Selection status

## MaskSplines

Collection of masking splines

- `active` · pointer · -> MaskSpline — Active spline of masking layer
- `active_point` · pointer · -> MaskSplinePoint — Active point of masking layer

## Menu

Editor menu containing buttons

- `layout` · pointer · -> UILayout · read-only — Defines the structure of the menu in the UI
- `bl_idname` · string — If this is set, the menu gets a custom ID, otherwise it takes the name of the class used to define the menu (for example, if the class name is "OBJECT_MT_hello", and bl_idname is not set by the script, then bl_idname = "OBJECT_MT_hello")
- `bl_label` · string — The menu label
- `bl_translation_context` · string
- `bl_description` · string
- `bl_owner_id` · string
- `bl_options` · enum · {SEARCH_ON_KEY_PRESS} — Options for this menu type

## MetaBall

Metaball data-block to define blobby surfaces

- `elements` · collection · [] of MetaBallElements · read-only — Metaball elements
- `update_method` · enum · {UPDATE_ALWAYS, HALFRES, FAST, NEVER} — Metaball edit update behavior
- `resolution` · float — Polygonization resolution in the 3D viewport
- `render_resolution` · float — Polygonization resolution in rendering
- `threshold` · float — Influence of metaball elements
- `use_auto_texspace` · boolean — Adjust active object's texture space automatically when transforming object
- `texspace_location` · float · [3] — Texture space location
- `texspace_size` · float · [3] — Texture space size
- `materials` · collection · [] of IDMaterials · read-only
- `is_editmode` · boolean · read-only — True when used in editmode
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `cycles` · pointer · -> CyclesMeshSettings · read-only — Cycles mesh settings

## MetaBallElements

Collection of metaball elements

- `active` · pointer · -> MetaElement · read-only — Last selected element

## MetaElement

Blobby element in a metaball data-block

- `type` · enum · {BALL, CAPSULE, PLANE, ELLIPSOID, CUBE} — Metaball type
- `co` · float · [3]
- `rotation` · float · [4] — Normalized quaternion rotation
- `radius` · float
- `size_x` · float — Size of element, use of components depends on element type
- `size_y` · float — Size of element, use of components depends on element type
- `size_z` · float — Size of element, use of components depends on element type
- `stiffness` · float — Stiffness defines how much of the element to fill
- `use_negative` · boolean — Set metaball as negative one
- `use_scale_stiffness` · boolean — Scale stiffness instead of radius
- `select` · boolean — Select element
- `hide` · boolean — Hide element

## MotionPath

Cache of the world-space positions of an element over a frame range

- `points` · collection · read-only — Cached positions per frame
- `frame_start` · int · read-only — Starting frame of the stored range
- `frame_end` · int · read-only — End frame of the stored range
- `length` · int · read-only — Number of frames cached
- `color` · float · [3] — Custom color for motion path before the current frame
- `color_post` · float · [3] — Custom color for motion path after the current frame
- `line_thickness` · int — Line thickness for motion path
- `use_bone_head` · boolean · read-only — For PoseBone paths, use the bone head location when calculating this path
- `is_modified` · boolean — Path is being edited
- `use_custom_color` · boolean — Use custom color for this motion path
- `lines` · boolean — Use straight lines between keyframe points

## MotionPathVert

Cached location on path

- `co` · float · [3]
- `select` · boolean — Path point is selected for editing

## MovieClip

MovieClip data-block referencing an external movie file

- `filepath` · string — Filename of the movie or sequence file
- `tracking` · pointer · -> MovieTracking · read-only
- `proxy` · pointer · -> MovieClipProxy · read-only
- `use_proxy` · boolean — Use a preview proxy for this clip
- `size` · int · [2] · read-only — Width and height in pixels, zero when image data cannot be loaded
- `display_aspect` · float · [2] — Display Aspect for this clip, does not affect rendering
- `source` · enum · {SEQUENCE, MOVIE} · read-only — Where the clip comes from
- `use_proxy_custom_directory` · boolean — Create proxy images in a custom directory (default is movie location)
- `annotation` · pointer · -> Annotation — Annotation data for this movie clip
- `frame_start` · int — Global scene frame number at which this movie starts playing (affects all data associated with a clip)
- `frame_offset` · int — Offset of footage first frame relative to its file name (affects only how footage is loading, does not change data associated with a clip)
- `frame_duration` · int · read-only — Detected duration of movie clip in frames
- `fps` · float · read-only — Detected frame rate of the movie clip in frames per second
- `colorspace_settings` · pointer · -> ColorManagedInputColorspaceSettings · read-only — Input color space settings
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## MovieClipProxy

Proxy parameters for a movie clip

- `build_25` · boolean — Build proxy resolution 25% of the original footage dimension
- `build_50` · boolean — Build proxy resolution 50% of the original footage dimension
- `build_75` · boolean — Build proxy resolution 75% of the original footage dimension
- `build_100` · boolean — Build proxy resolution 100% of the original footage dimension
- `build_undistorted_25` · boolean — Build proxy resolution 25% of the original undistorted footage dimension
- `build_undistorted_50` · boolean — Build proxy resolution 50% of the original undistorted footage dimension
- `build_undistorted_75` · boolean — Build proxy resolution 75% of the original undistorted footage dimension
- `build_undistorted_100` · boolean — Build proxy resolution 100% of the original undistorted footage dimension
- `quality` · int — JPEG quality of proxy images
- `directory` · string — Location to store the proxy files

## MovieClipUser

Parameters defining how a MovieClip data-block is used by another data-block

- `frame_current` · int — Current frame number in movie or image sequence
- `proxy_render_size` · enum · {PROXY_25, PROXY_50, PROXY_75, PROXY_100, FULL} — Display preview using full resolution or different proxy resolutions
- `use_render_undistorted` · boolean — Render preview using undistorted proxy

## MovieTracking

Match-moving data for tracking

- `settings` · pointer · -> MovieTrackingSettings · read-only
- `camera` · pointer · -> MovieTrackingCamera · read-only
- `tracks` · collection · [] of MovieTrackingTracks · read-only — Collection of tracks in this tracking data object. Deprecated, use objects[name].tracks
- `plane_tracks` · collection · [] of MovieTrackingPlaneTracks · read-only — Collection of plane tracks in this tracking data object. Deprecated, use objects[name].plane_tracks
- `stabilization` · pointer · -> MovieTrackingStabilization · read-only
- `reconstruction` · pointer · -> MovieTrackingReconstruction · read-only
- `objects` · collection · [] of MovieTrackingObjects · read-only — Collection of objects in this tracking data object
- `active_object_index` · int — Index of active object
- `dopesheet` · pointer · -> MovieTrackingDopesheet · read-only

## MovieTrackingDopesheet

Match-moving dopesheet data

- `sort_method` · enum · {NAME, LONGEST, TOTAL, AVERAGE_ERROR, START, END} — Method to be used to sort channels in dopesheet view
- `use_invert_sort` · boolean — Invert sort order of dopesheet channels
- `show_only_selected` · boolean — Only include channels relating to selected objects and data
- `show_hidden` · boolean — Include channels from objects/bone that are not visible

## MovieTrackingMarker

Match-moving marker data for tracking

- `co` · float · [2] — Marker position at frame in normalized coordinates
- `frame` · int — Frame number marker is keyframed on
- `mute` · boolean — Is marker muted for current frame
- `pattern_corners` · float · [8] — Array of coordinates which represents pattern's corners in normalized coordinates relative to marker position
- `pattern_bound_box` · float · [4] · read-only — Pattern area bounding box in normalized coordinates
- `search_min` · float · [2] — Left-bottom corner of search area in normalized coordinates relative to marker position
- `search_max` · float · [2] — Right-bottom corner of search area in normalized coordinates relative to marker position
- `is_keyed` · boolean — Whether the position of the marker is keyframed or tracked

## MovieTrackingPlaneMarker

Match-moving plane marker data for tracking

- `frame` · int — Frame number marker is keyframed on
- `corners` · float · [8] — Array of coordinates which represents UI rectangle corners in frame normalized coordinates
- `mute` · boolean — Is marker muted for current frame

## MovieTrackingPlaneTrack

Match-moving plane track data for tracking

- `name` · string — Unique name of track
- `markers` · collection · [] of MovieTrackingPlaneMarkers · read-only — Collection of markers in track
- `select` · boolean — Plane track is selected
- `use_auto_keying` · boolean — Automatic keyframe insertion when moving plane corners
- `image` · pointer · -> Image — Image displayed in the track during editing in clip editor
- `image_opacity` · float — Opacity of the image

## MovieTrackingPlaneTracks

Collection of movie tracking plane tracks

- `active` · pointer · -> MovieTrackingPlaneTrack — Active plane track in this tracking data object. Deprecated, use objects[name].plane_tracks.active

## MovieTrackingReconstruction

Match-moving reconstruction data from tracker

- `is_valid` · boolean · read-only — Whether the tracking data contains valid reconstruction information
- `average_error` · float · read-only — Average error of reconstruction
- `cameras` · collection · [] of MovieTrackingReconstructedCameras · read-only — Collection of solved cameras

## MovieTrackingSettings

Match moving settings

- `speed` · enum · {FASTEST, DOUBLE, REALTIME, HALF, QUARTER} — Limit speed of tracking to make visual feedback easier (this does not affect the tracking quality)
- `use_keyframe_selection` · boolean — Automatically select keyframes when solving camera/object motion
- `refine_intrinsics_focal_length` · boolean — Refine focal length during camera solving
- `refine_intrinsics_principal_point` · boolean — Refine principal point during camera solving
- `refine_intrinsics_radial_distortion` · boolean — Refine radial coefficients of distortion model during camera solving
- `refine_intrinsics_tangential_distortion` · boolean — Refine tangential coefficients of distortion model during camera solving
- `distance` · float — Distance between two bundles used for scene scaling
- `clean_frames` · int — Effect on tracks which are tracked less than the specified amount of frames
- `clean_error` · float — Effect on tracks which have a larger re-projection error
- `clean_action` · enum · {SELECT, DELETE_TRACK, DELETE_SEGMENTS} — Cleanup action to execute
- `use_tripod_solver` · boolean — Use special solver to track a stable camera position, such as a tripod
- `default_frames_limit` · int — Every tracking cycle, this number of frames are tracked
- `default_pattern_match` · enum · {KEYFRAME, PREV_FRAME} — Track pattern from given frame when tracking marker to next frame
- `default_margin` · int — Default distance from image boundary at which marker stops tracking
- `default_motion_model` · enum · {Perspective, Affine, LocRotScale, LocScale, LocRot, Loc} — Default motion model to use for tracking
- `use_default_brute` · boolean — Use a brute-force translation-only initialization when tracking
- `use_default_mask` · boolean — Use a Grease Pencil data-block as a mask to use only specified areas of pattern when tracking
- `use_default_normalization` · boolean — Normalize light intensities while tracking (slower)
- `default_correlation_min` · float — Default minimum value of correlation between matched pattern and reference that is still treated as successful tracking
- `default_pattern_size` · int — Size of pattern area for newly created tracks
- `default_search_size` · int — Size of search area for newly created tracks
- `use_default_red_channel` · boolean — Use red channel from footage for tracking
- `use_default_green_channel` · boolean — Use green channel from footage for tracking
- `use_default_blue_channel` · boolean — Use blue channel from footage for tracking
- `default_weight` · float — Influence of newly created track on a final solution
- `object_distance` · float — Distance between two bundles used for object scaling

## MovieTrackingStabilization

2D stabilization based on tracking markers

- `use_2d_stabilization` · boolean — Use 2D stabilization for footage
- `use_stabilize_rotation` · boolean — Stabilize detected rotation around center of frame
- `use_stabilize_scale` · boolean — Compensate any scale changes relative to center of rotation
- `tracks` · collection · read-only — Collection of tracks used for 2D stabilization (translation)
- `active_track_index` · int — Index of active track in translation stabilization tracks list
- `rotation_tracks` · collection · read-only — Collection of tracks used for 2D stabilization (translation)
- `active_rotation_track_index` · int — Index of active track in rotation stabilization tracks list
- `anchor_frame` · int — Reference point to anchor stabilization (other frames will be adjusted relative to this frame's position)
- `target_position` · float · [2] — Known relative offset of original shot, will be subtracted (e.g. for panning shot, can be animated)
- `target_rotation` · float — Rotation present on original shot, will be compensated (e.g. for deliberate tilting)
- `target_scale` · float — Explicitly scale resulting frame to compensate zoom of original shot
- `use_autoscale` · boolean — Automatically scale footage to cover unfilled areas when stabilizing
- `scale_max` · float — Limit the amount of automatic scaling
- `influence_location` · float — Influence of stabilization algorithm on footage location
- `influence_scale` · float — Influence of stabilization algorithm on footage scale
- `influence_rotation` · float — Influence of stabilization algorithm on footage rotation
- `filter_type` · enum · {NEAREST, BILINEAR, BICUBIC} — Interpolation to use for sub-pixel shifts and rotations due to stabilization
- `show_tracks_expanded` · boolean — Show UI list of tracks participating in stabilization

## MovieTrackingTrack

Match-moving track data for tracking

- `name` · string — Unique name of track
- `frames_limit` · int — Every tracking cycle, this number of frames are tracked
- `pattern_match` · enum · {KEYFRAME, PREV_FRAME} — Track pattern from given frame when tracking marker to next frame
- `margin` · int — Distance from image boundary at which marker stops tracking
- `motion_model` · enum · {Perspective, Affine, LocRotScale, LocScale, LocRot, Loc} — Default motion model to use for tracking
- `correlation_min` · float — Minimal value of correlation between matched pattern and reference that is still treated as successful tracking
- `use_brute` · boolean — Use a brute-force translation only pre-track before refinement
- `use_mask` · boolean — Use a Grease Pencil data-block as a mask to use only specified areas of pattern when tracking
- `use_normalization` · boolean — Normalize light intensities while tracking (slower)
- `markers` · collection · [] of MovieTrackingMarkers · read-only — Collection of markers in track
- `use_red_channel` · boolean — Use red channel from footage for tracking
- `use_green_channel` · boolean — Use green channel from footage for tracking
- `use_blue_channel` · boolean — Use blue channel from footage for tracking
- `use_grayscale_preview` · boolean — Display what the tracking algorithm sees in the preview
- `use_alpha_preview` · boolean — Apply track's mask on displaying preview
- `has_bundle` · boolean · read-only — True if track has a valid bundle
- `bundle` · float · [3] · read-only — Position of bundle reconstructed from this track
- `hide` · boolean — Track is hidden
- `select` · boolean — Track is selected
- `select_anchor` · boolean — Track's anchor point is selected
- `select_pattern` · boolean — Track's pattern area is selected
- `select_search` · boolean — Track's search area is selected
- `lock` · boolean — Track is locked and all changes to it are disabled
- `use_custom_color` · boolean — Use custom color instead of theme-defined
- `color` · float · [3] — Color of the track in the Movie Clip Editor and the 3D viewport after a solve
- `average_error` · float · read-only — Average error of re-projection
- `annotation` · pointer · -> Annotation — Annotation data for this track
- `weight` · float — Influence of this track on a final solution
- `weight_stab` · float — Influence of this track on 2D stabilization
- `offset` · float · [2] — Offset of track from the parenting point

## MovieTrackingTracks

Collection of movie tracking tracks

- `active` · pointer · -> MovieTrackingTrack — Active track in this tracking data object. Deprecated, use objects[name].tracks.active

## NDOFMotionEventData

NDOF motion data for window manager events

- `translation` · float · [3] · read-only — The translation of this motion event. The range on each axis is [-1 to 1], before being multiplied by the sensitivity preference. This is typically scaled by the time-delta before use.
- `rotation` · float · [3] · read-only — Axis-angle rotation of this motion event. The vector magnitude is the angle where 1.0 represents 360 degrees. The angle is typically scaled by the time-delta before use.
- `progress` · enum · {STARTING, IN_PROGRESS, FINISHING} · read-only — Indicates the gesture phase
- `time_delta` · float · read-only — Time since previous motion event (in seconds)

## PackedFile

External file packed into the .blend file

- `size` · int · read-only — Size of packed file in bytes
- `data` · string · read-only — Raw data (bytes, exact content of the embedded file)

## Paint

- `brush` · pointer · -> Brush · read-only — Active brush
- `brush_asset_reference` · pointer · -> AssetWeakReference · read-only — A weak reference to the matching brush asset, used e.g. to restore the last used brush on file load
- `palette` · pointer · -> Palette — Active Palette
- `show_brush` · boolean
- `show_brush_on_surface` · boolean
- `show_low_resolution` · boolean — For multires, show low resolution while navigating the view
- `use_sculpt_delay_updates` · boolean — Update the geometry when it enters the view, providing faster view navigation
- `show_bvh_nodes` · boolean — Show the underlying BVH nodes as differently colored faces
- `use_symmetry_x` · boolean — Mirror brush across the X axis
- `use_symmetry_y` · boolean — Mirror brush across the Y axis
- `use_symmetry_z` · boolean — Mirror brush across the Z axis
- `use_symmetry_feather` · boolean — Reduce the strength of the brush where it overlaps symmetrical daubs
- `cavity_curve` · pointer · -> CurveMapping · read-only — Editable cavity curve
- `use_cavity` · boolean — Mask painting according to mesh geometry cavity
- `tile_offset` · float · [3] — Stride at which tiled strokes are copied
- `tile_x` · boolean — Tile along X axis
- `tile_y` · boolean — Tile along Y axis
- `tile_z` · boolean — Tile along Z axis
- `show_strength_curve` · boolean
- `show_size_curve` · boolean
- `show_jitter_curve` · boolean
- `unified_paint_settings` · pointer · -> UnifiedPaintSettings · read-only
- `mesh_automasking_settings` · pointer · -> MeshAutomaskingSettings · read-only

## PaintModeSettings

Properties of paint mode

- `canvas_source` · enum · {COLOR_ATTRIBUTE, MATERIAL, IMAGE} — Source to select canvas from
- `canvas_image` · pointer · -> Image — Image used as painting target

## Palette

- `colors` · collection · [] of PaletteColors · read-only

## PaletteColor

- `color` · float · [3]
- `strength` · float
- `weight` · float

## PaletteColors

Collection of palette colors

- `active` · pointer · -> PaletteColor

## Panel

Panel containing UI elements

- `layout` · pointer · -> UILayout · read-only — Defines the structure of the panel in the UI
- `text` · string — Override for the panel label in the UI
- `custom_data` · pointer · -> Constraint · read-only — Panel data
- `bl_idname` · string — If this is set, the panel gets a custom ID, otherwise it takes the name of the class used to define the panel. For example, if the class name is "OBJECT_PT_hello", and bl_idname is not set by the script, then bl_idname = "OBJECT_PT_hello".
- `bl_label` · string — The panel label, shows up in the panel header at the right of the triangle used to collapse the panel
- `bl_translation_context` · string — Specific translation context, only define when the label needs to be disambiguated from others using the exact same label
- `bl_description` · string — The panel tooltip
- `bl_category` · string — The category (tab) in which the panel will be displayed, when applicable
- `bl_icon` · enum · {NONE, CHAR_NOTDEF, CHAR_REPLACEMENT, NOT_FOUND, BLANK1, AUTOMERGE_OFF, AUTOMERGE_ON, CHECKBOX_DEHLT, CHECKBOX_HLT, CLIPUV_DEHLT, CLIPUV_HLT, DECORATE_UNLOCKED, DECORATE_LOCKED, FAKE_USER_OFF, FAKE_USER_ON, HIDE_ON, HIDE_OFF, INDIRECT_ONLY_OFF, INDIRECT_ONLY_ON, ONIONSKIN_OFF, ONIONSKIN_ON, UNPINNED, PINNED, RADIOBUT_OFF, RADIOBUT_ON, RECORD_OFF, RECORD_ON, RESTRICT_RENDER_ON, RESTRICT_RENDER_OFF, RESTRICT_SELECT_ON, RESTRICT_SELECT_OFF, RESTRICT_VIEW_ON, RESTRICT_VIEW_OFF, RIGHTARROW, DOWNARROW_HLT, SELECT_INTERSECT, SELECT_DIFFERENCE, SNAP_OFF, SNAP_ON, PLAYHEAD_SNAP_OFF, ... (1033 total)} — Icon override for the panel category tab
- `bl_icon_value` · int — Icon value override for the panel category tab
- `bl_owner_id` · string — The ID owning the data displayed in the panel, if any
- `bl_space_type` · enum · {EMPTY, VIEW_3D, IMAGE_EDITOR, NODE_EDITOR, SEQUENCE_EDITOR, CLIP_EDITOR, DOPESHEET_EDITOR, GRAPH_EDITOR, NLA_EDITOR, TEXT_EDITOR, CONSOLE, INFO, TOPBAR, STATUSBAR, OUTLINER, PROPERTIES, FILE_BROWSER, SPREADSHEET, PREFERENCES} — The space where the panel is going to be used in
- `bl_region_type` · enum · {WINDOW, HEADER, CHANNELS, TEMPORARY, UI, TOOLS, TOOL_PROPS, ASSET_SHELF, ASSET_SHELF_HEADER, PREVIEW, HUD, NAVIGATION_BAR, EXECUTE, FOOTER, TOOL_HEADER, XR, SCRUBBING} — The region where the panel is going to be used in
- `bl_context` · string — The context in which the panel belongs to. (TODO: explain the possible combinations bl_context/bl_region_type/bl_space_type)
- `bl_options` · enum · {DEFAULT_CLOSED, HIDE_HEADER, INSTANCED, HEADER_LAYOUT_EXPAND} — Options for this panel type
- `bl_parent_id` · string — If this is set, the panel becomes a sub-panel
- `bl_ui_units_x` · int — When set, defines popup panel width
- `bl_order` · int — Panels with lower numbers are default ordered before panels with higher numbers
- `use_pin` · boolean — Show the panel on all tabs
- `is_popover` · boolean · read-only

## PathCompare

Match paths against this value

- `path` · string
- `use_glob` · boolean — Enable wildcard globbing

## Point

Point in a point cloud

- `co` · float · [3]
- `radius` · float
- `index` · int · read-only — Index of this point

## PointCache

Active point cache for physics simulations

- `frame_start` · int — Frame on which the simulation starts
- `frame_end` · int — Frame on which the simulation stops
- `frame_step` · int — Number of frames between cached frames
- `index` · int — Index number of cache files
- `is_baked` · boolean · read-only — The cache is baked
- `is_baking` · boolean · read-only — The cache is being baked
- `use_disk_cache` · boolean — Save cache files to disk (.blend file must be saved first)
- `is_outdated` · boolean · read-only
- `is_frame_skip` · boolean · read-only — Some frames were skipped while baking/saving that cache
- `name` · string — Cache name
- `filepath` · string — Cache file path
- `info` · string · read-only — Info on current cache status
- `use_external` · boolean — Read cache from an external location
- `use_library_path` · boolean — Use this file's path for the disk cache when library linked into another file (for local bakes per scene file, disable this option)
- `point_caches` · collection · [] of PointCaches · read-only

## PointCacheItem

Point cache for physics simulations

- `frame_start` · int — Frame on which the simulation starts
- `frame_end` · int — Frame on which the simulation stops
- `frame_step` · int — Number of frames between cached frames
- `index` · int — Index number of cache files
- `is_baked` · boolean · read-only — The cache is baked
- `is_baking` · boolean · read-only — The cache is being baked
- `use_disk_cache` · boolean — Save cache files to disk (.blend file must be saved first)
- `is_outdated` · boolean · read-only
- `is_frame_skip` · boolean · read-only — Some frames were skipped while baking/saving that cache
- `name` · string — Cache name
- `filepath` · string — Cache file path
- `info` · string · read-only — Info on current cache status
- `use_external` · boolean — Read cache from an external location
- `use_library_path` · boolean — Use this file's path for the disk cache when library linked into another file (for local bakes per scene file, disable this option)

## PointCaches

Collection of point caches

- `active_index` · int

## PointCloud

Point cloud data-block

- `points` · collection · read-only
- `materials` · collection · [] of IDMaterials · read-only
- `attributes` · collection · [] of AttributeGroupPointCloud · read-only — Geometry attributes
- `color_attributes` · collection · [] of AttributeGroupPointCloud · read-only — Geometry color attributes
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## PointerProperty

RNA pointer property to point to another RNA struct

- `fixed_type` · pointer · -> Struct · read-only — Fixed pointer type, empty if variable type

## Pose

A collection of pose channels, including settings for animating bones

- `bones` · collection · read-only — Individual pose bones for the armature
- `ik_solver` · enum · {LEGACY, ITASC} — Selection of IK solver for IK chain
- `ik_param` · pointer · -> IKParam · read-only — Parameters for IK solver
- `use_mirror_x` · boolean — Apply changes to matching bone on opposite side of X-Axis
- `use_mirror_relative` · boolean — Apply relative transformations in X-mirror mode (not supported with Auto IK)
- `use_auto_ik` · boolean — Add temporary IK constraints while grabbing bones in Pose Mode
- `animation_visualization` · pointer · -> AnimViz · read-only — Animation data for this data-block

## PrimitiveBoolean

RNA wrapped boolean

- `value` · boolean · read-only

## PrimitiveFloat

RNA wrapped float

- `value` · float · read-only

## PrimitiveInt

RNA wrapped int

- `value` · int · read-only

## PrimitiveString

RNA wrapped string

- `value` · string · read-only

## Property

RNA property definition

- `name` · string · read-only — Human readable name
- `identifier` · string · read-only — Unique name used in the code and scripting
- `description` · string · read-only — Description of the property for tooltips
- `translation_context` · string · read-only — Translation context of the property's name
- `type` · enum · {BOOLEAN, INT, FLOAT, STRING, ENUM, POINTER, COLLECTION} · read-only — Data type of the property
- `subtype` · enum · {NONE, FILE_PATH, DIR_PATH, FILE_NAME, BYTE_STRING, PASSWORD, PIXEL, PIXEL_DIAMETER, UNSIGNED, PERCENTAGE, FACTOR, MASS, ANGLE, TIME, TIME_ABSOLUTE, DISTANCE, DISTANCE_DIAMETER, DISTANCE_CAMERA, POWER, TEMPERATURE, WAVELENGTH, COLOR_TEMPERATURE, FREQUENCY, COLOR, TRANSLATION, DIRECTION, VELOCITY, ACCELERATION, MATRIX, EULER, QUATERNION, AXISANGLE, XYZ, XYZ_LENGTH, COLOR_GAMMA, COORDINATES, LAYER, LAYER_MEMBER} · read-only — Semantic interpretation of the property
- `srna` · pointer · -> Struct · read-only — Struct definition used for properties assigned to this item
- `unit` · enum · {NONE, LENGTH, AREA, VOLUME, ROTATION, TIME, TIME_ABSOLUTE, VELOCITY, ACCELERATION, MASS, CAMERA, POWER, TEMPERATURE, WAVELENGTH, COLOR_TEMPERATURE, FREQUENCY} · read-only — Type of units for this property
- `icon` · enum · {NONE, CHAR_NOTDEF, CHAR_REPLACEMENT, NOT_FOUND, BLANK1, AUTOMERGE_OFF, AUTOMERGE_ON, CHECKBOX_DEHLT, CHECKBOX_HLT, CLIPUV_DEHLT, CLIPUV_HLT, DECORATE_UNLOCKED, DECORATE_LOCKED, FAKE_USER_OFF, FAKE_USER_ON, HIDE_ON, HIDE_OFF, INDIRECT_ONLY_OFF, INDIRECT_ONLY_ON, ONIONSKIN_OFF, ONIONSKIN_ON, UNPINNED, PINNED, RADIOBUT_OFF, RADIOBUT_ON, RECORD_OFF, RECORD_ON, RESTRICT_RENDER_ON, RESTRICT_RENDER_OFF, RESTRICT_SELECT_ON, RESTRICT_SELECT_OFF, RESTRICT_VIEW_ON, RESTRICT_VIEW_OFF, RIGHTARROW, DOWNARROW_HLT, SELECT_INTERSECT, SELECT_DIFFERENCE, SNAP_OFF, SNAP_ON, PLAYHEAD_SNAP_OFF, ... (1033 total)} · read-only — Icon of the item
- `is_readonly` · boolean · read-only — Property is editable through RNA
- `is_animatable` · boolean · read-only — Property is animatable through RNA
- `is_overridable` · boolean · read-only — Property is overridable through RNA
- `is_required` · boolean · read-only — False when this property is an optional argument in an RNA function
- `is_argument_optional` · boolean · read-only — True when the property is optional in a Python function implementing an RNA function
- `is_never_none` · boolean · read-only — True when this value cannot be set to None
- `is_hidden` · boolean · read-only — True when the property is hidden
- `is_skip_save` · boolean · read-only — True when the property uses ghost values
- `is_skip_preset` · boolean · read-only — True when the property is not saved in presets
- `is_output` · boolean · read-only — True when this property is an output value from an RNA function
- `is_registered` · boolean · read-only — Property is registered as part of type registration
- `is_registered_optional` · boolean · read-only — Property is optionally registered as part of type registration
- `is_runtime` · boolean · read-only — Property has been dynamically created at runtime
- `is_enum_flag` · boolean · read-only — True when multiple enums
- `is_library_editable` · boolean · read-only — Property is editable from linked instances (changes not saved)
- `is_path_output` · boolean · read-only — Property is a filename, filepath or directory output
- `is_path_supports_blend_relative` · boolean · read-only — Property is a path which supports the "//" prefix, signifying the location as relative to the ".blend" file's directory
- `is_path_supports_templates` · boolean · read-only — Property is a path which supports the "{variable_name}" variable expression syntax, which substitutes the value of the referenced variable in place of the expression
- `is_deprecated` · boolean · read-only — The property is deprecated
- `deprecated_note` · string · read-only — A note regarding deprecation
- `deprecated_version` · int · [3] · read-only — The Blender version this was deprecated
- `deprecated_removal_version` · int · [3] · read-only — The Blender version this is expected to be removed
- `tags` · enum · {} · read-only — Subset of tags (defined in parent struct) that are set for this property

## PropertyGroup

Group of ID properties

- `name` · string — Unique name used in the code and scripting, can be re-defined in Python sub-classes if needed

## PropertyGroupItem

Property that stores arbitrary, user defined properties

- `string` · string
- `int` · int
- `int_array` · int
- `float` · float
- `float_array` · float
- `double` · float
- `double_array` · float
- `bool` · boolean
- `bool_array` · boolean
- `enum` · enum · {DEFAULT}
- `group` · pointer · -> PropertyGroup · read-only
- `collection` · collection · read-only
- `idp_array` · collection · read-only
- `id` · pointer · -> ID

## QuaternionAttribute

Geometry attribute that stores rotation

- `data` · collection · read-only

## QuaternionAttributeValue

Rotation value in geometry attribute

- `value` · float · [4] — Quaternion

## RaytraceEEVEE

Quality options for the raytracing pipeline

- `resolution_scale` · enum · {1, 2, 4, 8, 16} — Determines the number of rays per pixel. Higher resolution uses more memory.
- `use_backface_hit` · boolean — Consider rays hitting backfaces as valid
- `backface_radiance_scale` · float — Amount of the front face lighting to reuse for backface lighting approximation
- `use_denoise` · boolean — Enable noise reduction techniques for raytraced effects
- `denoise_spatial` · boolean — Reuse neighbor pixels' rays
- `denoise_temporal` · boolean — Accumulate samples by reprojecting last tracing results
- `denoise_bilateral` · boolean — Blur the resolved radiance using a bilateral filter
- `screen_trace_thickness` · float — Surface thickness used to detect intersection when using screen-tracing
- `trace_max_roughness` · float — Maximum roughness to use the tracing pipeline for. Higher roughness surfaces will use fast GI approximation. A value of 1 will disable fast GI approximation.
- `screen_trace_quality` · float — Precision of the screen space ray-tracing

## ReadOnlyInteger

- `value` · int · read-only

## Region

Region in a subdivided screen area

- `type` · enum · {WINDOW, HEADER, CHANNELS, TEMPORARY, UI, TOOLS, TOOL_PROPS, ASSET_SHELF, ASSET_SHELF_HEADER, PREVIEW, HUD, NAVIGATION_BAR, EXECUTE, FOOTER, TOOL_HEADER, XR, SCRUBBING} · read-only — Type of this region
- `x` · int · read-only — The window relative vertical location of the region
- `y` · int · read-only — The window relative horizontal location of the region
- `width` · int · read-only — Region width
- `height` · int · read-only — Region height
- `view2d` · pointer · -> View2D · read-only — 2D view of the region
- `alignment` · enum · {NONE, TOP, BOTTOM, LEFT, RIGHT, HORIZONTAL_SPLIT, VERTICAL_SPLIT, FLOAT, QUAD_SPLIT} · read-only — Alignment of the region within the area
- `data` · pointer · -> AnyType · read-only — Region specific data (the type depends on the region type)
- `active_panel_category` · enum · {UNSUPPORTED} — The current active panel category, may be Null if the region does not support this feature (NOTE: these categories are generated at runtime, so list may be empty at initialization, before any drawing took place)

## RepeatItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor

## Report

Report entry

- `session_uid` · int · read-only — Unique per-session report identifier
- `type` · enum · {DEBUG, INFO, OPERATOR, PROPERTY, WARNING, ERROR, ERROR_INVALID_INPUT, ERROR_INVALID_CONTEXT, ERROR_OUT_OF_MEMORY} · read-only — Report type (severity)
- `message` · string · read-only — Report message text

## Scopes

Scopes for statistical view of an image

- `use_full_resolution` · boolean — Sample every pixel of the image
- `accuracy` · float — Proportion of original image source pixel lines to sample
- `histogram` · pointer · -> Histogram · read-only — Histogram for viewing image statistics
- `waveform_mode` · enum · {LUMA, PARADE, YCBCR601, YCBCR709, YCBCRJPG, RGB}
- `waveform_alpha` · float — Opacity of the points
- `vectorscope_mode` · enum · {LUMA, RGB}
- `vectorscope_alpha` · float — Opacity of the points

## Screen

Screen data-block, defining the layout of areas in a window

- `areas` · collection · read-only — Areas the screen is subdivided into
- `is_animation_playing` · boolean · read-only — Animation playback is active
- `is_scrubbing` · boolean · read-only — True when the user is scrubbing through time
- `is_temporary` · boolean · read-only
- `show_fullscreen` · boolean · read-only — An area is maximized, filling this screen
- `show_statusbar` · boolean — Show status bar
- `use_play_top_left_3d_editor` · boolean
- `use_play_3d_editors` · boolean
- `use_follow` · boolean — Follow current frame in editors
- `use_play_animation_editors` · boolean
- `use_play_properties_editors` · boolean
- `use_play_image_editors` · boolean
- `use_play_sequence_editors` · boolean
- `use_play_node_editors` · boolean
- `use_play_clip_editors` · boolean
- `use_play_spreadsheet_editors` · boolean

## ScriptDirectory

- `name` · string — Identifier for the Python scripts directory
- `directory` · string — Alternate script path, matching the default layout with sub-directories: startup, add-ons, modules, and presets (requires restart)

## Sculpt

- `lock_x` · boolean — Disallow changes to the X axis of vertices
- `lock_y` · boolean — Disallow changes to the Y axis of vertices
- `lock_z` · boolean — Disallow changes to the Z axis of vertices
- `use_deform_only` · boolean — Use only deformation modifiers (temporary disable all constructive modifiers except multi-resolution)
- `detail_size` · float — Maximum edge length for dynamic topology sculpting (in pixels)
- `detail_percent` · float — Maximum edge length for dynamic topology sculpting (in brush percentage)
- `constant_detail_resolution` · float — Maximum edge length for dynamic topology sculpting (as divisor of Blender unit - higher value means smaller edge length)
- `symmetrize_direction` · enum · {NEGATIVE_X, POSITIVE_X, NEGATIVE_Y, POSITIVE_Y, NEGATIVE_Z, POSITIVE_Z} — Source and destination for symmetrize operator
- `detail_refine_method` · enum · {SUBDIVIDE, COLLAPSE, SUBDIVIDE_COLLAPSE} — In dynamic-topology mode, how to add or remove mesh detail
- `detail_type_method` · enum · {RELATIVE, CONSTANT, BRUSH, MANUAL} — In dynamic-topology mode, how mesh detail size is calculated
- `gravity` · float — Amount of gravity after each dab
- `transform_mode` · enum · {ALL_VERTICES, RADIUS_ELASTIC} — How the transformation is going to be applied to the target
- `gravity_object` · pointer · -> Object — Object whose Z axis defines orientation of gravity

## SelectedUvElement

- `element_index` · int
- `face_index` · int

## ShaderFx

Effect affecting the Grease Pencil object

- `name` · string — Effect name
- `type` · enum · {FX_BLUR, FX_COLORIZE, FX_FLIP, FX_GLOW, FX_PIXEL, FX_RIM, FX_SHADOW, FX_SWIRL, FX_WAVE} · read-only
- `show_viewport` · boolean — Display effect in viewport
- `show_render` · boolean — Use effect during render
- `show_in_editmode` · boolean — Display effect in Edit mode
- `show_expanded` · boolean — Set effect expansion in the user interface

## ShaderFxBlur

Gaussian Blur effect

- `size` · float · [2] — Factor of Blur
- `samples` · int — Number of Blur Samples (zero, disable blur)
- `rotation` · float — Rotation of the effect
- `use_dof_mode` · boolean — Blur using camera depth of field

## ShaderFxColorize

Colorize effect

- `factor` · float — Mix factor
- `low_color` · float · [4] — First color used for effect
- `high_color` · float · [4] — Second color used for effect
- `mode` · enum · {GRAYSCALE, SEPIA, DUOTONE, TRANSPARENT, CUSTOM} — Effect mode

## ShaderFxFlip

Flip effect

- `use_flip_x` · boolean — Flip image horizontally
- `use_flip_y` · boolean — Flip image vertically

## ShaderFxGlow

Glow effect

- `glow_color` · float · [3] — Color used for generated glow
- `opacity` · float — Effect Opacity
- `select_color` · float · [3] — Color selected to apply glow
- `mode` · enum · {LUMINANCE, COLOR} — Glow mode
- `threshold` · float — Limit to select color for glow effect
- `size` · float · [2] — Size of the effect
- `samples` · int — Number of Blur Samples
- `use_glow_under` · boolean — Glow only areas with alpha (not supported with Regular blend mode)
- `rotation` · float — Rotation of the effect
- `blend_mode` · enum · {REGULAR, ADD, SUBTRACT, MULTIPLY, DIVIDE} — Blend mode

## ShaderFxPixel

Pixelate effect

- `size` · int · [2] — Pixel size
- `use_antialiasing` · boolean — Antialias pixels

## ShaderFxRim

Rim effect

- `offset` · int · [2] — Offset of the rim
- `rim_color` · float · [3] — Color used for Rim
- `mask_color` · float · [3] — Color that must be kept
- `mode` · enum · {NORMAL, OVERLAY, ADD, SUBTRACT, MULTIPLY, DIVIDE} — Blend mode
- `blur` · int · [2] — Number of pixels for blurring rim (set to 0 to disable)
- `samples` · int — Number of Blur Samples (zero, disable blur)

## ShaderFxShadow

Shadow effect

- `object` · pointer · -> Object — Object to determine center of rotation
- `offset` · int · [2] — Offset of the shadow
- `scale` · float · [2] — Scale of the shadow
- `shadow_color` · float · [4] — Color used for Shadow
- `orientation` · enum · {HORIZONTAL, VERTICAL} — Direction of the wave
- `amplitude` · float — Amplitude of Wave
- `period` · float — Period of Wave
- `phase` · float — Phase Shift of Wave
- `rotation` · float — Rotation around center or object
- `blur` · int · [2] — Number of pixels for blurring shadow (set to 0 to disable)
- `samples` · int — Number of Blur Samples (zero, disable blur)
- `use_object` · boolean — Use object as center of rotation
- `use_wave` · boolean — Use wave effect

## ShaderFxSwirl

Swirl effect

- `radius` · int — Radius to apply
- `angle` · float — Angle of rotation
- `use_transparent` · boolean — Make image transparent outside of radius
- `object` · pointer · -> Object — Object to determine center location

## ShaderFxWave

Wave Deformation effect

- `orientation` · enum · {HORIZONTAL, VERTICAL} — Direction of the wave
- `amplitude` · float — Amplitude of Wave
- `period` · float — Period of Wave
- `phase` · float — Phase Shift of Wave

## Short2Attribute

Geometry attribute that stores 2D integer vectors

- `data` · collection · read-only

## Short2AttributeValue

2D value in geometry attribute

- `value` · int · [2] — 2D vector

## SimulationStateItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `attribute_domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Domain where the field is captured if it is not already an attribute-field

## SoftBodySettings

Soft body simulation settings for an object

- `friction` · float — General media friction for point movements
- `mass` · float — General Mass value
- `vertex_group_mass` · string — Control point mass values
- `gravity` · float — Apply gravitation to point movement
- `speed` · float — Tweak timing for physics to control frequency and speed
- `vertex_group_goal` · string — Control point weight values
- `goal_min` · float — Goal minimum, vertex weights are scaled to match this range
- `goal_max` · float — Goal maximum, vertex weights are scaled to match this range
- `goal_default` · float — Default Goal (vertex target position) value
- `goal_spring` · float — Goal (vertex target position) spring stiffness
- `goal_friction` · float — Goal (vertex target position) friction
- `pull` · float — Edge spring stiffness when longer than rest length
- `push` · float — Edge spring stiffness when shorter than rest length
- `damping` · float — Edge spring friction
- `spring_length` · int — Alter spring length to shrink/blow up (unit %) 0 to disable
- `aero` · int — Make edges 'sail'
- `plastic` · int — Permanent deform
- `bend` · float — Bending Stiffness
- `shear` · float — Shear Stiffness
- `vertex_group_spring` · string — Control point spring strength values
- `collision_type` · enum · {MANUAL, AVERAGE, MINIMAL, MAXIMAL, MINMAX} — Choose Collision Type
- `ball_size` · float — Absolute ball size or factor if not manually adjusted
- `ball_stiff` · float — Ball inflating pressure
- `ball_damp` · float — Blending to inelastic collision
- `error_threshold` · float — The Runge-Kutta ODE solver error limit, low value gives more precision, high values speed
- `step_min` · int — Minimal # solver steps/frame
- `step_max` · int — Maximal # solver steps/frame
- `choke` · int — 'Viscosity' inside collision target
- `fuzzy` · int — Fuzziness while on collision, high values make collision handling faster but less stable
- `use_auto_step` · boolean — Use velocities for automagic step sizes
- `use_diagnose` · boolean — Turn on SB diagnose console prints
- `use_estimate_matrix` · boolean — Store the estimated transforms in the soft body settings
- `location_mass_center` · float · [3] — Location of center of mass
- `rotation_estimate` · float · [9] — Estimated rotation matrix
- `scale_estimate` · float · [9] — Estimated scale matrix
- `use_goal` · boolean — Define forces for vertices to stick to animated position
- `use_edges` · boolean — Use Edges as springs
- `use_stiff_quads` · boolean — Add diagonal springs on 4-gons
- `use_edge_collision` · boolean — Edges collide too
- `use_face_collision` · boolean — Faces collide too, can be very slow
- `aerodynamics_type` · enum · {SIMPLE, LIFT_FORCE} — Method of calculating aerodynamic interaction
- `use_self_collision` · boolean — Enable naive vertex ball self collision
- `collision_collection` · pointer · -> Collection — Limit colliders to this collection
- `effector_weights` · pointer · -> EffectorWeights · read-only

## Sound

Sound data-block referencing an external or packed sound file

- `filepath` · string — Sound sample file used by this Sound data-block
- `packed_file` · pointer · -> PackedFile · read-only
- `use_memory_cache` · boolean — The sound file is decoded and loaded into RAM
- `use_mono` · boolean — If the file contains multiple audio channels they are rendered to a single one
- `samplerate` · int · read-only — Sample rate of the audio in Hz
- `channels` · enum · {INVALID, MONO, STEREO, STEREO_LFE, CHANNELS_4, CHANNELS_5, SURROUND_51, SURROUND_61, SURROUND_71} · read-only — Definition of audio channels

## Speaker

Speaker data-block for 3D audio speaker objects

- `muted` · boolean — Mute the speaker
- `sound` · pointer · -> Sound — Sound data-block used by this speaker
- `volume_max` · float — Maximum volume, no matter how near the object is
- `volume_min` · float — Minimum volume, no matter how far away the object is
- `distance_max` · float — Maximum distance for volume calculation, no matter how far away the object is
- `distance_reference` · float — Reference distance at which volume is 100%
- `attenuation` · float — How strong the distance affects volume, depending on distance model
- `cone_angle_outer` · float — Angle of the outer cone, in degrees, outside this cone the volume is the outer cone volume, between inner and outer cone the volume is interpolated
- `cone_angle_inner` · float — Angle of the inner cone, in degrees, inside the cone the volume is 100%
- `cone_volume_outer` · float — Volume outside the outer cone
- `volume` · float — How loud the sound is
- `pitch` · float — Playback pitch of the sound
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## Spline

Element of a curve, either NURBS, Bézier or Polyline or a character with text objects

- `points` · collection · [] of SplinePoints · read-only — Collection of points that make up this poly or nurbs spline
- `bezier_points` · collection · [] of SplineBezierPoints · read-only — Collection of points for Bézier curves only
- `tilt_interpolation` · enum · {LINEAR, CARDINAL, BSPLINE, EASE} — The type of tilt interpolation for 3D, Bézier curves
- `radius_interpolation` · enum · {LINEAR, CARDINAL, BSPLINE, EASE} — The type of radius interpolation for Bézier curves
- `type` · enum · {POLY, BEZIER, NURBS} — The interpolation type for this curve element
- `point_count_u` · int · read-only — Total number points for the curve or surface in the U direction
- `point_count_v` · int · read-only — Total number points for the surface on the V direction
- `order_u` · int — NURBS order in the U direction. Higher values make each point influence a greater area, but have worse performance.
- `order_v` · int — NURBS order in the V direction. Higher values make each point influence a greater area, but have worse performance.
- `resolution_u` · int — Curve or Surface subdivisions per segment
- `resolution_v` · int — Surface subdivisions per segment
- `use_cyclic_u` · boolean — Make this curve or surface a closed loop in the U direction
- `use_cyclic_v` · boolean — Make this surface a closed loop in the V direction
- `use_endpoint_u` · boolean — Make this nurbs curve or surface meet the endpoints in the U direction
- `use_endpoint_v` · boolean — Make this nurbs surface meet the endpoints in the V direction
- `use_bezier_u` · boolean — Make this nurbs curve or surface act like a Bézier spline in the U direction
- `use_bezier_v` · boolean — Make this nurbs surface act like a Bézier spline in the V direction
- `use_smooth` · boolean — Smooth the normals of the surface or beveled curve
- `hide` · boolean — Hide this curve in Edit mode
- `material_index` · int — Material slot index of this curve
- `character_index` · int · read-only — Location of this character in the text data (only for text curves)

## SplinePoint

Spline point without handles

- `select` · boolean — Selection status
- `hide` · boolean — Visibility status
- `co` · float · [4] — Point coordinates
- `weight` · float — NURBS weight
- `tilt` · float — Tilt in 3D View
- `weight_softbody` · float — Softbody goal weight
- `radius` · float — Radius for beveling

## SpreadsheetColumn

Persistent data associated with a spreadsheet column

- `data_type` · enum · {INT32, FLOAT, BOOLEAN, INSTANCES} · read-only — The data type of the corresponding column visible in the spreadsheet
- `id` · pointer · -> SpreadsheetColumnID · read-only — Data used to identify the corresponding data from the data source

## SpreadsheetColumnID

Data used to identify a spreadsheet column

- `name` · string

## SpreadsheetRowFilter

- `enabled` · boolean
- `show_expanded` · boolean
- `column_name` · string
- `operation` · enum · {EQUAL, GREATER, LESS}
- `value_float` · float
- `value_float2` · float · [2]
- `value_float3` · float · [3]
- `value_float4` · float · [4]
- `value_color` · float · [4]
- `value_string` · string
- `threshold` · float — How close float values need to be to be equal
- `value_int` · int
- `value_int8` · int
- `value_int2` · int · [2]
- `value_int3` · int · [3]
- `value_boolean` · boolean

## SpreadsheetTable

Persistent data associated with a table

- `id` · pointer · -> SpreadsheetTableID · read-only — Data used to identify the table
- `columns` · collection · read-only — Columns within the table

## SpreadsheetTableID

Data used to identify a spreadsheet table

- `type` · enum · {GEOMETRY} · read-only — The type of the table identifier

## SpreadsheetTableIDGeometry

- `object_eval_state` · enum · {EVALUATED, ORIGINAL, VIEWER_NODE} · read-only
- `geometry_component_type` · enum · {MESH, POINTCLOUD, CURVE, INSTANCES, GREASEPENCIL, VOLUME} · read-only — Part of the geometry to display data from
- `attribute_domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} · read-only — Attribute domain to display
- `geometry_item_type` · enum · {DOMAIN, BUNDLE} · read-only — Item Type
- `viewer_path` · pointer · -> ViewerPath · read-only — Path to the data that is displayed
- `layer_index` · int · read-only — Index of the Grease Pencil layer

## SpreadsheetTables

Active table and persisted state of previously displayed tables

- `active` · pointer · -> SpreadsheetTable · read-only

## Stereo3dDisplay

Settings for stereo 3D display

- `display_mode` · enum · {ANAGLYPH, INTERLACE, TIMESEQUENTIAL, SIDEBYSIDE, TOPBOTTOM}
- `anaglyph_type` · enum · {RED_CYAN, GREEN_MAGENTA, YELLOW_BLUE}
- `interlace_type` · enum · {ROW_INTERLEAVED, COLUMN_INTERLEAVED, CHECKERBOARD_INTERLEAVED}
- `use_interlace_swap` · boolean — Swap left and right stereo channels
- `use_sidebyside_crosseyed` · boolean — Right eye should see left image and vice versa

## Stereo3dFormat

Settings for stereo output

- `display_mode` · enum · {ANAGLYPH, INTERLACE, SIDEBYSIDE, TOPBOTTOM}
- `anaglyph_type` · enum · {RED_CYAN, GREEN_MAGENTA, YELLOW_BLUE}
- `interlace_type` · enum · {ROW_INTERLEAVED, COLUMN_INTERLEAVED, CHECKERBOARD_INTERLEAVED}
- `use_interlace_swap` · boolean — Swap left and right stereo channels
- `use_sidebyside_crosseyed` · boolean — Right eye should see left image and vice versa
- `use_squeezed_frame` · boolean — Combine both views in a squeezed image

## StringAttribute

Geometry attribute that stores strings

- `data` · collection · read-only

## StringAttributeValue

String value in geometry attribute

- `value` · string

## StringProperty

RNA text string property definition

- `default` · string · read-only — String default value
- `length_max` · int · read-only — Maximum length of the string, 0 means unlimited

## Struct

RNA structure definition

- `name` · string · read-only — Human readable name
- `identifier` · string · read-only — Unique name used in the code and scripting
- `description` · string · read-only — Description of the Struct's purpose
- `translation_context` · string · read-only — Translation context of the struct's name
- `base` · pointer · -> Struct · read-only — Struct definition this is derived from
- `nested` · pointer · -> Struct · read-only — Struct in which this struct is always nested, and to which it logically belongs
- `name_property` · pointer · -> StringProperty · read-only — Property that gives the name of the struct
- `properties` · collection · read-only — Properties in the struct
- `functions` · collection · read-only
- `property_tags` · collection · read-only — Tags that properties can use to influence behavior

## TexMapping

Texture coordinate mapping settings

- `vector_type` · enum · {POINT, TEXTURE, VECTOR, NORMAL} — Type of vector that the mapping transforms
- `translation` · float · [3]
- `rotation` · float · [3]
- `scale` · float · [3]
- `min` · float · [3] — Minimum value for clipping
- `max` · float · [3] — Maximum value for clipping
- `use_min` · boolean — Whether to use minimum clipping value
- `use_max` · boolean — Whether to use maximum clipping value
- `mapping_x` · enum · {NONE, X, Y, Z}
- `mapping_y` · enum · {NONE, X, Y, Z}
- `mapping_z` · enum · {NONE, X, Y, Z}
- `mapping` · enum · {FLAT, CUBE, TUBE, SPHERE}

## TexPaintSlot

Slot that contains information about texture painting

- `name` · string · read-only — Name of the slot
- `icon_value` · int · read-only — Paint slot icon
- `uv_layer` · string — Name of UV map
- `is_valid` · boolean · read-only — Slot has a valid image and UV map

## Text

Text data-block referencing an external or packed text file

- `filepath` · string — Filename of the text file
- `is_dirty` · boolean · read-only — Text file has been edited since last save
- `is_modified` · boolean · read-only — Text file on disk is different than the one in memory
- `is_in_memory` · boolean · read-only — Text file is in memory, without a corresponding file on disk
- `use_module` · boolean — Run this text as a Python script on loading
- `indentation` · enum · {TABS, SPACES} — Use tabs or spaces for indentation
- `lines` · collection · read-only — Lines of text
- `current_line` · pointer · -> TextLine · read-only — Current line, and start line of selection if one exists
- `current_character` · int — Index of current character in current line, and also start index of character in selection if one exists
- `current_line_index` · int — Index of current TextLine in TextLine collection
- `select_end_line` · pointer · -> TextLine · read-only — End line of selection
- `select_end_line_index` · int — Index of last TextLine in selection
- `select_end_character` · int — Index of character after end of selection in the selection end line

## TextBox

Text bounding box for layout

- `x` · float
- `y` · float
- `width` · float
- `height` · float

## TextCharacterFormat

Text character formatting settings

- `use_bold` · boolean
- `use_italic` · boolean
- `use_underline` · boolean
- `use_small_caps` · boolean
- `material_index` · int — Material slot index of this character
- `kerning` · float — Spacing between characters

## TextLine

Line of text in a Text data-block

- `body` · string — Text in the line

## TextboxState

- `visible_lines` · int · read-only
- `scroll` · int · read-only

## Theme

User interface styling and color settings

- `name` · string — Name of the theme
- `filepath` · string — The path to the preset loaded into this theme (if any)
- `theme_area` · enum · {USER_INTERFACE, STYLE, REGIONS, COMMON, VIEW_3D, DOPESHEET_EDITOR, FILE_BROWSER, GRAPH_EDITOR, IMAGE_EDITOR, INFO, CLIP_EDITOR, NODE_EDITOR, NLA_EDITOR, OUTLINER, PREFERENCES, PROPERTIES, CONSOLE, SPREADSHEET, STATUSBAR, TEXT_EDITOR, TOPBAR, SEQUENCE_EDITOR, BONE_COLOR_SETS}
- `user_interface` · pointer · -> ThemeUserInterface · read-only
- `regions` · pointer · -> ThemeRegions · read-only — Theme properties for common editor regions
- `common` · pointer · -> ThemeCommon · read-only — Theme properties shared by different editors
- `view_3d` · pointer · -> ThemeView3D · read-only
- `graph_editor` · pointer · -> ThemeGraphEditor · read-only
- `file_browser` · pointer · -> ThemeFileBrowser · read-only
- `nla_editor` · pointer · -> ThemeNLAEditor · read-only
- `dopesheet_editor` · pointer · -> ThemeDopeSheet · read-only
- `image_editor` · pointer · -> ThemeImageEditor · read-only
- `sequence_editor` · pointer · -> ThemeSequenceEditor · read-only
- `properties` · pointer · -> ThemeProperties · read-only
- `text_editor` · pointer · -> ThemeTextEditor · read-only
- `node_editor` · pointer · -> ThemeNodeEditor · read-only
- `outliner` · pointer · -> ThemeOutliner · read-only
- `info` · pointer · -> ThemeInfo · read-only
- `preferences` · pointer · -> ThemePreferences · read-only
- `console` · pointer · -> ThemeConsole · read-only
- `clip_editor` · pointer · -> ThemeClipEditor · read-only
- `topbar` · pointer · -> ThemeTopBar · read-only
- `statusbar` · pointer · -> ThemeStatusBar · read-only
- `spreadsheet` · pointer · -> ThemeSpreadsheet · read-only
- `bone_color_sets` · collection · read-only
- `collection_color` · collection · read-only
- `strip_color` · collection · read-only

## ThemeClipEditor

Theme settings for the Movie Clip Editor

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `grid` · float · [4]
- `marker_outline` · float · [3] — Color of marker's outline
- `marker` · float · [3] — Color of marker
- `active_marker` · float · [3] — Color of active marker
- `selected_marker` · float · [3] — Color of selected marker
- `disabled_marker` · float · [3] — Color of disabled marker
- `locked_marker` · float · [3] — Color of locked marker
- `path_before` · float · [3] — Color of path before current frame
- `path_after` · float · [3] — Color of path after current frame
- `path_keyframe_before` · float · [3] — Color of keyframes on a path before current frame
- `path_keyframe_after` · float · [3] — Color of keyframes on a path after current frame
- `metadatabg` · float · [3]
- `metadatatext` · float · [3]

## ThemeCollectionColor

Theme settings for collection colors

- `color` · float · [3] — Collection Color Tag

## ThemeCommon

Theme properties shared by different editors

- `anim` · pointer · -> ThemeCommonAnim · read-only
- `curves` · pointer · -> ThemeCommonCurves · read-only

## ThemeCommonAnim

Shared animation theme properties

- `playhead` · float · [3]
- `preview_range` · float · [4] — Color of preview range overlay
- `scene_strip_range` · float · [4] — Color of scene strip range overlay
- `channels` · float · [4]
- `channels_sub` · float · [4]
- `channel_group` · float · [4]
- `channel_group_active` · float · [4]
- `channel` · float · [4]
- `channel_selected` · float · [4]
- `keyframe` · float · [3] — Color of regular keyframe
- `keyframe_selected` · float · [3] — Color of selected keyframe
- `keyframe_extreme` · float · [3] — Color of extreme keyframe
- `keyframe_extreme_selected` · float · [3] — Color of selected extreme keyframe
- `keyframe_breakdown` · float · [3] — Color of breakdown keyframe
- `keyframe_breakdown_selected` · float · [3] — Color of selected breakdown keyframe
- `keyframe_jitter` · float · [3] — Color of jitter keyframe
- `keyframe_jitter_selected` · float · [3] — Color of selected jitter keyframe
- `keyframe_moving_hold` · float · [3] — Color of moving hold keyframe
- `keyframe_moving_hold_selected` · float · [3] — Color of selected moving hold keyframe
- `keyframe_generated` · float · [3] — Color of generated keyframe
- `keyframe_generated_selected` · float · [3] — Color of selected generated keyframe
- `long_key` · float · [4]
- `long_key_selected` · float · [4]

## ThemeConsole

Theme settings for the Console

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `line_output` · float · [3]
- `line_input` · float · [3]
- `line_info` · float · [3]
- `line_error` · float · [3]
- `cursor` · float · [3]
- `select` · float · [4]

## ThemeDopeSheet

Theme settings for the Dope Sheet

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `grid` · float · [3]
- `keyframe_border` · float · [4] — Color of keyframe border
- `keyframe_border_selected` · float · [4] — Color of selected keyframe border
- `keyframe_scale_factor` · float — Scale factor for adjusting the height of keyframes
- `summary` · float · [4] — Color of summary channel
- `anim_interpolation_linear` · float · [4] — Color of lines showing linear interpolation mode
- `anim_interpolation_constant` · float · [4] — Color of lines showing constant interpolation mode
- `anim_interpolation_other` · float · [4] — Color of lines showing easings & dynamic interpolation mode
- `simulated_frames` · float · [4]

## ThemeFileBrowser

Theme settings for the File Browser

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `selected_file` · float · [3]
- `row_alternate` · float · [4] — Overlay color on every other row

## ThemeFontStyle

Theme settings for Font

- `points` · float — Font size in points
- `character_weight` · int — Weight of the characters. 100-900, 400 is normal.
- `shadow` · int — Shadow type (0 none, 3, 5 blur, 6 outline)
- `shadow_offset_x` · int — Shadow offset in pixels
- `shadow_offset_y` · int — Shadow offset in pixels
- `shadow_alpha` · float
- `shadow_value` · float — Shadow color in gray value

## ThemeGradientColors

Theme settings for background colors and gradient

- `background_type` · enum · {SINGLE_COLOR, LINEAR, RADIAL} — Type of background in the 3D viewport
- `high_gradient` · float · [3]
- `gradient` · float · [3]

## ThemeGraphEditor

Theme settings for the graph editor

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `grid` · float · [3]
- `vertex` · float · [3]
- `vertex_select` · float · [3]
- `vertex_active` · float · [3]
- `vertex_size` · int

## ThemeInfo

Theme settings for Info

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `info_selected` · float · [3] — Background color of selected line
- `info_selected_text` · float · [3] — Text color of selected line
- `info_error_text` · float · [3] — Foreground color of Error icon
- `info_warning_text` · float · [3] — Foreground color of Warning icon
- `info_info_text` · float · [3] — Foreground color of Info icon
- `info_debug` · float · [4] — Background color of Debug icon
- `info_debug_text` · float · [3] — Foreground color of Debug icon
- `info_property` · float · [4] — Background color of Property icon
- `info_property_text` · float · [3] — Foreground color of Property icon
- `info_operator` · float · [4] — Background color of Operator icon
- `info_operator_text` · float · [3] — Foreground color of Operator icon

## ThemeOutliner

Theme settings for the Outliner

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `match` · float · [3]
- `selected_highlight` · float · [3]
- `active` · float · [3]
- `selected_object` · float · [3]
- `active_object` · float · [3]
- `edited_object` · float · [4]
- `row_alternate` · float · [4] — Overlay color on every other row

## ThemeProperties

Theme settings for the Properties

- `match` · float · [3]
- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space

## ThemeRegions

Theme settings for regions that are common among editors

- `asset_shelf` · pointer · -> ThemeRegionsAssetShelf · read-only
- `channels` · pointer · -> ThemeRegionsChannels · read-only
- `scrubbing` · pointer · -> ThemeRegionsScrubbing · read-only
- `sidebars` · pointer · -> ThemeRegionsSidebars · read-only

## ThemeRegionsAssetShelf

Theme settings for asset shelves

- `back` · float · [4]
- `header_back` · float · [4]

## ThemeRegionsChannels

- `back` · float · [3]
- `text` · float · [3]
- `text_selected` · float · [3]

## ThemeRegionsScrubbing

- `back` · float · [4]
- `text` · float · [3]
- `time_marker` · float · [4]
- `time_marker_selected` · float · [4]

## ThemeRegionsSidebars

- `back` · float · [4]
- `tab_back` · float · [4]

## ThemeSpreadsheet

Theme settings for the Spreadsheet

- `row_alternate` · float · [4] — Overlay color on every other row
- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space

## ThemeStatusBar

Theme settings for the Status Bar

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space

## ThemeStyle

Theme settings for style sets

- `panel_title` · pointer · -> ThemeFontStyle · read-only
- `widget` · pointer · -> ThemeFontStyle · read-only
- `tooltip` · pointer · -> ThemeFontStyle · read-only

## ThemeTextEditor

Theme settings for the Text Editor

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `line_numbers` · float · [3]
- `line_numbers_background` · float · [3]
- `selected_text` · float · [3]
- `cursor` · float · [3]
- `syntax_builtin` · float · [3]
- `syntax_symbols` · float · [3]
- `syntax_special` · float · [3]
- `syntax_preprocessor` · float · [3]
- `syntax_reserved` · float · [3]
- `syntax_comment` · float · [3]
- `syntax_string` · float · [3]
- `syntax_numbers` · float · [3]

## ThemeTopBar

Theme settings for the Top Bar

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space

## ThemeUserInterface

Theme settings for user interface elements

- `wcol_regular` · pointer · -> ThemeWidgetColors · read-only
- `wcol_tool` · pointer · -> ThemeWidgetColors · read-only
- `wcol_toolbar_item` · pointer · -> ThemeWidgetColors · read-only
- `wcol_radio` · pointer · -> ThemeWidgetColors · read-only
- `wcol_text` · pointer · -> ThemeWidgetColors · read-only
- `wcol_option` · pointer · -> ThemeWidgetColors · read-only
- `wcol_toggle` · pointer · -> ThemeWidgetColors · read-only
- `wcol_num` · pointer · -> ThemeWidgetColors · read-only
- `wcol_numslider` · pointer · -> ThemeWidgetColors · read-only
- `wcol_box` · pointer · -> ThemeWidgetColors · read-only
- `wcol_curve` · pointer · -> ThemeWidgetColors · read-only
- `wcol_menu` · pointer · -> ThemeWidgetColors · read-only
- `wcol_pulldown` · pointer · -> ThemeWidgetColors · read-only
- `wcol_menu_back` · pointer · -> ThemeWidgetColors · read-only
- `wcol_pie_menu` · pointer · -> ThemeWidgetColors · read-only
- `wcol_tooltip` · pointer · -> ThemeWidgetColors · read-only
- `wcol_menu_item` · pointer · -> ThemeWidgetColors · read-only
- `wcol_scroll` · pointer · -> ThemeWidgetColors · read-only
- `wcol_progress` · pointer · -> ThemeWidgetColors · read-only
- `wcol_list_item` · pointer · -> ThemeWidgetColors · read-only
- `wcol_state` · pointer · -> ThemeWidgetStateColors · read-only
- `wcol_tab` · pointer · -> ThemeWidgetColors · read-only
- `menu_shadow_fac` · float — Blending factor for panel and menu shadows
- `menu_shadow_width` · int — Width of panel and menu shadows, set to zero to disable
- `icon_alpha` · float — Transparency of icons in the interface, to reduce contrast
- `icon_saturation` · float — Saturation of icons in the interface
- `widget_emboss` · float · [4] — Color of the 1px shadow line underlying widgets
- `link` · float · [4] — Color of link widgets
- `editor_border` · float · [3] — Color of the border between editors
- `editor_outline` · float · [4] — Color of the outline of each editor, except the active one
- `editor_outline_active` · float · [4] — Color of the outline of the active editor
- `widget_text_cursor` · float · [3] — Color of the text insertion cursor (caret)
- `panel_roundness` · float — Roundness of the corners of panels and sub-panels
- `panel_header` · float · [4]
- `panel_title` · float · [3]
- `panel_text` · float · [3]
- `panel_back` · float · [4]
- `panel_sub_back` · float · [4]
- `panel_outline` · float · [4] — Color of the outline of top-level panels
- `panel_active` · float · [4] — Color of the outline of top-level panels that are active
- `transparent_checker_primary` · float · [3] — Primary color of checkerboard pattern indicating transparent areas
- `transparent_checker_secondary` · float · [3] — Secondary color of checkerboard pattern indicating transparent areas
- `transparent_checker_size` · int — Size of checkerboard pattern indicating transparent areas
- `axis_x` · float · [3]
- `axis_y` · float · [3]
- `axis_z` · float · [3]
- `axis_w` · float · [3] — W-axis for quaternion and axis-angle rotations
- `gizmo_hi` · float · [3]
- `gizmo_primary` · float · [3]
- `gizmo_secondary` · float · [3]
- `gizmo_view_align` · float · [3]
- `gizmo_a` · float · [3]
- `gizmo_b` · float · [3]
- `icon_scene` · float · [4]
- `icon_collection` · float · [4]
- `icon_object` · float · [4]
- `icon_object_data` · float · [4]
- `icon_modifier` · float · [4]
- `icon_shading` · float · [4]
- `icon_folder` · float · [4] — Color of folders in the file browser
- `icon_autokey` · float · [4] — Color of Auto Keying indicator when enabled
- `icon_border_intensity` · float — Control the intensity of the border around themes icons

## ThemeWidgetColors

Theme settings for widget color sets

- `outline` · float · [4]
- `outline_sel` · float · [4]
- `inner` · float · [4]
- `inner_sel` · float · [4]
- `item` · float · [4]
- `text` · float · [3]
- `text_sel` · float · [3]
- `show_shaded` · boolean
- `shadetop` · int
- `shadedown` · int
- `roundness` · float — Amount of edge rounding

## ThemeWidgetStateColors

Theme settings for widget state colors

- `error` · float · [4] — Color for error items
- `warning` · float · [4] — Color for warning items
- `info` · float · [4] — Color for informational items
- `success` · float · [4] — Color for successful items
- `inner_anim` · float · [3]
- `inner_anim_sel` · float · [3]
- `inner_key` · float · [3]
- `inner_key_sel` · float · [3]
- `inner_driven` · float · [3]
- `inner_driven_sel` · float · [3]
- `inner_overridden` · float · [3]
- `inner_overridden_sel` · float · [3]
- `inner_changed` · float · [3]
- `inner_changed_sel` · float · [3]
- `blend` · float

## TimelineMarker

Marker for noting points in the timeline

- `name` · string
- `frame` · int — The frame on which the timeline marker appears
- `select` · boolean — Marker selection state
- `camera` · pointer · -> Object — Camera that becomes active on this frame

## Timer

Window event timer

- `time_step` · float · read-only
- `time_delta` · float · read-only — Time since last step in seconds
- `time_duration` · float · read-only — Time since the timer started seconds

## ToolSettings

- `sculpt` · pointer · -> Sculpt · read-only
- `curves_sculpt` · pointer · -> CurvesSculpt · read-only
- `use_auto_normalize` · boolean — Ensure all bone-deforming vertex groups add up to 1.0 while weight painting or assigning to vertices
- `use_lock_relative` · boolean — Display bone-deforming groups as if all locked deform groups were deleted, and the remaining ones were re-normalized
- `use_multipaint` · boolean — Paint across the weights of all selected bones, maintaining their relative influence
- `vertex_group_user` · enum · {NONE, ACTIVE, ALL} — Display unweighted vertices
- `vertex_group_subset` · enum · {ALL, BONE_DEFORM, OTHER_DEFORM} — Filter Vertex groups for Display
- `vertex_paint` · pointer · -> VertexPaint · read-only
- `weight_paint` · pointer · -> VertexPaint · read-only
- `image_paint` · pointer · -> ImagePaint · read-only
- `paint_mode` · pointer · -> PaintModeSettings · read-only
- `uv_sculpt` · pointer · -> UvSculpt · read-only
- `gpencil_paint` · pointer · -> GpPaint · read-only
- `gpencil_vertex_paint` · pointer · -> GpVertexPaint · read-only
- `gpencil_sculpt_paint` · pointer · -> GpSculptPaint · read-only
- `gpencil_weight_paint` · pointer · -> GpWeightPaint · read-only
- `particle_edit` · pointer · -> ParticleEdit · read-only
- `uv_sculpt_lock_borders` · boolean — Disable editing of boundary edges
- `uv_sculpt_all_islands` · boolean — Brush operates on all islands
- `lock_object_mode` · boolean — Restrict selection to objects using the same mode as the active object, to prevent accidental mode switch when selecting
- `workspace_tool_type` · enum · {DEFAULT, FALLBACK} — Action when dragging in the viewport
- `use_proportional_edit` · boolean — Proportional edit mode
- `use_proportional_edit_objects` · boolean — Proportional editing object mode
- `use_proportional_projected` · boolean — Proportional Editing using screen space locations
- `use_proportional_connected` · boolean — Proportional Editing using connected geometry only
- `use_proportional_edit_mask` · boolean — Proportional editing mask mode
- `use_proportional_action` · boolean — Proportional editing in action editor
- `use_proportional_fcurve` · boolean — Proportional editing in F-Curve editor
- `lock_markers` · boolean — Prevent marker editing
- `proportional_edit_falloff` · enum · {SMOOTH, SPHERE, ROOT, INVERSE_SQUARE, SHARP, LINEAR, CONSTANT, RANDOM} — Falloff type for proportional editing mode
- `proportional_size` · float — Display size for proportional editing circle
- `proportional_distance` · float — Display size for proportional editing circle
- `double_threshold` · float — Threshold distance for Auto Merge
- `transform_pivot_point` · enum · {BOUNDING_BOX_CENTER, CURSOR, INDIVIDUAL_ORIGINS, MEDIAN_POINT, ACTIVE_ELEMENT} — Pivot center for rotation/scaling
- `use_transform_pivot_point_align` · boolean — Only transform object locations, without affecting rotation or scaling
- `use_transform_data_origin` · boolean — Transform object origins, while leaving the shape in place
- `use_transform_skip_children` · boolean — Transform the parents, leaving the children in place
- `use_transform_correct_face_attributes` · boolean — Correct data such as UVs and color attributes when transforming
- `use_transform_correct_keep_connected` · boolean — During the Face Attributes correction, merge attributes connected to the same vertex
- `use_mesh_automerge` · boolean — Automatically merge vertices moved to the same location
- `use_mesh_automerge_and_split` · boolean — Automatically split edges and faces
- `use_snap` · boolean — Snap during transform
- `use_snap_node` · boolean — Snap Node during transform
- `use_snap_sequencer` · boolean — Snap strips during transform
- `use_snap_uv` · boolean — Snap UV during transform
- `use_snap_align_rotation` · boolean — Align rotation with the snapping target
- `use_snap_grid_absolute` · boolean — Absolute grid alignment while translating (based on the pivot center)
- `snap_angle_increment_2d` · float — Angle used for rotation increments in 2D editors
- `snap_angle_increment_2d_precision` · float — Precision angle used for rotation increments in 2D editors
- `snap_angle_increment_3d` · float — Angle used for rotation increments in 3D editors
- `snap_angle_increment_3d_precision` · float — Precision angle used for rotation increments in 3D editors
- `snap_elements` · enum · {INCREMENT, GRID, VERTEX, EDGE, FACE, VOLUME, EDGE_MIDPOINT, EDGE_PERPENDICULAR, FACE_MIDPOINT, FACE_PROJECT, FACE_NEAREST} — Type of element to snap to
- `snap_elements_base` · enum · {INCREMENT, GRID, VERTEX, EDGE, FACE, VOLUME, EDGE_MIDPOINT, EDGE_PERPENDICULAR, FACE_MIDPOINT} — Type of element for the "Snap Base" to snap to
- `snap_elements_individual` · enum · {FACE_PROJECT, FACE_NEAREST} — Type of element for individual transformed elements to snap to
- `snap_face_nearest_steps` · int — Number of steps to break transformation into for face nearest snapping
- `use_snap_to_same_target` · boolean — Snap only to target that source was initially near ("Face Nearest" only)
- `use_snap_anim` · boolean — Enable snapping when transforming keyframes
- `use_snap_driver` · boolean — Enable snapping when transforming keys in the Driver Editor
- `use_snap_time_absolute` · boolean — Absolute time alignment when transforming keyframes
- `use_snap_driver_absolute` · boolean — Snap to full values
- `snap_anim_element` · enum · {FRAME, SECOND, MARKER} — Type of element to snap to
- `use_snap_playhead` · boolean — Snap playhead when scrubbing
- `snap_playhead_element` · enum · {FRAME, SECOND, MARKER, KEY, Strip} — Type of element to snap to
- `snap_playhead_frame_step` · int — At which interval to snap to frames
- `snap_playhead_second_step` · int — At which interval to snap to seconds
- `playhead_snap_distance` · int — Maximum distance for snapping in pixels
- `snap_uv_element` · enum · {INCREMENT, GRID, VERTEX} — Type of element to snap to
- `snap_target` · enum · {CLOSEST, CENTER, MEDIAN, ACTIVE} — Which part to snap onto the target
- `use_snap_peel_object` · boolean — Consider objects as whole when finding volume center
- `use_snap_backface_culling` · boolean — Exclude back facing geometry from snapping
- `use_snap_self` · boolean — Snap onto itself only if enabled (edit mode only)
- `use_snap_edit` · boolean — Snap onto non-active objects in edit mode (edit mode only)
- `use_snap_nonedit` · boolean — Snap onto objects not in edit mode (edit mode only)
- `use_snap_selectable` · boolean — Snap only onto objects that are selectable
- `use_snap_translate` · boolean — Move is affected by snapping settings
- `use_snap_rotate` · boolean — Rotate is affected by the snapping settings
- `use_snap_scale` · boolean — Scale is affected by snapping settings
- `plane_axis` · enum · {X, Y, Z} — The axis used for placing the base region
- `plane_axis_auto` · boolean — Select the closest axis when placing objects (surface overrides)
- `plane_depth` · enum · {SURFACE, CURSOR_PLANE, CURSOR_VIEW} — The initial depth used when placing the cursor
- `plane_orientation` · enum · {SURFACE, DEFAULT} — The initial depth used when placing the cursor
- `snap_elements_tool` · enum · {GEOMETRY, DEFAULT} — The target to use while snapping
- `use_gpencil_draw_additive` · boolean — When creating new frames, the strokes from the previous/active frame are included as the basis for the new one
- `use_gpencil_draw_onback` · boolean — New strokes are drawn below of all strokes in the layer
- `use_gpencil_thumbnail_list` · boolean — Show compact list of colors instead of thumbnails
- `use_gpencil_weight_data_add` · boolean — Weight data for new strokes is added according to the current vertex group and weight. If no vertex group selected, weight is not added.
- `use_gpencil_automerge_strokes` · boolean — Join the last drawn stroke with previous strokes in the active layer by distance
- `gpencil_sculpt` · pointer · -> GPencilSculptSettings · read-only — Settings for stroke sculpting tools and brushes
- `gpencil_interpolate` · pointer · -> GPencilInterpolateSettings · read-only — Settings for Grease Pencil interpolation tools
- `gpencil_stroke_placement_view3d` · enum · {ORIGIN, CURSOR, SURFACE, STROKE}
- `gpencil_stroke_snap_mode` · enum · {NONE, ENDS, FIRST}
- `gpencil_surface_offset` · float — Offset along the normal when drawing on surfaces
- `use_gpencil_project_only_selected` · boolean — Project the strokes only onto selected objects
- `gpencil_selectmode_edit` · enum · {POINT, STROKE, SEGMENT}
- `use_gpencil_select_mask_point` · boolean — Only sculpt selected stroke points
- `use_gpencil_select_mask_stroke` · boolean — Only sculpt selected strokes
- `use_gpencil_select_mask_segment` · boolean — Only sculpt selected stroke points between other strokes
- `use_gpencil_vertex_select_mask_point` · boolean — Only paint selected stroke points
- `use_gpencil_vertex_select_mask_stroke` · boolean — Only paint selected strokes
- `use_gpencil_vertex_select_mask_segment` · boolean — Only paint selected stroke points between other strokes
- `use_grease_pencil_multi_frame_editing` · boolean — Enable multi-frame editing
- `annotation_stroke_placement_view2d` · enum · {IMAGE, VIEW}
- `annotation_stroke_placement_view3d` · enum · {CURSOR, VIEW, SURFACE} — How annotation strokes are orientated in 3D space
- `use_annotation_stroke_endpoints` · boolean — Only use the first and last parts of the stroke for snapping
- `use_annotation_project_only_selected` · boolean — Project the strokes only onto selected objects
- `annotation_thickness` · int — Thickness of annotation strokes
- `use_keyframe_insert_auto` · boolean — Automatically insert keyframes on modified properties
- `auto_keying_mode` · enum · {ADD_REPLACE_KEYS, REPLACE_KEYS} — Can add additional constraints on when auto keying can insert keyframes
- `use_record_with_nla` · boolean — Add a new NLA Track + Strip for every loop/pass made over the animation to allow non-destructive tweaking
- `use_keyframe_insert_keyingset` · boolean — Automatic keyframe insertion using active Keying Set only
- `use_keyframe_cycle_aware` · boolean — For channels with cyclic extrapolation, keyframe insertion is automatically remapped inside the cycle time range, and keeps ends in sync. Curves newly added to actions with a Manual Frame Range and Cyclic Animation are automatically made cyclic.
- `keyframe_type` · enum · {KEYFRAME, BREAKDOWN, MOVING_HOLD, EXTREME, JITTER, GENERATED} — Type of keyframes to create when inserting keyframes
- `anim_mirror_object` · pointer · -> Object — Object to mirror over. Leave empty and name a bone to always mirror over that bone of the active armature
- `anim_mirror_bone` · string — Bone to use for the mirroring
- `anim_relative_object` · pointer · -> Object — Object to which matrices are made relative
- `anim_fix_to_cam_use_loc` · boolean — Create location keys when fixing to the scene camera
- `anim_fix_to_cam_use_rot` · boolean — Create rotation keys when fixing to the scene camera
- `anim_fix_to_cam_use_scale` · boolean — Create scale keys when fixing to the scene camera
- `uv_select_mode` · enum · {VERTEX, EDGE, FACE} — UV selection and display mode
- `uv_sticky_select_mode` · enum · {DISABLED, SHARED_LOCATION, SHARED_VERTEX} — Method for extending UV vertex selection
- `use_uv_select_sync` · boolean — Keep UV and edit mode mesh selection in sync
- `use_uv_select_island` · boolean — Island selection
- `show_uv_local_view` · boolean — Display only faces with the currently displayed image assigned
- `use_uv_custom_region` · boolean — Custom defined region
- `mesh_select_mode` · boolean — Which mesh elements selection works on
- `vertex_group_weight` · float — Weight to assign in vertex groups
- `use_edge_path_live_unwrap` · boolean — Changing edge seams recalculates UV unwrap
- `normal_vector` · float · [3] — Normal vector used to copy, add or multiply
- `curve_paint_settings` · pointer · -> CurvePaintSettings · read-only
- `statvis` · pointer · -> MeshStatVis · read-only
- `custom_bevel_profile_preset` · pointer · -> CurveProfile · read-only — Used for defining a profile's path
- `sequencer_tool_settings` · pointer · -> SequencerToolSettings · read-only

## TransformOrientation

- `matrix` · float · [9]
- `name` · string — Name of the custom transform orientation

## TransformOrientationSlot

- `type` · enum · {GLOBAL, LOCAL, NORMAL, GIMBAL, VIEW, CURSOR, PARENT} — Transformation orientation
- `custom_orientation` · pointer · -> TransformOrientation · read-only
- `use` · boolean — Use scene orientation instead of a custom setting

## UDIMTile

Properties of the UDIM tile

- `label` · string — Tile label
- `number` · int — Number of the position that this tile covers
- `size` · int · [2] · read-only — Width and height of the tile buffer in pixels, zero when image data cannot be loaded
- `channels` · int · read-only — Number of channels in the tile pixels buffer
- `generated_type` · enum · {BLANK, UV_GRID, COLOR_GRID} — Generated image type
- `generated_width` · int — Generated image width
- `generated_height` · int — Generated image height
- `use_generated_float` · boolean — Generate floating-point buffer
- `is_generated_tile` · boolean · read-only — Is this image tile generated
- `generated_color` · float · [4] — Fill color for the generated image

## UDIMTiles

Collection of UDIM tiles

- `active_index` · int — Active index in tiles array
- `active` · pointer · -> UDIMTile — Active Image Tile

## USDHook

Defines callback functions to extend USD IO

- `bl_idname` · string
- `bl_label` · string
- `bl_description` · string — A short description of the USD hook

## UVLoopLayers

Collection of UV map layers

- `active` · pointer · -> MeshUVLoopLayer — Active UV Map layer
- `active_index` · int — Active UV map index
- `active_render` · pointer · -> MeshUVLoopLayer — Active Render UV Map layer
- `active_render_index` · int — Active Render UV map index

## UVProjector

UV projector used by the UV project modifier

- `object` · pointer · -> Object — Object to use as projector transform

## UnifiedPaintSettings

Overrides for some of the active brush's settings

- `use_unified_size` · boolean — Instead of per-brush size, the size is shared across brushes
- `use_unified_strength` · boolean — Instead of per-brush strength, the strength is shared across brushes
- `use_unified_weight` · boolean — Instead of per-brush weight, the weight is shared across brushes
- `use_unified_color` · boolean — Instead of per-brush color, the color is shared across brushes
- `use_unified_input_samples` · boolean — Instead of per-brush input samples, the value is shared across brushes
- `size` · int — Diameter of the brush
- `unprojected_size` · float — Diameter of brush in Blender units
- `strength` · float — How powerful the effect of the brush is when applied
- `weight` · float — Weight to assign in vertex groups
- `color` · float · [3]
- `secondary_color` · float · [3]
- `use_color_jitter` · boolean — Jitter brush color
- `hue_jitter` · float — Color jitter effect on hue
- `saturation_jitter` · float — Color jitter effect on saturation
- `value_jitter` · float — Color jitter effect on value
- `use_stroke_random_hue` · boolean — Use randomness at stroke level
- `use_stroke_random_sat` · boolean — Use randomness at stroke level
- `use_stroke_random_val` · boolean — Use randomness at stroke level
- `use_random_press_hue` · boolean — Use pressure to modulate randomness
- `use_random_press_sat` · boolean — Use pressure to modulate randomness
- `use_random_press_val` · boolean — Use pressure to modulate randomness
- `input_samples` · int — Number of input samples to average together to smooth the brush stroke
- `use_locked_size` · enum · {VIEW, SCENE} — Measure brush size relative to the view or the scene

## UnitSettings

- `system` · enum · {NONE, METRIC, IMPERIAL} — The unit system to use for user interface controls
- `system_rotation` · enum · {DEGREES, RADIANS} — Unit to use for displaying/editing rotation values
- `scale_length` · float — Scale to use when converting between Blender units and dimensions. When working at microscopic or astronomical scale, a small or large unit scale respectively can be used to avoid numerical precision problems
- `use_separate` · boolean — Display units in pairs (e.g. 1m 0cm)
- `length_unit` · enum · {DEFAULT} — Unit that will be used to display length values
- `mass_unit` · enum · {DEFAULT} — Unit that will be used to display mass values
- `time_unit` · enum · {DEFAULT} — Unit that will be used to display time values
- `temperature_unit` · enum · {DEFAULT} — Unit that will be used to display temperature values

## UserAssetLibrary

Settings to define a reusable library for Asset Browsers to use

- `name` · string — Identifier (not necessarily unique) for the asset library
- `path` · string — Path to a directory with .blend files to use as an asset library
- `enabled` · boolean — Enable the asset library
- `remote_url` · string — Remote URL to the asset library
- `import_method` · enum · {LINK, APPEND, APPEND_REUSE, PACK} — Determine how the asset will be imported, unless overridden by the Asset Browser
- `use_relative_path` · boolean — Use relative path when linking assets from this asset library
- `use_remote_url` · boolean · read-only — Synchronize the asset library with a remote URL

## UserExtensionRepo

Settings to define an extension repository

- `name` · string — Unique repository name
- `module` · string — Unique module identifier
- `custom_directory` · string — The local directory containing extensions
- `directory` · string · read-only — The local directory containing extensions
- `remote_url` · string — Remote URL to the extension repository, the file-system may be referenced using the file URI scheme: "file://"
- `access_token` · string — Personal access token, may be required by some repositories
- `source` · enum · {USER, SYSTEM} — Select if the repository is in a user managed or system provided directory
- `use_cache` · boolean — Downloaded package files are deleted after installation
- `enabled` · boolean — Enable the repository
- `use_sync_on_startup` · boolean — Allow Blender to check for updates upon launch
- `use_access_token` · boolean — Repository requires an access token
- `use_custom_directory` · boolean — Manually set the path for extensions to be stored. When disabled a user's extensions directory is created.
- `use_remote_url` · boolean — Synchronize the repository with a remote URL

## UvSculpt

- `size` · int
- `strength` · float
- `curve_distance_falloff` · pointer · -> CurveMapping · read-only
- `curve_distance_falloff_preset` · enum · {CUSTOM, SMOOTH, SMOOTHER, SPHERE, ROOT, SHARP, LIN, POW4, INVSQUARE, CONSTANT}

## VectorFont

Vector font for Text objects

- `filepath` · string
- `packed_file` · pointer · -> PackedFile · read-only

## VertexGroup

Group of vertices, used for armature deform and other purposes

- `name` · string — Vertex group name
- `lock_weight` · boolean — Maintain the relative weights for the group
- `index` · int · read-only — Index number of the vertex group

## VertexGroupElement

Weight value of a vertex in a vertex group

- `group` · int · read-only
- `weight` · float — Vertex Weight

## VertexGroups

Collection of vertex groups

- `active` · pointer · -> VertexGroup — Vertex groups of the object
- `active_index` · int — Active index in vertex group array

## VertexPaint

Properties of vertex and weight paint mode

- `use_group_restrict` · boolean — Restrict painting to vertices in the group

## Volume

Volume data-block for 3D volume grids

- `filepath` · string — Volume file used by this Volume data-block
- `packed_file` · pointer · -> PackedFile · read-only
- `is_sequence` · boolean — Whether the cache is separated in a series of files
- `frame_start` · int — Global starting frame of the sequence, assuming first has a #1
- `frame_duration` · int — Number of frames of the sequence to use
- `frame_offset` · int — Offset the number of the frame to use in the animation
- `sequence_mode` · enum · {CLIP, EXTEND, REPEAT, PING_PONG} — Sequence playback mode
- `grids` · collection · [] of VolumeGrids · read-only — 3D volume grids
- `materials` · collection · [] of IDMaterials · read-only
- `display` · pointer · -> VolumeDisplay · read-only — Volume display settings for 3D viewport
- `render` · pointer · -> VolumeRender · read-only — Volume render settings for 3D viewport
- `velocity_grid` · string — Name of the velocity field, or the base name if the velocity is split into multiple grids
- `velocity_unit` · enum · {SECOND, FRAME} — Define how the velocity vectors are interpreted with regard to time, 'frame' means the delta time is 1 frame, 'second' means the delta time is 1 / FPS
- `velocity_scale` · float — Factor to control the amount of motion blur
- `velocity_x_grid` · string · read-only — Name of the grid for the X axis component of the velocity field if it was split into multiple grids
- `velocity_y_grid` · string · read-only — Name of the grid for the Y axis component of the velocity field if it was split into multiple grids
- `velocity_z_grid` · string · read-only — Name of the grid for the Z axis component of the velocity field if it was split into multiple grids
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block

## VolumeDisplay

Volume object display settings for 3D viewport

- `density` · float — Thickness of volume display in the viewport
- `wireframe_type` · enum · {NONE, BOUNDS, BOXES, POINTS} — Type of wireframe display
- `wireframe_detail` · enum · {COARSE, FINE} — Amount of detail for wireframe display
- `interpolation_method` · enum · {LINEAR, CUBIC, CLOSEST} — Interpolation method to use for volumes in solid mode
- `use_slice` · boolean — Perform a single slice of the domain object
- `slice_axis` · enum · {AUTO, X, Y, Z}
- `slice_depth` · float — Position of the slice

## VolumeGrid

3D volume grid

- `name` · string · read-only — Volume grid name
- `data_type` · enum · {BOOLEAN, FLOAT, DOUBLE, INT, INT64, MASK, VECTOR_FLOAT, VECTOR_DOUBLE, VECTOR_INT, POINTS, UNKNOWN} · read-only — Data type of voxel values
- `channels` · int · read-only — Number of dimensions of the grid data type
- `matrix_object` · float · [16] · read-only — Transformation matrix from voxel index to object space
- `is_loaded` · boolean · read-only — Grid tree is loaded in memory

## VolumeGrids

3D volume grids

- `active_index` · int — Index of active volume grid
- `error_message` · string · read-only — If loading grids failed, error message with details
- `is_loaded` · boolean · read-only — List of grids and metadata are loaded in memory
- `frame` · int · read-only — Frame number that volume grids will be loaded at, based on scene time and volume parameters
- `frame_filepath` · string · read-only — Volume file used for loading the volume at the current frame. Empty if the volume has not be loaded or the frame only exists in memory.

## WalkNavigation

Walk navigation settings

- `mouse_speed` · float — Speed factor for when looking around, high values mean faster mouse movement
- `walk_speed` · float — Base speed for walking and flying
- `walk_speed_factor` · float — Multiplication factor when using the fast or slow modifiers
- `view_height` · float — View distance from the floor when walking
- `jump_height` · float — Maximum height of a jump
- `teleport_time` · float — Interval of time warp when teleporting in navigation mode
- `use_gravity` · boolean — Walk with gravity, or free navigate
- `use_mouse_reverse` · boolean — Reverse the vertical movement of the mouse

## Window

Open window

- `parent` · pointer · -> Window · read-only — Active workspace and scene follow this window
- `scene` · pointer · -> Scene — Active scene to be edited in the window
- `workspace` · pointer · -> WorkSpace — Active workspace showing in the window
- `screen` · pointer · -> Screen — Active workspace screen showing in the window
- `view_layer` · pointer · -> ViewLayer — The active workspace view layer showing in the window
- `x` · int · read-only — Horizontal location of the window
- `y` · int · read-only — Vertical location of the window
- `width` · int · read-only — Window width
- `height` · int · read-only — Window height
- `stereo_3d_display` · pointer · -> Stereo3dDisplay · read-only — Settings for stereo 3D display
- `support_hdr_color` · boolean · read-only — The window has a HDR graphics buffer that wide gamut and high dynamic range colors can be written to, in extended sRGB color space.
- `modal_operators` · collection · read-only — A list of currently running modal operators

## WindowManager

Window manager data-block defining open windows and other user interface data

- `operators` · collection · read-only — Operator registry
- `reports` · collection · read-only — Collection of reports
- `windows` · collection · [] of Windows · read-only — Open windows
- `keyconfigs` · collection · [] of KeyConfigurations · read-only — Registered key configurations
- `xr_session_settings` · pointer · -> XrSessionSettings · read-only
- `xr_session_state` · pointer · -> XrSessionState · read-only — Runtime state information about the VR session
- `extensions_updates` · int — Number of extensions with available update
- `extensions_blocked` · int — Number of installed extensions which are blocked
- `is_event_handling_break` · boolean · read-only — Remaining events in the queue are delayed until the next main loop iteration
- `is_interface_locked` · boolean · read-only — If true, the interface is currently locked by a running job and data should not be modified from application timers. Otherwise, the running job might conflict with the handler causing unexpected results or even crashes.
- `preset_name` · string — Name for new preset
- `asset_path_dummy` · string · read-only — Full path to the Blender file containing the active asset
- `addon_search` · string — Filter by add-on name, author & category
- `addon_filter` · enum · {} — Filter add-ons by category
- `addon_support` · enum · {OFFICIAL, COMMUNITY} — Display support level
- `addon_tags` · collection · read-only
- `extension_tags` · collection · read-only
- `extension_search` · string — Filter by extension name, author & category
- `extension_type` · enum · {ALL, ADDON, THEME} — Show extensions by type
- `extension_use_filter` · boolean — Filter Extensions by Tags & Repository
- `extension_show_panel_installed` · boolean — Show the installed extensions panel
- `extension_show_panel_available` · boolean — Show the available extensions panel
- `extension_repo_filter` · enum · {} — Filter extensions by repository
- `poselib_previous_action` · pointer · -> Action

## XrComponentPath

- `path` · string — OpenXR component path

## XrEventData

XR Data for Window Manager Event

- `action_set` · string · read-only — XR action set name
- `action` · string · read-only — XR action name
- `user_path` · string · read-only — User path of the action. E.g. "/user/hand/left"
- `user_path_other` · string · read-only — Other user path, for bimanual actions. E.g. "/user/hand/right"
- `type` · enum · {FLOAT, VECTOR2D, POSE, VIBRATION} · read-only — XR action type
- `state` · float · [2] · read-only — XR action values corresponding to type
- `state_other` · float · [2] · read-only — State of the other user path for bimanual actions
- `float_threshold` · float · read-only — Input threshold for float/2D vector actions
- `controller_location` · float · [3] · read-only — Location of the action's corresponding controller aim in world space
- `controller_rotation` · float · [4] · read-only — Rotation of the action's corresponding controller aim in world space
- `controller_location_other` · float · [3] · read-only — Controller aim location of the other user path for bimanual actions
- `controller_rotation_other` · float · [4] · read-only — Controller aim rotation of the other user path for bimanual actions
- `bimanual` · boolean · read-only — Whether bimanual interaction is occurring

## XrNavigation

VR navigation settings

- `vignette_intensity` · float — Intensity of vignette that appears when moving
- `turn_speed` · float — Turn speed in degrees per second
- `turn_amount` · float — Amount in degrees per turn when using snap turn
- `snap_turn` · boolean — Instantly rotates the camera by a fixed angle instead of smoothly turning
- `invert_rotation` · boolean — Reverses the direction of rotation input

## XrSessionSettings

- `shading` · pointer · -> View3DShading · read-only
- `base_pose_type` · enum · {SCENE_CAMERA, OBJECT, CUSTOM} — Define where the location and rotation for the VR view come from, to which translation and rotation deltas from the VR headset will be applied to
- `base_pose_object` · pointer · -> Object — Object to take the location and rotation to which translation and rotation deltas from the VR headset will be applied to
- `base_pose_location` · float · [3] — Coordinates to apply translation deltas from the VR headset to
- `base_pose_angle` · float — Rotation angle around the Z-Axis to apply the rotation deltas from the VR headset to
- `base_scale` · float — Uniform base pose scale to apply to VR view
- `show_floor` · boolean — Show the ground plane grid
- `show_passthrough` · boolean — Show the passthrough view
- `show_annotation` · boolean — Show annotations for this view
- `show_selection` · boolean — Show selection outlines
- `show_controllers` · boolean — Show VR controllers (requires VR actions for controller poses)
- `show_custom_overlays` · boolean — Show custom VR overlays
- `show_object_extras` · boolean — Show object extras, including empties, lights, and cameras
- `controller_draw_style` · enum · {DARK, LIGHT, DARK_RAY, LIGHT_RAY} — Style to use when drawing VR controllers
- `viewfinder_enabled` · boolean — Enable the Location Scouting Viewfinder
- `viewfinder_hand` · enum · {LEFT, RIGHT} — Hand on which to place the Location Scouting Viewfinder
- `viewfinder_scale` · float — Location Scouting Viewfinder size scale
- `viewfinder_crosshair_enabled` · boolean — Enable the Viewfinder Crosshair
- `viewfinder_passepartout_overscan` · float — Border size of the Viewfinder Passepartout overlay
- `viewfinder_passepartout_opacity` · float — Opacity of the darkened Viewfinder Passepartout overlay
- `clip_start` · float — VR viewport near clipping distance
- `clip_end` · float — VR viewport far clipping distance
- `fly_speed` · float — Fly speed in meters per second
- `view_scale` · float — Scaling factor applied to the VR view for fine adjustements. Modifying this value will keep the viewer at the same world relative position
- `use_positional_tracking` · boolean — Allow VR headsets to affect the location in virtual space, in addition to the rotation
- `use_absolute_tracking` · boolean — Allow the VR tracking origin to be defined independently of the headset location
- `show_object_viewport_mesh` · boolean — Show mesh objects
- `show_object_viewport_curve` · boolean — Show curves
- `show_object_viewport_surf` · boolean — Show surfaces
- `show_object_viewport_meta` · boolean — Show metaballs
- `show_object_viewport_font` · boolean — Show text objects
- `show_object_viewport_curves` · boolean — Show hair curves
- `show_object_viewport_pointcloud` · boolean — Show point clouds
- `show_object_viewport_volume` · boolean — Show volumes
- `show_object_viewport_armature` · boolean — Show armatures
- `show_object_viewport_lattice` · boolean — Show lattices
- `show_object_viewport_empty` · boolean — Show empties
- `show_object_viewport_grease_pencil` · boolean — Show Grease Pencil objects
- `show_object_viewport_camera` · boolean — Show cameras
- `show_object_viewport_light` · boolean — Show lights
- `show_object_viewport_speaker` · boolean — Show speakers
- `show_object_viewport_light_probe` · boolean — Show light probes
- `show_object_select_mesh` · boolean — Allow selection of mesh objects
- `show_object_select_curve` · boolean — Allow selection of curves
- `show_object_select_surf` · boolean — Allow selection of surfaces
- `show_object_select_meta` · boolean — Allow selection of metaballs
- `show_object_select_font` · boolean — Allow selection of text objects
- `show_object_select_curves` · boolean — Allow selection of hair curves
- `show_object_select_pointcloud` · boolean — Allow selection of point clouds
- `show_object_select_volume` · boolean — Allow selection of volumes
- `show_object_select_armature` · boolean — Allow selection of armatures
- `show_object_select_lattice` · boolean — Allow selection of lattices
- `show_object_select_empty` · boolean — Allow selection of empties
- `show_object_select_grease_pencil` · boolean — Allow selection of Grease Pencil objects
- `show_object_select_camera` · boolean — Allow selection of cameras
- `show_object_select_light` · boolean — Allow selection of lights
- `show_object_select_speaker` · boolean — Allow selection of speakers
- `show_object_select_light_probe` · boolean — Allow selection of light probes
- `icon_from_show_object_viewport` · int · read-only

## XrSessionState

Runtime state information about the VR session

- `viewfinder` · pointer · -> XrViewfinderState · read-only — Viewfinder State
- `viewer_pose_location` · float · [3] · read-only — Last known location of the viewer pose (center between the eyes) in world space
- `viewer_pose_rotation` · float · [4] · read-only — Last known rotation of the viewer pose (center between the eyes) in world space
- `navigation_location` · float · [3] — Location offset to apply to base pose when determining viewer location
- `navigation_rotation` · float · [4] — Rotation offset to apply to base pose when determining viewer rotation
- `navigation_scale` · float — Navigation scale multiplier applied when determining viewer scale
- `viewer_scale` · float · read-only — Viewer XR scale factor, computed from the navigation scale, view scale session setting, and active scene unit scale
- `actionmaps` · collection · [] of XrActionMaps · read-only
- `active_actionmap` · int
- `selected_actionmap` · int

## XrUserPath

- `path` · string — OpenXR user path

## wmOwnerID

- `name` · string
