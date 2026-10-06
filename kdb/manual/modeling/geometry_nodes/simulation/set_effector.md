---
area: manual-modeling
source: blender-manual
version: 5.2
---

************
Set Effector
************

Add an effector definition to a geometry. Simulations can access this information and include the
effector in the simulation world.

Inputs
======

Geometry
   Geometry in which to store the effector.

Effector
   Bundle making up the effector definition. This can be generated using for example the
   Custom Effector </modeling/geometry_nodes/simulation/custom_effector> node.

Name
   Unique name of the effector. If the name is empty then the effector is stored as the default
   effector.

Outputs
=======

Geometry
   Geometry with the effector stored in its
   Geometry Bundle </modeling/geometry_nodes/geometry/write/set_geometry_bundle>.
