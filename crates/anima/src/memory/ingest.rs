//! Ingestion pipelines for documents and media.
//!
//! Entry points:
//! - `ingest_file(path)` — dispatches by extension
//! - per-kind handlers: `ingest_text`, `ingest_image`, `ingest_video`
//!
//! All implementations deferred — this file currently only defines the
//! public interface so the debug server and task runner can reference it.

use std::path::Path;

use anyhow::{Result, bail};

#[derive(Debug, Clone)]
pub struct IngestReport {
    pub source: String,
    pub chunks_inserted: usize,
}

pub async fn ingest_file(path: &Path) -> Result<IngestReport> {
    let source = path.to_string_lossy().into_owned();
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match ext.as_str() {
        "txt" | "md" | "pdf" => bail!("text ingestion not implemented yet (source={source})"),
        "jpg" | "jpeg" | "png" | "webp" => {
            bail!("image ingestion not implemented yet (source={source})")
        }
        "mp4" | "mov" | "wav" | "mp3" | "m4a" => {
            bail!("media ingestion not implemented yet (source={source})")
        }
        other => bail!("unsupported extension {other:?} (source={source})"),
    }
}
