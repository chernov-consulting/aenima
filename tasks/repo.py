"""One-time setup for contributors."""

from __future__ import annotations

import shutil
from pathlib import Path

from invoke import task

ROOT = Path(__file__).resolve().parent.parent


@task
def bootstrap(ctx):
    """Install toolchain-adjacent bits: lefthook hooks, dev-key template, cargo check."""
    _check_tool("cargo")
    _check_tool("docker")
    _check_tool("uv")

    ctx.run("uv sync", echo=True)

    devkeys = ROOT / ".env.devkeys"
    if not devkeys.exists():
        shutil.copy(ROOT / ".env.example", devkeys)
        print(f"  created {devkeys.name} from .env.example — fill in your API keys")

    if shutil.which("lefthook"):
        ctx.run("lefthook install", echo=True)
    else:
        print("  lefthook not installed; skipping hook install "
              "(brew install lefthook to enable pre-commit)")

    ctx.run("cargo check --workspace", echo=True)


def _check_tool(name: str) -> None:
    if not shutil.which(name):
        raise RuntimeError(
            f"required tool '{name}' not found on PATH. "
            "see README.md 'Quick start' for install instructions."
        )
