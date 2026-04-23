"""Daemon-side tasks: build, run, lint, test."""

from __future__ import annotations

from invoke import task


@task(help={"release": "Build with --release"})
def run(ctx, release=False):
    """Run the anima daemon (cargo run -p anima)."""
    flags = "--release" if release else ""
    ctx.run(f"cargo run -p anima {flags}".strip(), echo=True, pty=True)


@task
def build(ctx, release=False):
    """Build the daemon."""
    flags = "--release" if release else ""
    ctx.run(f"cargo build -p anima {flags}".strip(), echo=True)


@task
def lint(ctx):
    """cargo fmt --check && cargo clippy -- -D warnings."""
    ctx.run("cargo fmt --all --check", echo=True)
    ctx.run("cargo clippy --workspace --all-targets -- -D warnings", echo=True)


@task
def fmt(ctx):
    """cargo fmt --all."""
    ctx.run("cargo fmt --all", echo=True)


@task
def test(ctx):
    """cargo test --workspace."""
    ctx.run("cargo test --workspace", echo=True, pty=True)


@task(help={"name": "Migration name (snake_case)"})
def migrate_new(ctx, name: str):
    """Create a new sqlx migration (requires sqlx-cli)."""
    ctx.run(f"cd crates/anima && sqlx migrate add {name}", echo=True)
