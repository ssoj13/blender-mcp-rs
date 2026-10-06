---
area: manual-physics
source: blender-manual
version: 5.2
---

************
Introduction
************

Liquid Simulations
==================

Fluid physics are used to simulate physical properties of liquids especially water.
While creating a scene in Blender, certain objects can be marked to become a part of the fluid simulation.
For a fluid simulation you have to have a domain to define the space that the simulation takes up.
In the domain settings you will be able to define the global simulation parameters (such as viscosity
and gravity).

Gas Simulations
===============

Gas or smoke simulations are a subset of the fluids system, and can be used for simulating collections
of airborne solids, liquid particulates and gases, such as those that make up smoke.
It simulates the fluid movement of air and generates animated Voxel
textures representing the density, heat, and velocity of other fluids or suspended particles
(e.g. smoke) which can be used for rendering.

Gases or smoke are emitted inside of a Domain </physics/fluid/type/domain/index> from
a mesh object or particle system. The smoke movement is controlled by airflow inside the domain,
which can be influenced by Effector </physics/fluid/type/effector> objects.
Smoke will also be affected by the scene's gravity and force fields </physics/forces/force_fields/index>.
Airflow inside the domain can affect other physics simulations
via the Fluid Flow </physics/forces/force_fields/types/fluid_flow> force field.

Workflow
========

At least a Domain </physics/fluid/type/domain/index> object and
one Flow </physics/fluid/type/flow> object are required to create a fluid simulation.

#. Create a Domain object </physics/fluid/type/domain/index>
   that defines the bounds of the simulation volume.
#. Set up Flow </physics/fluid/type/flow> objects which will emit fluid.
#. Set up Effector </physics/fluid/type/effector> objects to make
   the fluid interact with objects in the scene.
#. Assign a material </physics/fluid/material> to the domain object.
#. Save the blend-file.
#. Bake the Cache </physics/fluid/type/domain/cache> for the simulation.

**Note:**

   There are Quick Liquid and Quick Smoke <bpy.ops.object.quick> tools
   which will automatically create a domain object with a basic liquid or smoke and fire material.
