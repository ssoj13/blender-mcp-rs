"""
Blender MCP Server Addon
Exposes Blender to AI agents via Model Context Protocol (MCP).
"""
import json
import logging
from pathlib import Path
from typing import Optional

import bpy

bl_info = {
    "name": "MCP Server",
    "author": "blender-mcp-rs",
    "version": (0, 1, 0),
    "blender": (4, 0, 0),
    "location": "Topbar > MCP",
    "description": "MCP Server for AI agent control of Blender",
    "category": "System",
}

LOG = logging.getLogger(__name__)


# ============================================================================
# Addon Preferences
# ============================================================================

class MCPPreferences(bpy.types.AddonPreferences):
    bl_idname = __name__

    auto_start: bpy.props.BoolProperty(
        name="Auto-start on Blender launch",
        description="Automatically start MCP client when Blender starts",
        default=False,
    )

    def draw(self, context):
        layout = self.layout
        layout.prop(self, "auto_start")

        global _bridge
        if _bridge and _bridge.is_running():
            box = layout.box()
            box.label(text="Client running", icon='CHECKMARK')
        else:
            layout.label(text="Client not running", icon='X')

# Global state
_bridge: Optional["BlenderMcpBridge"] = None
_mcp_module = None


def _find_server_binary() -> str:
    addon_dir = Path(__file__).parent
    candidates = [
        addon_dir / "blender-mcp-rs.exe",
        addon_dir / "blender_mcp.exe",
        addon_dir.parent.parent / "target" / "release" / "blender-mcp-rs.exe",
    ]
    for path in candidates:
        if path.exists():
            return str(path)
    return str(candidates[0])


def get_mcp():
    """Lazy import of blender_mcp module."""
    global _mcp_module
    if _mcp_module is None:
        from . import blender_mcp
        _mcp_module = blender_mcp
    return _mcp_module


# ============================================================================
# MCP Bridge (copied from blender_mcp_wrapper.py for self-contained addon)
# ============================================================================

class BlenderMcpBridge:
    """Bridge between Rust MCP server and Blender bpy API."""

    def __init__(self, tag: str = "blender"):
        self.tag = tag
        self._mcp = None
        self._timer_running = False

    def start(self, poll_interval: float = 0.05):
        if self._mcp is not None and self._mcp.is_running():
            return

        mcp = get_mcp()
        self._mcp = mcp.BlenderMcp(self.tag)
        self._mcp.start()

        self._timer_running = True
        bpy.app.timers.register(
            lambda: self._poll(poll_interval),
            first_interval=poll_interval
        )

        LOG.info("MCP client started")
        return

    def stop(self):
        self._timer_running = False
        if self._mcp is not None:
            self._mcp.stop()
            LOG.info("MCP client stopped")
            self._mcp = None

    def is_running(self) -> bool:
        return self._mcp is not None and self._mcp.is_running()



    def _poll(self, interval: float):
        if not self._timer_running or self._mcp is None:
            return None
        while True:
            cmd = self._mcp.poll()
            if cmd is None:
                break
            self._process(cmd)
        return interval

    def _process(self, cmd: dict):
        cid, method = cmd["id"], cmd["method"]
        params_raw = cmd["params"] if cmd["params"] else "{}"

        # Every tool lives in the Rust registry; execute_tool runs it against bpy.
        try:
            mod = get_mcp()
            result_json = mod.execute_tool(method, params_raw)
            self._mcp.respond(cid, result_json, None)
            return
        except Exception as e:
            LOG.exception(f"Error in {method}")
            self._mcp.respond(cid, None, str(e))

    def _invoke_tool(self, tool: str, params: dict):
        """Run one tool through the Rust registry, bypassing the IPC round-trip."""
        mod = get_mcp()
        raw = mod.execute_tool(tool, json.dumps(params))
        return json.loads(raw)

# ============================================================================
# Operators
# ============================================================================

class MCP_OT_Start(bpy.types.Operator):
    bl_idname = "mcp.start"
    bl_label = "Start MCP Client"
    bl_description = "Start MCP client for AI agent control"

    def execute(self, context):
        global _bridge
        if _bridge and _bridge.is_running():
            self.report({'WARNING'}, "Already running")
            return {'CANCELLED'}
        try:
            _bridge = BlenderMcpBridge()
            _bridge.start()
            self.report({'INFO'}, "MCP client started")
        except Exception as e:
            self.report({'ERROR'}, str(e))
            return {'CANCELLED'}
        return {'FINISHED'}


class MCP_OT_Stop(bpy.types.Operator):
    bl_idname = "mcp.stop"
    bl_label = "Stop MCP Client"
    bl_description = "Stop MCP client"

    def execute(self, context):
        global _bridge
        if not _bridge or not _bridge.is_running():
            self.report({'WARNING'}, "Client not running")
            return {'CANCELLED'}
        _bridge.stop()
        _bridge = None
        self.report({'INFO'}, "MCP client stopped")
        return {'FINISHED'}


class MCP_OT_Status(bpy.types.Operator):
    bl_idname = "mcp.status"
    bl_label = "MCP Status"
    bl_description = "Show MCP server status"

    def execute(self, context):
        global _bridge
        if _bridge and _bridge.is_running():
            self.report({'INFO'}, "Client running")
        else:
            self.report({'INFO'}, "Client not running")
        return {'FINISHED'}


class MCP_OT_TestHandlers(bpy.types.Operator):
    bl_idname = "mcp.test_handlers"
    bl_label = "Test Handlers"
    bl_description = "Test all MCP handlers locally (no server needed)"

    def execute(self, context):
        global _bridge
        if not _bridge:
            _bridge = BlenderMcpBridge()

        passed = 0
        failed = 0
        errors = []

        # Test each handler with minimal params
        tests = [
            ("scene_get_info", {}),
            ("obj_list", {}),
            ("obj_get_selection", {}),
            ("mat_list", {}),
            ("col_list", {}),
        ]

        for method, params in tests:
            try:
                result = _bridge._invoke_tool(method, params)
                if result is not None:
                    passed += 1
                else:
                    errors.append(f"{method}: returned None")
                    failed += 1
            except Exception as e:
                errors.append(f"{method}: {e}")
                failed += 1

        if failed == 0:
            self.report({'INFO'}, f"All {passed} handlers OK")
        else:
            self.report({'WARNING'}, f"{passed} OK, {failed} failed: {'; '.join(errors[:3])}")

        return {'FINISHED'}


class MCP_OT_CopyCommand(bpy.types.Operator):
    bl_idname = "mcp.copy_command"
    bl_label = "Copy Command"
    bl_description = "Copy MCP command path to clipboard"

    def execute(self, context):
        binary_path = _find_server_binary()
        context.window_manager.clipboard = binary_path
        self.report({'INFO'}, f"Copied: {binary_path}")
        return {'FINISHED'}


class MCP_OT_CopyConfig(bpy.types.Operator):
    bl_idname = "mcp.copy_config"
    bl_label = "Copy Claude Code Config"
    bl_description = "Copy config for Claude Code settings"

    def execute(self, context):
        binary_path = _find_server_binary()
        cfg = f'{{"mcpServers": {{"blender": {{"command": "{binary_path}"}}}}}}'
        context.window_manager.clipboard = cfg
        self.report({'INFO'}, "Config copied! Add to mcpServers in ~/.claude/config.json")
        return {'FINISHED'}


# ============================================================================
# Menu
# ============================================================================

class MCP_MT_Menu(bpy.types.Menu):
    bl_label = "MCP"
    bl_idname = "MCP_MT_menu"

    def draw(self, context):
        layout = self.layout
        global _bridge
        running = _bridge and _bridge.is_running()

        if running:
            layout.label(text="Running", icon='CHECKMARK')
            layout.separator()
            layout.operator("mcp.stop", icon='CANCEL')
        else:
            layout.label(text="Not running", icon='X')
            layout.separator()
            layout.operator("mcp.start", icon='PLAY')

        layout.separator()
        layout.operator("mcp.test_handlers", icon='EXPERIMENTAL')
        layout.operator("mcp.status", icon='INFO')


def menu_draw(self, context):
    self.layout.menu(MCP_MT_Menu.bl_idname)


# ============================================================================
# Registration
# ============================================================================

CLASSES = [
    MCPPreferences,
    MCP_OT_Start,
    MCP_OT_Stop,
    MCP_OT_Status,
    MCP_OT_TestHandlers,
    MCP_OT_CopyCommand,
    MCP_OT_CopyConfig,
    MCP_MT_Menu,
]


@bpy.app.handlers.persistent
def _auto_start_handler(dummy):
    """Handler called after Blender loads - auto-start if enabled."""
    global _bridge

    # Get addon preferences
    prefs = bpy.context.preferences.addons.get(__name__)
    if not prefs:
        return

    if prefs.preferences.auto_start:
        if _bridge and _bridge.is_running():
            return  # Already running

        try:
            _bridge = BlenderMcpBridge()
            _bridge.start()
            LOG.info("MCP auto-started")
        except Exception as e:
            LOG.error(f"MCP auto-start failed: {e}")


def register():
    for cls in CLASSES:
        bpy.utils.register_class(cls)
    bpy.types.TOPBAR_MT_editor_menus.append(menu_draw)

    # Register auto-start handler
    if _auto_start_handler not in bpy.app.handlers.load_post:
        bpy.app.handlers.load_post.append(_auto_start_handler)

    LOG.info("Blender MCP addon registered")

    # Auto-start on first register (when enabling addon)
    # Use timer to delay until Blender is fully ready
    def delayed_auto_start():
        _auto_start_handler(None)
        return None  # Don't repeat

    bpy.app.timers.register(delayed_auto_start, first_interval=0.5)


def unregister():
    global _bridge
    if _bridge:
        _bridge.stop()
        _bridge = None

    # Remove handler
    if _auto_start_handler in bpy.app.handlers.load_post:
        bpy.app.handlers.load_post.remove(_auto_start_handler)

    bpy.types.TOPBAR_MT_editor_menus.remove(menu_draw)
    for cls in reversed(CLASSES):
        bpy.utils.unregister_class(cls)
    LOG.info("Blender MCP addon unregistered")


if __name__ == "__main__":
    register()
