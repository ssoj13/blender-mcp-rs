---
area: manual-modeling
source: blender-manual
version: 5.2
---

************************
Assigning a Vertex Group
************************

Creating Vertex Groups
======================

Vertex groups are maintained within the *Object Data* tab (1) in the Properties.
As long as no vertex groups are defined (the default for new mesh objects),
the panel is empty (2).

You create a vertex group by LMB on the :bl-icon:`add` button on the right panel border (3).
Initially the group is named "Group" (or "Group.nnn" when the name already exists) and
gets displayed in the panel (2) (see next image).

Vertex Groups Panel Controls
----------------------------

Once a new vertex group has been added, the new group appears
in the Vertex Groups panel. There you find three clickable elements:

Group Name
   The group name can be changed by double-clicking LMB on the name itself.
   Then you can edit the name as you like.

Filter (arrow icon)
   When the little arrow icon in the left lower corner is clicked, a new row opens up
   where you can enter a search term. This becomes handy when the number of
   vertex groups gets big.

Drag Handle
   If you have a large number of vertex groups and you want to see more
   than a few groups, you can LMB on the small drag handle to make
   the vertex groups list larger or smaller.

Active Group
   When a vertex group is created,
   then it is also automatically marked as the *Active Group*.
   This is indicated by setting the background of the panel entry
   to a light gray color. If you have two or more groups in the list,
   then you can change the active group by LMB on
   the corresponding entry in the Vertex Groups panel.

Deleting Vertex Groups
======================

You delete a vertex group by first making it the active group
(select it in the panel) and then LMB
the :bl-icon:`remove` button at the right panel border.

Deleting a vertex group only deletes the vertex assignments to the group.
The vertices themselves are not deleted.

Locking Vertex Groups
=====================

Right after creation of a vertex group,
an open padlock icon shows up on the right side of the list entry.
This icon indicates that the vertex group can be edited.
You can add vertex assignments to the group or remove assignments from the group.
And you can change it with the weight paint brushes, etc.

When you click on the icon,
it changes to a closed padlock icon and all vertex group modifications get disabled.
You can only rename or delete the group, and unlock it again.
No other operations are allowed on locked vertex groups,
thus all corresponding buttons become disabled for locked vertex groups.

Working with Content of Vertex Groups
=====================================

Assigning Vertices to a Group
-----------------------------

You add vertices to a group as follows:

- Select the group from the group list, thus making it the active group (1).
- From the 3D Viewport select Shift-LMB all vertices that you want to add to the group.
- Set the weight value that shall be assigned to all selected vertices (2).
- LMB the *Assign* button to assign the selected vertices to the active group using the given weight (3).

Note that weight assignment is not available for locked vertex groups.
The *Assign* button is grayed out in that case.

**Note:**

   The *Assign* button only adds the currently
   selected vertices to the active group. Vertices already
   assigned to the group are not removed from the group.

   Also keep in mind that a vertex can be assigned to multiple groups.

Checking Assignments
--------------------

To be sure the selected vertices are in the desired vertex group,
you can try press the deselect button.
If the vertices remain selected then they are not yet in the current vertex group.

At this point you may assign them, but take care since all selected vertices
will have their weight set to the value in the *Weight* field.

Removing Assignments from a Group
---------------------------------

You remove vertices from a group as follows:

- Select the group from the group list (make it the active group).
- Select all vertices that you want to remove from the group.
- LMB click the *Remove* button.

Note that removing weight assignments is not available for locked vertex groups.
The *Remove* button is grayed out in that case.

Using Groups for Selecting/Deselecting
--------------------------------------

You can quickly select all assigned vertices of a group:

- (Optionally) press Alt-A to deselect all vertices.
- Select the group from the group list (make it the active group).
- When you now LMB click the *Select* button,
  then the vertices assigned to the active group will be selected and highlighted in the 3D Viewport.
- When you LMB click the *Deselect* button instead,
  then the vertices assigned to the active group will be deselected in the 3D Viewport.

**Note:**

   If you already have vertices selected in the 3D Viewport,
   then selecting the vertices of a group will add the vertices
   but also keep the already selected vertices selected.
   Vice versa, deselecting the vertices of a vertex group
   will only deselect the vertices assigned to the group
   and keep all other vertices selected.

Finding Ungrouped Vertices
--------------------------

You can find ungrouped vertices as follows:

- Press Alt-A to deselect all vertices.
- In the header of the 3D Viewport, navigate to
  Select --> Select All by Trait --> Ungrouped Vertices.
