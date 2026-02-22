# keriox Storage & Configuration Reference

---

## Database Traits (keri-core)

### EventDatabase
Associated: `type Error; type LogDatabaseType: LogDatabase<'static>;`

- `get_log_db() -> Arc<LogDatabaseType>`
- `add_kel_finalized_event(SignedEventMessage, &IdentifierPrefix)`
- `add_receipt_t(SignedTransferableReceipt, &IdentifierPrefix)`
- `add_receipt_nt(SignedNontransferableReceipt, &IdentifierPrefix)`
- `get_key_state(&IdentifierPrefix) -> Option<IdentifierState>`
- `get_kel_finalized_events(QueryParameters) -> Option<impl DoubleEndedIterator<Item=TimestampedSignedEventMessage>>`
- `get_receipts_t(QueryParameters) -> Option<..Transferable>`
- `get_receipts_nt(QueryParameters) -> Option<..SignedNontransferableReceipt>`
- `accept_to_kel(&KeriEvent<KeyEvent>)`
- `save_reply(SignedReply)` -- `#[cfg(feature = "query")]`
- `get_reply(&IdentifierPrefix, &IdentifierPrefix) -> Option<SignedReply>` -- `#[cfg(feature = "query")]`

### LogDatabase<'db>
Associated: `type DatabaseType; type Error; type TransactionType;`

- `new(Arc<DatabaseType>)`, `log_event(&Txn, &SignedEventMessage)`, `log_event_with_new_transaction(&SignedEventMessage)`
- `log_receipt(&Txn, &SignedNontransferableReceipt)`, `log_receipt_with_new_transaction(&SignedNontransferableReceipt)`
- `get_signed_event(&SAID) -> Option<TimestampedSignedEventMessage>`, `get_event(&SAID) -> Option<KeriEvent<KeyEvent>>`
- `get_signatures(&SAID) -> Option<..IndexedSignature>`, `get_nontrans_couplets(&SAID) -> Option<..Nontransferable>`
- `get_trans_receipts(&SAID) -> ..Transferable`, `remove_nontrans_receipt(&Txn, &SAID, impl IntoIterator<Nontransferable>)`

### SequencedEventDatabase
Associated: `type DatabaseType; type Error; type DigestIter: Iterator<Item=SAID>;`

- `new(Arc<DatabaseType>, &'static str)` -- table_name param
- `insert(&IdentifierPrefix, u64, &SAID)`, `get(&IdentifierPrefix, u64) -> DigestIter`
- `get_greater_than(&IdentifierPrefix, u64) -> DigestIter`, `remove(&IdentifierPrefix, u64, &SAID)`

### EscrowDatabase
- `save_digest(&IdentifierPrefix, u64, &SAID)`, `insert(&SignedEventMessage)`, `insert_key_value(&IdentifierPrefix, u64, &SignedEventMessage)`
- `get(&IdentifierPrefix, u64) -> EventIter`, `get_from_sn(&IdentifierPrefix, u64) -> EventIter`
- `remove(&KeriEvent<KeyEvent>)`, `contains(&IdentifierPrefix, u64, &SAID) -> bool`

### TelEventDatabase (teliox)
- `new(impl AsRef<Path>)`, `add_new_event(VerifiableEvent, &IdentifierPrefix)`
- `get_events(&IdentifierPrefix) -> Option<..VerifiableEvent>` (VC TEL)
- `get_management_events(&IdentifierPrefix) -> Option<..VerifiableEvent>` (Mgmt TEL)

### Backends

| Backend | Error | LogDB | EscrowDB | Txn |
|---------|-------|-------|----------|-----|
| `MemoryDatabase` | `keri_core::Error` | `MemoryLogDatabase` | `MemoryEscrowDb` | `()` |
| `RedbDatabase` | `RedbError` | `loging::LogDatabase` | `SnKeyEscrow` | WriteTransaction |
| `DynamoDatabase` | `keri_core::Error` | `DynamoLogDatabase` | `DynamoEscrowDb` | `()` |

`QueryParameters`: `BySn{id, sn}`, `Range{id, start, limit}`, `All{&id}`

---

## ReDB Table Schemas (feature: `storage-redb`)

### KEL (keri-core)

| Table | Key | Value | Kind |
|-------|-----|-------|------|
| `kels` | `(aid:&str, sn:u64)` | rkyv(SAID) | Table |
| `key_states` | `aid:&str` | rkyv(IdentifierState) | Table |
| `events` | rkyv(SAID) | rkyv(KeriEvent) | Table |
| `signatures` | rkyv(SAID) | rkyv(IndexedSignature) | Multimap |
| `nontrans_receipts` | rkyv(SAID) | rkyv(Nontransferable) | Multimap |
| `trans_receipts` | rkyv(SAID) | rkyv(Transferable) | Multimap |
| `seals` | rkyv(SAID) | rkyv(SourceSeal) | Table |
| `ksns` | rkyv(SAID) | cbor(SignedReply) | Table (+query) |
| `accepted` | `(about:&str, from:&str)` | SAID string | Table (+query) |
| `timestamps_escrow` | rkyv(SAID) | u64 unix ts | Table |
| *(dynamic escrow)* | `(aid:&str, sn:u64)` | rkyv(SAID) | Multimap |

### Mailbox (keri-core, +mailbox)

| Table | Key | Value | Kind |
|-------|-----|-------|------|
| `mailbox_log` | SAID bytes | cbor(SignedEventMessage) | Table |
| `timestamps_mailbox` | `(aid, index:u64)` | u64 unix ts | Table |
| `indexes` | `(aid, table_name)` | u64 next index | Table |
| *(dynamic topic)* | `(aid, index:u64)` | cbor(D) | Table |

Topics: `"mbxrct"`, `"mbxrpy"`, `"mbxm"`, `"mbxd"`

### OOBI (keri-core, +oobi-manager)

| Table | Key | Value | Kind |
|-------|-----|-------|------|
| `location` | `(eid:&str, scheme:&str)` | cbor(SignedReply) | Table |
| `end_role` | `(cid:&[u8], role:&[u8])` | cbor(SignedReply) | Multimap |

### TEL (teliox, +storage-redb)

| Table | Key | Value | Kind |
|-------|-----|-------|------|
| `events` | event_digest | cbor(VerifiableEvent) | Table |
| `kels` (VC) | `(aid, sn:u64)` | event_digest | Table |
| `kels` (Mgmt) | `(aid, sn:u64)` | event_digest | Table |

---

## DynamoDB Table Schemas

Names: `{config.prefix}{suffix}`. SNs: 16-digit zero-padded (`format!("{:016}", sn)`).

| Suffix | PK | SK | Value Attrs |
|--------|----|----|-------------|
| `kel` | `aid`(S) | `sn`(S,zpad) | `digest`(S), `event_cesr`(B), `timestamp`(S) |
| `states` | `aid`(S) | -- | `state_json`(S), `sn`(N) |
| `events` | `digest`(S) | -- | `event_cesr`(B), `timestamp`(S), `nontrans_couplets`(L/B) |
| `receipts-t` | `digest`(S) | `receipt_id`(S:`{pfx}#{sn}`) | `receipt`(B,CESR) |
| `receipts-nt` | `digest`(S) | `witness`(S) | `couplet`(B,CESR) |
| `escrows` | `escrow_key`(S:`{name}#{aid}`) | `sn_digest`(S:`{sn}#{dig}`) | `escrow_name`(S) |
| `replies` | `aid`(S) | `signer`(S) | `reply_cesr`(B) (+query) |
| `tel-events` | `digest`(S) | -- | `event_cbor`(B,CBOR) |
| `tel-mgmt` | `aid`(S) | `sn`(S,zpad) | `digest`(S) |
| `tel-vc` | `aid`(S) | `sn`(S,zpad) | `digest`(S) |

Conditional writes: KEL `attribute_not_exists(aid) AND attribute_not_exists(sn)`; States `attribute_not_exists(aid) OR sn < :new_sn`

---

## Feature Flags

| Feature | Crate | Enables |
|---------|-------|---------|
| `query` | keri-core | `Rpy` tag, `SignedReply`, `SignedKelQuery`, `QueryEvent`, `KeyStateNotice`, `ReplyEscrow`, `bada_logic()`, KSN storage |
| `oobi` | keri-core | `Oobi`, `LocationScheme`, `EndRole`, `Role`, `Scheme`, `ReplyRoute::LocScheme/EndRole*` |
| `oobi-manager` | keri-core | `OobiManager`, `OobiStorage`, `Transport` trait. Requires `storage-redb` |
| `mailbox` | keri-core | `Exchange`, `SignedExchange`, `MailboxResponse`, `MailboxRoute`, `MailboxQuery`, `ForwardTopic` |
| `storage-redb` | keri-core | `RedbDatabase`, `LogDatabase`, `SnKeyDatabase`, `SnKeyEscrow`, `ReplyEscrow` |
| `mailbox`+`oobi-manager` | keri-core | `SimpleController` module |
| `storage-redb` (default) | teliox | `RedbTelDatabase`, `DigestKeyDatabase`, `TelLogDatabase`, escrow module (3 types) |
| `query_cache` | keri-controller | `IdentifierCache` (SQLite). Without: starts from sn=0 |
| `query` | keri-dynamodb | `replies` table interactions |
| `sqs` | keri-dynamodb | `SqsDispatch`, notification serde |

Propagation: keri-sdk -> `query`+`oobi`; keri-controller -> `oobi-manager`+`mailbox`

Always-on modules: `actor`, `database`, `error`, `event`, `event_message`, `keys`, `prefix`, `processor`, `signer`, `state`

---

## Serialization Formats

| Context | Format | Notes |
|---------|--------|-------|
| Event wire format | JSON | default; also MGPK, CBOR. Version string: `KERI10JSON{hex_size}_` |
| CESR prefixes | Qualified base64 | via `CesrPrimitive::to_str()` / `FromStr` |
| KERI JSON | Custom serializer | `-` prefix unquoted; `sn` hex; empty key = value only |
| ReDB events/state | rkyv | zero-copy; remote wrappers: `SAIDef`, `SerializationInfoDef`, `HashFunctionDef` |
| ReDB KSN/mailbox/OOBI | serde_cbor | |
| DynamoDB state | serde_json | `state_json` attr |
| DynamoDB TEL | serde_cbor | `event_cbor` attr |
| DynamoDB SQS | serde_json | `SqsNotificationMessage` |
| Gossip | bincode | UDP, 2048B buffer |

---

## Escrow Configuration

All EscrowConfig timeouts default 60s. Note: duration accepted but currently unused (`_` prefixed).

| Escrow | Default | Config Field |
|--------|---------|--------------|
| Out of order | 60s | `out_of_order_timeout` |
| Partially signed | 60s | `partially_signed_timeout` |
| Partially witnessed | 60s | `partially_witnessed_timeout` |
| Transferable receipt | 60s | `trans_receipt_timeout` |
| Delegation | 60s | `delegation_timeout` |
| TEL (teliox) | 100s | Hardcoded in `default_escrow_bus` |

WitnessEscrowConfig: subset (partially_signed, out_of_order, delegation). Supports `default_timeout` YAML fallback.

### KEL Escrow Wiring

| Escrow | Listens | Publishes | DB Name |
|--------|---------|-----------|---------|
| `MaybeOutOfOrderEscrow` | `OutOfOrder`, `KeyEventAdded` | `KeyEventAdded` | `out_of_order_escrow` |
| `PartiallySignedEscrow` | `PartiallySigned` | `KeyEventAdded`, `PartiallyWitnessed`, `MissingDelegatingEvent` | `partially_signed_escrow` |
| `PartiallyWitnessedEscrow` | `PartiallyWitnessed`, `ReceiptOutOfOrder` | `KeyEventAdded` | `partially_witnessed_escrow` |
| `DelegationEscrow` | `MissingDelegatingEvent`, `KeyEventAdded` | `KeyEventAdded` | `delegation_escrow` |
| `DuplicitousEvents` | `DupliciousEvent` | None | `duplicitous_escrow` |
| `ReplyEscrow` | `KsnOutOfOrder`, `KeyEventAdded` | None | `reply_escrow` (+redb+query) |

### TEL Escrow Wiring

| Escrow | Listens (TEL bus) | Cross-Bus |
|--------|-------------------|-----------|
| `OutOfOrderEscrow` | `OutOfOrder`, `TelEventAdded` | -- |
| `MissingRegistryEscrow` | `MissingRegistry`, `TelEventAdded` | -- |
| `MissingIssuerEscrow` | `MissingIssuer` | +`KeyEventAdded` on KEL bus |

### Promotion Chain

```
NotEnoughSigs       -> PartiallySignedEscrow   -> PartiallyWitnessedEscrow or KEL
NotEnoughReceipts   -> PartiallyWitnessedEscrow -> KEL
OutOfOrder          -> MaybeOutOfOrderEscrow    -> re-validate -> KEL
MissingDelegation   -> DelegationEscrow         -> KEL
Duplicate           -> DuplicitousEvents (storage only)
Ok                  -> KEL + notify(KeyEventAdded)
```

SQS dispatch (`SqsNotificationMessage`): `notification_type`, `payload_cesr_b64`, `aid`, `sn`, `timestamp`. All 11 notification types.

---

## Service Configuration

### CLI Args

**Witness** (clap): `-c`/`--config-file` (default `./witness.yml`), `-d`/`--db-path` (`WITNESS_DB_PATH`), `-u`/`--public-url` (`WITNESS_PUBLIC_URL`), `-p`/`--http-port` (`WITNESS_HTTP_PORT`), `-s`/`--seed` (`WITNESS_SEED`, null=random)

**Watcher** (clap): same pattern with `WATCHER_*` prefix, plus `-t`/`--tel-storage-path` (`WATCHER_TEL_STORAGE_PATH`)

**Precedence** (Figment): YAML -> env vars -> CLI args (highest)

### YAML Defaults

| Field | Witness | Watcher |
|-------|---------|---------|
| `db_path` | `./witness-db` | `./watcher-db` |
| `public_url` | `http://localhost:3232` | `http://localhost:3236` |
| `http_port` | 3232 | 3236 |
| `seed` | null (random) | null (random) |
| `tel_storage_path` | -- | `./tel_storage` |
| `initial_oobis` | -- | `[]` |

### DynamoConfig

`prefix` (String, required), `region` (String, required), `endpoint` (Option, `.with_endpoint()`)

### HTTP Endpoints

**Witness:** GET `/introduce`, `/oobi/{id}`, `/oobi/{cid}/{role}/{eid}`, `/info`; POST `/process`, `/query`, `/query/tel`, `/process/tel`, `/register`, `/forward`

**Watcher:** GET `/introduce`, `/oobi/{id}`, `/oobi/{cid}/{role}/{eid}`, `/info`; POST `/process`, `/query`, `/query/tel`, `/register`, `/resolve`

**DefaultTransport routing:** Notice->`/process`, Reply->`/register`, Exchange->`/forward`, Query->`/query`, LocScheme->GET `/oobi/{eid}`, EndRole->GET `/oobi/{cid}/{role}/{eid}`, OOBI resolve->POST `/resolve`

### DB Directory Layout

- **Witness:** `{db_path}/events/events_database`, `{db_path}/events/tel/events/`, `{db_path}/events/tel/escrow/`
- **Watcher:** `{db_path}/events_database`, `{tel_storage_path}/registry`, `{tel_storage_path}/to_forward`
- **Controller:** `{db_path}/events_database`, `{db_path}/tel/events`, `{db_path}/tel/escrow`, `{db_path}/query_cache` (+query_cache)

### Key Infrastructure

- Key type: `BasicPrefix::Ed25519NT` (non-transferable), Sig: `SelfSigningPrefix::Ed25519Sha512`
- HTTP: `actix-web`, async: `#[actix_web::main]`
- Watcher channels: `tokio::sync::mpsc::channel(100)` for KEL and TEL updates
- Gossip: UDP, 3-4s jitter, 3 peers/round, 2048B, bincode, async-std
