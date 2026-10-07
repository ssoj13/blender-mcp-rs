#!/usr/bin/env python3
"""Portable CI/release commands, invoked through bootstrap.py (Python 3.11+)."""

from __future__ import annotations

import argparse
import os
import re
import stat
import subprocess
import sys
import tempfile
import tomllib
import zipfile
from pathlib import Path

import bootstrap

ROOT = Path(__file__).resolve().parent
TARGETS = {
    "x86_64-pc-windows-msvc": "windows-x86_64",
    "x86_64-unknown-linux-musl": "linux-x86_64",
    "aarch64-apple-darwin": "macos-aarch64",
}


def run(command: list[str], env: dict[str, str] | None = None, **kwargs):
    print("+ " + " ".join(map(str, command)), flush=True)
    return subprocess.run(command, cwd=ROOT, env=env, check=True, **kwargs)


def rust_host() -> str:
    output = run(["rustc", "-vV"], capture_output=True, text=True).stdout
    return next(line.removeprefix("host: ") for line in output.splitlines()
                if line.startswith("host: "))


def build_env(target: str) -> dict[str, str]:
    env = os.environ.copy()
    # This is a build role, not a Cargo feature: --all-features still includes all
    # tools. Only Blender's invocation keeps the Python handler pointers.
    flags = env.get("RUSTFLAGS", "") + " --cfg blender_mcp_server"
    if target.endswith("windows-msvc"):
        flags += " -C target-feature=+crt-static"
    env["RUSTFLAGS"] = flags.strip()
    # Cargo gives CARGO_ENCODED_RUSTFLAGS precedence over RUSTFLAGS.
    env.pop("CARGO_ENCODED_RUSTFLAGS", None)
    env["CARGO_TARGET_DIR"] = str(ROOT / "target" / "portable")
    if target.endswith("linux-musl"):
        env["PYO3_NO_PYTHON"] = "1"
    return env


def verify_dependencies(binary: Path, target: str, *, extension: bool = False) -> None:
    if target.endswith("windows-msvc"):
        sysroot = Path(run(["rustc", "--print", "sysroot"], capture_output=True,
                           text=True).stdout.strip())
        readobj = sysroot / "lib" / "rustlib" / rust_host() / "bin" / "llvm-readobj.exe"
        output = run([str(readobj), "--coff-imports", str(binary)],
                     capture_output=True, text=True).stdout
        imports = re.findall(r"Name: (\S+\.dll)", output, re.IGNORECASE)
        if not imports:
            raise RuntimeError("LLVM returned no PE imports")
        forbidden = ("vcruntime", "msvcp", "ucrtbase", "api-ms-win-crt")
        if not extension:
            forbidden += ("python",)
        if any(name.lower().startswith(forbidden) for name in imports):
            raise RuntimeError(f"non-portable Windows imports: {imports}")
        print(f"Windows imports: {', '.join(imports)}", flush=True)
    elif target.endswith("linux-musl"):
        headers = run(["readelf", "--program-headers", str(binary)],
                      capture_output=True, text=True).stdout
        dynamic = run(["readelf", "--dynamic", str(binary)],
                      capture_output=True, text=True).stdout
        if "INTERP" in headers or "(NEEDED)" in dynamic:
            raise RuntimeError("Linux binary requires a dynamic loader or shared libraries")
    elif "linux" in target:
        output = run(["readelf", "--version-info", str(binary)],
                     capture_output=True, text=True).stdout
        versions = [tuple(map(int, version.split(".")))
                    for version in re.findall(r"GLIBC_(\d+(?:\.\d+)+)", output)]
        if not versions or max(versions) > (2, 28):
            raise RuntimeError(f"extension requires glibc newer than 2.28: {versions}")
        dynamic = run(["readelf", "--dynamic", str(binary)],
                      capture_output=True, text=True).stdout
        libraries = re.findall(r"\(NEEDED\).*\[(.*?)\]", dynamic)
        allowed = {"libc.so.6", "libm.so.6", "libdl.so.2", "libpthread.so.0",
                   "librt.so.1", "libgcc_s.so.1", "ld-linux-x86-64.so.2"}
        if not libraries or set(libraries) - allowed:
            raise RuntimeError(f"unexpected extension dependencies: {libraries}")
        print(f"Extension glibc baseline: {'.'.join(map(str, max(versions)))}", flush=True)
    else:
        output = run(["otool", "-L", str(binary)], capture_output=True, text=True).stdout
        libraries = [line.strip().split(" (", 1)[0] for line in output.splitlines()[1:]]
        if extension:
            # otool also prints the dylib's own install ID, before its dependencies.
            libraries = libraries[1:]
        if not libraries or any(not name.startswith(("/usr/lib/", "/System/Library/"))
                                for name in libraries):
            raise RuntimeError(f"non-system macOS dependencies: {libraries}")


def build_extension(host: str) -> Path:
    env = os.environ.copy()
    # Separate target directory and flags: a server-role cdylib has no handlers
    # and must never be mistaken for the module that Blender executes.
    env["CARGO_TARGET_DIR"] = str(ROOT / "target" / "extension")
    env["RUSTFLAGS"] = "-C target-feature=+crt-static" if "windows" in host else ""
    env.pop("CARGO_ENCODED_RUSTFLAGS", None)
    env.pop("PYO3_NO_PYTHON", None)
    env["PYO3_PYTHON"] = sys.executable
    if "linux" in host:
        # Blender uses glibc, so the server's static musl library cannot be loaded.
        # Zig targets Blender's minimum supported glibc instead of the runner's.
        env["CARGO_ZIGBUILD_PYTHON_PATH"] = sys.executable
        command = ["cargo", "zigbuild"]
        target = f"{host}.2.28"
    else:
        command = ["cargo", "build"]
        target = host
    run([*command, "--locked", "--all-features", "--release", "--lib",
         "--target", target], env)
    extension = Path(env["CARGO_TARGET_DIR"]) / host / "release" / bootstrap.ext_built(False).name
    verify_dependencies(extension, host, extension=True)
    return extension


def package(binary: Path, extension: Path, target: str, version: str) -> Path:
    destination = ROOT / "dist" / f"blender-mcp-rs-v{version}-{TARGETS[target]}.zip"
    destination.parent.mkdir(parents=True, exist_ok=True)
    payload = {binary.name: binary, "bootstrap.py": ROOT / "bootstrap.py"}
    payload.update({f"mcp_server/{py.name}": py
                    for py in sorted((ROOT / "addon" / "mcp_server").glob("*.py"))})
    payload[f"mcp_server/{bootstrap.ext_name()}"] = extension
    with zipfile.ZipFile(destination, "w", compression=zipfile.ZIP_DEFLATED,
                         compresslevel=9) as archive:
        for name, source in payload.items():
            entry = zipfile.ZipInfo(name)
            entry.create_system = 3
            mode = 0o755 if name == binary.name else 0o644
            entry.external_attr = (stat.S_IFREG | mode) << 16
            entry.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(entry, source.read_bytes())
    with zipfile.ZipFile(destination) as archive:
        if archive.namelist() != list(payload) or archive.testzip() is not None:
            raise RuntimeError("release ZIP failed verification")
        with tempfile.TemporaryDirectory(prefix="blender-mcp-release-") as directory:
            archive.extractall(directory)
            extracted = Path(directory) / binary.name
            extracted.chmod(0o755)
            # No Python directory in the child's PATH: the executable must start
            # without finding a Python DLL beside a build-time interpreter.
            env = os.environ.copy()
            env["PATH"] = ""
            for key in ("PYTHONHOME", "PYTHONPATH", "LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH"):
                env.pop(key, None)
            output = subprocess.run([str(extracted), "--version"], cwd=directory,
                                    env=env, check=True, capture_output=True,
                                    text=True, timeout=30).stdout.strip()
            if output != f"blender-mcp-rs {version}":
                raise RuntimeError(f"unexpected binary version: {output!r}")
            # Import the actual module from the archive, then call a registered
            # handler. Without Blender it must fail importing bpy, not report
            # that handlers were stripped out by the server build role.
            subprocess.run([sys.executable, "-I", "-c", """
import sys
from pathlib import Path
from argparse import Namespace
root = Path(sys.argv[1])
sys.path.insert(0, str(root))
import bootstrap
assert bootstrap.server_bin(False) == root / sys.argv[2]
deploy = root / 'deploy-test'
bootstrap.blender_addons_dir = lambda: deploy
assert bootstrap.run_deploy(Namespace(debug=False)) == 0
for source in (root / 'mcp_server').iterdir():
    assert (deploy / 'mcp_server' / source.name).read_bytes() == source.read_bytes()
sys.path.insert(0, str(deploy / 'mcp_server'))
import blender_mcp
try:
    blender_mcp.execute_tool('scene_get_info', '{}')
except ModuleNotFoundError as error:
    if error.name != 'bpy':
        raise
else:
    raise RuntimeError('expected a real Blender handler requiring bpy')
print('Packaged native module: import and handler verified')
""", directory, binary.name], check=True, timeout=30)
    print(f"Release ZIP: {destination}", flush=True)
    return destination


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("ci", "release"))
    parser.add_argument("--target", choices=TARGETS)
    args = parser.parse_args()
    host = rust_host()
    target = args.target or ("x86_64-unknown-linux-musl" if "linux" in host else host)
    if target not in TARGETS:
        parser.error(f"unsupported release host: {host}")
    # Smoke tests run on the build host; supported CI runners are native for each
    # release architecture (Linux's libc variant is the only difference).
    if host.split("-", 1)[0] != target.split("-", 1)[0] or not any(
        platform in host and platform in target for platform in ("windows", "linux", "darwin")
    ):
        parser.error(f"{target} requires a native runner, got {host}")

    version = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))["package"]["version"]
    if os.environ.get("GITHUB_REF_TYPE") == "tag":
        tag = os.environ.get("GITHUB_REF_NAME")
        if tag != f"v{version}":
            parser.error(f"release tag {tag!r} must match Cargo.toml: v{version}")

    env = build_env(target)
    common = ["--locked", "--workspace", "--all-features", "--target", target]
    if args.command == "ci":
        run(["cargo", "fmt", "--all", "--check"])
        # Type-check the actual Blender handlers as well as the portable registry.
        run(["cargo", "clippy", "--locked", "--workspace", "--all-features",
             "--all-targets", "--", "-D", "warnings"])
        run(["cargo", "clippy", *common, "--all-targets", "--", "-D", "warnings"], env)
        run(["cargo", "test", *common, "--all-targets"], env)
    run(["cargo", "build", *common, "--release", "--bin", "blender-mcp-rs"], env)
    binary = Path(env["CARGO_TARGET_DIR"]) / target / "release" / (
        "blender-mcp-rs.exe" if "windows" in target else "blender-mcp-rs")
    verify_dependencies(binary, target)
    smoke_env = os.environ.copy()
    smoke_env["BLENDER_MCP_BINARY"] = str(binary)
    run([sys.executable, "-u", "python/test_mcp_server.py"],
        smoke_env, timeout=120)
    extension = build_extension(host)
    package(binary, extension, target, version)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
