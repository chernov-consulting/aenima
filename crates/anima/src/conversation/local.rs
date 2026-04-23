//! Local inference backend for DGX Spark day.
//!
//! Plan:
//! - STT:  whisper-rs (in-process, FFI to whisper.cpp)
//! - LLM:  llama-cpp-rs (in-process, FFI to llama.cpp; target Gemma 4 26B-A4B at Q4)
//! - TTS:  Piper subprocess (lightweight; subprocess is fine since output
//!         audio is the bottleneck, not control plane)
//!
//! The interesting piece is reconstructing realtime semantics (barge-in,
//! partial transcripts) on top of a cascade. See docs/architecture.md for
//! the design.
//!
//! Not implemented in the prototype scaffold.

#![allow(dead_code)]
