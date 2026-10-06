#!/usr/bin/env python3
"""
Run Blender MCP Server standalone tests.

Tests:
1. test_mcp_server.py - MCP protocol tests (no Blender needed)

For Blender handler tests, use: MCP menu > Test Handlers in Blender.

Usage:
    python run_tests.py
"""
import subprocess
import sys
from pathlib import Path

TESTS_DIR = Path(__file__).parent


def run_test(name: str, path: Path) -> bool:
    """Run a test file and return success status."""
    print()
    print("=" * 60)
    print(f"Running: {name}")
    print("=" * 60)

    result = subprocess.run(
        [sys.executable, str(path)],
        cwd=TESTS_DIR.parent
    )

    return result.returncode == 0


def main():
    tests = [
        ("MCP Server Standalone", TESTS_DIR / "test_mcp_server.py"),
    ]

    passed = 0
    failed = 0
    results = []

    for name, path in tests:
        if not path.exists():
            print(f"SKIP: {name} - file not found")
            continue

        success = run_test(name, path)
        results.append((name, success))
        if success:
            passed += 1
        else:
            failed += 1

    # Summary
    print()
    print("=" * 60)
    print("TEST SUMMARY")
    print("=" * 60)

    for name, success in results:
        status = "PASS" if success else "FAIL"
        print(f"  [{status}] {name}")

    print()
    print(f"Total: {passed} passed, {failed} failed")
    print()
    print("For Blender handler tests: MCP menu > Test Handlers")
    print("=" * 60)

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
