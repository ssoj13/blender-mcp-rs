---
area: manual-render
source: blender-manual
version: 5.2
---

.. _bpy.types.CyclesRenderSettings:

###################
  Render Settings
###################

.. _bpy.types.CyclesRenderSettings.device:

Device
   Specifies the compute device used for rendering in Cycles.
   The choice of device can significantly affect render performance, *GPU Compute* is often faster .

   :CPU:
      Render using the system's central processing unit. This mode is compatible with all Cycles features.
      The number of CPU threads used can be changed in the
      Performance Properties </render/cycles/render_settings/performance>.
   :GPU Compute:
      Render using one or more graphics processing units (GPUs) through the compute backend
      and devices selected in the editors_preferences_cycles.
      Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously.

      For more details, including setup instructions, device compatibility, and known limitations,
      see /render/cycles/gpu_rendering.

**Note:**

      GPU rendering may have limitations compared to CPU rendering, depending on the backend.

**Seealso:**

      Denoising
      and Compositing <bpy.types.RenderSettings.compositor_device> have their own device selection.

.. _bpy.types.CyclesRenderSettings.shading_system:

Open Shading Language
   Enables the use of /render/cycles/osl/index which allows custom shading using shader scripts.
   When OSL is enabled, you can:

   - Use OSL shader nodes in materials using the Script Node </render/shader_nodes/utilities/script>.
   - Write custom shading logic that is not possible with built-in Cycles nodes.

   Limitations:

   - OSL is supported only when rendering with the *CPU* or with the
     OptiX <render-cycles-gpu-optix> GPU Compute backend.
   - Rendering performance is typically slower than with built-in nodes due to shader compilation and interpretation.
   - Some features, such as baking and certain volume shaders, may not work with OSL.
