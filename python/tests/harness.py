"""Shared test harness for the per-domain tool tests.

Each test file (`test_<domain>.py`) exposes a `run(h)` function and gets a `Harness`.
The harness imports the freshly-built `blender_mcp` extension, drives tools exactly
as the server does (`blender_mcp.execute_tool`), and tallies pass/fail. The runner
(`run.py`) builds a fresh scene, hands the same harness to every domain, and reports.

A test that only checks a tool returned *something* is nearly worthless: a broken tool
often returns a plausible dict. So the helpers here push you to assert against ground
truth read straight from `bpy`, and to prove failures actually fail.
"""

import json
import traceback

import bpy


class Harness:
    def __init__(self, ext_dir: str):
        import sys

        if ext_dir not in sys.path:
            sys.path.insert(0, ext_dir)
        import blender_mcp

        self._m = blender_mcp
        self.ok = 0
        self.fail = 0
        self.domain = "?"

    # -- calling tools --------------------------------------------------------

    def call(self, tool: str, params: dict | None = None):
        """Call a tool and return its parsed result. Raises on tool error — use this
        when the call is expected to succeed."""
        return json.loads(self._m.execute_tool(tool, json.dumps(params or {})))

    # -- assertions -----------------------------------------------------------

    def check(self, label: str, cond: bool):
        """Assert a plain condition (usually read from bpy, not from the tool's own
        output). This is the real test: did Blender's state actually change."""
        if cond:
            self.ok += 1
            print(f"  OK   [{self.domain}] {label}")
        else:
            self.fail += 1
            print(f"  FAIL [{self.domain}] {label}")

    def ran(self, label: str, tool: str, params: dict | None = None):
        """Call a tool that should succeed; pass if it does not raise. Prefer `check`
        with a bpy read where you can — this is for tools whose effect is awkward to
        read back."""
        try:
            self.call(tool, params)
            self.ok += 1
            print(f"  OK   [{self.domain}] {label}")
            return True
        except Exception as e:  # noqa: BLE001 - a raise here is a test failure
            self.fail += 1
            print(f"  FAIL [{self.domain}] {label}: {type(e).__name__}: {str(e)[:120]}")
            return False

    def errors(self, label: str, tool: str, params: dict, expect_in: str = ""):
        """A call that MUST fail (a guard). Passing when it succeeds is a bug: a failed
        operation reported as success is the anti-pattern this project forbids."""
        try:
            self.call(tool, params)
            self.fail += 1
            print(f"  FAIL [{self.domain}] {label}: should have errored, did not")
        except Exception as e:  # noqa: BLE001 - the expected outcome
            msg = str(e)
            if expect_in and expect_in.lower() not in msg.lower():
                self.fail += 1
                print(f"  FAIL [{self.domain}] {label}: errored, but not about "
                      f"'{expect_in}' ({msg[:90]})")
            else:
                self.ok += 1
                print(f"  OK   [{self.domain}] {label}")

    def section(self, domain: str):
        self.domain = domain
        print(f"--- {domain} ---")

    def guard(self, label: str, fn):
        """Run a chunk of a test, catching an unexpected crash so one broken tool does
        not abort the rest of a domain's file."""
        try:
            fn()
        except Exception:  # noqa: BLE001
            self.fail += 1
            print(f"  FAIL [{self.domain}] {label}: crashed")
            traceback.print_exc()


def modifier_input(mod, identifier):
    """Read one Geometry Nodes modifier input, on whichever address this Blender has.

    Blender 5.2 moved these from the modifier's ID-properties (`mod["Socket_2"]`,
    which now raises "this type doesn't support IDProperties") to
    `mod.properties.inputs.Socket_2.value`. Mirrors `bpy::read_modifier_input` in Rust.
    """
    props = getattr(mod, "properties", None)
    if props is not None and hasattr(props.inputs, identifier):
        return getattr(props.inputs, identifier).value
    return mod[identifier]
