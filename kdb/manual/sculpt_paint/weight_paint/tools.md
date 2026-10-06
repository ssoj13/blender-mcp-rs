---
area: manual-sculpt_paint
source: blender-manual
version: 5.2
---

******************
Weight Paint Tools
******************

Brush
   Tool to use for any of the weight paint brushes <brushes>.

Gradient
   Applies a linear/radial weight gradient;
   this is useful at times when painting gradual changes in weight becomes difficult.
   Blends the weights of selected vertices with unselected vertices.

   Weight
      The gradient starts at the current selected weight value, blending out to nothing.
   Strength
      Lower values can be used so the gradient mixes in with the existing weights (just like with the brush).
   Type
      The shape of the gradient.

      :Linear: Create gradient that forms a straight line.
      :Radial: Create gradient that forms a circle.

**Note:**
         These are also available via shortcuts as the menu operators <bpy.ops.paint.weight_gradient>.

Sample
   Weights
      Sets the brush *Weight* as the weight selected under the cursor.
      The sampled weight is displayed in the tool settings.
   Vertex Group
      Displays a list of possible vertex groups to select that are under the cursor.

Annotate <tool-annotate-freehand>
   Draw free-hand annotation.

   Annotate Line
      Draw straight line annotation.
   Annotate Polygon
      Draw a polygon annotation.
   Annotate Eraser
      Erase previous drawn annotations.
