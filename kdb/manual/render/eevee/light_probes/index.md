---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.ops.object.lightprobe:
.. _bpy.types.LightProbe:

################
  Light Probes
################

Light probe objects are used by EEVEE as support objects.

There are three different types of light probes.
Each type of light probe records the lighting at a different resolution and frequency.
Probes are used together to recover incoming light information when using ray tracing is not possible
(either for performance or for technical limitations).

These types of objects are only useful for EEVEE (and by extension, the Material Preview mode).

Types
=====
