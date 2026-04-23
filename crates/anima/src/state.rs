//! Shared runtime state passed to every async task.
//!
//! Cheap to clone (`Arc` inside). Owns the `ConversationBackend` selected by
//! config, the database pool, and the retrieval tool registry.

use std::sync::Arc;

use anyhow::{Context, Result};
use sqlx::PgPool;
use tracing::info;

use crate::{
    config::Config,
    conversation::{self, ConversationBackend},
    memory::store::MemoryStore,
    retrieval::tools::ToolRegistry,
};

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    pub config: Config,
    pub db: PgPool,
    pub backend: Box<dyn ConversationBackend>,
    pub memory: MemoryStore,
    pub tools: ToolRegistry,
}

impl AppState {
    pub async fn init(config: Config) -> Result<Self> {
        let db = PgPool::connect(&config.database_url)
            .await
            .with_context(|| format!("connecting to {}", redact_db(&config.database_url)))?;

        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .context("applying sqlx migrations")?;

        info!("database connected and migrations applied");

        let memory = MemoryStore::new(db.clone());
        let backend = conversation::build_backend(&config).context("constructing backend")?;
        let tools = ToolRegistry::default();

        Ok(Self {
            inner: Arc::new(Inner {
                config,
                db,
                backend,
                memory,
                tools,
            }),
        })
    }

    pub fn config(&self) -> &Config {
        &self.inner.config
    }

    pub fn db(&self) -> &PgPool {
        &self.inner.db
    }

    pub fn backend(&self) -> &dyn ConversationBackend {
        &*self.inner.backend
    }

    pub fn memory(&self) -> &MemoryStore {
        &self.inner.memory
    }

    pub fn tools(&self) -> &ToolRegistry {
        &self.inner.tools
    }

    pub async fn shutdown(&self) {
        self.inner.db.close().await;
    }
}

fn redact_db(url: &str) -> String {
    // Naive password redact for log-safety.
    if let Some(start) = url.find("://") {
        let rest = &url[start + 3..];
        if let Some(at) = rest.find('@') {
            let creds = &rest[..at];
            if let Some(colon) = creds.find(':') {
                return format!("{}://{}:***@{}", &url[..start], &creds[..colon], &rest[at + 1..]);
            }
        }
    }
    url.to_owned()
}
