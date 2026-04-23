//! Shared domain types for the anima daemon.
//!
//! Kept deliberately small. Anything that needs to be usable from both the
//! daemon and a future split-out adapter (e.g. a model-server binary) belongs
//! here. Anything that's only consumed by the daemon itself stays inside
//! `crates/anima`.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Stable identifier for a single realtime voice session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Stable identifier for a single tool invocation within a session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ToolCallId(pub String);

/// Which backend implementation is active. Drives the feature-surface the
/// daemon exposes (e.g. native-audio vs cascaded STT+TTS).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackendKind {
    Openai,
    Gemini,
    /// Local inference on DGX Spark (whisper-rs + llama-cpp-rs + Piper).
    /// Not yet implemented in the prototype scaffold.
    Local,
}

impl std::str::FromStr for BackendKind {
    type Err = BackendParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "openai" => Ok(Self::Openai),
            "gemini" => Ok(Self::Gemini),
            "local" => Ok(Self::Local),
            other => Err(BackendParseError(other.to_owned())),
        }
    }
}

impl std::fmt::Display for BackendKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Openai => "openai",
            Self::Gemini => "gemini",
            Self::Local => "local",
        })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("unknown backend kind: {0!r} (expected 'openai' | 'gemini' | 'local')")]
pub struct BackendParseError(pub String);
