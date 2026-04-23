"""Day-to-day dev helpers: bring up side services, seed data, poke the daemon."""

from __future__ import annotations

from pathlib import Path

from invoke import task

ROOT = Path(__file__).resolve().parent.parent


@task(help={"detach": "Run in background (default true)"})
def up(ctx, detach=True):
    """Start the local dev stack (currently: postgres+pgvector)."""
    ctx.run("./local up", echo=True, pty=True)


@task
def down(ctx):
    """Stop the local dev stack."""
    ctx.run("./local down", echo=True)


@task
def clean(ctx):
    """Stop and wipe the local postgres volume."""
    ctx.run("./local clean", echo=True)


@task
def logs(ctx):
    """Tail docker logs."""
    ctx.run("./local logs", echo=True, pty=True)


@task(help={"path": "File to ingest (pdf, md, txt, jpg, png, mp4, wav)"})
def ingest(ctx, path: str):
    """Send a local file to the running daemon's /ingest endpoint."""
    import os

    bind = os.environ.get("ANIMA_DEBUG_BIND", "127.0.0.1:7000")
    ctx.run(
        f'curl -fsS -X POST -F "file=@{path}" http://{bind}/ingest',
        echo=True,
        pty=True,
    )


@task(help={"text": "Text to synthesize and feed into the daemon"})
def test_voice(ctx, text: str = "hello anima"):
    """Trigger a canned voice turn against the running daemon (stub)."""
    print("  [stub] not yet wired — will synthesize PCM via the active provider")
    print(f"  text: {text!r}")
