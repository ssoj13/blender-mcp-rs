#!/usr/bin/env python3
"""
bootstrap.py - build / deploy / test / check for blender-mcp-rs.

Cross-platform, Python 3, stdlib only. Same shape as the gitnexus-rs / squarebob-rs
bootstrap scripts.

One crate produces TWO artifacts, and both are needed:
    blender-mcp-rs(.exe)   the MCP server the agent launches
    blender_mcp.pyd/.so    the Python extension Blender loads (the same tool registry)
So `build` builds both, and `deploy` puts the addon + the extension where Blender
looks for them. Without the deploy step the addon imports nothing and the pipe is
never opened.

Commands:
    b(uild)      cargo build (server binary + Python extension)
    d(eploy)     Copy addon/mcp_server + the extension into Blender's addons dir
    t(est)       cargo test + the stdio smoke test + tools against a real Blender
    c(heck)      cargo fmt --check + clippy (all targets, warnings are errors)
    i(nstall)    Register the server with your AI agents (mcp-setup)
    cl(ean)      cargo clean
    h(elp)       This

Flags:
    -d, --debug        Debug profile (default is release)
    -n, --nocapture    Show test output
    --blender PATH     Blender executable (default: $BLENDER, else a known location)
    --scope SCOPE      install: user (default) or project

Examples:
    python bootstrap.py b
    python bootstrap.py d
    python bootstrap.py t -n
    python bootstrap.py c
    python bootstrap.py i --scope user
"""

from __future__ import annotations

import argparse
import os
import platform
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT_DIR = Path(__file__).parent.resolve()
IS_WINDOWS = platform.system() == "Windows"
IS_MACOS = platform.system() == "Darwin"

BLENDER_SERIES = "5.2"  # the tools target the Blender 5.2 LTS API


class C:
    RST = "\033[0m"
    RED = "\033[91m"
    GRN = "\033[92m"
    YLW = "\033[93m"
    CYN = "\033[96m"
    WHT = "\033[97m"

    @classmethod
    def init(cls) -> None:
        if IS_WINDOWS:
            os.system("")


def fmt_time(ms: float) -> str:
    if ms < 1000:
        return f"{ms:.0f}ms"
    if ms < 60000:
        return f"{ms / 1000:.1f}s"
    mins = int(ms // 60000)
    return f"{mins}m{(ms % 60000) / 1000:.0f}s"


def header(text: str) -> None:
    line = "=" * 60
    print(f"\n{C.CYN}{line}\n{text}\n{line}{C.RST}")


def step(text: str) -> None:
    print(f"  {C.WHT}{text}{C.RST}")


def ok(text: str) -> None:
    print(f"  {C.GRN}[OK] {text}{C.RST}")


def warn(text: str) -> None:
    print(f"  {C.YLW}[WARN] {text}{C.RST}")


def err(text: str) -> None:
    print(f"  {C.RED}[ERR] {text}{C.RST}")


def run(args: list[str], cwd: Path | None = None, capture: bool = False) -> tuple[int, str, float]:
    start = time.perf_counter()
    result = subprocess.run(args, cwd=cwd or ROOT_DIR, capture_output=capture, text=True)
    elapsed_ms = (time.perf_counter() - start) * 1000
    output = (result.stdout or "") + (result.stderr or "") if capture else ""
    return result.returncode, output, elapsed_ms


def which(cmd: str) -> Path | None:
    found = shutil.which(cmd)
    return Path(found) if found else None


def check_cargo() -> bool:
    if not which("cargo"):
        err("Rust/Cargo not found - https://rustup.rs")
        return False
    return True


# ---------------------------------------------------------------------------
# Artifact and Blender locations
# ---------------------------------------------------------------------------

def profile_dir(debug: bool) -> Path:
    return ROOT_DIR / "target" / ("debug" if debug else "release")


def server_bin(debug: bool) -> Path:
    name = "blender-mcp-rs.exe" if IS_WINDOWS else "blender-mcp-rs"
    return profile_dir(debug) / name


def ext_built(debug: bool) -> Path:
    """Where cargo puts the cdylib. Blender wants it renamed (see `ext_name`)."""
    d = profile_dir(debug)
    if IS_WINDOWS:
        return d / "blender_mcp.dll"
    if IS_MACOS:
        return d / "libblender_mcp.dylib"
    return d / "libblender_mcp.so"


def ext_name() -> str:
    """What Python must see for `import blender_mcp` to work."""
    return "blender_mcp.pyd" if IS_WINDOWS else "blender_mcp.so"


def find_blender(explicit: str | None) -> Path | None:
    """Explicit flag, then $BLENDER, then the usual places. Never guess silently."""
    if explicit:
        p = Path(explicit)
        return p if p.exists() else None
    if env := os.environ.get("BLENDER"):
        p = Path(env)
        if p.exists():
            return p
    candidates = []
    if IS_WINDOWS:
        candidates = [
            Path(r"C:\Programs\Blender5\blender.exe"),
            Path(rf"C:\Program Files\Blender Foundation\Blender {BLENDER_SERIES}\blender.exe"),
        ]
    elif IS_MACOS:
        candidates = [Path("/Applications/Blender.app/Contents/MacOS/Blender")]
    else:
        candidates = [Path("/usr/bin/blender"), Path("/usr/local/bin/blender")]
    if found := which("blender"):
        candidates.append(found)
    return next((c for c in candidates if c.exists()), None)


def blender_addons_dir() -> Path:
    if IS_WINDOWS:
        base = Path(os.environ["APPDATA"]) / "Blender Foundation" / "Blender"
    elif IS_MACOS:
        base = Path.home() / "Library" / "Application Support" / "Blender"
    else:
        base = Path.home() / ".config" / "blender"
    return base / BLENDER_SERIES / "scripts" / "addons"


# ---------------------------------------------------------------------------
# Commands
# ---------------------------------------------------------------------------

def run_build(args: argparse.Namespace) -> int:
    header("Build")
    if not check_cargo():
        return 1

    cargo = ["cargo", "build", "--all-targets"]
    if not args.debug:
        cargo.append("--release")

    step(f"cargo build ({'debug' if args.debug else 'release'})")
    code, _, ms = run(cargo)
    if code != 0:
        err("build failed")
        return code

    # Both artifacts come out of the same crate; a missing one means the crate-type
    # or the [[bin]] section was changed, not that the build merely "half worked".
    for path, what in ((server_bin(args.debug), "server binary"),
                       (ext_built(args.debug), "Python extension")):
        if not path.exists():
            err(f"{what} missing after a successful build: {path}")
            return 1

    ok(f"built in {fmt_time(ms)}")
    step(f"  server:    {server_bin(args.debug)}")
    step(f"  extension: {ext_built(args.debug)}")
    step("next: python bootstrap.py d   (deploy the addon into Blender)")
    return 0


def run_deploy(args: argparse.Namespace) -> int:
    header("Deploy addon to Blender")

    ext = ext_built(args.debug)
    if not ext.exists():
        err(f"extension not built: {ext}")
        step("run: python bootstrap.py b")
        return 1

    dest = blender_addons_dir() / "mcp_server"
    step(f"target: {dest}")
    dest.mkdir(parents=True, exist_ok=True)

    src = ROOT_DIR / "addon" / "mcp_server"
    for py in src.glob("*.py"):
        shutil.copy2(py, dest / py.name)
        step(f"  {py.name}")

    # The extension must sit NEXT TO __init__.py: the addon does a plain
    # `import blender_mcp`, and Blender puts the addon's own directory on sys.path.
    target_ext = dest / ext_name()
    shutil.copy2(ext, target_ext)
    step(f"  {ext_name()}  ({ext.stat().st_size // 1024} KB)")

    ok("deployed")
    step("in Blender: Edit > Preferences > Add-ons > 'MCP Server', then MCP > Start")
    return 0


def run_test(args: argparse.Namespace) -> int:
    header("Test")
    if not check_cargo():
        return 1

    # Build first, always. Testing whatever artifact happens to be lying around in
    # target/ is how a stale .pyd once reported green against code that no longer
    # existed - the worst kind of test failure, the silent pass.
    if run_build(args) != 0:
        return 1

    failures: list[str] = []

    step("cargo test (registry guards)")
    cargo = ["cargo", "test", "--lib"]
    if not args.debug:
        cargo.append("--release")
    if args.nocapture:
        cargo += ["--", "--nocapture"]
    if run(cargo)[0] != 0:
        failures.append("cargo test")

    step("stdio smoke test (server, no Blender)")
    if run([sys.executable, str(ROOT_DIR / "python" / "run_tests.py")])[0] != 0:
        failures.append("stdio smoke")

    blender = find_blender(args.blender)
    if not blender:
        warn("Blender not found - skipping the tool tests")
        warn("set $BLENDER or pass --blender PATH")
    else:
        step(f"tools against a real Blender ({blender})")
        script = ROOT_DIR / "python" / "blender_smoke.py"
        if not script.exists():
            warn(f"{script.name} not found - skipping")
        else:
            ext = ext_built(args.debug)
            if not ext.exists():
                err("extension not built; run: python bootstrap.py b")
                return 1
            # Blender imports the extension from the directory we hand it.
            staging = ROOT_DIR / "target" / "blender_test"
            staging.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ext, staging / ext_name())
            env_cmd = [str(blender), "--background", "--factory-startup",
                       "--python", str(script), "--", str(staging)]
            if run(env_cmd)[0] != 0:
                failures.append("blender tools")

            # Per-domain regression suite: one test_<domain>.py per area, run against a
            # fresh scene each. This is where every tool earns lasting coverage; the
            # smoke file above stays a fast cross-section.
            runner = ROOT_DIR / "python" / "tests" / "run.py"
            if runner.exists():
                step("per-domain tool tests")
                dom_cmd = [str(blender), "--background", "--factory-startup",
                           "--python", str(runner), "--", str(staging)]
                if run(dom_cmd)[0] != 0:
                    failures.append("domain tests")

    if failures:
        err(f"failed: {', '.join(failures)}")
        return 1
    ok("all tests passed")
    return 0


def run_check(args: argparse.Namespace) -> int:
    header("Check")
    if not check_cargo():
        return 1
    failures = []

    step("cargo fmt --check")
    if run(["cargo", "fmt", "--check"])[0] != 0:
        failures.append("fmt")

    step("cargo clippy --all-targets -D warnings")
    if run(["cargo", "clippy", "--all-targets", "--", "-D", "warnings"])[0] != 0:
        failures.append("clippy")

    if failures:
        err(f"failed: {', '.join(failures)}")
        return 1
    ok("clean")
    return 0


def run_install(args: argparse.Namespace) -> int:
    header("Register the MCP server with your agents")
    exe = server_bin(args.debug)
    if not exe.exists():
        err(f"server not built: {exe}")
        step("run: python bootstrap.py b")
        return 1
    # The binary registers itself (mcp-setup): it knows its own path, and it is the
    # thing the agent will launch.
    code, _, _ = run([str(exe), "install", "--scope", args.scope])
    if code != 0:
        err("install failed")
        return code
    ok(f"registered (scope={args.scope})")
    step(f"remove with: {exe.name} uninstall")
    return 0


def run_clean(_args: argparse.Namespace) -> int:
    header("Clean")
    code, _, ms = run(["cargo", "clean"])
    if code != 0:
        return code
    ok(f"cleaned in {fmt_time(ms)}")
    return 0


def run_help(_args: argparse.Namespace) -> int:
    print(__doc__)
    return 0


COMMANDS = {
    "b": run_build, "build": run_build,
    "d": run_deploy, "deploy": run_deploy,
    "t": run_test, "test": run_test,
    "c": run_check, "check": run_check,
    "i": run_install, "install": run_install,
    "cl": run_clean, "clean": run_clean,
    "h": run_help, "help": run_help,
}


def main() -> int:
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(line_buffering=True)
    C.init()

    parser = argparse.ArgumentParser(
        description="blender-mcp-rs build system",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("command", nargs="?", choices=sorted(COMMANDS), default="h")
    parser.add_argument("-d", "--debug", action="store_true", help="debug profile")
    parser.add_argument("-n", "--nocapture", action="store_true", help="show test output")
    parser.add_argument("--blender", help="path to the Blender executable")
    parser.add_argument("--scope", default="user", choices=["user", "project"],
                        help="install scope")
    args = parser.parse_args()

    try:
        return COMMANDS[args.command](args)
    except KeyboardInterrupt:
        err("interrupted")
        return 130


if __name__ == "__main__":
    sys.exit(main())
