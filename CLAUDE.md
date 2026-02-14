# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

KERI.host is a serverless KERI (Key Event Receipt Infrastructure) witness service on AWS Lambda + DynamoDB. It is a forkable, self-hostable reference implementation — not a platform or identity provider. It includes a Jekyll blog on GitHub Pages at `blog.keri.host`.

## Architecture

Three-layer CDK stack design where data survives infrastructure updates:

```
DataStack (Layer 1)     → DynamoDB tables (KEL, States, Receipts, Escrows), Secrets Manager
ApiStack (Layer 2)      → API Gateway, ACM cert, Route53 (api.keri.host, blog.keri.host)
WitnessStack (Layer 3)  → 4 Rust Lambdas, API routes, EventBridge schedule
```

### Rust Workspace

```
crates/kerihost-core/      — KERI protocol: events, key state, validation, receipts
crates/kerihost-db/        — Database trait + DynamoDB/in-memory implementations
crates/kerihost-witness/   — Witness business logic (process, receipt, escrow, OOBI)
lambdas/witness-process/   — POST /witness/process (event ingestion)
lambdas/witness-query/     — POST /witness/query (state/kel/receipt queries)
lambdas/witness-oobi/      — GET /witness, /witness/oobi/{id} (OOBI resolution)
lambdas/witness-escrow-check/ — Scheduled (5min) escrow promotion/expiration
```

**Dependency flow:** `kerihost-core` ← `kerihost-db` ← `kerihost-witness` ← `lambdas/*`

### Key Design Decisions

- **KERI-Honest:** Never claims finality. Uses `ConfidenceLevel` (LocalOnly, ReceiptThresholdMet). All responses include `asOf` timestamp.
- **Escrow as state, not error:** Out-of-order events return HTTP 202 (Accepted), not 4xx. Scheduled Lambda promotes or expires escrowed events.
- **Trait-based DB:** `WitnessDatabase` trait with DynamoDB (production) and in-memory (test) implementations.
- **DynamoDB schema:** KEL table: `PK=aid, SK=sn` (zero-padded). States: `PK=aid`. Receipts: `PK=event_digest, SK=witness_aid`. Escrows: `PK=aid, SK=reason_digest` with TTL and GSI.

### Key Dependencies

- `cesride` 0.6 — CESR encoding and cryptographic primitives
- `parside` 0.2 — KERI message parsing
- `cargo-lambda-cdk` — Builds Rust Lambdas for CDK deployment

## Common Commands

### Build

```bash
cargo build --release              # Build all Rust crates and lambdas
cargo test                         # Run all Rust unit tests
```

### Infrastructure (CDK)

```bash
cd infrastructure
pnpm install
pnpm run build                     # Compile TypeScript CDK
pnpm cdk diff                      # Preview changes
pnpm cdk deploy --all -c hostedZoneId=Z0070723WLKQKTOACN5H
pnpm cdk deploy KerihostApiStack -c hostedZoneId=Z0070723WLKQKTOACN5H
```

AWS profile: `personal`

### Integration Tests

```bash
cd tests/integration
pnpm install
WITNESS_API_URL=https://api.keri.host/prod pnpm test
```

Tests use Vitest, cover: OOBI resolution, event processing (inception/interaction), duplicate detection, escrow (HTTP 202), state/kel/receipt queries, KERI-honest response format.

### Blog

Blog uses Jekyll + Minimal Mistakes ("air" skin) on GitHub Pages. Source: `docs/`. Live at `https://blog.keri.host`.

```bash
# Posts go in docs/_posts/YYYY-MM-DD-slug-name.md
# No local Ruby needed — GitHub Pages builds server-side
# Use /blog skill for writing posts in the KERI.host voice
```

Config: `docs/_config.yml`. Comments: Giscus (GitHub Discussions). Navigation: `docs/_data/navigation.yml`.

## API Endpoints

| Endpoint | Method | Lambda | Purpose |
|----------|--------|--------|---------|
| `/witness` | GET | oobi | Witness introduction |
| `/witness/introduce` | GET | oobi | Witness OOBI |
| `/witness/process` | POST | process | Submit KERI events |
| `/witness/query` | POST | query | Query state, KEL, receipts |
| `/witness/oobi/{id}` | GET | oobi | Resolve OOBI for identifier |

## Skills

- **`/blog`** (`.claude/skills/keri-blog/SKILL.md`) — Write blog posts in the KERI.host voice. Commands: `/blog new [topic]`, `/blog edit [name]`, `/blog review [name]`, `/blog list`. Posts must be conversational, anti-hype, grounded in real architecture. Never use crypto buzzwords without human context.
