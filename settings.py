"""Layered .env.* loader for pyinvoke tasks.

Mirrors the relsa/goclaw.io layering (last wins):
  1. .env.base           (committed)
  2. .env.devkeys        (gitignored; API keys)
  3. .env.personal       (gitignored; per-developer overrides)

The Rust daemon does its own .env loading at startup via `dotenvy`; this module
exists so pyinvoke tasks can read the same values when they need to (e.g. to
run `docker compose` with the right ports, or to print the current backend).
"""

from __future__ import annotations

from pathlib import Path

from dotenv import load_dotenv
from pydantic_settings import BaseSettings, SettingsConfigDict

ROOT_PATH = Path(__file__).resolve().parent

# Order matters: later files override earlier ones.
_ENV_FILES = [".env.base", ".env.devkeys", ".env.personal"]
for name in _ENV_FILES:
    path = ROOT_PATH / name
    if path.exists():
        load_dotenv(path, override=True)


class Settings(BaseSettings):
    model_config = SettingsConfigDict(extra="ignore", case_sensitive=False)

    # Daemon
    anima_backend: str = "openai"
    anima_debug_bind: str = "127.0.0.1:7000"
    rust_log: str = "info,anima=debug,sqlx=warn"

    # Postgres
    database_url: str = "postgres://anima:anima@127.0.0.1:5433/anima"
    postgres_host: str = "127.0.0.1"
    postgres_port: int = 5433
    postgres_user: str = "anima"
    postgres_password: str = "anima"
    postgres_db: str = "anima"

    # Providers (keys may be absent in pure-dev environments)
    openai_api_key: str | None = None
    gemini_api_key: str | None = None
    openai_realtime_model: str = "gpt-4o-realtime-preview"
    gemini_live_model: str = "gemini-2.0-flash-live"


settings = Settings()
