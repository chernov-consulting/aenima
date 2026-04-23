//! Embedding backend trait + OpenAI implementation stub.

use anyhow::{Result, bail};
use async_trait::async_trait;

/// Abstracts text/image embedding so we can swap OpenAI for a local BGE /
/// CLIP model on DGX Spark day.
#[async_trait]
pub trait EmbeddingBackend: Send + Sync {
    /// Dimensionality of the embedding vector. Must match the pgvector
    /// column type (`vector(N)`) configured in migrations.
    fn dimensions(&self) -> usize;

    async fn embed_text(&self, text: &str) -> Result<Vec<f32>>;

    async fn embed_image(&self, _bytes: &[u8]) -> Result<Vec<f32>> {
        bail!("image embedding not implemented for this backend")
    }
}

/// Placeholder. Real implementation will call
/// `POST https://api.openai.com/v1/embeddings` with `text-embedding-3-large`.
pub struct OpenAIEmbeddingBackend {
    #[allow(dead_code)]
    api_key: String,
}

impl OpenAIEmbeddingBackend {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl EmbeddingBackend for OpenAIEmbeddingBackend {
    fn dimensions(&self) -> usize {
        3072 // text-embedding-3-large
    }

    async fn embed_text(&self, _text: &str) -> Result<Vec<f32>> {
        bail!("OpenAI embeddings not implemented yet — scaffold only")
    }
}
