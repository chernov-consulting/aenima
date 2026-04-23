-- Initial schema for the anima daemon.
--
-- Everything durable lives in Postgres: conversations, messages, ingested
-- memory chunks (with pgvector embeddings), media assets, and the
-- tool-call audit log.

CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- One row per realtime voice session.
CREATE TABLE conversations (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    backend     TEXT NOT NULL,
    started_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at    TIMESTAMPTZ,
    meta        JSONB NOT NULL DEFAULT '{}'::jsonb
);

-- User / assistant turns. Audio blobs are stored on disk and referenced
-- by `audio_path`; the DB holds only the transcript and metadata.
CREATE TABLE messages (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    role            TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    text            TEXT NOT NULL DEFAULT '',
    audio_path      TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    meta            JSONB NOT NULL DEFAULT '{}'::jsonb
);
CREATE INDEX messages_conversation_idx ON messages(conversation_id, created_at);

-- Ingested memory chunks. Dimension 1536 matches OpenAI
-- text-embedding-3-small; on DGX Spark day we'll likely switch to 3072
-- (text-embedding-3-large) or a local BGE model — a migration will alter
-- the column type then. Tracked dimension lives in `meta.dims`.
CREATE TABLE memories (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source      TEXT NOT NULL,
    body        TEXT NOT NULL,
    embedding   vector(1536) NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    meta        JSONB NOT NULL DEFAULT '{}'::jsonb,
    tsv         tsvector GENERATED ALWAYS AS (to_tsvector('english', body)) STORED
);
CREATE INDEX memories_embedding_hnsw ON memories USING hnsw (embedding vector_cosine_ops);
CREATE INDEX memories_tsv_idx        ON memories USING gin(tsv);
CREATE INDEX memories_source_idx     ON memories(source);

-- Photos, videos, audio files surfaced to the model via `search_media`.
CREATE TABLE media_assets (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    path            TEXT NOT NULL UNIQUE,
    kind            TEXT NOT NULL CHECK (kind IN ('image', 'audio', 'video')),
    thumbnail_path  TEXT,
    caption         TEXT,
    embedding       vector(1536),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    meta            JSONB NOT NULL DEFAULT '{}'::jsonb
);
CREATE INDEX media_assets_kind_idx      ON media_assets(kind);
CREATE INDEX media_assets_embedding_idx ON media_assets USING hnsw (embedding vector_cosine_ops);

-- Audit log of tool invocations + results (useful for debugging and
-- future eval harnesses).
CREATE TABLE tool_calls (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    conversation_id UUID REFERENCES conversations(id) ON DELETE CASCADE,
    message_id      UUID REFERENCES messages(id) ON DELETE SET NULL,
    name            TEXT NOT NULL,
    args            JSONB NOT NULL,
    result          JSONB,
    error           TEXT,
    called_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at    TIMESTAMPTZ
);
CREATE INDEX tool_calls_conversation_idx ON tool_calls(conversation_id, called_at);
