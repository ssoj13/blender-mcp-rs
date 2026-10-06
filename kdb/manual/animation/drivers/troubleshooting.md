---
area: manual-animation
source: blender-manual
version: 5.2
---

***************
Troubleshooting
***************

Some common problems people may run into when using drivers.

Scripted Expression
===================

By default Blender will restrict execution of Python scripts.

If using a *Scripted Expression* Driver Type that doesn't follow
the Simple Expressions <drivers-simple-expressions>
subset, you will have to open the file as *Trusted Source*,
or set *Auto Run Python Scripts* in Preferences --> Save & Load --> Blender Files.

.. list-table::
   :widths: 40 60

   * - .. figure:: /images/animation_drivers_troubleshooting_autorun-file-browser.png

          The Trusted Source checkbox in the File Browser.

     - .. figure:: /images/animation_drivers_troubleshooting_autorun-user-preference.png

          The Auto Run Python Scripts checkbox in the Preferences.

Rotational Properties are Radians
=================================

Parts of the User Interface may use different units of measurements for angles, rotation.
In the Graph Editor, while working with Drivers, all angles are Radians.
