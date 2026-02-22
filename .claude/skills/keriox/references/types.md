# keriox Type Reference

Compact data-shape reference for the keriox Rust workspace. JSON keys shown where serde renames apply.

## 1. Events (keri-core)

**KeyEvent**: `{ prefix("i"): IdentifierPrefix, sn("s"): u64 hex, event_data(flatten): EventData }`

**EventData** `#[serde(untagged)]`: `Icp(InceptionEvent)` | `Rot(RotationEvent)` | `Ixn(InteractionEvent)` | `Dip(DelegatedInceptionEvent)` | `Drt(RotationEvent)`

**EventTypeTag** `#[serde(rename_all = "lowercase")]`: `Icp | Rot | Ixn | Dip | Drt | Rct | Exn | Rpy[query] | Qry`

**InceptionEvent**: `{ key_config(flatten): KeyConfig, witness_config(flatten): InceptionWitnessConfig, inception_configuration("c"): Vec<String>, data("a"): Vec<Seal> }`

**RotationEvent**: `{ previous_event_hash("p"): SaidValue [priv], key_config(flatten): KeyConfig, witness_config(flatten): RotationWitnessConfig, data("a"): Vec<Seal> }` -- also used for Drt

**InteractionEvent**: `{ previous_event_hash("p"): SaidValue [priv], data("a"): Vec<Seal> }`

**DelegatedInceptionEvent**: `{ inception_data(flatten): InceptionEvent, delegator("di"): IdentifierPrefix }`

**Receipt** (no rkyv): `{ serialization_info("v"): SerializationInfo, event_type("t"): EventTypeTag [=Rct], receipted_event_digest("d"): SelfAddressingIdentifier, prefix("i"): IdentifierPrefix, sn("s"): u64 hex }`

## 2. Sections (keri-core)

**KeyConfig**: `{ threshold("kt"): SignatureThreshold, public_keys("k"): Vec<BasicPrefix>, next_keys_data(flatten): NextKeysData }`

**NextKeysData**: `{ threshold("nt"): SignatureThreshold, next_key_hashes("n"): Vec<SaidValue> [priv] }`

**SignatureThreshold** (default `Simple(1)`): `Simple(u64)` | `Weighted(WeightedThreshold)`

**WeightedThreshold**: `Single(ThresholdClause)` | `Multi(MultiClauses)` -- ThresholdClause=newtype Vec\<ThresholdFraction\>, MultiClauses=newtype Vec\<ThresholdClause\>

**InceptionWitnessConfig** (default tally=0, witnesses=[]): `{ tally("bt"): SignatureThreshold, initial_witnesses("b"): Vec<BasicPrefix> }`

**RotationWitnessConfig** (default tally=0): `{ tally("bt"): SignatureThreshold, prune("br"): Vec<BasicPrefix>, graft("ba"): Vec<BasicPrefix> }`

**Seal** `#[serde(untagged)]`: `Location(LocationSeal)` | `Event(EventSeal)` | `Digest(DigestSeal)` | `Root(RootSeal)`

**EventSeal**: `{ prefix("i"): IdentifierPrefix, sn("s"): u64 hex, event_digest("d"): SaidValue [priv] }`

**DigestSeal**: `{ dig("d"): SaidValue [priv] }` | **RootSeal**: `{ tree_root("rd"): SaidValue [priv] }`

**LocationSeal**: `{ prefix("i"): IdentifierPrefix, sn("s"): u64 hex, ilk("t"): String, prior_digest("p"): SaidValue [priv] }`

**DelegatingEventSeal** (no rkyv): `{ prefix("i"): IdentifierPrefix, commitment("d"): SelfAddressingIdentifier }`

**SourceSeal** (no serde): `{ sn: u64, digest: SaidValue }`

## 3. Messages (keri-core)

**TypedEvent\<T, D\>**: `{ serialization_info("v"): SerializationInfo, event_type("t"): T, digest("d"): Option<SaidValue> [pub(crate)], data(flatten): D }`

`type KeriEvent<D> = TypedEvent<EventTypeTag, D>;` -- primary: `KeriEvent<KeyEvent>`

**TimestampedEventMessage**: `{ timestamp: DateTime<Local>, event_message: KeriEvent<KeyEvent> }` -- ordered by sn

**Timestamped\<D\>**: `{ timestamp("dt"): DateTime<FixedOffset>, data(flatten): D }`

`type TimeStamp = DateTime<FixedOffset>;`

**Message**: `Notice(Notice)` | `Op(Op)` [query]

**Notice**: `Event(SignedEventMessage)` | `NontransferableRct(SignedNontransferableReceipt)` | `TransferableRct(SignedTransferableReceipt)`

**Op** [query/oobi]: `Exchange(SignedExchange)` [mailbox] | `Reply(SignedReply)` [query] | `Query(SignedQueryMessage)` [query]

**SignedEventMessage** (custom Serialize, rkyv): `{ event_message: KeriEvent<KeyEvent>, signatures: Vec<IndexedSignature>, witness_receipts: Option<Vec<Nontransferable>>, delegator_seal: Option<SourceSeal> }`

**SignedTransferableReceipt**: `{ body: Receipt, validator_seal: EventSeal, signatures: Vec<IndexedSignature> }`

**SignedNontransferableReceipt** (rkyv): `{ body: Receipt, signatures: Vec<Nontransferable> }`

**Signature** (no rkyv): `Transferable(SignerData, Vec<IndexedSignature>)` | `NonTransferable(Nontransferable)`

**SignerData** (no rkyv): `EventSeal(EventSeal)` | `LastEstablishment(IdentifierPrefix)` | `JustSignatures`

**Nontransferable** (rkyv): `Indexed(Vec<IndexedSignature>)` | `Couplet(Vec<(BasicPrefix, SelfSigningPrefix)>)`

**Transferable** (rkyv): `Seal(EventSeal, Vec<IndexedSignature>)`

## 4. Prefixes (keri-core)

**IdentifierPrefix** (serde: CESR string, default: SelfAddressing): `Basic(BasicPrefix)` | `SelfAddressing(SaidValue)` | `SelfSigning(SelfSigningPrefix)`

**BasicPrefix** (serde: CESR string) -- all wrap `PublicKey`:

| Variant | Code | Notes |
|---------|------|-------|
| ECDSAsecp256k1NT | 1AAA | non-transferable |
| ECDSAsecp256k1 | 1AAB | transferable |
| Ed25519NT | B | non-transferable |
| Ed25519 | D | transferable |
| Ed448NT | 1AAC | non-transferable |
| Ed448 | 1AAD | transferable |
| X25519 | C | key exchange |
| X448 | L | key exchange |

**SelfSigningPrefix** (serde: CESR string) -- all wrap `Vec<u8>`: `Ed25519Sha512("0B", 64B)` | `ECDSAsecp256k1Sha256("0C", 64B)` | `Ed448("1AAE", 114B)`

**IndexedSignature**: `{ index: Index, signature: SelfSigningPrefix }`

**Index**: `CurrentOnly(u16)` | `BothSame(u16)` | `BothDifferent(u16, u16)` [current, prev_next]

**SeedPrefix** (CESR string): `RandomSeed128 | RandomSeed256Ed25519("A") | RandomSeed256ECDSAsecp256k1 | RandomSeed448` -- wrap `Vec<u8>`

**PublicKey**: `{ public_key: Vec<u8> }` | **PrivateKey** (Drop zeroes): `{ key: Vec<u8> [priv] }`

**SaidValue** (rkyv adapter): `{ said: SelfAddressingIdentifier }` -- `#[rkyv(with = SAIDef)]`

## 5. State (keri-core)

**IdentifierState**: `{ prefix("i"): IdentifierPrefix, sn("s"): u64 hex, last_event_digest("d"): SaidValue, last_previous("p"): Option<SaidValue>, last_event_type("et"): Option<EventTypeTag>, current(flatten): KeyConfig, witness_config(flatten): WitnessConfig, delegator("di"): Option<IdentifierPrefix> [empty_string_as_none], last_est("ee"): LastEstablishmentData }`

**WitnessConfig** (state): `{ tally("bt"): SignatureThreshold, witnesses("b"): Vec<BasicPrefix> }`

**LastEstablishmentData**: `{ sn("s"): u64 [pub(crate)], digest("d"): SaidValue [pub(crate)], br("br"): Vec<BasicPrefix> [pub(crate)], ba("ba"): Vec<BasicPrefix> [pub(crate)] }`

### EventSemantics (trait)

```
fn apply_to(&self, state: IdentifierState) -> Result<IdentifierState, Error>;
```

Implementors: KeyEvent, EventData, InceptionEvent, RotationEvent, InteractionEvent, DelegatedInceptionEvent, KeriEvent\<KeyEvent\>, SignedEventMessage

## 6. TEL (teliox)

**Event** `#[serde(untagged)]`: `Management(ManagerTelEventMessage)` | `Vc(VCEventMessage)`

`type ManagerTelEventMessage = TypedEvent<ManagementTelType, ManagerTelEvent>;`
`type VCEventMessage = TypedEvent<TelEventType, TimestampedVCEvent>;`

**ManagementTelType**: `Vcp` (registry icp) | `Vrt` (registry rot)
**TelEventType**: `Iss | Rev | Bis | Brv`

**ManagerTelEvent**: `{ prefix("i"): IdentifierPrefix, sn("s"): u64, event_type(flatten): ManagerEventType }`

**ManagerEventType** `#[serde(untagged)]`: `Vcp(Inc)` | `Vrt(Rot)`

**Inc**: `{ issuer_id("ii"): IdentifierPrefix, config("c"): Vec<Config>, backer_threshold("bt"): u64, backers("b"): Vec<IdentifierPrefix> }`

**Rot** (TEL): `{ prev_event("p"): SAI, backers_to_add("ba"): Vec<IdentifierPrefix>, backers_to_remove("br"): Vec<IdentifierPrefix> }`

**Config** (TEL): `NoBackers` ("NB")

**VCEvent**: `{ prefix("i"): IdentifierPrefix, sn("s"): u64, event_type(flatten): VCEventType }`

**VCEventType** `#[serde(untagged)]`: `Rev(SimpleRevocation)` | `Iss(SimpleIssuance)` | `Bis(Issuance)` | `Brv(Revocation)`

**SimpleIssuance**: `{ registry_id("ri"): IdentifierPrefix }` | **SimpleRevocation**: `{ registry_id("ri"): IdentifierPrefix, prev_event_hash("p"): SAI }`

**Issuance** (backed): `{ issuer_id("ii"): IdentifierPrefix [priv], registry_anchor("ra"): EventSeal }`

**Revocation** (backed): `{ prev_event_hash("p"): SAI, registry_anchor("ra"): Option<EventSeal> }`

**TelState** (default NotIssued): `NotIssued | Issued(SAI) | Revoked`

**ManagerTelState**: `{ prefix: IdentifierPrefix, sn: u64, last: SAI, issuer: IdentifierPrefix, backers: Option<Vec<IdentifierPrefix>> }`

## 7. Database Traits (keri-core)

### EventDatabase

```
type Error; type LogDatabaseType: LogDatabase<'static>;
fn get_log_db(&self) -> Arc<Self::LogDatabaseType>;
fn add_kel_finalized_event(&self, event: SignedEventMessage, id: &IdentifierPrefix) -> Result<()>;
fn add_receipt_t(&self, receipt: SignedTransferableReceipt, id: &IdentifierPrefix) -> Result<()>;
fn add_receipt_nt(&self, receipt: SignedNontransferableReceipt, id: &IdentifierPrefix) -> Result<()>;
fn get_key_state(&self, id: &IdentifierPrefix) -> Option<IdentifierState>;
fn get_kel_finalized_events(&self, params: QueryParameters) -> Option<impl DoubleEndedIterator<Item = TimestampedSignedEventMessage>>;
fn accept_to_kel(&self, event: &KeriEvent<KeyEvent>) -> Result<()>;
```

### LogDatabase\<'db\>: Send + Sync

```
type DatabaseType; type Error; type TransactionType;
fn log_event(&self, txn: &Self::TransactionType, event: &SignedEventMessage) -> Result<()>;
fn log_receipt(&self, txn: &Self::TransactionType, receipt: &SignedNontransferableReceipt) -> Result<()>;
fn get_signed_event(&self, said: &SAI) -> Result<Option<TimestampedSignedEventMessage>>;
fn get_event(&self, said: &SAI) -> Result<Option<KeriEvent<KeyEvent>>>;
fn get_signatures(&self, said: &SAI) -> Result<Option<impl Iterator<Item = IndexedSignature>>>;
fn get_nontrans_couplets(&self, said: &SAI) -> Result<Option<impl Iterator<Item = Nontransferable>>>;
```

**QueryParameters**: `BySn { id, sn }` | `Range { id, start, limit }` | `All { id }`

`type TimestampedSignedEventMessage = Timestamped<SignedEventMessage>;`

| Backend | Error | LogDb | EscrowDb |
|---------|-------|-------|----------|
| MemoryDatabase | crate::error::Error | MemoryLogDatabase | MemoryEscrowDb |
| RedbDatabase | RedbError | loging::LogDatabase | SnKeyEscrow |
| DynamoDatabase | crate::error::Error | DynamoLogDatabase | DynamoEscrowDb |

## 8. Query / Reply / OOBI (keri-core)

**QueryRoute** `#[serde(tag = "r")]`: `Logs { reply_route, args: LogsQueryArgs }` | `Ksn { reply_route, args: LogsQueryArgs }`

**LogsQueryArgs**: `{ s: Option<u64>, limit: Option<u64>, i: IdentifierPrefix, src: Option<IdentifierPrefix> }`

`type QueryEvent = KeriEvent<Timestamped<QueryRoute>>;`
`type SignedKelQuery = SignedQuery<QueryEvent>;`

**SignedQuery\<D\>**: `{ query: D, signature: Signature }`

**ReplyRoute** (custom serde): `Ksn(IdentifierPrefix, KeyStateNotice)` | `LocScheme(LocationScheme)` [oobi] | `EndRoleAdd(EndRole)` [oobi] | `EndRoleCut(EndRole)` [oobi]

`type ReplyEvent = KeriEvent<Timestamped<ReplyRoute>>;`

**SignedReply**: `{ reply: ReplyEvent, signature: Signature }`

**KeyStateNotice** (custom Serialize): `{ serialization_info("v"), state(flatten): IdentifierState, first_seen_sn("f") [priv], timestamp("dt"): DateTime<FixedOffset>, config("c") [priv] }`

**Oobi** `#[serde(untagged)]` [oobi]: `Location(LocationScheme)` | `EndRole(EndRole)`

**LocationScheme**: `{ eid: IdentifierPrefix, scheme: Scheme, url: Url }`

**EndRole**: `{ cid: IdentifierPrefix, role: Role, eid: IdentifierPrefix }`

**Scheme**: `Http | Tcp` | **Role**: `Controller | Witness | Watcher | Messagebox`

## 9. Mailbox (keri-core, feature=mailbox)

**MailboxResponse**: `{ receipt: Vec<SignedNontransferableReceipt>, multisig: Vec<SignedEventMessage>, delegate: Vec<SignedEventMessage> }`

**Exchange** `#[serde(tag = "r")]`: `Fwd("/fwd") { args: FwdArgs, to_forward: KeriEvent<KeyEvent> }`

`type ExchangeMessage = KeriEvent<Timestamped<Exchange>>;`

**FwdArgs**: `{ recipient_id("pre"): IdentifierPrefix, topic: ForwardTopic }`

**ForwardTopic**: `Multisig | Delegate`

`type MailboxQuery = KeriEvent<Timestamped<MailboxRoute>>;`
`type SignedMailboxQuery = SignedQuery<MailboxQuery>;`

## 10. Core Traits

### Processor (keri-core)

```
type Database: EventDatabase + 'static;
fn process_notice(&self, notice: &Notice) -> Result<(), Error>;
fn process_op_reply(&self, reply: &SignedReply) -> Result<(), Error>; // [query]
fn register_observer(&self, observer: Arc<dyn Notifier>, notifications: &[JustNotification]) -> Result<()>;
fn process(&self, msg: &Message) -> Result<(), Error>; // default
```

### KeyManager (keri-core)

```
fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, Error>;
fn public_key(&self) -> PublicKey;
fn next_public_key(&self) -> PublicKey;
fn rotate(&mut self) -> Result<(), Error>;
```

### Transport (async, feature=oobi-manager)

```
async fn send_message(&self, loc: LocationScheme, msg: Message) -> Result<()>;
async fn send_query(&self, loc: LocationScheme, qry: SignedQueryMessage) -> Result<PossibleResponse>;
async fn request_loc_scheme(&self, loc: LocationScheme) -> Result<Vec<Op>>;
async fn resolve_oobi(&self, loc: LocationScheme, oobi: Oobi) -> Result<()>;
```

**Notification**: `KeyEventAdded(SEM) | OutOfOrder(SEM) | PartiallySigned(SEM) | PartiallyWitnessed(SEM) | ReceiptAccepted | ReceiptEscrowed | ReceiptOutOfOrder(SNTR) | TransReceiptOutOfOrder(STR) | DupliciousEvent(SEM) | MissingDelegatingEvent(SEM) | KsnOutOfOrder(SignedReply)[query]`

**EscrowConfig** (all 60s): `{ out_of_order_timeout, partially_signed_timeout, partially_witnessed_timeout, trans_receipt_timeout, delegation_timeout: Duration }`

## 11. DynamoDB (keri-dynamodb)

**DynamoConfig**: `{ prefix: String, region: String, endpoint: Option<String> }`

DynamoSequencedDb key: PK=`{escrow_name}#{aid}`, SK=`{zero_padded_sn}#{digest}`

**DynamoDbError**: `Sdk(String) | Serialization(String) | ConditionalCheckFailed(String) | NotFound(String) | Generic(String)`

## Appendix: JSON Field Map

| Key | Field | Domain |
|-----|-------|--------|
| v | serialization_info | version |
| t | event_type/ilk | type tag |
| d | digest | SAID |
| i | prefix | identifier |
| s | sn | seq number (hex) |
| p | previous_event_hash | prior digest |
| kt | threshold | key threshold |
| k | public_keys | key list |
| nt | next threshold | next key threshold |
| n | next_key_hashes | next commitments |
| bt | tally | witness threshold |
| b | witnesses/backers | witness list |
| br | prune | remove list |
| ba | graft | add list |
| c | config | config traits |
| a | data | anchored seals |
| di | delegator | delegator prefix |
| dt | timestamp | RFC 3339 |
| et | last_event_type | last type |
| ee | last_est | last establishment |
| ii | issuer_id | TEL issuer |
| ri | registry_id | TEL registry |
| ra | registry_anchor | TEL anchor seal |

## Appendix: Feature Gates

| Feature | Crate | Enables |
|---------|-------|---------|
| query | keri-core | ReplyEvent, SignedReply, QueryEvent, KeyStateNotice |
| oobi | keri-core | Oobi, LocationScheme, EndRole, Role, Scheme |
| oobi-manager | keri-core | OobiManager, Transport trait |
| mailbox | keri-core | Exchange, MailboxResponse, MailboxQuery |
| storage-redb | keri-core | RedbDatabase, SnKeyEscrow |
| sqs | keri-dynamodb | SqsDispatch, SqsNotificationMessage |

## Appendix: Crate Flow

`cesride/cesrox/said -> keri-core -> teliox, keri-dynamodb -> keri-sdk -> keri-controller -> keri-witness, keri-watcher`
