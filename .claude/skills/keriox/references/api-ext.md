# keriox API: teliox, controller, sdk, witness, watcher, dynamodb, gossip

## teliox (Transaction Event Log)

### Events

```
enum Event { Management(ManagerTelEventMessage), Vc(VCEventMessage) }
  ::get_digest, ::get_prefix, ::get_sn, ::get_registry_id, ::serialize

type ManagerTelEventMessage = TypedEvent<ManagementTelType, ManagerTelEvent>
ManagerTelEvent { prefix, sn, event_type: ManagerEventType }
  ::new(prefix, sn, event_type) -> Self
  ::to_message(self, format, derivation) -> Result<ManagerTelEventMessage>
enum ManagementTelType { Vcp, Vrt }
enum ManagerEventType { Vcp(Inc), Vrt(Rot) }
Inc { issuer_id, config: Vec<Config>, backer_threshold: u64, backers: Vec<IdentifierPrefix> }
  ::incept_self_addressing(self, derivation, format) -> Result<ManagerTelEvent>
Rot { prev_event, backers_to_add, backers_to_remove }
enum Config { NoBackers }

type VCEventMessage = TypedEvent<TelEventType, TimestampedVCEvent>
VCEvent { prefix, sn, event_type: VCEventType }
  ::new, ::to_message, ::registry_id
enum TelEventType { Iss, Rev, Bis, Brv }
enum VCEventType { Iss(SimpleIssuance), Rev(SimpleRevocation), Bis(Issuance), Brv(Revocation) }
SimpleIssuance { registry_id }
SimpleRevocation { registry_id, prev_event_hash }
Issuance { issuer_id, registry_anchor: EventSeal }  ::new(issuer_id, registry_anchor) -> Self
Revocation { prev_event_hash, registry_anchor: Option<EventSeal> }

VerifiableEvent { event: Event, seal: AttachedSourceSeal }
  ::new, ::serialize, ::parse(stream: &[u8]) -> Result<Vec<Self>>
AttachedSourceSeal::new(sn, dig: SelfAddressingIdentifier) -> Self
parse_tel_query_stream(stream: &[u8]) -> Result<Vec<SignedTelQuery>>
```

### State

```
enum State { Management(ManagerTelState), Tel(TelState) }
ManagerTelState { prefix, sn, last, issuer, backers: Option<Vec<IdentifierPrefix>> }
  ::apply(&self, event: &ManagerTelEventMessage) -> Result<Self>
enum TelState { NotIssued, Issued(SelfAddressingIdentifier), Revoked }
  ::apply(&self, event: &VCEventMessage) -> Result<Self>
```

### Tel Facade

```
Tel<D: TelEventDatabase, K: EventDatabase> { processor, recently_added_events }
  ::new(tel_reference, kel_reference, publisher: Option<TelNotificationBus>) -> Self
  .make_inception_event, .make_rotation_event, .make_issuance_event, .make_revoke_event
  .parse_and_process_tel_stream(&self, stream: &[u8]) -> Result<()>
  .get_vc_state(&self, vc_hash) -> Result<Option<TelState>>
  .get_tel(&self, vc_hash) -> Result<Vec<VerifiableEvent>>
  .get_management_tel(&self, registry_id) -> Result<Option<Box<dyn DoubleEndedIterator<Item = VerifiableEvent>>>>
  .get_management_tel_state(&self, id) -> Result<Option<ManagerTelState>>
```

### Processor & Storage

```
TelEventProcessor<D, K>::new(kel_reference, tel_reference, tel_publisher) -> Self
  .process(&self, event: VerifiableEvent) -> Result<()>
  .process_signed_query(&self, qr: SignedTelQuery) -> Result<TelReplyType>
enum TelReplyType { Tel(Vec<u8>) }

TelEventStorage<D>::new(db: Arc<D>) -> Self
  .compute_management_tel_state, .compute_vc_state, .get_events, .add_event, .process_query
```

### Notification Bus

```
TelNotificationBus::new() -> Self
  .register_observer, .notify
trait TelNotifier { fn notify(&self, notification: &TelNotification, bus: &TelNotificationBus) -> Result<()>; }
enum TelNotification { MissingRegistry(VerifiableEvent), MissingIssuer(..), OutOfOrder(..), TelEventAdded(..) }
```

### Escrows (cfg(storage-redb))

```
MissingIssuerEscrow::new(db, escrow_db, duration, kel_reference, bus) -> Self  // also impl keri_core::Notifier
OutOfOrderEscrow::new(tel_reference, kel_reference, escrow_db, duration) -> Self
MissingRegistryEscrow::new(tel_reference, kel_reference, escrow_db, duration) -> Self
default_escrow_bus(tel_db, kel_storage, escrow_db) -> (TelNotificationBus, MissingIssuerEscrow, OutOfOrderEscrow, MissingRegistryEscrow)
```

### Query & Database

```
type TelQueryEvent = KeriEvent<Timestamped<TelQueryRoute>>
type SignedTelQuery = SignedQuery<TelQueryEvent>
enum TelQueryRoute { Tels { reply_route, args: TelQueryArgs } }
TelQueryArgs { i: Option<IdentifierPrefix>, ri: Option<IdentifierPrefix> }

trait TelEventDatabase {
    fn new(path) -> Result<Self>; fn add_new_event; fn get_events; fn get_management_events;
}
RedbTelDatabase  // cfg(storage-redb), impl TelEventDatabase + TelLogDatabase
```

---

## keri-sdk (`keriox_sdk/`) -- Generic, Lightweight

```
KeriRuntime<D: EventDatabase+EscrowCreator> { processor, storage, escrows, notification_bus }
  ::new(event_db: Arc<D>) -> Self
  ::with_config(event_db, escrow_config, notification_bus) -> Self

Controller<D, T: TelEventDatabase> { kel: KeriRuntime<D>, tel: Arc<Tel<T,D>> }
  ::new(event_db, tel_db) -> Self
  .incept(&self, public_keys, next_pub_keys) -> Result<String>
  .finalize_incept(&self, event: &[u8], sig: &SelfSigningPrefix) -> Result<Identifier<D>>
  .load_identifier(&self, id) -> Result<Identifier<D>>
  .process_kel(&self, messages: &[Message]) -> Result<()>
  .process_tel(&self, tel: &[u8]) -> Result<()>
  .get_vc_state, .get_state

Identifier<D> { id, event_storage }
  .get_prefix, .get_own_kel, .get_log_query, .get_tel_query
```

---

## keri-controller (`components/controller/`) -- Full-Featured, RedbDatabase-bound

### Controller

```
Controller { known_events, communication, cache }
  ::new(config: ControllerConfig) -> Result<Self>
  .incept(&self, public_keys, next_pub_keys, witnesses: Vec<LocationScheme>, witness_threshold) -> Result<String>  // async
  .finalize_incept(&self, event: &[u8], sig) -> Result<Identifier>
  .get_kel_with_receipts(&self, id) -> Option<Vec<Notice>>
  .verify(&self, data, signature: &Signature) -> Result<(), VerificationError>
  .find_state(&self, id) -> Result<IdentifierState>
```

### Identifier (full-featured)

Core:
```
.id() -> &IdentifierPrefix
.find_state, .current_public_keys, .witnesses, .watchers
.get_own_kel, .get_kel, .get_last_establishment_event_seal
```

KEL management:
```
async .rotate(current_keys, new_next_keys, new_next_threshold, wit_add, wit_remove, wit_threshold) -> Result<String>
.anchor(payload: &[SelfAddressingIdentifier]) -> Result<String>
.anchor_with_seal(seal_list: &[Seal]) -> Result<KeriEvent<KeyEvent>>
async .finalize_rotate, .finalize_anchor(&mut self, event: &[u8], sig) -> Result<()>
async .notify_witnesses(&mut self) -> Result<usize>
```

OOBI:
```
async .resolve_oobi(&self, oobi: &Oobi) -> Result<()>
async .send_oobi_to_watcher(&self, id, oobi) -> Result<()>
.get_location(&self, identifier) -> Result<Vec<LocationScheme>>
```

Query:
```
.query_watchers(&self, about_who: &EventSeal) -> Result<Vec<QueryEvent>>
async .finalize_query(&self, queries) -> (QueryResponse, Vec<WatcherResponseError>)
.query_mailbox(&self, identifier, witnesses) -> Result<Vec<MailboxQuery>>
async .finalize_query_mailbox(&mut self, queries) -> Result<Vec<ActionRequired>>
```

Group multisig:
```
.incept_group(&self, participants, sig_threshold, next_keys_threshold, witnesses, wit_threshold, delegator) -> Result<(String, Vec<String>)>
async .finalize_group_incept(&mut self, group_event, sig, exchanges) -> Result<IdentifierPrefix>
async .finalize_group_event(&mut self, group_event, sig, exchanges) -> Result<()>
```

TEL:
```
.incept_registry(&mut self) -> Result<(IdentifierPrefix, TypedEvent<EventTypeTag, KeyEvent>)>
.issue(&self, credential_digest) -> Result<(IdentifierPrefix, TypedEvent)>
.revoke(&self, credential_sai) -> Result<Vec<u8>>
async .finalize_incept_registry, .finalize_issue, .finalize_revoke(&mut self, event, sig)
.query_tel(&self, registry_id, vc_identifier) -> Result<TelQueryEvent>
```

Signing:
```
.sign_to_cesr(&self, data: &str, signatures: &[SelfSigningPrefix]) -> Result<String>
.verify_from_cesr(&self, stream: &[u8]) -> Result<()>
```

```
enum ActionRequired { MultisigRequest(KeriEvent<KeyEvent>, ExchangeMessage), DelegationRequest(..) }
enum QueryResponse { Updates, NoUpdates }
```

### Supporting Types

```
NontransferableIdentifier { id: BasicPrefix, communication }
  ::new(public_key, communication) -> Self
  .sign, .query_log, .query_ksn, async .finalize_query

KnownEvents { processor, storage, oobi_manager, partially_witnessed_escrow, tel }
  ::new(db_path, escrow_config) -> Result<Self>
  .save, .process, .process_stream, .get_state, .verify

Communication { events, transport, tel_transport }
  async .resolve_oobi, .send_message_to, .publish

ControllerConfig { db_path, initial_oobis, escrow_config, transport, tel_transport }
```

### Errors

```
ControllerError  -- top-level (RedbError, SQLError, SendingError, ParseError, etc.)
MechanicsError   -- identifier ops (SendingError, Transport, EventProcessing, etc.)
SendingError, WatcherResponseError, BroadcastingError, OobiRetrieveError
```

---

## Witness (`components/witness/`)

```
Witness { address: Url, prefix: BasicPrefix, processor: WitnessProcessor, event_storage, oobi_manager, signer, receipt_generator, tel }
  ::new(address: Url, signer, event_path: &Path, escrow_config: WitnessEscrowConfig) -> Result<Self>
  ::setup(public_address, event_db_path, priv_key: Option<String>, escrow_config) -> Result<Self>
  .oobi(&self) -> LocationScheme
  .process_notice, .process_exchange, .process_reply, .process_query
  .parse_and_process_notices, .parse_and_process_queries
  .parse_and_process_tel_queries, .parse_and_process_tel_events
  .get_mailbox_messages(&self, id) -> Result<MailboxResponse>

WitnessProcessor::new(redb: Arc<RedbDatabase>, escrow_config) -> Self  // impl Processor
WitnessReceiptGenerator { prefix, signer, storage }                    // impl Notifier (KeyEventAdded, PartiallyWitnessed)
WitnessEscrowConfig { partially_signed_timeout, out_of_order_timeout, delegation_timeout }
WitnessListener { witness_data: Arc<Witness> }
  ::setup(pub_addr, event_db_path, priv_key, escrow_config) -> Result<Self>
  .listen_http(&self, addr) -> actix_web::dev::Server

HTTP: /introduce, /oobi/{id}, /oobi/{cid}/{role}/{eid}, /process, /query, /query/tel, /process/tel, /register, /forward, /info
```

---

## Watcher (`components/watcher/`)

```
Watcher { watcher_data }
  ::new(config: WatcherConfig) -> Result<Self>
  .prefix, .oobi
  async .process_update_requests, .process_update_tel_requests
  .parse_and_process_notices, async .parse_and_process_queries
  async .resolve_end_role, .resolve_loc_scheme

WatcherData { address, prefix, processor, event_storage, oobi_manager, signer, transport }
  ::new(config, tx, tel_tx) -> Result<Arc<Self>>
  .process_notice, async .process_op, async .update_local_kel

WatcherListener::new(config: WatcherConfig) -> Result<Self>
  .listen_http(self, addr) -> actix_web::dev::Server

WatcherConfig { public_address, db_path, priv_key, transport, tel_transport, tel_storage_path, escrow_config }
#[async_trait] trait WatcherTelTransport { async fn send_query(&self, qry, location) -> Result<String>; }

HTTP: /introduce, /oobi/{id}, /oobi/{cid}/{role}/{eid}, /process, /query, /register, /resolve, /query/tel, /info
```

---

## keri-dynamodb (`support/dynamodb/`)

### Core Types

```
DynamoConfig { prefix: String, region: String, endpoint: Option<String> }
  ::new(prefix, region) -> Self
  ::with_endpoint(self, endpoint) -> Self
  ::table_name(&self, name: &str) -> String  // "{prefix}{name}"

DynamoDatabase::new(client: aws_sdk_dynamodb::Client, config: DynamoConfig) -> Self
  async ::from_aws_config(config) -> Self
  ::from_aws_config_blocking(config) -> Self  // Lambda cold start
  // impl EventDatabase, EscrowCreator

DynamoLogDatabase::new(client, config) -> Self       // impl LogDatabase<'static>
DynamoSequencedDb::new(client, escrows_table, escrow_name) -> Self  // impl SequencedEventDatabase
DynamoEscrowDb                                       // impl EscrowDatabase
DynamoTelDatabase::from_config(client, config) -> Self  // impl TelEventDatabase
  async ::from_aws_config(config) -> Self
```

### SQS (cfg(sqs))

```
SqsDispatch::new(client: aws_sdk_sqs::Client, queue_url) -> Self  // impl NotificationDispatch
  async ::from_aws_config(queue_url, region, endpoint) -> Self
SqsNotificationMessage { notification_type, payload_cesr_b64, aid, sn, timestamp }
```

### Serialization Utilities

```
signed_event_to_cesr(event) -> Result<Vec<u8>>       cesr_to_signed_event(bytes) -> Result<SignedEventMessage>
receipt_nt_to_cesr(receipt) -> Result<Vec<u8>>        cesr_to_receipt_nt(bytes) -> Result<SignedNontransferableReceipt>
state_to_json(state) -> Result<String>                json_to_state(json) -> Result<IdentifierState>
format_sn(sn: u64) -> String                         parse_sn(s: &str) -> Result<u64>
reply_to_cesr(reply) -> Result<Vec<u8>>               cesr_to_reply(bytes) -> Result<SignedReply>
serialize_notification(notification) -> Result<String>
deserialize_notification_message(json) -> Result<SqsNotificationMessage>
```

### DynamoDB Table Schema

| Table | PK | SK | Attributes |
|-------|----|----|------------|
| `{prefix}kel` | `aid` | `sn` (zero-padded) | digest, event_cesr, timestamp |
| `{prefix}states` | `aid` | -- | state_json, sn |
| `{prefix}events` | `digest` | -- | event_cesr, timestamp, nontrans_couplets |
| `{prefix}receipts-t` | `digest` | `receipt_id` | receipt (CESR) |
| `{prefix}receipts-nt` | `digest` | `witness` | couplet (CESR) |
| `{prefix}escrows` | `{name}#{aid}` | `{sn}#{digest}` | escrow_name |
| `{prefix}replies` | `aid` | `signer` | reply_cesr |
| `{prefix}tel-events` | `digest` | -- | event_cbor (CBOR) |
| `{prefix}tel-mgmt` | `aid` | `sn` (zero-padded) | digest |
| `{prefix}tel-vc` | `aid` | `sn` (zero-padded) | digest |

```
enum DynamoDbError { Sdk(String), Serialization(String), ConditionalCheckFailed(String), NotFound(String), Generic(String) }
// Converts into keri_core::Error::SemanticError and teliox::Error::Generic
```

---

## gossip (`support/gossip/`)

```
Server<T: Clone>::new(data: T, addr) -> io::Result<Self>  // async
  async .start, .bootstrap(peer_addr), .addr, .peers, .data
Data<T: Clone> { version: u32, value: T }
  ::new(value) -> Self  // impl Deref + DerefMut (auto-increments version)
// Protocol: UDP gossip, bincode, send every 3-4s to 3 random peers, 2048-byte buffer
```

---

## Cross-Reference Index

| Type | Crate | Used By |
|------|-------|---------|
| `IdentifierPrefix`, `BasicPrefix`, `SelfSigningPrefix` | keri-core | All |
| `SelfAddressingIdentifier` | `said` (external) | All |
| `KeriEvent<KeyEvent>`, `SignedEventMessage`, `Notice`/`Message` | keri-core | All |
| `EventDatabase`, `LogDatabase`, `EscrowCreator` | keri-core | dynamodb, sdk, controller, witness, watcher |
| `Processor`, `Notifier`, `NotificationBus` | keri-core | All processor setups |
| `EventStorage`, `IdentifierState` | keri-core | controller, sdk, witness, watcher, teliox |
| `Signer`, `KeyManager`, `CryptoBox` | keri-core | witness, watcher, controller |
| `RedbDatabase` / `MemoryDatabase` | keri-core | controller, witness, watcher / tests |
| `Tel`, `TelEventDatabase`, `TelState` | teliox | controller, sdk, witness |
| `DynamoDatabase`, `DynamoConfig`, `SqsDispatch` | keri-dynamodb | kerihost |
| `Server<T>`, `Data<T>` | gossip | No current consumers |
