---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

**************
Manage Brushes
**************

Brush assets are stored in asset libraries </files/asset_libraries/introduction> to make them accessible from
any Blender session. There are two ways of managing brush assets:

- Using asset operators <brush-management-utility-operators>: Create and update brush assets
  using utility operators from any Blender file. Storage is managed by Blender. Convenient for
  simple "on the fly" management of personal brush
  asset libraries.
- Using manual storage <brush-management-manual>: Create and update brush assets by opening
  blend files within asset libraries, and managing brush asset data-blocks in there. Useful for
  careful curation of asset libraries, especially to prepare them for sharing with others.

.. _brush-management-utility-operators:

Asset Operators
===============

Brushes can be managed through a few operators that let Blender handle the act of saving and
updating the brushes in asset libraries for you. Assets managed this way will be saved in special
asset system files <asset-system-files> using a `.asset.blend` file extension.

**Note:**
   Note that only brush assets created via Duplicate Asset
   can be edited further using these asset operators. For others, these operations will be grayed
   out, and manual management <brush-management-manual> is necessary.

   Brushes from the *Essentials* asset library cannot be edited.

.. _bpy.ops.brush.asset_save_as:

:bl-icon:`duplicate` Duplicate Asset...
   Creates a copy of the currently active brush as asset, and activates it. A popup is spawned to input some settings
   to use:

   Name
      A custom name to use for the new brush.
   Library
      Choose an Asset Library to store the new brush asset in. The available asset libraries are configured
      in the Preferences <bpy.types.UserAssetLibrary>.
   Catalog
      Choose an Asset Catalog </files/asset_libraries/catalogs> to assign the brush asset to. Entering a
      non-existent name/path will create a new catalog accordingly.

Delete Asset
   Permanently remove this brush asset from the Asset Library it is stored in. This cannot be undone, so a popup will
   ask for confirmation.
Edit Metadata...
   Spawns a popup to change some of the available asset metadata <editing-asset-metadata> fields:

   Catalog
      Choose an Asset Catalog </files/asset_libraries/catalogs> to assign the brush asset to. Entering a
      non-existent name/path will create a new catalog accordingly.
   Author
      See Asset Author <bpy.types.AssetMetaData.description>.
   Description
      See Asset Description <bpy.types.AssetMetaData.description>.
Edit Preview Image...
   Opens a window with the File Browser to select an image for the asset preview.
Save Changes to Asset
   Saves any changes made to the active brush to the asset library.
Revert to Asset
   Discards any unsaved changes made to the brush asset.

.. _brush-management-manual:

Manual Storage
==============

**Seealso:**
   asset-life-cycle
      Complete description of the manual asset create, edit, share and use workflow.

It is also possible to manually manage brushes in blend-files like any other asset data-block. By
marking brushes as assets and saving the file in an asset library, they become available from any
Blender session. This gives full control over how assets are stored, and is particularly useful for
curating asset libraries that can be shared with others.

Brushes can be imported as normal data-blocks from other files (including from `.asset.blend` files
from an asset library) through appending <bpy.ops.wm.append>. In the
Blender File <outliner-blender-file-mode> mode of the Outliner, the brush will be listed
under *Brushes*. Right-click the brush and select *Mark as Asset*. By saving the file inside of
an asset library directory, the asset becomes available from all Blender sessions. If necessary,
configure an asset library directory in the Preferences.
