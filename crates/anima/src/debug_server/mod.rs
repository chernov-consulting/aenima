//! Built-in debug dashboard served on `ANIMA_DEBUG_BIND` (default
//! `127.0.0.1:7000`). A single static HTML page plus a few JSON endpoints —
//! no framework, no build step.
//!
//! Endpoints:
//! - `GET  /`            -> embedded dashboard HTML
//! - `GET  /state`       -> JSON snapshot of current daemon state
//! - `GET  /devices`     -> audio device enumeration (cpal)
//! - `GET  /memory/count`-> number of memory chunks in pgvector
//! - `POST /ingest`      -> multipart file upload; forwards to `memory::ingest`
//! - `GET  /log-stream`  -> (TODO) SSE tail of the tracing subscriber

use std::net::SocketAddr;

use anyhow::{Context, Result};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde_json::json;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{audio, state::AppState};

const DASHBOARD_HTML: &str = include_str!("static/dashboard.html");

pub async fn serve(state: AppState, addr: SocketAddr) -> Result<()> {
    let app = Router::new()
        .route("/", get(|| async { Html(DASHBOARD_HTML) }))
        .route("/state", get(state_handler))
        .route("/devices", get(devices_handler))
        .route("/memory/count", get(memory_count_handler))
        .route("/ingest", post(ingest_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("binding debug server to {addr}"))?;
    info!(%addr, "debug server listening");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn state_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "backend": state.backend().label(),
        "backend_kind": state.config().backend.to_string(),
        "tool_count": state.tools().defs().len(),
        "started_at": chrono::Utc::now().to_rfc3339(),
    }))
}

async fn devices_handler() -> impl IntoResponse {
    let input = audio::input::list_devices().unwrap_or_default();
    let output = audio::output::list_devices().unwrap_or_default();
    Json(json!({ "input": input, "output": output }))
}

async fn memory_count_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.memory().count().await {
        Ok(n) => Json(json!({ "count": n })).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn ingest_handler() -> impl IntoResponse {
    // Multipart upload + crate::memory::ingest::ingest_file lands once
    // ingestion pipelines exist. Return 501 for now so the UI can show
    // "pending".
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({ "error": "ingest not implemented yet" })),
    )
}
