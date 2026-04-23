# anima

> **Status:** Prototype scaffold
> **Owner:** Anton Chernov
> **Project:** Codename The Ark

Anima is a real-time conversational AI companion. This repo is the MacBook prototype — a single Rust daemon that streams live voice to and from hosted realtime APIs (OpenAI Realtime, Gemini Live) and grounds responses in a local pgvector-backed memory of ingested documents and media.

The architecture is deliberately identical to the eventual DGX Spark bonsai device: same daemon, same config, same storage. On the device we swap `ANIMA_BACKEND=local` and fill in `crates/anima/src/conversation/local.rs` with `whisper-rs` + `llama-cpp-rs` + Piper.

## Repository structure

```
anima/
  Cargo.toml                  # Rust workspace
  pyproject.toml              # uv; pyinvoke task runner only
  tasks.py                    # root invoke Collection
  settings.py                 # layered .env.* loader for tasks
  .env.base                   # committed safe defaults
  .env.example                # secret template (keys go in .env.devkeys)
  docker-compose.yaml         # Postgres + pgvector
  local                       # smart ./local wrapper
  lefthook.yml                # cargo fmt / clippy / test + ruff
  AGENTS.md                   # Cursor / Claude Code guidance
  docs/                       # overview, architecture, setup
  crates/
    anima/                    # the daemon binary
      src/
        conversation/         # ConversationBackend trait + provider impls
        audio/                # cpal mic/speaker
        memory/               # ingest, embed, pgvector store
        retrieval/            # tool schemas + RAG
        debug_server/         # built-in axum dashboard on localhost:7000
      migrations/             # sqlx migrations (conversations, memories, ...)
    anima-core/               # shared domain types
  tasks/                      # pyinvoke task modules (dev / daemon / repo)
  model-server/               # empty stub; filled on DGX Spark day
```

## Tech stack

| Layer | Choice | Why |
|---|---|---|
| Daemon | Rust (tokio async) | `cpal` + `tokio` for real-time audio+networking; direct FFI to `whisper-rs`/`llama-cpp-rs` on DGX Spark day |
| Storage | Postgres 17 + pgvector | Metadata, vectors, full-text, tool-call log in one transactional store |
| SQL | sqlx (compile-time checked queries) + `pgvector` crate | |
| Audio I/O | cpal | Pure-Rust cross-platform (CoreAudio / ALSA / PipeWire) |
| HTTP / debug UI | axum + tower-http; single vanilla HTML page | No framework |
| Task runner | uv + pyinvoke | Same pattern as `relsa/goclaw.io` |
| Pre-commit | lefthook | Same pattern as `relsa/goclaw.io` |

## Quick start

```bash
# 1. Install toolchains
#    - Rust: https://rustup.rs
#    - Docker Desktop or OrbStack
#    - uv: brew install uv
#    - lefthook: brew install lefthook

# 2. Bootstrap the repo
uv sync
lefthook install
cp .env.example .env.devkeys  # then fill in OPENAI_API_KEY / GEMINI_API_KEY

# 3. Start Postgres
./local up

# 4. Run the daemon
uv run inv daemon.run

# 5. Open the debug dashboard
open http://127.0.0.1:7000
```

## The swappable-backend contract

Three traits live in `crates/anima/src/conversation/`:

- `ConversationBackend` — opens a realtime voice session; streams PCM in, audio/text/tool-call events out.
  - Impls: `OpenAIRealtimeBackend`, `GeminiLiveBackend`, `LocalBackend` (DGX Spark day).
- `EmbeddingBackend` — text + image embeddings for ingestion.
- `CompletionBackend` — non-realtime LLM calls (ingestion summaries, captions).

Selection at runtime via `ANIMA_BACKEND=openai|gemini|local`.

## DGX Spark day

1. `git clone` on the Spark
2. `cargo build --release --target aarch64-unknown-linux-gnu`
3. `ANIMA_BACKEND=local` in `.env.personal`
4. Fill in `crates/anima/src/conversation/local.rs`
5. `systemd` unit launches the binary on boot

No client protocol changes, no architectural changes — same binary shape.

See [docs/architecture.md](docs/architecture.md) for the full picture.
