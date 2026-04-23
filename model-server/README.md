# model-server (placeholder)

Intentionally empty for the MacBook prototype.

On DGX Spark day this directory will hold code for local inference that the
daemon uses via the `LocalBackend` (`crates/anima/src/conversation/local.rs`).
Likely tenants:

- Piper TTS process (spawned + managed by the daemon via stdin/stdout)
- Any helpers that are too large to embed via FFI

Everything that _can_ be FFI'd (whisper-rs, llama-cpp-rs, candle) stays
in-process in the main `anima` crate to avoid IPC on the hot path.
