#!/usr/bin/env python3
"""Build `kdb/` — the knowledge base the server ships and searches.

Three raw sources, all pinned to the targeted Blender version, plus the
hand-written guides that live in `kdb/guides/` and are never touched here:

    kdb/api/            generated from the LIVE Blender (dump_api.py): RNA types,
                        operator signatures, real node socket names. This is the
                        "ask Blender, don't guess" rule as data.
    kdb/release-notes/  what changed per version — why code written for an older
                        Blender breaks on this one.
    kdb/manual/         the user manual for this version, reStructuredText flattened
                        to plain Markdown-ish text.

Everything written here is derived: `kdb build` may delete and regenerate all of it.
`kdb/guides/` is ours and is left alone.

Usage:
    python python/kdb_build.py --blender <blender.exe> [--work <dir>] [--skip-manual]
"""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
KDB = ROOT / "kdb"

MANUAL_REPO = "https://projects.blender.org/blender/blender-manual.git"
DEVDOCS_REPO = "https://projects.blender.org/blender/blender-developer-docs.git"
# Release notes worth shipping: the targeted version, what came before it (so an
# agent can tell "this changed recently" from "this was always so"), and the next
# one as a preview.
NOTE_VERSIONS = ["5.0", "5.1", "5.2", "5.3"]


def run(args: list[str], cwd: Path | None = None) -> None:
    proc = subprocess.run(args, cwd=cwd, text=True, capture_output=True)
    if proc.returncode != 0:
        raise SystemExit(f"failed: {' '.join(args)}\n{proc.stderr.strip()}")


def blender_version(blender: Path) -> str:
    out = subprocess.run([str(blender), "--version"], text=True, capture_output=True).stdout
    m = re.search(r"Blender (\d+\.\d+)", out)
    if not m:
        raise SystemExit(f"cannot read a version out of {blender}")
    return m.group(1)


# ---------------------------------------------------------------------------
# reStructuredText -> plain text
# ---------------------------------------------------------------------------

ROLE = re.compile(r":[a-z:]+:`([^`]*)`")          # :ref:`Foo <bar>` -> Foo
DIRECTIVE = re.compile(r"^\s*\.\.\s+(\w+)::.*$")   # .. figure:: /images/x.png
COMMENT = re.compile(r"^\s*\.\.\s*$")
# Directives whose *body* is prose worth keeping (the rest are images/markup).
KEEP_BODY = {"note", "tip", "warning", "important", "admonition", "seealso", "hint"}


def rst_to_text(src: str) -> str:
    out: list[str] = []
    skip_indent: int | None = None
    for raw in src.splitlines():
        line = raw.rstrip()
        if skip_indent is not None:
            if not line.strip():
                out.append("")
                continue
            indent = len(line) - len(line.lstrip())
            if indent > skip_indent:
                continue
            skip_indent = None
        m = DIRECTIVE.match(line)
        if m:
            if m.group(1) in KEEP_BODY:
                out.append(f"**{m.group(1).title()}:**")
            else:
                skip_indent = len(line) - len(line.lstrip())
            continue
        if COMMENT.match(line):
            continue
        line = ROLE.sub(r"\1", line)
        line = re.sub(r"``([^`]*)``", r"`\1`", line)
        line = re.sub(r"\s*<[^<>]*>\s*$", "", line) if line.startswith("   ") else line
        out.append(line)
    text = "\n".join(out)
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.strip() + "\n"


def convert_manual(manual_root: Path, dest: Path, version: str) -> int:
    count = 0
    for rst in sorted(manual_root.rglob("*.rst")):
        rel = rst.relative_to(manual_root).with_suffix(".md")
        target = dest / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        body = rst_to_text(rst.read_text(encoding="utf-8", errors="replace"))
        if len(body.strip()) < 40:  # index stubs that are only a toctree
            continue
        area = rel.parts[0] if len(rel.parts) > 1 else "manual"
        target.write_text(
            f"---\narea: manual-{area}\nsource: blender-manual\nversion: {version}\n---\n\n{body}",
            encoding="utf-8", newline="\n",
        )
        count += 1
    return count


# ---------------------------------------------------------------------------
# steps
# ---------------------------------------------------------------------------

def build_api(blender: Path, version: str) -> None:
    dest = KDB / "api"
    if dest.exists():
        shutil.rmtree(dest)
    script = ROOT / "python" / "dump_api.py"
    proc = subprocess.run(
        [str(blender), "--background", "--factory-startup", "--python", str(script),
         "--", str(dest)],
        text=True, capture_output=True,
    )
    if "KDB_API_DUMP_DONE" not in proc.stdout:
        raise SystemExit(f"API dump failed:\n{proc.stdout[-2000:]}\n{proc.stderr[-2000:]}")
    print(f"  api: {sum(1 for _ in dest.rglob('*.md'))} files")


def build_release_notes(work: Path) -> None:
    repo = work / "devdocs"
    if not repo.exists():
        run(["git", "clone", "--depth", "1", "--filter=blob:none", "--sparse",
             DEVDOCS_REPO, str(repo)])
        run(["git", "sparse-checkout", "set", "--no-cone", "/docs/release_notes/**/*.md"],
            cwd=repo)
    dest = KDB / "release-notes"
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    n = 0
    for ver in NOTE_VERSIONS:
        src_dir = repo / "docs" / "release_notes" / ver
        if not src_dir.is_dir():
            print(f"  release-notes: {ver} not published yet, skipped")
            continue
        for md in sorted(src_dir.glob("*.md")):
            body = md.read_text(encoding="utf-8", errors="replace")
            (dest / f"{ver}-{md.name}").write_text(
                f"---\narea: release-notes\nsource: blender-{ver}\nversion: {ver}\n---\n\n{body}",
                encoding="utf-8", newline="\n",
            )
            n += 1
    print(f"  release-notes: {n} files")


def build_manual(work: Path, version: str) -> None:
    repo = work / "manual"
    branch = f"blender-v{version}-release"
    if not repo.exists():
        run(["git", "clone", "--depth", "1", "--filter=blob:none", "--sparse",
             "-b", branch, MANUAL_REPO, str(repo)])
        run(["git", "sparse-checkout", "set", "--no-cone", "/manual/**/*.rst"], cwd=repo)
    dest = KDB / "manual"
    if dest.exists():
        shutil.rmtree(dest)
    n = convert_manual(repo / "manual", dest, version)
    print(f"  manual: {n} files")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--blender", required=True, help="Blender executable to introspect")
    ap.add_argument("--work", default=None, help="checkout cache (default: <temp>/kdb-src)")
    ap.add_argument("--skip-manual", action="store_true")
    args = ap.parse_args()

    blender = Path(args.blender)
    if not blender.exists():
        raise SystemExit(f"no Blender at {blender}")
    work = Path(args.work) if args.work else Path(__import__("tempfile").gettempdir()) / "kdb-src"
    work.mkdir(parents=True, exist_ok=True)

    version = blender_version(blender)
    print(f"kdb build for Blender {version}")
    build_api(blender, version)
    build_release_notes(work)
    if not args.skip_manual:
        build_manual(work, version)

    total = sum(f.stat().st_size for f in KDB.rglob("*.md"))
    print(f"kdb: {sum(1 for _ in KDB.rglob('*.md'))} files, {total / 1_048_576:.1f} MB")
    return 0


if __name__ == "__main__":
    sys.exit(main())
