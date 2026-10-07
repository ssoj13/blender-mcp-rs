#!/usr/bin/env python3
"""
Smoke-test the MCP server (stdio) without Blender.

Sends JSON-RPC-shaped MCP messages (same wire format `rmcp` uses on stdio).
Requires: `cargo build` or `cargo build --release` (binary: blender-mcp-rs).
"""
import json
import os
import subprocess
import sys
import threading
from pathlib import Path
from typing import Optional

PROJECT_ROOT = Path(__file__).parent.parent


def find_binary() -> Optional[Path]:
    """Prefer release, fall back to debug; Windows .exe or Unix name."""
    if explicit := os.environ.get("BLENDER_MCP_BINARY"):
        path = Path(explicit)
        return path if path.is_file() else None
    for profile in ("release", "debug"):
        for name in ("blender-mcp-rs.exe", "blender-mcp-rs"):
            p = PROJECT_ROOT / "target" / profile / name
            if p.is_file():
                return p
    return None


BINARY = find_binary()

if BINARY is None:
    print("ERROR: blender-mcp-rs binary not found under target/release or target/debug")
    print("Run: cargo build   or   cargo build --release")
    sys.exit(1)


def mcp_request(proc, method: str, params: dict = None, req_id: int = 1) -> dict:
    """Send MCP request and get response."""
    request = {
        "jsonrpc": "2.0",
        "id": req_id,
        "method": method,
        "params": params or {}
    }
    line = json.dumps(request) + "\n"
    proc.stdin.write(line)
    proc.stdin.flush()

    response_line = proc.stdout.readline()
    if not response_line:
        return {"error": "No response"}
    return json.loads(response_line)


def send_notification(proc, method: str, params: dict) -> None:
    """Fire-and-forget MCP notification (no id)."""
    msg = {"jsonrpc": "2.0", "method": method, "params": params}
    proc.stdin.write(json.dumps(msg) + "\n")
    proc.stdin.flush()


def test_initialize(proc) -> bool:
    """Test initialize method."""
    print("TEST: initialize")
    resp = mcp_request(proc, "initialize", {
        "protocolVersion": "2024-11-05",
        "capabilities": {},
        "clientInfo": {"name": "test", "version": "1.0"}
    }, req_id=1)

    if "error" in resp:
        print(f"  FAIL: {resp['error']}")
        return False

    result = resp.get("result", {})
    server_info = result.get("serverInfo", {})
    print(f"  Server: {server_info.get('name')} v{server_info.get('version')}")
    print(f"  Protocol: {result.get('protocolVersion')}")
    # MCP clients normally send this after a successful initialize
    send_notification(proc, "notifications/initialized", {})
    return True


def test_tools_list(proc) -> bool:
    """Test tools/list method."""
    print("TEST: tools/list")
    resp = mcp_request(proc, "tools/list", {}, req_id=2)

    if "error" in resp:
        print(f"  FAIL: {resp['error']}")
        return False

    tools = resp.get("result", {}).get("tools", [])
    print(f"  Found {len(tools)} tools")

    # Check we have core tools
    expected = {
        "scene_get_info", "obj_list", "obj_get", "obj_create_primitive",
        "obj_update", "obj_delete", "obj_duplicate",
        "obj_get_selection", "obj_set_selection", "obj_parent",
        "col_list", "col_move_to",
        "mat_list", "mat_create", "mat_assign",
        "mod_list", "mod_add", "mod_apply",
        "anim_set_keyframe",
        "scene_render", "file_export", "file_import", "file_save",
        "util_set_mode", "util_undo", "util_redo", "util_execute_python"
    }
    actual = {t["name"] for t in tools}

    missing = expected - actual
    if missing:
        print(f"  FAIL: Missing tools: {missing}")
        return False

    extra = actual - expected
    if extra:
        print(f"  WARN: Extra tools: {extra}")

    # Verify inputSchema present
    for tool in tools[:3]:
        if "inputSchema" not in tool:
            print(f"  FAIL: Tool {tool['name']} missing inputSchema")
            return False

    print("  Core tools present with schemas")
    return True


def test_tools_call_no_blender(proc) -> bool:
    """Test tools/call - should timeout without Blender."""
    print("TEST: tools/call (expect timeout)")

    # This will timeout because Blender addon is not running
    # We just verify the request is properly formatted
    import threading
    import time

    result = {"done": False, "response": None}

    def call():
        try:
            resp = mcp_request(proc, "tools/call", {
                "name": "scene_get_info",
                "arguments": {}
            }, req_id=3)
            result["response"] = resp
        except:
            pass
        result["done"] = True

    thread = threading.Thread(target=call)
    thread.start()
    thread.join(timeout=2)  # Wait max 2 seconds

    if result["done"] and result["response"]:
        resp = result["response"]
        if "error" in resp:
            msg = resp["error"].get("message", "")
            if "Timeout" in msg or "addon" in msg.lower():
                print(f"  OK: Got expected timeout/error: {msg[:50]}...")
                return True
        print(f"  Unexpected response: {resp}")
        return False
    else:
        print("  OK: Request pending (Blender not connected)")
        return True


def test_control_tools(proc) -> bool:
    """Control tools are server-side: they must be listed, and blender_status must
    answer immediately (no Blender, no pipe) — unlike scene tools, which would time
    out here."""
    print("TEST: control tools (server-side)")
    resp = mcp_request(proc, "tools/list", {}, req_id=10)
    tools = {t["name"] for t in resp.get("result", {}).get("tools", [])}
    control = {"blender_status", "blender_launch", "blender_quit",
               "blender_kill", "blender_restart"}
    missing = control - tools
    if missing:
        print(f"  FAIL: control tools missing from tools/list: {missing}")
        return False

    resp = mcp_request(proc, "tools/call", {
        "name": "blender_status", "arguments": {}
    }, req_id=11)
    if "error" in resp:
        print(f"  FAIL: blender_status errored: {resp['error']}")
        return False
    content = resp.get("result", {}).get("content", [])
    text = content[0].get("text", "") if content else ""
    try:
        status = json.loads(text)
    except (json.JSONDecodeError, TypeError):
        print(f"  FAIL: blender_status returned non-JSON: {text!r}")
        return False
    # Not asserting a value for `connected`: a real Blender may or may not be up on
    # this machine. Assert the shape only.
    for key in ("process_running", "connected", "tag"):
        if key not in status:
            print(f"  FAIL: blender_status missing '{key}': {status}")
            return False
    print(f"  OK: 5 control tools listed; blender_status -> "
          f"connected={status['connected']}, pid={status.get('pid')}")
    return True


def test_unknown_method(proc) -> bool:
    """Test unknown method returns error."""
    print("TEST: unknown method")
    resp = mcp_request(proc, "foo/bar", {}, req_id=99)

    if "error" not in resp:
        print(f"  FAIL: Expected error, got: {resp}")
        return False

    print(f"  OK: Got error: {resp['error'].get('message', '')[:50]}")
    return True


def main():
    print("=" * 60)
    print("Blender MCP Server Tests (Standalone)")
    print("=" * 60)
    print(f"Binary: {BINARY}")
    print()

    # Start server process
    proc = subprocess.Popen(
        [str(BINARY)],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )
    # A blocked request must fail the smoke test and close its server, including
    # when this script is run directly rather than through the CI wrapper.
    watchdog = threading.Timer(60, proc.kill)
    watchdog.daemon = True
    watchdog.start()

    passed = 0
    failed = 0

    try:
        tests = [
            test_initialize,
            test_tools_list,
            test_control_tools,
            test_unknown_method,
            # test_tools_call_no_blender,  # Skip - blocks
        ]

        for test in tests:
            print()
            try:
                if test(proc):
                    passed += 1
                else:
                    failed += 1
            except Exception as e:
                print(f"  ERROR: {e}")
                failed += 1
    finally:
        watchdog.cancel()
        proc.terminate()
        proc.wait(timeout=2)

    print()
    print("=" * 60)
    print(f"Results: {passed} passed, {failed} failed")
    print("=" * 60)

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
