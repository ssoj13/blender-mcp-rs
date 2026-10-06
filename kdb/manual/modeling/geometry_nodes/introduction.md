---
area: manual-modeling
source: blender-manual
version: 5.2
---

************
Introduction
************

Geometry Nodes is a system for modifying the geometry of an object with node-based operations.
It can be accessed by adding a Geometry Nodes Modifier </modeling/modifiers/geometry_nodes>
and setting up the nodes in the /editors/geometry_node.

The geometry node tree connected to a modifier is a Node Group </interface/controls/nodes/groups>.
The geometry from the state before the modifier (the original geometry or the result of the previous
modifier) will be passed to the *Group Input* node. Then the node group can operate on the geometry
and pass an output to the *Group Output* node, where it will be passed to the next modifier.

Geometry nodes can modify different types of geometry:

- Empties </modeling/empties>
- Meshes </modeling/meshes/introduction>
- Curves </modeling/curves/introduction>
- Point Clouds </modeling/point_cloud/index>
- Volumes </modeling/volumes/introduction>
- Instances </modeling/geometry_nodes/instances>

The interface of the modifier is described in the
Modifier </modeling/modifiers/geometry_nodes> page.

To expand Blender with node-group operators, see the /modeling/geometry_nodes/tools page.
