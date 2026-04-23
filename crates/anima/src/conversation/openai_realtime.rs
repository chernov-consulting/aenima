//! OpenAI Realtime API provider.
//!
//! TODO: wire up the WebSocket at `wss://api.openai.com/v1/realtime?model=...`
//! with the `OpenAI-Beta: realtime=v1` header. This scaffold only implements
//! the struct and `ConversationBackend` impl so the rest of the daemon can
//! compile and the backend selection at startup works.

use anyhow::{Result, bail};
use async_trait::async_trait;

use anima_core::BackendKind;

use super::{ConversationBackend, Session, ToolDef};

pub struct OpenAIRealtimeBackend {
    api_key: String,
    model: String,
}

impl OpenAIRealtimeBackend {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

#[async_trait]
impl ConversationBackend for OpenAIRealtimeBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Openai
    }

    fn label(&self) -> String {
        format!("openai:{}", self.model)
    }

    async fn start_session(
        &self,
        _tools: Vec<ToolDef>,
        _instructions: String,
    ) -> Result<Box<dyn Session>> {
        // Silence unused-field warnings until the WS loop lands.
        let _ = (&self.api_key, &self.model);
        bail!("OpenAI Realtime session not implemented yet — scaffold only")
    }
}
