//! Host audio I/O via cpal.
//!
//! Provides two building blocks used by the session:
//! - [`input::capture_default`] — opens the selected input device and streams
//!   PCM16 frames on a `tokio::sync::mpsc::Receiver<Bytes>`.
//! - [`output::Player`] — opens the selected output device and plays PCM16
//!   frames pushed via `play(Bytes)`.
//!
//! Stubs for now — the real implementations land alongside the first working
//! OpenAI Realtime loop.

pub mod input;
pub mod output;

/// Canonical audio format we speak to providers in.
pub const CHANNELS: u16 = 1;
pub const SAMPLE_FORMAT_BITS: u16 = 16;
