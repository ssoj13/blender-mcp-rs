---
area: manual-animation
source: blender-manual
version: 5.2
---

.. _bpy.types.Constraint.mute:

******
Header
******

Every constraint has a header at the top:

:bl-icon:`rightarrow` / :bl-icon:`downarrow_hlt` Expand/Collapse
   Show or hide the settings of the constraint. Collapsed constraints remain active.

Icon
   Icon representing the constraint's type. If this icon is red, one or more settings are not
   correctly filled in and the constraint has no effect.

.. _bpy.types.Constraint.name:

Name
   Initially this is simply the constraint type, but it can be customized to something
   more specific.

.. _bpy.types.Constraint.enabled:

:bl-icon:`hide_off` Enabled
   Enable or disable the constraint. Disabling a constraint turns off its effects while keeping
   its settings around for the future.

   Another way of disabling a constraint is to set its Influence
   to zero. Unlike the *Enabled* setting, this can be animated.

:bl-icon:`downarrow_hlt` Extras
   .. _bpy.ops.constraint.apply:

   Apply Ctrl-A
      Applies the constraint's result to the object's/bone's own transformation,
      then deletes the constraint.

**Warning:**

         Applying a constraint that is not first in the stack will ignore the constraints before it,
         which may produce undesired results.

**Seealso:**

         Apply Visual Transform <bpy.ops.object.visual_transform_apply> to apply the combined
         result of all constraints without deleting them.

   .. _bpy.ops.constraint.copy:

   Duplicate Shift-D
      Creates a copy of the constraint just below current one.

   .. _bpy.ops.constraint.copy_to_selected:

   Copy to Selected
      Copies the constraint from the Active object to all selected objects.

   .. _bpy.ops.constraint.move_to_index:

   Move to First/Last
      Moves the constraint to the first or last position in the stack.

.. _bpy.ops.constraint.delete:

:bl-icon:`x` Delete X, Delete
   Delete the constraint from the stack.
   Its settings will be lost and it will no longer affect the object/bone.

:bl-icon:`grip` Move
   Drag to move the constraint up or down in the stack </animation/constraints/interface/stack>.
   Since the stack is evaluated from top to bottom,
   moving a constraint can significantly affect the final outcome.
