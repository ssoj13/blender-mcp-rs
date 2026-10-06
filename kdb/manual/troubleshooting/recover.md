---
area: manual-troubleshooting
source: blender-manual
version: 5.2
---

***************
Recovering Data
***************

Computer crashes, power outages, or simply forgetting to save can result in the loss or corruption of your work. You
can use Blender's *Auto Save* feature to reduce the chance of losing files when such events occur.

There are three options to help prevent accidental data loss:

- *Save Prompt* prompts you to save unsaved changes on exit
- *Save Versions* saves additional copies of your blend file with a number appended to the extension
- *Auto Save* saves your file automatically every few minutes (by default, every 2 minutes)

**Note:**

   For your actions, there are options like *Undo*, *Redo* and an *Undo History*, used to roll back from mistakes
   under normal operation, or return back to a specific action. See /interface/undo_redo.

.. _troubleshooting-file_recovery-save_versions:

Recovering Save Versions
========================

By default Blender keeps an additional backup when saving files.
So saving renames the previously saved file with a `.blend1` extension instead of overwriting it.

This file can be used to revert to a previous state.

See Save Versions <bpy.types.PreferencesFilePaths.save_version> to configure the number of versions kept.

Recovering The Last Session
===========================

The *Recover Last Session* will open the `quit.blend` file that is saved into the temp-dir when you quit
Blender under normal operation (see Blender Session). Note that files in your temporary directory may be
deleted when you reboot your computer (depending on your system configuration) or by scheduled disk cleanup tools or
scripts.

.. _troubleshooting-file_recovery-auto_save:

Recovering an Auto Save
=======================

Blender automatically saves temporary backups of your work at regular intervals.
If Blender crashes or you close it without saving, you may be able to recover your work using the autosave.

Follow these steps to recover an autosave:

#. Open Blender.
#. Go to the top-left menu bar and select: File --> Recover --> Auto Save <bpy.ops.wm.recover_auto_save>.
#. In the file browser that appears, find the desired auto save.
   The files are named like: <filename>_autosave.blend
   Note, these files are timestamped to help identify the correct one.
#. Select the autosave file and click *Open*.
#. Once opened, immediately save the file manually using
   Save As <bpy.ops.wm.save_as_mainfile> to avoid overwriting the autosave.

**Tip:**

   Enable the detailed list view when browsing auto-saved files to show which is the most recent.

**Seealso:**

   Auto Save Interval Preference
