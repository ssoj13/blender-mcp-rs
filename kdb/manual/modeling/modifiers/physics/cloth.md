---
area: manual-modeling
source: blender-manual
version: 5.2
---

**************
Cloth Modifier
**************

The Cloth modifier is a container for a Cloth Physics </physics/cloth/index> simulation. It can be useful
for example, to simulate on a low-poly mesh then add a /modeling/modifiers/generate/subdivision_surface
after the Cloth modifier to improve the visual quality of the cloth without drastically increasing simulation times.

Options
=======

As the modifier is only a container its actual options can be configured in the *Physics Properties* tab.
See the Cloth Physics Properties </physics/cloth/settings/index> for more information.

Example
=======

.. list-table::

   * - .. figure:: /images/physics_cloth_introduction_example1.jpg

          Cloth example.

     - .. figure:: /images/physics_cloth_introduction_oncarved-wood.jpg

          Cloth on carved wooden men (made by motorsep).

     - .. figure:: /images/physics_cloth_introduction_example2.jpg

          Cloth example.
