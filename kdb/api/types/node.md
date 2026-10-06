---
area: api-types
group: node
version: 5.2.2
---

# Blender 5.2.2 RNA types: node

## CompositorNodeColorBalance

Adjust color and values

- `input_whitepoint` · float · [3] — The color which gets mapped to white (automatically converted to/from temperature and tint)
- `output_whitepoint` · float · [3] — The color which gets white gets mapped to (automatically converted to/from temperature and tint)

## CompositorNodeCombineColor

Combine an image from its composite color channels

- `mode` · enum · {RGB, HSV, HSL, YCC, YUV} — Mode of color processing
- `ycc_mode` · enum · {ITUBT601, ITUBT709, JFIF} — Color space used for YCbCrA processing

## CompositorNodeConvertColorSpace

Convert between color spaces

- `from_color_space` · enum · {ACES 1.3 sRGB, ACES 2.0 sRGB, ACES2065-1, ACEScc, ACEScct, ACEScg, ADX10, ADX16, ARRI LogC3 (EI800), ARRI LogC4, AgX Base sRGB, AgX Log, Apple Log, Apple Log 2, BMDFilm WideGamut Gen5, CanonLog2 CinemaGamut D55, CanonLog3 CinemaGamut D55, D-Log D-Gamut, DaVinci Intermediate WideGamut, Display P3, Filmic Log, Filmic sRGB, Gamma 1.8 Encoded Rec.709, Gamma 2.2 Encoded AP1, Gamma 2.2 Encoded AdobeRGB, Gamma 2.2 Encoded Rec.709, Gamma 2.4 Encoded Rec.709, Khronos PBR Neutral sRGB, Linear ARRI Wide Gamut 3, Linear ARRI Wide Gamut 4, Linear Adobe RGB, Linear Apple Wide Gamut, Linear BMD WideGamut Gen5, Linear CIE-XYZ D65, Linear CIE-XYZ E, Linear CinemaGamut D55, Linear D-Gamut, Linear DCI-P3 D65, Linear DaVinci WideGamut, Linear FilmLight E-Gamut, ... (63 total)} — Color space of the input image
- `to_color_space` · enum · {ACES 1.3 sRGB, ACES 2.0 sRGB, ACES2065-1, ACEScc, ACEScct, ACEScg, ADX10, ADX16, ARRI LogC3 (EI800), ARRI LogC4, AgX Base sRGB, AgX Log, Apple Log, Apple Log 2, BMDFilm WideGamut Gen5, CanonLog2 CinemaGamut D55, CanonLog3 CinemaGamut D55, D-Log D-Gamut, DaVinci Intermediate WideGamut, Display P3, Filmic Log, Filmic sRGB, Gamma 1.8 Encoded Rec.709, Gamma 2.2 Encoded AP1, Gamma 2.2 Encoded AdobeRGB, Gamma 2.2 Encoded Rec.709, Gamma 2.4 Encoded Rec.709, Khronos PBR Neutral sRGB, Linear ARRI Wide Gamut 3, Linear ARRI Wide Gamut 4, Linear Adobe RGB, Linear Apple Wide Gamut, Linear BMD WideGamut Gen5, Linear CIE-XYZ D65, Linear CIE-XYZ E, Linear CinemaGamut D55, Linear D-Gamut, Linear DCI-P3 D65, Linear DaVinci WideGamut, Linear FilmLight E-Gamut, ... (63 total)} — Color space of the output image

## CompositorNodeConvertToDisplay

Convert from scene linear to display color space, with a view transform and look for tone mapping

- `view_settings` · pointer · -> ColorManagedViewSettings · read-only — Color management view transform settings
- `display_settings` · pointer · -> ColorManagedDisplaySettings · read-only — Color management display device settings

## CompositorNodeCryptomatte

Deprecated. Use Cryptomatte Node instead

- `matte_id` · string — List of object and material crypto IDs to include in matte
- `add` · float · [3] — Add object or material to matte, by picking a color from the Pick output
- `remove` · float · [3] — Remove object or material from matte, by picking a color from the Pick output

## CompositorNodeCryptomatteV2

Generate matte for individual objects and materials using Cryptomatte render passes

- `source` · enum · {RENDER, IMAGE} — Where the Cryptomatte passes are loaded from
- `scene` · pointer · -> Scene
- `image` · pointer · -> Image
- `matte_id` · string — List of object and material crypto IDs to include in matte
- `add` · float · [3] — Add object or material to matte, by picking a color from the Pick output
- `remove` · float · [3] — Remove object or material from matte, by picking a color from the Pick output
- `layer_name` · enum · {CryptoObject, CryptoMaterial, CryptoAsset} — What Cryptomatte layer is used
- `entries` · collection · read-only
- `frame_duration` · int — Number of images of a movie to use
- `frame_start` · int — Global starting frame of the movie/sequence, assuming first picture has a #1
- `frame_offset` · int — Offset the number of the frame to use in the animation
- `use_cyclic` · boolean — Cycle the images in the movie
- `use_auto_refresh` · boolean — Always refresh image on frame changes
- `layer` · enum · {PLACEHOLDER}
- `has_layers` · boolean · read-only — True if this image has any named layer
- `view` · enum · {ALL}
- `has_views` · boolean · read-only — True if this image has multiple views

## CompositorNodeCurveRGB

Perform level adjustments on each color channel of an image

- `mapping` · pointer · -> CurveMapping · read-only

## CompositorNodeCustomGroup

Custom Compositor Group Node for Python nodes

- `node_tree` · pointer · -> NodeTree

## CompositorNodeDefocus

Apply depth of field in 2D, using a Z depth map or mask

- `scene` · pointer · -> Scene — Scene from which to select the active camera (render scene if undefined)
- `bokeh` · enum · {OCTAGON, HEPTAGON, HEXAGON, PENTAGON, SQUARE, TRIANGLE, CIRCLE}
- `angle` · float — Bokeh shape rotation offset
- `f_stop` · float — Amount of focal blur, 128 (infinity) is perfect focus, half the value doubles the blur radius
- `blur_max` · float — Blur limit, maximum CoC radius
- `use_zbuffer` · boolean — Disable when using an image as input instead of actual z-buffer (auto enabled if node not image based, eg. time node)
- `z_scale` · float — Scale the Z input when not using a z-buffer, controls maximum blur designated by the color white or input value 1

## CompositorNodeGroup

- `node_tree` · pointer · -> NodeTree

## CompositorNodeHueCorrect

Adjust hue, saturation, and value with a curve

- `mapping` · pointer · -> CurveMapping · read-only

## CompositorNodeImage

Input image or movie file

- `image` · pointer · -> Image
- `frame_duration` · int — Number of images of a movie to use
- `frame_start` · int — Global starting frame of the movie/sequence, assuming first picture has a #1
- `frame_offset` · int — Offset the number of the frame to use in the animation
- `use_cyclic` · boolean — Cycle the images in the movie
- `use_auto_refresh` · boolean — Always refresh image on frame changes
- `layer` · enum · {PLACEHOLDER}
- `has_layers` · boolean · read-only — True if this image has any named layer
- `view` · enum · {ALL}
- `has_views` · boolean · read-only — True if this image has multiple views

## CompositorNodeKeyingScreen

Create plates for use as a color reference for keying nodes

- `clip` · pointer · -> MovieClip
- `tracking_object` · string

## CompositorNodeMask

Input mask from a mask data-block, created in the image editor

- `mask` · pointer · -> Mask

## CompositorNodeMovieClip

Input image or movie from a movie clip data-block, typically used for motion tracking

- `clip` · pointer · -> MovieClip

## CompositorNodeMovieDistortion

Remove lens distortion from footage, using motion tracking camera lens settings

- `clip` · pointer · -> MovieClip

## CompositorNodeOutputFile

Write image file to disk

- `file_output_items` · collection · [] of NodeCompositorFileOutputItems · read-only
- `active_item_index` · int — Index of the active item
- `directory` · string — The directory where the image will be written
- `file_name` · string — The base name of the file. Other information might be included in the final file name depending on the node options
- `format` · pointer · -> ImageFormatSettings · read-only
- `save_as_render` · boolean — Apply render part of display transform when saving byte image
- `use_file_extension` · boolean — Add the file format extensions to the saved file name (eg: filename + .jpg)

## CompositorNodePlaneTrackDeform

Replace flat planes in footage by another image, detected by plane tracks from motion tracking

- `clip` · pointer · -> MovieClip
- `tracking_object` · string
- `plane_track_name` · string

## CompositorNodeRLayers

Input render passes from a scene render

- `scene` · pointer · -> Scene
- `layer` · enum · {PLACEHOLDER}

## CompositorNodeRelativeToPixel

Converts values that are relative to the image size to be in terms of pixels

- `data_type` · enum · {FLOAT, VECTOR} — The type of data
- `reference_dimension` · enum · {PER_DIMENSION, X, Y, Greater, Smaller, Diagonal} — Defines the dimension of the image that the relative value is in reference to

## CompositorNodeSeparateColor

Split an image into its composite color channels

- `mode` · enum · {RGB, HSV, HSL, YCC, YUV} — Mode of color processing
- `ycc_mode` · enum · {ITUBT601, ITUBT709, JFIF} — Color space used for YCbCrA processing

## CompositorNodeStabilize

Stabilize footage using 2D stabilization motion tracking settings

- `clip` · pointer · -> MovieClip

## CompositorNodeTime

Generate a factor value (from 0.0 to 1.0) between scene start and end time, using a curve mapping

- `curve` · pointer · -> CurveMapping · read-only

## CompositorNodeTrackPos

Provide information about motion tracking points, such as x and y values

- `clip` · pointer · -> MovieClip
- `tracking_object` · string
- `track_name` · string

## CompositorNodeTree

Node tree consisting of linked nodes used for compositing

- `use_viewer_border` · boolean — Unused but kept for compatibility reasons. Use boundaries for viewer nodes and composite backdrop
- `is_strip_modifier` · boolean — The node group is used as a sequencer strip modifier

## CompositorNodeViewer

Visualize data from inside a node graph, in the image editor or as a backdrop

- `ui_shortcut` · int

## EvaluateClosureNodeViewerPathElem

- `evaluate_node_id` · int
- `source_output_node_id` · int
- `source_node_tree` · pointer · -> NodeTree · read-only

## FunctionNodeAlignEulerToVector

Orient an Euler rotation along the given direction

- `axis` · enum · {X, Y, Z} — Axis to align to the vector
- `pivot_axis` · enum · {AUTO, X, Y, Z} — Axis to rotate around

## FunctionNodeAlignRotationToVector

Orient a rotation along the given direction

- `axis` · enum · {X, Y, Z} — Axis to align to the vector
- `pivot_axis` · enum · {AUTO, X, Y, Z} — Axis to rotate around

## FunctionNodeAxesToRotation

Create a rotation from a primary and (ideally orthogonal) secondary axis

- `primary_axis` · enum · {X, Y, Z} — Axis that is aligned exactly to the provided primary direction
- `secondary_axis` · enum · {X, Y, Z} — Axis that is aligned as well as possible given the alignment of the primary axis

## FunctionNodeBitMath

Perform bitwise operations on 32-bit integers

- `operation` · enum · {AND, OR, XOR, NOT, SHIFT, ROTATE}

## FunctionNodeBooleanMath

Perform a logical operation on the given boolean inputs

- `operation` · enum · {AND, OR, NOT, NAND, NOR, XNOR, XOR, IMPLY, NIMPLY}

## FunctionNodeCombineColor

Combine four channels into a single color, based on a particular color model

- `mode` · enum · {RGB, HSV, HSL} — Mode of color processing

## FunctionNodeCompare

Perform a comparison operation on the two given inputs

- `operation` · enum · {LESS_THAN, LESS_EQUAL, GREATER_THAN, GREATER_EQUAL, EQUAL, NOT_EQUAL, BRIGHTER, DARKER}
- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `mode` · enum · {ELEMENT, LENGTH, AVERAGE, DOT_PRODUCT, DIRECTION}

## FunctionNodeFloatToInt

Convert the given floating-point number to an integer, with a choice of methods

- `rounding_mode` · enum · {ROUND, FLOOR, CEILING, TRUNCATE} — Method used to convert the float to an integer

## FunctionNodeFormatString

Insert values into a string using a Python and path template compatible formatting syntax

- `format_items` · collection · [] of NodeFunctionFormatStringItems · read-only
- `active_index` · int — Index of the active item

## FunctionNodeHashValue

Generate a randomized integer using the given input value as a seed

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}

## FunctionNodeInputBool

Provide a True/False value that can be connected to other nodes in the tree

- `boolean` · boolean — Input value used for unconnected socket

## FunctionNodeInputColor

Output a color value chosen with the color picker widget

- `value` · float · [4]

## FunctionNodeInputInt

Provide an integer value that can be connected to other nodes in the tree

- `integer` · int — Input value used for unconnected socket

## FunctionNodeInputIntVector

Provide an integer vector value that can be connected to other nodes in the tree

- `vector` · int · [3]
- `vector_dimensions` · int — Dimensions of the vector socket

## FunctionNodeInputMenu

Provide a menu value that can be connected to other nodes in the tree

- `value` · enum · {} — Value of the output socket

## FunctionNodeInputRotation

Provide a rotation value that can be connected to other nodes in the tree

- `rotation_euler` · float · [3] — Input value used for unconnected socket

## FunctionNodeInputString

Provide a string value that can be connected to other nodes in the tree

- `string` · string
- `textbox_state` · pointer · -> TextboxState · read-only

## FunctionNodeInputVector

Provide a vector value that can be connected to other nodes in the tree

- `vector` · float · [4]
- `vector_dimensions` · int — Dimensions of the vector socket

## FunctionNodeIntegerMath

Perform various math operations on the given integer inputs

- `operation` · enum · {ADD, SUBTRACT, MULTIPLY, DIVIDE, MULTIPLY_ADD, ABSOLUTE, NEGATE, POWER, MINIMUM, MAXIMUM, SIGN, DIVIDE_ROUND, DIVIDE_FLOOR, DIVIDE_CEIL, FLOORED_MODULO, MODULO, GCD, LCM}

## FunctionNodeRandomValue

Output a randomized value

- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR} — Type of data stored in attribute

## FunctionNodeRotateEuler

Apply a secondary Euler rotation to a given Euler rotation

- `rotation_type` · enum · {AXIS_ANGLE, EULER} — Method used to describe the rotation
- `space` · enum · {OBJECT, LOCAL} — Base orientation for rotation

## FunctionNodeRotateRotation

Apply a secondary rotation to a given rotation value

- `rotation_space` · enum · {GLOBAL, LOCAL} — Base orientation for the rotation

## FunctionNodeSeparateColor

Split a color into separate channels, based on a particular color model

- `mode` · enum · {RGB, HSV, HSL} — Mode of color processing

## FunctionNodeStringToValue

Derive a numeric value from a given string representation

- `data_type` · enum · {FLOAT, INT}

## FunctionNodeValueToString

Generate a string representation of the given input value

- `data_type` · enum · {FLOAT, INT}

## GeometryNodeAccumulateField

Add the values of an evaluated field together and output the running total for each element

- `data_type` · enum · {FLOAT, INT, FLOAT_VECTOR, TRANSFORM} — Type of data that is accumulated
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER}

## GeometryNodeAttributeDomainSize

Retrieve the number of elements in a geometry for each attribute domain

- `component` · enum · {MESH, POINTCLOUD, CURVE, INSTANCES, GREASEPENCIL, VOLUME}

## GeometryNodeAttributeStatistic

Calculate statistics about a data set from a field evaluated on a geometry

- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR} — The data type the attribute is converted to before calculating the results
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Which domain to read the data from

## GeometryNodeBake

Cache the incoming data so that it can be used without recomputation

- `bake_items` · collection · [] of NodeGeometryBakeItems · read-only
- `active_index` · int — Index of the active item
- `active_item` · pointer · -> RepeatItem — Index of the active item

## GeometryNodeBlurAttribute

Mix attribute values of neighboring elements

- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR}

## GeometryNodeBoneInfo

Retrieve information of armature bones

- `transform_space` · enum · {ORIGINAL, RELATIVE} — The transformation of the vector and geometry outputs

## GeometryNodeCaptureAttribute

Store the result of a field on a geometry and output the data as a node socket. Allows remembering or interpolating data as the geometry changes, such as positions before deformation

- `capture_items` · collection · [] of NodeGeometryCaptureAttributeItems · read-only
- `active_index` · int — Index of the active item
- `active_item` · pointer · -> RepeatItem — Index of the active item
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Which domain to store the data in

## GeometryNodeClosureToList

Create a list of values

- `list_items` · collection · [] of GeometryNodeClosureToListItems · read-only
- `active_index` · int — Index of the active item
- `active_item` · pointer · -> RepeatItem — Index of the active item

## GeometryNodeClosureToListItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE}
- `identifier` · int · read-only

## GeometryNodeCollectionInfo

Retrieve geometry instances from a collection

- `transform_space` · enum · {ORIGINAL, RELATIVE} — The transformation of the instances output. Does not affect the internal geometry

## GeometryNodeCurveArc

Generate a poly spline arc

- `mode` · enum · {POINTS, RADIUS} — Method used to determine radius and placement

## GeometryNodeCurveHandleTypeSelection

Provide a selection based on the handle types of Bézier control points

- `handle_type` · enum · {FREE, AUTO, VECTOR, ALIGN}
- `mode` · enum · {LEFT, RIGHT} — Whether to check the type of left and right handles

## GeometryNodeCurvePrimitiveBezierSegment

Generate a 2D Bézier spline from the given control points and handles

- `mode` · enum · {POSITION, OFFSET} — Method used to determine control handles

## GeometryNodeCurvePrimitiveCircle

Generate a poly spline circle

- `mode` · enum · {POINTS, RADIUS} — Method used to determine radius and placement

## GeometryNodeCurvePrimitiveLine

Generate a poly spline line with two points

- `mode` · enum · {POINTS, DIRECTION} — Method used to determine radius and placement

## GeometryNodeCurvePrimitiveQuadrilateral

Generate a polygon with four points

- `mode` · enum · {RECTANGLE, PARALLELOGRAM, TRAPEZOID, KITE, POINTS}

## GeometryNodeCurveSetHandles

Set the handle type for the control points of a Bézier curve

- `handle_type` · enum · {FREE, AUTO, VECTOR, ALIGN}
- `mode` · enum · {LEFT, RIGHT} — Whether to update left and right handles

## GeometryNodeCurveSplineType

Change the type of curves

- `spline_type` · enum · {CATMULL_ROM, POLY, BEZIER, NURBS} — The curve type to change the selected curves to

## GeometryNodeCurveToPoints

Generate a point cloud by sampling positions along curves

- `mode` · enum · {EVALUATED, COUNT, LENGTH} — How to generate points from the input curve

## GeometryNodeCustomGroup

Custom Geometry Group Node for Python nodes

- `node_tree` · pointer · -> NodeTree

## GeometryNodeDeleteGeometry

Remove selected elements of a geometry

- `mode` · enum · {ALL, EDGE_FACE, ONLY_FACE} — Which parts of the mesh component to delete
- `domain` · enum · {POINT, EDGE, FACE, CURVE, INSTANCE, LAYER} — Which domain to delete in

## GeometryNodeDistributePointsInGrid

Generate points inside a volume grid

- `mode` · enum · {DENSITY_RANDOM, DENSITY_GRID} — Method to use for scattering points

## GeometryNodeDistributePointsOnFaces

Generate points spread out on the surface of a mesh

- `distribute_method` · enum · {RANDOM, POISSON} — Method to use for scattering points
- `use_legacy_normal` · boolean — Output the normal and rotation values that have been output before the node started taking smooth normals into account

## GeometryNodeDuplicateElements

Generate an arbitrary number copies of each selected input element

- `domain` · enum · {POINT, EDGE, FACE, SPLINE, LAYER, INSTANCE} — Which domain to duplicate

## GeometryNodeExtrudeMesh

Generate new vertices, edges, or faces from selected elements and move them based on an offset while keeping them connected by their boundary

- `mode` · enum · {VERTICES, EDGES, FACES}

## GeometryNodeFieldAtIndex

Retrieve data of other elements in the context's geometry

- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Domain the field is evaluated in
- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR}

## GeometryNodeFieldAverage

Calculate the mean and median of a given field

- `data_type` · enum · {FLOAT, FLOAT_VECTOR} — Type of data the outputs are calculated from
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER}

## GeometryNodeFieldMinAndMax

Calculate the minimum and maximum of a given field

- `data_type` · enum · {FLOAT, INT, FLOAT_VECTOR} — Type of data the outputs are calculated from
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER}

## GeometryNodeFieldOnDomain

Retrieve values from a field on a different domain besides the domain from the context

- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Domain the field is evaluated in
- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR}

## GeometryNodeFieldToGrid

Create new grids by evaluating new values on an existing volume grid topology

- `grid_items` · collection · [] of GeometryNodeFieldToGridItems · read-only
- `active_index` · int — Index of the active item
- `active_item` · pointer · -> RepeatItem — Index of the active item
- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Data type for topology grid

## GeometryNodeFieldToGridItem

- `name` · string
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `identifier` · int · read-only

## GeometryNodeFieldToList

Create a list of values

- `list_items` · collection · [] of GeometryNodeFieldToListItems · read-only
- `active_index` · int — Index of the active item
- `active_item` · pointer · -> GeometryNodeFieldToListItem — Index of the active item

## GeometryNodeFieldToListItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `identifier` · int · read-only

## GeometryNodeFieldVariance

Calculate the standard deviation and variance of a given field

- `data_type` · enum · {FLOAT, FLOAT_VECTOR} — Type of data the outputs are calculated from
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER}

## GeometryNodeFilterList

Remove items from a list

- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}

## GeometryNodeForeachGeometryElementInput

- `paired_output` · pointer · -> Node · read-only — Zone output node that this input node is paired with

## GeometryNodeForeachGeometryElementOutput

- `input_items` · collection · [] of NodeGeometryForeachGeometryElementInputItems · read-only
- `main_items` · collection · [] of NodeGeometryForeachGeometryElementMainItems · read-only
- `generation_items` · collection · [] of NodeGeometryForeachGeometryElementGenerationItems · read-only
- `active_input_index` · int — Index of the active item
- `active_generation_index` · int — Index of the active item
- `active_main_index` · int — Index of the active item
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Geometry domain that is iterated over
- `inspection_index` · int — Iteration index that is used by inspection features like the viewer node or socket inspection

## GeometryNodeGetNamedGrid

Get volume grid from a volume geometry with the specified name

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeGizmoDial

Show a dial gizmo in the viewport for a value

- `color_id` · enum · {PRIMARY, SECONDARY, X, Y, Z}

## GeometryNodeGizmoLinear

Show a linear gizmo in the viewport for a value

- `color_id` · enum · {PRIMARY, SECONDARY, X, Y, Z}
- `draw_style` · enum · {ARROW, CROSS, BOX}

## GeometryNodeGizmoTransform

Show a transform gizmo in the viewport

- `use_translation_x` · boolean
- `use_translation_y` · boolean
- `use_translation_z` · boolean
- `use_rotation_x` · boolean
- `use_rotation_y` · boolean
- `use_rotation_z` · boolean
- `use_scale_x` · boolean
- `use_scale_y` · boolean
- `use_scale_z` · boolean

## GeometryNodeGridAdvect

Move grid values through a velocity field using numerical integration. Supports multiple integration schemes for different accuracy and performance trade-offs

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeGridClip

Deactivate grid voxels outside minimum and maximum coordinates, setting them to the background value.

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeGridDilateAndErode

Dilate or erode the active regions of a grid. This changes which voxels are active but does not change their values.

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeGridInfo

Retrieve information about a volume grid

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeGridMean

Apply mean (box) filter smoothing to a voxel. The mean value from surrounding voxels in a box-shape defined by the radius replaces the voxel value.

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeGridMedian

Apply median (box) filter smoothing to a voxel. The median value from surrounding voxels in a box-shape defined by the radius replaces the voxel value.

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeGridPrune

Make the storage of a volume grid more efficient by collapsing data into tiles or inner nodes

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeGridToPoints

Generate a point cloud from a volume grid's active voxels

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeGridVoxelize

Remove sparseness from a volume grid by making the active tiles into voxels

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeGroup

- `node_tree` · pointer · -> NodeTree

## GeometryNodeImageTexture

Sample values from an image texture

- `interpolation` · enum · {Linear, Closest, Cubic} — Method for smoothing values between pixels
- `extension` · enum · {REPEAT, EXTEND, CLIP, MIRROR} — How the image is extrapolated past its original bounds

## GeometryNodeIndexSwitch

Choose between an arbitrary number of values with an index

- `index_switch_items` · collection · [] of NodeIndexSwitchItems · read-only
- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}

## GeometryNodeInputCollection

Output a single collection

- `collection` · pointer · -> Collection

## GeometryNodeInputFont

Output a font

- `font` · pointer · -> VectorFont

## GeometryNodeInputImage

Input an image data-block

- `image` · pointer · -> Image

## GeometryNodeInputMaterial

Output a single material

- `material` · pointer · -> Material

## GeometryNodeInputNamedAttribute

Retrieve the data of a specified attribute

- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR} — The data type used to read the attribute values

## GeometryNodeInputNormal

Retrieve a unit length vector indicating the direction pointing away from the geometry at each element

- `legacy_corner_normals` · boolean — Always use face normals for the face corner domain, matching old behavior of the node

## GeometryNodeInputObject

Output a single object

- `object` · pointer · -> Object

## GeometryNodeListGetItem

Retrieve a value from a list

- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Value may be implicitly converted if the type does not match
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} — What kind of higher order types are expected to flow through this socket

## GeometryNodeListLength

Count how many items are in a given list

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}

## GeometryNodeMenuSwitch

Select from multiple inputs by name

- `enum_items` · collection · [] of NodeMenuSwitchItems · read-only
- `active_index` · int — Index of the active item
- `active_item` · pointer · -> NodeEnumItem — Active item
- `enum_definition` · pointer · -> Node · read-only — The enum definition can now be accessed directly on the node. This exists for backward compatibility.
- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}

## GeometryNodeMergeLayers

Join groups of Grease Pencil layers into one

- `mode` · enum · {MERGE_BY_NAME, MERGE_BY_ID} — Determines how to choose which layers are merged

## GeometryNodeMeshBoolean

Cut, subtract, or join multiple mesh inputs

- `operation` · enum · {INTERSECT, UNION, DIFFERENCE}
- `solver` · enum · {EXACT, FLOAT, MANIFOLD}

## GeometryNodeMeshCircle

Generate a circular ring of edges

- `fill_type` · enum · {NONE, NGON, TRIANGLE_FAN}

## GeometryNodeMeshCone

Generate a cone mesh

- `fill_type` · enum · {NONE, NGON, TRIANGLE_FAN}

## GeometryNodeMeshCylinder

Generate a cylinder mesh

- `fill_type` · enum · {NONE, NGON, TRIANGLE_FAN}

## GeometryNodeMeshLine

Generate vertices in a line and connect them with edges

- `mode` · enum · {OFFSET, END_POINTS}
- `count_mode` · enum · {TOTAL, RESOLUTION}

## GeometryNodeMeshToCurve

Generate a curve from a mesh

- `mode` · enum · {EDGES, FACES}

## GeometryNodeMeshToPoints

Generate a point cloud from a mesh's vertices

- `mode` · enum · {VERTICES, EDGES, FACES, CORNERS}

## GeometryNodeObjectInfo

Retrieve information from an object

- `transform_space` · enum · {ORIGINAL, RELATIVE} — The transformation of the vector and geometry outputs

## GeometryNodeProximity

Compute the closest location on the target geometry

- `target_element` · enum · {POINTS, EDGES, FACES} — Element of the target geometry to calculate the distance from

## GeometryNodeRaycast

Cast rays from the context geometry onto a target geometry, and retrieve information from each hit point

- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR} — Type of data stored in attribute

## GeometryNodeRealizeInstances

Convert instances into real geometry data

- `realize_to_point_domain` · boolean — Propagate instance attributes to the point domain rather than the curve domain. This property exists for compatibility with 5.0 and earlier.

## GeometryNodeRepeatInput

- `paired_output` · pointer · -> Node · read-only — Zone output node that this input node is paired with

## GeometryNodeRepeatOutput

- `repeat_items` · collection · [] of NodeGeometryRepeatOutputItems · read-only
- `active_index` · int — Index of the active item
- `active_item` · pointer · -> RepeatItem — Index of the active item
- `inspection_index` · int — Iteration index that is used by inspection features like the viewer node or socket inspection

## GeometryNodeResampleCurve

Generate a poly spline for each input spline

- `keep_last_segment` · boolean — Do not collapse curves to single points if they are shorter than the given length. The collapsing behavior exists for compatibility reasons.

## GeometryNodeSDFGridBoolean

Cut, subtract, or join multiple SDF volume grid inputs

- `operation` · enum · {INTERSECT, UNION, DIFFERENCE}

## GeometryNodeSampleCurve

Retrieve data from a point on a curve at a certain distance from its start

- `mode` · enum · {FACTOR, LENGTH} — Method for sampling input
- `use_all_curves` · boolean — Sample lengths based on the total length of all curves, rather than using a length inside each selected curve
- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR}

## GeometryNodeSampleGrid

Retrieve values from the specified volume grid

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeSampleGridIndex

Retrieve volume grid values at specific voxels

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeSampleIndex

Retrieve values from specific geometry elements

- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR}
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER}
- `clamp` · boolean — Clamp the indices to the size of the attribute domain instead of outputting a default value for invalid indices

## GeometryNodeSampleNearest

Find the element of a geometry closest to a position. Similar to the "Index of Nearest" node

- `domain` · enum · {POINT, EDGE, FACE, CORNER}

## GeometryNodeSampleNearestSurface

Calculate the interpolated value of a mesh attribute on the closest point of its surface

- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR}

## GeometryNodeSampleUVSurface

Calculate the interpolated values of a mesh attribute at a UV coordinate

- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR}

## GeometryNodeScaleElements

Scale groups of connected edges and faces

- `domain` · enum · {FACE, EDGE} — Element type to transform

## GeometryNodeSeparateGeometry

Split a geometry into two geometry outputs based on a selection

- `domain` · enum · {POINT, EDGE, FACE, CURVE, INSTANCE, LAYER} — Which domain to separate on

## GeometryNodeSetCurveHandlePositions

Set the positions for the handles of Bézier curves

- `mode` · enum · {LEFT, RIGHT} — Whether to update left and right handles

## GeometryNodeSetGreasePencilColor

Set color and opacity attributes on Grease Pencil geometry

- `mode` · enum · {STROKE, FILL}

## GeometryNodeSetGreasePencilDepth

Set the Grease Pencil depth order to use

- `depth_order` · enum · {2D, 3D}

## GeometryNodeSetGridBackground

Set the background value used for inactive voxels and tiles

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeSetGridTransform

Set the transform for the grid from index space into object space.

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Node socket data type

## GeometryNodeSetMeshNormal

Store a normal vector for each mesh element

- `mode` · enum · {SHARPNESS, FREE, TANGENT_SPACE} — Storage mode for custom normal data
- `domain` · enum · {POINT, FACE, CORNER} — Attribute domain to store free custom normals

## GeometryNodeSetShadeSmooth

Control the smoothness of mesh normals around each face by changing the "shade smooth" attribute

- `domain` · enum · {EDGE, FACE}

## GeometryNodeSimulationInput

Input data for the simulation zone

- `paired_output` · pointer · -> Node · read-only — Zone output node that this input node is paired with

## GeometryNodeSimulationOutput

Output data from the simulation zone

- `state_items` · collection · [] of NodeGeometrySimulationOutputItems · read-only
- `active_index` · int — Index of the active item
- `active_item` · pointer · -> SimulationStateItem — Index of the active item

## GeometryNodeSortElements

Rearrange geometry elements, changing their indices

- `domain` · enum · {POINT, EDGE, FACE, CURVE, INSTANCE}

## GeometryNodeSortList

Sort a list based on weights

- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}

## GeometryNodeSplitToInstances

Create separate geometries containing the elements from the same group

- `domain` · enum · {POINT, EDGE, FACE, CURVE, INSTANCE, LAYER} — Attribute domain for the Selection and Group ID inputs

## GeometryNodeStoreNamedAttribute

Store the result of a field on a geometry as an attribute with the specified name

- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR} — Type of data stored in attribute
- `domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Which domain to store the data in

## GeometryNodeStoreNamedGrid

Store grid data in a volume geometry with the specified name

- `data_type` · enum · {BOOLEAN, FLOAT, DOUBLE, INT, INT64, MASK, VECTOR_FLOAT, VECTOR_DOUBLE, VECTOR_INT, POINTS, UNKNOWN} — Type of grid data

## GeometryNodeSwitch

Switch between two inputs

- `input_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}

## GeometryNodeToolActiveElement

Active element indices of the edited geometry, for tool execution

- `domain` · enum · {POINT, EDGE, FACE, LAYER}

## GeometryNodeToolSetSelection

Set selection of the edited geometry, for tool execution

- `domain` · enum · {POINT, EDGE, FACE, CURVE}
- `selection_type` · enum · {BOOLEAN, FLOAT}

## GeometryNodeTree

Node tree consisting of linked nodes used for geometries

- `is_tool` · boolean — The node group is used as a tool
- `node_tool_idname` · string — Unique operator identifier for the node tool
- `is_modifier` · boolean — The node group is used as a geometry modifier
- `is_mode_object` · boolean — The node group is used in object mode
- `is_mode_edit` · boolean — The node group is used in edit mode
- `is_mode_sculpt` · boolean — The node group is used in sculpt mode
- `is_mode_paint` · boolean — The node group is used in paint mode
- `is_type_mesh` · boolean — The node group is used for meshes
- `is_type_curve` · boolean — The node group is used for curves
- `is_type_pointcloud` · boolean — The node group is used for point clouds
- `use_wait_for_click` · boolean — Wait for mouse click input before running the operator from a menu
- `is_type_grease_pencil` · boolean — The node group is used for Grease Pencil
- `show_modifier_manage_panel` · boolean — Turn on the option to display the manage panel when creating a modifier

## GeometryNodeTrimCurve

Shorten curves by removing portions at the start or end

- `mode` · enum · {FACTOR, LENGTH} — How to find endpoint positions for the trimmed spline

## GeometryNodeViewer

Display the input data in the Spreadsheet Editor

- `ui_shortcut` · int
- `viewer_items` · collection · [] of NodeGeometryViewerItems · read-only
- `active_index` · int — Index of the active item
- `active_item` · pointer · -> NodeGeometryViewerItem
- `domain` · enum · {AUTO, POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Domain to evaluate fields on

## GeometryNodeWarning

Create custom warnings in node groups

- `warning_type` · enum · {ERROR, WARNING, INFO}

## GreasePencilTreeNode

Grease Pencil node in the layer tree. Either a layer or a group

- `name` · string — The name of the tree node
- `hide` · boolean — Set tree node visibility
- `lock` · boolean — Protect tree node from editing
- `select` · boolean — Tree node is selected
- `use_onion_skinning` · boolean — Display onion skins before and after the current frame
- `use_masks` · boolean — The visibility of drawings in this tree node is affected by the layers in the masks list
- `channel_color` · float · [3] — Color of the channel in the dope sheet
- `next_node` · pointer · -> GreasePencilTreeNode · read-only — The layer tree node after (i.e. above) this one
- `prev_node` · pointer · -> GreasePencilTreeNode · read-only — The layer tree node before (i.e. below) this one
- `parent_group` · pointer · -> GreasePencilLayerGroup · read-only — The parent group of this layer tree node

## GroupNodeViewerPathElem

- `node_id` · int

## Node

Node in a node tree

- `type` · string · read-only — Legacy unique node type identifier, redundant with bl_idname property
- `location` · float · [2] — Location of the node within its parent frame
- `location_absolute` · float · [2] — Location of the node in the entire canvas
- `width` · float — Width of the node
- `height` · float — Height of the node. This property holds true data only under certain circumstances, e.g. for a Frame node after the node graph was displayed. For most types of nodes, the displayed height is based on the node's contents and not reflected in this property.
- `dimensions` · float · [2] · read-only — Absolute bounding box dimensions of the node after it was displayed
- `name` · string — Unique node identifier
- `label` · string — Optional custom node label
- `inputs` · collection · [] of NodeInputs · read-only
- `outputs` · collection · [] of NodeOutputs · read-only
- `panel_states` · collection · read-only — Expansion state of each panel in the node
- `internal_links` · collection · read-only — Internal input-to-output connections for muting
- `parent` · pointer · -> Node — Parent this node is attached to, e.g. a Frame node
- `warning_propagation` · enum · {ALL, ERRORS_AND_WARNINGS, ERRORS, NONE} — The kinds of messages that should be propagated from this node to the parent group node
- `use_custom_color` · boolean — Use custom color for the node
- `color` · float · [3] — Custom color of the node body
- `color_tag` · enum · {NONE, ATTRIBUTE, COLOR, CONVERTER, DISTORT, FILTER, GEOMETRY, INPUT, MATTE, OUTPUT, SCRIPT, SHADER, TEXTURE, VECTOR, PATTERN, INTERFACE, GROUP} · read-only — Node header color tag
- `select` · boolean — Node selection state
- `show_options` · boolean — Whether the node options are visible, e.g. the selected data-block of a node group node
- `show_preview` · boolean
- `hide` · boolean — Node collapsed state
- `mute` · boolean
- `show_texture` · boolean — Display node in viewport textured shading mode
- `bl_idname` · string
- `bl_label` · string — The node label
- `bl_description` · string
- `bl_icon` · enum · {NONE, CHAR_NOTDEF, CHAR_REPLACEMENT, NOT_FOUND, BLANK1, AUTOMERGE_OFF, AUTOMERGE_ON, CHECKBOX_DEHLT, CHECKBOX_HLT, CLIPUV_DEHLT, CLIPUV_HLT, DECORATE_UNLOCKED, DECORATE_LOCKED, FAKE_USER_OFF, FAKE_USER_ON, HIDE_ON, HIDE_OFF, INDIRECT_ONLY_OFF, INDIRECT_ONLY_ON, ONIONSKIN_OFF, ONIONSKIN_ON, UNPINNED, PINNED, RADIOBUT_OFF, RADIOBUT_ON, RECORD_OFF, RECORD_ON, RESTRICT_RENDER_ON, RESTRICT_RENDER_OFF, RESTRICT_SELECT_ON, RESTRICT_SELECT_OFF, RESTRICT_VIEW_ON, RESTRICT_VIEW_OFF, RIGHTARROW, DOWNARROW_HLT, SELECT_INTERSECT, SELECT_DIFFERENCE, SNAP_OFF, SNAP_ON, PLAYHEAD_SNAP_OFF, ... (1033 total)} — The node icon
- `bl_static_type` · string · read-only — Legacy unique node type identifier, redundant with bl_idname property
- `bl_width_default` · float — Default width of the node when it is created
- `bl_width_min` · float — When changing the node's size, it has at least this width
- `bl_width_max` · float — When changing the node's size, it can have at most this width
- `bl_height_default` · float — Default height of the node when it is created (mostly unused, see Height)
- `bl_height_min` · float — When changing the node's size, it has at least this height (mostly unused, see Height)
- `bl_height_max` · float — When changing the node's size, it can have at most this height (mostly unused, see Height)

## NodeClosureInput

- `paired_output` · pointer · -> Node · read-only — Zone output node that this input node is paired with

## NodeClosureInputItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} — What kind of higher order types are expected to flow through this socket

## NodeClosureOutput

- `input_items` · collection · [] of NodeClosureInputItems · read-only
- `output_items` · collection · [] of NodeClosureOutputItems · read-only
- `active_input_index` · int — Index of the active item
- `active_output_index` · int — Index of the active item
- `define_signature` · boolean — This zone defines a closure signature that should be used by other nodes

## NodeClosureOutputItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} — What kind of higher order types are expected to flow through this socket

## NodeCombineBundle

Combine multiple socket values into one.

- `bundle_items` · collection · [] of NodeCombineBundleItems · read-only
- `active_index` · int — Index of the active item
- `define_signature` · boolean — This node defines a bundle signature that should be used by other nodes

## NodeCombineBundleItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} — What kind of higher order types are expected to flow through this socket

## NodeCompositorFileOutputItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `vector_socket_dimensions` · int — Dimensions of the vector socket
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `override_node_format` · boolean — Use a different format instead of the node format for this file
- `save_as_render` · boolean — Apply render part of display transform when saving byte image
- `format` · pointer · -> ImageFormatSettings · read-only

## NodeCustomGroup

Base node type for custom registered node group types

- `node_tree` · pointer · -> NodeTree

## NodeEnableOutput

Either pass through the input value or output the fallback value

- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}

## NodeEnumItem

- `name` · string
- `description` · string

## NodeEvaluateClosure

Execute a given closure

- `input_items` · collection · [] of NodeEvaluateClosureInputItems · read-only
- `output_items` · collection · [] of NodeEvaluateClosureOutputItems · read-only
- `active_input_index` · int — Index of the active item
- `active_output_index` · int — Index of the active item
- `define_signature` · boolean — This node defines a closure signature that should be used by other nodes

## NodeEvaluateClosureInputItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} — What kind of higher order types are expected to flow through this socket

## NodeEvaluateClosureOutputItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} — What kind of higher order types are expected to flow through this socket

## NodeFrame

Collect related nodes together in a common area. Useful for organization when the re-usability of a node group is not required

- `text` · pointer · -> Text
- `shrink` · boolean — Shrink the frame to minimal bounding box
- `label_size` · int — Font size to use for displaying the label

## NodeFunctionFormatStringItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor

## NodeGeometryBakeItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `attribute_domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Domain where the field is captured if it is not already an attribute-field
- `is_attribute` · boolean — Bake item is an attribute stored on a geometry

## NodeGeometryCaptureAttributeItem

- `name` · string
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR}

## NodeGeometryViewerItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `auto_remove` · boolean — Remove the item automatically when it is unlinked

## NodeGetBundleItem

Retrieve a bundle item by path.

- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Value may be implicitly converted if the type does not match
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} — What kind of higher order types are expected to flow through this socket

## NodeGroup

- `node_tree` · pointer · -> NodeTree

## NodeGroupOutput

Output data from inside of a node group

- `is_active_output` · boolean — True if this node is used as the active group output

## NodeImplicitConversion

Implicitly convert the input value to a fixed socket type

- `socket_idname` · string
- `data_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}

## NodeInternalSocketTemplate

Type and default value of a node socket

- `name` · string · read-only — Name of the socket
- `identifier` · string · read-only — Identifier of the socket
- `type` · enum · {CUSTOM, VALUE, INT, BOOLEAN, VECTOR, ROTATION, MATRIX, STRING, RGBA, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, MENU, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} · read-only — Data type of the socket

## NodeLink

Link between nodes in a node tree

- `is_valid` · boolean — Link is valid
- `is_muted` · boolean — Link is muted and can be ignored
- `from_node` · pointer · -> Node · read-only
- `to_node` · pointer · -> Node · read-only
- `from_socket` · pointer · -> NodeSocket · read-only
- `to_socket` · pointer · -> NodeSocket · read-only
- `is_hidden` · boolean · read-only — Link is hidden due to invisible sockets
- `multi_input_sort_id` · int · read-only — Used to sort multiple links coming into the same input. The highest ID is at the top.

## NodePanelState

Expansion state of a panel in a node

- `identifier` · int · read-only — Unique identifier matching the panel's persistent_uid
- `is_collapsed` · boolean — Whether the panel is collapsed

## NodeRaycastSampleAttributeItem

- `name` · string
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `data_type` · enum · {FLOAT, INT, BOOLEAN, FLOAT_VECTOR, FLOAT_COLOR, QUATERNION, FLOAT4X4, STRING, INT8, INT16_2D, INT32_2D, FLOAT2, FLOAT4, BYTE_COLOR}

## NodeReroute

A single-socket organization tool that supports one input and multiple outputs

- `socket_idname` · string

## NodeSeparateBundle

Split a bundle into multiple sockets.

- `bundle_items` · collection · [] of NodeSeparateBundleItems · read-only
- `active_index` · int — Index of the active item
- `define_signature` · boolean — This node defines a bundle signature that should be used by other nodes

## NodeSeparateBundleItem

- `name` · string
- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `color` · float · [4] · read-only — Color of the corresponding socket type in the node editor
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} — What kind of higher order types are expected to flow through this socket

## NodeSocket

Input or output socket of a node

- `name` · string — Socket name
- `label` · string · read-only — Custom dynamic defined UI label for the socket. Can be translated if translation is enabled in the preferences
- `identifier` · string · read-only — Unique identifier for mapping sockets
- `description` · string — Socket tooltip
- `is_output` · boolean · read-only — True if the socket is an output, otherwise input
- `select` · boolean · read-only — True if the socket is selected
- `hide` · boolean — Hide the socket
- `enabled` · boolean — Enable the socket
- `link_limit` · int — Max number of links allowed for this socket
- `is_linked` · boolean · read-only — True if the socket is connected
- `is_unavailable` · boolean · read-only — True if the socket is unavailable
- `is_multi_input` · boolean · read-only — True if the socket can accept multiple ordered input links
- `show_expanded` · boolean — Socket links are expanded in the user interface
- `is_inactive` · boolean · read-only — Socket is grayed out because it has been detected to not have any effect on the output
- `is_icon_visible` · boolean · read-only — Socket is drawn as interactive icon in the node editor
- `hide_value` · boolean — Hide the socket input value
- `pin_gizmo` · boolean — Keep gizmo visible even when the node is not selected
- `node` · pointer · -> Node · read-only — Node owning this socket
- `type` · enum · {CUSTOM, VALUE, INT, BOOLEAN, VECTOR, ROTATION, MATRIX, STRING, RGBA, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, MENU, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR} — Data type
- `display_shape` · enum · {CIRCLE, SQUARE, DIAMOND, CIRCLE_DOT, SQUARE_DOT, DIAMOND_DOT, LINE, VOLUME_GRID, LIST} — Socket shape
- `inferred_structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} · read-only — Best known structure type of the socket. This may not match the socket shape, e.g. for unlinked input sockets
- `bl_idname` · string
- `bl_label` · string — Label to display for the socket type in the UI
- `bl_subtype_label` · string — Label to display for the socket subtype in the UI

## NodeSocketBool

Boolean value socket of a node

- `default_value` · boolean

## NodeSocketCollection

Collection socket of a node

- `default_value` · pointer · -> Collection

## NodeSocketColor

RGBA color socket of a node

- `default_value` · float · [4]

## NodeSocketFloat

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatAngle

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatColorTemperature

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatDistance

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatFactor

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatFrequency

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatMass

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatPercentage

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatPixel

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatTime

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatTimeAbsolute

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatUnsigned

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFloatWavelength

Floating-point number socket of a node

- `default_value` · float

## NodeSocketFont

Font socket of a node

- `default_value` · pointer · -> VectorFont

## NodeSocketImage

Image socket of a node

- `default_value` · pointer · -> Image

## NodeSocketInt

Integer number socket of a node

- `default_value` · int

## NodeSocketIntFactor

Integer number socket of a node

- `default_value` · int

## NodeSocketIntPercentage

Integer number socket of a node

- `default_value` · int

## NodeSocketIntPixel

Integer number socket of a node

- `default_value` · int

## NodeSocketIntUnsigned

Integer number socket of a node

- `default_value` · int

## NodeSocketIntVector2D

Integer vector socket of a node

- `default_value` · int · [2]

## NodeSocketIntVector3D

Integer vector socket of a node

- `default_value` · int · [3]

## NodeSocketIntVectorFactor2D

Integer vector socket of a node

- `default_value` · int · [2]

## NodeSocketIntVectorFactor3D

Integer vector socket of a node

- `default_value` · int · [3]

## NodeSocketIntVectorPercentage2D

Integer vector socket of a node

- `default_value` · int · [2]

## NodeSocketIntVectorPercentage3D

Integer vector socket of a node

- `default_value` · int · [3]

## NodeSocketIntVectorPixel2D

Integer vector socket of a node

- `default_value` · int · [2]

## NodeSocketIntVectorPixel3D

Integer vector socket of a node

- `default_value` · int · [3]

## NodeSocketIntVectorUnsigned2D

Integer vector socket of a node

- `default_value` · int · [2]

## NodeSocketIntVectorUnsigned3D

Integer vector socket of a node

- `default_value` · int · [3]

## NodeSocketMask

Mask socket of a node

- `default_value` · pointer · -> Mask

## NodeSocketMaterial

Material socket of a node

- `default_value` · pointer · -> Material

## NodeSocketMenu

Menu socket of a node

- `default_value` · enum · {}

## NodeSocketObject

Object socket of a node

- `default_value` · pointer · -> Object

## NodeSocketRotation

Rotation value socket of a node

- `default_value` · float · [3]

## NodeSocketScene

Scene socket of a node

- `default_value` · pointer · -> Scene

## NodeSocketSound

Sound socket of a node

- `default_value` · pointer · -> Sound

## NodeSocketString

String socket of a node

- `default_value` · string

## NodeSocketStringFilePath

String socket of a node

- `default_value` · string

## NodeSocketText

Text socket of a node

- `default_value` · pointer · -> Text

## NodeSocketTexture

Texture socket of a node

- `default_value` · pointer · -> Texture

## NodeSocketVector

3D vector socket of a node

- `default_value` · float · [3]

## NodeSocketVector2D

3D vector socket of a node

- `default_value` · float · [2]

## NodeSocketVector4D

3D vector socket of a node

- `default_value` · float · [4]

## NodeSocketVectorAcceleration

3D vector socket of a node

- `default_value` · float · [3]

## NodeSocketVectorAcceleration2D

3D vector socket of a node

- `default_value` · float · [2]

## NodeSocketVectorAcceleration4D

3D vector socket of a node

- `default_value` · float · [4]

## NodeSocketVectorDirection

3D vector socket of a node

- `default_value` · float · [3]

## NodeSocketVectorDirection2D

3D vector socket of a node

- `default_value` · float · [2]

## NodeSocketVectorDirection4D

3D vector socket of a node

- `default_value` · float · [4]

## NodeSocketVectorEuler

3D vector socket of a node

- `default_value` · float · [3]

## NodeSocketVectorEuler2D

3D vector socket of a node

- `default_value` · float · [2]

## NodeSocketVectorEuler4D

3D vector socket of a node

- `default_value` · float · [4]

## NodeSocketVectorFactor

3D vector socket of a node

- `default_value` · float · [3]

## NodeSocketVectorFactor2D

3D vector socket of a node

- `default_value` · float · [2]

## NodeSocketVectorFactor4D

3D vector socket of a node

- `default_value` · float · [4]

## NodeSocketVectorPercentage

3D vector socket of a node

- `default_value` · float · [3]

## NodeSocketVectorPercentage2D

3D vector socket of a node

- `default_value` · float · [2]

## NodeSocketVectorPercentage4D

3D vector socket of a node

- `default_value` · float · [4]

## NodeSocketVectorPixel

3D vector socket of a node

- `default_value` · float · [3]

## NodeSocketVectorPixel2D

3D vector socket of a node

- `default_value` · float · [2]

## NodeSocketVectorPixel4D

3D vector socket of a node

- `default_value` · float · [4]

## NodeSocketVectorTranslation

3D vector socket of a node

- `default_value` · float · [3]

## NodeSocketVectorTranslation2D

3D vector socket of a node

- `default_value` · float · [2]

## NodeSocketVectorTranslation4D

3D vector socket of a node

- `default_value` · float · [4]

## NodeSocketVectorVelocity

3D vector socket of a node

- `default_value` · float · [3]

## NodeSocketVectorVelocity2D

3D vector socket of a node

- `default_value` · float · [2]

## NodeSocketVectorVelocity4D

3D vector socket of a node

- `default_value` · float · [4]

## NodeSocketVectorXYZ

3D vector socket of a node

- `default_value` · float · [3]

## NodeSocketVectorXYZ2D

3D vector socket of a node

- `default_value` · float · [2]

## NodeSocketVectorXYZ4D

3D vector socket of a node

- `default_value` · float · [4]

## NodeStoreBundleItem

Store a bundle item by path and data type.

- `socket_type` · enum · {FLOAT, INT, BOOLEAN, VECTOR, RGBA, ROTATION, MATRIX, STRING, MENU, SHADER, OBJECT, IMAGE, GEOMETRY, COLLECTION, TEXTURE, MATERIAL, BUNDLE, CLOSURE, FONT, SCENE, TEXT, MASK, SOUND, INT_VECTOR}
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} — What kind of higher order types are expected to flow through this socket

## NodeTree

Node tree consisting of linked nodes used for shading, textures and compositing

- `color_tag` · enum · {NONE, ATTRIBUTE, COLOR, CONVERTER, DISTORT, FILTER, GEOMETRY, INPUT, MATTE, OUTPUT, SCRIPT, SHADER, TEXTURE, VECTOR, PATTERN, INTERFACE, GROUP} — Color tag of the node group which influences the header color
- `default_group_node_width` · int — The width for newly created group nodes
- `view_center` · float · [2] · read-only — The current location (offset) of the view for this Node Tree
- `description` · string — Description of the node tree
- `animation_data` · pointer · -> AnimData · read-only — Animation data for this data-block
- `nodes` · collection · [] of Nodes · read-only
- `links` · collection · [] of NodeLinks · read-only
- `annotation` · pointer · -> Annotation — Annotation data
- `type` · enum · {UNDEFINED, CUSTOM, SHADER, TEXTURE, COMPOSITING, GEOMETRY} · read-only — Node Tree type (deprecated, bl_idname is the actual node tree type identifier)
- `interface` · pointer · -> NodeTreeInterface · read-only — Interface declaration for this node tree
- `bl_idname` · string
- `bl_label` · string — The node tree label
- `bl_description` · string
- `bl_icon` · enum · {NONE, CHAR_NOTDEF, CHAR_REPLACEMENT, NOT_FOUND, BLANK1, AUTOMERGE_OFF, AUTOMERGE_ON, CHECKBOX_DEHLT, CHECKBOX_HLT, CLIPUV_DEHLT, CLIPUV_HLT, DECORATE_UNLOCKED, DECORATE_LOCKED, FAKE_USER_OFF, FAKE_USER_ON, HIDE_ON, HIDE_OFF, INDIRECT_ONLY_OFF, INDIRECT_ONLY_ON, ONIONSKIN_OFF, ONIONSKIN_ON, UNPINNED, PINNED, RADIOBUT_OFF, RADIOBUT_ON, RECORD_OFF, RECORD_ON, RESTRICT_RENDER_ON, RESTRICT_RENDER_OFF, RESTRICT_SELECT_ON, RESTRICT_SELECT_OFF, RESTRICT_VIEW_ON, RESTRICT_VIEW_OFF, RIGHTARROW, DOWNARROW_HLT, SELECT_INTERSECT, SELECT_DIFFERENCE, SNAP_OFF, SNAP_ON, PLAYHEAD_SNAP_OFF, ... (1033 total)} — The node tree icon
- `bl_use_group_interface` · boolean — Determines the visibility of some UI elements related to node groups

## NodeTreeInterface

Declaration of sockets and ui panels of a node group

- `items_tree` · collection · read-only — Items in the node interface
- `active_index` · int — Index of the active item
- `active` · pointer · -> NodeTreeInterfaceItem — Active item

## NodeTreeInterfaceItem

Item in a node tree interface

- `item_type` · enum · {SOCKET, PANEL} · read-only — Type of interface item
- `parent` · pointer · -> NodeTreeInterfacePanel · read-only — Panel that contains the item
- `position` · int · read-only — Position of the item in its parent panel
- `index` · int · read-only — Global index of the item among all items in the interface

## NodeTreeInterfacePanel

Declaration of a node panel

- `name` · string — Panel name
- `description` · string — Panel description
- `default_closed` · boolean — Panel is closed by default on new nodes
- `select` · boolean — Panel is selected in the interface
- `interface_items` · collection · read-only — Items in the node panel
- `persistent_uid` · int · read-only — Unique identifier for this panel within this node tree
- `identifier` · int · read-only — Unique identifier for this panel within this node tree

## NodeTreeInterfaceSocket

Declaration of a node socket

- `name` · string — Socket name
- `identifier` · string · read-only — Unique identifier for mapping sockets
- `description` · string — Socket description
- `socket_type` · enum · {DEFAULT} — Type of the socket generated by this interface item
- `in_out` · enum · {INPUT, OUTPUT} · read-only — Input or output socket type
- `hide_value` · boolean — Hide the socket input value even when the socket is not connected
- `hide_in_modifier` · boolean — Don't show the input value in the geometry nodes modifier interface
- `force_non_field` · boolean — Only allow single value inputs rather than field.
Deprecated. Will be remove in 5.0.
- `is_inspect_output` · boolean — Take link out of node group to connect to root tree output node
- `is_panel_toggle` · boolean — This socket is meant to be used as the toggle in its panel header
- `layer_selection_field` · boolean — Take Grease Pencil Layer or Layer Group as selection field
- `menu_expanded` · boolean — Draw the menu socket as an expanded drop-down menu
- `optional_label` · boolean — Indicate that the label of this socket is not necessary to understand its meaning. This may result in the label being skipped in some cases
- `select` · boolean — Socket is selected in the interface
- `attribute_domain` · enum · {POINT, EDGE, FACE, CORNER, CURVE, INSTANCE, LAYER} — Attribute domain used by the geometry nodes modifier to create an attribute output
- `default_attribute_name` · string — The attribute name used by default when the node group is used by a geometry nodes modifier
- `structure_type` · enum · {AUTO, DYNAMIC, FIELD, GRID, LIST, SINGLE} — What kind of higher order types are expected to flow through this socket
- `default_input` · enum · {VALUE, INDEX, ID_OR_INDEX, NORMAL, POSITION, INSTANCE_TRANSFORM, HANDLE_LEFT, HANDLE_RIGHT, SCENE_FRAME, UNIFORM_IMAGE_COORDINATES, SELF_OBJECT} — Input to use when the socket is unconnected. Requires "Hide Value".
- `bl_socket_idname` · string — Name of the socket type

## NodeTreeInterfaceSocketBool

Boolean value socket of a node

- `default_value` · boolean — Input value used for unconnected socket

## NodeTreeInterfaceSocketCollection

Collection socket of a node

- `default_value` · pointer · -> Collection — Input value used for unconnected socket

## NodeTreeInterfaceSocketColor

RGBA color socket of a node

- `default_value` · float · [4] — Input value used for unconnected socket

## NodeTreeInterfaceSocketFloat

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatAngle

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatColorTemperature

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatDistance

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatFactor

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatFrequency

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatMass

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatPercentage

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatPixel

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatTime

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatTimeAbsolute

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatUnsigned

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFloatWavelength

Floating-point number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · float — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketFont

Font socket of a node

- `default_value` · pointer · -> VectorFont — Input value used for unconnected socket

## NodeTreeInterfaceSocketImage

Image socket of a node

- `default_value` · pointer · -> Image — Input value used for unconnected socket

## NodeTreeInterfaceSocketInt

Integer number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · int — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntFactor

Integer number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · int — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntPercentage

Integer number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · int — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntPixel

Integer number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · int — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntUnsigned

Integer number socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · int — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntVector2D

Integer vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the integer vector socket
- `default_value` · int · [2] — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntVector3D

Integer vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the integer vector socket
- `default_value` · int · [3] — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntVectorFactor2D

Integer vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the integer vector socket
- `default_value` · int · [2] — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntVectorFactor3D

Integer vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the integer vector socket
- `default_value` · int · [3] — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntVectorPercentage2D

Integer vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the integer vector socket
- `default_value` · int · [2] — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntVectorPercentage3D

Integer vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the integer vector socket
- `default_value` · int · [3] — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntVectorPixel2D

Integer vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the integer vector socket
- `default_value` · int · [2] — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntVectorPixel3D

Integer vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the integer vector socket
- `default_value` · int · [3] — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntVectorUnsigned2D

Integer vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the integer vector socket
- `default_value` · int · [2] — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketIntVectorUnsigned3D

Integer vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the integer vector socket
- `default_value` · int · [3] — Input value used for unconnected socket
- `min_value` · int — Minimum value
- `max_value` · int — Maximum value

## NodeTreeInterfaceSocketMask

Mask socket of a node

- `default_value` · pointer · -> Mask — Input value used for unconnected socket

## NodeTreeInterfaceSocketMaterial

Material socket of a node

- `default_value` · pointer · -> Material — Input value used for unconnected socket

## NodeTreeInterfaceSocketMenu

Menu socket of a node

- `default_value` · enum · {} — Input value used for unconnected socket

## NodeTreeInterfaceSocketObject

Object socket of a node

- `default_value` · pointer · -> Object — Input value used for unconnected socket

## NodeTreeInterfaceSocketRotation

Rotation value socket of a node

- `default_value` · float · [3] — Input value used for unconnected socket

## NodeTreeInterfaceSocketScene

Scene socket of a node

- `default_value` · pointer · -> Scene — Input value used for unconnected socket

## NodeTreeInterfaceSocketSound

Sound socket of a node

- `default_value` · pointer · -> Sound — Input value used for unconnected socket

## NodeTreeInterfaceSocketString

String socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · string — Input value used for unconnected socket

## NodeTreeInterfaceSocketStringFilePath

String socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `default_value` · string — Input value used for unconnected socket

## NodeTreeInterfaceSocketText

Text socket of a node

- `default_value` · pointer · -> Text — Input value used for unconnected socket

## NodeTreeInterfaceSocketTexture

Texture socket of a node

- `default_value` · pointer · -> Texture — Input value used for unconnected socket

## NodeTreeInterfaceSocketVector

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [3] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVector2D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [2] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVector4D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [4] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorAcceleration

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [3] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorAcceleration2D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [2] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorAcceleration4D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [4] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorDirection

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [3] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorDirection2D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [2] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorDirection4D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [4] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorEuler

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [3] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorEuler2D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [2] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorEuler4D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [4] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorFactor

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [3] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorFactor2D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [2] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorFactor4D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [4] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorPercentage

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [3] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorPercentage2D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [2] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorPercentage4D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [4] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorPixel

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [3] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorPixel2D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [2] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorPixel4D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [4] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorTranslation

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [3] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorTranslation2D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [2] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorTranslation4D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [4] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorVelocity

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [3] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorVelocity2D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [2] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorVelocity4D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [4] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorXYZ

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [3] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorXYZ2D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [2] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreeInterfaceSocketVectorXYZ4D

3D vector socket of a node

- `subtype` · enum · {DEFAULT} — Subtype of the default value
- `dimensions` · int — Dimensions of the vector socket
- `default_value` · float · [4] — Input value used for unconnected socket
- `min_value` · float — Minimum value
- `max_value` · float — Maximum value

## NodeTreePath

Element of the node space tree path

- `node_tree` · pointer · -> NodeTree · read-only — Base node tree from context

## Nodes

Collection of Nodes

- `active` · pointer · -> Node — Active node in this tree

## NodesModifier

- `node_group` · pointer · -> NodeTree — Node group that controls what this modifier does
- `bake_directory` · string — Location on disk where the bake data is stored
- `bake_target` · enum · {PACKED, DISK} — Where to store the baked data
- `bakes` · collection · [] of NodesModifierBakes · read-only — All potential bakes, as defined by the assigned Geometry Nodes
- `panels` · collection · [] of NodesModifierPanels · read-only
- `show_group_selector` · boolean
- `show_manage_panel` · boolean
- `node_warnings` · collection · read-only
- `open_output_attributes_panel` · boolean
- `open_manage_panel` · boolean
- `open_bake_panel` · boolean
- `open_named_attributes_panel` · boolean
- `open_bake_data_blocks_panel` · boolean
- `open_warnings_panel` · boolean
- `properties` · pointer · -> NodesModifierProperties · read-only

## NodesModifierBake

- `directory` · string — Location on disk where the bake data is stored
- `frame_start` · int — Frame where the baking starts
- `frame_end` · int — Frame where the baking ends
- `use_custom_simulation_frame_range` · boolean — Override the simulation frame range from the scene
- `use_custom_path` · boolean — Specify a path where the baked data should be stored manually
- `bake_target` · enum · {INHERIT, PACKED, DISK} — Where to store the baked data
- `bake_mode` · enum · {ANIMATION, STILL}
- `bake_id` · int · read-only — Identifier for this bake which remains unchanged even when the bake node is renamed, grouped or ungrouped
- `node` · pointer · -> Node · read-only — Bake node or simulation output node that corresponds to this bake. This node may be deeply nested in the modifier node group. It can be none in some cases like missing linked data blocks.
- `data_blocks` · collection · [] of NodesModifierBakeDataBlocks · read-only

## NodesModifierBakeDataBlocks

Collection of data-blocks that can be referenced by baked data

- `active_index` · int

## NodesModifierDataBlock

- `id_name` · string · read-only — Name that is mapped to the referenced data-block
- `lib_name` · string · read-only — Used when the data block is not local to the current .blend file but is linked from some library
- `id` · pointer · -> ID
- `id_type` · enum · {ACTION, ARMATURE, BRUSH, CACHEFILE, CAMERA, COLLECTION, CURVE, CURVES, FONT, GREASEPENCIL, GREASEPENCIL_V3, IMAGE, KEY, LATTICE, LIBRARY, LIGHT, LIGHT_PROBE, LINESTYLE, MASK, MATERIAL, MESH, META, MOVIECLIP, NODETREE, OBJECT, PAINTCURVE, PALETTE, PARTICLE, POINTCLOUD, SCENE, SCREEN, SOUND, SPEAKER, TEXT, TEXTURE, VOLUME, WINDOWMANAGER, WORKSPACE, WORLD} · read-only

## NodesModifierPanel

- `is_open` · boolean — Whether the panel is expanded or closed

## NodesModifierWarning

Warning created during evaluation of a geometry nodes modifier

- `message` · string · read-only
- `type` · enum · {ERROR, WARNING, INFO} · read-only

## ShaderNodeAmbientOcclusion

Compute how much the hemisphere above the shading point is occluded, for example to add weathering effects to corners.
Note: For Cycles, this may slow down renders significantly

- `samples` · int — Number of rays to trace per shader evaluation
- `inside` · boolean — Trace rays towards the inside of the object
- `only_local` · boolean — Only consider the object itself when computing AO

## ShaderNodeAttribute

Retrieve attributes attached to objects or geometry

- `attribute_type` · enum · {GEOMETRY, OBJECT, INSTANCER, VIEW_LAYER} — General type of the attribute
- `attribute_name` · string

## ShaderNodeBevel

Generates normals with round corners.
Note: only supported in Cycles, and may slow down renders

- `samples` · int — Number of rays to trace per shader evaluation

## ShaderNodeBsdfAnisotropic

Reflection with microfacet distribution, used for materials such as metal or mirrors

- `distribution` · enum · {BECKMANN, GGX, ASHIKHMIN_SHIRLEY, MULTI_GGX} — Light scattering distribution on rough surface

## ShaderNodeBsdfGlass

Glass-like shader mixing refraction and reflection at grazing angles

- `distribution` · enum · {BECKMANN, GGX, MULTI_GGX} — Light scattering distribution on rough surface

## ShaderNodeBsdfHair

Reflection and transmission shaders optimized for hair rendering

- `component` · enum · {Reflection, Transmission} — Hair BSDF component to use

## ShaderNodeBsdfHairPrincipled

Physically-based, easy-to-use shader for rendering hair and fur

- `model` · enum · {CHIANG, HUANG} — Select from Chiang or Huang model
- `parametrization` · enum · {ABSORPTION, MELANIN, COLOR} — Select the shader's color parametrization

## ShaderNodeBsdfMetallic

Metallic reflection with microfacet distribution, and metallic fresnel

- `distribution` · enum · {BECKMANN, GGX, MULTI_GGX} — Light scattering distribution on rough surface
- `fresnel_type` · enum · {PHYSICAL_CONDUCTOR, F82} — Fresnel method used to tint the metal

## ShaderNodeBsdfPrincipled

Physically-based, easy-to-use shader for rendering surface materials, based on the OpenPBR model

- `distribution` · enum · {GGX, MULTI_GGX} — Light scattering distribution on rough surface
- `subsurface_method` · enum · {BURLEY, RANDOM_WALK, RANDOM_WALK_SKIN, RANDOM_WALK_LEGACY} — Method for rendering subsurface scattering

## ShaderNodeBsdfRefraction

Glossy refraction with sharp or microfacet distribution, typically used for materials that transmit light

- `distribution` · enum · {BECKMANN, GGX} — Light scattering distribution on rough surface

## ShaderNodeBsdfSheen

Reflection for materials such as cloth.
Typically mixed with other shaders (such as a Diffuse Shader) and is not particularly useful on its own

- `distribution` · enum · {ASHIKHMIN, MICROFIBER} — Sheen shading model

## ShaderNodeBsdfToon

Diffuse and Glossy shaders with cartoon light effects

- `component` · enum · {DIFFUSE, GLOSSY} — Toon BSDF component to use

## ShaderNodeBump

Generate a perturbed normal from a height texture for bump mapping. Typically used for faking highly detailed surfaces

- `invert` · boolean — Invert the bump mapping direction to push into the surface instead of out

## ShaderNodeClamp

Clamp a value between a minimum and a maximum

- `clamp_type` · enum · {MINMAX, RANGE}

## ShaderNodeCombineColor

Create a color from individual components using multiple models

- `mode` · enum · {RGB, HSV, HSL} — Mode of color processing

## ShaderNodeCustomGroup

Custom Shader Group Node for Python nodes

- `node_tree` · pointer · -> NodeTree

## ShaderNodeDisplacement

Displace the surface along the surface normal

- `space` · enum · {OBJECT, WORLD} — Space of the input height

## ShaderNodeFloatCurve

Map an input float to a curve and outputs a float value

- `mapping` · pointer · -> CurveMapping · read-only

## ShaderNodeGroup

- `node_tree` · pointer · -> NodeTree

## ShaderNodeMapRange

Remap a value from a range to a target range

- `clamp` · boolean — Clamp the result to the target range [To Min, To Max]
- `interpolation_type` · enum · {LINEAR, STEPPED, SMOOTHSTEP, SMOOTHERSTEP}
- `data_type` · enum · {FLOAT, FLOAT_VECTOR}

## ShaderNodeMapping

Transform the input vector by applying translation, rotation, and scale

- `vector_type` · enum · {POINT, TEXTURE, VECTOR, NORMAL} — Type of vector that the mapping transforms

## ShaderNodeMath

Perform math operations

- `operation` · enum · {ADD, SUBTRACT, MULTIPLY, DIVIDE, MULTIPLY_ADD, POWER, LOGARITHM, SQRT, INVERSE_SQRT, ABSOLUTE, EXPONENT, MINIMUM, MAXIMUM, LESS_THAN, GREATER_THAN, SIGN, COMPARE, SMOOTH_MIN, SMOOTH_MAX, ROUND, FLOOR, CEIL, TRUNC, FRACT, MODULO, FLOORED_MODULO, WRAP, SNAP, PINGPONG, SINE, COSINE, TANGENT, ARCSINE, ARCCOSINE, ARCTANGENT, ARCTAN2, SINH, COSH, TANH, RADIANS, ... (41 total)}
- `use_clamp` · boolean — Clamp result of the node to 0.0 to 1.0 range

## ShaderNodeMix

Mix values by a factor

- `data_type` · enum · {FLOAT, VECTOR, RGBA, ROTATION}
- `factor_mode` · enum · {UNIFORM, NON_UNIFORM}
- `blend_type` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE}
- `clamp_factor` · boolean — Clamp the factor to [0,1] range
- `clamp_result` · boolean — Clamp the result to [0,1] range

## ShaderNodeMixRGB

Mix two input colors

- `blend_type` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE}
- `use_alpha` · boolean — Include alpha of second input in this operation
- `use_clamp` · boolean — Clamp result of the node to 0.0 to 1.0 range

## ShaderNodeNormalMap

Generate a perturbed normal from an RGB normal map image. Typically used for faking highly detailed surfaces

- `space` · enum · {TANGENT, OBJECT, WORLD, BLENDER_OBJECT, BLENDER_WORLD} — Space of the input normal
- `uv_map` · string — UV Map for tangent space maps
- `convention` · enum · {OPENGL, DIRECTX} — OpenGL or DirectX
- `base` · enum · {ORIGINAL, DISPLACED} — Base surface that the normal map is applied relative to, when displacement is used

## ShaderNodeOutputAOV

Arbitrary Output Variables.
Provide custom render passes for arbitrary shader node outputs

- `aov_name` · string — Name of the AOV that this output writes to

## ShaderNodeOutputLight

Output light information to a light object

- `is_active_output` · boolean — True if this node is used as the active output
- `target` · enum · {ALL, EEVEE, CYCLES} — Which renderer and viewport shading types to use the shaders for

## ShaderNodeOutputLineStyle

Control the mixing of texture information into the base color of line styles

- `is_active_output` · boolean — True if this node is used as the active output
- `target` · enum · {ALL, EEVEE, CYCLES} — Which renderer and viewport shading types to use the shaders for
- `blend_type` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE}
- `use_alpha` · boolean — Include alpha of second input in this operation
- `use_clamp` · boolean — Clamp result of the node to 0.0 to 1.0 range

## ShaderNodeOutputMaterial

Output surface material information for use in rendering

- `is_active_output` · boolean — True if this node is used as the active output
- `target` · enum · {ALL, EEVEE, CYCLES} — Which renderer and viewport shading types to use the shaders for

## ShaderNodeOutputWorld

Output light color information to the scene's World

- `is_active_output` · boolean — True if this node is used as the active output
- `target` · enum · {ALL, EEVEE, CYCLES} — Which renderer and viewport shading types to use the shaders for

## ShaderNodeRGBCurve

Apply color corrections for each color channel

- `mapping` · pointer · -> CurveMapping · read-only

## ShaderNodeRadialTiling

Transform Coordinate System for Radial Tiling

- `normalize` · boolean — Normalize the X coordinate of the Segment Coordinates output to a [0, 1] interval and offset the Y coordinate into a [0, infinity) interval. When checked, the textures are stretched to fit into each angular segment. When not checked, the parts of the textures that don't fit into each angular segment are cropped

## ShaderNodeRaycast

Cast rays and retrieve information from the hit point

- `only_local` · boolean — Only raycast against the object itself
- `sample_attribute_items` · collection · [] of NodeRaycastSampleAttributeItems · read-only
- `active_index` · int — Index of the active item
- `active_item` · pointer · -> RepeatItem — Index of the active item

## ShaderNodeScript

Generate an OSL shader from a file or text data-block.
Note: OSL shaders are not supported on all GPU backends

- `script` · pointer · -> Text — Internal shader script to define the shader
- `filepath` · string — Shader script path
- `mode` · enum · {INTERNAL, EXTERNAL}
- `use_auto_update` · boolean — Automatically update the shader when the .osl file changes (external scripts only)
- `bytecode` · string — Compile bytecode for shader script node
- `bytecode_hash` · string — Hash of compile bytecode, for quick equality checking

## ShaderNodeSeparateColor

Split a color into its individual components using multiple models

- `mode` · enum · {RGB, HSV, HSL} — Mode of color processing

## ShaderNodeSubsurfaceScattering

Subsurface multiple scattering shader to simulate light entering the surface and bouncing internally.
Typically used for materials such as skin, wax, marble or milk

- `falloff` · enum · {BURLEY, RANDOM_WALK, RANDOM_WALK_SKIN, RANDOM_WALK_LEGACY} — Method for rendering subsurface scattering

## ShaderNodeTangent

Generate a tangent direction for the Anisotropic BSDF

- `direction_type` · enum · {RADIAL, UV_MAP} — Method to use for the tangent
- `axis` · enum · {X, Y, Z} — Axis for radial tangents
- `uv_map` · string — UV Map for tangent generated from UV

## ShaderNodeTexBrick

Generate a procedural texture producing bricks

- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings
- `offset_frequency` · int — How often rows are offset. A value of 2 gives an even/uneven pattern of rows.
- `squash_frequency` · int — How often rows consist of "squished" bricks
- `offset` · float — Determines the brick offset of the various rows
- `squash` · float — Factor to adjust the brick's width for particular rows determined by the Offset Frequency

## ShaderNodeTexChecker

Generate a checkerboard texture

- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings

## ShaderNodeTexCoord

Retrieve multiple types of texture coordinates.
Typically used as inputs for texture nodes

- `object` · pointer · -> Object — Use coordinates from this object (for object texture coordinates output)
- `from_instancer` · boolean — Use the parent of the instance object if possible

## ShaderNodeTexEnvironment

Sample an image file as an environment texture. Typically used to light the scene with the background node

- `image` · pointer · -> Image
- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings
- `projection` · enum · {EQUIRECTANGULAR, MIRROR_BALL} — Projection of the input image
- `interpolation` · enum · {Linear, Closest, Cubic, Smart} — Texture interpolation
- `image_user` · pointer · -> ImageUser · read-only — Parameters defining which layer, pass and frame of the image is displayed

## ShaderNodeTexGabor

Generate Gabor noise

- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings
- `gabor_type` · enum · {2D, 3D} — The type of Gabor noise to evaluate

## ShaderNodeTexGradient

Generate interpolated color and intensity values based on the input vector

- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings
- `gradient_type` · enum · {LINEAR, QUADRATIC, EASING, DIAGONAL, SPHERICAL, QUADRATIC_SPHERE, RADIAL} — Style of the color blending

## ShaderNodeTexIES

Match real world lights with IES files, which store the directional intensity distribution of light sources

- `ies` · pointer · -> Text — Internal IES file
- `filepath` · string — IES light path
- `mode` · enum · {INTERNAL, EXTERNAL} — Whether the IES file is loaded from disk or from a text data-block

## ShaderNodeTexImage

Sample an image file as a texture

- `image` · pointer · -> Image
- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings
- `projection` · enum · {FLAT, BOX, SPHERE, TUBE} — Method to project 2D image on object with a 3D texture vector
- `interpolation` · enum · {Linear, Closest, Cubic, Smart} — Texture interpolation
- `projection_blend` · float — For box projection, amount of blend to use between sides
- `extension` · enum · {REPEAT, EXTEND, CLIP, MIRROR} — How the image is extrapolated past its original bounds
- `image_user` · pointer · -> ImageUser · read-only — Parameters defining which layer, pass and frame of the image is displayed

## ShaderNodeTexMagic

Generate a psychedelic color texture

- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings
- `turbulence_depth` · int — Level of detail in the added turbulent noise

## ShaderNodeTexNoise

Generate fractal Perlin noise

- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings
- `noise_dimensions` · enum · {1D, 2D, 3D, 4D} — Number of dimensions to output noise for
- `noise_type` · enum · {MULTIFRACTAL, RIDGED_MULTIFRACTAL, HYBRID_MULTIFRACTAL, FBM, HETERO_TERRAIN} — Type of the Noise texture
- `normalize` · boolean — Normalize outputs to 0.0 to 1.0 range

## ShaderNodeTexSky

Generate a procedural sky texture

- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings
- `sky_type` · enum · {SINGLE_SCATTERING, MULTIPLE_SCATTERING, PREETHAM, HOSEK_WILKIE} — Which sky model should be used
- `sun_disc` · boolean — Include the sun itself in the output
- `sun_size` · float — Size of sun disc
- `sun_intensity` · float — Strength of Sun
- `sun_elevation` · float — Sun angle from horizon
- `sun_rotation` · float — Rotation of sun around zenith
- `altitude` · float — Height from sea level
- `air_density` · float — Density of air molecules.
0 means no air, 1 means urban city air
- `aerosol_density` · float — Density of dust, pollution and water droplets.
0 means no aerosols, 1 means urban city aerosols
- `ozone_density` · float — Density of ozone layer.
0 means no ozone, 1 means urban city ozone
- `sun_direction` · float · [3] — Direction from where the sun is shining
- `turbidity` · float — Atmospheric turbidity
- `ground_albedo` · float — Ground color that is subtly reflected in the sky

## ShaderNodeTexVoronoi

Generate Worley noise based on the distance to random points. Typically used to generate textures such as stones, water, or biological cells

- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings
- `voronoi_dimensions` · enum · {1D, 2D, 3D, 4D} — Number of dimensions to output noise for
- `distance` · enum · {EUCLIDEAN, MANHATTAN, CHEBYCHEV, MINKOWSKI} — The distance metric used to compute the texture
- `feature` · enum · {F1, F2, SMOOTH_F1, DISTANCE_TO_EDGE, N_SPHERE_RADIUS} — The Voronoi feature that the node will compute
- `normalize` · boolean — Normalize output Distance to 0.0 to 1.0 range

## ShaderNodeTexWave

Generate procedural bands or rings with noise

- `texture_mapping` · pointer · -> TexMapping · read-only — Texture coordinate mapping settings
- `color_mapping` · pointer · -> ColorMapping · read-only — Color mapping settings
- `wave_type` · enum · {BANDS, RINGS}
- `bands_direction` · enum · {X, Y, Z, DIAGONAL}
- `rings_direction` · enum · {X, Y, Z, SPHERICAL}
- `wave_profile` · enum · {SIN, SAW, TRI}

## ShaderNodeTexWhiteNoise

Calculate a random value or color based on an input seed

- `noise_dimensions` · enum · {1D, 2D, 3D, 4D} — Number of dimensions to output noise for

## ShaderNodeUVAlongStroke

UV coordinates that map a texture along the stroke length

- `use_tips` · boolean — Lower half of the texture is for tips of the stroke

## ShaderNodeUVMap

Retrieve a UV map from the geometry, or the default fallback if none is specified

- `from_instancer` · boolean — Use the parent of the instance object if possible
- `uv_map` · string — UV coordinates to be used for mapping

## ShaderNodeValToRGB

Map values to colors with the use of a gradient

- `color_ramp` · pointer · -> ColorRamp · read-only

## ShaderNodeVectorCurve

Map input vector components with curves

- `mapping` · pointer · -> CurveMapping · read-only

## ShaderNodeVectorDisplacement

Displace the surface along an arbitrary direction

- `space` · enum · {TANGENT, OBJECT, WORLD} — Space of the input height

## ShaderNodeVectorMath

Perform vector math operation

- `operation` · enum · {ADD, SUBTRACT, MULTIPLY, DIVIDE, MULTIPLY_ADD, CROSS_PRODUCT, PROJECT, REFLECT, REFRACT, FACEFORWARD, DOT_PRODUCT, DISTANCE, LENGTH, SCALE, NORMALIZE, ABSOLUTE, POWER, SIGN, MINIMUM, MAXIMUM, ROUND, FLOOR, CEIL, FRACTION, MODULO, WRAP, SNAP, SINE, COSINE, TANGENT}

## ShaderNodeVectorRotate

Rotate a vector around a pivot point (center)

- `rotation_type` · enum · {AXIS_ANGLE, X_AXIS, Y_AXIS, Z_AXIS, EULER_XYZ} — Type of angle input
- `invert` · boolean — Invert the rotation angle

## ShaderNodeVectorTransform

Convert a vector, point, or normal between world, camera, and object coordinate space

- `vector_type` · enum · {POINT, VECTOR, NORMAL}
- `convert_from` · enum · {WORLD, OBJECT, CAMERA} — Space to convert from
- `convert_to` · enum · {WORLD, OBJECT, CAMERA} — Space to convert to

## ShaderNodeVertexColor

Retrieve a color attribute, or the default fallback if none is specified

- `layer_name` · string — Color Attribute

## ShaderNodeVolumeCoefficients

Model all three physical processes in a volume, represented by their coefficients

- `phase` · enum · {HENYEY_GREENSTEIN, FOURNIER_FORAND, DRAINE, RAYLEIGH, MIE} — Phase function for the scattered light

## ShaderNodeVolumeScatter

Scatter light as it passes through the volume, often used to add fog to a scene

- `phase` · enum · {HENYEY_GREENSTEIN, FOURNIER_FORAND, DRAINE, RAYLEIGH, MIE} — Phase function for the scattered light

## ShaderNodeWireframe

Retrieve the edges of an object as it appears to Cycles.
Note: as meshes are triangulated before being processed by Cycles, topology will always appear triangulated

- `use_pixel_size` · boolean — Use screen pixel size instead of world units

## SpaceNodeEditor

Node editor space data

- `show_region_toolbar` · boolean
- `show_region_ui` · boolean
- `show_region_asset_shelf` · boolean — Display a region with assets that may currently be relevant (such as brushes in paint modes, or poses in Pose Mode)
- `tree_type` · enum · {GeometryNodeTree, CompositorNodeTree, ShaderNodeTree, TextureNodeTree} — Node tree type to display and edit
- `texture_type` · enum · {WORLD, BRUSH, LINESTYLE} — Type of data to take texture from
- `shader_type` · enum · {OBJECT, WORLD, LINESTYLE} — Type of data to take shader from
- `node_tree_sub_type` · enum · {}
- `id` · pointer · -> ID · read-only — Data-block whose nodes are being edited
- `id_from` · pointer · -> ID · read-only — Data-block from which the edited data-block is linked
- `path` · collection · [] of SpaceNodeEditorPath · read-only — Path from the data-block to the currently edited node tree
- `node_tree` · pointer · -> NodeTree — Base node tree from context
- `edit_tree` · pointer · -> NodeTree · read-only — Node tree being displayed and edited
- `pin` · boolean — Use the pinned node tree
- `show_backdrop` · boolean — Use active Viewer Node output as backdrop for compositing nodes
- `selected_node_group` · pointer · -> NodeTree — Node group to edit
- `show_annotation` · boolean — Show annotations for this view
- `backdrop_zoom` · float — Backdrop zoom factor
- `backdrop_offset` · float · [2] — Backdrop offset
- `backdrop_channels` · enum · {COLOR_ALPHA, COLOR, ALPHA, RED, GREEN, BLUE} — Channels of the image to draw
- `cursor_location` · float · [2] — Location for adding new nodes
- `insert_offset_direction` · enum · {RIGHT, LEFT} — Direction to offset nodes on insertion
- `show_gizmo` · boolean — Show gizmos of all types
- `show_gizmo_active_node` · boolean — Context sensitive gizmo for the active node
- `overlay` · pointer · -> SpaceNodeOverlay · read-only — Settings for display of overlays in the Node Editor
- `supports_previews` · boolean · read-only — Whether the node editor's type supports displaying node previews

## SpaceNodeEditorPath

Get the node tree path as a string

- `to_string` · string · read-only

## SpaceNodeOverlay

Settings for display of overlays in the Node Editor

- `show_overlays` · boolean — Display overlays like colored or dashed wires
- `show_wire_color` · boolean — Color node links based on their connected sockets
- `show_reroute_auto_labels` · boolean — Label reroute nodes based on the label of connected reroute nodes
- `show_timing` · boolean — Display each node's last execution time
- `show_context_path` · boolean — Display breadcrumbs for the editor's context
- `show_named_attributes` · boolean — Show when nodes are using named attributes
- `show_previews` · boolean — Display each node's preview if node is toggled
- `preview_shape` · enum · {FLAT, 3D} — Preview shape used by the node previews
- `show_render_size` · boolean — Display the region of the final render
- `passepartout_alpha` · float — Opacity of the darkened overlay outside the render region

## TextureNodeBricks

- `offset` · float — Determines the brick offset of the various rows
- `offset_frequency` · int — Offset every N rows
- `squash` · float — Factor to adjust the brick's width for particular rows determined by the Offset Frequency
- `squash_frequency` · int — Squash every N rows

## TextureNodeCombineColor

- `mode` · enum · {RGB, HSV, HSL} — Mode of color processing

## TextureNodeCurveRGB

- `mapping` · pointer · -> CurveMapping · read-only

## TextureNodeCurveTime

- `curve` · pointer · -> CurveMapping · read-only

## TextureNodeGroup

- `node_tree` · pointer · -> NodeTree

## TextureNodeImage

- `image` · pointer · -> Image
- `image_user` · pointer · -> ImageUser · read-only — Parameters defining the image duration, offset and related settings

## TextureNodeMath

- `operation` · enum · {ADD, SUBTRACT, MULTIPLY, DIVIDE, MULTIPLY_ADD, POWER, LOGARITHM, SQRT, INVERSE_SQRT, ABSOLUTE, EXPONENT, MINIMUM, MAXIMUM, LESS_THAN, GREATER_THAN, SIGN, COMPARE, SMOOTH_MIN, SMOOTH_MAX, ROUND, FLOOR, CEIL, TRUNC, FRACT, MODULO, FLOORED_MODULO, WRAP, SNAP, PINGPONG, SINE, COSINE, TANGENT, ARCSINE, ARCCOSINE, ARCTANGENT, ARCTAN2, SINH, COSH, TANH, RADIANS, ... (41 total)}
- `use_clamp` · boolean — Clamp result of the node to 0.0 to 1.0 range

## TextureNodeMixRGB

- `blend_type` · enum · {MIX, DARKEN, MULTIPLY, BURN, LIGHTEN, SCREEN, DODGE, ADD, OVERLAY, SOFT_LIGHT, LINEAR_LIGHT, DIFFERENCE, EXCLUSION, SUBTRACT, DIVIDE, HUE, SATURATION, COLOR, VALUE}
- `use_alpha` · boolean — Include alpha of second input in this operation
- `use_clamp` · boolean — Clamp result of the node to 0.0 to 1.0 range

## TextureNodeOutput

- `filepath` · string

## TextureNodeSeparateColor

- `mode` · enum · {RGB, HSV, HSL} — Mode of color processing

## TextureNodeTexture

- `texture` · pointer · -> Texture
- `node_output` · int — For node-based textures, which output node to use

## TextureNodeValToRGB

- `color_ramp` · pointer · -> ColorRamp · read-only

## ThemeNodeEditor

Theme settings for the Node Editor

- `space` · pointer · -> ThemeSpaceGeneric · read-only — Settings for space
- `grid` · float · [3]
- `node_outline` · float · [4]
- `node_selected` · float · [3]
- `node_active` · float · [3]
- `wire` · float · [4]
- `wire_inner` · float · [3]
- `wire_select` · float · [4]
- `node_backdrop` · float · [4]
- `converter_node` · float · [3]
- `color_node` · float · [3]
- `group_node` · float · [3]
- `group_socket_node` · float · [3]
- `frame_node` · float · [4]
- `matte_node` · float · [3]
- `distor_node` · float · [3]
- `noodle_curving` · int — Curving of the noodle
- `grid_levels` · int — Number of subdivisions for the dot grid displayed in the background
- `dash_alpha` · float — Opacity for the dashed lines in wires
- `input_node` · float · [3]
- `output_node` · float · [3]
- `filter_node` · float · [3]
- `vector_node` · float · [3]
- `texture_node` · float · [3]
- `shader_node` · float · [3]
- `script_node` · float · [3]
- `geometry_node` · float · [3]
- `attribute_node` · float · [3]
- `simulation_zone` · float · [4]
- `repeat_zone` · float · [4]
- `foreach_geometry_element_zone` · float · [4]
- `closure_zone` · float · [4]

## ViewerNodeViewerPathElem

- `node_id` · int
