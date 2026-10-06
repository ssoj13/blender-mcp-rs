---
area: manual-advanced
source: blender-manual
version: 5.2
---

.. _extensions-create_repo-index:

*********************************
Creating an Extensions Repository
*********************************

Third party sites that wish to support extensions in Blender can do so in several ways, in order of complexity:

- Static <static_repository>:
  Host a static JSON file listing all the packages of your repository.
- Dynamic <dynamic_repository>:
  Serve the JSON file on-demand based on the Blender version and platform.
- `Platform <https://projects.blender.org/infrastructure/extensions-website>`__:
  Fork the entire Extensions Website to create your own.

Tags and Translations
=====================

If you are planning to use a different set of tags <extensions-tags> than the ones used by
Blender Extensions Platform, remember to submit a pull request to
`tags.py <https://projects.blender.org/blender/blender/src/scripts/modules/_bpy_internal/extensions/tags.py>`__.

This way they can be shown translated within Blender.

Sub-Pages
=========
