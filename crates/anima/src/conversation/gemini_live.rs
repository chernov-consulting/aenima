//! Gemini Live API provider.
//!
//! TODO: wire up the WebSocket at
//! `wss://generativelanguage.googleapis.com/ws/google.ai.generativelanguage.v1beta.GenerativeService.BidiGenerateContent?key=...`
//! This scaffold only implements the struct and `ConversationBackend` impl.

use anyhow::{Result, bail};
use async_trait::async_trait;

use anima_core::BackendKind;

use super::{ConversationBackend, Session, ToolDef};

pub struct GeminiLiveBackend {
    api_key: String,
    model: String,
}

impl GeminiLiveBackend {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

#[async_trait]
impl ConversationBackend for GeminiLiveBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Gemini
    }

    fn label(&self) -> String {
        format!("gemini:{}", self.model)
    }

    async fn start_session(
        &self,
        _tools: Vec<ToolDef>,
        _instructions: String,
    ) -> Result<Box<dyn Session>> {
        let _ = (&self.api_key, &self.model);
        bail!("Gemini Live session not implemented yet — scaffold only")
    }
}
