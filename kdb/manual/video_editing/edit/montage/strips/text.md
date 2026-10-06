---
area: manual-video_editing
source: blender-manual
version: 5.2
---

.. _bpy.types.TextStrip:

**********
Text Strip
**********

The Text strip allows you to directly display text in the Sequence editor.
The strip will display the text inserted in its text field on the final sequence.

**Tip:**

   All Text strips in a video sequence can be exported
   as a `SubRip <https://en.wikipedia.org/wiki/SubRip>`__ file.
   This is useful when using Text strips as subtitles.

.. _bpy.ops.sequencer.text_strip_style_preset_add:

Options
=======

Text strips support customizable typography, layout, and styling options
for creating subtitles, captions, titles, credits, and other text overlays.

The appearance of the text can be customized with outlines, shadows,
background boxes, alignment controls, and font styling options.

Style presets are available in the header for quickly applying common layouts
and visual styles.

Blender includes several example presets:

- Subtitle -- Traditional subtitle styling.
- Main Title -- Large centered title text.
- Title Corner -- Small title aligned to the lower-left corner.

Text
   The actual text displayed by the strip.

   The text can contain multiple lines and supports standard UTF-8 characters,
   including accented characters, symbols, and many non-Latin writing systems.

   Text can be edited directly in the text field or interactively in the
   Preview region <bpy.ops.sequencer.text_edit_mode_toggle>.

Wrap Width
   Wraps the text by the percentage of the frame width,
   setting this to zero disables word wrapping.

Style
-----

Font
   ui-data-block to choose which font-file is used to render the text.

   :bl-icon:`bold` (Bold)
      Use a bold font face with a strong/thick visual appearance.
   :bl-icon:`italic` (Italic)
      Use an italicized font face with a slanted visual appearance.
Size
   Size of the text.
Line Spacing
   Controls the vertical spacing between lines of text.

   By default, the spacing is specified as a factor of the text size.
   Press :bl-icon:`fixed_size` to switch to an absolute spacing using a pixel value.
Color
   The text color.

Outline
^^^^^^^

Creates a line enclosing the shape of the text.

Color
   The color and opacity of the outline.
Width
   The thickness of the outline.

Shadow
^^^^^^

Creates a shadow under the text.

Color
   The color and opacity of the shadow.
Angle
   Defines the position of the shadow as an angle, 0° being to the right and 90° being below.
Offset
   Amount to shift the shadow compared to the normal text.
Blur
   Amount to blur the shadow.

Box
^^^

Creates a background for the text to improve the readability and clarity of text in some situations.

Color
   The color and opacity of the box.
Margin
   The distance the box boundaries extends from the boundaries of the font glyphs.
   The distance is measured as a factor of the image's width.
Roundness
   Rounds the corners of the box, defined as a factor of box height.

Layout
------

Location X, Y
   Positions the text on the X, Y axis.
Alignment X, Y
   Controls the horizontal positioning of text within the text strip.
   The text content can be aligned to the left, center, or right.
Anchor X, Y
   Horizontal (X) or vertical (Y) "origin" of the text relative to the location.

.. _bpy.ops.sequencer.text_edit_mode_toggle:

Text Editing in Preview
=======================

Text strips can be edited directly in the **Preview** window, providing a more intuitive workflow.

- **Enable Editing**: Press Tab to enter edit mode. A boundary box and cursor will appear.
- **Disable Editing**: Press Tab again to exit edit mode.

Editing
-------

Copy Text Ctrl-C
   Copy the selected text.
Paste Text Ctrl-V
   Uses the system clipboard, allowing text from external applications to be pasted.
Cut Text Ctrl-X
   Cut the selected text.
Delete Character Backspace
   Delete the character before the cursor.
Insert Line Break Return
   Insert a new line.
Select All Ctrl-A
   Select all the text.
Deselect All Esc
   Deselect the text.

Navigation
----------

- Left -- Move the cursor one character left.
- Right -- Move the cursor one character right.
- Up -- Move the cursor one line up**.
- Down -- Move the cursor one line down**.
- Home -- Move the cursor to the beginning of the current line.
- End -- Move the cursor to the end of the current line.
- Ctrl-Left -- Move the cursor one word left.
- Ctrl-Right -- Move the cursor one word right.
- PageUp -- Move the cursor to the start of the text.
- PageDown -- Move the cursor to the end of the text.

Text Selection
--------------

Holding Shift while pressing a navigation key enables text selection.

- Shift-Left -- Select the previous character.
- Shift-Right -- Select the next character.
- Shift-Up -- Select the previous line.
- Shift-Down -- Select the next line.
- Shift-Home -- Select from the cursor to the beginning of the line.
- Shift-End -- Select from the cursor to the end of the line.
- Shift-Ctrl-Left -- Select the previous word.
- Shift-Ctrl-Right -- Select the next word.
- Shift-PageUp -- Select from the cursor to the start of the text.
- Shift-PageDown -- Select from the cursor to the end of the text.
