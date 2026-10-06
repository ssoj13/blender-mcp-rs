---
area: manual-editors
source: blender-manual
version: 5.2
---

.. _bpy.types.SpaceTextEditor:
.. _bpy.types.Text:
.. _bpy.ops.text:

***********
Text Editor
***********

This editor can be used to write Python scripts, scripting based shaders such as OSL and GLSL,
or just plain text notes. To open it, you can switch to the *Scripting*
workspace </interface/window_system/workspaces> or press
Shift-F11 to replace the current editor.

Header
======

The newly opened Text editor is empty, with a very simple header.
More options become available when a text file is created or opened.

.. _fig-text-header-plain:

.. _fig-text-header-full:

Editor Type
   The standard editor selection button.
Menus
   Editor's menus.

.. _bpy.ops.text.resolve_conflict:

Resolve Conflict
   Resolves modified file conflicts when an external text file is updated from another program.

   Reload from Disk
      Opens the file from drive again, overriding any local changes.
   Make Text Internal
      Converts the external text data-block into an internal one.
   Ignore
      Hides the warning message until the external text file is modified externally again.

Text
   A data-block menu <ui-data-block> to select a text or to create a new one.
   After that the header will change.

.. _editors-text-run-script:

Run Script (play icon)
   Executes the text as a Python script Alt-P. See `Template Menu`_.
Show
   Toggles for line numbers, word wrapping, and syntax highlighting.
Script Node Update (refresh icon)
   When an `OSL-file <https://github.com/AcademySoftwareFoundation/OpenShadingLanguage>`__
   is opened, this updates the Shader Script <bpy.types.ShaderNodeScript> node
   with new options and sockets from the script.

View Menu
---------

Sidebar Ctrl-T
   Show or hide the Sidebar <ui-region-sidebar>.

-----

Line Numbers
   Displays the text file's line numbers on the left of the `Main View`_.
Word Wrap
   Wraps words that don't fit into the horizontal space by pushing them to a new "pseudo line".
Syntax Highlight
   Colors special words, in the `Main View`_, that are used in the Python programming language.
Highlight Line
   Emphasizes the active line by altering the color of the background.

-----

Zoom In/Out
   Increase/decrease the font size of text in the main view.

-----

Navigation
   Top Ctrl-Home
      Moves the view and cursor to the start of the text file.
   Bottom Ctrl-End
      Moves the view and cursor to the end of the text file.
   Line Begin Home
      Moves the cursor to the start of the current line.
   Line End End
      Moves the cursor to the end of the current line.
   Previous Line Up
      Moves the cursor to the same position in the line above the current line.
   Next Line Down
      Moves the cursor to the same position in the line below the current line.
   Previous Word Ctrl-Left
      Moves the cursor to the beginning of the previous word.
      If the cursor is in the middle of a word, the cursor is moved to the beginning of the current word.
   Next Word Ctrl-Right
      Moves the cursor to the end of the next word.
      If the cursor is in the middle of a word, the cursor is moved to the end of the current word.

Text Menu
---------

.. _bpy.ops.text.new:

New Alt-N
   Creates a new text Data Block.

.. _bpy.ops.text.open:

Open Alt-O.
   Loads an external text file that is selected via the File Browser </editors/file_browser>.

.. _bpy.ops.text.reload:

Reload Alt-R
   Reopens (reloads) the current buffer (all non-saved modifications are lost).

.. _bpy.ops.text.jump_to_file_at_point:

Edit Externally
   Edit text file in external text editor.
   The external editor can be configured in the User Preferences <prefs-file_paths-text_editor>.

.. _bpy.ops.text.save:

Save Alt-S
   Saves an already open file.

.. _bpy.ops.text.save_as:

Save As Shift-Ctrl-Alt-S.
   Saves text as a new text file.
   A File Browser </editors/file_browser> is opened to select the directory
   to save the file along with giving the file a name and extension.

.. _bpy.types.Text.use_module:

Register
   Runs the text data-block as a Python script on loading the blend-file.
   Read more about the registration of Python modules in
   `API documentation <https://docs.blender.org/api/current/info_overview.html#registration>`__.

.. _bpy.types.SpaceTextEditor.use_live_edit:

Live Edit
   Runs the Python script each time you make a change.

.. _bpy.ops.text.run_script:

Run Script Alt-P
   Executes the text as a Python script. See `Running Scripts`_ for more information.

Edit Menu
---------

Undo/Redo
   See /interface/undo_redo.
Cut Ctrl-X
   Cuts out the marked text into the clipboard.
Copy Ctrl-C
   Copies the marked text into the clipboard.
Paste Ctrl-V
   Pastes the text from the clipboard at the cursor location in the Text editor.
Duplicate Line Ctrl-D
   Duplicates the current line.
Move Line(s) Up Shift-Ctrl-Up
   Swaps the current/selected line(s) with the above.
Move Line(s) Down Shift-Ctrl-Down
   Swaps the current/selected line(s) with the below.
Find & Replace Ctrl-F
   Shows the *Find & Replace* panel in the Sidebar.
Find & Set Selection Ctrl-G
   Finds the next instance of the selected text.
Jump To Ctrl-J
   Shows a pop-up, which lets you select a line number where to move the cursor to.
Text Auto Complete Tab
   Shows a selectable list of words already used in the text.
Text to 3D Object
   Converts the text file to a Text Object
   either as *One Object* or *One Object Per Line*.

Select Menu
-----------

All Ctrl-A
   Selects the entire text file.
Line Shift-Ctrl-A
   Selects the entire current line.
Word double-click LMB
   Selects the entire current word.
Top Shift-Ctrl-Home
   Selects everything above the cursor.
Bottom Shift-Ctrl-End
   Selects everything below the cursor.
Line Begin Shift-Home
   Selects everything between the beginning of the current line and the cursor.
Line End Shift-End
   Selects everything between the cursor and the end of the current line.
Previous Line Shift-Up
   Selects everything between the cursor and the position of the cursor one line above.
Next Line Shift-Down
   Selects everything between the cursor and the position of the cursor one line below.
Previous Word Shift-Ctrl-Left
   Selects everything between the cursor and the beginning of the previous word.
   If the cursor is in the middle of a word, select everything to the beginning of the current word.
Next Word Shift-Ctrl-Right
   Selects everything between the cursor and the end of the next word.
   If the cursor is in the middle of a word, select everything to the end of the current word.

Format Menu
-----------

Indent Tab
   Inserts a tab character at the cursor.
Unindent Shift-Tab.
   Unindents the selection.
Toggle Comments Ctrl-Slash.
   Toggles whether the selected line(s) are a Python comment.
   If no lines are selected, the current line is toggled.
Convert Whitespace
   Converts indentation characters *To Spaces* or *To Tabs*.

Template Menu
-------------

Contains a number of templates for both Python and Open Shading Language scripts.

Main View
=========

Typing on the keyboard produces text in the text buffer.

As usual, pressing, dragging and releasing LMB selects text.
Pressing RMB opens the context menu.

**Tip:**

   The Text editor is also handy when you want to share your blend-file with others:
   you can leave a note that explains how the file is structured.
   Be sure to keep the editor visible when saving so they'll see it!

Sidebar
=======

Find & Replace
--------------

.. _bpy.types.SpaceTextEditor.find_text:
.. _bpy.ops.text.find:

Find Text Ctrl-F
   Searches for instances of a text that occur after the cursor.
   Using the eyedropper icon will search for the currently selected text
   and sets the selection to the match.
   *Find Next* searches for the next instance of the text.

.. _bpy.types.SpaceTextEditor.replace_text:
.. _bpy.ops.text.replace:

Replace Text Ctrl-H
   Searches for the text specified in *Find Text* and replaces it with the new text.
   Using the eyedropper icon will set the currently selected text as the replace text.
   *Replace* searches for the next match and replaces it.
   *Replace All* searches for the match and replaces all occurrences of the match with the new text.

.. _bpy.types.SpaceTextEditor.use_match_case:
.. _bpy.types.SpaceTextEditor.use_find_wrap:
.. _bpy.types.SpaceTextEditor.use_find_all:

Case
   Search is sensitive to uppercase and lowercase letters.
Wrap
   Search again from the start of the file when reaching the end.
All
   Search in all text data-blocks instead of only the active one.

Properties
----------

.. _bpy.types.SpaceTextEditor.show_margin:
.. _bpy.types.SpaceTextEditor.margin_column:

Margin
   Shows a vertical margin line to help keep text lines at a reasonable length.
   The position of this margin line is specified by *Margin Column*.

.. _bpy.types.SpaceTextEditor.font_size:

Font Size Ctrl-WheelUp
   The size of the font used to display text.

.. _bpy.types.SpaceTextEditor.tab_width:

Tab Width
   The number of character spaces to display tab characters with.

.. _bpy.types.Text.indentation:

Indentation
   Whether to use *Tabs* or *Spaces* for indentation.

Footer
======

The Text editor footer displays whether the text is saved internally or externally and
if there are unsaved changes to an external file.
For external files, this region also displays the file path to the text file.

Usage
=====

Running Scripts
---------------

The most notable keystroke is Alt-P which executes the current text as a Python script.
You can access not just the standard Python modules, but also a whole bunch of Blender-specific ones;
see /advanced/scripting/index.
