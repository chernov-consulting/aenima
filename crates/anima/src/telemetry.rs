//! Tracing / logging bootstrap.
//!
//! Writes human-readable logs to stdout; the debug dashboard subscribes to an
//! in-memory ring buffer for its `/log-stream` SSE endpoint (see `debug_server`).

use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub fn init() -> Result<()> {
    let filter = EnvFilter::try_from_env("RUST_LOG")
        .unwrap_or_else(|_| EnvFilter::new("info,anima=debug,sqlx=warn"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true).with_thread_ids(false))
        .try_init()
        .map_err(|e| anyhow::anyhow!("tracing init: {e}"))?;

    Ok(())
}
