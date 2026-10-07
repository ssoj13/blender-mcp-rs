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


def verify_dependencies(binary: Path, target: str) -> None:
    if target.endswith("windows-msvc"):
        sysroot = Path(run(["rustc", "--print", "sysroot"], capture_output=True,
                           text=True).stdout.strip())
        readobj = sysroot / "lib" / "rustlib" / rust_host() / "bin" / "llvm-readobj.exe"
        output = run([str(readobj), "--coff-imports", str(binary)],
                     capture_output=True, text=True).stdout
        imports = re.findall(r"Name: (\S+\.dll)", output, re.IGNORECASE)
        if not imports:
            raise RuntimeError("LLVM returned no PE imports")
        forbidden = ("python", "vcruntime", "msvcp", "ucrtbase", "api-ms-win-crt")
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
    else:
        output = run(["otool", "-L", str(binary)], capture_output=True, text=True).stdout
        libraries = [line.strip().split(" (", 1)[0] for line in output.splitlines()[1:]]
        if not libraries or any(not name.startswith(("/usr/lib/", "/System/Library/"))
                                for name in libraries):
            raise RuntimeError(f"non-system macOS dependencies: {libraries}")


def package(binary: Path, target: str, version: str) -> Path:
    destination = ROOT / "dist" / f"blender-mcp-rs-v{version}-{TARGETS[target]}.zip"
    destination.parent.mkdir(parents=True, exist_ok=True)
    # Explicit Unix attributes preserve executable permissions even if the ZIP is
    # produced on Windows. There is exactly one file, at the archive root.
    entry = zipfile.ZipInfo(binary.name)
    entry.create_system = 3
    entry.external_attr = (stat.S_IFREG | 0o755) << 16
    entry.compress_type = zipfile.ZIP_DEFLATED
    with zipfile.ZipFile(destination, "w", compression=zipfile.ZIP_DEFLATED,
                         compresslevel=9) as archive:
        archive.writestr(entry, binary.read_bytes())
    with zipfile.ZipFile(destination) as archive:
        if archive.namelist() != [binary.name] or archive.testzip() is not None:
            raise RuntimeError("release ZIP failed verification")
        with tempfile.TemporaryDirectory(prefix="blender-mcp-release-") as directory:
            extracted = Path(archive.extract(binary.name, directory))
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
    package(binary, target, version)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
