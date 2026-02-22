# TEL (Transaction Event Log) -- keriox Reference

Crate: `teliox` (`support/teliox/`). Depends on `keri-core`. Two parallel event tracks: Management (registry) and VC (credential).

---

## Event Types

### Management Track (Registry TEL)

| Tag | Type | Struct | Purpose |
|-----|------|--------|---------|
| `vcp` | `ManagementTelType::Vcp` | `Inc` | Registry inception |
| `vrt` | `ManagementTelType::Vrt` | `Rot` | Registry rotation (backer changes) |

```rust
type ManagerTelEventMessage = TypedEvent<ManagementTelType, ManagerTelEvent>;

struct ManagerTelEvent { prefix: IdentifierPrefix, sn: u64, event_type: ManagerEventType }
enum ManagerEventType { Vcp(Inc), Vrt(Rot) }

struct Inc { issuer_id: IdentifierPrefix, config: Vec<Config>, backer_threshold: u64, backers: Vec<IdentifierPrefix> }
struct Rot { prev_event: SelfAddressingIdentifier, backers_to_add: Vec<IdentifierPrefix>, backers_to_remove: Vec<IdentifierPrefix> }
enum Config { NoBackers }
```

### VC Track (Credential TEL)

| Tag | Type | Struct | Backers | Purpose |
|-----|------|--------|---------|---------|
| `iss` | `TelEventType::Iss` | `SimpleIssuance` | No | Simple issuance |
| `rev` | `TelEventType::Rev` | `SimpleRevocation` | No | Simple revocation |
| `bis` | `TelEventType::Bis` | `Issuance` | Yes | Backed issuance |
| `brv` | `TelEventType::Brv` | `Revocation` | Yes | Backed revocation |

```rust
type VCEventMessage = TypedEvent<TelEventType, TimestampedVCEvent>;

struct VCEvent { prefix: IdentifierPrefix, sn: u64, event_type: VCEventType }
struct TimestampedVCEvent { data: VCEvent, timestamp: DateTime<FixedOffset> }

struct SimpleIssuance { registry_id: IdentifierPrefix }
struct SimpleRevocation { registry_id: IdentifierPrefix, prev_event_hash: SelfAddressingIdentifier }
struct Issuance { issuer_id: IdentifierPrefix, registry_anchor: EventSeal }       // private fields
struct Revocation { prev_event_hash: SelfAddressingIdentifier, registry_anchor: Option<EventSeal> }
```

### Top-Level Event Enum

```rust
enum Event { Management(ManagerTelEventMessage), Vc(VCEventMessage) }
```

Methods: `get_digest()`, `get_prefix()`, `get_sn()`, `get_registry_id()`, `serialize()`

### Verifiable Event (TEL event + KEL anchor proof)

```rust
struct VerifiableEvent { event: Event, seal: AttachedSourceSeal }
struct AttachedSourceSeal { seal: EventSourceSeal }
struct EventSourceSeal { sn: u64, digest: SelfAddressingIdentifier }
```

The seal points to the KEL interaction event that anchors this TEL event.

---

## State Types

### ManagerTelState (Registry)

```rust
struct ManagerTelState {
    prefix: IdentifierPrefix,     // registry identifier
    sn: u64,
    last: SelfAddressingIdentifier,
    issuer: IdentifierPrefix,     // issuer's KEL identifier
    backers: Option<Vec<IdentifierPrefix>>,  // None if NoBackers config
}
```

`apply(&self, event: &ManagerTelEventMessage) -> Result<Self, Error>`

### TelState (Credential)

```rust
enum TelState {
    NotIssued,                                    // default
    Issued(SelfAddressingIdentifier),             // holds last event digest
    Revoked,
}
```

`apply(&self, event: &VCEventMessage) -> Result<Self, Error>`

State machine: `NotIssued -> Issued(digest) -> Revoked` (one-way, terminal)

### Top-Level State Enum

```rust
enum State { Management(ManagerTelState), Tel(TelState) }
```

---

## Processing Pipeline

```
VerifiableEvent arrives
  |
  +-- TelEventProcessor::process(event)
        |
        +-- TelEventValidator::validate(event)
        |     |
        |     +-- Event::Management -> validate_management(event, seal)
        |     |     - check_kel_event: KEL event at seal.sn has matching digest
        |     |     - KEL ixn contains seal to TEL event digest
        |     |     - ManagerTelState::apply()
        |     |
        |     +-- Event::Vc -> validate_vc(vc_event, seal)
        |           - lookup registry_id from vc_event
        |           - get issuer_id from ManagerTelState
        |           - check_kel_event: same KEL anchor verification
        |           - TelState::apply()
        |
        +-- On Ok: TelEventStorage::add_event() + notify(TelEventAdded)
        +-- On Err: route to escrow (see below)
```

### TelEventValidator -- `TelEventValidator<D: TelEventDatabase, K: EventDatabase>`

- `validate(event)` -- Dispatch to management or vc validation
- `validate_management(event, seal)` -- KEL anchor check + state transition for registry
- `validate_vc(vc_event, seal)` -- Registry lookup + KEL anchor check + state transition
- `check_kel_event(kel_ref, seal, issuer_id, expected_digest)` -- Static: verify KEL anchor

---

## Escrow Routing

### Error -> Notification -> Escrow Decision Tree

| Error | Notification | Escrow | Re-check Trigger |
|-------|-------------|--------|-----------------|
| `OutOfOrderError` | `TelNotification::OutOfOrder` | `OutOfOrderEscrow` | `TelEventAdded` (predecessor TEL event) |
| `MissingIssuerEventError` | `TelNotification::MissingIssuer` | `MissingIssuerEscrow` | KEL `KeyEventAdded` (cross-bus bridge) |
| `MissingRegistryError` | `TelNotification::MissingRegistry` | `MissingRegistryEscrow` | `TelEventAdded` (registry event) |
| `EventAlreadySavedError` | _(none)_ | _(silently Ok)_ | Idempotent duplicate |
| All other errors | _(none)_ | _(propagated as Err)_ | Hard failure |

### Cross-Bus Bridge (critical pattern)

`MissingIssuerEscrow` implements both `TelNotifier` (TEL bus) and `keri_core::Notifier` (KEL bus). Register it on the KEL notification bus for `KeyEventAdded`:

```rust
keri_processor.register_observer(
    missing_issuer.clone(),
    &[JustNotification::KeyEventAdded],
)?;
```

When a KEL event arrives -> re-checks escrowed TEL events waiting for issuer's KEL state.

### Notification Types

```rust
enum TelNotification {
    MissingRegistry(VerifiableEvent),
    MissingIssuer(VerifiableEvent),
    OutOfOrder(VerifiableEvent),
    TelEventAdded(VerifiableEvent),
}

enum TelNotificationKind { MissingRegistry, MissingIssuer, OutOfOrder, TelEventAdded }
```

---

## API Surface

### Tel Facade -- `Tel<D: TelEventDatabase, K: EventDatabase>`

Fields: `processor: TelEventProcessor<D, K>`, `recently_added_events: Arc<RecentlyAddedEvents>`

- `new(tel_storage, kel_storage, publisher)` -- Constructor
- `make_inception_event(issuer, config, bt, backers) -> Result<Event>` -- Create registry vcp
- `make_rotation_event(id, ba, br) -> Result<Event>` -- Create registry vrt
- `make_issuance_event(registry_id, vc_digest) -> Result<Event>` -- Create iss/bis
- `make_revoke_event(registry_id, vc_hash) -> Result<Event>` -- Create rev/brv
- `parse_and_process_tel_stream(stream) -> Result<()>` -- Parse CESR + process
- `get_vc_state(vc_hash) -> Result<Option<TelState>>` -- Credential state
- `get_tel(vc_hash) -> Result<Vec<VerifiableEvent>>` -- Full credential TEL
- `get_management_tel(registry_id) -> Result<Option<Iterator>>` -- Registry events
- `get_management_tel_state(id) -> Result<Option<ManagerTelState>>` -- Registry state

### TelEventProcessor -- `TelEventProcessor<D: TelEventDatabase, K: EventDatabase>`

Fields: `tel_reference: Arc<TelEventStorage<D>>`, `publisher: TelNotificationBus`

- `new(kel_ref, tel_ref, publisher)` -- Constructor
- `process(event: VerifiableEvent) -> Result<()>` -- Validate + store + notify; routes errors to escrow
- `process_signed_query(qr) -> Result<TelReplyType>` -- Handle TEL queries
- `register_observer(observer, notifications)` -- Register escrow on notification bus

### TelEventStorage -- `TelEventStorage<D: TelEventDatabase>`

Field: `db: Arc<D>`

- `compute_management_tel_state(id) -> Result<Option<ManagerTelState>>` -- Replay management events
- `compute_vc_state(vc_id) -> Result<Option<TelState>>` -- Replay VC events
- `get_events(vc_id) -> Result<Vec<VerifiableEvent>>` -- All VC events
- `add_event(event) -> Result<()>` -- Store verifiable event
- `process_query(qry: &TelQueryRoute) -> Result<TelReplyType>` -- Query dispatch

### TelNotificationBus

- `new()`, `register_observer(escrow, notifications)`, `notify(notification)`
- `trait TelNotifier { fn notify(&self, notification: &TelNotification, bus: &TelNotificationBus) -> Result<(), Error>; }`

---

## Database Traits

**`TelEventDatabase`** -- primary storage trait. Methods: `new(path)`, `add_new_event(event, id)`, `get_events(id)`, `get_management_events(id)`. Implementations: `RedbTelDatabase` (cfg `storage-redb`), `DynamoTelDatabase` (keri-dynamodb).

**`EscrowDatabase`** (cfg storage-redb) -- `EscrowDatabase(Arc<redb::Database>)`. Separate file for TEL escrow storage, used by all three escrow types.

---

## TEL Lifecycle Workflow

Every TEL event follows the same 4-step pattern: **create -> anchor in KEL -> wrap with seal -> process**.

```rust
// Step 1: Create TEL event via Tel facade
let tel_event: Event = tel.make_inception_event(issuer, vec![Config::NoBackers], 0, vec![])?;
// Or: tel.make_issuance_event(&registry_id, vc_digest)?
// Or: tel.make_revoke_event(&registry_id, &vc_hash)?

// Step 2: Anchor in issuer's KEL (ixn with digest seal)
let seal = Seal::Digest(DigestSeal::new(tel_event.get_digest()?));
let ixn = /* build ixn with seal, sign, process in KEL */;

// Step 3: Wrap with source seal pointing to anchoring KEL event
let verifiable = VerifiableEvent::new(tel_event, AttachedSourceSeal::new(ixn_sn, ixn_digest));

// Step 4: Process through TEL
tel.processor.process(verifiable)?;
```

**Via keri-controller** (two-phase: generate -> sign -> finalize):
- `identifier.incept_registry()` / `finalize_incept_registry()`
- `identifier.issue(digest)` / `finalize_issue()`
- `identifier.revoke(&sai)` / `finalize_revoke()`

---

## TEL Setup with Escrows

```rust
// 1. Create databases
let tel_db = Arc::new(RedbTelDatabase::new("tel.redb")?);
let escrow_db = EscrowDatabase::new(Path::new("escrow.redb"))?;

// 2. Default escrow bus (100s timeout for all three escrow types)
let (tel_bus, missing_issuer, out_of_order, missing_registry) =
    default_escrow_bus(tel_db.clone(), kel_storage.clone(), escrow_db)?;

// 3. Cross-bus bridge: register MissingIssuerEscrow on KEL bus
keri_processor.register_observer(missing_issuer.clone(), &[JustNotification::KeyEventAdded])?;

// 4. Create Tel facade
let tel_storage = Arc::new(TelEventStorage::new(tel_db.clone()));
let tel = Tel::new(tel_storage, kel_storage, Some(tel_bus));
```

---

## Query Types

```rust
type TelQueryEvent = KeriEvent<Timestamped<TelQueryRoute>>;
type SignedTelQuery = SignedQuery<TelQueryEvent>;

enum TelQueryRoute { Tels { reply_route: String, args: TelQueryArgs } }
struct TelQueryArgs { i: Option<IdentifierPrefix>, ri: Option<IdentifierPrefix> }

enum TelReplyType { Tel(Vec<u8>) }
```

---

## Error Types (`teliox::Error`)

| Variant | Escrow? | Notes |
|---------|---------|-------|
| `KeriError(keri_core::Error)` | No | KEL-level error during validation |
| `OutOfOrderError` | Yes -> `OutOfOrderEscrow` | TEL sn mismatch |
| `MissingIssuerEventError` | Yes -> `MissingIssuerEscrow` | KEL event not found |
| `MissingRegistryError` | Yes -> `MissingRegistryEscrow` | Registry state not found |
| `EventAlreadySavedError` | Silently Ok | Idempotent duplicate |
| `MissingSealError` | No | KEL event missing TEL seal |
| `DigestsNotMatchError` | No | KEL seal digest mismatch |
| `UnknownIdentifierError` | No | No TEL state for id |
| `RedbError` | No | Storage failure (cfg storage-redb) |
| `Generic(String)` | No | Catch-all; DynamoDB conversion target |
| `EncodingError(String)` | No | CESR/CBOR serialization failure |
| `EscrowDatabaseError(String)` | No | Escrow storage failure |

Conversions: `keri_core::Error -> teliox::Error::KeriError` (via `#[from]`); `DynamoDbError -> teliox::Error::Generic` (via `Into`)

## JSON Field Mapping

| Field | Key | Field | Key |
|-------|-----|-------|-----|
| `issuer_id` | `"ii"` | `registry_id` | `"ri"` |
| `registry_anchor` | `"ra"` | `backer_threshold` | `"bt"` |
| `backers` | `"b"` | `backers_to_add` | `"ba"` |
| `backers_to_remove` | `"br"` | `prev_event(_hash)` | `"p"` |
| `config` | `"c"` | `timestamp` | `"dt"` |
