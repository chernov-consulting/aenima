//! Entry point for the anima daemon.
//!
//! Loads layered `.env.*` files, constructs the `AppState`, spawns the debug
//! HTTP server, initialises the audio I/O streams, and idles until Ctrl-C.
//! The actual realtime voice loop is not yet wired — this is the structural
//! scaffold.

use anyhow::{Context, Result};
use std::net::SocketAddr;
use tokio::signal;
use tracing::{info, warn};

mod audio;
mod config;
mod conversation;
mod debug_server;
mod memory;
mod retrieval;
mod state;
mod telemetry;

use crate::config::Config;
use crate::state::AppState;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    load_dotenv();
    telemetry::init()?;

    let config = Config::from_env().context("loading config from env")?;
    info!(backend = %config.backend, "starting anima daemon");

    let state = AppState::init(config.clone())
        .await
        .context("initialising AppState (db, backends, audio)")?;

    // Debug server — localhost-only HTTP dashboard for dev visibility.
    let debug_addr: SocketAddr = config
        .debug_bind
        .parse()
        .with_context(|| format!("parsing ANIMA_DEBUG_BIND={:?}", config.debug_bind))?;
    let debug_handle = {
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(err) = debug_server::serve(state, debug_addr).await {
                warn!(%err, "debug server exited with error");
            }
        })
    };

    info!(%debug_addr, "debug dashboard: http://{debug_addr}");
    info!("press Ctrl-C to shut down");
    signal::ctrl_c().await?;
    info!("shutdown signal received");

    debug_handle.abort();
    state.shutdown().await;
    Ok(())
}

/// Load `.env.base`, `.env.devkeys`, and `.env.personal` in that order (last
/// wins). Any file that doesn't exist is silently skipped.
fn load_dotenv() {
    for name in [".env.base", ".env.devkeys", ".env.personal"] {
        let _ = dotenvy::from_filename_override(name);
    }
}
