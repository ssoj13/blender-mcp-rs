---
area: manual-files
source: blender-manual
version: 5.2
---

****************
Opening & Saving
****************

Opening and saving blend-files is usually done using the File Browser </editors/file_browser>.

**Tip:**

   Blend-files can also be opened by dragging and dropping blend-files into the Blender window.
   This method also allows to link/append </files/linked_libraries/index> the file.

**Note:**

   By default, when exiting Blender or loading a new blend-file, if you have unsaved changes,
   a pop-up will ask you to either confirm discarding those changes, or save them.

   This behavior can be disabled with the *Save Prompt* option in the prefs-save-load section
   of the *Preferences*.

.. _bpy.ops.wm.open_mainfile:

Opening Files
=============

The upper text field displays the current directory path,
and the lower text field contains the selected filename.

Options
-------

.. _file-load-ui:

Load UI
   When enabled, the screen layout saved inside each blend-file is used,
   replacing the current layout and Workspaces </interface/window_system/workspaces>.
   Otherwise the file screen layout is ignored.

**Tip:**

      If you want to work on a blend-file using your own defaults, start a fresh Blender,
      open the File Browser and turn off the *Load UI* button, and open the desired file.

Trusted Source
   When enabled, Python scripts and drivers that may be included in the file will be run automatically.
   Enable this only if you created the file yourself,
   or you trust that the person who gave it to you did not include any malicious code with it.
   See Python Security </advanced/scripting/security> to configure default trust options.

.. _other-file-open-options:

Open Recent
===========

Displays a list of recently opened blend-files.
Hovering over items will show a preview, and information about the blend-file.
Select any of the file names in the list to open that blend-file.
When RMB on a listed item, a context menu will appear; One of the available options is *Open File Location*,
which will open that location in an OS file explorer or Finder window.

.. _bpy.ops.wm.clear_recent_files:

Clear Recent Files List
-----------------------

Removes items from the recent files list.

Remove
   Choose which type of items to remove.

   :All Items: Removes all recent files.
   :Items Not Found: Removes files from the list that cannot be found i.e. that have been moved or deleted.

Recover
=======

.. _bpy.ops.wm.recover_last_session:

Last Session
------------

This will load the `quit.blend` file Blender automatically saved just before exiting.
This option enables you to recover your last work session if, for example, you closed
Blender by accident.

.. _bpy.ops.wm.recover_auto_save:

Auto Save
---------

The *Auto Save* option allows you to recover the most recent automatically saved version of your file.
This is useful in the event of a crash or if you closed Blender without saving.

Selecting this option opens a file browser pointed to your system's temp-dir.
Auto-saved files typically have a name such as `<filename>_autosave.blend` or a random identifier,
and use the `.blend` extension.

You can configure the autosave interval and behavior in the
Auto Save Preferences <bpy.types.PreferencesFilePaths.use_auto_save_temporary_files>.

**Warning:**

   Auto save has some limitations, notably it will not save changes in Sculpt, Texture Paint, and Edit mode.

**Important:**

   Only one auto-saved file is kept per project.
   When recovering from an auto save, any changes made after the last save will be lost.
   Older auto saves are not retained.

.. _bpy.ops.wm.save_mainfile:

Saving Files
============

Saves the current blend-file.

If the blend-file has not been saved before, Blender automatically opens
the Save As <bpy.ops.wm.save_as_mainfile> file browser.

When saving a blend-file, Blender may also save modified external images
depending on the Save Modified Images <bpy.types.PreferencesFilePaths.save_modified_images>
preference. See Saving Modified Images <files-images-saving-modified>.

Save Incremental
================

Save the current Blender file with a numerically
incremented name that does not overwrite any existing files.

.. _bpy.ops.wm.save_as_mainfile:

Save As
=======

Choose a file path to save the blend-file to.

**Warning:**

   If a file with the same given name already exists,
   the text field will turn red as a warning that the file will be overwritten.

**Tip:**

   Use the *plus* or *minus* buttons to the right of the file name,
   or NumpadPlus, NumpadMinus to increase/decrease a number at the end of the file name
   (e.g. changing `file_01.blend` to `file_02.blend`).

Options
-------

.. _files-blend-compress:

Compress
   Reduces the file size of the resulting blend-file but takes longer to save and load.
   This option is useful for distributing files online and saving drive space for large projects.
   But it can cause slowdowns when quitting Blender,
   or under normal operation when auto-saving backup files.
   See files-linked_libraries-known_limitations-compression for more information.

**Hint:**

      The used compression algorithm is Zstandard.
      It is not unique to Blender so files can be compressed/decompressed with external tools.

Remap Relative
   This option remaps files-blend-relative_paths
   (such as linked libraries and images) when saving a file in a new location.
Save Copy
   Saves a copy of the actual working state but does not make the saved file active.

Save Copy
=========

Choose a file path to save the blend-file to, but return to editing the original file upon completion.
This can be used to save backups of the current working state without modifying the original file.

For options see bpy.ops.wm.save_as_mainfile.

.. _files-blend-relative_paths:

Relative Paths
==============

Many blend-files reference external images or other linked blend-files.
A path tells Blender where to look for these files.
If the external files are moved, the blend-file that references them will not look right.

When you specify one of these external files, the default option is to make the path relative.
Blender stores a partial path evaluated relative to the directory location of the referencing blend-file.
This choice helps when you need to reorganize folders or move your files.

With a relative path, you can move the blend-file to a new location provided
the externally linked files are moved along with it.
For example, you could send someone a folder that contains a blend-file
and a subfolder of external images that it references.

When relative paths are supported, the File Browser provides a *Relative Path* checkbox,
when entering the path into a text field, use a double slash prefix (`//`) to make it so.

Relative paths are the default but this can be changed
in the File </editors/preferences/file_paths> tab of the *Preferences*.

**Note:**

   You cannot use relative paths into a new *untitled* blend-file.
   Save it before linking to external files.

**Hint:**

   If it is necessary to relocate a blend-file relative to its linked resources,
   use Blender's File Save As
   function which has an option to *Remap Relative* file links.
