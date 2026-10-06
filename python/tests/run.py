"""Run every per-domain tool test against a real Blender.

Invoked by `python bootstrap.py t`, or directly:
    blender --background --factory-startup --python python/tests/run.py -- <dir-with-.pyd>

Each `test_<domain>.py` beside this file must expose `run(h)` taking a Harness. The
runner discovers them, gives each a FRESH factory scene (so tests do not leak state
into each other), and prints one tally at the end. Exit code is non-zero on any
failure, so bootstrap catches it.
"""

import importlib.util
import sys
from pathlib import Path

import bpy

HERE = Path(__file__).parent


def load_module(path: Path):
    spec = importlib.util.spec_from_file_location(path.stem, path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def reset_scene():
    """Back to a clean factory file between domains, so one domain's leftovers cannot
    make another's assertions pass or fail by accident."""
    bpy.ops.wm.read_factory_settings(use_empty=False)


def main() -> int:
    argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    if not argv:
        print("run.py: no extension directory given")
        return 1
    ext_dir = argv[0]

    sys.path.insert(0, str(HERE))
    from harness import Harness

    h = Harness(ext_dir)
    print(f"TESTS_BLENDER: {bpy.app.version_string}")

    test_files = sorted(HERE.glob("test_*.py"))
    if not test_files:
        print("run.py: no test_*.py files found")
        return 1

    for tf in test_files:
        reset_scene()
        mod = load_module(tf)
        if not hasattr(mod, "run"):
            print(f"  (skip {tf.name}: no run(h))")
            continue
        h.guard(tf.stem, lambda mod=mod: mod.run(h))

    print(f"TESTS_TOTAL ok={h.ok} fail={h.fail} across {len(test_files)} domain file(s)")
    return 1 if h.fail else 0


if __name__ == "__main__":
    sys.exit(main())
