"""Dump the live Blender API as Markdown — the ask-Blender-don't-guess corpus.

Run: blender --background --factory-startup --python dump_api.py -- <outdir>

Produces, under <outdir>:
  types/<group>.md    RNA classes: properties, their types, enum items, read-only flags
  ops/<module>.md     operators: full signature + enum values per argument
  nodes/<kind>.md     every node type with its real input/output socket names
"""
import os
import sys
from collections import defaultdict

import bpy

OUT = sys.argv[sys.argv.index("--") + 1] if "--" in sys.argv else "kdb-api"
VER = bpy.app.version_string.split()[0]


def prop_line(p):
    bits = [f"`{p.identifier}`", p.type.lower()]
    if p.type == "ENUM":
        items = [i.identifier for i in p.enum_items]
        # A few enums are catalogues (icons, event types) running to thousands of
        # entries; keep them searchable without letting one property own a megabyte.
        shown = items[:40]
        more = f", ... ({len(items)} total)" if len(items) > 40 else ""
        bits.append("{" + ", ".join(shown) + more + "}" if items else "{}")
    elif p.type in {"FLOAT", "INT"} and getattr(p, "array_length", 0) > 1:
        bits.append(f"[{p.array_length}]")
    elif p.type == "POINTER" and getattr(p, "fixed_type", None):
        bits.append(f"-> {p.fixed_type.identifier}")
    elif p.type == "COLLECTION" and getattr(p, "srna", None):
        bits.append(f"[] of {p.srna.identifier}")
    if p.is_readonly:
        bits.append("read-only")
    desc = (p.description or "").strip()
    return "- " + " · ".join(bits) + (f" — {desc}" if desc else "")


def group_of(name):
    """Coarse bucket so the corpus is a few dozen files, not two thousand."""
    for key in ("Node", "Modifier", "Constraint", "Sequence", "Strip", "GreasePencil",
                "Brush", "Object", "Mesh", "Curve", "Material", "Image", "Scene",
                "Render", "Light", "Camera", "Particle", "Fluid", "Cloth", "Rigid",
                "Bone", "Armature", "Action", "FCurve", "NLA", "Key", "Texture",
                "World", "View", "Space", "UI", "Preferences", "Operator"):
        if key.lower() in name.lower():
            return key.lower()
    return "misc"


def dump_types():
    groups = defaultdict(list)
    for name in dir(bpy.types):
        cls = getattr(bpy.types, name)
        srna = getattr(cls, "bl_rna", None)
        if srna is None:
            continue
        # Only what this class adds: every RNA class repeats its parents' properties,
        # and a Node subclass inheriting ~100 of them would bury the handful that
        # actually distinguish it.
        inherited = set()
        for base in cls.__mro__[1:]:
            base_rna = getattr(base, "bl_rna", None)
            if base_rna is not None:
                inherited |= {p.identifier for p in base_rna.properties}
        own = [p for p in srna.properties
               if p.identifier != "rna_type" and p.identifier not in inherited]
        if not own:
            continue
        lines = [f"## {name}"]
        if srna.description:
            lines.append("")
            lines.append(srna.description.strip())
        lines.append("")
        lines += [prop_line(p) for p in own]
        lines.append("")
        groups[group_of(name)].append("\n".join(lines))
    os.makedirs(f"{OUT}/types", exist_ok=True)
    for g, chunks in groups.items():
        with open(f"{OUT}/types/{g}.md", "w", encoding="utf-8", newline="\n") as fh:
            fh.write(f"---\narea: api-types\ngroup: {g}\nversion: {VER}\n---\n\n")
            fh.write(f"# Blender {VER} RNA types: {g}\n\n")
            fh.write("\n".join(chunks))


def dump_ops():
    os.makedirs(f"{OUT}/ops", exist_ok=True)
    for mod_name in dir(bpy.ops):
        mod = getattr(bpy.ops, mod_name)
        lines = []
        for op_name in dir(mod):
            op = getattr(mod, op_name)
            try:
                srna = op.get_rna_type()
            except Exception:  # noqa: BLE001
                continue
            args = []
            for p in srna.properties:
                if p.identifier == "rna_type":
                    continue
                a = f"{p.identifier}: {p.type.lower()}"
                if p.type == "ENUM":
                    a += "{" + ", ".join(i.identifier for i in p.enum_items) + "}"
                args.append(a)
            desc = (srna.description or "").strip()
            lines.append(f"- `bpy.ops.{mod_name}.{op_name}({', '.join(args)})`"
                         + (f" — {desc}" if desc else ""))
        if lines:
            with open(f"{OUT}/ops/{mod_name}.md", "w", encoding="utf-8", newline="\n") as fh:
                fh.write(f"---\narea: api-ops\ngroup: {mod_name}\nversion: {VER}\n---\n\n")
                fh.write(f"# Blender {VER} operators: bpy.ops.{mod_name}\n\n")
                fh.write("\n".join(lines))


def dump_nodes():
    """Real socket names per node type — read by building each node once."""
    os.makedirs(f"{OUT}/nodes", exist_ok=True)
    kinds = {
        "shader": ("ShaderNodeTree", "shader"),
        "geometry": ("GeometryNodeTree", "geometry"),
        "compositor": ("CompositorNodeTree", "compositor"),
        "texture": ("TextureNodeTree", "texture"),
    }
    for kind, (tree_type, _) in kinds.items():
        tree = bpy.data.node_groups.new(f"__kdb_{kind}", tree_type)
        lines = []
        for name in sorted(dir(bpy.types)):
            cls = getattr(bpy.types, name)
            if not isinstance(cls, type) or not issubclass(cls, bpy.types.Node):
                continue
            if cls is bpy.types.Node:
                continue
            try:
                node = tree.nodes.new(name)
            except Exception:  # noqa: BLE001
                continue  # registered but not creatable in this tree kind
            ins = ", ".join(f"{s.name}:{s.type}" for s in node.inputs)
            outs = ", ".join(f"{s.name}:{s.type}" for s in node.outputs)
            doc = (cls.bl_rna.description or "").strip()
            lines.append(f"- `{name}` ({node.bl_label})"
                         + (f" — {doc}" if doc else "")
                         + f"\n  - in: {ins or '(none)'}\n  - out: {outs or '(none)'}")
            tree.nodes.remove(node)
        bpy.data.node_groups.remove(tree)
        if lines:
            with open(f"{OUT}/nodes/{kind}.md", "w", encoding="utf-8", newline="\n") as fh:
                fh.write(f"---\narea: api-nodes\ngroup: {kind}\nversion: {VER}\n---\n\n")
                fh.write(f"# Blender {VER} {kind} nodes: real socket names\n\n")
                fh.write("\n".join(lines))


dump_types()
dump_ops()
dump_nodes()
print("KDB_API_DUMP_DONE", OUT)
