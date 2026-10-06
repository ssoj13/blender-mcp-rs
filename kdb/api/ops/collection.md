---
area: api-ops
group: collection
version: 5.2.2
---

# Blender 5.2.2 operators: bpy.ops.collection

- `bpy.ops.collection.create(name: string)` — Create an object collection from selected objects
- `bpy.ops.collection.export_all()` — Invoke all configured exporters on this collection
- `bpy.ops.collection.exporter_add(name: string)` — Add exporter to the exporter list
- `bpy.ops.collection.exporter_export(index: int)` — Invoke the export operation
- `bpy.ops.collection.exporter_move(direction: enum{UP, DOWN})` — Move exporter up or down in the exporter list
- `bpy.ops.collection.exporter_remove(index: int)` — Remove exporter from the exporter list
- `bpy.ops.collection.importer_add(name: string)` — Add Importer
- `bpy.ops.collection.importer_remove()` — Remove Importer
- `bpy.ops.collection.objects_add_active(collection: enum{})` — Add selected objects to one of the collections the active-object is part of. Optionally add to "All Collections" to ensure selected objects are included in the same collections as the active object
- `bpy.ops.collection.objects_remove(collection: enum{})` — Remove selected objects from a collection
- `bpy.ops.collection.objects_remove_active(collection: enum{})` — Remove the object from an object collection that contains the active object
- `bpy.ops.collection.objects_remove_all()` — Remove selected objects from all collections