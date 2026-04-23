# AGENTS.md

Guidance for Cursor / Claude Code / other agents working in this repository.

## What this project is

Anima is a real-time voice companion. The **product itself is one Rust daemon** that runs on a MacBook during prototyping and on an NVIDIA DGX Spark inside a bonsai pot in production. There is **no separate backend service and no GUI application** — the device is voice-first and has no screen; the MacBook prototype mirrors that shape.

A small built-in debug dashboard (`axum` on `localhost:7000`) exists only for development visibility. Treat it as a debugging tool, not a UI.

## Commands

### Local development

```bash
./local up          # start postgres+pgvector in docker
./local down        # stop
./local clean       # stop and delete volumes
```

```bash
uv run inv daemon.run       # cargo run -p anima (loads layered .env.*)
uv run inv daemon.lint      # cargo fmt --check && cargo clippy -- -D warnings
uv run inv daemon.test      # cargo test --workspace
uv run inv dev.ingest --path PATH
uv run inv dev.test-voice --text "..."
```

### Pre-commit

`lefthook install` registers hooks. They run `cargo fmt --check`, `cargo clippy -- -D warnings`, and `ruff` on the Python task files.

## Architecture

```
+-----------------------------------------------+
|  anima daemon (Rust, single tokio runtime)    |
|                                                |
|  [audio::input (cpal)]                         |
|            |                                   |
|            v                                   |
|  [conversation::Session]  <----+               |
|            |                   |               |
|            v                   |               |
|  [ConversationBackend impl]    |               |
|            |                   |               |
|            v                   |               |
|     provider WS  ----------> tool_call         |
|     (OpenAI / Gemini)         dispatch         |
|            |                   |               |
|            v                   v               |
|  [audio::output (cpal)]  [retrieval::tools]    |
|                                |               |
|                                v               |
|                          [memory::store]       |
|                          (Postgres+pgvector)   |
|                                                |
|  [debug_server (axum)] - localhost:7000        |
+------------------------------------------------+
```

### The three provider traits

Everything above the provider only depends on these; never import concrete client types outside `conversation/` and `memory/embed.rs`.

| Trait | What | Impls shipped | Impls later |
|---|---|---|---|
| `ConversationBackend` | Bi-directional realtime voice session | `OpenAIRealtimeBackend`, `GeminiLiveBackend` | `LocalBackend` (whisper-rs + llama-cpp-rs + Piper) |
| `EmbeddingBackend` | Text/image embeddings for ingestion | `OpenAIEmbeddingBackend` | `BGELocalBackend` |
| `CompletionBackend` | Non-realtime LLM (summaries, captions) | `OpenAIChatBackend`, `GeminiChatBackend` | `LocalLlamaBackend` |

Selection via `ANIMA_BACKEND=openai|gemini|local` (and per-feature overrides if needed).

### Data model (sqlx migrations)

| Table | Purpose |
|---|---|
| `conversations` | One row per voice session |
| `messages` | User/assistant turns with audio pointers |
| `memories` | Chunks of ingested content; `vector(1536)` column for embedding; tsvector for hybrid search |
| `media_assets` | Files on disk (`data/media/`) + metadata + thumbnail path |
| `tool_calls` | Audit log of tool invocations and results |

## Code conventions

### Rust

- Edition 2024.
- `anyhow::Result` at boundaries; `thiserror` for public error types in `anima-core`.
- Always wrap context: `.context("opening provider WS")?` — never bare `?` at an error boundary.
- `tokio::spawn` for long-lived tasks; always attach a `tracing::Instrument` span.
- `async_trait` is fine — we don't need the perf edge here.
- Public functions get rustdoc; non-trivial internal functions get `// why` comments, not `// what`.
- Functions ≲ 60 lines; files ≲ 500 lines. Refactor before exceeding.
- No `unwrap()` in non-test code. `.expect("reason")` only during startup with a clear invariant.
- Never silence errors — log and either retry with backoff or propagate.

### Python (tasks/ only)

- Python is **only** for pyinvoke tasks. No business logic lives in Python.
- `ruff` + `ruff format`. Type hints required.

### Secrets

Layered env loading order (last wins):

1. `.env.base` — committed, safe defaults
2. `.env.devkeys` — gitignored, real API keys
3. `.env.personal` — gitignored, developer overrides

Only `.env.base` and `.env.example` are committed.

## Default environment

When investigating an issue without a specified environment, assume **local**: the daemon running via `cargo run` and Postgres in docker compose. Read logs from the daemon's stdout or `http://127.0.0.1:7000/log-stream`.

## What NOT to do

- Do **not** add a separate "backend" service, web server, or API gateway. The daemon owns everything.
- Do **not** introduce a GUI framework (Tauri, Flutter, Electron, ...). If the debug dashboard needs more, add HTML/JS to `crates/anima/src/debug_server/static/`.
- Do **not** add a second datastore for embeddings or metadata. pgvector handles both.
- Do **not** swallow the DGX Spark story: every new abstraction must have a clear path to the `local` backend impl. Prompt-stuffing external data is forbidden — all grounding goes through tools.
- Do **not** commit `.env.devkeys`, `.env.personal`, or anything in `data/`.

## Potential downsides

After any significant change, add a short "Potential downsides" note covering performance, edge cases, maintainability, or security risks specific to the change. Skip for trivially safe edits.
