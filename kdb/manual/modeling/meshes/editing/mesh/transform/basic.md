---
area: manual-modeling
source: blender-manual
version: 5.2
---

*******************
Move, Rotate, Scale
*******************

Move G, rotate R, or scale S the selected vertices, edges, or faces.
This can also be done using the transform gizmos </editors/3dview/display/gizmo>.

**Seealso:**

   - Manipulation in 3D Space
   - /editors/3dview/controls/orientation
   - /editors/3dview/controls/pivot_point/index

These transformations are affected by /editors/3dview/controls/proportional_editing.

Pressing G twice will instead activate
Vertex Slide </modeling/meshes/editing/vertex/slide_vertices>
or Edge Slide </modeling/meshes/editing/edge/edge_slide> depending on the selection.

.. _modeling-mesh-transform-panel:

Transform Panel
===============

This panel shows the properties of the selected vertex or edge.
If more than one item is selected, it shows the average of their properties instead.

X/Y/Z
   The coordinates of the selected vertex. (When multiple vertices are selected, the label
   says "Median," even though technically speaking the panel does *not* show the
   `geometric median <https://en.wikipedia.org/wiki/Geometric_median>`__
   but the `geometric center <https://en.wikipedia.org/wiki/Centroid>`__.)
Space
   Whether to show the coordinates in Local Space or Global Space.

Vertex Data
-----------

.. _modeling-vertex-bevel-weight:

Bevel Weight
   Typically used with the /modeling/modifiers/generate/bevel (with *Affect* set to
   *Vertices*). By default, this modifier applies the same bevel amount to all vertices,
   but if its *Limit Method* is set to *Weight*, the Bevel Weight of each vertex serves as
   a multiplier. This makes it possible to apply a smaller bevel, or no bevel at all,
   to specific vertices.

   Internally, the Bevel Weight is stored as an
   attribute
   called `bevel_weight_vert`. This means it can also be read by the
   /modeling/geometry_nodes/geometry/read/named_attribute in
   geometry nodes </modeling/geometry_nodes/introduction>.

.. _modeling-vertex-crease-subdivision:

Crease
   A higher Crease value makes the corner at the vertex appear sharper when using the
   /modeling/modifiers/generate/subdivision_surface.

   Internally, the Crease value is stored as an attribute called `crease_vert`.

Radius X/Y
   The radii for the /modeling/modifiers/generate/skin.
   Only shown if the mesh has this modifier.

Edge Data
---------

When an edge is selected, the following options are available:

.. _modeling-edges-bevel-weight:

Bevel Weight
   Typically used with the /modeling/modifiers/generate/bevel (with *Affect* set to
   *Edges*). See the vertex *Bevel Weight* above.

   Internally, this property is stored as an attribute called `bevel_weight_edge`.
   It can also be set using the bpy.ops.transform.edge_bevelweight operator.

.. todo move to attribute page
.. _modeling-edges-crease-subdivision:

Crease
   A higher Crease value makes the corner at the edge appear sharper when using the
   /modeling/modifiers/generate/subdivision_surface.

   Internally, this property is stored as an attribute called `crease_edge`.
   It can also be set using the bpy.ops.transform.edge_crease operator.
