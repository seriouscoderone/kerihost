---
name: keriox
description: >
  Rust KERI protocol implementation (keriox). Auto-activates when working with
  keriox imports, keriox_core types, keriox_sdk Controller/Identifier, KERI event
  processing in Rust, redb database operations, witness/watcher components, or
  Rust TEL (teliox) processing. THCLab fork (EUPL-1.2).
---

# keriox — Rust KERI Protocol Implementation

## Overview

keriox is the Rust implementation of KERI (Key Event Receipt Infrastructure), forked from
Human Colossus Foundation (THCLab). It provides a complete KERI protocol stack: event
creation, signing, processing, verification, escrow management, database persistence,
witness/watcher infrastructure, TEL (Transaction Event Log) for credential issuance, and
a high-level SDK with two-phase signing (keys never touch the library).

The workspace contains 8 crates: `keri-core` (foundation, ~18K lines), `teliox` (TEL),
`keri-controller` (identifier management), `keri-sdk` (simplified wrapper),
`witness`/`watcher` (actix-web infrastructure nodes), `keri-dynamodb` (serverless backend),
and `gossip` (protocol). License: EUPL-1.2.

**Key design principles:**
- **Escrow as state, not error.** Out-of-order/partially-signed events go to escrow and
  are promoted when prerequisites arrive via the notification bus.
- **Two-phase signing.** SDK generates events, returns raw bytes for external signing,
  then finalizes with provided signatures. Private keys never enter the library.
- **Notification-driven processing.** `NotificationBus` dispatches events to registered
  `Notifier` observers (escrow handlers, log watchers) enabling extensible pipelines.

## Quick Reference

| Item | Crate | Purpose |
|------|-------|---------|
| `EventMsgBuilder` | keri-core | Builder for icp/rot/ixn/dip/drt events |
| `BasicProcessor` | keri-core | Event processing + escrow routing via NotificationBus |
| `IdentifierState` | keri-core | Key state computed from KEL (prefix, keys, witnesses, thresholds) |
| `KeyConfig` | keri-core | Public keys + next key commitment + threshold |
| `SignedEventMessage` | keri-core | Event + signatures + witness receipts + delegator seal |
| `KeriEvent<KeyEvent>` | keri-core | Typed, self-addressing event with SAID |
| `EventValidator` | keri-core | Signature/state validation |
| `NotificationBus` | keri-core | Observer dispatch for escrow promotion |
| `EventDatabase` trait | keri-core | KEL/receipt/state storage abstraction |
| `EscrowDatabase` trait | keri-core | Escrow storage abstraction |
| `Tel` | teliox | TEL facade: process VC/management events, query state |
| `Controller` | keri-sdk | High-level: create identifiers, rotate, interact |
| `Identifier` | keri-controller | Single identifier lifecycle (incept, rotate, query, finalize) |
| `WitnessListener` | witness | actix-web witness HTTP server |
| `WatcherListener` | watcher | actix-web watcher HTTP server |
| `CryptoBox` | keri-core | Ed25519 key pair with rotate() |
| `Seal` | keri-core | Digest/Event/Location/Root seal for anchoring |
| `Receipt` / `ReceiptBuilder` | keri-core | Witness/validator receipt creation |
| `SignatureThreshold` | keri-core | Simple(u64) or Weighted threshold |
| `RedbEventDatabase` | keri-core | Default redb storage backend |

## Import Guide

```toml
# Cargo.toml — typical dependency configurations

# Core protocol (default features include redb storage)
keri-core = { path = "../keriox/keriox_core" }

# Core with OOBI + query + mailbox
keri-core = { path = "../keriox/keriox_core", features = ["oobi-manager", "mailbox"] }

# TEL support
teliox = { path = "../keriox/support/teliox" }

# High-level SDK (pulls in keri-controller)
keri-sdk = { path = "../keriox/keriox_sdk" }

# Infrastructure nodes
witness = { path = "../keriox/components/witness" }
watcher = { path = "../keriox/components/watcher" }

# Serverless backend
keri-dynamodb = { path = "../keriox/support/dynamodb" }
```

**Feature flags (keri-core):**

| Flag | Enables | Implies |
|------|---------|---------|
| `storage-redb` (default) | ReDB database implementation | — |
| `query` | Query module + CBOR | — |
| `oobi` | OOBI resolution | — |
| `oobi-manager` | Full OOBI management | oobi + query + storage-redb + reqwest |
| `mailbox` | Mailbox module | query + storage-redb |

## Reference Files

| File | Contents | Size |
|------|----------|------|
| [references/api-core.md](references/api-core.md) | keri-core API: events, processor, database, primitives, actor, query | 15KB |
| [references/api-ext.md](references/api-ext.md) | Extension crate APIs: teliox, sdk, controller, witness, watcher, dynamodb, gossip | 15KB |
| [references/types.md](references/types.md) | All structs with serde renames, enums, type aliases, trait signatures | 15KB |
| [references/patterns.md](references/patterns.md) | 10 workflow sections, two-phase signing, escrow promotion, anti-patterns | 16KB |
| [references/errors.md](references/errors.md) | Error variants, escrow routing table, cross-crate conversions | 13KB |
| [references/storage.md](references/storage.md) | Database traits, ReDB/DynamoDB schemas, feature flags, config | 12KB |
| [references/tel.md](references/tel.md) | TEL event types, state machine, processing, escrow routing | 12KB |

## Usage Patterns

### Inception (builder)

```rust
let icp = EventMsgBuilder::new(EventTypeTag::Icp)
    .with_keys(keys).with_next_keys(next_keys)
    .with_threshold(&SignatureThreshold::Simple(2))
    .with_next_threshold(&SignatureThreshold::Simple(2))
    .with_witness_list(&witnesses)
    .with_witness_threshold(&SignatureThreshold::Simple(1))
    .build()?;
// sign externally, then: icp.sign(sigs, witness_receipts, delegator_seal)
```

### Processing pipeline

```
Processor::process(msg)
  Notice::Event -> validate
    Ok -> add_kel_finalized_event + notify(KeyEventAdded)
    OutOfOrder -> notify(OutOfOrder)        -> MaybeOutOfOrderEscrow
    NotEnoughSig -> notify(PartiallySigned) -> PartiallySignedEscrow
    NotEnoughRct -> notify(PartiallyWitnessed) -> PartiallyWitnessedEscrow
    MissingDeleg -> notify(MissingDelegatingEvent) -> DelegationEscrow
    Duplicate -> notify(DupliciousEvent)
```

### Two-phase signing (SDK)

```rust
let controller = Controller::new(ControllerConfig { db_path, .. })?;
let pk = PublicKey::new(pub_key_bytes);
let (inception_msg, _) = controller.incept(
    vec![pk], SignatureThreshold::Simple(1),
    next_keys, SignatureThreshold::Simple(1),
    witnesses, wit_threshold)?;

// Phase 1: get bytes to sign
let to_sign = inception_msg.encode()?;
// Phase 2: sign externally, finalize
let signature = /* external signing */;
let identifier = controller.finalize_incept(to_sign, &[signature])?;
```

### Database setup (ReDB)

```rust
let db = Arc::new(RedbEventDatabase::new(Path::new("./db"))?);
let bus = NotificationBus::new();
// Register escrow handlers
bus.register_observer(Arc::new(MaybeOutOfOrderEscrow::new(..)), &[JustNotification::OutOfOrder]);
bus.register_observer(Arc::new(PartiallySignedEscrow::new(..)), &[JustNotification::PartiallySigned]);
let processor = BasicProcessor::new(db.clone(), Some(bus));
```

## Anti-Patterns

| Mistake | Fix |
|---------|-----|
| Treating escrow errors as failures | Check for `EventOutOfOrderError`, `NotEnoughSigsError` etc. — these go to escrow, not rejection |
| Building `KeyEvent` struct directly | Use `EventMsgBuilder` — it handles self-addressing derivation and serialization format |
| Forgetting `notify_witnesses()` after SDK operations | Always call `identifier.notify_witnesses().await?` after incept/rotate/interact |
| Passing private keys to keri-sdk | SDK uses two-phase: generate event -> sign externally -> finalize. Keys stay external |
| Skipping `previous_event` on rot/ixn | Rotation and interaction require `.with_previous_event(&prev_digest)` — the builder doesn't auto-fetch |
