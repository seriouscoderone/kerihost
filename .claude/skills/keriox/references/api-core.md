# keriox API: keri-core (`keriox_core`)

## Events

### Core Types

```
enum EventData { Icp(InceptionEvent), Rot(RotationEvent), Ixn(InteractionEvent), Dip(DelegatedInceptionEvent), Drt(RotationEvent) }
enum EventTypeTag { Icp, Rot, Ixn, Dip, Drt, Rct, Exn, Rpy, Qry }
```

| Struct | Key Fields |
|--------|-----------|
| `KeyEvent` | `prefix: IdentifierPrefix`, `sn: u64`, `event_data: EventData` |
| `InceptionEvent` | `key_config: KeyConfig`, `witness_config: InceptionWitnessConfig`, `data: Vec<Seal>` |
| `RotationEvent` | prev_hash(priv), `key_config`, `witness_config: RotationWitnessConfig`, `data: Vec<Seal>` |
| `InteractionEvent` | prev_hash(priv), `data: Vec<Seal>` (apply_to is no-op) |
| `DelegatedInceptionEvent` | `inception_data: InceptionEvent`, `delegator: IdentifierPrefix` (no new(), struct literal) |

```
KeyEvent::new(prefix, sn, event_data) -> Self
KeyEvent::to_message(self, format: SerializationFormats, derivation: HashFunction) -> Result<KeriEvent<KeyEvent>>
InceptionEvent::new(key_config, witness_config: Option, inception_config: Option) -> Self
InceptionEvent::incept_self_addressing(self, derivation, format) -> Result<KeriEvent<KeyEvent>>
RotationEvent::new(prev_hash, kc: KeyConfig, wc: RotationWitnessConfig, data: Vec<Seal>) -> Self
InteractionEvent::new(prev_hash, data: Vec<Seal>) -> Self
```

### Config & Threshold

| Struct | Fields |
|--------|--------|
| `InceptionWitnessConfig` | `tally: SignatureThreshold`, `initial_witnesses: Vec<BasicPrefix>` |
| `RotationWitnessConfig` | `tally`, `prune: Vec<BasicPrefix>`, `graft: Vec<BasicPrefix>` |
| `KeyConfig` | `threshold: SignatureThreshold`, `public_keys: Vec<BasicPrefix>`, `next_keys_data: NextKeysData` |
| `NextKeysData` | `threshold: SignatureThreshold`, next_key_hashes(priv) |

```
KeyConfig::new(public_keys, next_keys_data, threshold: Option) -> Self
KeyConfig::verify(&self, msg: &[u8], sigs: &[IndexedSignature]) -> Result<bool, SignatureError>
KeyConfig::verify_next(&self, next: &KeyConfig) -> Result<bool>
KeyConfig::commit(&self, derivation: &HashFunction) -> NextKeysData
NextKeysData::new(threshold, hashes: impl IntoIterator<Item = SelfAddressingIdentifier>) -> Self
nxt_commitment(threshold, keys: &[BasicPrefix], derivation) -> NextKeysData
enum SignatureThreshold { Simple(u64), Weighted(WeightedThreshold) }
  ::simple(u64) | ::single_weighted(Vec<(u64,u64)>) | ::multi_weighted(Vec<Vec<(u64,u64)>>)
  ::enough_signatures(&self, sigs_indexes: &[usize]) -> Result<()>
```

### Seals & Receipts

```
enum Seal { Location(LocationSeal), Event(EventSeal), Digest(DigestSeal), Root(RootSeal) }
DigestSeal::new(said) | EventSeal::new(id, sn, digest) | SourceSeal::new(sn, digest)
Receipt::new(format, receipted_event_digest, prefix, sn) -> Self
Receipt::encode(&self) -> Result<Vec<u8>>
```

### KeriEvent (TypedEvent)

```
type KeriEvent<D> = TypedEvent<EventTypeTag, D>
TypedEvent::new(format, derivation: HashFunction, event: D) -> Self  // auto-digest
  .digest() -> Result<SelfAddressingIdentifier>
  .check_digest() -> Result<()>
  .encode() -> Result<Vec<u8>>

KeriEvent<KeyEvent> extensions:
  .sign(sigs, witness_sigs: Option, delegator_seal: Option) -> SignedEventMessage
  .compare_digest(&self, sai) -> Result<bool>
  .to_derivation_data(&self) -> Result<Vec<u8>>
verify_identifier_binding(icp_event) -> Result<bool>
```

### Signed Messages

```
enum Message { Notice(Notice), Op(Op) }  .to_cesr() -> Result<Vec<u8>>
enum Notice { Event(SignedEventMessage), NontransferableRct(..), TransferableRct(..) }
enum Op { Exchange(SignedExchange), Reply(SignedReply), Query(SignedQueryMessage) }

SignedEventMessage { event_message, signatures: Vec<IndexedSignature>, witness_receipts: Option<Vec<Nontransferable>>, delegator_seal: Option<SourceSeal> }
  ::new(..) -> Self | .encode() -> Result<Vec<u8>>
SignedNontransferableReceipt { body: Receipt, signatures: Vec<Nontransferable> }
SignedTransferableReceipt { body: Receipt, validator_seal: EventSeal, signatures: Vec<IndexedSignature> }

enum Signature { Transferable(SignerData, Vec<IndexedSignature>), NonTransferable(Nontransferable) }
  .verify<D: EventDatabase>(&self, data: &[u8], storage: &EventStorage<D>) -> Result<bool>
enum Nontransferable { Indexed(Vec<IndexedSignature>), Couplet(Vec<(BasicPrefix, SelfSigningPrefix)>) }
enum SignerData { EventSeal(EventSeal), LastEstablishment(IdentifierPrefix), JustSignatures }
```

### EventMsgBuilder (builder pattern)

```
EventMsgBuilder::new(event_type: EventTypeTag) -> Self
  .with_prefix .with_keys .with_next_keys .with_next_keys_hashes
  .with_sn .with_previous_event .with_seal .with_delegator
  .with_threshold .with_next_threshold
  .with_witness_list .with_witness_to_add .with_witness_to_remove .with_witness_threshold
  .build(self) -> Result<KeriEvent<KeyEvent>>
ReceiptBuilder::default().with_format(..).with_receipted_event(..).build() -> Result<Receipt>
```

### Parsing

```
parse_event_type(input: &[u8]) -> Result<EventType, ParseError>
enum EventType { KeyEvent(..), Receipt(..), Exn, Qry, MailboxQry, Rpy }
Timestamped<D>::new(data) -> Self  // Utc::now()
TimestampedEventMessage::new(event) -> Self  // Local::now()
```

## Processor

### Traits

```
trait Processor {
    type Database: EventDatabase;
    fn process_notice(&self, &Notice) -> Result<()>;
    fn process_op_reply(&self, &SignedReply) -> Result<()>;  // cfg(query)
    fn register_observer(&self, Arc<dyn Notifier>, &[JustNotification]) -> Result<()>;
    fn process(&self, &Message) -> Result<()>;
}
trait Notifier { fn notify(&self, &Notification, &NotificationBus) -> Result<()>; }
trait NotificationDispatch: Send+Sync { fn dispatch(&self, &Notification) -> Result<()>; }
```

### Core Structs

```
BasicProcessor<D>::new(db: Arc<D>, notification_bus: Option<NotificationBus>) -> Self  // impl Processor
NotificationBus::new() -> Self | ::from_dispatch(Arc<dyn NotificationDispatch>) -> Self
  .notify(&self, &Notification) -> Result<()>
```

### Notifications (variants carry SignedEventMessage or similar)

Notification: KeyEventAdded, OutOfOrder, PartiallySigned, PartiallyWitnessed, ReceiptAccepted, ReceiptEscrowed, ReceiptOutOfOrder, TransReceiptOutOfOrder, DupliciousEvent, MissingDelegatingEvent, KsnOutOfOrder
JustNotification: all above + KsnUpdated, GotOobi, ReplayLog, ReplyKsn, GetMailbox

### Validator & Storage

```
EventValidator<D>::new(event_database: Arc<D>) -> Self
  .validate_event(&self, signed_event) -> Result<Option<IdentifierState>>
  .validate_validator_receipt, .validate_witness_receipt
  .verify(&self, data: &[u8], sig: &Signature) -> Result<(), VerificationError>
enum VerificationError { VerificationFailure, SignatureError, NotEstablishment(EventSeal), MissingSigner, MoreInfo(MoreInfoError) }
enum MoreInfoError { EventNotFound(EventSeal), UnknownIdentifier(IdentifierPrefix) }

EventStorage<D>::new(events_db: Arc<D>) -> Self
  .get_state(id) -> Option<IdentifierState>
  .get_kel(id) -> Result<Option<Vec<u8>>>  // CESR bytes
  .get_kel_messages(id) -> Result<Option<Vec<Notice>>>
  .get_kel_messages_with_receipts_all(id) | _range(id, sn, limit)
  .get_event_at_sn(id, sn) -> Option<TimestampedSignedEventMessage>
  .get_last_establishment_event_seal(id) -> Option<EventSeal>
  .compute_state_at_sn(id, sn) | .get_keys_at_event(id, sn, digest)
  .has_receipt(id, sn, validator_pref) -> Result<bool>
  .get_nt_receipts(id, sn) | .get_ksn_for_prefix(prefix, format)  // cfg(query)
```

### Escrows

```
default_escrow_bus<D>(event_db, escrow_config, notification_bus) -> (NotificationBus, EscrowSet<D>)
EscrowConfig { out_of_order_timeout, partially_signed_timeout, partially_witnessed_timeout, trans_receipt_timeout, delegation_timeout }  // all default 60s
MaybeOutOfOrderEscrow<D>::new(db, duration)       // OutOfOrder, KeyEventAdded
PartiallySignedEscrow<D>::new(db, duration)       // PartiallySigned
PartiallyWitnessedEscrow<D>::new(db, log_db, dur) // PartiallyWitnessed, ReceiptOutOfOrder
DelegationEscrow<D>::new(db, duration)            // MissingDelegatingEvent, KeyEventAdded
DuplicitousEvents<D>::new(db)                     // DupliciousEvent
compute_state<D>(db: Arc<D>, id) -> Option<IdentifierState>  // replay all events
```

## Database

### Traits

```
trait EventDatabase {
    type Error; type LogDatabaseType: LogDatabase<'static>;
    fn add_kel_finalized_event, add_receipt_t, add_receipt_nt;
    fn get_key_state(id) -> Option<IdentifierState>;
    fn get_kel_finalized_events(params: QueryParameters) -> Option<impl DoubleEndedIterator<Item=TimestampedSignedEventMessage>>;
    fn get_receipts_t, get_receipts_nt; fn accept_to_kel;
    fn save_reply, get_reply;  // cfg(query)
}
trait LogDatabase<'db>: Send+Sync {
    fn log_event, log_event_with_new_transaction;
    fn get_signed_event(said) -> Option<TimestampedSignedEventMessage>;
    fn get_event(said) -> Option<KeriEvent<KeyEvent>>;
    fn get_signatures(said), get_nontrans_couplets(said);
}
trait EscrowDatabase: Send+Sync { fn save_digest, insert, get, remove, contains; }
trait EscrowCreator { fn create_escrow_db(&self, table_name) -> Self::EscrowDatabaseType; }
enum QueryParameters { BySn { id, sn }, Range { id, start, limit }, All { id } }
```

### Implementations

```
MemoryDatabase::new() -> Self  // in-memory, testing. impl EventDatabase, EscrowCreator
RedbDatabase::new(db_path: &Path) -> Result<Self>  // cfg(storage-redb), persistent
type TimestampedSignedEventMessage = Timestamped<SignedEventMessage>
```

## Primitives

### Prefixes & Signatures

```
enum IdentifierPrefix { Basic(BasicPrefix), SelfAddressing(SaidValue), SelfSigning(SelfSigningPrefix) }
  ::self_addressing(said) | ::basic(bp) | impl FromStr, Display

enum BasicPrefix { ECDSAsecp256k1NT(..), ECDSAsecp256k1(..), Ed25519NT(..), Ed25519(..), Ed448NT(..), Ed448(..), X25519(..), X448(..) }
  ::new(code: CesrBasic, public_key: PublicKey) -> Self
  ::verify(&self, data, signature: &SelfSigningPrefix) -> Result<bool>
  ::is_transferable(&self) -> bool

enum SelfSigningPrefix { Ed25519Sha512(Vec<u8>), ECDSAsecp256k1Sha256(Vec<u8>), Ed448(Vec<u8>) }
enum Index { CurrentOnly(u16), BothSame(u16), BothDifferent(u16, u16) }
IndexedSignature { index: Index, signature: SelfSigningPrefix }
  ::new_both_same(sig, index) | ::new_current_only(sig, index)
enum SeedPrefix { RandomSeed128, RandomSeed256Ed25519, RandomSeed256ECDSAsecp256k1, RandomSeed448 }
  ::derive_key_pair(&self) -> Result<(PublicKey, PrivateKey)>
```

### Keys & Signer

```
PublicKey::new(key: Vec<u8>)  .verify_ed(msg, sig) -> bool  .verify_ecdsa(msg, sig) -> bool
PrivateKey::new(key: Vec<u8>)  .sign_ed(msg) .sign_ecdsa(msg)  // zeroize on Drop
trait KeyManager { fn sign, public_key, next_public_key, rotate; }
Signer::new() -> Self  // random Ed25519
  ::new_with_key(priv_key: &[u8;32]) | ::new_with_seed(seed: &SeedPrefix)
  .sign(msg) -> Result<Vec<u8>>  .public_key() -> PublicKey
CryptoBox::new() -> Result<Self>  // random current+next, impl KeyManager
```

### IdentifierState

```
IdentifierState { prefix, sn, last_event_digest, last_previous, last_event_type, current: KeyConfig, witness_config: WitnessConfig, delegator: Option, last_est }
  ::apply<T: EventSemantics>(self, event: &T) -> Result<Self>
trait EventSemantics { fn apply_to(&self, state) -> Result<IdentifierState>; }
WitnessConfig { tally: SignatureThreshold, witnesses: Vec<BasicPrefix> }
  .enough_receipts(receipts_couplets, indexed_receipts) -> Result<bool>
verify(data, key: &BasicPrefix, sig: &SelfSigningPrefix) -> Result<bool>
derive(seed: &SeedPrefix, transferable: bool) -> Result<BasicPrefix>
```

## Actor

### Parsers (all `&[u8] -> Result<Vec<T>>`)

```
parse_event_stream -> Vec<Message>  |  parse_notice_stream -> Vec<Notice>
parse_op_stream -> Vec<Op>          |  parse_query_stream -> Vec<SignedQueryMessage>  // cfg(query|oobi-manager)
parse_reply_stream -> Vec<SignedReply>  // cfg(query)
parse_exchange_stream -> Vec<SignedExchange>  // cfg(mailbox)
```

### Processors

```
process_notice<P: Processor>(msg: Notice, processor) -> Result<()>
process_reply<P: Processor>(sr, oobi_manager, processor, event_storage) -> Result<()>  // cfg(query)
process_signed_oobi<D>(signed_oobi, oobi_manager, event_storage) -> Result<()>         // cfg(oobi-manager)
process_signed_exn<D>(exn, storage) -> Result<()>                                      // cfg(mailbox)
process_signed_query<D>(qr, storage) -> Result<ReplyType>                              // cfg(query)
```

### Event Generators

```
incept(public_keys, next_pub_keys, witnesses, wit_threshold, delegator_id) -> Result<String>
incept_with_next_hashes(.., next_hashes, ..) -> Result<KeriEvent<KeyEvent>>
rotate(state, current_keys, new_next, new_next_threshold, wit_add, wit_remove, wit_threshold) -> Result<String>
anchor(state, payload: &[SelfAddressingIdentifier]) -> Result<String>
anchor_with_seal(state, seal_list: &[Seal]) -> Result<KeriEvent<KeyEvent>>
generate_end_role(controller_id, watcher_id, role: Role, enabled) -> ReplyEvent  // cfg(oobi)
exchange(recipient, data, topic: ForwardTopic) -> ExchangeMessage  // cfg(mailbox)
enum ReplyType { Ksn(KeyStateNotice), Kel(Vec<Message>), Mbx(MailboxResponse) }
```

### OOBI & Transport

```
enum Oobi { Location(LocationScheme), EndRole(EndRole) }
LocationScheme { eid: IdentifierPrefix, scheme: Scheme, url: Url }
EndRole { cid, role: Role, eid }
enum Scheme { Http, Tcp }  |  enum Role { Controller, Witness, Watcher, Messagebox }
OobiManager::new(events_db: Arc<RedbDatabase>)  // cfg(oobi-manager)
  .check_oobi_reply .parse_and_save .save_oobi .get_loc_scheme .get_end_role .process_oobi

#[async_trait] trait Transport {
    async fn send_message(&self, loc, msg: Message) -> Result<()>;
    async fn send_query(&self, loc, qry) -> Result<PossibleResponse>;  // cfg(query)
    async fn request_loc_scheme, request_end_role, resolve_oobi;
}
DefaultTransport::new() -> Self  // HTTP via reqwest
```

## Query / Reply -- cfg(query)

```
type QueryEvent = KeriEvent<Timestamped<QueryRoute>>
enum QueryRoute { Logs { reply_route, args: LogsQueryArgs }, Ksn { reply_route, args: LogsQueryArgs } }
LogsQueryArgs { s: Option<u64>, limit: Option<u64>, i: IdentifierPrefix, src: Option }
SignedQuery<D> { query: D, signature: Signature }
  ::new_nontrans(query, signer, signature) | ::new_trans(query, signer_id, sigs)

type ReplyEvent = KeriEvent<Timestamped<ReplyRoute>>
enum ReplyRoute { Ksn(..), LocScheme(..), EndRoleAdd(..), EndRoleCut(..) }
SignedReply { reply: ReplyEvent, signature: Signature }  ::new_nontrans | ::new_trans
bada_logic(new_rpy, old_rpy) -> Result<()>  // Best Available Data Acceptance
KeyStateNotice { serialization_info, state, first_seen_sn, timestamp, config }
  ::new_ksn(state, serialization) -> Self
```

### Mailbox -- cfg(mailbox)

```
type MailboxQuery = KeriEvent<Timestamped<MailboxRoute>>
QueryArgsMbx { pre, topics: QueryTopics, i, src }
MailboxResponse { receipt, multisig, delegate }
type ExchangeMessage = KeriEvent<Timestamped<Exchange>>
enum Exchange { Fwd { args: FwdArgs, to_forward: KeriEvent<KeyEvent> } }
enum ForwardTopic { Multisig, Delegate }
```

## Feature Gates

`query`: Rpy, SignedReply, SignedQuery, ReplyType, bada_logic, KSN
`mailbox`: Exchange, SignedExchange, MailboxResponse, ForwardTopic
`oobi`: Oobi, LocationScheme, EndRole, Role, Scheme
`oobi-manager`: OobiManager, OobiStorage (requires storage-redb)
`storage-redb`: RedbDatabase, ReplyEscrow, persistent storage
