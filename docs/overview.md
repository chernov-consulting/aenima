# Overview

**Anima** is a real-time voice companion. The working product name is Anima;
the umbrella project is Codename The Ark (an AI-driven digital presence
service — see the project README for context).

## Goals of this repository

1. **Prototype a real-time multimodal conversation** running on a MacBook
   using hosted realtime APIs (OpenAI Realtime, Gemini Live).
2. **Build the ingestion and retrieval plumbing** that lets Anima ground its
   responses in the user's documents, photos, and transcripts.
3. **Stay architecturally identical to the DGX Spark bonsai device** so the
   only difference on the final hardware is a third `ConversationBackend`
   implementation plus a different audio device.

## Non-goals (for now)

- Multi-user / auth / hosting. This is a single-user local daemon.
- A graphical application or mobile client. The bonsai has no screen; the
  MacBook prototype doesn't pretend otherwise. A tiny HTTP debug dashboard
  exists only for development visibility.
- A microservice split. One daemon, one binary, one process.

## High-level shape

```
+-----------------------------------------------+
|  anima daemon (Rust, single tokio runtime)    |
|                                                |
|  audio::input (cpal) --> conversation::Session |
|                          |                     |
|                          v                     |
|                 ConversationBackend impl       |
|                          |                     |
|                          v                     |
|                    provider WS                 |
|                 (OpenAI / Gemini / Local)      |
|                          |                     |
|                          v                     |
|  audio::output (cpal) <-- text/audio deltas    |
|                          +--> tool_call        |
|                                |               |
|                                v               |
|                        retrieval::tools        |
|                                |               |
|                                v               |
|                        memory::store           |
|                       (Postgres+pgvector)      |
|                                                |
|  debug_server (axum) @ localhost:7000          |
+------------------------------------------------+
```

See [architecture.md](architecture.md) for the full design, including the
`ConversationBackend` contract and the DGX Spark-day swap.

## Glossary

| Term | Meaning |
|---|---|
| **Anima** | The product — a conversational AI companion. |
| **The Ark** | The parent project codename. |
| **Bonsai device** | The final hardware target: DGX Spark inside a bonsai pot. |
| **ConversationBackend** | Trait that abstracts hosted vs local realtime voice providers. |
| **Memory** | Ingested documents, photos, or transcripts the model can retrieve via tools. |
