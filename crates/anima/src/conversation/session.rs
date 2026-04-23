//! Session trait and event types.
//!
//! A [`Session`] is the per-call object that higher layers talk to: feed PCM
//! in, iterate events out. All three provider impls return a
//! `Box<dyn Session>` from `start_session`.

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};

use anima_core::ToolCallId;

/// Declarative tool schema passed into `start_session`. Providers translate
/// this into their own tool-definition format (OpenAI function-calling /
/// Gemini function declarations).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    /// JSON Schema (object). We keep it as raw JSON to avoid coupling to any
    /// one schema crate.
    pub parameters: serde_json::Value,
}

/// Event emitted by a provider session.
#[derive(Debug, Clone)]
pub enum SessionEvent {
    /// Partial audio produced by the model. PCM16 at `config.sample_rate`.
    AudioDelta(Bytes),
    /// Partial assistant text (captions / final transcript).
    TextDelta(String),
    /// Model asked for a tool to be invoked.
    ToolCall {
        id: ToolCallId,
        name: String,
        args: serde_json::Value,
    },
    /// Model started a new turn.
    TurnStart,
    /// Model finished a turn. `full_text` is the concatenation of TextDelta.
    TurnEnd { full_text: Option<String> },
    /// Non-fatal error from the provider. The session may continue.
    Warning(String),
}

#[async_trait]
pub trait Session: Send {
    /// Push a PCM frame (PCM16, little-endian, mono, `config.sample_rate` Hz).
    async fn send_audio(&mut self, pcm: Bytes) -> Result<()>;

    /// Return the result of a tool call the model previously requested.
    async fn send_tool_result(
        &mut self,
        id: ToolCallId,
        result: serde_json::Value,
    ) -> Result<()>;

    /// Stream of events from the provider. The caller drives this in a
    /// dedicated task.
    fn events(&mut self) -> BoxStream<'_, SessionEvent>;

    /// Gracefully close the provider WS. `Box<Self>` is required because the
    /// trait is used behind a trait object.
    async fn close(self: Box<Self>) -> Result<()>;
}
