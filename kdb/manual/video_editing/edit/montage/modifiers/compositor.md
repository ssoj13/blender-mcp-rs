---
area: manual-video_editing
source: blender-manual
version: 5.2
---

.. _bpy.types.SequencerCompositorModifierData:

*******************
Compositor Modifier
*******************

Compositor modifier is powerful tool to create arbitrary effects. To use it, you need to open compositor editor
and change node subtree to sequencer. Then you can add new node tree and use it for any strip. These node trees
can be marked as asset </files/asset_libraries/index> and reused across multiple .blend files.

To learn more about how to use compositor, see /compositing/index

Node Group
   The compositor node group used by the modifier.

   The selector can be hidden by toggling
   bpy.types.SequencerCompositorModifierData.show_group_selector.

Compatible node group inputs are automatically exposed as modifier properties,
allowing effects to be adjusted directly from the Sequencer without opening
the node editor.
