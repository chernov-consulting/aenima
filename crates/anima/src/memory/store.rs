//! Postgres + pgvector wrapper.
//!
//! Owns sqlx `PgPool` access for the `memories`, `media_assets`, and related
//! tables. All reads/writes against embeddings go through here.
//!
//! We use runtime-checked `query_as` / `query` rather than the compile-time
//! `query!` macro so `cargo check` does not require a live database. Once we
//! have a stable schema we can flip to `sqlx prepare` + offline mode.

use anyhow::Result;
use pgvector::Vector;
use serde::Serialize;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct MemoryStore {
    pool: PgPool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MemoryChunk {
    pub id: Uuid,
    pub source: String,
    pub body: String,
    pub score: f32,
}

impl MemoryStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retrieve the top-`k` chunks most similar to `query_embedding`.
    pub async fn similar(&self, query_embedding: Vec<f32>, k: i64) -> Result<Vec<MemoryChunk>> {
        let vec = Vector::from(query_embedding);
        let rows = sqlx::query(
            r#"
            SELECT id, source, body,
                   (1 - (embedding <=> $1))::float4 AS score
            FROM memories
            ORDER BY embedding <=> $1
            LIMIT $2
            "#,
        )
        .bind(vec)
        .bind(k)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| MemoryChunk {
                id: r.get("id"),
                source: r.get("source"),
                body: r.get("body"),
                score: r.try_get::<f32, _>("score").unwrap_or(0.0),
            })
            .collect())
    }

    /// Insert a new memory chunk.
    pub async fn insert(&self, source: &str, body: &str, embedding: Vec<f32>) -> Result<Uuid> {
        let vec = Vector::from(embedding);
        let row = sqlx::query(
            r#"
            INSERT INTO memories (source, body, embedding)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(source)
        .bind(body)
        .bind(vec)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.get("id"))
    }

    pub async fn count(&self) -> Result<i64> {
        let row = sqlx::query("SELECT COUNT(*) AS n FROM memories")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.try_get::<i64, _>("n").unwrap_or(0))
    }
}
