//! The swappable realtime-voice backend.
//!
//! Everything above this module depends only on [`ConversationBackend`] and
//! [`Session`]; concrete provider clients live in submodules and are selected
//! at startup by `build_backend`.

use anyhow::{Result, bail};
use async_trait::async_trait;
use bytes::Bytes;
use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};

use anima_core::{BackendKind, ToolCallId};

use crate::config::Config;

pub mod gemini_live;
pub mod local;
pub mod openai_realtime;
pub mod session;

pub use session::{Session, SessionEvent, ToolDef};

/// Trait object returned by `build_backend`. Cheap to `Arc` internally — we
/// don't parameterise higher layers on the concrete type.
#[async_trait]
pub trait ConversationBackend: Send + Sync {
    fn kind(&self) -> BackendKind;

    /// Human-readable identifier for logs / dashboard display.
    fn label(&self) -> String {
        self.kind().to_string()
    }

    /// Open a new realtime voice session with the given tools and system
    /// instructions.
    async fn start_session(
        &self,
        tools: Vec<ToolDef>,
        instructions: String,
    ) -> Result<Box<dyn Session>>;
}

/// Pick a backend based on `config.backend` and construct it. Missing API keys
/// produce a clear error at startup rather than a late failure.
pub fn build_backend(config: &Config) -> Result<Box<dyn ConversationBackend>> {
    match config.backend {
        BackendKind::Openai => {
            let key = config
                .openai
                .api_key
                .clone()
                .ok_or_else(|| anyhow::anyhow!("ANIMA_BACKEND=openai but OPENAI_API_KEY is unset"))?;
            Ok(Box::new(openai_realtime::OpenAIRealtimeBackend::new(
                key,
                config.openai.realtime_model.clone(),
            )))
        }
        BackendKind::Gemini => {
            let key = config
                .gemini
                .api_key
                .clone()
                .ok_or_else(|| anyhow::anyhow!("ANIMA_BACKEND=gemini but GEMINI_API_KEY is unset"))?;
            Ok(Box::new(gemini_live::GeminiLiveBackend::new(
                key,
                config.gemini.live_model.clone(),
            )))
        }
        BackendKind::Local => {
            bail!(
                "ANIMA_BACKEND=local requires whisper-rs + llama-cpp-rs; not implemented in the \
                 prototype. Fill in crates/anima/src/conversation/local.rs on DGX Spark day."
            )
        }
    }
}

// Re-exports for convenience.
pub use anima_core::{BackendKind as Kind, ToolCallId as CallId};

/// Shorthand so callers can say `conversation::Audio(bytes)` etc. without
/// depending on `bytes::Bytes` directly. Mostly cosmetic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio(#[serde(with = "base64_bytes")] pub Bytes);

mod base64_bytes {
    use bytes::Bytes;
    use serde::{Deserializer, Serializer, de::Error};

    pub fn serialize<S: Serializer>(v: &Bytes, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&base64_encode(v))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Bytes, D::Error> {
        let s: String = serde::Deserialize::deserialize(d)?;
        base64_decode(&s).map(Bytes::from).map_err(D::Error::custom)
    }

    // Minimal base64 (standard alphabet, no padding-strictness) to avoid
    // pulling a dep for a single use-site. Replace with `base64` crate once
    // ingestion or other code needs it.
    fn base64_encode(b: &[u8]) -> String {
        const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::with_capacity((b.len() + 2) / 3 * 4);
        for chunk in b.chunks(3) {
            let (b0, b1, b2) = (
                chunk[0] as u32,
                *chunk.get(1).unwrap_or(&0) as u32,
                *chunk.get(2).unwrap_or(&0) as u32,
            );
            let n = (b0 << 16) | (b1 << 8) | b2;
            out.push(A[((n >> 18) & 0x3F) as usize] as char);
            out.push(A[((n >> 12) & 0x3F) as usize] as char);
            out.push(if chunk.len() > 1 { A[((n >> 6) & 0x3F) as usize] as char } else { '=' });
            out.push(if chunk.len() > 2 { A[(n & 0x3F) as usize] as char } else { '=' });
        }
        out
    }

    fn base64_decode(s: &str) -> Result<Vec<u8>, String> {
        fn v(c: u8) -> Result<u32, String> {
            Ok(match c {
                b'A'..=b'Z' => (c - b'A') as u32,
                b'a'..=b'z' => (c - b'a' + 26) as u32,
                b'0'..=b'9' => (c - b'0' + 52) as u32,
                b'+' => 62,
                b'/' => 63,
                _ => return Err(format!("invalid base64 byte: {c:#x}")),
            })
        }
        let bytes: Vec<u8> = s.bytes().filter(|&c| c != b'=' && !c.is_ascii_whitespace()).collect();
        let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
        for chunk in bytes.chunks(4) {
            let mut n = 0u32;
            for (i, &b) in chunk.iter().enumerate() {
                n |= v(b)? << (18 - i * 6);
            }
            out.push((n >> 16) as u8);
            if chunk.len() > 2 {
                out.push((n >> 8) as u8);
            }
            if chunk.len() > 3 {
                out.push(n as u8);
            }
        }
        Ok(out)
    }
}

// Keep unused re-exports from flagging until downstream code consumes them.
#[allow(dead_code)]
pub(crate) fn _compile_checks(_: ToolCallId) {}
