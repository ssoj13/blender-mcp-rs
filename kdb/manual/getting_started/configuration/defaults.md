---
area: manual-getting_started
source: blender-manual
version: 5.2
---

.. _splash-quick-start:

********
Defaults
********

When you start Blender for the first time or update to a new version, the interactive region of
the Splash Screen </interface/window_system/splash> is replaced with a few initial preferences to configure
how you interact with Blender.

**Note:**

   These options can always be changed later in the Preferences </editors/preferences/index>.

Import Preferences From Previous Version
========================================

Selecting this option will copy preferences from an older version of Blender. Doing so will copy preferences and
startup files from the previous version of Blender and load them. This will include installed add-ons and extensions.

The preferences need to be imported from previous versions because the configuration files of each Blender version are
stored in separate folders. Refer to the /advanced/blender_directory_layout page for the location of these
folders.

If you would like to start fresh with the new version, continue to `Create New Preferences`_.

**Note:**

   Some previous Blender add-ons and extensions may not be compatible with a new version of Blender, and choosing this
   option may lead to errors on startup.  If this occurs, the recommended first step is to try `Loading Factory
   Settings`_.

Create New Preferences
======================

Language
   The language used in the user interface. The list is broken up into categories determining how complete the
   translations are. More language preferences can be set in the
   Translation Preferences <prefs-interface-translation>.

Theme
   Choose between a light or dark theme for Blender. Themes can be customized more in the
   Preferences </editors/preferences/themes>. Additional themes can be installed by visiting the
   `Blender Extensions Platform <https://extensions.blender.org/>`__. This is optional, and will require internet
   access.

Keymap
   Presets for the default keymap </editors/preferences/keymap> for Blender. Note that this manual assumes that
   you use the default "Blender" keymap.

   :Blender:
      This is the default keymap. Read more about this keymap here </interface/keymap/blender_default>.
   :Blender 2.7x:
      This keymap is intended to match an older series of Blender versions and is designed for people upgrading who do
      not want to learn the updated keymap.
   :Industry Compatible:
      This keymap is intended to match common commercial creation software and is intended for people who use many
      different such applications. Read more about this keymap here </interface/keymap/industry_compatible>.

Mouse Select
   Controls which mouse button, either right or left, is used to select items in Blender. The default is for selection
   to use the left button.

Spacebar Action
   Controls the action of Spacebar.
   These and other shortcuts can be modified in the keymap preferences </editors/preferences/keymap>.

   :Play:
      Starts/stops animation playback </animation/index>.
      This option is good for animation or video editing work.
   :Tools:
      Opens the Toolbar underneath the cursor to quickly change the active tool. This option is good if doing a lot of
      modeling or rigging.
   :Search:
      Opens up the Menu Search <bpy.ops.wm.search_menu>. This option is good for someone who is new to Blender
      and is unfamiliar with its menus and shortcuts.

Save New Preferences
   Saves the preferences set above and opens the regular splash.

Saving Defaults
===============

The preferences are automatically saved when changed. This behavior can be changed by following the instructions under
Auto-Save Preferences <autosave-preferences>

Changing the default startup file can be done via File --> Defaults --> Save Startup File. See
Startup File <startup-file>.

There are two areas where Blender's defaults are stored:

Preferences
   The Preferences <prefs-menu> file stores keymap, add-ons theme and other options.
Startup File
   The Startup File <startup-file> stores the scene and UI setup which are displayed at startup and when
   creating a new file (File --> New).

Loading Factory Settings
========================

You can revert your customizations to Blender's defaults:

Preferences
   The Preferences <prefs-menu> Load Factory Settings.
Startup File & Preferences
   File --> Defaults --> Load Factory Settings.

**Note:**

   After loading the factory settings, the preferences won't be auto-saved.

   See prefs-menu for details.
