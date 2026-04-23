//! Declarative tool registry.
//!
//! Tools are declared as JSON schemas so each `ConversationBackend` impl can
//! translate them into its own tool-definition format (OpenAI function
//! tools vs. Gemini function declarations). Dispatching happens here via
//! [`ToolRegistry::invoke`].

use std::collections::HashMap;

use anyhow::{Result, anyhow};
use serde_json::{Value, json};

use crate::conversation::ToolDef;

/// Closed set of tool names the daemon knows how to invoke. Everything the
/// model can call must be listed here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolName {
    RetrieveMemory,
    RememberThis,
    SearchMedia,
    CurrentTime,
}

impl ToolName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RetrieveMemory => "retrieve_memory",
            Self::RememberThis => "remember_this",
            Self::SearchMedia => "search_media",
            Self::CurrentTime => "current_time",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "retrieve_memory" => Some(Self::RetrieveMemory),
            "remember_this" => Some(Self::RememberThis),
            "search_media" => Some(Self::SearchMedia),
            "current_time" => Some(Self::CurrentTime),
            _ => None,
        }
    }
}

/// Typed description a provider translates into its own tool-call format.
pub struct ToolRegistry {
    definitions: HashMap<ToolName, ToolDef>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        let mut definitions = HashMap::new();
        for (name, def) in [
            (ToolName::RetrieveMemory, retrieve_memory_def()),
            (ToolName::RememberThis, remember_this_def()),
            (ToolName::SearchMedia, search_media_def()),
            (ToolName::CurrentTime, current_time_def()),
        ] {
            definitions.insert(name, def);
        }
        Self { definitions }
    }
}

impl ToolRegistry {
    pub fn defs(&self) -> Vec<ToolDef> {
        self.definitions.values().cloned().collect()
    }

    /// Invoke a tool by name. Real implementations will live in
    /// `retrieval::rag` and call into `MemoryStore`; for now we only handle
    /// `current_time` so the end-to-end path has one working tool.
    pub async fn invoke(&self, name: &str, args: Value) -> Result<Value> {
        let Some(tool) = ToolName::from_str(name) else {
            return Err(anyhow!("unknown tool: {name}"));
        };
        match tool {
            ToolName::CurrentTime => Ok(json!({
                "iso8601": chrono::Utc::now().to_rfc3339(),
            })),
            ToolName::RetrieveMemory
            | ToolName::RememberThis
            | ToolName::SearchMedia => {
                let _ = args;
                Err(anyhow!(
                    "tool {name} not implemented yet — lives under retrieval::rag"
                ))
            }
        }
    }
}

fn retrieve_memory_def() -> ToolDef {
    ToolDef {
        name: "retrieve_memory".into(),
        description:
            "Retrieve chunks of previously ingested documents or transcripts relevant to a \
             free-form query. Use whenever the user asks about a specific fact, document, or \
             moment they've shared with you."
                .into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "A natural-language question or topic to search for."
                },
                "k": {
                    "type": "integer",
                    "description": "Number of chunks to return (default 6).",
                    "minimum": 1, "maximum": 20
                }
            },
            "required": ["query"]
        }),
    }
}

fn remember_this_def() -> ToolDef {
    ToolDef {
        name: "remember_this".into(),
        description:
            "Store a short piece of text to be retrievable later. Use sparingly — only when the \
             user explicitly asks you to remember something."
                .into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "content": { "type": "string" },
                "tags":    { "type": "array", "items": { "type": "string" } }
            },
            "required": ["content"]
        }),
    }
}

fn search_media_def() -> ToolDef {
    ToolDef {
        name: "search_media".into(),
        description: "Search the user's ingested photos and videos by natural-language query."
            .into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": { "type": "string" },
                "k":     { "type": "integer", "minimum": 1, "maximum": 20 }
            },
            "required": ["query"]
        }),
    }
}

fn current_time_def() -> ToolDef {
    ToolDef {
        name: "current_time".into(),
        description: "Return the current UTC time in ISO-8601 format.".into(),
        parameters: json!({ "type": "object", "properties": {} }),
    }
}
