//! Runtime configuration, sourced entirely from environment variables.
//!
//! Layered `.env.*` loading happens in `main.rs` before this module is touched,
//! so `std::env::var` is the single source of truth here.

use std::env;

use anima_core::BackendKind;
use anyhow::{Context, Result};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub backend: BackendKind,
    pub debug_bind: String,
    pub database_url: String,
    pub sample_rate: u32,
    pub input_device: Option<String>,
    pub output_device: Option<String>,
    pub openai: OpenaiConfig,
    pub gemini: GeminiConfig,
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenaiConfig {
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
    pub realtime_model: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeminiConfig {
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
    pub live_model: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let backend = env::var("ANIMA_BACKEND")
            .unwrap_or_else(|_| "openai".into())
            .parse::<BackendKind>()
            .context("parsing ANIMA_BACKEND")?;

        Ok(Self {
            backend,
            debug_bind: env::var("ANIMA_DEBUG_BIND").unwrap_or_else(|_| "127.0.0.1:7000".into()),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://anima:anima@127.0.0.1:5433/anima".into()),
            sample_rate: env::var("ANIMA_SAMPLE_RATE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(24_000),
            input_device: env::var("ANIMA_INPUT_DEVICE").ok().filter(|s| !s.is_empty()),
            output_device: env::var("ANIMA_OUTPUT_DEVICE").ok().filter(|s| !s.is_empty()),
            openai: OpenaiConfig {
                api_key: env::var("OPENAI_API_KEY").ok().filter(|s| !s.is_empty()),
                realtime_model: env::var("OPENAI_REALTIME_MODEL")
                    .unwrap_or_else(|_| "gpt-4o-realtime-preview".into()),
            },
            gemini: GeminiConfig {
                api_key: env::var("GEMINI_API_KEY").ok().filter(|s| !s.is_empty()),
                live_model: env::var("GEMINI_LIVE_MODEL")
                    .unwrap_or_else(|_| "gemini-2.0-flash-live".into()),
            },
        })
    }
}
